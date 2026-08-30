package main

// meta — the gate that tests the gates. Ported from gates/meta.mjs.
//
// The most expensive recurring bug class in this repo's history is not a
// broken component, it is a gate that PASSES WHEN IT SHOULD FAIL:
//
//	ed1bef4  install-import check matched only raw quotes; highlighted fences
//	         carry &quot; — the gate passed vacuously over every page
//	900555a  regex LITERALS don't interpolate ${comp} — shipped id-less dead
//	         pages, caught only because that tool self-tests
//	e3c2b12  sync driftScan walked the wrong path — dead code, always
//	         reported "no drift"
//	244e20c  --scope=x was silently ignored, so scoped runs checked nothing
//
// Each was found by a HAND-RUN negative test, mentioned in a commit message
// and then lost. This makes those permanent: every gate in the graph must
// declare at least one mutation, and every mutation must actually make its
// gate exit non-zero.
//
// The split between the two tests in meta_test.go is deliberate and is an
// improvement on the JS version. The WIRING checks (every gate has a `why`
// and a mutation, every mutation names a gate that lists it) are pure and
// instant, so they run in the ordinary `go test` suite — under gates/meta.mjs
// they only ran when someone remembered to invoke meta, which is the same
// "nobody ran the check" failure mode one level up. Only the part that has to
// execute real gates is opt-in.

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"sync"
)

const upstreamExamples = ".upstream/shadcn-ui/apps/v4/examples/radix"

// resolveOwnedExample returns the first page in the ownership manifest that
// still has an upstream example source, so a re-pin that retires one file
// does not silently disarm the mutation that targets it.
func resolveOwnedExample(root string) ([]string, error) {
	b, err := os.ReadFile(filepath.Join(root, "docs/example-oracle.json"))
	if err != nil {
		return nil, fmt.Errorf("docs/example-oracle.json missing (build first): %w", err)
	}
	var owned []struct {
		Name string `json:"name"`
	}
	if err := json.Unmarshal(b, &owned); err != nil {
		return nil, fmt.Errorf("docs/example-oracle.json: %w", err)
	}
	for _, o := range owned {
		rel := filepath.Join(upstreamExamples, o.Name+".tsx")
		if _, err := os.Stat(filepath.Join(root, rel)); err == nil {
			return []string{rel}, nil
		}
	}
	return nil, fmt.Errorf("no page in docs/example-oracle.json has an upstream example under %s", upstreamExamples)
}

// ------------------------------------------------------------- wiring

// metaWiring cross-checks the graph against the mutation set in both
// directions and returns every problem found, so one run reports all of them
// rather than the first.
func metaWiring(g *Graph, muts []Mutation) []string {
	byID := map[string]Mutation{}
	for _, m := range muts {
		if _, dup := byID[m.ID]; dup {
			// a duplicate id would shadow one definition and silently halve
			// the proven set
			continue
		}
		byID[m.ID] = m
	}
	var problems []string
	for _, id := range g.IDs() {
		n, _ := g.Node(id)
		if n.Kind != "gate" {
			continue
		}
		if len(n.Mutations) == 0 {
			problems = append(problems, fmt.Sprintf("gate %s: declares no mutation — unproven", id))
		}
		for _, mid := range n.Mutations {
			if _, ok := byID[mid]; !ok {
				problems = append(problems, fmt.Sprintf(
					"gate %s: declares mutation %q but no such mutation is defined in mutations.go", id, mid))
			}
		}
		if n.Why == "" {
			problems = append(problems, fmt.Sprintf(
				"gate %s: no Why — a gate nobody can explain cannot be reviewed", id))
		}
	}
	seen := map[string]bool{}
	for _, m := range muts {
		if seen[m.ID] {
			problems = append(problems, fmt.Sprintf("mutation %s: defined twice", m.ID))
		}
		seen[m.ID] = true
		n, ok := g.Node(m.Gate)
		if !ok {
			problems = append(problems, fmt.Sprintf("mutation %s: targets unknown gate %q", m.ID, m.Gate))
		} else if !contains(n.Mutations, m.ID) {
			problems = append(problems, fmt.Sprintf(
				"mutation %s: gate %q does not list it in nodes.go", m.ID, m.Gate))
		}
		if m.Why == "" {
			problems = append(problems, fmt.Sprintf(
				"mutation %s: no Why — say which real bug class this proves", m.ID))
		}
		if (m.Files == nil) == (m.Resolve == nil) {
			problems = append(problems, fmt.Sprintf(
				"mutation %s: set exactly one of Files or Resolve", m.ID))
		}
		if m.Apply == nil {
			problems = append(problems, fmt.Sprintf("mutation %s: no Apply", m.ID))
		}
	}
	sort.Strings(problems)
	return problems
}

