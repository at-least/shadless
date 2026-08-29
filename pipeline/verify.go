package main

// Declared vs actual writes.
//
// wireit manages a node's DECLARED outputs and is blind to anything else it
// writes. An undeclared write is not harmless: if some other node declares
// that file as an input, the graph's freshness is being driven by a file that
// no node admits to producing.
//
// No node is known to do this today. The check exists because nothing else
// can see it: wireit manages a node's declared outputs and has no way to
// notice the undeclared ones, so "the declarations are honest" was an
// assumption the pipeline had no way to test.
//
// The check is narrowed to the set that can actually do damage: the union of
// every file any node declares as an input. A write outside `produces` that
// nothing reads cannot affect freshness; a write that lands on someone's
// declared input can.

import (
	"fmt"
	"path/filepath"
	"sort"
)

// inputUniverse is every file the graph declares as an input, anywhere.
func inputUniverse(root string, g *Graph) (map[string]string, error) {
	snap := map[string]string{}
	for _, id := range g.IDs() {
		n, _ := g.Node(id)
		if n.NeverFresh() {
			continue
		}
		files, err := Files(root, n.Inputs)
		if err != nil {
			return nil, err
		}
		for _, f := range files {
			if _, done := snap[f]; done {
				continue
			}
			h, err := hashFile(filepath.Join(root, f))
			if err != nil {
				continue
			}
			snap[f] = h
		}
	}
	return snap, nil
}

type Violation struct {
	Path    string
	Readers []NodeID // nodes declaring Path as an input
}

// undeclaredWrites reports files that changed across a node's run, are read by
// somebody, and are not covered by that node's `produces`.
func undeclaredWrites(root string, g *Graph, n Node, before, after map[string]string) ([]Violation, error) {
	produced, err := Files(root, n.Produces)
	if err != nil {
		return nil, err
	}
	declared := map[string]bool{}
	for _, f := range produced {
		declared[f] = true
	}
	// `produces` may name a directory that did not exist before the run, so
	// also treat anything under a declared literal prefix as covered.
	covered := func(p string) bool {
		if declared[p] {
			return true
		}
		for _, pat := range n.Produces {
			re, err := globToRegexp(pat)
			if err == nil && re.MatchString(p) {
				return true
			}
			if lp := literalPrefix(pat); lp == pat && len(p) > len(lp)+1 && p[:len(lp)+1] == lp+"/" {
				return true
			}
		}
		return false
	}

	var out []Violation
	for path, h := range after {
		if b, existed := before[path]; existed && b == h {
			continue
		}
		if covered(path) {
			continue
		}
		var readers []NodeID
		for _, id := range g.IDs() {
			m, _ := g.Node(id)
			if m.NeverFresh() {
				continue
			}
			fs, err := Files(root, m.Inputs)
			if err != nil {
				continue
			}
			for _, f := range fs {
				if f == path {
					readers = append(readers, id)
					break
				}
			}
		}
		if len(readers) == 0 {
			continue // written but read by nobody: cannot affect freshness
		}
		out = append(out, Violation{Path: path, Readers: readers})
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Path < out[j].Path })
	return out, nil
}

func reportViolations(id NodeID, vs []Violation) {
	if len(vs) == 0 {
		return
	}
	fmt.Printf("  ⚠ %s wrote %d file(s) it does not declare in `produces`:\n", id, len(vs))
	for _, v := range vs {
		fmt.Printf("      %s   (declared as an input by: %v)\n", v.Path, v.Readers)
	}
}
