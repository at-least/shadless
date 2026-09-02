package main

// docs-build, ported from tools/docs-build.mjs: upstream mdx → VitePress
// markdown. The text transform chain, the four MDX shapes, the grey-list
// cross-check, content-map emission, index + sidebar. The ONE node
// dependency left is prettier's html printer for the demo markup shown under
// each preview (tools/prettier-batch.mjs — one subprocess per build).

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

const (
	docsRadixDir = ".upstream/shadcn-ui/apps/v4/content/docs/components/radix"
	docsRoot     = "docs"
)

// FT5: canonical grey list — the radix meta.json entries with NO shadless
// implementation. Cross-checked against docs/catalog.json at every build.
var greyComponents = []string{
	// 10 tombstones (Wave D/E externals; form has no mdx page; the medium
	// pair menubar/navigation-menu emitted since — contract-tested glue)
	"calendar", "chart", "combobox", "command", "drawer", "form", "input-otp",
	"resizable", "sidebar", "sonner",
	// 5 FT6 grey dispositions (0 implement / 5 grey)
	"data-table", "date-picker", "questionnaire", "toast", "typography",
}

type docsCatalog struct {
	Previews []struct {
		Name     string `json:"name"`
		Status   string `json:"status"`
		DemoPath string `json:"demoPath"`
	} `json:"previews"`
	Sources []struct {
		Name   string `json:"name"`
		Status string `json:"status"`
	} `json:"sources"`
}

type docsMeta struct {
	Pages []string `json:"pages"`
}

// ---- markup-scoped helpers ----------------------------------------------------

var (
	reFenceSplit     = regexp.MustCompile("(?s)(```.*?```)")
	reInlineCodeSpan = regexp.MustCompile("`+[^`\n]*`+")
	reJsxComponent   = regexp.MustCompile(`<([A-Z]\w*)`)
	reCompPreviewAll = regexp.MustCompile(`(?s)<ComponentPreview\b.*?/>`)
	reCompSourceAll  = regexp.MustCompile(`(?s)<ComponentSource\b.*?/>`)
	reStepsTag       = regexp.MustCompile(`</?Steps\b[^>]*>`)
	reStepBlock      = regexp.MustCompile(`(?s)<Step>(.*?)</Step>`)
	reKbdTag         = regexp.MustCompile(`</?Kbd>`)
	reLinkedCard     = regexp.MustCompile(`<LinkedCard\b[^>]*>|</LinkedCard>`)
	reClassName      = regexp.MustCompile(`\bclassName=`)
	reMdLink         = regexp.MustCompile(`\[([^\]]*)\]\((/[^)\s]*)\)`)
	reJsxAttr        = regexp.MustCompile(`([A-Za-z][\w-]*)=(?:"([^"]*)"|\{((?:[^{}]|\{[^}]*\})*)\})`)
	reBodyTag        = regexp.MustCompile(`(?s)<body[^>]*>(.*?)</body>`)
	reScriptBlock    = regexp.MustCompile(`(?s)<script.*?</script>`)
	reFmBlock        = regexp.MustCompile(`(?s)^---\n.*?\n---\n`)
	reApiRefHeading  = regexp.MustCompile(`(?m)^## API Reference[ \t]*\n`)
	reInitAll        = regexp.MustCompile(`shadless\.initAll\(\)`)
)

func inlineCodeShadow(text string) string {
	return reInlineCodeSpan.ReplaceAllStringFunc(text, func(m string) string {
		return strings.Repeat(" ", len(m))
	})
}

func markupShadow(text string) string { return inlineCodeShadow(fenceShadow(text)) }

// replaceMarkup runs re over the markup shadow and splices replacements into
// the REAL text at the same offsets, right to left.
func replaceMarkup(text string, re *regexp.Regexp, fn func(whole string, m []string) string) string {
	shadow := markupShadow(text)
	var hits [][]int
	locs := re.FindAllStringSubmatchIndex(shadow, -1)
	for _, loc := range locs {
		groups := make([]string, len(loc)/2)
		for i := range groups {
			if loc[2*i] >= 0 {
				groups[i] = shadow[loc[2*i]:loc[2*i+1]]
			}
		}
		hits = append(hits, loc)
		_ = groups
	}
	out := text
	for i := len(locs) - 1; i >= 0; i-- {
		loc := locs[i]
		groups := make([]string, len(loc)/2)
		for g := range groups {
			if loc[2*g] >= 0 {
				groups[g] = shadow[loc[2*g]:loc[2*g+1]]
			}
		}
		whole := text[loc[0]:loc[1]]
		out = out[:loc[0]] + fn(whole, groups) + out[loc[1]:]
	}
	return out
}

