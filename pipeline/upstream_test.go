package main

// The re-pin drill's classification.
//
// The whole value of the drill is the EXPECTED/UNEXPECTED split: it tells you
// which red gates are consequences of an upstream change and which are your
// own regression. Getting that backwards sends a reviewer to the wrong place,
// and on a re-pin with dozens of red gates that is the difference between an
// afternoon and a week. The report shape itself was verified against the JS by
// diffing --report-only over the real tree (byte-identical).

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestUnitClassifyFailures(t *testing.T) {
	registry := []string{"accordion", "badge", "dialog", "select"}
	changed := map[string]bool{"dialog": true}
	run := runReport{Failed: map[NodeID]failedNode{
		// names a component that DID move upstream
		"contracts:dialog": {Cmd: "x", Tail: "FAIL contracts dialog: slot mismatch"},
		// names one that did NOT
		"path-parity": {Cmd: "y", Tail: "FAIL path-parity: badge lost its padding"},
		// names nothing recognisable
		"pack": {Cmd: "z", Tail: "FAIL pack: exports map is broken"},
	}}
	expected, unexpected := classifyFailures(run, registry, changed)
	if len(expected) != 1 || !strings.Contains(expected[0], "EXPECTED — upstream changed: dialog") {
		t.Errorf("expected = %v", expected)
	}
	if len(unexpected) != 2 {
		t.Fatalf("unexpected = %v, want 2 entries", unexpected)
	}
	joined := strings.Join(unexpected, "\n")
	if !strings.Contains(joined, "UNEXPECTED — mentions badge, none changed upstream") {
		t.Errorf("a failure naming an unchanged component should be UNEXPECTED:\n%s", joined)
	}
	if !strings.Contains(joined, "UNEXPECTED — no component attribution") {
		t.Errorf("a failure naming nothing should say so:\n%s", joined)
	}
	// every entry must carry a repro line — the report is meant to be acted on
	for _, e := range append(expected, unexpected...) {
		if !strings.Contains(e, "repro: `./build/pipeline run ") {
			t.Errorf("entry has no repro line:\n%s", e)
		}
	}
}

// Word boundaries matter: "select" must not match inside "selected", or every
// gate mentioning a selected element would be attributed to the select
// component and classified EXPECTED on any re-pin that touched it.
func TestUnitClassifyUsesWordBoundaries(t *testing.T) {
	run := runReport{Failed: map[NodeID]failedNode{
		"g": {Tail: "the selected item was unselected"},
	}}
	_, unexpected := classifyFailures(run, []string{"select"}, map[string]bool{"select": true})
	if len(unexpected) != 1 {
		t.Fatalf("want the failure classified UNEXPECTED, got expected instead")
	}
	if !strings.Contains(unexpected[0], "no component attribution") {
		t.Errorf("`select` matched inside `selected`: %s", unexpected[0])
	}
}

// Ordering must be deterministic: the report is committed to build/ and read
// by a human, and a Go map would shuffle the sections on every run.
func TestUnitClassifyIsDeterministic(t *testing.T) {
	run := runReport{Failed: map[NodeID]failedNode{
		"zeta": {Tail: "no names"}, "alpha": {Tail: "no names"}, "mid": {Tail: "no names"},
	}}
	var first string
	for i := 0; i < 5; i++ {
		_, unexpected := classifyFailures(run, nil, nil)
		got := strings.Join(unexpected, "|")
		if i == 0 {
			first = got
			continue
		}
		if got != first {
			t.Fatalf("classification order is not stable:\n%s\nvs\n%s", first, got)
		}
	}
	if !strings.HasPrefix(first, "### alpha") {
		t.Errorf("want the entries sorted by id, got %s", first[:20])
	}
}

// An example filename maps back to its component by the LONGEST matching
// registry name, so accordion-multiple belongs to accordion, and a component
// whose name prefixes another does not steal it.
func TestUnitComponentOfExample(t *testing.T) {
	registry := []string{"accordion", "toggle", "toggle-group", "select"}
	for name, want := range map[string]string{
		"accordion":             "accordion",
		"accordion-multiple":    "accordion",
		"toggle-group":          "toggle-group",
		"toggle-group-multiple": "toggle-group",
		"toggle-demo":           "toggle",
		"something-unrelated":   "something-unrelated",
	} {
		if got := componentOfExample(name, registry); got != want {
			t.Errorf("componentOfExample(%q) = %q, want %q", name, got, want)
		}
	}
}

func TestUnitComponentOf(t *testing.T) {
	for in, want := range map[string]string{
		"apps/v4/registry/bases/radix/ui/badge.tsx":      "badge",
		"apps/v4/examples/radix/accordion-demo.tsx":      "accordion-demo",
		"apps/v4/content/docs/components/radix/tabs.mdx": "tabs",
		"apps/v4/app/globals.css":                        "globals",
		"noslash.tsx":                                    "noslash",
	} {
		if got := componentOf(in); got != want {
			t.Errorf("componentOf(%q) = %q, want %q", in, got, want)
		}
	}
}

// A missing run report must not crash the drill: --report-only is explicitly
// documented as re-classifying "the last run", which may not exist.
func TestUnitReadRunReportTolerAtesMissing(t *testing.T) {
	r := readRunReport(t.TempDir())
	if r.Failed == nil {
		t.Error("Failed must be non-nil so the caller can range over it")
	}
	if len(r.Passed) != 0 || len(r.Blocked) != 0 {
		t.Error("a missing report should read as empty, not as content")
	}
}

func TestUnitCopyTreeReplaces(t *testing.T) {
	root := t.TempDir()
	src := filepath.Join(root, "src")
	dst := filepath.Join(root, "dst")
	if err := os.MkdirAll(filepath.Join(src, "sub"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(src, "sub", "a.json"), []byte("new"), 0o644); err != nil {
		t.Fatal(err)
	}
	// a stale file in the destination must not survive: the drill compares
	// the IR before and after, and a leftover would show as a phantom change
	if err := os.MkdirAll(dst, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dst, "stale.json"), []byte("old"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := copyTree(src, dst); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(filepath.Join(dst, "stale.json")); !os.IsNotExist(err) {
		t.Error("copyTree left a stale file behind")
	}
	b, err := os.ReadFile(filepath.Join(dst, "sub", "a.json"))
	if err != nil || string(b) != "new" {
		t.Errorf("copyTree did not reproduce the tree: %v %q", err, b)
	}
}

func TestUnitFlagValue(t *testing.T) {
	args := []string{"--to=shadcn@4.20.0", "--fetch"}
	if got := flagValue(args, "to"); got != "shadcn@4.20.0" {
		t.Errorf("flagValue(to) = %q", got)
	}
	if got := flagValue(args, "missing"); got != "" {
		t.Errorf("flagValue(missing) = %q, want empty", got)
	}
	// a bare flag is not a value flag
	if got := flagValue(args, "fetch"); got != "" {
		t.Errorf("flagValue(fetch) = %q, want empty", got)
	}
}
