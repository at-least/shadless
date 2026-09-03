package main

// docs-fidelity pure helpers, ported from tools/docs-fidelity-lib.mjs. The
// builder and the gate share the transform locators (docs_transforms.go);
// this file holds the mdx↔markdown fact extraction and comparison.

import (
	"fmt"
	"regexp"
	"sort"
	"strings"
)

var reFenceLine = regexp.MustCompile("^`{3,}([^`]*)$")

// scanFences: one entry per fenced block; lang is the info string up to the
// first space; content is verbatim. Unclosed fences run to EOF.
type fenceInfo struct {
	lang    string
	content string
}

func scanFences(src string) []fenceInfo {
	var out []fenceInfo
	lines := strings.Split(src, "\n")
	open := -1
	var lang, content string
	for _, line := range lines {
		if open < 0 {
			if m := reFenceLine.FindStringSubmatch(line); m != nil && strings.HasPrefix(strings.TrimSpace(line), "```") {
				open = 3
				lang = strings.SplitN(m[1], " ", 2)[0]
				content = ""
				continue
			}
		} else {
			if strings.HasPrefix(strings.TrimSpace(line), "```") {
				out = append(out, fenceInfo{lang, content})
				open = -1
				continue
			}
			if content != "" || line != "" || len(out) > 0 || true {
				content += line + "\n"
			}
		}
	}
	if open >= 0 {
		out = append(out, fenceInfo{lang, content})
	}
	return out
}

// blankFences returns a length-stable view with fence content blanked.
func blankFences(src string) string {
	lines := strings.Split(src, "\n")
	open := false
	for i, line := range lines {
		t := strings.HasPrefix(strings.TrimSpace(line), "```")
		if !open && t {
			open = true
			lines[i] = blankLine(line)
			continue
		}
		if open {
			if t {
				open = false
			}
			lines[i] = blankLine(line)
		}
	}
	return strings.Join(lines, "\n")
}

// ---- span-dropping helpers (gate side of the shared locators) --------------

func replaceSpan(raw string, s span, replacement string) string {
	return raw[:s.start] + replacement + raw[s.end:]
}

func withoutCodeTabs(raw string) string {
	out := raw
	for _, s := range locateCodeTabsSpans(fenceShadow(raw)) {
		_ = s
	}
	// reverse order to keep offsets valid
	spans := locateCodeTabsSpans(fenceShadow(raw))
	for i := len(spans) - 1; i >= 0; i-- {
		out = out[:spans[i].start] + "\n" + out[spans[i].end:]
	}
	return out
}

func withoutInstallSection(raw string) string {
	s := locateInstallSection(fenceShadow(raw))
	if s == nil {
		return raw
	}
	return replaceSpan(raw, *s, "## Installation\n\n")
}

func withoutRtlMigrate(raw string) string {
	s := locateRtlMigrateSpan(fenceShadow(raw))
	if s == nil {
		return raw
	}
	return replaceSpan(raw, *s, "\n")
}

// withoutUsageSection drops the heading too, mirroring usageMdx returning ""
// — the section has no shadless replacement any more (see usageMdx). Both
// sides must agree on the span AND on what is left behind, so this is the
// second half of that change, not an independent decision.
func withoutUsageSection(raw string) string {
	s := locateUsageSpan(fenceShadow(raw))
	if s == nil {
		return raw
	}
	return replaceSpan(raw, *s, "")
}

func withoutCompositionSection(raw string) string {
	s := locateCompositionSpan(fenceShadow(raw))
	if s == nil {
		return raw
	}
	return replaceSpan(raw, *s, "## Composition\n")
}

func withoutApiReferenceSection(comp, raw string) string {
	s := locateApiReferenceSpan(comp, fenceShadow(raw))
	if s == nil {
		return raw
	}
	return replaceSpan(raw, *s, "## API Reference\n\n")
}

func withoutChangelogSection(raw string) string {
	s := locateChangelogSpan(fenceShadow(raw))
	if s == nil {
		return raw
	}
	return replaceSpan(raw, *s, "")
}