// parseAttrs: JSX attributes name="value" (expressions dropped — every one
// upstream is a React icon element).
func parseAttrs(tag string) map[string]string {
	attrs := map[string]string{}
	for _, m := range reJsxAttr.FindAllStringSubmatch(tag, -1) {
		if m[2] != "" || strings.Contains(m[0], `="`) {
			attrs[m[1]] = m[2]
		}
	}
	return attrs
}

// stripImportsOutsideFences: split on fence blocks, strip only outside them.
func stripImportsOutsideFences(src string) string {
	segs := reFenceSplit.Split(src, -1)
	fences := reFenceSplit.FindAllStringSubmatch(src, -1)
	var b strings.Builder
	for i, seg := range segs {
		b.WriteString(stripImports(seg))
		if i < len(fences) {
			b.WriteString(fences[i][0])
		}
	}
	return b.String()
}

// ---- the four MDX shapes --------------------------------------------------------

var calloutKind = map[string]string{"info": "tip", "warning": "warning", "danger": "danger"}

var reCalloutOpen = regexp.MustCompile(`<Callout\b[^>]*>`)

func convertCallouts(text, page string) (string, error) {
	out := text
	for {
		shadow := markupShadow(out)
		open := reCalloutOpen.FindStringSubmatchIndex(shadow)
		if open == nil {
			break
		}
		close := strings.Index(shadow[open[1]:], "</Callout>")
		if close < 0 {
			return "", fmt.Errorf("%s: <Callout> without a closing tag", page)
		}
		close += open[1]
		attrs := parseAttrs(out[open[0]:open[1]])
		kind := calloutKind[attrs["variant"]]
		if kind == "" {
			kind = calloutKind["info"]
		}
		body := out[open[1]:close]
		var ls []string
		for _, l := range strings.Split(body, "\n") {
			ls = append(ls, stripUpTo3Spaces(l))
		}
		trimmed := strings.TrimSpace(strings.Join(ls, "\n"))
		block := "::: " + kind
		if attrs["title"] != "" {
			block += " " + attrs["title"]
		}
		block += "\n" + trimmed + "\n:::"
		out = out[:open[0]] + block + out[close+len("</Callout>"):]
	}
	return out, nil
}

// stripUpTo3Spaces removes 1–3 leading spaces before a non-space char —
// the JS /^ {1,3}(?=\S)/ has a lookahead RE2 cannot express.
func stripUpTo3Spaces(l string) string {
	n := 0
	for n < 3 && n < len(l) && l[n] == ' ' {
		n++
	}
	if n > 0 && n < len(l) {
		return l[n:]
	}
	return l
}

func convertSteps(text string) string {
	out := replaceMarkup(text, reStepsTag, func(w string, m []string) string { return "" })
	out = replaceMarkup(out, reStepBlock, func(w string, m []string) string {
		return "**" + strings.TrimSpace(m[1]) + "**"
	})
	return out
}

func convertKbd(text string) string {
	return replaceMarkup(text, reKbdTag, func(w string, m []string) string {
		if strings.HasPrefix(w, "</") {
			return "</kbd>"
		}
		return "<kbd>"
	})
}

func convertLinkedCards(text string) string {
	return replaceMarkup(text, reLinkedCard, func(w string, m []string) string {
		if strings.HasPrefix(w, "</") {
			return "</div>"
		}
		return `<div class="linked-card">`
	})
}

func convertClassName(text string) string {
	return replaceMarkup(text, reClassName, func(w string, m []string) string { return "class=" })
}

func rewriteLinks(text string, siteMembers map[string]bool) string {
	return replaceMarkup(text, reMdLink, func(whole string, m []string) string {
		route := resolveDocsRoute(m[2], siteMembers)
		if route == nil {
			return whole
		}
		if route.grey {
			return m[1]
		}
		slug := strings.TrimSuffix(route.file, ".html")
		target := "/guides/" + slug
		if siteMembers[slug] {
			target = "/components/" + slug
		}
		if route.frag != "" {
			target += "#" + route.frag
		}
		return "[" + m[1] + "](" + target + ")"
	})
}

// assertNoJsx: any JSX reaching here is a shape this mapping has never seen.
func assertNoJsx(page, text string) error {
	shadow := markupShadow(text)
	seen := map[string]bool{}
	var left []string
	for _, m := range reJsxComponent.FindAllStringSubmatch(shadow, -1) {
		if !seen[m[1]] {
			seen[m[1]] = true
			left = append(left, m[1])
		}
	}
	if len(left) > 0 {
		return fmt.Errorf("%s: unmapped JSX components: %s", page, strings.Join(left, ", "))
	}
	return nil
}

// ---- section transforms ----------------------------------------------------------

