package main

// docs-build, ported from tools/docs-build.mjs: upstream mdx → Zola markdown
// (built by the vitezola theme — see docs/site/). The text transform chain,
// the four MDX shapes, the grey-list cross-check, content-map emission,
// index + section pages. The ONE node dependency left is prettier's html
// printer for the demo markup shown under each preview
// (tools/prettier-batch.mjs — one subprocess per build).

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

const (
	docsRadixDir = docsUpstreamMirror + "/components/radix"
	docsRoot     = "docs"
	// docs/site is the Zola site (config.toml, templates/, sass/, themes/).
	// Its content/ and static/ are fully generated here each run.
	siteRoot    = docsRoot + "/site"
	contentRoot = siteRoot + "/content"
	staticRoot  = siteRoot + "/static"
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
	locs := re.FindAllStringSubmatchIndex(shadow, -1)
	out := text
	for i := len(locs) - 1; i >= 0; i-- {
		loc := locs[i]
		groups := make([]string, len(loc)/2)
		for g := range groups {
			if loc[2*g] >= 0 {
				groups[g] = text[loc[2*g]:loc[2*g+1]]
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

var reCalloutStart = regexp.MustCompile(`<Callout\b`)

// findCalloutOpen returns the [start, end) of a <Callout …> opening tag.
//
// It is a scanner rather than `<Callout\b[^>]*>` because a JSX attribute value
// is an expression that can contain ">": native-select.mdx opens with
// `<Callout variant="info" icon={<InfoIcon className="translate-y-[3px]!" />}>`,
// where the character class stopped at the ">" inside "/>" and left a bare
// "}>" as the first line of the rendered callout. Braces nest, quotes are
// opaque, and the tag ends at the first ">" outside both.
func findCalloutOpen(s string) (int, int) {
	m := reCalloutStart.FindStringIndex(s)
	if m == nil {
		return -1, -1
	}
	depth := 0
	var quote byte
	for i := m[1]; i < len(s); i++ {
		c := s[i]
		switch {
		case quote != 0:
			if c == quote {
				quote = 0
			}
		case c == '"' || c == '\'':
			quote = c
		case c == '{':
			depth++
		case c == '}':
			if depth > 0 {
				depth--
			}
		case c == '>' && depth == 0:
			return m[0], i + 1
		}
	}
	return -1, -1 // an unterminated tag is not an opening tag
}

func convertCallouts(text, page string) (string, error) {
	out := text
	for {
		shadow := markupShadow(out)
		os_, oe := findCalloutOpen(shadow)
		if os_ < 0 {
			break
		}
		open := []int{os_, oe}
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
		// vitezola's tip component; `title` must be listed on a block call
		// (empty falls back to the kind's default title). A title with a
		// double quote would break the call — none of the sources carry one,
		// so fail loudly instead of emitting a page that cannot render.
		title := attrs["title"]
		if strings.ContainsAny(title, `"\`) {
			return "", fmt.Errorf("%s: <Callout> title with a quote/backslash is not expressible: %q", page, title)
		}
		block := "{% <tip kind=\"" + kind + "\" title=\"" + title + "\" no_title={false}> %}"
		block += "\n" + trimmed + "\n{% </tip> %}"
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

// convertDetails rewrites the VitePress `::: details <summary>` container
// (hand-authored in docs/content/introduction.mdx and passed through the
// guide transforms) into vitezola's details component. Embedded double
// quotes are HTML-escaped: &quot; inside the attribute round-trips to a
// literal quote in the rendered summary.
func convertDetails(text, page string) (string, error) {
	lines := strings.Split(text, "\n")
	var out []string
	inFence := false
	open := false
	for _, l := range lines {
		trimmed := strings.TrimSpace(l)
		if strings.HasPrefix(trimmed, "```") {
			inFence = !inFence
			out = append(out, l)
			continue
		}
		if inFence {
			out = append(out, l)
			continue
		}
		if !open && strings.HasPrefix(l, "::: details ") {
			sum := strings.TrimSpace(strings.TrimPrefix(l, "::: details "))
			sum = strings.ReplaceAll(sum, `"`, "&quot;")
			out = append(out, `{% <details summary="`+sum+`" open={false}> %}`)
			open = true
			continue
		}
		if open && l == ":::" {
			out = append(out, "{% </details> %}")
			open = false
			continue
		}
		out = append(out, l)
	}
	if open {
		return "", fmt.Errorf("%s: ::: details without a closing :::", page)
	}
	return strings.Join(out, "\n"), nil
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

// reRadixLegacyPath: radix swapped the two path segments; 24 upstream pages
// still link the old form, which answers 308 to the new one. Rewritten to
// what it redirects to (each target verified 200) rather than left as a hop.
var reRadixLegacyPath = regexp.MustCompile(`https://www\.radix-ui\.com/docs/primitives/`)

func rewriteLinks(text string, siteMembers map[string]bool) string {
	text = reRadixLegacyPath.ReplaceAllString(text, "https://www.radix-ui.com/primitives/docs/")
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
	seen := map[string]bool{}
	var left []string
	for _, m := range reJsxComponent.FindAllStringSubmatch(text, -1) {
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
	catalog   docsCatalog
	previewSt map[string]docsCatalog
	rtlLangs  map[string][]string
	markup    map[string]string // demo file → prettier-formatted markup
	sections  map[string][]string
	realSlots map[string]bool // lazily built by shippedSlots()
	irCache   map[string]*cssIrComponent
}

// componentIR reads+parses generated/ir/<name>.json once per name — memoized
// the way shippedSlots() memoizes ctx.realSlots — instead of each of
// compositionTransform/apiReferenceTransform independently os.ReadFile +
// json.Unmarshal'ing the same small file for the same page. nil (on a
// missing file or a parse error) is a valid cached result: both callers
// already treat "no IR" as "skip this bit of content".
func (ctx *docsBuildCtx) componentIR(name string) *cssIrComponent {
	if ir, ok := ctx.irCache[name]; ok {
		return ir
	}
	if ctx.irCache == nil {
		ctx.irCache = map[string]*cssIrComponent{}
	}
	var ir *cssIrComponent
	if irb, err := os.ReadFile(filepath.Join("generated/ir", name+".json")); err == nil {
		var parsed cssIrComponent
		if json.Unmarshal(irb, &parsed) == nil {
			ir = &parsed
		}
	}
	ctx.irCache[name] = ir
	return ir
}

var reDataSlotAttr = regexp.MustCompile(`data-slot="([a-z0-9-]+)"`)
var reDataSlotSet = regexp.MustCompile(`setAttribute\(\s*"data-slot"\s*,\s*"([a-z0-9-]+)"`)

// shippedSlots: every data-slot that EXISTS — present in shipped markup, or
// written by the shipped runtime (dialog-portal, sheet-portal and
// navigation-menu-viewport are created at open time and have no static page).
//
// The API Reference's slot table is built from generated/ir/<name>.json, which
// carries every node upstream's TSX declares, while the table's own preamble
// says "every node is a data-slot attribute in the shipped markup". Those are
// different sets: 21 of the listed slots are React-only wrappers with no DOM
// presence anywhere — dialog, popover, select, tooltip-provider,
// context-menu-sub, menubar-label … The vanilla shape of those components is
// trigger + <template>, with no wrapping element at all.
func (ctx *docsBuildCtx) shippedSlots() map[string]bool {
	if ctx.realSlots != nil {
		return ctx.realSlots
	}
	out := map[string]bool{}
	for _, dir := range []string{"dist/components", "docs/demos"} {
		ents, err := os.ReadDir(dir)
		if err != nil {
			continue
		}
		for _, e := range ents {
			if !strings.HasSuffix(e.Name(), ".html") {
				continue
			}
			b, err := os.ReadFile(filepath.Join(dir, e.Name()))
			if err != nil {
				continue
			}
			for _, m := range reDataSlotAttr.FindAllStringSubmatch(string(b), -1) {
				out[m[1]] = true
			}
		}
	}
	jsFiles := []string{"dist/shadless.js"}
	if ents, err := os.ReadDir("dist/js"); err == nil {
		for _, e := range ents {
			if strings.HasSuffix(e.Name(), ".js") {
				jsFiles = append(jsFiles, filepath.Join("dist/js", e.Name()))
			}
		}
	}
	for _, f := range jsFiles {
		b, err := os.ReadFile(f)
		if err != nil {
			continue
		}
		for _, m := range reDataSlotSet.FindAllStringSubmatch(string(b), -1) {
			out[m[1]] = true
		}
	}
	ctx.realSlots = out
	return out
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
		"\n\n<Step>" + stepTail + " from any example on this page (the code tab under its preview) into your page and adapt it" + inlineTail + " — the inline utilities are picked up by your build's content scan.</Step>\n" +
		protocolMdx(name) + trivialMdx(name) +
		"\nNo Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.\n\n</Steps>"
}

// usageMdx: nothing. Upstream's `## Usage` is an import + a JSX composition;
// the shadless replacement for it was one sentence ("Copy the markup from
// dist/components/<name>.html and adapt it") that Installation's own last
// step already says ten lines earlier, plus an axis table that spelled every
// value `outline` and claimed a runtime-driven `data-state` on the 24
// components whose own Installation table says "no JavaScript". The axes are
// documented once, from the IR, in API Reference (cvaAxisTableMdx); the copy
// instruction stays in Installation. So the section is dropped outright —
// the gate drops the same span (withoutUsageSection), so headings still line
// up on both sides.
func (ctx *docsBuildCtx) usageMdx(name string) string { return "" }

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
		if ir := ctx.componentIR(name); ir != nil {
			for _, c := range ir.Components {
				for _, e := range c.Elements {
					if e.Slot != "" {
						nameToSlot[c.Fn] = e.Slot
						break
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
	var axes []cvaAxisRow
	tier := ""
	slotSeen := map[string]bool{}
	real := ctx.shippedSlots()
	if ir := ctx.componentIR(name); ir != nil {
		for _, c := range ir.Components {
			for _, e := range c.Elements {
				if e.Slot != "" && !slotSeen[e.Slot] && real[e.Slot] {
					slotSeen[e.Slot] = true
					slots = append(slots, e.Slot)
				}
			}
		}
		axes = cvaAxisRows(*ir)
		tier = ir.Tier
	}
	s := locateApiReferenceSpan(name, fenceShadow(raw))
	extra := apiReferenceMdx(name, slots, axes, tier, s != nil)
	if extra == "" {
		return raw
	}
	if s != nil {
		return raw[:s.start] + "## API Reference\n\n" + extra + "\n" + raw[s.end:]
	}
	at := m[1]
	return raw[:at] + "\n" + extra + raw[at:]
}

func (ctx *docsBuildCtx) componentTransform(name, raw string) (string, error) {
	var seen []string
	raw, err := applyJsxOverrides(name, raw)
	if err != nil {
		return "", err
	}
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
	if s := locateChangelogSpan(fenceShadow(out)); s != nil {
		out = replaceSpan(out, *s, "")
	}
	if s := locateMessageScrollerJsSpan(fenceShadow(out)); s != nil {
		out = replaceSpan(out, *s, messageScrollerJsNote())
	}
	out, err = rewriteLeakedJsxFences(name, out)
	if err != nil {
		return "", err
	}
	out, err = rewriteInlineJsxMentions(name, out)
	if err != nil {
		return "", err
	}
	sort.Strings(seen)
	ctx.sections[name] = seen
	return applyTextAdjustments(name+".mdx", out)
}

// utilsInstallMdx used to say the utilities "ship precompiled inside
// dist/shadless-core.css" and that you could "load shadless-core.css and use
// the classes directly". Neither is true: that file opens with
// `@import "tailwindcss"` and declares the utilities as `@utility` rules — it
// is a Tailwind SOURCE, not a stylesheet a browser can load — and the
// precompiled `shadless.full.min.css` carries none of them beyond the single
// `.shimmer` one of the demos happens to use.
func utilsInstallMdx(util string) string {
	return "## Installation\n\nThe `" + util + "` utilities are declared as Tailwind `@utility` rules in\n`dist/shadless-core.css` (npm: bare `shadless`) — the same file every shadless\ncomponent already needs — so on the Tailwind path there is nothing extra to\ninstall or import: write the class and your build emits it.\n\nThey are not in the no-build stylesheet. `dist/shadless.full.min.css` is\ncompiled ahead of time from shadless's own demo markup, and Tailwind emits a\nutility only where it saw the class, so a class you have not used yet is not in\nthere. These utilities need Tailwind running over your own markup (see the\n[Installation](/docs/installation) guide)."
}

func rtlMigrateMdx() string {
	return "shadless components ship the pinned registry's classes as-is, and that cuts both ways: many slots are already logical (start/end-aware) and need nothing but `dir=\"rtl\"` on the page, while others still carry the physical utilities upstream wrote (`pl-*`, `right-*`, `rounded-l-*`) — `css-direction` keeps a committed inventory of exactly which. There is no migration command to run; check the components you actually use, and prefer this page's `-rtl` examples over the LTR ones where upstream authored a pair. To flip an individual icon, give it the `rtl:rotate-180` utility class."
}

func guideTransform(g guide, raw string) (string, error) {
	raw, err := applyJsxOverrides(g.slug, raw)
	if err != nil {
		return "", err
	}
	if g.rtlMigrate {
		s := locateRtlMigrateSpan(fenceShadow(raw))
		if s == nil {
			return "", fmt.Errorf("rtl migrate section: not found in %s", g.source)
		}
		raw = replaceSpan(raw, *s, rtlMigrateMdx())
		// The framework/CLI run (Get Started … Supported Styles) goes with it;
		// its "For other styles, see the Migration Guide" link used to be
		// rewritten in place, but the section it lives in is dropped now.
		f := locateRtlFrameworkSpan(fenceShadow(raw))
		if f == nil {
			return "", fmt.Errorf("rtl framework section: not found in %s", g.source)
		}
		raw = replaceSpan(raw, *f, rtlFrameworkNote())
	}
	if g.installSection {
		s := locateInstallSection(fenceShadow(raw))
		if s == nil {
			return "", fmt.Errorf("utils Installation section: not found (or no following ## Usage)")
		}
		raw = replaceSpan(raw, *s, utilsInstallMdx(g.util)+"\n\n")
	}
	if g.util != "" {
		return rewriteUtilityJsxFences(g.slug, raw)
	}
	raw, err = rewriteLeakedJsxFences(g.slug, raw)
	if err != nil {
		return "", err
	}
	raw, err = rewriteInlineJsxMentions(g.slug, raw)
	if err != nil {
		return "", err
	}
	return raw, nil
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
	// vitezola's demo component (docs/site/templates/demo.html): the card
	// wrapping the preview iframe and the demo's source. Block calls list
	// every parameter.
	return "{% <demo name=\"" + name + "\" status=\"" + status + "\"> %}\n" +
		"<iframe class=\"demo\" src=\"/demos/" + file + "\" title=\"" + name + "\" data-status=\"" + status + "\" loading=\"lazy\"></iframe>\n" +
		"\n<p class=\"demo-langs\"><a href=\"/demos/" + file + "\">Open the demo page</a>" + othersS + "</p>\n" +
		ctx.demoSource(name, file) +
		"\n{% </demo> %}\n", nil
}

func (ctx *docsBuildCtx) demoSource(name, file string) string {
	path := filepath.Join(staticRoot, "demos", file)
	markup, ok := ctx.markup[file]
	if !ok {
		return ""
	}
	scripts := readDemoScripts(path)
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
	// vitezola's codegroup: one fenced block per tab, the tab label from the
	// name= annotation. Markup rides the `text` language (highlighting off —
	// it is code a reader copies verbatim out of the shipped demo page).
	if jsText == "" {
		return "\n{% <codegroup> %}\n```text,name=" + file + "\n" + markup + "\n```\n{% </codegroup> %}\n"
	}
	return "\n{% <codegroup> %}\n```text,name=" + file + "\n" + markup + "\n```\n\n```js,name=behavior\n" + jsText + "\n```\n{% </codegroup> %}\n"
}

func (ctx *docsBuildCtx) buildPage(name, source string, weight int, transform func(string) (string, error), skipJsxCheck bool) ([]byte, error) {
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
	body, err = convertDetails(body, name)
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
		// search AFTER the marker: body[i:] starts with "%%ERROR:", so
		// Index(body[i:], "%%") was always 0 and body[i+8:i+0] panicked —
		// the one path meant to report a malformed <ComponentPreview> could
		// only ever crash.
		start := i + len("%%ERROR:")
		end := len(body)
		if j := strings.Index(body[start:], "%%"); j >= 0 {
			end = start + j
		}
		return nil, fmt.Errorf("%s", body[start:end])
	}
	if !skipJsxCheck {
		if err := assertNoJsx(name, body); err != nil {
			return nil, err
		}
	}
	title := fmString(fm, "title")
	if title == "" {
		title = name
	}
	// Upstream's frontmatter `links:` (the "doc · api" chips) are NOT emitted.
	// Both point at the component's Radix page — `api` deep-links its React
	// prop table, which is the one thing a shadless page must not present as
	// "the api". They were also duplicates: 26 of the 28 pages that carry
	// them already link the same URL from the API Reference section, in a
	// sentence that says what it is. Two (carousel's, to embla) were dead
	// 404s. The links themselves are kept, with their context, in the body.
	// docs-fidelity asserts the built page carries no page-links chips.
		front := "---\ntitle: " + yamlScalar(title) + "\n"
		if d := fmString(fm, "description"); d != "" {
			front += "description: " + yamlScalar(d) + "\n"
		}
		// sidebar order on the Zola site: sections sort their pages by weight
		front += "weight: " + fmt.Sprint(weight) + "\n"
		front += "---"
	desc := fmString(fm, "description")
	lead := ""
	if desc != "" {
		lead = desc + "\n\n"
	}
	return []byte(front + "\n\n# " + title + "\n\n" + lead + normalizeBlankLines(strings.TrimSpace(body)) + "\n"), nil
}

// normalizeBlankLines tidies the seams the section transforms leave behind:
// a replacement that ends without a trailing newline butts its successor's
// `## ` heading straight onto the last line of prose, and one that ends with
// several leaves a four-line gap. 55 headings across the built pages had no
// blank line in front of them.
//
// Whitespace only, and only outside fences — fence content is compared
// verbatim by docs-fidelity and is code a reader copies.
func normalizeBlankLines(body string) string {
	lines := strings.Split(body, "\n")
	var out []string
	inFence := false
	for _, line := range lines {
		if strings.HasPrefix(strings.TrimSpace(line), "```") {
			inFence = !inFence
			out = append(out, line)
			continue
		}
		if inFence {
			out = append(out, line)
			continue
		}
		if strings.TrimSpace(line) == "" {
			// at most one blank line in a row
			if len(out) > 0 && strings.TrimSpace(out[len(out)-1]) == "" {
				continue
			}
			out = append(out, "")
			continue
		}
		// a heading always gets a blank line above it
		if strings.HasPrefix(line, "#") && len(out) > 0 && strings.TrimSpace(out[len(out)-1]) != "" {
			out = append(out, "")
		}
		out = append(out, line)
	}
	return strings.Join(out, "\n")
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
	// docs/site's content/ and static/ are generated in full; only the theme
	// (themes/vitezola), the site shell (config.toml, templates/, sass/) and
	// this pipeline's own inputs are tracked
	for _, d := range []string{contentRoot + "/components", contentRoot + "/guides", staticRoot + "/demos", staticRoot + "/js"} {
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
			if err := os.WriteFile(filepath.Join(staticRoot, "demos", e.Name()), b, 0o644); err != nil {
				fmt.Fprintln(os.Stderr, "docs-build:", err)
				return 1
			}
			copied++
		}
	}
	for _, asset := range []string{"out.css", "shadless.js"} {
		b, _ := os.ReadFile("dist/" + asset)
		if err := os.WriteFile(filepath.Join(staticRoot, asset), b, 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "docs-build:", err)
			return 1
		}
	}
	glue := 0
	ents, _ := os.ReadDir("dist/js")
	for _, e := range ents {
		b, _ := os.ReadFile("dist/js/" + e.Name())
		if err := os.WriteFile(filepath.Join(staticRoot, "js", e.Name()), b, 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "docs-build:", err)
			return 1
		}
		glue++
	}
	fmt.Printf("demos copied: %d pages, %d behavior files + out.css/shadless.js\n", copied, glue)

	// markup pretty-printed ONCE per file via the prettier shell
	var items []prettierItem
	demoEnts, _ := os.ReadDir(staticRoot + "/demos")
	for _, e := range demoEnts {
		if !strings.HasSuffix(e.Name(), ".html") {
			continue
		}
		raw, _ := os.ReadFile(filepath.Join(staticRoot, "demos", e.Name()))
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
		weight            int
		transform         func(string) (string, error)
		skipJsxCheck      bool
	}
	var allPages []pageJob
	for i, c := range componentPages {
		c := c
		allPages = append(allPages, pageJob{c.name, c.source, contentRoot + "/components", i + 1, func(src string) (string, error) {
			return ctx.componentTransform(c.name, src)
		}, false})
	}
	for i, g := range guides {
		g := g
		allPages = append(allPages, pageJob{g.slug, g.source, contentRoot + "/guides", i + 1, func(src string) (string, error) {
			return guideTransform(g, src)
		}, false})
	}
	built := 0
	var errors_ []string
	for _, page := range allPages {
		out, err := ctx.buildPage(page.name, page.source, page.weight, page.transform, page.skipJsxCheck)
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

	// home + section pages. The Zola sidebar is derived per top-level section
	// from the content tree (weight order set above), so there is no sidebar
	// artifact to emit.
	guideBySlug := map[string]guide{}
	for _, g := range guides {
		guideBySlug[g.slug] = g
	}
	compIndex := "---\ntitle: \"Components\"\nsort_by: \"weight\"\n---\n\n# Components\n\n" +
		fmt.Sprintf("%d components ported · %d not ported (they need React, or upstream removed them) · %d guides.\n\n",
			len(mirrorSetCache), len(greyComponents), len(guides)) +
		"New here? Read the [Introduction](/guides/introduction) to learn what shadless is and why it exists.\n\n"
	for _, n := range meta.Pages {
		if greySet[n] {
			line := "- " + n + " <span class=\"unavailable\">not available</span>"
			// A name can be greyed as a component and still have a guide —
			// typography is CSS, not a component, so it ships as one.
			if g, ok := guideBySlug[n]; ok {
				line += " — see the [" + g.title + "](/guides/" + g.slug + ") guide"
			}
			compIndex += line + "\n"
		} else {
			compIndex += "- [" + n + "](/components/" + n + ")\n"
		}
	}
	compIndex += "\n## Guides\n\n"
	for _, g := range guides {
		compIndex += "- [" + g.title + "](/guides/" + g.slug + ")\n"
	}
	if err := os.MkdirAll(contentRoot+"/components", 0o755); err != nil {
		fmt.Fprintln(os.Stderr, "docs-build:", err)
		return 1
	}
	if err := os.WriteFile(contentRoot+"/components/_index.md", []byte(compIndex), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "docs-build:", err)
		return 1
	}
	guidesIndex := "---\ntitle: \"Guides\"\nsort_by: \"weight\"\n---\n\n# Guides\n\n" +
		fmt.Sprintf("%d guides — ported where they make sense for a component library that ships static HTML.\n", len(guides))
	if err := os.WriteFile(contentRoot+"/guides/_index.md", []byte(guidesIndex), 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "docs-build:", err)
		return 1
	}
	// the hero home — the theme's index.html renders vitepress_home and
	// nothing else, so the component listing lives on /components/ above
	// TOML frontmatter: the vitepress_home tables are TOML, and a YAML block
	// cannot carry them (the pages keep YAML — Zola accepts either per file)
	home := "+++\ntemplate = \"index.html\"\n" +
		"[extra.vitepress_home]\ntext = \"shadless\"\ntagline = \"shadcn/ui as static HTML and a vanilla runtime — no React. Copy the markup, load the stylesheets, done.\"\n" +
		"[[extra.vitepress_home.actions]]\ntext = \"Get Started\"\ntheme = \"brand\"\nlink = \"/guides/introduction/\"\n\n" +
		"[[extra.vitepress_home.actions]]\ntext = \"Components\"\ntheme = \"alt\"\nlink = \"/components/\"\n\n" +
		"[[extra.vitepress_home.features]]\ntitle = \"Static HTML\"\ndetails = \"Every component is markup + CSS you copy — data-slot attributes, Tailwind utilities, no framework.\"\n\n" +
		"[[extra.vitepress_home.features]]\ntitle = \"Vanilla runtime\"\ndetails = \"A dependency-free JS base wires the interactive pieces; shadless.init(root) for content added later.\"\n\n" +
		"[[extra.vitepress_home.features]]\ntitle = \"shadcn/ui, ported\"\ndetails = \"" + fmt.Sprintf("%d components mirrored from the pinned upstream registry, plus the guides that still apply.", len(mirrorSetCache)) + "\"\n+++\n"
	if err := os.WriteFile(contentRoot+"/_index.md", []byte(home), 0o644); err != nil {
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

// jsonKV + stringifyJSON emit JSON.stringify(v, null, 2) byte-exactly for
// the value shapes docs-build produces (objects as []jsonKV, arrays,
// strings) — still used by writeContentMap (docs/content-map.json).
type jsonKV struct {
	k string
	v any
}

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
