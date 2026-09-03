package main

// docs transforms, ported from src/docs/transforms.mjs — the SINGLE SOURCE
// for where each mdx section transform touches the raw source. Two consumers
// must agree on the span: the builder (docs-build replaces it) and the gate
// (docs-fidelity drops it). If someone re-inlines a locator on either side,
// the contract tests fail.
//
// The JS regexes with lookaheads ((?:(?!```)[\s\S])*?, ^## (?!Usage$)) have
// no RE2 equivalent; those two matchers are hand-rolled scanners below with
// the same semantics.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// fenceShadow blanks fenced regions (newlines and offsets preserved) so span
// searches see only prose/markup. Line-based: 3+ backtick fences, info
// strings, unclosed fences blank to EOF.
func fenceShadow(text string) string {
	lines := strings.Split(text, "\n")
	open := -1
	for i, line := range lines {
		if open < 0 {
			if reFenceOpen.MatchString(line) {
				open = len(reLeadingBackticks.FindString(line))
				lines[i] = blankLine(line)
			}
		} else {
			if n := len(reLeadingBackticks.FindString(line)); n >= open {
				open = -1
			}
			lines[i] = blankLine(line)
		}
	}
	return strings.Join(lines, "\n")
}

var (
	reFenceOpen       = regexp.MustCompile("^`{3,}")
	reLeadingBackticks = regexp.MustCompile("^`+")
)

func blankLine(s string) string { return strings.Repeat(" ", len(s)) }

// span is a [start, end) byte range into the ORIGINAL text (the caller
// locates on the shadow and splices on the real string).
type span struct{ start, end int }

// locateCodeTabsSpans: every <CodeTabs>…</CodeTabs> outside fences. Malformed
// (no close) stops the scan — the builder throws unless exactly one exists.
func locateCodeTabsSpans(shadow string) []span {
	var spans []span
	off := 0
	for {
		i := strings.Index(shadow[off:], "<CodeTabs>")
		if i < 0 {
			break
		}
		start := off + i
		close := strings.Index(shadow[start:], "</CodeTabs>")
		if close < 0 {
			break
		}
		end := start + close + len("</CodeTabs>")
		spans = append(spans, span{start, end})
		off = end
	}
	return spans
}

// locateInstallSection: the `## Installation` … `## Usage` span in utils
// guides; end = the '#' of "## Usage". Nil when absent/malformed.
func locateInstallSection(shadow string) *span {
	open := reH2Installation.FindStringIndex(shadow)
	if open == nil {
		return nil
	}
	next := reH2Usage.FindStringIndex(shadow)
	if next == nil || next[0] <= open[0] {
		return nil
	}
	return &span{open[0], next[0]}
}

var (
	reH2Installation = regexp.MustCompile(`(?m)^## Installation$`)
	reH2Usage        = regexp.MustCompile(`(?m)^## Usage$`)
)

// locateRtlMigrateSpan: `## Migrating existing components` … </Steps>.
func locateRtlMigrateSpan(shadow string) *span {
	open := reH2Migrating.FindStringIndex(shadow)
	if open == nil {
		return nil
	}
	close := strings.Index(shadow[open[0]:], "</Steps>")
	if close < 0 {
		return nil
	}
	return &span{open[0], open[0] + close + len("</Steps>")}
}

var reH2Migrating = regexp.MustCompile(`(?m)^## Migrating existing components$`)

// locateUsageSpan: `## Usage` … next `## ` heading (any heading that is not
// Usage itself). The JS regex /^## (?!Usage$)/m needs a lookahead — scanned
// by line instead.
func locateUsageSpan(shadow string) *span {
	open := reH2Usage.FindStringIndex(shadow)
	if open == nil {
		return nil
	}
	after := shadow[open[0]+len("## Usage"):]
	off := 0
	for _, line := range strings.Split(after, "\n") {
		if strings.HasPrefix(line, "## ") && strings.TrimSpace(line) != "## Usage" {
			return &span{open[0], open[0] + len("## Usage") + off}
		}
		off += len(line) + 1
	}
	return nil
}