type docsBuildCtx struct {
	catalog    docsCatalog
	previewSt  map[string]docsCatalog
	rtlLangs   map[string][]string
	markup     map[string]string // demo file → prettier-formatted markup
	sections   map[string][]string
}

func (ctx *docsBuildCtx) installStepsMdx(name string) string {
	demo, _ := os.ReadFile(filepath.Join("dist/components", name+".html"))
	demoS := string(demo)
	initAll := reInitAll.MatchString(demoS)
	scripts := extractDemoScripts(demoS)
	inlineInit := len(scripts.inlineScripts) > 0
	hasOwnCss := fileExists(filepath.Join("dist/css", name+".css"))
	rows := []string{
		"| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |",
	}
	if hasOwnCss {
		rows = append(rows, "| `dist/css/"+name+".css` | this component's slot styles (`@apply` source — your build compiles it) |")
	}
	rows = append(rows, "| `dist/components/"+name+".html` | component markup — copy your page's structure from here |")
	var loadLines []string
	hasShadlessJS := false
	for _, s := range scripts.srcScripts {
		if s == "shadless.js" {
			hasShadlessJS = true
			break
		}
	}
	if hasShadlessJS {
		rows = append(rows, "| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |")
		loadLines = append(loadLines, `<script src="shadless.js"></script>`)
	} else if len(scripts.srcScripts) == 0 {
		rows = append(rows, "| — | no JavaScript: this component is markup + CSS |")
	}
	for _, s := range scripts.srcScripts {
		if s == "shadless.js" {
			continue
		}
		label := "vendored runtime (" + s + ")"
		if strings.HasPrefix(s, "js/") {
			label = "this component's behavior — registers with the base"
		}
		rows = append(rows, "| `dist/"+s+"` | "+label+" |")
		loadLines = append(loadLines, `<script src="`+s+`"></script>`)
	}
	inlineNote := ""
	if !initAll && len(scripts.srcScripts) > 0 && inlineInit {
		inlineNote = " (including the inline init script at the bottom of the demo page)"
	}
	jsStep := ""
	if len(loadLines) > 0 {
		jsStep = "\n\n<Step>Load the behavior files in your page:</Step>\n\n```html\n" + strings.Join(loadLines, "\n") + "\n```"
	}
	cssImports := ""
	if hasOwnCss {
		cssImports = "\n@import \"shadless/" + name + ".css\";"
	}
	noCssNote := ""
	if !hasOwnCss {
		noCssNote = "\nThis component has no stylesheet of its own — its styling rides the core theme and utilities in `shadless`.\n"
	}
	andComp := ""
	if hasOwnCss {
		andComp = " and this component"
	}
	stepTail := "Copy the markup"
	if inlineNote != "" {
		stepTail += " and init"
	}
	inlineTail := ""
	if inlineInit && !initAll {
		inlineTail = inlineNote
	}
	return "<Steps>\n\n<Step>Add shadless" + andComp + " to your Tailwind v4 entry:</Step>\n\n```css\n@import \"shadless\";" + cssImports + "\n```\n" + noCssNote +
		"\nThe files this component needs:\n\n| File | Purpose |\n| --- | --- |\n" + strings.Join(rows, "\n") + "\n" + jsStep +
		"\n\n<Step>" + stepTail + " from `dist/components/" + name + ".html` into your page and adapt it" + inlineTail + " — the inline utilities are picked up by your build's content scan.</Step>\n" +
		protocolMdx(name) + trivialMdx(name) +
		"\nNo Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.\n\n</Steps>"
}

func (ctx *docsBuildCtx) usageMdx(name string) string {
	var axes []string
	seen := map[string]bool{}
	if irb, err := os.ReadFile(filepath.Join("generated/ir", name+".json")); err == nil {
		var ir cssIrComponent
		if json.Unmarshal(irb, &ir) == nil {
			for _, key := range ir.Cva.keys {
				table := ir.Cva.tables[key]
				for _, ax := range table.axisOrder {
					if !seen[ax] {
						seen[ax] = true
						axes = append(axes, ax)
					}
				}
			}
		}
	}
	var rows []string
	for _, a := range axes {
		rows = append(rows, "| `"+a+"=\"outline\"` (JSX prop) | `data-"+a+"=\"outline\"` (markup) |")
	}
	axesTable := ""
	if len(rows) > 0 {
		axesTable = " The component's API axes are data attributes:\n\n| JSX prop | Markup |\n| --- | --- |\n" + strings.Join(rows, "\n")
	}
	return "## Usage\n\nCopy the markup from `dist/components/" + name + ".html` and adapt it — every slot\nis a `data-slot` attribute, and open/close state is a `data-state` the\nruntime drives." + axesTable + "\n"
}

