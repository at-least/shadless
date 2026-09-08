package main

// css-direction — inventory physical reading-direction utilities in the
// emitted CSS and hold them to a committed baseline. Ported from
// tools/css-direction-gate.mjs.
//
// Why a baseline instead of "fail on any physical utility": dist/shadless.css
// is a MECHANICAL conversion of the pinned upstream registry. Where upstream
// shadcn uses physical utilities, fidelity to the oracle REQUIRES emitting
// them — the gate's job is not to forbid them but to make drift loud: a new
// or disappeared physical utility means the upstream pin moved the RTL story,
// and that must be a visible decision at sync time, not a silent regression
// in the -rtl-* variant pages.
//
// Scan target: dist/shadless.css (the library's emitted source; hand-authored
// demo markup is out of scope). Runs after the demo css build so the scanned
// state is the shipped one (committed dist carries the overlay).
//
//	go test -C pipeline -run '^TestCssDirection$'   compare against baseline
//	pipeline css-direction --update                 re-record after review
//
// NOTE: nothing in this file may contain literal utility syntax. Tracked
// files are content-scanned by the tailwind CLI (see pipeline/tw.go) and a
// literal here would surface as a phantom rule in dist/out.css. That is why
// the baseline keys are written as CONCATENATED PAIRS — constant expressions,
// folded by the compiler, invisible to a content scan. Keep the pair shape.

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// Physical reading-direction utility SHAPES (the logical twins are fine).
// Matched against the utility segment AFTER stripping variant prefixes
// (responsive, has-*, named group, arbitrary-variant, pseudo-element).
var physicalPatterns = []*regexp.Regexp{
	regexp.MustCompile(`^m[lr]-`),             // physical margin start/end
	regexp.MustCompile(`^p[lr]-`),             // physical padding start/end
	regexp.MustCompile(`^s[lr]-`),             // physical scroll margin/padding
	regexp.MustCompile(`^(left|right)-`),      // positioning on the reading axis
	regexp.MustCompile(`^text-(left|right)$`), // alignment on the reading axis
	regexp.MustCompile(`^border-[lr]-\d`),     // physical horizontal border width
	regexp.MustCompile(`^rounded-[lr]-`),      // physical start/end corner radius
	regexp.MustCompile(`^(space|divide)-x`),   // physical sibling spacing
}

// Utilities that LOOK physical but are sanctioned (a11y semantics).
var sanctionedUtilities = map[string]bool{"sr" + "-only": true}

var applyBlock = regexp.MustCompile(`@apply([^;]+);`)

// utilitySegment strips variant prefixes: keep only the segment after the
// LAST ':'. Arbitrary values may contain ':' inside brackets — those never
// hold our utilities.
func utilitySegment(token string) string {
	if i := strings.LastIndex(token, ":"); i >= 0 {
		return token[i+1:]
	}
	return token
}

func isPhysicalUtility(token string) bool {
	u := utilitySegment(token)
	if sanctionedUtilities[u] {
		return false
	}
	for _, re := range physicalPatterns {
		if re.MatchString(u) {
			return true
		}
	}
	return false
}

// extractApplyTokens pulls every @apply token list out of the CSS source.
func extractApplyTokens(css string) []string {
	var tokens []string
	for _, m := range applyBlock.FindAllStringSubmatch(css, -1) {
		tokens = append(tokens, strings.Fields(m[1])...)
	}
	return tokens
}

type dirEntry struct {
	Token string
	N     int
}

// scanDirections counts the physical subset, sorted for stable baselines.
//
// The JS predecessor sorted with localeCompare, which orders "[" before a
// digit; this sorts bytewise, which does not. Only the ORDER of the printed
// inventory changes — the comparison itself is by token — and the recorded
// baseline below is written in the new order.
func scanDirections(css string) []dirEntry {
	counts := map[string]int{}
	for _, tok := range extractApplyTokens(css) {
		if !isPhysicalUtility(tok) {
			continue
		}
		counts[utilitySegment(tok)]++
	}
	out := make([]dirEntry, 0, len(counts))
	for tok, n := range counts {
		out = append(out, dirEntry{tok, n})
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Token < out[j].Token })
	return out
}