// locateCompositionSpan: `## Composition` … next non-Composition `## `.
func locateCompositionSpan(shadow string) *span {
	open := reH2Composition.FindStringIndex(shadow)
	if open == nil {
		return nil
	}
	after := shadow[open[0]+len("## Composition"):]
	off := 0
	for _, line := range strings.Split(after, "\n") {
		if strings.HasPrefix(line, "## ") && strings.TrimSpace(line) != "## Composition" {
			return &span{open[0], open[0] + len("## Composition") + off}
		}
		off += len(line) + 1
	}
	return nil
}

var reH2Composition = regexp.MustCompile(`(?m)^## Composition$`)

// locateChangelogSpan: `## Changelog` … EOF. Upstream's Changelog sections
// are React/shadcn-CLI upgrade diffs for a previous version of the
// component's own source — shadless has no "previous generated copy" to
// patch (it always mirrors the current pinned registry), so the section is
// dropped outright rather than translated. Unlike Composition/Usage, the
// builder drops the heading too — there's no vanilla content to put there.
func locateChangelogSpan(shadow string) *span {
	open := reH2Changelog.FindStringIndex(shadow)
	if open == nil {
		return nil
	}
	return &span{open[0], len(shadow)}
}

var reH2Changelog = regexp.MustCompile(`(?m)^## Changelog$`)

// locateHeadingRangeSpan: `## fromHeading` … `## toHeading` (exclusive of
// the closing heading). nil if either heading is absent or out of order.
func locateHeadingRangeSpan(shadow, fromHeading, toHeading string) *span {
	openRe := regexp.MustCompile(`(?m)^## ` + regexp.QuoteMeta(fromHeading) + `$`)
	closeRe := regexp.MustCompile(`(?m)^## ` + regexp.QuoteMeta(toHeading) + `$`)
	open := openRe.FindStringIndex(shadow)
	if open == nil {
		return nil
	}
	close := closeRe.FindStringIndex(shadow[open[1]:])
	if close == nil {
		return nil
	}
	return &span{open[0], open[1] + close[0]}
}

// message-scroller ships as a pure static component (tier "static" —
// generated/ir/message-scroller.json — no dist/js file at all). Its
// upstream "Core Concepts"/"Performance"/"Virtualization" sections document
// React-only imperative behavior (data-* attribute tracking driven by
// hooks, useMessageScrollerVisibility, scrollToMessage) that shadless does
// not port — there is no vanilla shape to translate the examples to, so the
// span is dropped and replaced with one honest paragraph instead of being
// fence-by-fence rewritten.
func locateMessageScrollerJsSpan(shadow string) *span {
	return locateHeadingRangeSpan(shadow, "Core Concepts", "Accessibility")
}

func messageScrollerJsNote() string {
	return "## Core Concepts\n\nshadless's `message-scroller` is pure static markup and CSS — `generated/ir/message-scroller.json` reports tier `static`, and there is no `dist/js/message-scroller.js`. Upstream's virtualization, scroll anchoring, `useMessageScrollerVisibility`, and `scrollToMessage` are all React-only behavior this port does not include; build them yourself on top of the shipped markup if you need them.\n\n"
}

// reApiRefLeak: a markdown table row or a `### ` subheading inside the
// section — the shape shadcn's own per-subcomponent React prop docs always
// take (verified against every upstream radix/*.mdx: no section has a fence
// without one of these too). A section without either is prose only (e.g.
// "See the Radix UI docs…") and has nothing React-specific to replace.
var reApiRefLeak = regexp.MustCompile(`(?m)^\s*\|.*\||^### `)

// locateApiReferenceSpan: `## API Reference` … next `## ` heading or EOF —
// ONLY where the section actually contains leaked React content
// (reApiRefLeak). A section that is just a link to the Radix docs has nothing
// React-specific to replace and is left in place (nil here).
//
// The family/trivial components used to be excluded outright, on the
// assumption that their upstream section is only that Radix link. avatar and
// alert-dialog disproved it: both carried shadcn's own `| Prop | Type |`
// tables (avatar six of them, listing React `className` on every slot) right
// under the shadless surface table. The reApiRefLeak test is the real
// discriminator, so it is now the only one — verified against the whole
// registry: lifting the exclusion changed exactly those two pages.
func locateApiReferenceSpan(comp string, shadow string) *span {
	open := reApiRefHeading.FindStringIndex(shadow)
	if open == nil {
		return nil
	}
	after := shadow[open[1]:]
	off := 0
	end := len(shadow)
	for _, line := range strings.Split(after, "\n") {
		if strings.HasPrefix(line, "## ") {
			end = open[1] + off
			break
		}
		off += len(line) + 1
	}
	if !reApiRefLeak.MatchString(shadow[open[1]:end]) {
		return nil
	}
	return &span{open[0], end}
}