func (ctx *docsBuildCtx) compositionTransform(name, raw string, seen *[]string) string {
	s := locateCompositionSpan(fenceShadow(raw))
	if s == nil {
		return raw
	}
	*seen = append(*seen, "composition")
	section := raw[s.start:s.end]
	tree := ""
	if m := regexp.MustCompile("(?s)```text\n(.*?)```").FindStringSubmatch(section); m != nil {
		tree = strings.TrimRight(m[1], " \t\n")
	}
	mapped := ""
	if tree != "" {
		nameToSlot := map[string]string{}
		if irb, err := os.ReadFile(filepath.Join("generated/ir", name+".json")); err == nil {
			var ir cssIrComponent
			if json.Unmarshal(irb, &ir) == nil {
				for _, c := range ir.Components {
					for _, e := range c.Elements {
						if e.Slot != "" {
							nameToSlot[c.Fn] = e.Slot
							break
						}
					}
				}
			}
		}
		mapped = regexp.MustCompile(`[A-Z][A-Za-z0-9]+`).ReplaceAllStringFunc(tree, func(w string) string {
			if s, ok := nameToSlot[w]; ok {
				return s
			}
			return w
		})
	}
	body := "See the demos for real compositions — every slot is a `data-slot` attribute in the shipped markup.\n"
	if mapped != "" {
		body = "The slot tree — every node is a `data-slot` attribute in the shipped markup:\n\n```text\n" + mapped + "\n```\n"
	}
	return replaceSpan(raw, *s, "## Composition\n\n"+body+"\n")
}

func (ctx *docsBuildCtx) apiReferenceTransform(name, raw string, seen *[]string) string {
	m := reApiRefHeading.FindStringIndex(raw)
	if m == nil {
		return raw
	}
	*seen = append(*seen, "api-reference")
	var slots []string
	slotSeen := map[string]bool{}
	if irb, err := os.ReadFile(filepath.Join("generated/ir", name+".json")); err == nil {
		var ir cssIrComponent
		if json.Unmarshal(irb, &ir) == nil {
			for _, c := range ir.Components {
				for _, e := range c.Elements {
					if e.Slot != "" && !slotSeen[e.Slot] {
						slotSeen[e.Slot] = true
						slots = append(slots, e.Slot)
					}
				}
			}
		}
	}
	extra := apiReferenceMdx(name, slots)
	if extra == "" {
		return raw
	}
	at := m[1]
	return raw[:at] + "\n" + extra + raw[at:]
}

func (ctx *docsBuildCtx) componentTransform(name, raw string) (string, error) {
	var seen []string
	raw = stripImportsFromMixedFences(dropReactImportFences(raw))
	spans := locateCodeTabsSpans(fenceShadow(raw))
	if len(spans) != 1 {
		return "", fmt.Errorf("install code-tabs: fence-shadowed count %d, expected 1", len(spans))
	}
	seen = append(seen, "installation")
	out := replaceSpan(raw, spans[0], ctx.installStepsMdx(name))
	if u := locateUsageSpan(fenceShadow(out)); u != nil {
		seen = append(seen, "usage")
		out = replaceSpan(out, *u, ctx.usageMdx(name))
	}
	out = ctx.compositionTransform(name, out, &seen)
	out = ctx.apiReferenceTransform(name, out, &seen)
	sort.Strings(seen)
	ctx.sections[name] = seen
	return applyTextAdjustments(name+".mdx", out)
}

func utilsInstallMdx(util string) string {
	return "## Installation\n\nIn shadless, the `" + util + "` utilities ship precompiled inside `dist/out.css` —\nno npm install, Tailwind setup, or CSS import is required. Load `out.css` and\nuse the classes directly (see the [Installation](/docs/installation) guide)."
}

func rtlMigrateMdx() string {
	return "shadless components ship the pinned registry's classes as-is — the current registry already uses logical (start/end-aware) utilities, so there is no migration step: every component is RTL-ready the moment the page carries `dir=\"rtl\"`. To flip an individual icon, give it the `rtl:rotate-180` utility class."
}

func guideTransform(g guide, raw string) (string, error) {
	if g.rtlMigrate {
		s := locateRtlMigrateSpan(fenceShadow(raw))
		if s == nil {
			return "", fmt.Errorf("rtl migrate section: not found in %s", g.source)
		}
		raw = replaceSpan(raw, *s, rtlMigrateMdx())
		const LINK = "For other styles, see the [Migration Guide](#migrating-existing-components)."
		if !strings.Contains(raw, LINK) {
			return "", fmt.Errorf("rtl guide: migrate anchor link not found — re-anchor")
		}
		raw = strings.Replace(raw, LINK, "For other styles, the shipped utilities are already logical (start/end-aware) — `dir=\"rtl\"` is all it takes.", 1)
	}
	if !g.installSection {
		return raw, nil
	}
	s := locateInstallSection(fenceShadow(raw))
	if s == nil {
		return "", fmt.Errorf("utils Installation section: not found (or no following ## Usage)")
	}
	return replaceSpan(raw, *s, utilsInstallMdx(g.util)+"\n\n"), nil
}

