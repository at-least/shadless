package main

// Graph invariants. Named TestUnit* so they run as part of the `unit` gate:
// they are pure functions over the declared graph and cost microseconds.
//
// These used to be unwritable. The graph lived in gates/registry.mjs and Go
// only saw a generated copy, so the only thing anyone could check was that
// the copy matched — never that the graph itself was coherent.

import (
	"testing"
)

func TestUnitGraphLoads(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatalf("the declared graph does not load: %v", err)
	}
	if len(g.IDs()) == 0 {
		t.Fatal("empty graph")
	}
	seen := map[NodeID]bool{}
	for _, id := range g.IDs() {
		if seen[id] {
			t.Errorf("duplicate node id %s", id)
		}
		seen[id] = true
		n, _ := g.Node(id)
		switch n.Kind {
		case "gate", "build":
		default:
			t.Errorf("%s: kind %q is neither gate nor build", id, n.Kind)
		}
		if tierRank(n.Tier) >= len(tiers) {
			t.Errorf("%s: unknown tier %q", id, n.Tier)
		}
		if len(n.Run) == 0 {
			t.Errorf("%s: no Run commands — the node does nothing", id)
		}
		for _, argv := range n.Run {
			if len(argv) == 0 {
				t.Errorf("%s: empty argv in Run", id)
			}
		}
		// A build node that declares nothing it produces cannot have its
		// outputs checked for presence, and the undeclared-write check has
		// nothing to compare against.
		if n.Kind == "build" && len(n.Produces) == 0 {
			t.Errorf("%s: a build node must declare Produces", id)
		}
		// A gate MAY declare Produces — `contracts` writes a review surface
		// (oracle.html, result.json) alongside its verdict, and the freshness
		// check needs to know about it. What a gate may not do is be the only
		// producer of something the product ships.
	}
}

// Plan must emit every dependency before the node that needs it — that
// ordering IS the runner's contract.
func TestUnitGraphPlanIsTopological(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	plan, err := g.Plan(g.IDs())
	if err != nil {
		t.Fatal(err)
	}
	if len(plan) != len(g.IDs()) {
		t.Fatalf("planning every id returned %d nodes, want %d", len(plan), len(g.IDs()))
	}
	pos := map[NodeID]int{}
	for i, n := range plan {
		pos[n.ID] = i
	}
	for i, n := range plan {
		for _, d := range n.Needs {
			if pos[d] >= i {
				t.Errorf("%s (at %d) is planned before its dependency %s (at %d)", n.ID, i, d, pos[d])
			}
		}
	}
}

// A targeted run must build the transitive closure and nothing else — that is
// what makes `pipeline run <id>` cheap and what stops it being wrong.
func TestUnitGraphPlanIsClosed(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	for _, id := range g.IDs() {
		plan, err := g.Plan([]NodeID{id})
		if err != nil {
			t.Fatalf("%s: %v", id, err)
		}
		in := map[NodeID]bool{}
		for _, n := range plan {
			in[n.ID] = true
		}
		if !in[id] {
			t.Errorf("%s: its own plan does not contain it", id)
		}
		for _, n := range plan {
			for _, d := range n.Needs {
				if !in[d] {
					t.Errorf("plan for %s includes %s but not its dependency %s", id, n.ID, d)
				}
			}
		}
	}
}

// EffectiveTier is the whole reason --tier=fast does not drag a browser in.
func TestUnitGraphEffectiveTierDominatesDeps(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	for _, id := range g.IDs() {
		n, _ := g.Node(id)
		eff := tierRank(g.EffectiveTier(id))
		if eff < tierRank(n.Tier) {
			t.Errorf("%s: effective tier %s is cheaper than its declared tier %s",
				id, g.EffectiveTier(id), n.Tier)
		}
		for _, d := range n.Needs {
			if de := tierRank(g.EffectiveTier(d)); eff < de {
				t.Errorf("%s: effective tier %s is cheaper than dependency %s's %s",
					id, g.EffectiveTier(id), d, g.EffectiveTier(d))
			}
		}
	}
}