// dropReactImportFences removes pure React-import fences (content is ONLY
// import statements from @/components/ui/*). The JS pattern tempered the
// import match with (?:(?!```)[\s\S])*? so an import statement can never
// span a closing fence — the line scanner below has that property for free.
var (
	reImportFrom = regexp.MustCompile(`from\s+"[@/][^"]+"\s*;?\s*$`)
)

func dropReactImportFences(raw string) string {
	lines := strings.Split(raw, "\n")
	var out []string
	i := 0
	for i < len(lines) {
		if !strings.HasPrefix(lines[i], "```tsx") {
			out = append(out, lines[i])
			i++
			continue
		}
		// Match a pure import fence: one or more import statements, each
		// ending on a from "@/…" (or from "/…") line, then IMMEDIATELY the
		// closing fence — the JS pattern allows no blank line between the
		// statements and ``` (a blank can only appear inside a multi-line
		// import). Any "```" encountered while an import statement is open
		// fails the WHOLE match (the tempered (?:(?!```)…)).
		j := i + 1
		statements := 0
		failed := false
	stmtLoop:
		for j < len(lines) {
			if !strings.HasPrefix(lines[j], "import") {
				break
			}
			statements++
			// consume through the from-line, spanning multi-line imports
			for j < len(lines) && !reImportFrom.MatchString(lines[j]) {
				if strings.Contains(lines[j], "```") {
					failed = true
					break stmtLoop
				}
				j++
			}
			if j >= len(lines) {
				failed = true
				break
			}
			j++ // the from-line itself
		}
		if !failed && statements > 0 && j < len(lines) && strings.HasPrefix(lines[j], "```") {
			// the pattern's trailing \n? consumes exactly ONE newline after
			// the closing fence — in line terms: skip the closing line only,
			// never a following blank line.
			i = j + 1
			continue
		}
		out = append(out, lines[i])
		i++
	}
	return strings.Join(out, "\n")
}

// stripImportsFromMixedFences strips import statements from ```tsx fences
// that MIX imports with a JSX example, and renumbers shiki {1,4-6} highlight
// refs past the removed lines. Imports-only fences drop entirely.
var reTsxMeta = regexp.MustCompile("(?s)```tsx([^\\n]*)\\n(.*?)```")

func stripImportsFromMixedFences(raw string) string {
	return reTsxMeta.ReplaceAllStringFunc(raw, func(whole string) string {
		m := reTsxMeta.FindStringSubmatch(whole)
		meta, body := m[1], m[2]
		lines := strings.Split(body, "\n")
		removed := 0
		i := 0
		for i < len(lines) {
			l := lines[i]
			if strings.HasPrefix(l, "import") {
				removed++
				i++
				// NOTE: the JS loop is GREEDY here — for a single-line import
				// the inner while keeps consuming subsequent lines until a
				// from-line or EOF. That is load-bearing (bubble.mdx's mixed
				// fence loses its whole body to it); do not "fix" it.
				for i < len(lines) && !reImportFrom.MatchString(lines[i]) {
					removed++
					i++
				}
				if i < len(lines) {
					removed++
					i++
				}
				continue
			}
			if strings.TrimSpace(l) == "" && removed > 0 {
				removed++
				i++
				continue
			}
			break
		}
		if removed == 0 {
			return whole
		}
		kept := lines[removed:]
		if removed > 0 && removed <= len(lines) && strings.TrimSpace(lines[removed-1]) == "" {
			kept = lines[removed-1:]
		}
		if strings.TrimSpace(strings.Join(kept, "")) == "" {
			return "" // fence was imports-only
		}
		metaOut := shiftHighlightRefs(meta, removed)
		return "```tsx" + strings.TrimRight(metaOut, " \t") + "\n" + strings.Join(kept, "\n") + "```"
	})
}

// shiftHighlightRefs maps {1,4-6} → refs with `removed` subtracted, dropping
// refs into the removed range (the JS callback's semantics).
var reHighlightList = regexp.MustCompile(`\{([0-9,\s-]+)\}`)

