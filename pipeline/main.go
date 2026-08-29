package main

// shadless pipeline runner.
//
//   pipeline plan   <tier|node…>   the closure, topologically sorted
//   pipeline status <tier|node…>   fresh / stale, per node
//   pipeline run    <tier|node…>   run the stale ones, record on success
//
// Stamps are written ONLY after a node's commands all exit 0, so a killed or
// failed run leaves the node stale rather than claiming work it did not do.

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"sort"
	"time"
)

const stampFile = "build/pipeline-stamps.json"

type stamps map[string]string // node id -> recorded key

func loadStamps(root string) stamps {
	s := stamps{}
	b, err := os.ReadFile(root + "/" + stampFile)
	if err != nil {
		return s
	}
	_ = json.Unmarshal(b, &s)
	return s
}

func saveStamps(root string, s stamps) error {
	if err := os.MkdirAll(root+"/build", 0o755); err != nil {
		return err
	}
	b, err := json.MarshalIndent(s, "", " ")
	if err != nil {
		return err
	}
	return os.WriteFile(root+"/"+stampFile, append(b, '\n'), 0o644)
}

func resolveTargets(g *Graph, args []string) ([]Node, error) {
	if len(args) == 1 {
		switch args[0] {
		case "fast", "medium", "full":
			return g.PlanTier(args[0])
		}
	}
	return g.Plan(args)
}

func die(err error) {
	if err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		os.Exit(1)
	}
}

func main() {
	if len(os.Args) < 3 {
		fmt.Fprintln(os.Stderr, "usage: pipeline <plan|status|run> <tier|node…>")
		os.Exit(2)
	}
	cmd, args := os.Args[1], os.Args[2:]
	force := false
	if len(args) > 0 && args[0] == "--force" {
		force, args = true, args[1:]
	}
	root, err := os.Getwd()
	die(err)

	g, err := LoadGraph("pipeline/graph.json")
	die(err)
	plan, err := resolveTargets(g, args)
	die(err)

	switch cmd {
	case "plan":
		for _, n := range plan {
			fmt.Println(n.ID)
		}

	case "adopt":
		// Record every node's current key WITHOUT running anything. This
		// asserts that the tree already is what the pipeline would produce —
		// only valid straight after a full green run. It is the migration
		// step onto an already-built tree, and nothing else should use it.
		k := NewKeyer(root, g)
		rec := loadStamps(root)
		n0 := 0
		for _, n := range plan {
			key, skippable, err := k.Key(n.ID)
			die(err)
			if !skippable {
				continue
			}
			rec[n.ID] = key
			n0++
		}
		die(saveStamps(root, rec))
		fmt.Printf("adopted %d nodes as fresh (assumes the tree is a green full run)\n", n0)

	case "status":
		k := NewKeyer(root, g)
		rec := loadStamps(root)
		for _, n := range plan {
			key, skippable, err := k.Key(n.ID)
			die(err)
			switch {
			case !skippable:
				fmt.Printf("%-22s NEVER-FRESH\n", n.ID)
			case rec[n.ID] == key:
				fmt.Printf("%-22s fresh\n", n.ID)
			default:
				fmt.Printf("%-22s STALE\n", n.ID)
			}
		}

	case "run":
		k := NewKeyer(root, g)
		rec := loadStamps(root)
		ran, skipped, violations := 0, 0, 0
		start := time.Now()
		for _, n := range plan {
			key, skippable, err := k.Key(n.ID)
			die(err)
			if skippable && rec[n.ID] == key && !force {
				skipped++
				continue
			}
			fmt.Printf("→ %s\n", n.ID)
			before, err := inputUniverse(root, g)
			die(err)
			for _, argv := range n.Run {
				c := exec.Command(argv[0], argv[1:]...)
				c.Dir, c.Stdout, c.Stderr = root, os.Stdout, os.Stderr
				if err := c.Run(); err != nil {
					// leave the stamp untouched: a failed node stays stale
					delete(rec, n.ID)
					_ = saveStamps(root, rec)
					fmt.Fprintf(os.Stderr, "\n✗ %s failed: %v\n", n.ID, err)
					os.Exit(1)
				}
			}
			ran++
			if after, err := inputUniverse(root, g); err == nil {
				if vs, err := undeclaredWrites(root, g, n, before, after); err == nil {
					reportViolations(n.ID, vs)
					violations += len(vs)
				}
			}
			if skippable {
				// recompute: the node's own run may have changed files that
				// feed its key (a build whose output is also its input)
				k2 := NewKeyer(root, g)
				if after, ok, err := k2.Key(n.ID); err == nil && ok {
					if after != key {
						fmt.Fprintf(os.Stderr, "  note: %s changed its own inputs (key moved during the run)\n", n.ID)
					}
					rec[n.ID] = after
				}
				die(saveStamps(root, rec))
			}
		}
		ids := make([]string, 0, len(rec))
		for id := range rec {
			ids = append(ids, id)
		}
		sort.Strings(ids)
		fmt.Printf("ran %d, skipped %d in %.1fs\n", ran, skipped, time.Since(start).Seconds())
		if violations > 0 {
			fmt.Fprintf(os.Stderr, "\n%d undeclared write(s): a node is driving the graph's freshness "+
				"through a file it does not admit to producing. Fix `produces`, or stop writing there.\n", violations)
			os.Exit(1)
		}
	default:
		fmt.Fprintln(os.Stderr, "unknown command:", cmd)
		os.Exit(2)
	}
}
