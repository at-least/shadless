package main

// Boundary-audit classification tests.
//
// The port was verified against the JS it replaces by diffing all three
// modes over the real tree (byte-identical but for the identifier names in
// two prose lines). These tests pin the behaviour that diff cannot: the
// first-match-wins ORDER, which is load-bearing and has been wrong before.

import (
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// Order is the whole contract of these tables. A catch-all placed before a
// specific rule silently swallows it — a dist/components RTL file was once
// attributed to the emitter instead of build-rtl for exactly that reason.
func TestUnitAuditClassifyOrder(t *testing.T) {
	for _, tc := range []struct{ path, kind, owner string }{
		// RTL variants must reach build-rtl, NOT the broader dist/components rule
		{"dist/components/alert-rtl-he.html", "programmatic", "tools/build-rtl.mjs"},
		{"dist/components/alert-rtl-en.html", "programmatic", "tools/build-rtl.mjs"},
		{"dist/components/alert-rtl-fa.html", "programmatic", "tools/build-rtl.mjs"},
		// alert-demo is the oracle's, carved out of the same rule
		{"dist/components/alert-demo.html", "programmatic", "tools/example-oracle.mjs"},
		// a plain component page belongs to the emitter/demo rule
		{"dist/components/accordion.html", "programmatic",
			"src/emitter/index.mjs OR tools/demo.mjs (per-tier fixture)"},
		// docs/demos RTL variants are build-rtl output, not hand-authored —
		// programmatic patterns are consulted before hand-authored ones
		{"docs/demos/alert-rtl-he.html", "programmatic", "tools/build-rtl.mjs"},
		// a bare -rtl.html read as hand-authored too, and is not: the oracle
		// manifest claims it, and examples/radix/alert-rtl.tsx is its source
		{"docs/demos/alert-rtl.html", "programmatic", "tools/example-oracle.mjs"},
		// written by example-oracle from the React render, which its manifest
		// records — this used to read as hand-authored
		{"docs/demos/badge-demo.html", "programmatic", "tools/example-oracle.mjs"},
		// IR json
		{"src/registry/ir/badge.json", "programmatic", "src/converter/index.mjs"},
		// tool source
		{"tools/demo.mjs", "tool-source", ""},
		{"src/tags.mjs", "tool-source", ""},
		// pin.json is hand-authored even though it sits under src/registry
		{"src/registry/pin.json", "hand-authored",
			"pipeline upstream (re-pin) / human (vendor re-hash via ./build/pipeline pin --force)"},
		// nothing claims a Go source file
		{"pipeline/main.go", "unknown", ""},
	} {
		got := classifyPath(tc.path)
		if got.Kind != tc.kind {
			t.Errorf("%s: kind = %s, want %s", tc.path, got.Kind, tc.kind)
			continue
		}
		if tc.owner == "" {
			continue
		}
		if label := got.label(); label != tc.owner {
			t.Errorf("%s: attributed to %q, want %q", tc.path, label, tc.owner)
		}
	}
}

// Every pattern must set exactly one of tool/note/owner — the kind is
// inferred from which table it is in, and a blank label makes a report row
// that says nothing.
func TestUnitAuditPatternsAreWellFormed(t *testing.T) {
	check := func(name string, ps []boundaryPattern, field func(boundaryPattern) string) {
		for i, p := range ps {
			if p.re == nil && p.match == nil {
				t.Errorf("%s[%d]: neither re nor match set", name, i)
			}
			if p.re != nil && p.match != nil {
				t.Errorf("%s[%d]: both re and match set — only one is consulted", name, i)
			}
			if field(p) == "" {
				t.Errorf("%s[%d]: no label", name, i)
			}
		}
	}
	check("programmaticPatterns", programmaticPatterns, func(p boundaryPattern) string { return p.tool })
	check("toolSourcePatterns", toolSourcePatterns, func(p boundaryPattern) string { return p.note })
	check("handAuthoredPatterns", handAuthoredPatterns, func(p boundaryPattern) string { return p.owner })

	for i, p := range programmaticPatterns {
		if p.source == "" {
			t.Errorf("programmaticPatterns[%d] (%s): no source — the report cannot say what it is built from", i, p.tool)
		}
	}
	for i, h := range heuristicHints {
		if h.kind == "" {
			t.Errorf("heuristicHints[%d]: no kind", i)
		}
		if h.tool == "" && h.note == "" && h.owner == "" {
			t.Errorf("heuristicHints[%d]: no suggestion payload", i)
		}
	}
	for i, p := range llmPatchPoints {
		if p.File == "" || p.What == "" {
			t.Errorf("llmPatchPoints[%d]: needs a file and an explanation", i)
		}
	}
}

// The specific heuristics must win over the dist/ catch-all.
func TestUnitAuditHeuristicOrder(t *testing.T) {
	for path, wantTool := range map[string]string{
		"dist/components/alert-rtl-he.html": "tools/build-rtl.mjs",
		"docs/demos/thing-demo.html":        "tools/example-oracle.mjs",
		"dist/glue/dialog-glue.js":          "tools/demo.mjs",
		// only the catch-all matches this one
		"dist/widgets/new.html": "src/emitter/index.mjs OR tools/demo.mjs (per-tier fixture)",
	} {
		s := suggestFromHeuristics(path)
		if s == nil {
			t.Errorf("%s: no heuristic matched", path)
			continue
		}
		if s.Tool != wantTool {
			t.Errorf("%s: suggested %q, want %q", path, s.Tool, wantTool)
		}
	}
}

func TestUnitAuditWalkSkips(t *testing.T) {
	for _, rel := range []string{
		".git/config", "node_modules/x/index.js", ".upstream/shadcn-ui/a.tsx",
		"tools/contracts/out/dialog/oracle.html", "a/node_modules/b",
	} {
		if !auditShouldSkip(rel) {
			t.Errorf("%s should be skipped", rel)
		}
	}
	for _, rel := range []string{
		"dist/components/accordion.html", "tools/demo.mjs",
		// the skip is a path prefix, not a substring: a sibling directory
		// whose name merely starts the same way must survive
		"tools/contracts/run.mjs", "tools/contracts/outer/x.mjs",
	} {
		if auditShouldSkip(rel) {
			t.Errorf("%s should NOT be skipped", rel)
		}
	}
}

func TestUnitAuditWalkIsSortedAndRelative(t *testing.T) {
	root := t.TempDir()
	for _, p := range []string{"b/2.txt", "a/1.txt", "node_modules/x.txt", "c.txt"} {
		full := filepath.Join(root, p)
		if err := os.MkdirAll(filepath.Dir(full), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(full, nil, 0o644); err != nil {
			t.Fatal(err)
		}
	}
	got, err := auditWalk(root)
	if err != nil {
		t.Fatal(err)
	}
	want := []string{"a/1.txt", "b/2.txt", "c.txt"}
	if strings.Join(got, ",") != strings.Join(want, ",") {
		t.Errorf("got %v, want %v", got, want)
	}
}

// Sibling suggestion must look at the SAME directory only — walking the whole
// subtree would let a huge directory like docs/site/ outvote the real one.
func TestUnitAuditSuggestFromSiblings(t *testing.T) {
	all := []string{
		"dist/components/a.html", "dist/components/b.html",
		"dist/other/deep/x.html", "dist/components/new.html",
	}
	byPath := map[string]classification{}
	for _, p := range all {
		byPath[p] = classifyPath(p)
	}
	byPath["dist/components/new.html"] = classification{Path: "dist/components/new.html", Kind: "unknown"}
	s := suggestFromSiblings("dist/components/new.html", all, byPath)
	if s == nil {
		t.Fatal("no sibling suggestion")
	}
	if s.Kind != "programmatic" {
		t.Errorf("kind = %s, want programmatic", s.Kind)
	}
	if s.Total != 2 {
		t.Errorf("counted %d siblings, want 2 (the deep one is in another directory)", s.Total)
	}
	// a file with no classified siblings gets nothing rather than a guess
	if got := suggestFromSiblings("lonely/x.txt", []string{"lonely/x.txt"}, byPath); got != nil {
		t.Errorf("expected no suggestion, got %+v", got)
	}
}

func TestUnitAuditParentDir(t *testing.T) {
	for in, want := range map[string]string{
		"a/b/c.txt": "a/b/",
		"c.txt":     "",
		"a/c.txt":   "a/",
	} {
		if got := parentDir(in); got != want {
			t.Errorf("parentDir(%q) = %q, want %q", in, got, want)
		}
	}
}

// Drift is "modified relative to HEAD". An untracked file is NOT drift — it
// is the normal state after `make build`, and reporting it would make the
// strict mode red on every fresh build.
func TestUnitAuditDriftIgnoresUntracked(t *testing.T) {
	root := t.TempDir()
	run := func(args ...string) {
		t.Helper()
		if err := runGit(root, args...); err != nil {
			t.Skipf("git unavailable: %v", err)
		}
	}
	run("init", "-q")
	run("config", "user.email", "t@example.com")
	run("config", "user.name", "t")
	tracked := filepath.Join(root, "dist", "out.css")
	if err := os.MkdirAll(filepath.Dir(tracked), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(tracked, []byte("a{}"), 0o644); err != nil {
		t.Fatal(err)
	}
	run("add", "-A")
	run("commit", "-qm", "init")

	programmatic := []classification{
		{Path: "dist/out.css", Kind: "programmatic", Tool: "tailwind"},
		{Path: "dist/globals.css", Kind: "programmatic", Tool: "tailwind"},
	}
	if d := detectDrift(root, programmatic); len(d) != 0 {
		t.Fatalf("a clean tree reported drift: %v", d)
	}
	// untracked: still not drift
	if err := os.WriteFile(filepath.Join(root, "dist", "globals.css"), []byte("b{}"), 0o644); err != nil {
		t.Fatal(err)
	}
	if d := detectDrift(root, programmatic); len(d) != 0 {
		t.Errorf("an untracked build output was reported as drift: %v", d)
	}
	// modified tracked file: that IS drift
	if err := os.WriteFile(tracked, []byte("a{color:red}"), 0o644); err != nil {
		t.Fatal(err)
	}
	d := detectDrift(root, programmatic)
	if len(d) != 1 || d[0].Path != "dist/out.css" {
		t.Errorf("drift = %v, want [dist/out.css]", d)
	}
}

// runGit is local to this test: the production code only ever runs
// `git status`, and a general git helper would be unused API.
func runGit(dir string, args ...string) error {
	cmd := exec.Command("git", args...)
	cmd.Dir = dir
	return cmd.Run()
}

// Ownership of docs/demos comes from the manifests the generating tools write,
// not from the path. Guessing from the path is what made this audit report 331
// generated files as owned by nobody — the directory looked hand-authored
// because most of it once was, and the classification was never revisited when
// example-oracle and example-fixture took it over.
func TestUnitAuditDemoOwnershipComesFromManifests(t *testing.T) {
	root := repoRoot(t)
	if _, err := os.Stat(filepath.Join(root, "docs/example-oracle.json")); err != nil {
		t.Skip("unbuilt tree: no manifest to read")
	}
	wd, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	if err := os.Chdir(root); err != nil { // inManifest resolves against the cwd
		t.Fatal(err)
	}
	defer os.Chdir(wd)

	oracle := readManifestPages(t, root, "docs/example-oracle.json", "out")
	if len(oracle) == 0 {
		t.Fatal("example-oracle claims no pages")
	}
	for _, p := range oracle {
		c := classifyPath(p)
		if c.Kind != "programmatic" {
			t.Errorf("%s is written by example-oracle but classified %s", p, c.Kind)
			break
		}
	}
	// and a page no manifest claims stays hand-authored: the message-scroller
	// examples cannot be bundled (external deps) and are recorded as exempt
	if c := classifyPath("docs/demos/message-scroller-streaming.html"); c.Kind != "hand-authored" {
		t.Errorf("an unclaimed demo page is classified %s, want hand-authored", c.Kind)
	}
}

func readManifestPages(t *testing.T, root, manifest, field string) []string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(root, manifest))
	if err != nil {
		t.Fatal(err)
	}
	var rows []map[string]any
	if err := json.Unmarshal(b, &rows); err != nil {
		t.Fatal(err)
	}
	var out []string
	for _, r := range rows {
		if v, ok := r[field].(string); ok && strings.HasPrefix(v, "docs/demos/") {
			out = append(out, v)
		}
	}
	return out
}