// ungatedBuilds lists build nodes with no gate anywhere downstream: artifacts
// that ship without anything asserting they are correct. Reported, not fatal.
func ungatedBuilds(g *Graph) []string {
	gated := map[NodeID]bool{}
	for _, id := range g.IDs() {
		n, _ := g.Node(id)
		if n.Kind != "gate" {
			continue
		}
		closure, err := g.Plan([]NodeID{id})
		if err != nil {
			continue
		}
		for _, d := range closure {
			gated[d.ID] = true
		}
	}
	var out []string
	for _, id := range g.IDs() {
		if n, _ := g.Node(id); n.Kind == "build" && !gated[id] {
			out = append(out, string(id))
		}
	}
	return out
}

// ------------------------------------------------------------- harness

// snapshot is the saved state of the files a mutation is about to touch.
//
// It records the REQUESTED paths, not just the ones that existed: restore has
// to handle both directions. coverage-drop-contract deletes its target, so
// restore recreates it; a mutation that creates a file must have it removed
// again rather than leave an untracked artifact behind.
type snapshot struct {
	root    string
	request []string          // every path asked for, in order
	content map[string][]byte // only those that existed
}

func takeSnapshot(root string, paths []string) (*snapshot, error) {
	s := &snapshot{root: root, request: append([]string(nil), paths...), content: map[string][]byte{}}
	for _, p := range paths {
		b, err := os.ReadFile(filepath.Join(root, p))
		if err == nil {
			s.content[p] = b
		} else if !os.IsNotExist(err) {
			return nil, fmt.Errorf("snapshotting %s: %w", p, err)
		}
	}
	return s, nil
}

// restore puts the tree back. It reports errors rather than swallowing them:
// a failed restore leaves a mutated file in the working tree, which is far
// worse than a failed mutation.
func (s *snapshot) restore() error {
	var errs []string
	for _, p := range s.request {
		full := filepath.Join(s.root, p)
		if b, existed := s.content[p]; existed {
			if err := os.WriteFile(full, b, 0o644); err != nil {
				errs = append(errs, fmt.Sprintf("%s: %v", p, err))
			}
			continue
		}
		if err := os.Remove(full); err != nil && !os.IsNotExist(err) {
			errs = append(errs, fmt.Sprintf("%s: %v", p, err))
		}
	}
	if len(errs) > 0 {
		return fmt.Errorf("RESTORE FAILED (the working tree is still mutated):\n  %s",
			strings.Join(errs, "\n  "))
	}
	return nil
}

// runGate executes a gate's commands in order and reports whether it went
// red. Output is captured: a caught mutation is expected to print a failure
// report, and that report is noise unless the mutation was NOT caught.
func runGate(root string, n Node) (red bool, output string) {
	var buf strings.Builder
	for _, argv := range n.Run {
		c := exec.Command(argv[0], argv[1:]...)
		c.Dir = root
		c.Stdout, c.Stderr = &buf, &buf
		if err := c.Run(); err != nil {
			return true, buf.String()
		}
	}
	return false, buf.String()
}

