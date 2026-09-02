package main

// reproducible — the committed generated trees must equal what the pipeline
// just produced.
//
// The ONLY authority on hand-edits to generated files. The pre-commit hook it
// replaced tried to guess ("vendor outputs staged without a source change")
// with three layers of heuristics and a 6-second rebuild, and still had to be
// bypassed for legitimate commits. Guessing is gone: run the pipeline, then the
// tree must be clean under the generated paths.
//
// Keeping dist/ in git is deliberate: a re-pin PR's most useful review surface
// is the diff of what consumers actually receive. docs/site/ is NOT in that
// list any more (2026-08-31) — it is a 12 MB rendering of trees already
// reviewed here, and the docs site is moving to VitePress, whose output is a
// build artifact like any other.

import (
	"fmt"
	"os/exec"
	"strings"
)

// Generated roots, in the order the JS gate listed them (the count appears in
// the PASS line).
// tools/contracts/out is deliberately absent: it is oracle scratch, no longer
// committed, so there is no committed tree to compare it against.
var generatedRoots = []string{
	"dist", "docs/catalog.json", "docs/demos", "docs/example-oracle.json",
	"docs/components", "docs/guides", "docs/index.md", "docs/content-map.json",
	"docs/.vitepress/sidebar.json",
	"generated/ir", "src/kernel/*.html",
}

func gateReproducible(root string) error {
	args := append([]string{"status", "--porcelain", "--untracked-files=all", "--"}, generatedRoots...)
	cmd := exec.Command("git", args...)
	cmd.Dir = root
	out, err := cmd.Output()
	if err != nil {
		return fmt.Errorf("FAIL  reproducible (git status failed: %v)", err)
	}
	var lines []string
	for _, l := range strings.Split(string(out), "\n") {
		if l != "" {
			lines = append(lines, l)
		}
	}
	if len(lines) > 0 {
		shown := lines
		more := ""
		if len(shown) > 40 {
			shown, more = shown[:40], fmt.Sprintf("\n  … +%d more", len(lines)-40)
		}
		return fmt.Errorf("FAIL  reproducible (%d generated paths differ from the committed tree)\n  %s%s\n\n"+
			"  The pipeline produced different bytes than what is committed under the generated paths.\n"+
			"  - a source change moved an output  → commit source + output together (git add -A)\n"+
			"  - a generated file was hand-edited → the edit is gone; put it in the tool or overlay that owns the file\n"+
			"  Inspect: git diff -- <path>",
			len(lines), strings.Join(shown, "\n  "), more)
	}
	fmt.Printf("PASS  reproducible (%d generated roots match the committed tree)\n", len(generatedRoots))
	return nil
}
