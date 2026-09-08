package main

// shadless pipeline runner.
//
//   pipeline plan   <tier|node…>   the closure, topologically sorted
//   pipeline list   <tier|node…>   the closure, annotated, no execution
//   pipeline status <tier|node…>   fresh / stale, per node
//   pipeline run    <tier|node…>   run the stale ones, record on success
//
// A target is a tier (fast|medium|full), "builds", "all", or node ids.
// --gates-only / --builds-only filter the resolved plan; --force ignores
// freshness; --keep-going runs past the first red and writes a report.
//
// Stamps are written ONLY after a node's commands all exit 0, so a killed or
// failed run leaves the node stale rather than claiming work it did not do.

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// One file per node under pipeline/stamps/, holding the key that produced the
// current outputs. NOT tracked: a freshness record is an intermediate product,
// and the .dagger/ port makes it a transitional one — Dagger's cache is
// content-addressed and lives in the engine, not in git.
//
// It was tracked, deliberately: the outputs are committed (dist/, docs/,
// generated/ir), so a fresh clone already held them and what it lacked was
// the record of which inputs produced them. Committing that record made a
// clone warm. That trade is off now, and the cost is real — a fresh clone and
// a new CI runner both start cold under this runner.
//
// Still one file per node rather than one map: a single stamps.json changes on
// every build and would conflict on every concurrent branch.
//
// Node ids carry ":" after a fan-out (contracts:dialog); it is legal in a
// POSIX filename but not on Windows, so it is escaped in the path.
const stampDir = "pipeline/stamps"

type stamps map[NodeID]string // node id -> recorded key

func stampFile(id NodeID) string { return strings.ReplaceAll(string(id), ":", "__") }
func stampID(name string) NodeID { return NodeID(strings.ReplaceAll(name, "__", ":")) }
func stampPath(root string, id NodeID) string {
	return filepath.Join(root, stampDir, stampFile(id))
}

func loadStamps(root string) stamps {
	s := stamps{}
	entries, err := os.ReadDir(filepath.Join(root, stampDir))
	if err != nil {
		return s
	}
	for _, e := range entries {
		if e.IsDir() {
			continue
		}
		b, err := os.ReadFile(filepath.Join(root, stampDir, e.Name()))
		if err != nil {
			continue
		}
		s[stampID(e.Name())] = strings.TrimSpace(string(b))
	}
	return s
}

func writeStamp(root string, id NodeID, key string) error {
	if err := os.MkdirAll(filepath.Join(root, stampDir), 0o755); err != nil {
		return err
	}
	return os.WriteFile(stampPath(root, id), []byte(key+"\n"), 0o644)
}

func removeStamp(root string, id NodeID) {
	_ = os.Remove(stampPath(root, id))
}

func resolveTargets(g *Graph, args []string) ([]Node, error) {
	if len(args) == 1 {
		switch args[0] {
		case "fast", "full":
			return g.PlanTier(args[0])
		case "builds":
			return g.PlanBuilds()
		case "all":
			// every node, including build artifacts no gate depends on —
			// PlanTier("full") reaches only what some gate needs
			return g.Plan(g.IDs())
		}
	}
	ids := make([]NodeID, len(args))
	for i, a := range args {
		if _, ok := g.Node(NodeID(a)); !ok {
			return nil, fmt.Errorf("unknown node: %s\nknown: %s", a, strings.Join(idStrings(g.IDs()), ", "))
		}
		ids[i] = NodeID(a)
	}
	return g.Plan(ids)
}

func idStrings(ids []NodeID) []string {
	out := make([]string, len(ids))
	for i, id := range ids {
		out[i] = string(id)
	}
	return out
}

// keepOnly filters a plan to gates or to builds. --gates-only assumes the
// artifacts are already fresh (`make verify`); --builds-only is the mutation
// harness's prelude, which runs the gates itself.
func keepOnly(plan []Node, kind string) []Node {
	var out []Node
	for _, n := range plan {
		if n.Kind == kind {
			out = append(out, n)
		}
	}
	return out
}

func has(args []string, f string) bool {
	for _, a := range args {
		if a == f {
			return true
		}
	}
	return false
}

func die(err error) {
	if err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		os.Exit(1)
	}
}


func main() {
	root, err := os.Getwd()
	die(err)
	g, err := LoadGraphAt(root)
	die(err)
	k := NewKeyer(root, g)
	for _, id := range g.IDs() {
		key, ok, err := k.Key(id)
		die(err)
		if !ok {
			fmt.Printf("%s\t0\t\n", id)
			continue
		}
		n, _ := g.Node(id)
		fmt.Printf("%s\t1\t%s\n", id, stampValue(root, n, key))
	}
}
