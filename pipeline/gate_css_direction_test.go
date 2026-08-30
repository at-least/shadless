package main

// Pure-function tests for the physical-utility classifier and scanner,
// ported from tools/unit/css-direction.mjs. Negative-case evidence (drift
// detection end to end) lives in the gate; these pin the classification rules.

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// Fixture tokens are CONCATENATED at runtime on purpose: this file is
// tracked, and the tailwind CLI's repo-wide auto-scan (see pipeline/tw.go)
// would otherwise absorb these literals into dist/out.css as phantom utility
// rules. Do not "simplify" them back to literals.
func u(parts ...string) string { return strings.Join(parts, "") }

func TestUnitCssDirectionSegment(t *testing.T) {
	// variant-prefix stripping: keep the segment after the LAST ':'
	cases := []struct{ in, want string }{
		{u("p", "r-9"), u("p", "r-9")},
		{u("sm:p", "l-2.5"), u("p", "l-2.5")},
		{u("has-[>kbd]:m", "r-[-0.35rem]"), u("m", "r-[-0.35rem]")},
		{u("group-data-[orientation=vertical]/attachment:right", "-3"), u("right", "-3")},
		{u("[&>*:not(:last-child)]:", u("rounded", "-r-none")), u("rounded", "-r-none")},
		{u("before:", u("m", "r-1")), u("m", "r-1")},
	}
	for _, c := range cases {
		if got := utilitySegment(c.in); got != c.want {
			t.Errorf("utilitySegment(%q) = %q, want %q", c.in, got, c.want)
		}
	}
}

func TestUnitCssDirectionClassify(t *testing.T) {
	physical := []string{
		u("m", "l-1"), u("mr", "-[-0.45rem]"), u("pl", "-3"), u("p", "r-0"),
		u("left", "-3"), u("right", "-3.5"), u("text", "-left"), u("text", "-right"),
		u("border", "-l-0"), u("rounded", "-r-none"), u("rounded", "-l-md"),
		u("space", "-x-2"), u("divide", "-x-2"),
	}
	for _, tok := range physical {
		if !isPhysicalUtility(tok) {
			t.Errorf("%q should be physical", tok)
		}
	}
	// lookalikes that must NOT match
	logical := []string{
		u("sr", "-only"), u("rounded", "-lg"), u("rounded", "-sm"), u("border", "-ring"),
		u("border", "-l"), u("inset", "-s-1/2"), u("inset", "-0"), u("m", "s-auto"),
		u("me", "-1"), u("p", "s-2"), u("pe", "-2"), u("start", "-0"), u("end", "-2"),
		u("text", "-muted-foreground"), u("focus-visible:", u("border", "-ring")),
		u("placeholder:", u("text", "-sm")), u("translate", "-x-2"), u("m", "x-2"),
		u("p", "x-3"), u("slashed", "-zero"),
	}
	for _, tok := range logical {
		if isPhysicalUtility(tok) {
			t.Errorf("%q should NOT be physical", tok)
		}
	}
}

func TestUnitCssDirectionScan(t *testing.T) {
	css := "@layer components {\n" +
		"  .a { @apply " + u("m", "l-1") + " " + u("m", "l-1") + " " + u("p", "r-2") +
		" " + u("rounded", "-lg") + " " + u("sr", "-only") + " " + u("border", "-ring") + "; }\n" +
		"  .b::before { @apply " + u("sm:p", "l-4") + " " + u("has-[>kbd]:m", "r-1") +
		" " + u("inset", "-s-1/2") + "; }\n" +
		"  .c { color: var(--x); } /* not an @apply */\n}"
	want := []dirEntry{
		{u("m", "l-1"), 2}, {u("m", "r-1"), 1}, {u("p", "l-4"), 1}, {u("p", "r-2"), 1},
	}
	got := scanDirections(css)
	if len(got) != len(want) {
		t.Fatalf("scanDirections = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Errorf("scanDirections[%d] = %v, want %v", i, got[i], want[i])
		}
	}
	if n := len(extractApplyTokens("@apply a b; x @apply c;")); n != 3 {
		t.Errorf("extractApplyTokens = %d tokens, want 3", n)
	}
}

// Named TestCssDirection*, not TestUnit*, ON PURPOSE: it reads
// dist/shadless.css, which `emit` produces. The css-direction GATE declares
// that file as an input and needs the node that builds it; the `unit` gate
// declares neither, so running this there judged an artifact whose freshness
// and ordering nothing tied to it. The undeclared-read check found it.
func TestCssDirectionRealCSS(t *testing.T) {
	// the real emitted CSS must stay scannable (catches syntax drift in the
	// @apply extraction regex when the emitter changes shape)
	b, err := os.ReadFile(filepath.Join(repoRoot(t), "dist/shadless.css"))
	if err != nil {
		t.Fatal(err)
	}
	if n := len(scanDirections(string(b))); n < 20 {
		t.Errorf("dist/shadless.css yields %d physical utilities, want >= 20", n)
	}
}
