package main

import (
	"os/exec"
	"path/filepath"
	"testing"
)

// emit parity: the Go emitter's rerun leaves dist untouched (the committed
// pages are the JS emitter's bytes; Go must reproduce them exactly).
func TestUnitEmitParity(t *testing.T) {
	root := "/home/newlix/github/at-least/shadless"
	if out, _ := exec.Command("git", "-C", root, "status", "--porcelain", "--", "dist").Output(); len(out) != 0 {
		t.Skipf("dist dirty: %s", out[:min(len(out), 120)])
	}
	pipelineBin := filepath.Join(t.TempDir(), "pipeline")
	if out, err := exec.Command("go", "build", "-o", pipelineBin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build: %v\n%s", err, out)
	}
	cmd := exec.Command(pipelineBin, "emit")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("emit: %v\n%s", err, out)
	}
	if out, _ := exec.Command("git", "-C", root, "status", "--porcelain", "--", "dist", "build/emit").Output(); len(out) != 0 {
		t.Fatalf("rebuild changed outputs:\n%s", out)
	}
}
