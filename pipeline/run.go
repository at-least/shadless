package main

// Parallel execution.
//
// Independent nodes run concurrently; a node is dispatched the moment every
// node it `needs` has finished. Its key is computed at dispatch time, not up
// front, because the key folds in its dependencies' keys and those are only
// final once they have run.
//
// The undeclared-write check (verify.go) compares the input universe before
// and after a node's commands, which only means anything when nothing else is
// writing at the same time. It is therefore enforced at -j1 and skipped above
// it, with a note, rather than silently reporting another node's writes.

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"sync"
	"time"
)

type result struct {
	node       Node
	skipped    bool
	err        error
	output     []byte
	violations []Violation
	elapsed    time.Duration
}

type Runner struct {
	root           string
	graph          *Graph
	jobs           int
	force          bool
	continueOnFail bool

	mu     sync.Mutex
	stamps stamps
}

func (r *Runner) record(id, key string) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.stamps[id] = key
	_ = saveStamps(r.root, r.stamps)
}

func (r *Runner) forget(id string) {
	r.mu.Lock()
	defer r.mu.Unlock()
	delete(r.stamps, id)
	_ = saveStamps(r.root, r.stamps)
}

func (r *Runner) recorded(id string) string {
	r.mu.Lock()
	defer r.mu.Unlock()
	return r.stamps[id]
}

// exec runs one node's commands, capturing output so parallel logs stay
// readable: a node's output is printed as one block when it finishes.
func (r *Runner) exec(n Node) ([]byte, error) {
	var buf bytes.Buffer
	for _, argv := range n.Run {
		c := exec.Command(argv[0], argv[1:]...)
		c.Dir, c.Stdout, c.Stderr = r.root, &buf, &buf
		if err := c.Run(); err != nil {
			return buf.Bytes(), err
		}
	}
	return buf.Bytes(), nil
}

func (r *Runner) runOne(n Node, key string, skippable bool) result {
	start := time.Now()
	var before map[string]string
	if r.jobs == 1 {
		before, _ = inputUniverse(r.root, r.graph)
	}
	out, err := r.exec(n)
	res := result{node: n, output: out, err: err, elapsed: time.Since(start)}
	if err != nil {
		r.forget(n.ID) // a failed node stays stale; it claims nothing
		return res
	}
	if r.jobs == 1 && before != nil {
		if after, e := inputUniverse(r.root, r.graph); e == nil {
			res.violations, _ = undeclaredWrites(r.root, r.graph, n, before, after)
		}
	}
	if skippable {
		// recompute after the run: a node whose own output feeds its key
		// would otherwise be stamped with a key it no longer has
		if after, ok, e := NewKeyer(r.root, r.graph).Key(n.ID); e == nil && ok {
			r.record(n.ID, after)
		} else {
			r.record(n.ID, key)
		}
	}
	return res
}

// Run dispatches the plan, honouring `needs`, and returns the number of nodes
// run, skipped, failed and the undeclared writes seen.
func (r *Runner) Run(plan []Node) (ran, skipped, failed, violations int) {
	inPlan := map[string]bool{}
	for _, n := range plan {
		inPlan[n.ID] = true
	}
	pending := map[string]int{} // id -> unfinished deps still in this plan
	for _, n := range plan {
		for _, d := range n.Needs {
			if inPlan[d] {
				pending[n.ID]++
			}
		}
	}
	done := map[string]bool{}
	dependents := map[string][]string{}
	for _, n := range plan {
		for _, d := range n.Needs {
			if inPlan[d] {
				dependents[d] = append(dependents[d], n.ID)
			}
		}
	}
	byID := map[string]Node{}
	var ready []string
	for _, n := range plan {
		byID[n.ID] = n
		if pending[n.ID] == 0 {
			ready = append(ready, n.ID)
		}
	}

	sem := make(chan struct{}, r.jobs)
	results := make(chan result)
	inflight := 0
	stop := false

	dispatch := func(id string) {
		n := byID[id]
		// the key must be computed here: every dependency has finished, so
		// their stamps are final
		key, skippable, err := NewKeyer(r.root, r.graph).Key(id)
		if err != nil {
			fmt.Fprintln(os.Stderr, "pipeline:", err)
			stop = true
			return
		}
		if skippable && r.recorded(id) == key && !r.force {
			skipped++
			done[id] = true
			for _, d := range dependents[id] {
				pending[d]--
				if pending[d] == 0 {
					ready = append(ready, d)
				}
			}
			return
		}
		inflight++
		go func() {
			sem <- struct{}{}
			defer func() { <-sem }()
			results <- r.runOne(n, key, skippable)
		}()
	}

	for {
		for len(ready) > 0 && !stop {
			id := ready[0]
			ready = ready[1:]
			dispatch(id)
		}
		if inflight == 0 {
			break
		}
		res := <-results
		inflight--
		id := res.node.ID
		status := "✔"
		if res.err != nil {
			status = "✗"
		}
		fmt.Printf("%s %s (%.1fs)\n", status, id, res.elapsed.Seconds())
		if len(res.output) > 0 && (res.err != nil || os.Getenv("PIPELINE_VERBOSE") != "") {
			os.Stdout.Write(res.output)
		}
		reportViolations(id, res.violations)
		violations += len(res.violations)
		if res.err != nil {
			failed++
			fmt.Fprintf(os.Stderr, "  %s failed: %v\n", id, res.err)
			if !r.continueOnFail {
				stop = true
			}
			continue // dependents never become ready: their pending count never reaches 0
		}
		ran++
		done[id] = true
		for _, d := range dependents[id] {
			pending[d]--
			if pending[d] == 0 && !stop {
				ready = append(ready, d)
			}
		}
	}
	if blocked := len(plan) - ran - skipped - failed; blocked > 0 {
		fmt.Fprintf(os.Stderr, "%d node(s) not reached (a dependency failed)\n", blocked)
	}
	return
}
