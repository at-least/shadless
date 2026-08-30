package main

// The ledger, tested on scratch trees.
//
// The gate itself (TestLedger, in gates_test.go) is a differential test by
// construction: it cross-checks the ledger against the sources in BOTH
// directions, so if this port derived even one id differently from the JS it
// replaced, that id would show up as either undocumented or stale and the
// gate would fail. What needs testing here is the rest — the writing paths,
// where a formatting slip silently reformats a committed file, and the JS
// source extraction, where a moved anchor could silently yield nothing.

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// --------------------------------------------------------- JS extraction

func TestUnitJSSetLiteral(t *testing.T) {
	got, err := jsSetLiteral(`export const SKIN_ALLOWLIST = new Set([
  "cn-menu-target",
  // "cn-commented-out",
  "cn-rtl-flip",
])`, "SKIN_ALLOWLIST")
	if err != nil {
		t.Fatal(err)
	}
	want := []string{"cn-menu-target", "cn-rtl-flip"}
	if len(got) != len(want) {
		t.Fatalf("got %v, want %v (a commented-out entry must not count)", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Errorf("got %v, want %v", got, want)
		}
	}
}

// The whole point of the extractors: a moved anchor must fail loudly. An
// empty set would make the ledger gate pass over nothing.
func TestUnitJSSetLiteralFailsLoudly(t *testing.T) {
	if _, err := jsSetLiteral(`export const RENAMED = new Set(["x"])`, "SKIN_ALLOWLIST"); err == nil {
		t.Fatal("a missing declaration returned no error — the gate would pass over an empty set")
	}
}

func TestUnitJSSetLiteralSkipsBracketsInStrings(t *testing.T) {
	got, err := jsSetLiteral(`const S = new Set(["a]b", "c"])`, "S")
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 2 || got[0] != "a]b" {
		t.Errorf("a bracket inside a string closed the region early: %v", got)
	}
}

func TestUnitJSFieldIsFalse(t *testing.T) {
	src := "export default {\n  name: \"x\",\n  mountedCheck: false,\n  mountedClasses: true,\n}"
	if !jsFieldIsFalse(src, "mountedCheck") {
		t.Error("mountedCheck: false not detected")
	}
	if jsFieldIsFalse(src, "mountedClasses") {
		t.Error("mountedClasses: true reported as false")
	}
	// a mention in a comment is not a declaration
	if jsFieldIsFalse("// mountedCheck: false (recorded)\nname: 1", "mountedCheck") {
		t.Error("a commented-out field counted as a declaration")
	}
}

func TestUnitJSObjectFieldAndAttrMap(t *testing.T) {
	src := `export default {
  ignoreAttrs: {
    "accordion": ["text"],
    // text: recorded structural difference
    "accordion-item": ["text", "data-state"],
  },
  scenarios: ["click:x"],
}`
	body, ok, err := jsObjectField(src, "ignoreAttrs")
	if err != nil || !ok {
		t.Fatalf("ignoreAttrs not found: ok=%v err=%v", ok, err)
	}
	got := jsAttrMap(body)
	if len(got) != 2 {
		t.Fatalf("got %d entries, want 2: %v", len(got), got)
	}
	if got[0].Key != "accordion" || len(got[0].Values) != 1 {
		t.Errorf("entry 0 = %+v", got[0])
	}
	if got[1].Key != "accordion-item" || len(got[1].Values) != 2 || got[1].Values[1] != "data-state" {
		t.Errorf("entry 1 = %+v", got[1])
	}
	// scenarios must not leak in — jsObjectField stops at the matching brace
	for _, e := range got {
		if e.Key == "scenarios" {
			t.Error("the region ran past ignoreAttrs")
		}
	}
	if _, ok, _ := jsObjectField(src, "noSuchField"); ok {
		t.Error("an absent field reported present")
	}
}