// ---- page assembly ---------------------------------------------------------------

func yamlScalar(s string) string { return jsonString(s) }

func (ctx *docsBuildCtx) previewMarkdown(attrs map[string]string, page string) (string, error) {
	name := attrs["name"]
	if name == "" {
		return "", fmt.Errorf("%s: <ComponentPreview> without a name", page)
	}
	status := "to-author"
	demoPath := ""
	for _, p := range ctx.catalog.Previews {
		if p.Name == name {
			status = p.Status
			demoPath = p.DemoPath
			break
		}
	}
	if status != "existing-dist" && status != "authored" {
		note := "demo not yet available"
		switch status {
		case "unavailable":
			note = "demo not available in shadless (base-style demo)"
		case "tombstoned":
			note = "demo not available in shadless (component greyed)"
		}
		return "<div class=\"demo-missing\" data-demo=\"" + name + "\" data-status=\"" + status + "\">" + note + " — <code>" + name + "</code></div>", nil
	}
	file := filepath.Base("docs/demos/" + name + ".html")
	if status == "existing-dist" {
		file = filepath.Base(demoPath)
	}
	var others []string
	if attrs["direction"] == "rtl" {
		for _, l := range ctx.rtlLangs[name] {
			if l == "ar" {
				continue
			}
			others = append(others, "<a href=\"/demos/"+name+"-"+l+".html\">"+strings.ToUpper(l)+"</a>")
		}
	}
	othersS := ""
	if len(others) > 0 {
		othersS = " · " + strings.Join(others, " · ")
	}
	return "::::demo " + name + "\n" +
		"<iframe class=\"demo\" src=\"/demos/" + file + "\" title=\"" + name + "\" data-status=\"" + status + "\" loading=\"lazy\"></iframe>\n" +
		"\n<p class=\"demo-langs\"><a href=\"/demos/" + file + "\">Open the demo page</a>" + othersS + "</p>\n" +
		ctx.demoSource(name, file) +
		"\n::::\n", nil
}

func (ctx *docsBuildCtx) demoSource(name, file string) string {
	path := filepath.Join(docsRoot, "public", "demos", file)
	raw, err := os.ReadFile(path)
	if err != nil {
		return ""
	}
	markup, ok := ctx.markup[file]
	if !ok {
		return ""
	}
	scripts := extractDemoScripts(string(raw))
	var js []string
	for _, s := range scripts.srcScripts {
		if s == "shadless.js" {
			js = append(js, "// <script src=\"shadless.js\"></script>  — the shared runtime (see Installation)")
			break
		}
	}
	for _, s := range scripts.srcScripts {
		if s == "shadless.js" {
			continue
		}
		b, _ := os.ReadFile(filepath.Join("dist", s))
		js = append(js, "// "+s+"\n"+strings.TrimSpace(string(b)))
	}
	js = append(js, scripts.inlineScripts...)
	jsText := strings.TrimSpace(strings.Join(js, "\n\n"))
	if jsText == "" {
		return "\n::: code-group\n```text:line-numbers [" + file + "]\n" + markup + "\n```\n:::\n"
	}
	return "\n::: code-group\n```text:line-numbers [" + file + "]\n" + markup + "\n```\n\n```js:line-numbers [behavior]\n" + jsText + "\n```\n:::\n"
}