func shiftHighlightRefs(meta string, removed int) string {
	return reHighlightList.ReplaceAllStringFunc(meta, func(m0 string) string {
		list := reHighlightList.FindStringSubmatch(m0)[1]
		var shifted []string
		for _, part := range strings.Split(list, ",") {
			part = strings.TrimSpace(part)
			if part == "" {
				continue
			}
			lohi := strings.SplitN(part, "-", 2)
			lo := atoiSafe(lohi[0])
			hi := (*int)(nil)
			if len(lohi) == 2 {
				hi = atoiSafe(lohi[1])
			}
			// JS semantics: a NaN endpoint keeps the part verbatim; a part
			// whose low end is inside the removed range drops; otherwise
			// both endpoints shift by -removed.
			if lo == nil || (len(lohi) == 2 && hi == nil) {
				shifted = append(shifted, part)
				continue
			}
			if *lo <= removed {
				continue // range[0] <= removed → null (dropped)
			}
			if hi != nil {
				shifted = append(shifted, fmt.Sprintf("%d-%d", *lo-removed, *hi-removed))
			} else {
				shifted = append(shifted, fmt.Sprintf("%d", *lo-removed))
			}
		}
		if len(shifted) == 0 {
			return ""
		}
		return "{" + strings.Join(shifted, ",") + "}"
	})
}

func atoiSafe(s string) *int {
	s = strings.TrimSpace(s)
	n := 0
	if s == "" {
		return nil
	}
	for _, c := range s {
		if c < '0' || c > '9' {
			return nil
		}
		n = n*10 + int(c-'0')
	}
	return &n
}

// ---- utility-guide JSX fences -------------------------------------------------
//
// shimmer.mdx and scroll-fade.mdx (docs/content, guides with a `util` field)
// document pure Tailwind utility classes — no component, no props, no logic
// — but upstream still shows every usage snippet as ```tsx with className,
// by convention rather than necessity. rewriteUtilityJsxFences converts
// those specific fences to plain HTML: fence lang tsx -> html, className=
// -> class=, and the JSX comment placeholder {/* ... */} -> an HTML
// comment. Shared by docs-build (rewrites the built page) and docs-fidelity
// (rewrites the mdx side before fact-extraction, so both sides compare
// equal fences) — same single-source-of-truth contract as the section
// locators above.

var (
	reTsxFenceOpen  = regexp.MustCompile("(?m)^```tsx\\b.*$")
	reClassNameAttr = regexp.MustCompile(`\bclassName=`)
	reJsxComment    = regexp.MustCompile(`\{/\*\s*(.*?)\s*\*/\}`)
)

// shimmerMarkerJsx / shimmerMarkerHtml: the one fence in shimmer.mdx that
// composes real components (Marker + Spinner), not a bare className on a
// <p>/<div> — className=/{/* */} alone can't fix fake JSX tags like
// <Marker>. Rewritten by hand against the real shipped markup
// (dist/components/marker.html: data-slot="marker"/"marker-icon"/
// "marker-content"; dist/components/spinner.html: a single
// data-slot="spinner" svg) rather than guessed.
const (
	shimmerMarkerJsx = "```tsx\n<Marker role=\"status\">\n  <MarkerIcon>\n    <Spinner />\n  </MarkerIcon>\n  <MarkerContent className=\"shimmer\">Thinking&hellip;</MarkerContent>\n</Marker>\n```"
	shimmerMarkerHtml = "```html\n<div data-slot=\"marker\" role=\"status\">\n  <span data-slot=\"marker-icon\"><!-- Spinner markup — see /components/spinner --></span>\n  <span data-slot=\"marker-content\" class=\"shimmer\">Thinking&hellip;</span>\n</div>\n```"
)