func TestUnitJSUnescape(t *testing.T) {
	for in, want := range map[string]string{
		`plain`:       "plain",
		`a—b`:         "a—b",
		`quote\"here`: `quote"here`,
		`back\\slash`: `back\slash`,
		`line\nbreak`: "line\nbreak",
	} {
		if got := jsUnescape(in); got != want {
			t.Errorf("jsUnescape(%q) = %q, want %q", in, got, want)
		}
	}
}

// --------------------------------------------------------- JSON writing

// The committed files are JSON.stringify output. A Go map would reorder them
// and turn a one-line change into a whole-file diff, so key order round-trips.
func TestUnitLedgerRoundTripsKeyOrder(t *testing.T) {
	const src = `{
  "pin": "shadcn@4.19.0",
  "entries": {
    "zeta": {
      "class": "permanent",
      "reason": "a reason long enough",
      "source": "contracts",
      "recorded_at_pin": "shadcn@4.19.0"
    },
    "alpha": {
      "class": "debt",
      "reason": "another reason here",
      "source": "golden",
      "recorded_at_pin": "shadcn@4.19.0"
    }
  },
  "budgets": {
    "b.two": {
      "max": 2,
      "target": 0,
      "class": "debt",
      "reason": "second"
    },
    "b.one": {
      "max": 1,
      "target": 0,
      "class": "debt",
      "reason": "first"
    }
  },
  "notes": [
    "note one"
  ]
}
`
	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, "gates"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, ledgerPath), []byte(src), 0o644); err != nil {
		t.Fatal(err)
	}
	l, err := readLedger(root)
	if err != nil {
		t.Fatal(err)
	}
	// declaration order, NOT sorted — "zeta" was written first
	if len(l.EntryOrder) != 2 || l.EntryOrder[0] != "zeta" {
		t.Fatalf("entry order = %v, want [zeta alpha]", l.EntryOrder)
	}
	if len(l.BudgetOrder) != 2 || l.BudgetOrder[0] != "b.two" {
		t.Fatalf("budget order = %v, want [b.two b.one]", l.BudgetOrder)
	}
	if err := l.write(root); err != nil {
		t.Fatal(err)
	}
	got, err := os.ReadFile(filepath.Join(root, ledgerPath))
	if err != nil {
		t.Fatal(err)
	}
	if string(got) != src {
		t.Errorf("rewriting a ledger changed its bytes:\n--- want\n%s\n--- got\n%s", src, got)
	}
}

// Non-ASCII must pass through as UTF-8: JSON.stringify does not escape it,
// and escaping would rewrite every reason carrying an em dash.
func TestUnitLedgerWritesLiteralNonASCII(t *testing.T) {
	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, "gates"), 0o755); err != nil {
		t.Fatal(err)
	}
	l := &ledgerFile{
		Pin:        "shadcn@4.19.0",
		EntryOrder: []string{"x"},
		Entries: map[string]*ledgerEntry{"x": {
			Class: "permanent", Reason: "an em — dash", Source: "contracts", Recorded: "shadcn@4.19.0",
		}},
		Budgets: map[string]*ledgerBudget{},
	}
	if err := l.write(root); err != nil {
		t.Fatal(err)
	}
	b, err := os.ReadFile(filepath.Join(root, ledgerPath))
	if err != nil {
		t.Fatal(err)
	}
	if strings.Contains(string(b), `\u2014`) {
		t.Error("the em dash was escaped as \\u2014; JSON.stringify emits it literally")
	}
	if !strings.Contains(string(b), "an em — dash") {
		t.Errorf("the em dash did not survive: %s", b)
	}
}

