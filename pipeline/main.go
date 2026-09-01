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
	"runtime"
	"strconv"
	"strings"
	"time"
)

// One file per node under pipeline/stamps/, holding the key that produced the
// current outputs. NOT tracked: a freshness record is an intermediate product,
// and the .dagger/ port makes it a transitional one — Dagger's cache is
// content-addressed and lives in the engine, not in git.
//
// It was tracked, deliberately: the outputs are committed (dist/, docs/,
// src/registry/ir), so a fresh clone already held them and what it lacked was
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
	// < 2, not < 3: the single-word subcommands (docs-catalog, oracle-css,
	// product-css, build-js) take no argument. plan/status/run/adopt validate
	// their own target list below.
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: pipeline <plan|list|status|run|adopt> <fast|full|builds|all|node…>\n       pipeline pin [--check-only]\n       pipeline tw <in> <out> [--minify] [--cwd DIR]\n       pipeline oracle-css\n       pipeline product-css\n       pipeline docs-catalog\n       pipeline ir-diff <git-ref>|<dirA> <dirB> [--json]\n       pipeline css-direction --update\n       pipeline ledger --record|--render|--dissolve\n       pipeline audit-boundary [--strict|discover]\n       pipeline upstream --to=shadcn@X.Y.Z [--fetch] [--no-build]\n       pipeline build-js\n       pipeline resolve-skins [--fixtures]\n       pipeline inputs <node> [--produces]\n\nThe gates are Go tests: go test -C pipeline -count=1 -v [-run '^TestPack$']")
		os.Exit(2)
	}
	cmd, args := os.Args[1], os.Args[2:]
	if len(args) == 0 && (cmd == "plan" || cmd == "list" || cmd == "status" || cmd == "run" || cmd == "adopt") {
		fmt.Fprintf(os.Stderr, "usage: pipeline %s <fast|full|builds|all|node…>\n", cmd)
		os.Exit(2)
	}
	if cmd == "ir-diff" {
		os.Exit(runIrDiff(args))
	}
	if cmd == "docs-catalog" {
		os.Exit(runDocsCatalog())
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
	if cmd == "ledger" {
		os.Exit(runLedger(args))
	}
	if cmd == "audit-boundary" {
		os.Exit(runAuditBoundary(args))
	}
	if cmd == "upstream" {
		os.Exit(runUpstream(args))
	}
	if cmd == "build-js" {
		os.Exit(runBuildJs())
	}
	if cmd == "resolve-skins" {
		os.Exit(runResolveSkins(args))
	}
	if cmd == "rtl-dict" {
		os.Exit(runRtlDict())
	}
	if cmd == "build-rtl" {
		os.Exit(runBuildRtl())
	}
	if cmd == "docs-consistency" {
		os.Exit(runDocsConsistency())
	}
	if cmd == "demo" {
		os.Exit(runDemo())
	}
	if cmd == "emit" {
		os.Exit(runEmit())
	}
	if cmd == "docs-build" {
		os.Exit(runDocsBuild())
	}
	if cmd == "docs-guides" {
		os.Exit(runDocsGuides())
	}
	if cmd == "docs-fidelity" {
		os.Exit(runDocsFidelity())
	}
	if cmd == "upstream-snapshot" {
		os.Exit(runUpstreamSnapshot(args))
	}
	if cmd == "demo-smoke" {
		os.Exit(runDemoSmoke())
	}
	if cmd == "docs-smoke" {
		os.Exit(runDocsSmoke(has(args, "--all")))
	}
	if cmd == "example-golden" {
		os.Exit(runExampleGolden(args))
	}
	if cmd == "inputs" {
		os.Exit(runInputs(args))
	}
	if cmd == "tw" {
		os.Exit(runTw(args))
	}
	if cmd == "pin" {
		root, err := os.Getwd()
		die(err)
		os.Exit(runPin(root, has(args, "--check-only"), has(args, "--force")))
	}
	force, gatesOnly, buildsOnly, keepGoing := false, false, false, false
	var targets []string
	for _, a := range args {
		switch a {
		case "--force":
			force = true
		case "--gates-only":
			gatesOnly = true
		case "--builds-only":
			buildsOnly = true
		case "--keep-going":
			keepGoing = true
		default:
			targets = append(targets, a)
		}
	}
	if gatesOnly && buildsOnly {
		die(fmt.Errorf("--gates-only and --builds-only are mutually exclusive"))
	}
	root, err := os.Getwd()
	die(err)

	g, err := LoadGraph()
	die(err)
	plan, err := resolveTargets(g, targets)
	die(err)
	if gatesOnly {
		plan = keepOnly(plan, "gate")
	}
	if buildsOnly {
		plan = keepOnly(plan, "build")
	}

	switch cmd {
	case "plan":
		for _, n := range plan {
			fmt.Println(n.ID)
		}

	case "list":
		width := 0
		for _, n := range plan {
			if len(n.ID) > width {
				width = len(n.ID)
			}
		}
		ngates := 0
		for _, n := range plan {
			kind := "build"
			if n.Kind == "gate" {
				kind, ngates = "GATE ", ngates+1
			}
			// the EFFECTIVE tier is what decides whether a tier run picks the
			// node up; show the declared one only when they disagree
			eff := g.EffectiveTier(n.ID)
			self := ""
			if eff != n.Tier {
				self = fmt.Sprintf(" (self %s)", n.Tier)
			}
			needs := ""
			if len(n.Needs) > 0 {
				needs = "  needs: " + strings.Join(idStrings(n.Needs), ", ")
			}
			fmt.Printf("%s %-*s  [%s%s]%s\n", kind, width, n.ID, eff, self, needs)
		}
		fmt.Printf("\n%d nodes (%d gates)\n", len(plan), ngates)

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
			continueOnFail: keepGoing || os.Getenv("PIPELINE_FAILURES") == "continue",
			stamps:         loadStamps(root),
		}
		start := time.Now()
		ran, skipped, failed, violations, badReads := r.Run(plan)
		if keepGoing {
			// the re-pin drill reads this to classify each red gate
			if err := r.writeReport(); err != nil {
				fmt.Fprintln(os.Stderr, "pipeline: writing run report:", err)
			}
		}
		fmt.Printf("ran %d, skipped %d in %.1fs (-j%d)\n", ran, skipped, time.Since(start).Seconds(), jobs)
		if jobs > 1 && ran > 0 {
			fmt.Println("note: the undeclared-write check only runs at -j1 (PIPELINE_PARALLEL=1)")
		}
		if violations > 0 {
			fmt.Fprintf(os.Stderr, "\n%d undeclared write(s): a node is driving the graph's freshness "+
				"through a file it does not admit to producing. Fix `produces`, or stop writing there.\n", violations)
		}
		if badReads > 0 {
			fmt.Fprintf(os.Stderr, "\n%d undeclared file access(es): a node opened a file it declares in "+
				"neither `inputs` nor `produces`. If it reads the file, it is not in the node's key and a "+
				"change to it leaves the node falsely fresh — add it to `inputs`, and if the file is another "+
				"node's output add that node to `needs` too. If it writes the file, add it to `produces`.\n", badReads)
		}
		if violations > 0 || badReads > 0 {
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
