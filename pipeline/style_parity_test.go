package main

// styleParityNorm is a pure string-in/string-out function — no browser
// needed — even though it is only reached transitively inside runStyleParity,
// a browser-driving driver this repo's convention correctly leaves untested.

import "testing"

func TestUnitStyleParityNorm(t *testing.T) {
	cases := []struct{ in, want string }{
		{"1.006px", "1.01px"}, // rounds to 2dp correctly
		{"-0.001px", "0px"},   // a tiny negative rounds to "0", not "-0"
		{"oklab(0.5 0 0)", "oklch(0.5 0 0)"},
		{"calc(1px + 2px)", "calc(…)"}, // engine-specific calc() formatting collapses
		{"", ""},                       // empty input is returned unchanged
	}
	for _, c := range cases {
		if got := styleParityNorm(c.in); got != c.want {
			t.Errorf("styleParityNorm(%q) = %q, want %q", c.in, got, c.want)
		}
	}
}
