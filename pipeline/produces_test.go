package main

// The derived `produces` sets. Named TestUnit* so they run as part of the
// `unit` gate.
//
// What is worth asserting is not the arithmetic of produces.go — restating it
// here would prove nothing — but the property the over-declared glob broke:
// the two nodes that write into dist/components must own DISJOINT halves of
// it, and between them the whole shipped set.

import (
	"strings"
	"testing"
)

func derived(t *testing.T, root string, id NodeID) []string {
	t.Helper()
	g, err := LoadGraphAt(root)
	if err != nil {
		t.Fatalf("graph does not load: %v", err)
	}
	n, ok := g.Node(id)
	if !ok {
		t.Fatalf("no node %s", id)
	}
	return n.Produces
}

func TestUnitComponentPagesAreOwnedOnce(t *testing.T) {
	root := repoRoot(t)
	emit := derived(t, root, NEmit)
	demo := derived(t, root, NDemo)

	pagesOf := func(list []string) map[string]bool {
		out := map[string]bool{}
		for _, p := range list {
			if strings.HasPrefix(p, "dist/components/") && strings.HasSuffix(p, ".html") {
				out[p] = true
			}
		}
		return out
	}
	e, d := pagesOf(emit), pagesOf(demo)

	// the glob has to be gone, or the substitution silently did nothing
	for _, list := range [][]string{emit, demo} {
		for _, p := range list {
			if p == componentPagesGlob || p == componentPagesNoRT {
				t.Errorf("%q survived substitution", p)
			}
		}
	}

	for p := range e {
		if d[p] {
			t.Errorf("%s is claimed by both emit and demo", p)
		}
	}
	// 51 shipped pages: 23 static from emit, 28 from demo. The count is
	// asserted because it is the number tools/demo.mjs itself insists on
	// (`emitted !== 51`) — if tiers.json changes, both must move together.
	if len(e)+len(d) != 51 {
		t.Errorf("emit %d + demo %d pages, want 51 shipped", len(e), len(d))
	}
	if len(e) != 23 {
		t.Errorf("emit produces %d static pages, want 23", len(e))
	}

	// spot the boundary from both sides: a static page belongs to emit, a
	// kernel page and both `emit: true` stragglers belong to demo
	if !e["dist/components/button.html"] {
		t.Error("emit does not claim the static page button.html")
	}
	for _, p := range []string{"dist/components/dialog.html", "dist/components/carousel.html", "dist/components/field.html"} {
		if !d[p] {
			t.Errorf("demo does not claim %s", p)
		}
		if e[p] {
			t.Errorf("emit claims %s, which it does not write", p)
		}
	}

	// non-page entries survive untouched
	for _, want := range []string{"dist/shadless.css", "build/emit"} {
		found := false
		for _, p := range emit {
			if p == want {
				found = true
			}
		}
		if !found {
			t.Errorf("emit lost its %q declaration", want)
		}
	}
}

// A derivation that cannot read its data must fail the graph load rather than
// fall back to the glob it replaces: a silent fallback restores exactly the
// over-declaration this file exists to remove.
func TestUnitDerivedProducesFailsLoud(t *testing.T) {
	if _, err := applyDerivedProduces(t.TempDir(), Nodes); err == nil {
		t.Fatal("produces derived from a tree with no tiers.json, without error")
	}
}
