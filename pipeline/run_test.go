package main

// Runner behaviour that the re-pin drill depends on.
//
// build/gates/run-report.json is an INTERFACE, not a log: the drill reads it
// to classify every red gate as EXPECTED (its components moved upstream) or
// UNEXPECTED (our pipeline regressed). A missing or mis-shaped report makes
// the drill silently classify nothing, which is the same as not running it.

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestUnitRunReportRecordsFailedBlockedPassed(t *testing.T) {
	root := t.TempDir()
	// ok -> bad -> downstream. `bad` fails, so `downstream` is never reached.
	g, err := newGraph([]Node{
		{ID: "ok", Kind: "build", Tier: "fast", Run: [][]string{{"true"}}, Produces: []string{"x"}},
		{ID: "bad", Kind: "gate", Tier: "fast", Needs: []NodeID{"ok"},
			Run: [][]string{{"sh", "-c", "echo boom >&2; exit 1"}}},
		{ID: "downstream", Kind: "gate", Tier: "fast", Needs: []NodeID{"bad"},
			Run: [][]string{{"true"}}},
	})
	if err != nil {
		t.Fatal(err)
	}
	r := &Runner{root: root, graph: g, jobs: 1, force: true, continueOnFail: true, stamps: stamps{}}
	plan, err := g.Plan([]NodeID{"downstream"})
	if err != nil {
		t.Fatal(err)
	}
	_, _, failed, _, _ := r.Run(plan)
	if failed != 1 {
		t.Fatalf("failed = %d, want 1", failed)
	}
	if err := r.writeReport(); err != nil {
		t.Fatal(err)
	}
	b, err := os.ReadFile(filepath.Join(root, "build", "gates", "run-report.json"))
	if err != nil {
		t.Fatal(err)
	}
	var got runReport
	if err := json.Unmarshal(b, &got); err != nil {
		t.Fatalf("the drill must be able to parse this: %v", err)
	}
	if _, ok := got.Failed["bad"]; !ok {
		t.Errorf("failed = %v, want it to name `bad`", got.Failed)
	}
	// the tail is what the drill greps for component names
	if !strings.Contains(got.Failed["bad"].Tail, "boom") {
		t.Errorf("tail = %q, want the command's output", got.Failed["bad"].Tail)
	}
	if got.Failed["bad"].Cmd == "" {
		t.Error("no cmd recorded — the report should say what to re-run")
	}
	if len(got.Blocked) != 1 || got.Blocked[0] != "downstream" {
		t.Errorf("blocked = %v, want [downstream]", got.Blocked)
	}
	if len(got.Passed) != 1 || got.Passed[0] != "ok" {
		t.Errorf("passed = %v, want [ok]", got.Passed)
	}
}

// An all-green run must still produce a well-formed report: the drill does
// `runReport.passed.length` unconditionally, and a null would break it.
func TestUnitRunReportIsWellFormedWhenGreen(t *testing.T) {
	root := t.TempDir()
	g, err := newGraph([]Node{
		{ID: "a", Kind: "gate", Tier: "fast", Run: [][]string{{"true"}}},
	})
	if err != nil {
		t.Fatal(err)
	}
	r := &Runner{root: root, graph: g, jobs: 1, force: true, continueOnFail: true, stamps: stamps{}}
	plan, _ := g.Plan([]NodeID{"a"})
	r.Run(plan)
	if err := r.writeReport(); err != nil {
		t.Fatal(err)
	}
	b, _ := os.ReadFile(filepath.Join(root, "build", "gates", "run-report.json"))
	s := string(b)
	for _, want := range []string{`"failed": {}`, `"blocked": []`, `"passed"`} {
		if !strings.Contains(s, want) {
			t.Errorf("report is missing %s:\n%s", want, s)
		}
	}
}

func TestUnitTailOf(t *testing.T) {
	if got := tailOf([]byte("a\n\n\nb\n")); got != "a\nb" {
		t.Errorf("blank lines should be dropped, got %q", got)
	}
	var many []string
	for i := 0; i < 40; i++ {
		many = append(many, string(rune('a'+i%26)))
	}
	got := tailOf([]byte(strings.Join(many, "\n")))
	if n := len(strings.Split(got, "\n")); n != 25 {
		t.Errorf("kept %d lines, want the last 25", n)
	}
}