// The recorded physical-utility inventory of the current pin.
var directionBaseline = map[string]int{
	"border" + "-l-0":     1,
	"left" + "-3":         1,
	"ml" + "-1":           1,
	"ml" + "-[-0.15rem]":  1,
	"ml" + "-[-0.3rem]":   1,
	"mr" + "-1":           1,
	"mr" + "-[-0.15rem]":  1,
	"mr" + "-[-0.3rem]":   1,
	"pl" + "-1.5":         4,
	"pl" + "-1.5!":        1,
	"pl" + "-2":           4,
	"pl" + "-2.5":         1,
	"pr" + "-0":           2,
	"pr" + "-1.5":         4,
	"pr" + "-1.5!":        1,
	"pr" + "-18":          1,
	"pr" + "-2":           4,
	"pr" + "-8":           1,
	"right" + "-2":        1,
	"right" + "-2.5":      1,
	"right" + "-3":        2,
	"rounded" + "-l-none": 1,
	"rounded" + "-r-lg":   1,
	"rounded" + "-r-lg!":  1,
	"rounded" + "-r-none": 1,
	"text" + "-left":      5,
}

var leadingWord = regexp.MustCompile(`^([a-z]+)(.*)$`)

func gateCSSDirection(root string) error {
	css, err := os.ReadFile(filepath.Join(root, "dist/shadless.css"))
	if err != nil {
		return fmt.Errorf("FAIL  css-direction: %v", err)
	}
	entries := scanDirections(string(css))

	seen := map[string]bool{}
	var diffs []string
	for _, e := range entries {
		seen[e.Token] = true
		want, ok := directionBaseline[e.Token]
		if !ok {
			diffs = append(diffs, fmt.Sprintf("  + %s ×%d (new)", e.Token, e.N))
		} else if want != e.N {
			diffs = append(diffs, fmt.Sprintf("  ~ %s: ×%d was ×%d", e.Token, e.N, want))
		}
	}
	var gone []string
	for tok := range directionBaseline {
		if !seen[tok] {
			gone = append(gone, tok)
		}
	}
	sort.Strings(gone)
	for _, tok := range gone {
		diffs = append(diffs, fmt.Sprintf("  - %s ×%d (gone)", tok, directionBaseline[tok]))
	}

	if len(diffs) > 0 {
		return fmt.Errorf("FAIL  css-direction-gate: emitted physical utilities drifted from baseline\n%s\n\n"+
			"If intended (upstream re-pin / reviewed change): re-record with ./build/pipeline css-direction --update",
			strings.Join(diffs, "\n"))
	}
	fmt.Printf("PASS  css-direction-gate (%d physical utilities match baseline)\n", len(entries))
	return nil
}

// runCSSDirectionUpdate prints the fresh inventory in the same concatenated-pair
// shape so it can be pasted back into directionBaseline.
func runCSSDirectionUpdate() int {
	wd, _ := os.Getwd()
	root, err := findRepoRoot(wd)
	if err != nil {
		fmt.Fprintln(os.Stderr, "css-direction:", err)
		return 1
	}
	css, err := os.ReadFile(filepath.Join(root, "dist/shadless.css"))
	if err != nil {
		fmt.Fprintln(os.Stderr, "css-direction:", err)
		return 1
	}
	entries := scanDirections(string(css))
	fmt.Printf("// fresh inventory (%d entries) — paste into directionBaseline in "+
		"pipeline/gate_css_direction.go after review:\n", len(entries))
	for _, e := range entries {
		m := leadingWord.FindStringSubmatch(e.Token)
		fmt.Printf("\t%q + %q: %d,\n", m[1], m[2], e.N)
	}
	return 0
}
