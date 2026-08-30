package main

// The undeclared-read check.
//
// This check exists to catch a stale GREEN — a node whose key omits a file it
// actually judges — so its own failure mode matters more than most: a bug
// here that made it report nothing would restore exactly the blind spot it
// was written to remove, and silently. Hence the negative cases below are as
// deliberate as the positive one.

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// runInDir runs a command with the repo root as its working directory.
func runInDir(root string, argv []string) error {
	c := exec.Command(argv[0], argv[1:]...)
	c.Dir = root
	return c.Run()
}

// writeTree lays out files and returns the root.
func writeTree(t *testing.T, files map[string]string) string {
	t.Helper()
	root := t.TempDir()
	for p, c := range files {
		full := filepath.Join(root, p)
		if err := os.MkdirAll(filepath.Dir(full), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(full, []byte(c), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

// testlog writes a log in the format `go test -test.testlogfile` produces.
func writeTestlog(t *testing.T, root string, lines ...string) string {
	t.Helper()
	var b strings.Builder
	b.WriteString("# test log\ngetenv PWD\n")
	for _, l := range lines {
		b.WriteString(l + "\n")
	}
	p := filepath.Join(t.TempDir(), "test.log")
	if err := os.WriteFile(p, []byte(b.String()), 0o644); err != nil {
		t.Fatal(err)
	}
	return p
}

func TestUnitTestlogOpens(t *testing.T) {
	root := writeTree(t, map[string]string{
		"a.txt":     "a",
		"sub/b.txt": "b",
	})
	log := writeTestlog(t, root,
		"open "+filepath.Join(root, "a.txt"),
		"open "+filepath.Join(root, "sub/b.txt"),
		"open "+filepath.Join(root, "a.txt"), // duplicate: reported once
		"open "+filepath.Join(root, "sub"),   // a directory
		"open "+filepath.Join(root, "gone.txt"),
		"open /usr/lib/go/src/os/file.go", // outside the repo
		"stat "+filepath.Join(root, "a.txt"),
		"getenv HOME",
	)
	got, err := testlogOpens(root, log)
	if err != nil {
		t.Fatal(err)
	}
	want := []string{"a.txt", "sub/b.txt"}
	if strings.Join(got, ",") != strings.Join(want, ",") {
		t.Errorf("got %v, want %v", got, want)
	}
}

// The point of the check: a file the node reads but does not declare.
func TestUnitUndeclaredReadsFindsTheGap(t *testing.T) {
	root := writeTree(t, map[string]string{
		"src/a.mjs":  "a",
		"dist/x.css": "x",
	})
	n := Node{ID: "g", Kind: "gate", Inputs: []string{"src/**"}}
	got, err := undeclaredReads(root, nil, n, []string{"src/a.mjs", "dist/x.css"})
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 1 || got[0] != "dist/x.css" {
		t.Errorf("got %v, want [dist/x.css]", got)
	}
}

// A declared file must not be reported, whichever glob shape covers it.
func TestUnitUndeclaredReadsHonoursGlobShapes(t *testing.T) {
	root := writeTree(t, map[string]string{
		"src/deep/a.mjs": "a",
		"gates/l.json":   "{}",
		"one.txt":        "1",
	})
	n := Node{ID: "g", Kind: "gate",
		Inputs: []string{"src/**", "gates/*.json", "one.txt"}}
	got, err := undeclaredReads(root, nil, n, []string{"src/deep/a.mjs", "gates/l.json", "one.txt"})
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 0 {
		t.Errorf("declared files were reported as undeclared: %v", got)
	}
}

// A node may read back what it produces; requiring it to also declare its own
// output as an input would be circular.
func TestUnitUndeclaredReadsAllowsOwnOutput(t *testing.T) {
	root := writeTree(t, map[string]string{
		"dist/out.css":   "x",
		"dist/css/a.css": "y",
	})
	n := Node{ID: "b", Kind: "build",
		Inputs:   []string{},
		Produces: []string{"dist/out.css", "dist/css"},
	}
	got, err := undeclaredReads(root, nil, n, []string{"dist/out.css", "dist/css/a.css"})
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 0 {
		t.Errorf("a node reading its own output was flagged: %v", got)
	}
}

// A never-fresh node has no key, so nothing about it can go stale-green and
// there is nothing to declare. `reproducible` is the real instance.
func TestUnitUndeclaredReadsSkipsNeverFresh(t *testing.T) {
	root := writeTree(t, map[string]string{"anything.txt": "x"})
	n := Node{ID: "reproducible", Kind: "gate", Inputs: nil}
	got, err := undeclaredReads(root, nil, n, []string{"anything.txt"})
	if err != nil {
		t.Fatal(err)
	}
	if got != nil {
		t.Errorf("a never-fresh node was checked: %v", got)
	}
}

func TestUnitIsGoTest(t *testing.T) {
	if !isGoTest([]string{"go", "test", "-C", "pipeline", "."}) {
		t.Error("a go test command was not recognised")
	}
	for _, argv := range [][]string{
		{"node", "tools/unit-check.mjs"},
		{"./build/pipeline", "build-js"},
		{"go"},
		{},
	} {
		if isGoTest(argv) {
			t.Errorf("%v was treated as a go test command", argv)
		}
	}
}

// End to end on the real graph: every Go-test gate's declared inputs must
// cover what it actually reads. This is the check guarding itself — it runs
// the gates' own testlogs against their declarations without the runner.
//
// Skipped unless the tree is built, because an unbuilt tree makes the gates
// read nothing and the check vacuously pass.
func TestUnitDeclaredInputsCoverRealReads(t *testing.T) {
	root := repoRoot(t)
	if _, err := os.Stat(filepath.Join(root, "dist", "shadless.css")); err != nil {
		t.Skip("unbuilt tree: nothing to check")
	}
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	// Only the cheap, pure gates: the point here is the wiring, and the
	// runner checks every gate on a real run anyway.
	for _, id := range []NodeID{NLedger, NCssDirection} {
		n, ok := g.Node(id)
		if !ok {
			t.Fatalf("no node %s", id)
		}
		var opens []string
		for i, argv := range n.Run {
			if !isGoTest(argv) {
				continue
			}
			log := filepath.Join(t.TempDir(), "t.log")
			cmd := append(append([]string{}, argv...), "-test.testlogfile="+log)
			if err := runInDir(root, cmd); err != nil {
				t.Fatalf("%s command %d failed: %v", id, i, err)
			}
			got, err := testlogOpens(root, log)
			if err != nil {
				t.Fatal(err)
			}
			opens = append(opens, got...)
		}
		if len(opens) == 0 {
			t.Errorf("%s: the gate opened no file in the repo — the testlog is not being captured", id)
			continue
		}
		bad, err := undeclaredReads(root, g, n, opens)
		if err != nil {
			t.Fatal(err)
		}
		if len(bad) > 0 {
			t.Errorf("%s reads files it does not declare in `inputs`: %v", id, bad)
		}
	}
}

// The merkle chain already covers a dependency's declarations: its key hashes
// its own inputs, and that key is folded into this node's. So reading either
// a dependency's input or its output is not a stale-green risk, and reporting
// it would teach people to ignore the report — on `emit` it was the
// difference between 64 findings and one real one.
func TestUnitUndeclaredReadsExemptsDependencyClosure(t *testing.T) {
	root := writeTree(t, map[string]string{
		"up/src.tsx":     "x", // dep's INPUT
		"ir/out.json":    "y", // dep's OUTPUT
		"other/loose.js": "z", // nobody's
	})
	g, err := newGraph([]Node{
		{ID: "dep", Kind: "build", Tier: "fast", Run: [][]string{{"true"}},
			Inputs: []string{"up/**"}, Produces: []string{"ir"}},
		{ID: "me", Kind: "build", Tier: "fast", Run: [][]string{{"true"}},
			Needs: []NodeID{"dep"}, Inputs: []string{}, Produces: []string{"dist"}},
	})
	if err != nil {
		t.Fatal(err)
	}
	me, _ := g.Node("me")
	got, err := undeclaredReads(root, g, me, []string{"up/src.tsx", "ir/out.json", "other/loose.js"})
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 1 || got[0] != "other/loose.js" {
		t.Errorf("got %v, want only [other/loose.js] — the dependency's input and output are already in the key", got)
	}
	// and without the graph the exemption cannot apply, so everything shows
	bare, err := undeclaredReads(root, nil, me, []string{"up/src.tsx", "ir/out.json"})
	if err != nil {
		t.Fatal(err)
	}
	if len(bare) != 2 {
		t.Errorf("with no graph, got %v, want both reported", bare)
	}
}

// The real graph's most important build node: convert reads the pinned
// registry sources it converts. Relying on `pin`'s key — which hashes only
// .git/HEAD — left every working-tree edit invisible, including the overlay
// patch series the re-pin drill applies with `git apply --3way`.
func TestUnitConvertDeclaresUpstreamRegistry(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	n, ok := g.Node(NConvert)
	if !ok {
		t.Fatal("no convert node")
	}
	want := ".upstream/shadcn-ui/apps/v4/registry/bases/radix/**"
	if !contains(n.Inputs, want) {
		t.Errorf("convert must declare %q, or an edit to the registry it converts "+
			"leaves it falsely fresh; inputs = %v", want, n.Inputs)
	}
}