// ---- fact extraction ---------------------------------------------------------

var (
	reEntity      = regexp.MustCompile(`&(amp|lt|gt|quot);`)
	reAnyTag      = regexp.MustCompile(`<[^>]*>`)
	reWs          = regexp.MustCompile(`\s+`)
	reMdHeading   = regexp.MustCompile(`(?m)^(#{2,4})[ \t]+(.+)$`)
	reHOpen        = regexp.MustCompile(`<h([234])\b[^>]*>`)
	reInlineCode  = regexp.MustCompile("`[^`\n]+`")
	reCompPreview = regexp.MustCompile(`<ComponentPreview\b([^>]*)>`)
	reCompSource  = regexp.MustCompile(`<ComponentSource\b([^>]*)>`)
)

func decodeEntities(s string) string {
	return reEntity.ReplaceAllStringFunc(s, func(e string) string {
		switch e {
		case "&amp;":
			return "&"
		case "&lt;":
			return "<"
		case "&gt;":
			return ">"
		case "&quot;":
			return `"`
		}
		return e
	})
}

func htmlText(s string) string {
	return decodeEntities(reWs.ReplaceAllString(reAnyTag.ReplaceAllString(s, ""), " "))
}

func mdText(s string) string { return reWs.ReplaceAllString(s, " ") }

func stripInlineCode(s string) string {
	return reInlineCode.ReplaceAllString(s, "$1")
}

type headingEnt struct {
	depth int
	text  string
}

type previewEnt struct {
	name       string
	styleName  string
	direction  string
}

type sourceEnt struct {
	name string
	src  string
}

type pageFacts struct {
	frontmatter frontmatter
	headings    []headingEnt
	previews    []previewEnt
	sources     []sourceEnt
	fences      []fenceInfo
}

// mdxPageFacts with the drop* predicates mirroring what the builder applied.
func mdxPageFacts(name, raw string, dropCodeTabs, dropInstallSection, dropRtlMigrate, dropUsageSection, dropCompositionSection, dropApiReferenceSection, dropChangelogSection, fixLeakedJsx bool) (pageFacts, error) {
	src := raw
	if fixLeakedJsx {
		var err error
		src, err = applyJsxOverrides(name, src)
		if err != nil {
			return pageFacts{}, err
		}
	}
	if dropCodeTabs {
		src = withoutCodeTabs(src)
	}
	if dropInstallSection {
		src = withoutInstallSection(src)
	}
	if dropRtlMigrate {
		src = withoutRtlMigrate(src)
	}
	if dropUsageSection {
		src = withoutUsageSection(src)
	}
	if dropCodeTabs {
		src = stripImportsFromMixedFences(dropReactImportFences(src))
	}
	if dropCompositionSection {
		src = withoutCompositionSection(src)
	}
	if dropApiReferenceSection {
		src = withoutApiReferenceSection(name, src)
	}
	if dropChangelogSection {
		src = withoutChangelogSection(src)
	}
	if s := locateMessageScrollerJsSpan(fenceShadow(src)); s != nil {
		src = replaceSpan(src, *s, messageScrollerJsNote())
	}
	if fixLeakedJsx {
		var err error
		src, err = rewriteLeakedJsxFences(name, src)
		if err != nil {
			return pageFacts{}, err
		}
		src, err = rewriteInlineJsxMentions(name, src)
		if err != nil {
			return pageFacts{}, err
		}
	}
	body := fenceShadow(src)
	noInlineCode := reInlineCode.ReplaceAllStringFunc(body, func(m string) string {
		return strings.Repeat(" ", len(m))
	})
	type posHead struct {
		at    int
		depth int
		text  string
	}
	var heads []posHead
	for _, m := range reMdHeading.FindAllStringSubmatchIndex(body, -1) {
		heads = append(heads, posHead{m[0], m[2] - m[1], stripInlineCode(mdText(body[m[4]:m[5]]))})
	}
	for _, h := range findAllRawHeadings(noInlineCode) {
		heads = append(heads, posHead{h.at, h.depth, htmlText(h.text)})
	}
	sort.SliceStable(heads, func(i, j int) bool { return heads[i].at < heads[j].at })
	var f pageFacts
	f.frontmatter = parseFrontmatter(raw)
	for _, h := range heads {
		f.headings = append(f.headings, headingEnt{h.depth, h.text})
	}
	for _, m := range reCompPreview.FindAllStringSubmatch(body, -1) {
		f.previews = append(f.previews, previewEnt{attrOfJS(m[1], "name"), attrOfJS(m[1], "styleName"), attrOfJS(m[1], "direction")})
	}
	for _, m := range reCompSource.FindAllStringSubmatch(body, -1) {
		f.sources = append(f.sources, sourceEnt{attrOfJS(m[1], "name"), attrOfJS(m[1], "src")})
	}
	f.fences = scanFences(src)
	return f, nil
}