func (ctx *docsBuildCtx) buildPage(name, source string, transform func(string) (string, error)) ([]byte, error) {
	rawB, err := os.ReadFile(source)
	if err != nil {
		return nil, err
	}
	raw := string(rawB)
	fm := parseFrontmatter(raw)
	body, err := transform(raw)
	if err != nil {
		return nil, err
	}
	body = reFmBlock.ReplaceAllString(body, "")
	body = stripImportsOutsideFences(body)
	body = replaceMarkup(body, reCompSourceAll, func(w string, m []string) string { return "" })
	body, err = convertCallouts(body, name)
	if err != nil {
		return nil, err
	}
	body = convertSteps(body)
	body = convertKbd(body)
	body = convertLinkedCards(body)
	body = convertClassName(body)
	siteMembers := ctx.siteMembers()
	body = rewriteLinks(body, siteMembers)
	body = replaceMarkup(body, reCompPreviewAll, func(whole string, m []string) string {
		md, err := ctx.previewMarkdown(parseAttrs(whole), name)
		if err != nil {
			return "%%ERROR:" + err.Error() + "%%"
		}
		return md
	})
	if i := strings.Index(body, "%%ERROR:"); i >= 0 {
		j := strings.Index(body[i:], "%%")
		return nil, fmt.Errorf("%s", body[i+len("%%ERROR:"):i+j])
	}
	if err := assertNoJsx(name, body); err != nil {
		return nil, err
	}
	title := fmString(fm, "title")
	if title == "" {
		title = name
	}
	links := fmLinksOrdered(raw)
	linksS := ""
	if len(links) > 0 {
		var as []string
		for _, l := range links {
			as = append(as, "<a href=\""+l.V+"\" rel=\"noopener\">"+l.K+"</a>")
		}
		linksS = "<p class=\"page-links\">" + strings.Join(as, " · ") + "</p>\n"
	}
	front := "---\ntitle: " + yamlScalar(title) + "\n"
	if d := fmString(fm, "description"); d != "" {
		front += "description: " + yamlScalar(d) + "\n"
	}
	front += "---"
	desc := fmString(fm, "description")
	lead := ""
	if desc != "" {
		lead = desc + "\n\n"
	}
	linksGap := ""
	if linksS != "" {
		linksGap = "\n"
	}
	return []byte(front + "\n\n# " + title + "\n\n" + lead + linksS + linksGap + strings.TrimSpace(body) + "\n"), nil
}

var mirrorSetCache []string

func (ctx *docsBuildCtx) siteMembers() map[string]bool {
	m := map[string]bool{}
	for _, s := range mirrorSetCache {
		m[s] = true
	}
	return m
}

func fileExists(p string) bool {
	_, err := os.Stat(p)
	return err == nil
}

// ---- the build --------------------------------------------------------------------

