package main

import (
	"strings"
	"testing"
)

// oracleNorm must be stable AND injective on a page: the same React id maps
// to the same token every time it appears, different ids to different
// tokens, in first-appearance order — so aria references keep resolving.
// Every spelling React's useId has used is covered; the 19.1+ CSR and SSR
// forms end in "_", which the old regex left behind.
func TestUnitOracleNormDistinct(t *testing.T) {
	in := `<button id="radix-_r_3_" aria-controls="radix-_r_4_"></button>` +
		`<div id="radix-_r_4_" aria-labelledby="radix-_r_3_"></div>` +
		`<i id="radix-_R_0H1_"></i><i id="radix-:r9:"></i><i id="radix-«r2»"></i>`
	got := oracleNorm(in)
	want := `<button id="radix-a1" aria-controls="radix-a2"></button>` +
		`<div id="radix-a2" aria-labelledby="radix-a1"></div>` +
		`<i id="radix-a3"></i><i id="radix-a4"></i><i id="radix-a5"></i>`
	if got != want {
		t.Fatalf("got  %s\nwant %s", got, want)
	}
	if strings.Contains(got, "_\"") || strings.Contains(got, "<auto>") {
		t.Errorf("residue survived: %s", got)
	}
	if oracleNorm(in) != got {
		t.Error("not stable across calls")
	}
	if oracleNorm("<p>no ids</p>") != "<p>no ids</p>" {
		t.Error("touched text with no auto-id")
	}
}

// The protocol patch is exact-anchor and loud: a page not in the table passes
// through untouched, the anchored page gains the attribute exactly once, and
// a missing or ambiguous anchor is an error rather than a silent pass-through.
func TestUnitOraProtocolPatch(t *testing.T) {
	root := `<div data-slot="accordion" class="flex w-full flex-col max-w-lg" data-orientation="vertical">`
	if out, err := oraProtocolPatch("accordion-basic", root); err != nil || out != root {
		t.Fatalf("untouched page changed: %v %q", err, out)
	}
	out, err := oraProtocolPatch("accordion-multiple", root+"<i></i>")
	if err != nil {
		t.Fatal(err)
	}
	if strings.Count(out, `data-type="multiple"`) != 1 {
		t.Errorf("attribute not added exactly once: %s", out)
	}
	if _, err := oraProtocolPatch("accordion-multiple", "<div>moved</div>"); err == nil {
		t.Error("missing anchor did not fail loud")
	}
	if _, err := oraProtocolPatch("accordion-multiple", root+root); err == nil {
		t.Error("ambiguous anchor did not fail loud")
	}
}