// mdPageFacts reads the facts the CONTENT transform is responsible for out of
// the built markdown (the TOC/pager/breadcrumb are VitePress's now).
type mdFacts struct {
	text     string
	h1       string
	lead     string
	headings []headingEnt
	previews []struct{ name, status, src string }
	fences   []fenceInfo
	chips    []attrPair
	iframes  []string
}

var (
	reMdH1        = regexp.MustCompile(`(?m)^# (.+)$`)
	reFrontmatterBlock = regexp.MustCompile(`(?s)^---\n.*?\n---\n`)
	reDemoIframe  = regexp.MustCompile(`<iframe class="demo" src="([^"]*)" title="([^"]*)" data-status="([^"]*)"`)
	reDemoMissing = regexp.MustCompile(`<div class="demo-missing" data-demo="([^"]*)" data-status="([^"]*)"`)
	reDemoInOrder = regexp.MustCompile(`<iframe class="demo"[^>]*title="([^"]*)"|<div class="demo-missing" data-demo="([^"]*)"`)
	rePageLinksP  = regexp.MustCompile(`(?s)<p class="page-links">(.*?)</p>`)
	reAnchor      = regexp.MustCompile(`<a href="([^"]*)"[^>]*>([^<]*)</a>`)
	reDocsHref    = regexp.MustCompile(`\]\((\/docs\/[^)]*)\)`)
	reAllHref     = regexp.MustCompile(`\]\(([^)]*)\)`)
)

func mdPageFacts(md string) mdFacts {
	front := parseFrontmatter(md)
	body := reFrontmatterBlock.ReplaceAllString(md, "")
	h1 := ""
	if m := reMdH1.FindStringSubmatch(body); m != nil {
		h1 = m[1]
	}
	afterH1 := ""
	if i := strings.Index(body, "# "+h1); i >= 0 {
		afterH1 = body[i+len(h1)+3:]
	} else {
		afterH1 = body
	}
	shadow := fenceShadow(afterH1)
	noInlineCode := reInlineCode.ReplaceAllStringFunc(shadow, func(m string) string {
		return strings.Repeat(" ", len(m))
	})
	type posHead struct {
		at    int
		depth int
		text  string
	}
	var heads []posHead
	for _, m := range reMdHeading.FindAllStringSubmatchIndex(shadow, -1) {
		d := m[2] - m[1]
		heads = append(heads, posHead{m[0], d, stripInlineCode(mdText(shadow[m[4]:m[5]]))})
	}
	for _, h := range findAllRawHeadings(noInlineCode) {
		heads = append(heads, posHead{h.at, h.depth, htmlText(h.text)})
	}
	sort.SliceStable(heads, func(i, j int) bool { return heads[i].at < heads[j].at })
	var f mdFacts
	f.text = body
	f.h1 = mdText(h1)
	f.lead = mdText(fmString(front, "description"))
	for _, h := range heads {
		f.headings = append(f.headings, headingEnt{h.depth, h.text})
	}
	byName := map[string]struct{ name, status, src string }{}
	for _, m := range reDemoIframe.FindAllStringSubmatch(afterH1, -1) {
		byName[m[2]] = struct{ name, status, src string }{m[2], m[3], m[1]}
	}
	for _, m := range reDemoMissing.FindAllStringSubmatch(afterH1, -1) {
		byName[m[1]] = struct{ name, status, src string }{m[1], m[2], ""}
	}
	for _, m := range reDemoInOrder.FindAllStringSubmatch(afterH1, -1) {
		n := m[1]
		if n == "" {
			n = m[2]
		}
		if e, ok := byName[n]; ok {
			f.previews = append(f.previews, struct{ name, status, src string }{e.name, e.status, e.src})
		}
	}
	f.fences = scanFences(afterH1)
	if m := rePageLinksP.FindStringSubmatch(afterH1); m != nil {
		for _, a := range reAnchor.FindAllStringSubmatch(m[1], -1) {
			f.chips = append(f.chips, attrPair{a[2], a[1]})
		}
	}
	for _, m := range reDemoIframe.FindAllStringSubmatch(afterH1, -1) {
		f.iframes = append(f.iframes, m[1])
	}
	return f
}