func runDocsBuild() int {
	catalogB, err := os.ReadFile("docs/catalog.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs-build:", err)
		return 1
	}
	var catalog docsCatalog
	if err := json.Unmarshal(catalogB, &catalog); err != nil {
		fmt.Fprintln(os.Stderr, "docs-build: catalog:", err)
		return 1
	}
	metaB, err := os.ReadFile(filepath.Join(docsRadixDir, "meta.json"))
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs-build:", err)
		return 1
	}
	var meta docsMeta
	if err := json.Unmarshal(metaB, &meta); err != nil {
		fmt.Fprintln(os.Stderr, "docs-build: meta:", err)
		return 1
	}
	rtlLangs := map[string][]string{}
	if rb, err := os.ReadFile("build/rtl-langs.json"); err == nil {
		json.Unmarshal(rb, &rtlLangs)
	}

	type compPage struct{ name, source string }
	var componentPages []compPage
	for _, s := range catalog.Sources {
		if s.Status != "existing-dist" {
			continue
		}
		p := filepath.Join(docsRadixDir, s.Name+".mdx")
		if !fileExists(p) {
			continue
		}
		componentPages = append(componentPages, compPage{s.Name, p})
	}
	sort.Slice(componentPages, func(i, j int) bool { return componentPages[i].name < componentPages[j].name })
	mirrorSet := map[string]bool{}
	for _, p := range componentPages {
		mirrorSet[p.name] = true
	}
	for _, n := range meta.Pages {
		if mirrorSet[n] {
			mirrorSetCache = append(mirrorSetCache, n)
		}
	}
	sort.Strings(mirrorSetCache)
	mirrorTotal := len(componentPages) + len(guides)
	fmt.Printf("mirror set: %d components = %d pages + %d guides\n", len(mirrorSetCache), len(componentPages), len(guides))

	greySet := map[string]bool{}
	for _, g := range greyComponents {
		greySet[g] = true
	}
	{
		var notSub, overlap, uncovered, orphan []string
		for _, s := range catalog.Sources {
			if s.Status == "no-dist" && !greySet[s.Name] {
				notSub = append(notSub, s.Name)
			}
		}
		for n := range mirrorSet {
			if greySet[n] {
				overlap = append(overlap, n)
			}
		}
		accounted := map[string]bool{}
		for n := range mirrorSet {
			accounted[n] = true
		}
		for _, g := range greyComponents {
			accounted[g] = true
		}
		for _, p := range meta.Pages {
			if !accounted[p] {
				uncovered = append(uncovered, p)
			}
		}
		for n := range accounted {
			if !containsTok(meta.Pages, n) {
				orphan = append(orphan, n)
			}
		}
		if len(notSub) > 0 || len(overlap) > 0 || len(uncovered) > 0 || len(orphan) > 0 || len(accounted) != len(meta.Pages) {
			fmt.Fprintf(os.Stderr, "FAIL grey-list cross-check: noDist-not-grey=%v built∩grey=%v meta-uncovered=%v grey-not-in-meta=%v\n", notSub, overlap, uncovered, orphan)
			return 1
		}
	}
	sidebarOrder := mirrorSetCache // meta order restricted to mirror set
	_ = sidebarOrder

	// .vitepress is NOT wiped: config.mts + theme/ are tracked; only the
	// generated sidebar.json inside it is rewritten below
	for _, d := range []string{docsRoot + "/components", docsRoot + "/guides", docsRoot + "/public/demos", docsRoot + "/public/js"} {
		if err := os.RemoveAll(d); err != nil {
			fmt.Fprintln(os.Stderr, "docs-build:", err)
			return 1
		}
		if err := os.MkdirAll(d, 0o755); err != nil {
			fmt.Fprintln(os.Stderr, "docs-build:", err)
			return 1
		}
	}
	// demos + assets into the served tree
	copied := 0
	for _, tree := range []string{"dist/components", "docs/demos"} {
		ents, _ := os.ReadDir(tree)
		for _, e := range ents {
			if !strings.HasSuffix(e.Name(), ".html") {
				continue
			}
			b, _ := os.ReadFile(filepath.Join(tree, e.Name()))
			if err := os.WriteFile(filepath.Join(docsRoot, "public/demos", e.Name()), b, 0o644); err != nil {
				fmt.Fprintln(os.Stderr, "docs-build:", err)
				return 1
			}
			copied++
		}
	}
	for _, asset := range []string{"out.css", "shadless.js"} {
		b, _ := os.ReadFile("dist/" + asset)
		if err := os.WriteFile(filepath.Join(docsRoot, "public", asset), b, 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "docs-build:", err)
			return 1
		}
	}
	glue := 0
	ents, _ := os.ReadDir("dist/js")
	for _, e := range ents {
		b, _ := os.ReadFile(filepath.Join("dist/js", e.Name()))
		if err := os.WriteFile(filepath.Join(docsRoot, "public/js", e.Name()), b, 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "docs-build:", err)
			return 1
		}
		glue++
	}
	fmt.Printf("demos copied: %d pages, %d behavior files + out.css/shadless.js\n", copied, glue)

	// markup pretty-printed ONCE per file via the prettier shell
	var items []prettierItem
	demoEnts, _ := os.ReadDir(docsRoot + "/public/demos")
	for _, e := range demoEnts {
		if !strings.HasSuffix(e.Name(), ".html") {
			continue
		}
		raw, _ := os.ReadFile(filepath.Join(docsRoot, "public/demos", e.Name()))
		body := string(raw)
		if m := reBodyTag.FindStringSubmatch(body); m != nil {
			body = m[1]
		}
		body = strings.TrimSpace(reScriptBlock.ReplaceAllString(body, ""))
		items = append(items, prettierItem{File: e.Name(), Body: body})
	}
	markup, err := prettierBatch(items)
	if err != nil {
		fmt.Fprintln(os.Stderr, "docs-build: prettier:", err)
		return 1
	}

	ctx := &docsBuildCtx{
		catalog:  catalog,
		rtlLangs: rtlLangs,
		markup:   markup,
		sections: map[string][]string{},
	}

	type pageJob struct {
		name, source, dir string
		transform         func(string) (string, error)
	}
	var allPages []pageJob
	for _, c := range componentPages {
		c := c
		allPages = append(allPages, pageJob{c.name, c.source, docsRoot + "/components", func(src string) (string, error) {
			return ctx.componentTransform(c.name, src)
		}})
	}
	for _, g := range guides {
		g := g
		allPages = append(allPages, pageJob{g.slug, g.source, docsRoot + "/guides", func(src string) (string, error) {
			return guideTransform(g, src)
		}})
	}
	built := 0
	var errors_ []string
	for _, page := range allPages {
		out, err := ctx.buildPage(page.name, page.source, page.transform)
		if err != nil {
			errors_ = append(errors_, page.name+": "+err.Error())
			continue
		}
		if err := os.WriteFile(filepath.Join(page.dir, page.name+".md"), out, 0o644); err != nil {
			errors_ = append(errors_, page.name+": "+err.Error())
			continue
		}
		built++
	}

	// content map with the per-page section sets the transforms recorded
	cpPages := make([]struct{ name, source string }, len(componentPages))
	for i, p := range componentPages {
		cpPages[i] = struct{ name, source string }{p.name, p.source}
	}
	if err := writeContentMap(cpPages, ctx.sections); err != nil {
		fmt.Fprintln(os.Stderr, "docs-build:", err)
		return 1
	}

	// index + sidebar
	idx := "---\ntitle: \"Components\"\n---\n\n# Components\n\n" +
		fmt.Sprintf("%d radix components available · %d not available (upstream tombstones or out of pipeline scope) · %d guides.\n\n",
			len(mirrorSetCache), len(greyComponents), len(guides)) +
		"New here? Read the [Introduction](/guides/introduction) to learn what shadless is and why it exists.\n\n"
	for _, n := range meta.Pages {
		if greySet[n] {
			idx += "- " + n + " <span class=\"unavailable\">not available</span>\n"
		} else {
			idx += "- [" + n + "](/components/" + n + ")\n"
		}
	}
	idx += "\n## Guides\n\n"
	for _, g := range guides {
		idx += "- [" + g.title + "](/guides/" + g.slug + ")\n"
	}
	if err := os.WriteFile(docsRoot+"/index.md", []byte(idx), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "docs-build:", err)
		return 1
	}

	sidebar := stringifyJSON(buildSidebarDocs(meta.Pages, greySet, mirrorSetCache), 0)
	if err := os.WriteFile(docsRoot+"/.vitepress/sidebar.json", []byte(sidebar+"\n"), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "docs-build:", err)
		return 1
	}

	if len(errors_) > 0 {
		for _, e := range errors_ {
			fmt.Fprintln(os.Stderr, "  - "+e)
		}
		fmt.Fprintf(os.Stderr, "FAIL  docs build (%d pages failed)\n", len(errors_))
		return 1
	}
	if built != mirrorTotal {
		fmt.Fprintf(os.Stderr, "FAIL  docs build (built %d, expected %d)\n", built, mirrorTotal)
		return 1
	}
	fmt.Printf("PASS  docs build (%d/%d pages: %d components + %d guides → markdown)\n",
		built, mirrorTotal, len(mirrorSetCache), len(guides))
	return 0
}

