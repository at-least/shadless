package main

import (
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// substituteAndPatch: pure-string translation, ported from tools/rtl-lib.mjs.
// Each assertion pins a Wave-G bug the original tool shipped (bare <html>
// lang injection, Persian dir=rtl, longest-first substitution, dir
// attribute boundary), exactly the cases tools/unit/rtl.mjs asserted on.
func TestUnitSubstituteAndPatch(t *testing.T) {
	type dictT = map[string]struct {
		Dir    string            `json:"dir"`
		Values map[string]string `json:"values"`
	}
	tr := dictT{
		"ar": {Dir: "rtl", Values: map[string]string{"title": "عنوان", "loading": "جار التحميل"}},
		"en": {Dir: "ltr", Values: map[string]string{"title": "Title", "loading": "Loading…"}},
		"he": {Dir: "rtl", Values: map[string]string{"title": "כותרת"}},
	}

	// dir flips to ltr for en; content substituted; missing-from-key warns
	out := substituteAndPatch(`<!doctype html><html lang="ar" dir="rtl"><body><div dir="rtl">عنوان</div></body></html>`,
		tr, "ar", "en", tr["en"].Values, "")
	if !strings.Contains(out, `<html lang="en"`) {
		t.Errorf("lang patched: %q", out)
	}
	if strings.Contains(out, `dir="rtl"`) {
		t.Errorf("en must flip dir to ltr: %q", out)
	}
	if !strings.Contains(out, "Title") {
		t.Errorf("content: %q", out)
	}

	// bare <html> gets lang injected; he keeps rtl by default
	out = substituteAndPatch("<html><body>x</body></html>", tr, "ar", "he", tr["he"].Values, "")
	if !strings.Contains(out, `<html lang="he"`) {
		t.Errorf("bare html lang injection: %q", out)
	}
	rest := strings.Replace(out, `lang="he"`, "", 1)
	if strings.Contains(rest, `dir="ltr"`) {
		t.Errorf("he must stay rtl: %q", out)
	}

	// explicit toDir wins
	out = substituteAndPatch(`<html lang="ar" dir="rtl"></html>`, tr, "ar", "fa",
		map[string]string{"title": "پ"}, "rtl")
	if !strings.Contains(out, `dir="rtl"`) {
		t.Errorf("explicit dir: %q", out)
	}

	// longest-first: "Loading" inside "Loading…" untouched
	trLongest := dictT{
		"ar": {Dir: "rtl", Values: map[string]string{"a": "جار", "b": "جار التحميل"}},
		"en": {Dir: "ltr", Values: map[string]string{"a": "Loading", "b": "Loading…"}},
	}
	out = substituteAndPatch("<p>جار التحميل</p>", trLongest, "ar", "en", trLongest["en"].Values, "")
	if out != "<p>Loading…</p>" {
		t.Errorf("longest-first: %q", out)
	}

	// attribute boundary: data-dir="ltr" is not the dir attribute
	out = substituteAndPatch(`<html lang="ar" dir="rtl" data-dir="ltr"></html>`,
		tr, "ar", "en", map[string]string{"title": "T"}, "")
	if !strings.Contains(out, `data-dir="ltr"`) {
		t.Errorf("data-dir untouched: %q", out)
	}
	if !strings.Contains(out, `dir="ltr" data-dir`) {
		t.Errorf("real dir rewritten: %q", out)
	}
}

// build-rtl against the committed tree: rerun must leave checked-in
// outputs unchanged. This proves the Go build-rtl is byte-equivalent to the
// JS one on the real corpus.
func TestUnitBuildRtlParity(t *testing.T) {
	root := "/home/newlix/github/at-least/shadless"
	pipelineBin := filepath.Join(t.TempDir(), "pipeline")
	if out, err := exec.Command("go", "build", "-o", pipelineBin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build: %v\n%s", err, out)
	}
	cmd := exec.Command(pipelineBin, "build-rtl")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("build-rtl: %v\n%s", err, out)
	}
	diff := exec.Command("git", "-C", root, "status", "--porcelain", "--", "docs/demos", "dist/components", "build/rtl-langs.json")
	if out, _ := diff.Output(); len(out) > 0 {
		t.Fatalf("rebuild changed committed outputs:\n%s", out)
	}
}
