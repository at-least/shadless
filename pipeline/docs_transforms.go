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
	"fmt"
	"regexp"
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

// reApiRefLeak: a markdown table row or a `### ` subheading inside the
// section — the shape shadcn's own per-subcomponent React prop docs always
// take (verified against every upstream radix/*.mdx: no section has a fence
// without one of these too). A section without either is prose only (e.g.
// "See the Radix UI docs…") and has nothing React-specific to replace.
var reApiRefLeak = regexp.MustCompile(`(?m)^\s*\|.*\||^### `)

// locateApiReferenceSpan: `## API Reference` … next `## ` heading or EOF —
// ONLY for components with no family/trivial behavior-protocol entry
// (docs_families.go) AND whose section actually contains leaked React
// content (reApiRefLeak). Components with a family/trivial entry, and
// uncovered components whose upstream section is just a link to the Radix
// docs, are left in place (nil here, same as always) — those aren't the
// wrapper components' own React prop tables/JSX this transform targets.
func locateApiReferenceSpan(comp string, shadow string) *span {
	if _, ok := trivial[comp]; ok {
		return nil
	}
	if _, ok := family[comp]; ok {
		return nil
	}
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

// ---- declared prose adjustments ---------------------------------------------

type textOp struct{ find, replace string }

type textAdjustment struct {
	id    string
	files []string
	note  string
	ops   []textOp
}

// TEXT_ADJUSTMENTS: prose rewrites where upstream mdx claims React-library
// specifics that are false for the no-React product.
var textAdjustments = []textAdjustment{
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
	{
		id:    "avatar-props-prose",
		files: []string{"avatar.mdx"},
		note:  "shadless Avatar slots are plain HTML elements driven by the runtime — 'accepts all Radix UI props' is false here",
		ops: []textOp{
			{
				find:    "It accepts all Radix UI Avatar Image props.",
				replace: "It is a plain `<img data-slot=\"avatar-image\">` — the shadless runtime switches to the fallback from its load state.",
			},
			{
				find:    "It accepts all Radix UI Avatar Fallback props.",
				replace: "It is a plain `<span data-slot=\"avatar-fallback\">` shown by the shadless runtime while the image is loading or failed.",
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