func rewriteUtilityJsxFences(slug, raw string) (string, error) {
	if slug == "shimmer" {
		if !strings.Contains(raw, shimmerMarkerJsx) {
			return "", fmt.Errorf("shimmer guide: the Marker+Spinner fence text moved — re-anchor rewriteUtilityJsxFences")
		}
		raw = strings.Replace(raw, shimmerMarkerJsx, shimmerMarkerHtml, 1)
	}
	lines := strings.Split(raw, "\n")
	inFence := false
	for i, line := range lines {
		if !inFence {
			if reTsxFenceOpen.MatchString(line) {
				lines[i] = "```html"
				inFence = true
			}
			continue
		}
		if strings.HasPrefix(line, "```") {
			inFence = false
			continue
		}
		line = reClassNameAttr.ReplaceAllString(line, "class=")
		line = reJsxComment.ReplaceAllStringFunc(line, func(m string) string {
			return "<!-- " + reJsxComment.FindStringSubmatch(m)[1] + " -->"
		})
		lines[i] = line
	}
	return strings.Join(lines, "\n"), nil
}

// ---- leaked upstream JSX fences (component pages) ----------------------------
//
// Only 4 sections of each upstream mdx get replaced outright (Installation,
// Usage, Composition, API Reference — see docsBuildCtx.componentTransform).
// Everything else (Sizes/Spacing/Orientation/API/Events/… on carousel.mdx,
// the per-variant examples on avatar/item/field/…) is prose the builder
// never touches, and upstream illustrates it with real React JSX. This pass
// runs LAST, after the four handled sections and any hand overrides
// (docs_overrides.go) are already applied — anything left is by definition
// a leak. Every component tag is resolved against the real artifacts
// (generated/ir/*.json for the fn→slot mapping, dist/components/*.html for
// the slot's real tag — never guessed) so the rewritten markup is something
// the pipeline actually ships. A fence this can't resolve mechanically
// (react hooks, framework imports, made-up props) fails loud, naming the
// exact leaked text — that's the cue to add a docs_overrides.go entry, the
// same exact-anchor pattern as rewriteUtilityJsxFences's Marker+Spinner one.

var (
	reLeakedFenceOpen = regexp.MustCompile("(?m)^```(tsx|jsx|ts)\\b.*$")
	reJsxTag          = regexp.MustCompile(`</?([A-Z][A-Za-z0-9]*)\b[^<>]*?(/?)>`)
	reIconFn          = regexp.MustCompile(`^[A-Z][A-Za-z0-9]*Icon$`)
	reCamelWord       = regexp.MustCompile(`[A-Z][a-z0-9]*`)
	reDataSlot        = regexp.MustCompile(`<([a-zA-Z][\w-]*)\b[^>]*\bdata-slot="([^"]+)"`)
	reInlineJsxMention = regexp.MustCompile("`</?([A-Z][A-Za-z0-9]*)[^`]*`")
)

type jsxTagInfo struct {
	tag, slot string
	// axes: the cva axis names declared by the IR file this fn comes from,
	// each mapped to its declared values. A JSX prop naming one of these is
	// a data attribute in the shipped markup (`variant="ghost"` →
	// `data-variant="ghost"`), and the value set is what makes a bogus one
	// (`orientation="vertical | horizontal"`) fail instead of shipping.
	axes map[string][]string
}

// htmlAttrs: plain HTML attributes a JSX tag may legitimately carry through
// to the rewritten markup unchanged. Anything outside this set that is not a
// cva axis (asChild, onClick, a made-up prop) has no vanilla meaning and
// fails loud — the cue for a docs_overrides.go entry.
var htmlAttrs = map[string]bool{
	"class": true, "id": true, "href": true, "src": true, "srcset": true,
	"alt": true, "title": true, "role": true, "type": true, "name": true,
	"value": true, "placeholder": true, "disabled": true, "checked": true,
	"hidden": true, "target": true, "rel": true, "style": true, "lang": true,
	"dir": true, "width": true, "height": true, "for": true, "tabindex": true,
	"colspan": true, "rowspan": true, "action": true, "method": true,
	"autocomplete": true, "required": true, "readonly": true, "multiple": true,
	"selected": true, "min": true, "max": true, "step": true, "rows": true,
	"cols": true, "loading": true, "open": true, "controls": true,
	"poster": true, "download": true, "autofocus": true, "maxlength": true,
}

var reJsxTagAttr = regexp.MustCompile(`([A-Za-z][\w:.-]*)(?:="([^"]*)")?`)

var jsxTagIndexCache map[string]jsxTagInfo