// activeRestore is the restore hook for the mutation currently applied, so an
// interrupt between apply and restore can still put the tree back. Exactly one
// mutation is live at a time (the harness is sequential by design: two
// mutations at once would each see the other's break).
var activeRestore struct {
	mu sync.Mutex
	fn func() error
}

func setActiveRestore(fn func() error) {
	activeRestore.mu.Lock()
	defer activeRestore.mu.Unlock()
	activeRestore.fn = fn
}

// RestoreActiveMutation undoes whatever mutation is applied right now, if any.
// Safe to call from a signal handler and safe to call twice.
func RestoreActiveMutation() error {
	activeRestore.mu.Lock()
	fn := activeRestore.fn
	activeRestore.fn = nil
	activeRestore.mu.Unlock()
	if fn == nil {
		return nil
	}
	return fn()
}

// mutationResult is one row of the meta report.
type mutationResult struct {
	ID     string
	Gate   NodeID
	Caught bool
	Note   string // why it was not caught, when that is known
}

// runMutation applies one mutation, runs its gate, and restores the tree —
// including when the mutation itself errors. The restore is the important
// part: it must happen on every path out of this function.
func runMutation(root string, g *Graph, m Mutation) (res mutationResult, restoreErr error) {
	res = mutationResult{ID: m.ID, Gate: m.Gate}
	n, ok := g.Node(m.Gate)
	if !ok {
		res.Note = fmt.Sprintf("unknown gate %q", m.Gate)
		return res, nil
	}
	// Resolved before the snapshot on purpose: a mutation that locates its
	// target inside a build artifact must fail loudly on an unbuilt tree
	// rather than snapshot nothing and then "apply" to a file that is not
	// there.
	files, err := m.targets(root)
	if err != nil {
		res.Note = fmt.Sprintf("could not resolve target: %v", err)
		return res, nil
	}
	snap, err := takeSnapshot(root, files)
	if err != nil {
		res.Note = fmt.Sprintf("could not snapshot: %v", err)
		return res, nil
	}
	setActiveRestore(snap.restore)
	defer func() { restoreErr = RestoreActiveMutation() }()

	if err := m.Apply(root, files); err != nil {
		res.Note = fmt.Sprintf("mutation itself errored: %s", firstLine(err.Error()))
		return res, nil
	}
	res.Caught, _ = runGate(root, n)
	return res, nil
}

func firstLine(s string) string {
	if i := strings.IndexByte(s, '\n'); i >= 0 {
		return s[:i]
	}
	return s
}

// selectMutations narrows the set the way gates/meta.mjs did: by explicit id
// list, or by the effective tier of each mutation's gate.
func selectMutations(g *Graph, muts []Mutation, only, tier string) ([]Mutation, error) {
	var out []Mutation
	switch {
	case only != "":
		want := map[string]bool{}
		for _, id := range strings.Split(only, ",") {
			if id = strings.TrimSpace(id); id != "" {
				want[id] = true
			}
		}
		for _, m := range muts {
			if want[m.ID] {
				out = append(out, m)
				delete(want, m.ID)
			}
		}
		if len(want) > 0 {
			var missing []string
			for id := range want {
				missing = append(missing, id)
			}
			sort.Strings(missing)
			return nil, fmt.Errorf("unknown mutation(s): %s", strings.Join(missing, ", "))
		}
	case tier != "":
		max := tierRank(tier)
		if max >= len(tiers) {
			return nil, fmt.Errorf("unknown tier: %s (fast|medium|full)", tier)
		}
		for _, m := range muts {
			if _, ok := g.Node(m.Gate); !ok {
				continue
			}
			if tierRank(g.EffectiveTier(m.Gate)) <= max {
				out = append(out, m)
			}
		}
	default:
		out = append(out, muts...)
	}
	if len(out) == 0 {
		return nil, fmt.Errorf("no mutations selected")
	}
	return out, nil
}
