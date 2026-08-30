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
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"time"
)

// One file per node under pipeline/stamps/, holding the key that produced the
// current outputs. TRACKED, and one file per node rather than one map for all
// of them, for two reasons:
//
//   - the outputs this pipeline produces are committed (dist/, docs/,
//     src/registry/ir), so a fresh clone already holds them; what it lacks is
//     the record of which inputs produced them. Committing that record is what
//     makes a clone warm.
//   - a single stamps.json changes on every build and would conflict on every
//     concurrent branch. Per node, a conflict happens only when two branches
//     genuinely changed the same node's inputs, and it is one line.
//
// Safe to commit because a stamp is verified, not trusted: the key is a hash
// over the actual contents of every declared input, so it cannot match a tree
// whose sources differ, and `reproducible` (which never goes fresh) is the
// backstop against a committed output that does not match its inputs.
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
		case "fast", "medium", "full":
			return g.PlanTier(args[0])
		case "builds":
			return g.PlanBuilds()
		}
	}
	ids := make([]NodeID, len(args))
	for i, a := range args {
		ids[i] = NodeID(a)
	}
	return g.Plan(ids)
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
	// < 2, not < 3: the single-word subcommands (docs-catalog, oracle-css,
	// product-css, hooks) take no argument. plan/status/run/adopt validate
	// their own target list below.
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: pipeline <plan|status|run|adopt> <fast|medium|full|builds|node…>\n       pipeline pin [--check-only]\n       pipeline tw <in> <out> [--minify] [--cwd DIR]\n       pipeline oracle-css\n       pipeline product-css\n       pipeline docs-catalog\n       pipeline ir-diff <git-ref>|<dirA> <dirB> [--json]\n       pipeline hooks [--uninstall] [--force]\n       pipeline css-direction --update\n\nThe gates are Go tests: go test -C pipeline -count=1 -v [-run '^TestPack$']")
		os.Exit(2)
	}
	cmd, args := os.Args[1], os.Args[2:]
	if len(args) == 0 && (cmd == "plan" || cmd == "status" || cmd == "run" || cmd == "adopt") {
		fmt.Fprintf(os.Stderr, "usage: pipeline %s <fast|medium|full|builds|node…>\n", cmd)
		os.Exit(2)
	}
	if cmd == "ir-diff" {
		os.Exit(runIrDiff(args))
	}
	if cmd == "docs-catalog" {
		os.Exit(runDocsCatalog())
	}
	if cmd == "hooks" {
		os.Exit(runHooks(args))
	}
	if cmd == "product-css" {
		os.Exit(runProductCSS())
	}
	if cmd == "css-direction" {
		if !has(args, "--update") {
			fmt.Fprintln(os.Stderr, "the css-direction GATE is a Go test: go test -C pipeline -run '^TestCssDirection$'\n"+
				"this subcommand only re-records the baseline: pipeline css-direction --update")
			os.Exit(2)
		}
		os.Exit(runCSSDirectionUpdate())
	}
	if cmd == "oracle-css" {
		os.Exit(runOracleCSS())
	}
	if cmd == "tw" {
		os.Exit(runTw(args))
	}
	if cmd == "pin" {
		root, err := os.Getwd()
		die(err)
		os.Exit(runPin(root, has(args, "--check-only"), has(args, "--force")))
	}
	force := false
	if len(args) > 0 && args[0] == "--force" {
		force, args = true, args[1:]
	}
	root, err := os.Getwd()
	die(err)

	g, err := LoadGraph()
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
		n0 := 0
		for _, n := range plan {
			key, skippable, err := k.Key(n.ID)
			die(err)
			if !skippable {
				continue
			}
			die(writeStamp(root, n.ID, key))
			n0++
		}
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
				if present, missing := OutputsPresent(root, n); !present {
					fmt.Printf("%-22s STALE (output missing: %s)\n", n.ID, missing)
					continue
				}
				fmt.Printf("%-22s fresh\n", n.ID)
			default:
				fmt.Printf("%-22s STALE\n", n.ID)
			}
		}

	case "run":
		jobs := runtime.NumCPU()
		if v := os.Getenv("PIPELINE_PARALLEL"); v != "" {
			if n, err := strconv.Atoi(v); err == nil && n > 0 {
				jobs = n
			}
		}
		r := &Runner{
			root: root, graph: g, jobs: jobs, force: force,
			continueOnFail: os.Getenv("PIPELINE_FAILURES") == "continue",
			stamps:         loadStamps(root),
		}
		start := time.Now()
		ran, skipped, failed, violations := r.Run(plan)
		fmt.Printf("ran %d, skipped %d in %.1fs (-j%d)\n", ran, skipped, time.Since(start).Seconds(), jobs)
		if jobs > 1 && ran > 0 {
			fmt.Println("note: the undeclared-write check only runs at -j1 (PIPELINE_PARALLEL=1)")
		}
		if violations > 0 {
			fmt.Fprintf(os.Stderr, "\n%d undeclared write(s): a node is driving the graph's freshness "+
				"through a file it does not admit to producing. Fix `produces`, or stop writing there.\n", violations)
			os.Exit(1)
		}
		if failed > 0 {
			os.Exit(1)
		}

	default:
		fmt.Fprintln(os.Stderr, "unknown command:", cmd)
		os.Exit(2)
	}
}