// loadJsxTagIndex maps a JSX fn name (as upstream mdx spells it, e.g.
// "CarouselItem") to the real tag + data-slot the shipped markup uses for
// that slot. Built once from generated/ir (fn → slot) and dist/components
// (slot → tag, the majority tag across every page that emits the slot —
// polymorphic slots like button/field-label render more than one tag).
func loadJsxTagIndex() (map[string]jsxTagInfo, error) {
	if jsxTagIndexCache != nil {
		return jsxTagIndexCache, nil
	}
	fnSlot := map[string]string{}
	fnAxes := map[string]map[string][]string{}
	ents, err := os.ReadDir("generated/ir")
	if err != nil {
		return nil, err
	}
	for _, e := range ents {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		b, err := os.ReadFile(filepath.Join("generated/ir", e.Name()))
		if err != nil {
			return nil, err
		}
		var ir cssIrComponent
		if json.Unmarshal(b, &ir) != nil {
			continue
		}
		// The axis union of this IR file. cva tables are per-file (a page's
		// itemVariants + itemMediaVariants), and upstream mdx spells the prop
		// the same on every fn the file exports, so the union is the right
		// granularity here.
		axes := map[string][]string{}
		for _, key := range ir.Cva.keys {
			table := ir.Cva.tables[key]
			for _, ax := range table.axisOrder {
				axes[ax] = append(axes[ax], table.valueOrder[ax]...)
			}
		}
		for _, c := range ir.Components {
			if c.Fn == "" {
				continue
			}
			if _, exists := fnSlot[c.Fn]; exists {
				continue
			}
			for _, el := range c.Elements {
				if el.Slot != "" {
					fnSlot[c.Fn] = el.Slot
					if len(axes) > 0 {
						fnAxes[c.Fn] = axes
					}
					break
				}
			}
		}
	}
	slotTagCount := map[string]map[string]int{}
	dents, err := os.ReadDir("dist/components")
	if err != nil {
		return nil, err
	}
	for _, e := range dents {
		if !strings.HasSuffix(e.Name(), ".html") {
			continue
		}
		b, err := os.ReadFile(filepath.Join("dist/components", e.Name()))
		if err != nil {
			return nil, err
		}
		for _, m := range reDataSlot.FindAllStringSubmatch(string(b), -1) {
			tag, slot := m[1], m[2]
			if slotTagCount[slot] == nil {
				slotTagCount[slot] = map[string]int{}
			}
			slotTagCount[slot][tag]++
		}
	}
	idx := map[string]jsxTagInfo{}
	for fn, slot := range fnSlot {
		counts := slotTagCount[slot]
		if len(counts) == 0 {
			continue
		}
		var tags []string
		for t := range counts {
			tags = append(tags, t)
		}
		sort.Strings(tags)
		bestTag, bestN := "", -1
		for _, t := range tags {
			if counts[t] > bestN {
				bestTag, bestN = t, counts[t]
			}
		}
		idx[fn] = jsxTagInfo{tag: bestTag, slot: slot, axes: fnAxes[fn]}
	}
	jsxTagIndexCache = idx
	return idx, nil
}

// kebabIconName: "ChevronLeftIcon" -> "chevron-left" — matches lucide's own
// kebab-case icon ids, which is what dist markup's `lucide-<id>` class uses.
func kebabIconName(fn string) string {
	base := strings.TrimSuffix(fn, "Icon")
	words := reCamelWord.FindAllString(base, -1)
	for i, w := range words {
		words[i] = strings.ToLower(w)
	}
	return strings.Join(words, "-")
}

