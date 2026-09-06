package main

import "testing"

// sweepFamilyOf is pure string logic (no browser dependency) but encodes a
// precedence rule worth pinning down: the `^(family)-` prefix match is tried
// BEFORE the generic `-rtl(-|$).*` strip, so a name whose family-prefix match
// and RTL-stripped form disagree must resolve via the family regex.
func TestUnitSweepFamilyOf(t *testing.T) {
	cases := []struct{ name, want string }{
		// plain name: matches the family-prefix regex directly.
		{"accordion-basic", "accordion"},
		// the actual precedence case: the family regex ("accordion-") matches
		// at the very start regardless of the "-rtl" suffix further in, so it
		// must win — stripping "-rtl" first would instead yield
		// "accordion-multiple", which is a different (wrong) answer. This is
		// the case that would catch the two regexes being tried in the wrong
		// order; "dialog-rtl-demo" would not, since both branches happen to
		// agree on "dialog" there.
		{"accordion-multiple-rtl", "accordion"},
		// RTL name whose base is NOT a listed family: falls through to the
		// RTL-strip branch, proving it actually fires (not just a no-op).
		{"code-block-rtl-en", "code-block"},
		// matches neither pattern: falls through unchanged.
		{"custom-widget-demo", "custom-widget-demo"},
	}
	for _, c := range cases {
		if got := sweepFamilyOf(c.name); got != c.want {
			t.Errorf("sweepFamilyOf(%q) = %q, want %q", c.name, got, c.want)
		}
	}
}
