package main

// Fan-out: where the runner's view of the graph is FINER than the registry's.
//
// A registry node whose work is really N independent jobs becomes N nodes,
// each with its own key and its own worker slot. Only legal when the jobs are
// genuinely independent — separate outputs, no shared mutable state, each
// failing on its own.
//
// Done here rather than in the generator so the split stays live: adding a
// contract def adds a node without regenerating anything.

import (
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// fanout maps a node id to a splitter. Returning nil leaves the node alone.
// The splitter takes the repo root explicitly: the runner happens to run with
// the root as its cwd, but `go test -C pipeline` does not, and a graph that
// silently comes back without its fan-out parts is the kind of difference
// nothing downstream would notice.
var fanout = map[NodeID]func(root string, n Node) ([]Node, error){
	NContracts: fanContracts,
}

// contracts runs 29 components, already one child process each, serially:
// 46% of the full tier spent on jobs that never touch one another.
func fanContracts(root string, n Node) ([]Node, error) {
	entries, err := os.ReadDir(filepath.Join(root, "tools/contracts/components"))
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		if strings.HasSuffix(e.Name(), ".mjs") {
			names = append(names, strings.TrimSuffix(e.Name(), ".mjs"))
		}
	}
	sort.Strings(names)

	// the two broad patterns cover every def, which is what we are narrowing;
	// the harness files they also matched are named explicitly
	var shared []string
	for _, p := range n.Inputs {
		if p != "tools/contracts/**/*.mjs" && p != "tools/contracts/components/**" {
			shared = append(shared, p)
		}
	}

	out := make([]Node, 0, len(names))
	for _, name := range names {
		c := n
		c.ID = NodeID("contracts:" + name)
		c.Run = [][]string{{"./build/pipeline", "contract", name}}
		c.Inputs = append([]string{
			"tools/contracts/components/" + name + ".mjs",
		}, shared...)
		c.Produces = []string{"tools/contracts/out/" + name}
		out = append(out, c)
	}
	return out, nil
}

// expandFanout rewrites the node list, rewiring every `needs` that pointed at
// a split node to point at all of its parts.
func expandFanout(root string, in []Node) ([]Node, error) {
	replaced := map[NodeID][]NodeID{}
	var out []Node
	for _, n := range in {
		split := fanout[n.ID]
		if split == nil {
			out = append(out, n)
			continue
		}
		parts, err := split(root, n)
		if err != nil {
			return nil, err
		}
		ids := make([]NodeID, 0, len(parts))
		for _, p := range parts {
			ids = append(ids, p.ID)
		}
		replaced[n.ID] = ids
		out = append(out, parts...)
	}
	if len(replaced) == 0 {
		return out, nil
	}
	for i := range out {
		var needs []NodeID
		for _, d := range out[i].Needs {
			if parts, ok := replaced[d]; ok {
				needs = append(needs, parts...)
			} else {
				needs = append(needs, d)
			}
		}
		out[i].Needs = needs
	}
	return out, nil
}
