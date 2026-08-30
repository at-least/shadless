package main

// Derived `produces`: where a node's real output set is DATA, not a glob.
//
// Two nodes write into dist/components, and one glob described the whole
// directory for both. `emit` writes the 23 static-tier pages; `demo` writes
// the other 28. Both declared `dist/components/*.html` — all 51.
//
// Over-declaring an output is not the harmless direction:
//
//   - the undeclared-READ check treats everything a node's dependency closure
//     `produces` as covered. `emit` claiming all 51 pages therefore excused
//     every node downstream of it for READING any page in there without
//     declaring it — and reading a file that is not in your key is precisely
//     the stale-green that check exists to find.
//   - the write check is per-node, so the same declaration also excused `emit`
//     itself for writing any of the 28 pages it does not own.
//
//     Not more than that, and the limit is worth stating: tools/fs-record.mjs
//     records READS only, so a JS node writing where it has not declared is
//     invisible to the access check no matter what any `produces` says. Only
//     the -j1 write check sees those.
//   - OutputsPresent stats a pattern's literal prefix, so
//     `dist/components/*.html` was satisfied by the directory merely existing.
//     Against the real file list, a missing page is a missing output.
//
// The tier split is data — src/registry/tiers.json — so it is read from there
// rather than transcribed. Commit 85906cb collapsed the three encodings of
// "which components ship, at which tier" into that file; a hand-kept list of
// 23 names in nodes.go would be a fourth, and the one that goes stale
// silently.
//
// `shipped` below restates the predicate tools/demo.mjs applies to pick the
// pages it writes. That is a second reader of the same data, not a second
// copy of it, and the drift is self-policing: if the two ever disagree, demo
// writes a page its `produces` does not name and the access check reports it.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
)

// The two patterns that stood for "the component pages". A node's remaining
// `produces` entries are kept as declared; only these are substituted, the
// same way fanContracts narrows the broad input patterns it replaces.
const (
	componentPagesGlob = "dist/components/*.html"
	componentPagesNoRT = "!dist/components/*-rtl-*.html"
)

// producesFrom maps a node id to the real output set for its component pages.
// Returning an error fails graph load: a derivation that cannot read its data
// must not quietly fall back to the glob it is replacing.
var producesFrom = map[NodeID]func(root string) ([]string, error){
	NEmit: staticPages,
	NDemo: nonStaticPages,
}

// tiersPath is where "which components ship, at which tier" lives.
const tiersPath = "src/registry/tiers.json"

type componentTier struct {
	Tier string `json:"tier"`
	Emit bool   `json:"emit"`
}

func componentTiers(root string) (map[string]componentTier, error) {
	b, err := os.ReadFile(filepath.Join(root, tiersPath))
	if err != nil {
		return nil, err
	}
	var m map[string]componentTier
	if err := json.Unmarshal(b, &m); err != nil {
		return nil, fmt.Errorf("%s: %w", tiersPath, err)
	}
	if len(m) == 0 {
		return nil, fmt.Errorf("%s: no components", tiersPath)
	}
	return m, nil
}

// shipped reports whether a component gets a page in dist/components at all.
// Three tiers ship wholesale; the stragglers that ship despite their tier
// (field, carousel, menubar, navigation-menu) carry `emit: true`.
func shipped(t componentTier) bool {
	switch t.Tier {
	case "static", "kernel", "trivial-js":
		return true
	}
	return t.Emit
}

func pages(root string, want func(componentTier) bool) ([]string, error) {
	m, err := componentTiers(root)
	if err != nil {
		return nil, err
	}
	var out []string
	for name, t := range m {
		if want(t) {
			out = append(out, "dist/components/"+name+".html")
		}
	}
	sort.Strings(out)
	if len(out) == 0 {
		return nil, fmt.Errorf("%s: no pages matched", tiersPath)
	}
	return out, nil
}

// staticPages are the pages src/emitter/index.mjs writes: tier "static", one
// html per component, and it fails loudly if the IR count disagrees with
// tiers.json.
func staticPages(root string) ([]string, error) {
	return pages(root, func(t componentTier) bool { return t.Tier == "static" })
}

// nonStaticPages are the pages tools/demo.mjs writes: every shipped component
// EXCEPT the static ones, which it finds already emitted and leaves alone.
func nonStaticPages(root string) ([]string, error) {
	return pages(root, func(t componentTier) bool { return shipped(t) && t.Tier != "static" })
}

// applyDerivedProduces substitutes the explicit page list for the directory
// glob, in place, leaving every other `produces` entry as declared.
func applyDerivedProduces(root string, in []Node) ([]Node, error) {
	out := append([]Node(nil), in...)
	for i := range out {
		derive := producesFrom[out[i].ID]
		if derive == nil {
			continue
		}
		list, err := derive(root)
		if err != nil {
			return nil, fmt.Errorf("produces for %s: %w", out[i].ID, err)
		}
		var kept []string
		substituted := false
		for _, p := range out[i].Produces {
			if p == componentPagesGlob || p == componentPagesNoRT {
				substituted = true
				continue
			}
			kept = append(kept, p)
		}
		if !substituted {
			return nil, fmt.Errorf("produces for %s: no %q to substitute", out[i].ID, componentPagesGlob)
		}
		out[i].Produces = append(list, kept...)
	}
	return out, nil
}
