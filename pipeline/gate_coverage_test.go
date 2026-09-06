package main

// Pure-function test for the state-token classifier that drives the coverage
// matrix's state axis. gateCoverage itself is exempt from unit testing (it is
// a runXxx-shaped driver proven end to end by TestCoverage), but hasStateToken
// is a standalone pure function with real branching — two regexes plus a
// hand-written exclusion — and deserves the same kind of table-driven pin
// TestUnitCssDirectionClassify already gives isPhysicalUtility.

import "testing"

// Fixture tokens are built with the u() helper from gate_css_direction_test.go
// (same package, same file-scoped convention): a literal utility class in a
// tracked .go file gets absorbed by the tailwind CLI's repo-wide content scan
// (see pipeline/tw.go) and would leak a phantom rule into dist/out.css,
// tripping the reproducible gate.
func TestUnitHasStateToken(t *testing.T) {
	trueCases := []string{
		// reStateData: bracket form, name=state, value present, ^ anchor
		u("data-[state=open]:bg", "-primary"),
		// reStateNamed: aria-expanded, literal word
		u("aria-expanded:rotate", "-180"),
		// reStateNamed: data-open, literal word (not a bracket form)
		u("data-open:opacity", "-100"),
		// reStateNamed: aria-[name=value] alternative
		u("aria-[foo=bar]:opacity", "-50"),
		// reStateData: the `-` anchor for Tailwind compound variants
		// (group-data-[...]:, peer-data-[...]:) — a component whose state is
		// only visible through a sibling/group must still count.
		u("group-data-[disabled=true]:opacity", "-50"),
		// reStateData: the `:` anchor, a data-[...] state stacked after
		// another variant.
		u("hover:data-[state=open]:opacity", "-50"),
		// reStateData: name=slot but NO value present — the exclusion only
		// fires when a value follows the "=", so a bare `data-[slot]:` (no
		// "=...") is not the mounted-classes lookalike and still counts.
		u("data-[slot]:opacity", "-50"),
		// reStateData: FindAllStringSubmatch must keep scanning past an
		// excluded match (slot+value) to find a real state token later in
		// the same joined class string.
		u("data-[slot=trigger]:x ", "data-[state=open]:opacity", "-50"),
	}
	for _, tok := range trueCases {
		if !hasStateToken(tok) {
			t.Errorf("%q should carry a state token", tok)
		}
	}

	falseCases := []string{
		// the mounted-classes exclusion: name in {slot,variant,size} AND a
		// value is present — these are structural/cva selectors, not state.
		u("data-[slot=trigger]:opacity", "-50"),
		u("data-[variant=outline]:opacity", "-50"),
		u("data-[size=lg]:opacity", "-50"),
		// no valid anchor before "data-" (preceded by a letter, not
		// ^ / \s / : / -)
		u("xdata-[state=open]:opacity", "-50"),
		// no trailing ":" — not a variant at all
		u("data-[state=open]"),
		// plain classes, no data-/aria- state token anywhere
		u("bg", "-primary", " ", "text", "-sm"),
	}
	for _, tok := range falseCases {
		if hasStateToken(tok) {
			t.Errorf("%q should NOT carry a state token", tok)
		}
	}
}
