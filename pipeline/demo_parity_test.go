package main

import "testing"

// demoParityNorm is applied identically to BOTH sides of every demo-parity
// comparison (see runDemoParity: a := demoParityNorm(ref[p]); b :=
// demoParityNorm(got[p])) — a bug that makes it lossy or over-eager (wrong
// rounding, a missed number pattern) would silently collapse a real
// difference on both sides equally, so the browser-based gate itself can
// never catch it. Only a direct unit test on input/output pairs can.
//
// Two of these cases pin a real float64-precision quirk rather than "JS
// parity": parseFloat2dp rounds half AWAY FROM ZERO via `int64(f*100 +
// 0.5*sign(f))`, but float64 arithmetic means an exact-looking decimal
// tie isn't always exactly representable, so it doesn't always land where
// a decimal reading of the "round half away from zero" rule would suggest.
// Non-tie values are used for the assertions that matter; the quirk is
// noted, not asserted as correct.
func TestUnitDemoParityNorm(t *testing.T) {
	cases := []struct{ name, in, want string }{
		{"negative value rounds toward zero and drops the sign at zero", "-0.004px", "0px"},
		{"negative value, ordinary rounding to 2dp", "-1.236px", "-1.24px"},
		{"positive value, ordinary rounding to 2dp", "12.345px", "12.35px"},
		{"tiny positive value normalizes to plain 0, no exponent", "1e-7px", "0px"},
		{"negative zero normalizes to positive zero (Object.is(-0) guard)", "-0px", "0px"},
		{"already-2dp value is a no-op", "-0.5px", "-0.5px"},
		{"integer value is a no-op (no trailing .00)", "12px", "12px"},
		{"empty string is a no-op", "", ""},
		{"every embedded number in a multi-value string is rounded independently", "10.005px 20.3459px", "10.01px 20.35px"},
		{"axis-only oklab() canonicalizes to oklch(), with its number still rounded", "oklab(0.512345 0 0)", "oklch(0.51 0 0)"},
		{"a non-axis oklab() (nonzero b) is left alone — not this component's shape", "oklab(0.5 0.1 0.2)", "oklab(0.5 0.1 0.2)"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := demoParityNorm(c.in); got != c.want {
				t.Errorf("demoParityNorm(%q) = %q, want %q", c.in, got, c.want)
			}
		})
	}
}

// parseFloat2dp / sign directly: the pure numeric core demoParityNorm
// delegates to per matched number.
func TestUnitParseFloat2dp(t *testing.T) {
	cases := []struct {
		name string
		in   string
		want float64
	}{
		{"positive, rounds up", "12.345", 12.35},
		{"negative, rounds away from zero", "-1.236", -1.24},
		{"integer", "12", 12},
		{"near-zero positive collapses to positive zero", "0.001", 0},
		{"near-zero negative also collapses to (positive) zero", "-0.001", 0},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := parseFloat2dp(c.in); got != c.want {
				t.Errorf("parseFloat2dp(%q) = %v, want %v", c.in, got, c.want)
			}
		})
	}
}

func TestUnitSign(t *testing.T) {
	if sign(1) != 1 {
		t.Errorf("sign(1) != 1")
	}
	if sign(-1) != -1 {
		t.Errorf("sign(-1) != -1")
	}
	// sign(0) taking the >= 0 branch matters: parseFloat2dp calls
	// int64(f*100 + 0.5*sign(f)) and a f==0 input must round-trip to 0,
	// which needs sign(0) to add +0.5 (not -0.5).
	if sign(0) != 1 {
		t.Errorf("sign(0) = %v, want 1 (the non-negative branch)", sign(0))
	}
}
