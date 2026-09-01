package main

import (
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// rewritePaths regression pins (2026-08-25 incident): the bare `out.css`
// form is NOT dead code — `[^"]*-out\.css` can never match it.
func TestUnitRewritePaths(t *testing.T) {
	t6 := `<link href="tooltip-out.css"><script src="../../dist/shadless.js"></script><script src="../../dist/js/tabs.js"></script>`
	out := rewritePaths(t6)
	if !strings.Contains(out, `href="../out.css"`) {
		t.Errorf("per-component css → unified: %q", out)
	}
	if !strings.Contains(out, `src="../shadless.js"`) {
		t.Errorf("base path: %q", out)
	}
	if !strings.Contains(out, `src="../js/tabs.js"`) {
		t.Errorf("component file into dist/js/: %q", out)
	}
	if got := rewritePaths(`<link href="out.css">`); !strings.Contains(got, `href="../out.css"`) {
		t.Errorf("bare out.css form (the deleted-regex incident): %q", got)
	}
}

func TestUnitEnsureLink(t *testing.T) {
	if got := ensureLink("<head><title>x</title></head>"); got != "<head>\n<link rel=\"stylesheet\" href=\"../out.css\"><title>x</title></head>" {
		t.Errorf("adds when missing: %q", got)
	}
	kept := `<head><link rel="stylesheet" href="../out.css"></head>`
	if got := ensureLink(kept); got != kept {
		t.Errorf("keeps existing: %q", got)
	}
}

// demo parity: the Go demo rebuild leaves the JS-built dist tree untouched.
func TestUnitDemoParity(t *testing.T) {
	root := "/home/newlix/github/at-least/shadless"
	if out, _ := exec.Command("git", "-C", root, "status", "--porcelain", "--", "dist").Output(); len(out) != 0 {
		t.Skipf("dist dirty, skip parity: %s", strings.SplitN(string(out), "\n", 2)[0])
	}
	if _, err := exec.Command("git", "-C", root, "ls-files", "dist/globals.css").Output(); err != nil {
		t.Skip(err)
	}
	pipelineBin := filepath.Join(t.TempDir(), "pipeline")
	if out, err := exec.Command("go", "build", "-o", pipelineBin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build: %v\n%s", err, out)
	}
	cmd := exec.Command(pipelineBin, "demo")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("demo: %v\n%s", err, out)
	}
	if out, _ := exec.Command("git", "-C", root, "status", "--porcelain", "--", "dist").Output(); len(out) != 0 {
		t.Fatalf("rebuild changed dist:\n%s", out)
	}
}
