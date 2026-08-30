package main

// Root discovery for the tailwind wrapper.
//
// tw resolves `in`, `out` and the CLI binary against the repo root, never
// against the compile cwd — controlling that cwd is the entire point of the
// wrapper. So getting the root wrong silently compiles the wrong thing.

import (
	"os"
	"path/filepath"
	"testing"
)

func TestUnitFindRepoRootMarker(t *testing.T) {
	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, "pipeline"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "pipeline", "nodes.go"), nil, 0o644); err != nil {
		t.Fatal(err)
	}
	deep := filepath.Join(root, "a", "b")
	if err := os.MkdirAll(deep, 0o755); err != nil {
		t.Fatal(err)
	}
	got, err := findRepoRoot(deep)
	if err != nil {
		t.Fatal(err)
	}
	if got != root {
		t.Errorf("got %q, want %q", got, root)
	}
}

// A deployed binary carries no pipeline/ directory — a container with
// build/pipeline and nothing else finds no marker at all. SHADLESS_ROOT is
// how such a caller says where the tree is.
func TestUnitFindRepoRootEnvOverride(t *testing.T) {
	t.Setenv("SHADLESS_ROOT", "/somewhere/else")
	got, err := findRepoRoot(t.TempDir())
	if err != nil {
		t.Fatal(err)
	}
	if got != "/somewhere/else" {
		t.Errorf("got %q, want the override", got)
	}
}

// Without a marker and without the override, the error has to say what to do
// about it — this exact case cost a container run to diagnose.
func TestUnitFindRepoRootFailureIsActionable(t *testing.T) {
	t.Setenv("SHADLESS_ROOT", "")
	dir := t.TempDir() // no marker anywhere above a temp dir
	_, err := findRepoRoot(dir)
	if err == nil {
		t.Fatal("a tree with no marker resolved a root")
	}
	if !contains([]string{err.Error()}, err.Error()) || len(err.Error()) == 0 {
		t.Fatal("empty error")
	}
	if want := "SHADLESS_ROOT"; !containsStr(err.Error(), want) {
		t.Errorf("error %q does not mention %s", err, want)
	}
}

func containsStr(hay, needle string) bool {
	return len(hay) >= len(needle) && (func() bool {
		for i := 0; i+len(needle) <= len(hay); i++ {
			if hay[i:i+len(needle)] == needle {
				return true
			}
		}
		return false
	})()
}
