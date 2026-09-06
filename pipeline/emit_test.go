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

// mergeRootAttrs exists instead of a plain `[^>]*` regex because a naive scan
// breaks on a ">" inside an attribute value. Drive it into exactly that: an
// open tag whose own attribute value embeds a literal ">".
func TestUnitMergeRootAttrs(t *testing.T) {
	// the case the quote-aware scan is for: a ">" inside a quoted attribute
	// value must not be mistaken for the tag's real close.
	h := `<div class="a>b">body</div>`
	got := mergeRootAttrs(h, []attrPair{{"title", "a > b"}})
	want := `<div class="a>b" title="a &gt; b">body</div>`
	if got != want {
		t.Fatalf("mergeRootAttrs with an embedded > in an attribute value:\n got  %q\n want %q\n(a naive [^>]* scan would insert mid-attribute, e.g. %q)",
			got, want, `<div class="a title="a &gt; b">b">body</div>`)
	}

	// plain case, no embedded ">": still inserts right before the real close.
	h = `<button type="button">Click</button>`
	got = mergeRootAttrs(h, []attrPair{{"aria-label", "Save"}})
	want = `<button type="button" aria-label="Save">Click</button>`
	if got != want {
		t.Fatalf("mergeRootAttrs(%q, ...) = %q, want %q", h, got, want)
	}

	// no attrs: the tag is returned untouched.
	if got := mergeRootAttrs(h, nil); got != h {
		t.Fatalf("mergeRootAttrs with no attrs changed the tag: %q", got)
	}
}
