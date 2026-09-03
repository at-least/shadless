package main

// docs-upstream-mirror: copies the slice of the pinned upstream docs tree
// that docs-build/docs-fidelity actually read (docsRadixDir, and the 5
// upstream-sourced guides.source entries) from the gitignored, re-cloned-
// per-pin .upstream/ into the tracked docsUpstreamMirror (generated/
// docs-upstream/), so it has the same properties as generated/ir: a
// git-diffable copy of upstream content, checked in, that a re-pin changes
// visibly instead of silently. See nodes.go's docsUpstreamMirror doc
// comment for why this needs to exist alongside upstreamDocsGlob.

import (
	"fmt"
	"os"
	"path/filepath"
)

// docsUpstreamFiles: the individual upstream docs files copied by name,
// beyond the whole-directory copy of components/radix (docsRadixDir) below.
// One entry per upstream-sourced guides[].source (docs_guides.go) — kept as
// a literal list, not derived from `guides`, so a newly added upstream-
// sourced guide fails loud here (file not found in the mirror) rather than
// silently reading straight from .upstream/ again.
var docsUpstreamFiles = []string{
	"rtl/index.mdx",
	"utils/shimmer.mdx",
	"utils/scroll-fade.mdx",
	"helpers/ai-sdk.mdx",
	"helpers/tanstack-ai.mdx",
}

func runDocsUpstreamMirror() int {
	const upstreamDocsDir = ".upstream/shadcn-ui/apps/v4/content/docs"

	if err := copyTree(filepath.Join(upstreamDocsDir, "components/radix"), filepath.Join(docsUpstreamMirror, "components/radix")); err != nil {
		fmt.Fprintln(os.Stderr, "docs-upstream-mirror: components/radix:", err)
		return 1
	}
	n := 0
	for _, rel := range docsUpstreamFiles {
		src := filepath.Join(upstreamDocsDir, rel)
		b, err := os.ReadFile(src)
		if err != nil {
			fmt.Fprintln(os.Stderr, "docs-upstream-mirror:", err)
			return 1
		}
		dst := filepath.Join(docsUpstreamMirror, rel)
		if err := os.MkdirAll(filepath.Dir(dst), 0o755); err != nil {
			fmt.Fprintln(os.Stderr, "docs-upstream-mirror:", err)
			return 1
		}
		if err := os.WriteFile(dst, b, 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "docs-upstream-mirror:", err)
			return 1
		}
		n++
	}
	radixCount := 0
	if ents, err := os.ReadDir(filepath.Join(docsUpstreamMirror, "components/radix")); err == nil {
		radixCount = len(ents)
	}
	fmt.Printf("docs-upstream-mirror: %d components/radix files + %d guide files -> %s\n", radixCount, n, docsUpstreamMirror)
	return 0
}
