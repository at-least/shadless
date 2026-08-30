package main

// The meta-gate as tests. See meta.go for why the wiring half always runs and
// the executing half is opt-in.
//
//	go test -run TestMetaWiring        instant, pure — part of the normal suite
//	SHADLESS_META=1 go test -run TestMeta$ -v -timeout 2h
//	SHADLESS_META=1 META_TIER=fast go test -run TestMeta$ -v
//	SHADLESS_META=1 META_ONLY=pin-commit-drift go test -run TestMeta$ -v

import (
	"os"
	"os/signal"
	"strings"
	"syscall"
	"testing"
)

// TestMetaWiring is the coverage half: every gate is proven by at least one
// mutation that exists, every mutation targets a gate that lists it, and both
// carry a Why. Pure — it reads the graph and the mutation set and executes
// nothing, so it belongs in the ordinary suite.
func TestMetaWiring(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	if problems := metaWiring(g, Mutations); len(problems) > 0 {
		t.Fatalf("FAIL  meta (graph/mutation wiring)\n  %s", strings.Join(problems, "\n  "))
	}
	gates := 0
	for _, id := range g.IDs() {
		if n, _ := g.Node(id); n.Kind == "gate" {
			gates++
		}
	}
	t.Logf("PASS  meta-wiring (%d gates, %d mutations, every gate proven)", gates, len(Mutations))
	if u := ungatedBuilds(g); len(u) > 0 {
		// reported, not fatal: an artifact with no gate downstream is worth
		// knowing about but is not by itself a defect
		t.Logf("note: build artifacts with no gate downstream: %s", strings.Join(u, ", "))
	}
}

// TestMeta is the executing half: apply each mutation to the real tree, run
// its gate, and require the gate to go red. It needs a freshly built tree and
// runs real browsers, so it is opt-in via SHADLESS_META.
func TestMeta(t *testing.T) {
	if os.Getenv("SHADLESS_META") == "" {
		t.Skip("set SHADLESS_META=1 to run mutation testing (needs a built tree; `make meta`)")
	}
	root := repoRoot(t)
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	if problems := metaWiring(g, Mutations); len(problems) > 0 {
		t.Fatalf("FAIL  meta (graph/mutation wiring)\n  %s", strings.Join(problems, "\n  "))
	}
	selected, err := selectMutations(g, Mutations, os.Getenv("META_ONLY"), os.Getenv("META_TIER"))
	if err != nil {
		t.Fatal(err)
	}

	// Mutations edit real files. A Ctrl-C between apply and restore would
	// leave the working tree broken, so the interrupt is caught, the restore
	// that the running mutation registered is run, and only then do we exit.
	sig := make(chan os.Signal, 1)
	signal.Notify(sig, os.Interrupt, syscall.SIGTERM)
	defer signal.Stop(sig)
	go func() {
		<-sig
		if err := RestoreActiveMutation(); err != nil {
			os.Stderr.WriteString(err.Error() + "\n")
		}
		os.Exit(130)
	}()
	// and if the run panics or the test binary exits early, still put it back
	defer func() { _ = RestoreActiveMutation() }()

	t.Logf("meta: %d mutations", len(selected))
	var failures []string
	for _, m := range selected {
		res, restoreErr := runMutation(root, g, m)
		if restoreErr != nil {
			// a tree left mutated is worse than any missed mutation: stop
			t.Fatalf("%s: %v", m.ID, restoreErr)
		}
		if res.Caught {
			t.Logf("  %-34s -> %-20s CAUGHT", m.ID, m.Gate)
			continue
		}
		note := ""
		if res.Note != "" {
			note = " (" + res.Note + ")"
		}
		t.Logf("  %-34s -> %-20s NOT CAUGHT%s", m.ID, m.Gate, note)
		failures = append(failures, m.ID+": "+string(m.Gate)+" stayed green under \""+m.Why+"\""+note)
	}
	if len(failures) > 0 {
		t.Fatalf("FAIL  meta (%d/%d mutations not caught)\n  %s\n\n"+
			"  A gate that cannot fail is not a gate. Fix the gate, not the mutation.",
			len(failures), len(selected), strings.Join(failures, "\n  "))
	}
	t.Logf("PASS  meta (%d mutations, every one caught by its gate)", len(selected))
}