// rewriteJsxAttrs ports one JSX tag's attribute text to markup. A cva axis
// becomes its data attribute (`variant="ghost"` → `data-variant="ghost"`,
// which is what dist/css/<name>.css actually selects on); a plain HTML
// attribute passes through; anything else — asChild, an event handler, a
// prop that never reaches the DOM — has no markup form and fails loud, as
// does an axis value the component does not declare.
//
// Before this existed the whole attribute text was carried over verbatim, so
// pages shipped `<button data-slot="button" variant="ghost">`: valid-looking
// HTML that no stylesheet matches, i.e. silently unstyled when copied.
func rewriteJsxAttrs(fn, attrs string, info jsxTagInfo) (string, error) {
	rest := strings.TrimSpace(attrs)
	if rest == "" {
		return "", nil
	}
	var out []string
	for _, m := range reJsxTagAttr.FindAllStringSubmatch(rest, -1) {
		name, val := m[1], m[2]
		hasVal := strings.Contains(m[0], "=")
		lower := strings.ToLower(name)
		// React's two renames of real HTML attributes. className is already
		// handled a step earlier (reClassNameAttr); htmlFor has no other form.
		if name == "htmlFor" {
			name, lower = "for", "for"
			m[0] = `for="` + val + `"`
		}
		switch {
		case strings.HasPrefix(lower, "data-"), strings.HasPrefix(lower, "aria-"):
			out = append(out, m[0])
		case htmlAttrs[lower]:
			out = append(out, m[0])
		case info.axes[name] != nil:
			if !hasVal {
				return "", fmt.Errorf("JSX prop %s on <%s> has no value", name, fn)
			}
			if !containsTok(info.axes[name], val) {
				return "", fmt.Errorf("JSX prop %s=%q on <%s> is not a declared value (%s)",
					name, val, fn, strings.Join(info.axes[name], ", "))
			}
			out = append(out, `data-`+name+`="`+val+`"`)
		default:
			return "", fmt.Errorf("JSX prop %s on <%s> has no markup form", name, fn)
		}
	}
	if len(out) == 0 {
		return "", nil
	}
	return " " + strings.Join(out, " "), nil
}

// rewriteJsxTagsInLine mechanically rewrites JSX tags on one fence line.
// Returns an error naming the exact unresolved tag when the fn is neither a
// known icon nor in the jsx tag index.
func rewriteJsxTagsInLine(line string, idx map[string]jsxTagInfo) (string, error) {
	var outErr error
	out := reJsxTag.ReplaceAllStringFunc(line, func(m string) string {
		if outErr != nil {
			return m
		}
		sub := reJsxTag.FindStringSubmatch(m)
		fn, selfClose := sub[1], sub[2] == "/"
		closing := strings.HasPrefix(m, "</")
		if reIconFn.MatchString(fn) {
			return `<!-- lucide "` + kebabIconName(fn) + `" icon -->`
		}
		info, ok := idx[fn]
		if !ok {
			outErr = fmt.Errorf("unmapped JSX tag <%s>", fn)
			return m
		}
		if closing {
			return "</" + info.tag + ">"
		}
		attrs := m[len("<"+fn):]
		if selfClose {
			attrs = strings.TrimSuffix(attrs, "/>")
		} else {
			attrs = strings.TrimSuffix(attrs, ">")
		}
		attrs = strings.TrimRight(attrs, " ")
		attrs, err := rewriteJsxAttrs(fn, attrs, info)
		if err != nil {
			outErr = err
			return m
		}
		open := "<" + info.tag + ` data-slot="` + info.slot + `"` + attrs
		if selfClose {
			return open + " />"
		}
		return open + ">"
	})
	if outErr != nil {
		return "", outErr
	}
	return out, nil
}

// rewriteLeakedJsxFences runs after the four handled sections and any hand
// overrides. Every remaining ```tsx/```jsx/```ts fence is upstream
// illustrating a prop or composition with real React JSX; this mechanically
// ports what it can (className→class, {/* */}→<!-- -->, known JSX tags →
// real tag+data-slot, IconFn → a lucide-id comment) and fails loud, naming
// the exact fence, on anything it can't (react hooks, `{expr}` JS left
// over, an unmapped tag) — those need a docs_overrides.go entry.
func rewriteLeakedJsxFences(page, raw string) (string, error) {
	idx, err := loadJsxTagIndex()
	if err != nil {
		return "", err
	}
	lines := strings.Split(raw, "\n")
	var out []string
	inFence := false
	fenceStart := 0
	for i, line := range lines {
		if !inFence {
			if reLeakedFenceOpen.MatchString(line) {
				inFence = true
				fenceStart = i
				out = append(out, "```html")
				continue
			}
			out = append(out, line)
			continue
		}
		if strings.HasPrefix(line, "```") {
			inFence = false
			out = append(out, line)
			continue
		}
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "import ") || strings.HasPrefix(trimmed, "import{") {
			return "", fmt.Errorf("%s: leaked fence at line %d has a React import — needs a docs_overrides.go entry: %q", page, fenceStart+1, trimmed)
		}
		line = reClassNameAttr.ReplaceAllString(line, "class=")
		line = reJsxComment.ReplaceAllStringFunc(line, func(m string) string {
			return "<!-- " + reJsxComment.FindStringSubmatch(m)[1] + " -->"
		})
		rewritten, err := rewriteJsxTagsInLine(line, idx)
		if err != nil {
			return "", fmt.Errorf("%s: leaked fence at line %d: %v — needs a docs_overrides.go entry: %q", page, fenceStart+1, err, trimmed)
		}
		if strings.ContainsAny(rewritten, "{}") {
			return "", fmt.Errorf("%s: leaked fence at line %d still has a JS expression — needs a docs_overrides.go entry: %q", page, fenceStart+1, trimmed)
		}
		out = append(out, rewritten)
	}
	return strings.Join(out, "\n"), nil
}