// buildSidebarDocs constructs the sidebar in JS object order (text, items;
// text, link) for stringifyJSON.
type jsonKV struct {
	k string
	v any
}

func buildSidebarDocs(metaPages []string, greySet map[string]bool, mirrorSet []string) []any {
	var pinned, other []any
	for _, g := range guides {
		entry := []jsonKV{{"text", g.title}, {"link", "/guides/" + g.slug}}
		if g.pinned {
			pinned = append(pinned, entry)
		} else {
			other = append(other, entry)
		}
	}
	compItems := []any{[]jsonKV{{"text", "All components"}, {"link", "/"}}}
	for _, n := range mirrorSet {
		compItems = append(compItems, []jsonKV{{"text", n}, {"link", "/components/" + n}})
	}
	var greyItems []any
	for _, p := range metaPages {
		if greySet[p] {
			greyItems = append(greyItems, []jsonKV{{"text", p}})
		}
	}
	return []any{
		[]jsonKV{{"text", "Getting started"}, {"items", pinned}},
		[]jsonKV{{"text", "Components"}, {"items", compItems}},
		[]jsonKV{{"text", "Guides"}, {"items", other}},
		[]jsonKV{{"text", "Not available"}, {"items", greyItems}},
	}
}

// stringifyJSON emits JSON.stringify(v, null, 2) byte-exactly for the value
// shapes docs-build produces (objects as []jsonKV, arrays, strings).
func stringifyJSON(v any, depth int) string {
	ind := strings.Repeat("  ", depth)
	inner := strings.Repeat("  ", depth+1)
	switch x := v.(type) {
	case []jsonKV:
		if len(x) == 0 {
			return "{}"
		}
		var parts []string
		for _, kv := range x {
			parts = append(parts, inner+jsonString(kv.k)+": "+stringifyJSON(kv.v, depth+1))
		}
		return "{\n" + strings.Join(parts, ",\n") + "\n" + ind + "}"
	case []any:
		if len(x) == 0 {
			return "[]"
		}
		var parts []string
		for _, e := range x {
			parts = append(parts, inner+stringifyJSON(e, depth+1))
		}
		return "[\n" + strings.Join(parts, ",\n") + "\n" + ind + "]"
	case string:
		return jsonString(x)
	case int:
		return fmt.Sprintf("%d", x)
	}
	return "null"
}

type prettierItem struct {
	File string `json:"file"`
	Body string `json:"body"`
}

// prettierBatch runs the one-shot node shell over all items.
func prettierBatch(items []prettierItem) (map[string]string, error) {
	if len(items) == 0 {
		return map[string]string{}, nil
	}
	stdin, _ := json.Marshal(items)
	cmd := exec.Command("node", "tools/prettier-batch.mjs")
	cmd.Stdin = bytes.NewReader(stdin)
	var out, errb bytes.Buffer
	cmd.Stdout, cmd.Stderr = &out, &errb
	if err := cmd.Run(); err != nil {
		return nil, fmt.Errorf("%v: %s", err, errb.String())
	}
	var res map[string]string
	if err := json.Unmarshal(out.Bytes(), &res); err != nil {
		return nil, err
	}
	return res, nil
}

var _ = io.Discard