// Cheaper tiers must be subsets of dearer ones, or "fast" would be running
// something "full" does not.
func TestUnitGraphTiersNest(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	ids := func(tier string) map[NodeID]bool {
		plan, err := g.PlanTier(tier)
		if err != nil {
			t.Fatalf("%s: %v", tier, err)
		}
		out := map[NodeID]bool{}
		for _, n := range plan {
			out[n.ID] = true
		}
		return out
	}
	fast, medium, full := ids("fast"), ids("medium"), ids("full")
	for _, pair := range []struct {
		small, large map[NodeID]bool
		names        string
	}{{fast, medium, "fast ⊄ medium"}, {medium, full, "medium ⊄ full"}} {
		for id := range pair.small {
			if !pair.large[id] {
				t.Errorf("%s: %s is planned in the cheaper tier but not the dearer one", pair.names, id)
			}
		}
	}
	if len(fast) == 0 {
		t.Error("the fast tier plans nothing")
	}
}

// Every gate must be reachable from some tier, or it never runs at all.
func TestUnitGraphEveryGateRunsInFull(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	plan, err := g.PlanTier("full")
	if err != nil {
		t.Fatal(err)
	}
	in := map[NodeID]bool{}
	for _, n := range plan {
		in[n.ID] = true
	}
	for _, id := range g.IDs() {
		if n, _ := g.Node(id); n.Kind == "gate" && !in[id] {
			t.Errorf("gate %s is in no tier — it never runs", id)
		}
	}
}

func TestUnitResolveTargets(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	// "all" must reach every node, including build artifacts no gate needs —
	// that is exactly how it differs from the full tier.
	all, err := resolveTargets(g, []string{"all"})
	if err != nil {
		t.Fatal(err)
	}
	if len(all) != len(g.IDs()) {
		t.Errorf(`resolveTargets("all") returned %d nodes, want every one of %d`, len(all), len(g.IDs()))
	}
	full, err := resolveTargets(g, []string{"full"})
	if err != nil {
		t.Fatal(err)
	}
	if len(full) > len(all) {
		t.Errorf("the full tier (%d) plans more than all (%d)", len(full), len(all))
	}
	if _, err := resolveTargets(g, []string{"no-such-node"}); err == nil {
		t.Error("an unknown node id resolved without error")
	}
	// a bare tier word is only a tier when it is the sole argument; as one of
	// several it would be a node id, and there is no node called "fast"
	if _, err := resolveTargets(g, []string{"fast", "pin"}); err == nil {
		t.Error(`resolveTargets("fast","pin") should fail: "fast" is not a node id`)
	}
}

func TestUnitKeepOnly(t *testing.T) {
	plan := []Node{
		{ID: "a", Kind: "build"},
		{ID: "b", Kind: "gate"},
		{ID: "c", Kind: "build"},
	}
	if got := keepOnly(plan, "gate"); len(got) != 1 || got[0].ID != "b" {
		t.Errorf("keepOnly(gate) = %v, want [b]", got)
	}
	if got := keepOnly(plan, "build"); len(got) != 2 {
		t.Errorf("keepOnly(build) = %v, want 2 nodes", got)
	}
	if got := keepOnly(nil, "gate"); got != nil {
		t.Errorf("keepOnly(nil) = %v, want nil", got)
	}
}

// A cycle is an authoring error and must be reported with the path, not hang.
func TestUnitGraphReportsCycles(t *testing.T) {
	g, err := newGraph([]Node{
		{ID: "a", Kind: "build", Tier: "fast", Needs: []NodeID{"b"}},
		{ID: "b", Kind: "build", Tier: "fast", Needs: []NodeID{"a"}},
	})
	if err != nil {
		t.Fatal(err)
	}
	if _, err := g.Plan([]NodeID{"a"}); err == nil {
		t.Fatal("a cycle planned without error")
	}
}

func TestUnitGraphRejectsUnknownDependency(t *testing.T) {
	_, err := newGraph([]Node{
		{ID: "a", Kind: "build", Tier: "fast", Needs: []NodeID{"nope"}},
	})
	if err == nil {
		t.Fatal("a node needing an undeclared id loaded without error")
	}
}