// rewriteInlineJsxMentions handles the prose form ("the `<CarouselItem />`")
// — same tag index, but a single-backtick span becomes a single-backtick
// data-slot mention instead of a fence line.
func rewriteInlineJsxMentions(page, raw string) (string, error) {
	idx, err := loadJsxTagIndex()
	if err != nil {
		return "", err
	}
	var outErr error
	out := reInlineJsxMention.ReplaceAllStringFunc(raw, func(m string) string {
		if outErr != nil {
			return m
		}
		sub := reInlineJsxMention.FindStringSubmatch(m)
		fn := sub[1]
		if reIconFn.MatchString(fn) {
			return "`<!-- lucide \"" + kebabIconName(fn) + "\" icon -->`"
		}
		info, ok := idx[fn]
		if !ok {
			outErr = fmt.Errorf("%s: unmapped inline JSX mention %s — needs a docs_overrides.go entry", page, m)
			return m
		}
		return "`data-slot=\"" + info.slot + "\"`"
	})
	if outErr != nil {
		return "", outErr
	}
	return out, nil
}

// ---- declared prose adjustments ---------------------------------------------

type textOp struct{ find, replace string }

type textAdjustment struct {
	id    string
	files []string
	note  string
	ops   []textOp
}

// TEXT_ADJUSTMENTS: prose rewrites where upstream mdx claims React-library
// specifics that are false for the no-React product — plus (carousel-embla)
// upstream links that no longer resolve at all.
var textAdjustments = []textAdjustment{
	{
		id:    "carousel-embla-doc-urls",
		files: []string{"carousel.mdx"},
		note:  "embla moved its docs under /docs/ — every deep link upstream still uses 404s (verified: GET with a browser UA returns embla's own PageNotFound component, so it is real rot, not bot-blocking)",
		ops: []textOp{
			{
				find:    "See the [Embla Carousel docs](https://www.embla-carousel.com/api/events/) for more information on using events.",
				replace: "See the [Embla Carousel docs](https://www.embla-carousel.com/docs/api/events) for more information on using events.",
			},
			{
				find:    "See the [Embla Carousel docs](https://www.embla-carousel.com/api/) for more information on props and plugins.",
				replace: "See the [Embla Carousel docs](https://www.embla-carousel.com/docs/api) for more information on props and plugins.",
			},
		},
	},
	{
		id:    "button-pointer-cli-prose",
		files: []string{"button.mdx"},
		note:  "shadless has no CLI — the init --pointer sentence is a shadcn-CLI instruction",
		ops: []textOp{
			{
				find:    "You can also enable this during project setup with `npx shadcn@latest init --pointer`.",
				replace: "In shadless just keep the CSS rule above — there is no CLI flag to set.",
			},
		},
	},
}

// applyTextAdjustments rewrites the declared ops for the named file. Throws
// (returns error) if a declared find string is missing — the adjustment must
// be re-anchored, never silently skipped.
func applyTextAdjustments(basename, raw string) (string, error) {
	out := raw
	for _, adj := range textAdjustments {
		if !containsTok(adj.files, basename) {
			continue
		}
		for _, op := range adj.ops {
			i := strings.Index(fenceShadow(out), op.find)
			if i < 0 {
				return "", fmt.Errorf("text adjustment %s: find string not present in %s — re-anchor against the new upstream prose", adj.id, basename)
			}
			out = out[:i] + op.replace + out[i+len(op.find):]
		}
	}
	return out, nil
}
