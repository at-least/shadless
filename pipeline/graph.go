package main

// The pipeline graph. One rule governs the whole thing:
//
//	a node needs recomputing only if its own inputs changed, or something
//	upstream of it did.
//
// That is enforced by making a node's key include the keys of its
// dependencies (a merkle chain), so "upstream changed" propagates by
// construction instead of by anyone remembering to declare it. There is no
// second freshness mechanism and no special case for the shadcn pin: the pin
// file is simply an input of the node that reads it, and its key flows
// downstream from there.

import (
	"encoding/json"
	"fmt"
	"os"
)

type Node struct {
	ID       string     `json:"id"`
	Kind     string     `json:"kind"`
	Tier     string     `json:"tier"`
	Needs    []string   `json:"needs"`
	Run      [][]string `json:"run"`
	Inputs   []string   `json:"inputs"` // nil = judges state outside the tree, never fresh
	Produces []string   `json:"produces"`
}

// NeverFresh reports whether the node declares no input set. Such a node
// cannot be skipped and neither can anything downstream of it.
func (n Node) NeverFresh() bool { return n.Inputs == nil }

type Graph struct {
	nodes map[string]Node
	order []string // declaration order, for stable output
}

func LoadGraph(path string) (*Graph, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var list []Node
	if err := json.Unmarshal(b, &list); err != nil {
		return nil, err
	}
	g := &Graph{nodes: make(map[string]Node, len(list))}
	for _, n := range list {
		if _, dup := g.nodes[n.ID]; dup {
			return nil, fmt.Errorf("duplicate node id: %s", n.ID)
		}
		g.nodes[n.ID] = n
		g.order = append(g.order, n.ID)
	}
	for _, n := range list {
		for _, d := range n.Needs {
			if _, ok := g.nodes[d]; !ok {
				return nil, fmt.Errorf("node %s needs unknown node %s", n.ID, d)
			}
		}
	}
	return g, nil
}

func (g *Graph) Node(id string) (Node, bool) { n, ok := g.nodes[id]; return n, ok }
func (g *Graph) IDs() []string               { return g.order }

// Plan returns the transitive closure of targets, topologically sorted.
// Cycles are an authoring error and are reported with the path that closed them.
func (g *Graph) Plan(targets []string) ([]Node, error) {
	seen := map[string]bool{}
	visiting := map[string]bool{}
	var out []Node
	var visit func(id string, path []string) error
	visit = func(id string, path []string) error {
		if seen[id] {
			return nil
		}
		if visiting[id] {
			return fmt.Errorf("cycle in graph: %v", append(path, id))
		}
		n, ok := g.nodes[id]
		if !ok {
			return fmt.Errorf("unknown node id: %s", id)
		}
		visiting[id] = true
		for _, d := range n.Needs {
			if err := visit(d, append(path, id)); err != nil {
				return err
			}
		}
		delete(visiting, id)
		seen[id] = true
		out = append(out, n)
		return nil
	}
	for _, t := range targets {
		if err := visit(t, nil); err != nil {
			return nil, err
		}
	}
	return out, nil
}

var tiers = []string{"fast", "medium", "full"}

func tierRank(t string) int {
	for i, x := range tiers {
		if x == t {
			return i
		}
	}
	return len(tiers) // unknown tier sorts last rather than silently passing as fast
}

// EffectiveTier is the most expensive tier in a node's dependency closure,
// not the tier it declares: a "fast" gate that can only run after a browser
// build is not fast.
func (g *Graph) EffectiveTier(id string) string {
	memo := map[string]int{}
	var walk func(string) int
	walk = func(id string) int {
		if r, ok := memo[id]; ok {
			return r
		}
		n := g.nodes[id]
		worst := tierRank(n.Tier)
		memo[id] = worst // cycle guard; Plan reports real cycles
		for _, d := range n.Needs {
			if r := walk(d); r > worst {
				worst = r
			}
		}
		memo[id] = worst
		return worst
	}
	return tiers[walk(id)]
}

// PlanTier is every gate whose ENTIRE closure fits within tier, plus the
// builds those gates need.
func (g *Graph) PlanTier(tier string) ([]Node, error) {
	max := tierRank(tier)
	var targets []string
	for _, id := range g.order {
		if g.nodes[id].Kind == "gate" && tierRank(g.EffectiveTier(id)) <= max {
			targets = append(targets, id)
		}
	}
	return g.Plan(targets)
}
