package main

import (
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

// resolve-skins must produce byte-identical build/resolved-ui to the JS
// original. The snapshot in internal tsx and twmerge covers the scanners;
// this test covers the composition.
//
// It runs resolve-skins via the pipeline binary (the real entry) against a
// tree built by the JS version committed in git, and diffs. A mismatch is
// never accepted silently — resolve-skins decides what markup the whole
// product ships.
func TestUnitResolveSkinsParity(t *testing.T) {
	root := ".."
	tmp := t.TempDir()
	// build/resolved-ui is .gitignore'd and produced by whichever
	// implementation ran last; it is the JS version today. Save, rerun with
	// Go, diff byte-for-byte, restore.
	existing := filepath.Join(root, "build", "resolved-ui")
	if _, err := os.Stat(filepath.Join(existing, "ui")); err != nil {
		t.Skip("no existing resolved-ui tree to compare against")
	}
	backup := filepath.Join(tmp, "resolved-ui")
	if err := exec.Command("cp", "-a", existing, backup).Run(); err != nil {
		t.Fatalf("backup: %v", err)
	}
	t.Cleanup(func() {
		os.RemoveAll(existing)
		if out, err := exec.Command("cp", "-a", backup, existing).CombinedOutput(); err != nil {
			t.Errorf("restore: %v\n%s", err, out)
		}
	})

	pipelineBin := filepath.Join(tmp, "pipeline")
	if out, err := exec.Command("go", "build", "-o", pipelineBin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build pipeline: %v\n%s", err, out)
	}
	cmd := exec.Command(pipelineBin, "resolve-skins")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("resolve-skins: %v\n%s", err, out)
	}
	if out, err := exec.Command("diff", "-r", backup, existing).CombinedOutput(); err != nil {
		t.Fatalf("resolved-ui diverged from the JS-built tree:\n%s", out)
	}
}
