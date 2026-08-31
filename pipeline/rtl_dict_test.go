package main

import (
	"os"
	"os/exec"
	"testing"
)

// The committed src/registry/rtl-translations.json was written by the JS
// rtl-dict; the Go one must produce byte-identical output or the whole RTL
// demo family shifts underneath gates that hash it.
func TestUnitRtlDictParity(t *testing.T) {
	root := ".."
	committed, err := os.ReadFile(root + "/src/registry/rtl-translations.json")
	if err != nil {
		t.Skip(err)
	}
	pipelineBin := t.TempDir() + "/pipeline"
	if out, err := exec.Command("go", "build", "-o", pipelineBin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build: %v\n%s", err, out)
	}
	cmd := exec.Command(pipelineBin, "rtl-dict")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("rtl-dict: %v\n%s", err, out)
	}
	got, err := os.ReadFile(root + "/src/registry/rtl-translations.json")
	if err != nil {
		t.Fatal(err)
	}
	if string(got) != string(committed) {
		t.Fatalf("rtl-translations.json diverges from the JS-written bytes (first 400:\n%s)", got[:400])
	}
}