// The golden exemptions file is written with indent 1, not 2.
func TestUnitGoldenExemptionsIndentIsOne(t *testing.T) {
	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, filepath.Dir(goldenExPath)), 0o755); err != nil {
		t.Fatal(err)
	}
	g := &goldenExemptions{
		Order:   []string{"b-demo", "a-demo"},
		Reasons: map[string]string{"b-demo": "reason b", "a-demo": "reason a"},
	}
	if err := g.write(root); err != nil {
		t.Fatal(err)
	}
	b, err := os.ReadFile(filepath.Join(root, goldenExPath))
	if err != nil {
		t.Fatal(err)
	}
	want := "{\n \"examples\": {\n  \"b-demo\": {\n   \"reason\": \"reason b\"\n  },\n" +
		"  \"a-demo\": {\n   \"reason\": \"reason a\"\n  }\n }\n}\n"
	if string(b) != want {
		t.Errorf("got:\n%q\nwant:\n%q", b, want)
	}
}

// --------------------------------------------------------- classification

func TestUnitClassOfGoldenReason(t *testing.T) {
	for reason, want := range map[string]string{
		"token drift vs live (ring-ring/50 vs /30) — deploy lag; re-check on re-pin":   "auto-dissolve",
		"radix SelectValue renders only after hydration (SSR frame empty) — frame lag": "auto-dissolve",
		"external dependency has no vanilla port":                                      "permanent",
		"RE-CHECK ON RE-PIN in caps":                                                   "auto-dissolve",
	} {
		if got := classOfGoldenReason(reason); got != want {
			t.Errorf("classOfGoldenReason(%q) = %s, want %s", reason, got, want)
		}
	}
}

// --------------------------------------------------------- budget ratchet

// A budget must fail in BOTH directions: growing is a regression, and
// shrinking without re-recording lets the slack be silently re-spent later.
func TestUnitLedgerBudgetRatchetBitesBothWays(t *testing.T) {
	root := ledgerFixture(t, 5)
	for _, tc := range []struct {
		name  string
		cells int
		want  string
	}{
		{"grew", 7, "may only shrink"},
		{"shrank", 3, "it improved"},
		{"exact", 5, ""},
	} {
		writeCells(t, root, tc.cells)
		err := gateLedger(root)
		switch {
		case tc.want == "" && err != nil:
			t.Errorf("%s: unexpected failure: %v", tc.name, err)
		case tc.want != "" && err == nil:
			t.Errorf("%s: budget did not bite", tc.name)
		case tc.want != "" && !strings.Contains(err.Error(), tc.want):
			t.Errorf("%s: error %q does not mention %q", tc.name, err, tc.want)
		}
	}
}