// ---- comparison ---------------------------------------------------------------

var knownStatuses = map[string]bool{
	"existing-dist": true, "authored": true, "unavailable": true,
	"to-author": true, "tombstoned": true, "unknown": true,
}

// comparePage returns the fidelity issues for one page.
func comparePage(m pageFacts, h mdFacts, pageName string, isComponentPage bool, expectedManualRef string) []string {
	var issues []string
	issue := func(kind, detail string) { issues = append(issues, kind+": "+detail) }

	wantTitle := fmString(m.frontmatter, "title")
	if wantTitle == "" {
		wantTitle = pageName
	}
	if h.h1 != mdText(wantTitle) {
		issue("h1", fmt.Sprintf("built=%q mdx=%q", h.h1, mdText(wantTitle)))
	}
	wantLead := fmString(m.frontmatter, "description")
	if h.lead != mdText(wantLead) {
		issue("lead", fmt.Sprintf("built=%.70q mdx=%.70q", h.lead, mdText(wantLead)))
	}
	wantHeads := fmt.Sprint(m.headings)
	gotHeads := fmt.Sprint(h.headings)
	if wantHeads != gotHeads {
		issue("headings", fmt.Sprintf("mdx %.140q != built %.140q", wantHeads, gotHeads))
	}
	var wantPrev, gotPrev []string
	for _, p := range m.previews {
		wantPrev = append(wantPrev, p.name)
	}
	for _, p := range h.previews {
		gotPrev = append(gotPrev, p.name)
	}
	if fmt.Sprint(wantPrev) != fmt.Sprint(gotPrev) {
		issue("previews", fmt.Sprintf("mdx %.120q != built %.120q", fmt.Sprint(wantPrev), fmt.Sprint(gotPrev)))
	}
	for _, p := range h.previews {
		if !knownStatuses[p.status] {
			issue("preview-status", fmt.Sprintf("%s: status %q not emitted by the catalog", p.name, p.status))
		}
		if p.status == "unknown" && isComponentPage {
			issue("preview-status", fmt.Sprintf("%s: unknown status on a component page (radix catalog must be complete)", p.name))
		}
	}
	for _, f := range m.fences {
		want := mdText(f.content)
		if want == "" {
			continue
		}
		found := false
		for _, p := range h.fences {
			if strings.Contains(mdText(p.content), want) {
				found = true
				break
			}
		}
		if !found {
			issue("fence", fmt.Sprintf("[%s] %.80q has no matching fence in the built page", f.lang, want))
		}
	}
	// The mdx frontmatter `links:` chips are a DECLARED drop, not a mirror:
	// buildPage stops emitting them (see the comment there — React prop-table
	// deep links, duplicated in-body with context, two of them dead). This
	// used to compare mdx links against the built chips; it now asserts the
	// built page has none, so re-introducing them fails here rather than
	// silently putting "doc · api" back on 28 pages.
	if len(h.chips) > 0 {
		var got []string
		for _, c := range h.chips {
			got = append(got, c.K+"→"+c.V)
		}
		sort.Strings(got)
		issue("chips", "built page still carries page-links chips: "+strings.Join(got, " | "))
	}
	if expectedManualRef != "" && !strings.Contains(h.text, expectedManualRef) {
		issue("manual-tab", "rewritten manual tab never mentions "+expectedManualRef)
	}
	// Two assertions that read ONLY the built page. Everything above compares
	// the build against a re-run of the same transforms, so a bug inside a
	// shared transform is invisible to it — rewriteJsxTagsInLine used to carry
	// JSX attributes over verbatim, and both sides agreed on the wrong answer.
	for _, f := range h.fences {
		// html fences only: those are the hand-written and rewritten ones.
		// The demo code-groups are `text:line-numbers` and hold markup copied
		// verbatim out of the shipped demo page, so they cannot carry a prop
		// the artifacts do not — checking them would only re-assert the demo.
		if f.lang != "html" {
			continue
		}
		for _, prop := range reactPropsInMarkup(f.content) {
			issue("react-prop", "html fence carries the JSX prop "+prop+
				" on a data-slot element — no stylesheet selects it; it must be a data attribute or be dropped")
		}
	}
	if m := reReactPropTable.FindString(h.text); m != "" {
		issue("react-prop-table", "built page keeps an upstream React props table: "+strings.TrimSpace(m))
	}
	return issues
}