// ledgerFixture builds the smallest tree gateLedger can judge: one budget,
// no exemptions in any source, and a baseline holding `cells` rows.
func ledgerFixture(t *testing.T, max int) string {
	t.Helper()
	root := t.TempDir()
	write := func(p, s string) {
		t.Helper()
		full := filepath.Join(root, p)
		if err := os.MkdirAll(filepath.Dir(full), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(full, []byte(s), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write(pinRegistry, `{"shadcn_ui":{"tag":"shadcn@4.19.0"}}`)
	write(goldenExPath, `{"examples":{}}`)
	write(emitterCSS, `export const DEAD_UTILITIES = new Set([])`)
	write(emitterSkin, `export const SKIN_ALLOWLIST = new Set([])`)
	write(sweepPath, `const KNOWN_DEAD = new Set([])`)
	if err := os.MkdirAll(filepath.Join(root, contractsDir), 0o755); err != nil {
		t.Fatal(err)
	}
	write("gates/demo-parity-baseline.json", `{"cells":[]}`)
	write("gates/path-parity-baseline.json", `{"cells":[]}`)
	write(ledgerPath, `{
  "pin": "shadcn@4.19.0",
  "entries": {},
  "budgets": {
    "style-parity.dirty-cells": {
      "max": `+itoa(max)+`,
      "target": 0,
      "class": "debt",
      "reason": "recorded drift"
    },
    "golden.exempt-demos": {
      "max": 0,
      "target": 0,
      "class": "debt",
      "reason": "none"
    },
    "interactivity.dead-families": {
      "max": 0,
      "target": 0,
      "class": "debt",
      "reason": "none"
    },
    "demo-parity.dirty-cells": {
      "max": 0,
      "target": 0,
      "class": "debt",
      "reason": "none"
    },
    "path-parity.dirty-cells": {
      "max": 0,
      "target": 0,
      "class": "debt",
      "reason": "none"
    }
  },
  "notes": []
}
`)
	return root
}

func writeCells(t *testing.T, root string, n int) {
	t.Helper()
	cells := make([]string, n)
	for i := range cells {
		cells[i] = `{"id":"c"}`
	}
	body := `{"cells":[` + strings.Join(cells, ",") + `]}`
	if err := os.WriteFile(filepath.Join(root, "gates/style-parity-baseline.json"), []byte(body), 0o644); err != nil {
		t.Fatal(err)
	}
}

func itoa(n int) string {
	if n == 0 {
		return "0"
	}
	var b []byte
	for n > 0 {
		b = append([]byte{byte('0' + n%10)}, b...)
		n /= 10
	}
	return string(b)
}

// An exemption present in a source but absent from the ledger must fail —
// this is the ledger-undocumented-exemption mutation, at unit scale.
func TestUnitLedgerCatchesUndocumentedExemption(t *testing.T) {
	root := ledgerFixture(t, 0)
	writeCells(t, root, 0)
	if err := gateLedger(root); err != nil {
		t.Fatalf("the empty fixture should pass: %v", err)
	}
	if err := os.WriteFile(filepath.Join(root, emitterSkin),
		[]byte(`export const SKIN_ALLOWLIST = new Set(["cn-brand-new"])`), 0o644); err != nil {
		t.Fatal(err)
	}
	err := gateLedger(root)
	if err == nil {
		t.Fatal("a new source exemption with no ledger entry passed")
	}
	if !strings.Contains(err.Error(), "skin-allowlist:cn-brand-new") {
		t.Errorf("the report does not name the undocumented id: %v", err)
	}
}

// And the reverse direction: a ledger entry whose source flag is gone.
func TestUnitLedgerCatchesStaleEntry(t *testing.T) {
	root := ledgerFixture(t, 0)
	writeCells(t, root, 0)
	l, err := readLedger(root)
	if err != nil {
		t.Fatal(err)
	}
	l.EntryOrder = append(l.EntryOrder, "skin-allowlist:cn-gone")
	l.Entries["skin-allowlist:cn-gone"] = &ledgerEntry{
		Class: "permanent", Reason: "a reason long enough", Source: "emitter", Recorded: "shadcn@4.19.0",
	}
	if err := l.write(root); err != nil {
		t.Fatal(err)
	}
	err = gateLedger(root)
	if err == nil {
		t.Fatal("a ledger entry with no source flag passed")
	}
	if !strings.Contains(err.Error(), "source flag is gone") {
		t.Errorf("unexpected report: %v", err)
	}
}

// Entry shape is checked too: a trivial reason is not a reason.
func TestUnitLedgerRejectsBadEntries(t *testing.T) {
	for name, mutate := range map[string]func(*ledgerEntry){
		"unknown class":  func(e *ledgerEntry) { e.Class = "someday" },
		"trivial reason": func(e *ledgerEntry) { e.Reason = "meh" },
		"no pin":         func(e *ledgerEntry) { e.Recorded = "" },
	} {
		root := ledgerFixture(t, 0)
		writeCells(t, root, 0)
		if err := os.WriteFile(filepath.Join(root, emitterSkin),
			[]byte(`export const SKIN_ALLOWLIST = new Set(["cn-x"])`), 0o644); err != nil {
			t.Fatal(err)
		}
		l, err := readLedger(root)
		if err != nil {
			t.Fatal(err)
		}
		e := &ledgerEntry{Class: "permanent", Reason: "a reason long enough",
			Source: "emitter", Recorded: "shadcn@4.19.0"}
		mutate(e)
		l.EntryOrder = append(l.EntryOrder, "skin-allowlist:cn-x")
		l.Entries["skin-allowlist:cn-x"] = e
		if err := l.write(root); err != nil {
			t.Fatal(err)
		}
		if err := gateLedger(root); err == nil {
			t.Errorf("%s: accepted", name)
		}
	}
}

// record must be a no-op on a reconciled tree — otherwise every run of it
// produces a diff and nobody can tell a real change from churn.
func TestUnitLedgerRecordIsIdempotent(t *testing.T) {
	root := ledgerFixture(t, 0)
	writeCells(t, root, 0)
	if err := ledgerRecord(root); err != nil {
		t.Fatal(err)
	}
	first, err := os.ReadFile(filepath.Join(root, ledgerPath))
	if err != nil {
		t.Fatal(err)
	}
	if err := ledgerRecord(root); err != nil {
		t.Fatal(err)
	}
	second, err := os.ReadFile(filepath.Join(root, ledgerPath))
	if err != nil {
		t.Fatal(err)
	}
	if string(first) != string(second) {
		t.Error("a second record changed the file")
	}
}

// dissolve drops auto-dissolve entries recorded at an OLDER pin and prunes
// the golden demos whose reason went with them — and must leave entries
// recorded at the current pin alone, so a same-tag drill is a no-op.
func TestUnitLedgerDissolve(t *testing.T) {
	root := ledgerFixture(t, 0)
	writeCells(t, root, 0)
	if err := os.WriteFile(filepath.Join(root, goldenExPath),
		[]byte(`{"examples":{"old-demo":{"reason":"deploy lag; re-check on re-pin"},`+
			`"kept-demo":{"reason":"external dep, no vanilla port"}}}`), 0o644); err != nil {
		t.Fatal(err)
	}
	l, err := readLedger(root)
	if err != nil {
		t.Fatal(err)
	}
	for id, e := range map[string]*ledgerEntry{
		"golden:deploy lag; re-check on re-pin": {
			Class: "auto-dissolve", Reason: "deploy lag; re-check on re-pin",
			Source: "golden", Recorded: "shadcn@4.18.0"}, // OLDER pin
		"golden:external dep, no vanilla port": {
			Class: "permanent", Reason: "external dep, no vanilla port",
			Source: "golden", Recorded: "shadcn@4.19.0"},
		"skin-allowlist:cn-current": {
			Class: "auto-dissolve", Reason: "recorded at the current pin",
			Source: "emitter", Recorded: "shadcn@4.19.0"}, // CURRENT pin
	} {
		l.EntryOrder = append(l.EntryOrder, id)
		l.Entries[id] = e
	}
	if err := l.write(root); err != nil {
		t.Fatal(err)
	}
	if err := ledgerDissolve(root); err != nil {
		t.Fatal(err)
	}
	after, err := readLedger(root)
	if err != nil {
		t.Fatal(err)
	}
	if _, still := after.Entries["golden:deploy lag; re-check on re-pin"]; still {
		t.Error("an auto-dissolve entry from an older pin survived")
	}
	if _, gone := after.Entries["skin-allowlist:cn-current"]; !gone {
		t.Error("an auto-dissolve entry recorded at the CURRENT pin was dissolved — " +
			"a same-tag drill must be a no-op")
	}
	if _, gone := after.Entries["golden:external dep, no vanilla port"]; !gone {
		t.Error("a permanent entry was dissolved")
	}
	g, err := readGoldenExemptions(root)
	if err != nil {
		t.Fatal(err)
	}
	if _, still := g.Reasons["old-demo"]; still {
		t.Error("the golden demo whose reason dissolved was not pruned")
	}
	if _, gone := g.Reasons["kept-demo"]; !gone {
		t.Error("a golden demo with a permanent reason was pruned")
	}
	// the deliberate shrink must be re-recorded, or the drill's own verify
	// step reports the improvement as an UNEXPECTED failure
	if b := after.Budgets["golden.exempt-demos"]; b != nil && b.Max != len(g.Order) {
		t.Errorf("golden budget = %d, want %d after pruning", b.Max, len(g.Order))
	}
}