var (
	// A `| Prop | Type | …` header row — the exact shape shadcn's per-component
	// React prop docs take, and meaningless for markup you copy.
	reReactPropTable = regexp.MustCompile("(?m)^\\|\\s*`?Prop`?\\s*\\|.*$")
	// Opening tags that carry a data-slot: shadless markup, so every attribute
	// on them should be HTML or data-*/aria-*.
	reSlotOpenTag = regexp.MustCompile(`<[a-z][\w-]*\b[^<>]*\bdata-slot="[^"]*"[^<>]*>`)
	// name="value" / name='value' / bare name — matching the value too is what
	// keeps words inside a quoted value from being read as attribute names.
	reTagAttrName = regexp.MustCompile(`\s([A-Za-z][\w:.-]*)(?:=(?:"[^"]*"|'[^']*'|[^\s>]*))?`)
)

// reactPropsInMarkup names every attribute on a data-slot element that is
// neither a plain HTML attribute nor data-*/aria-* — asChild, htmlFor, an
// event handler, or a cva axis left in its JSX spelling.
func reactPropsInMarkup(fence string) []string {
	var out []string
	seen := map[string]bool{}
	for _, tag := range reSlotOpenTag.FindAllString(fence, -1) {
		body := tag
		if i := strings.IndexAny(body, " \t"); i >= 0 {
			body = body[i:]
		}
		body = strings.TrimSuffix(strings.TrimSuffix(strings.TrimSpace(body), ">"), "/")
		for _, m := range reTagAttrName.FindAllStringSubmatch(" "+body, -1) {
			name := m[1]
			lower := strings.ToLower(name)
			if strings.HasPrefix(lower, "data-") || strings.HasPrefix(lower, "aria-") || htmlAttrs[lower] {
				continue
			}
			if !seen[name] {
				seen[name] = true
				out = append(out, name)
			}
		}
	}
	sort.Strings(out)
	return out
}

// fidelityRawMDX is set by the fidelity driver per page (chips compare needs
// the raw mdx for ordered links). A package-level hand-off keeps comparePage's
// signature identical to the JS.
var fidelityRawMDX string

// rawHeading is one <hN>…</hN> hit (JS used a \1 backreference RE2 lacks).
type rawHeading struct {
	at, depth int
	text      string
}

func findAllRawHeadings(s string) []rawHeading {
	var out []rawHeading
	off := 0
	for {
		i := reHOpen.FindStringSubmatchIndex(s[off:])
		if i == nil {
			break
		}
		depth := int(s[off+i[0]+2] - '0')
		open := off + i[1]
		close := "</h" + s[off+i[0]+2:off+i[0]+3] + ">"
		j := strings.Index(s[open:], close)
		if j < 0 {
			break
		}
		out = append(out, rawHeading{off + i[0], depth, s[open : open+j]})
		off = open + j + len(close)
	}
	return out
}
