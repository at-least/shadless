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
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"time"
)

type result struct {
	node       Node
	skipped    bool
	err        error
	output     []byte
	violations []Violation
	reads      []string // files read but not declared in `inputs`
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

	// report is what --keep-going writes to build/gates/run-report.json.
	// The re-pin drill consumes it to classify each red gate as EXPECTED
	// (its components moved upstream) or UNEXPECTED (we regressed), so the
	// failure TAIL is part of the interface, not a log.
	report runReport
}

// runReport mirrors the shape gates/run.mjs wrote, because gates/upstream.mjs
// reads it as data.
type runReport struct {
	Failed  map[NodeID]failedNode `json:"failed"`
	Blocked []NodeID              `json:"blocked"`
	Passed  []NodeID              `json:"passed"`
}

type failedNode struct {
	Cmd  string `json:"cmd"`
	Tail string `json:"tail"`
}

// tailOf is the last 25 non-empty lines — enough for the drill to attribute a
// failure to a component, short enough to paste into a report.
func tailOf(out []byte) string {
	var lines []string
	for _, l := range strings.Split(string(out), "\n") {
		if strings.TrimSpace(l) != "" {
			lines = append(lines, l)
		}
	}
	if len(lines) > 25 {
		lines = lines[len(lines)-25:]
	}
	return strings.Join(lines, "\n")
}

// writeReport persists the run report. Only --keep-going asks for it: a run
// that stops at the first red has nothing to classify.
func (r *Runner) writeReport() error {
	if r.report.Failed == nil {
		r.report.Failed = map[NodeID]failedNode{}
	}
	if r.report.Blocked == nil {
		r.report.Blocked = []NodeID{}
	}
	if r.report.Passed == nil {
		r.report.Passed = []NodeID{}
	}
	dir := filepath.Join(r.root, "build", "gates")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	b, err := json.MarshalIndent(r.report, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(dir, "run-report.json"), append(b, '\n'), 0o644)
}

func (r *Runner) record(id NodeID, key string) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.stamps[id] = key
	_ = writeStamp(r.root, id, key) // one file per node: parallel nodes do not contend
}

func (r *Runner) forget(id NodeID) {
	r.mu.Lock()
	defer r.mu.Unlock()
	delete(r.stamps, id)
	removeStamp(r.root, id)
}

func (r *Runner) recorded(id NodeID) string {
	r.mu.Lock()
	defer r.mu.Unlock()
	return r.stamps[id]
}

// isGoTest reports whether a command is a `go test` invocation, which is the
// only kind that can be asked for a testlog.
func isGoTest(argv []string) bool {
	return len(argv) >= 2 && argv[0] == "go" && argv[1] == "test"
}

// fsRecorder is the JS half of the same evidence. NODE_OPTIONS is inherited by
// child processes, so one variable covers a tool and everything it spawns —
// including the tailwind CLI, which is itself a node script.
const fsRecorder = "tools/fs-record.mjs"

// exec runs one node's commands, capturing output so parallel logs stay
// readable: a node's output is printed as one block when it finishes.
//
// Every `go test` command additionally gets -test.testlogfile, which makes the
// test binary record the files it opens. That is the evidence for the
// undeclared-read check and it costs nothing: the gate was going to run
// anyway. The path must be ABSOLUTE — these commands carry `-C pipeline`, and
// a relative one would land inside the package directory.
func (r *Runner) exec(n Node) ([]byte, []string, error) {
	var buf bytes.Buffer
	var logs []string
	// only create the scratch dir when there is a go test command to log:
	// a JS node would otherwise leak an empty temp dir on every run, since
	// readsFrom only cleans up when it has a log to clean up after.
	dir, err := os.MkdirTemp("", "pipeline-access-")
	if err != nil {
		dir = "" // no scratch dir: run without the check rather than fail the node
	}
	// the JS recorder appends to one log for the whole node, children included
	jsLog := ""
	if dir != "" {
		jsLog = filepath.Join(dir, "js.log")
	}
	for i, argv := range n.Run {
		cmd := argv
		if dir != "" && isGoTest(argv) {
			log := filepath.Join(dir, fmt.Sprintf("%s-%d.log", stampFile(n.ID), i))
			cmd = append(append([]string{}, argv...), "-test.testlogfile="+log)
			logs = append(logs, log)
		}
		c := exec.Command(cmd[0], cmd[1:]...)
		c.Dir, c.Stdout, c.Stderr = r.root, &buf, &buf
		if jsLog != "" {
			c.Env = append(os.Environ(),
				"SHADLESS_FSLOG="+jsLog,
				"NODE_OPTIONS="+strings.TrimSpace(os.Getenv("NODE_OPTIONS")+
					" --import "+filepath.Join(r.root, fsRecorder)))
		}
		if err := c.Run(); err != nil {
			return buf.Bytes(), append(logs, jsLog), err
		}
	}
	return buf.Bytes(), append(logs, jsLog), nil
}

// readsFrom collects the undeclared reads across a node's testlogs, then
// removes the temp directory holding them.
func (r *Runner) readsFrom(n Node, logs []string) []string {
	if len(logs) == 0 {
		return nil
	}
	defer os.RemoveAll(filepath.Dir(logs[0]))
	seen := map[string]bool{}
	var all []string
	for _, log := range logs {
		if log == "" {
			continue
		}
		var opens []string
		var err error
		if strings.HasSuffix(log, "js.log") {
			opens, err = fsRecordOpens(r.root, log)
		} else {
			opens, err = testlogOpens(r.root, log)
		}
		if err != nil {
			continue // a missing log means no evidence, not a violation
		}
		for _, p := range opens {
			if !seen[p] {
				seen[p] = true
				all = append(all, p)
			}
		}
	}
	undeclared, err := undeclaredReads(r.root, r.graph, n, all)
	if err != nil {
		return nil
	}
	return undeclared
}

func (r *Runner) runOne(n Node, key string, skippable bool) result {
	start := time.Now()
	var before map[string]string
	if r.jobs == 1 {
		before, _ = inputUniverse(r.root, r.graph)
	}
	out, logs, err := r.exec(n)
	res := result{node: n, output: out, err: err, elapsed: time.Since(start)}
	// computed even for a failed node: a gate that went red still read what it
	// read, and the declaration is wrong either way
	res.reads = r.readsFrom(n, logs)
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
			r.record(n.ID, stampValue(r.root, n, after))
		} else {
			r.record(n.ID, stampValue(r.root, n, key))
		}
	}
	return res
}

// Run dispatches the plan, honouring `needs`, and returns the number of nodes
// run, skipped and failed, plus the two declaration violations seen: writes
// outside `produces` and reads outside `inputs`. They are counted separately
// because they are different bugs with different fixes.
func (r *Runner) Run(plan []Node) (ran, skipped, failed, violations, badReads int) {
	inPlan := map[NodeID]bool{}
	for _, n := range plan {
		inPlan[n.ID] = true
	}
	pending := map[NodeID]int{} // id -> unfinished deps still in this plan
	for _, n := range plan {
		for _, d := range n.Needs {
			if inPlan[d] {
				pending[n.ID]++
			}
		}
	}
	done := map[NodeID]bool{}
	dependents := map[NodeID][]NodeID{}
	for _, n := range plan {
		for _, d := range n.Needs {
			if inPlan[d] {
				dependents[d] = append(dependents[d], n.ID)
			}
		}
	}
	byID := map[NodeID]Node{}
	var ready []NodeID
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

	dispatch := func(id NodeID) {
		n := byID[id]
		// the key must be computed here: every dependency has finished, so
		// their stamps are final
		key, skippable, err := NewKeyer(r.root, r.graph).Key(id)
		if err != nil {
			fmt.Fprintln(os.Stderr, "pipeline:", err)
			stop = true
			return
		}
		// the recorded stamp carries the output digest too: a node whose own
		// output was edited under it is stale, however the edit got there
		fresh := skippable && r.recorded(id) == stampValue(r.root, n, key) && !r.force
		if fresh {
			if present, missing := OutputsPresent(r.root, n); !present {
				fmt.Printf("  %s: key matches but %s is missing — rebuilding\n", id, missing)
				fresh = false
			}
		}
		if fresh {
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
		reportUndeclaredReads(id, res.reads)
		badReads += len(res.reads)
		if res.err != nil {
			failed++
			if r.report.Failed == nil {
				r.report.Failed = map[NodeID]failedNode{}
			}
			cmd := ""
			if len(res.node.Run) > 0 {
				cmd = strings.Join(res.node.Run[len(res.node.Run)-1], " ")
			}
			r.report.Failed[id] = failedNode{Cmd: cmd, Tail: tailOf(res.output)}
			// The `why` is the point of the report: a red gate is only
			// actionable if you know what it was protecting. Printed here
			// rather than left to the reader to look up in nodes.go.
			fmt.Fprintf(os.Stderr, "\nFAIL  %s: %v\n", id, res.err)
			if res.node.Why != "" {
				fmt.Fprintf(os.Stderr, "\n  why this node exists:\n    %s\n", res.node.Why)
			}
			fmt.Fprintf(os.Stderr, "\n  reproduce just this node (rebuilds only what it needs):\n    pipeline run %s\n", id)
			for _, argv := range res.node.Run {
				fmt.Fprintf(os.Stderr, "  run the command alone:\n    %s\n", strings.Join(argv, " "))
			}
			if !r.continueOnFail {
				stop = true
			}
			continue // dependents never become ready: their pending count never reaches 0
		}
		ran++
		done[id] = true
		r.report.Passed = append(r.report.Passed, id)
		for _, d := range dependents[id] {
			pending[d]--
			if pending[d] == 0 && !stop {
				ready = append(ready, d)
			}
		}
	}
	// whatever never ran and never skipped was blocked by a failed dependency
	for _, n := range plan {
		if !done[n.ID] {
			if _, isFailed := r.report.Failed[n.ID]; !isFailed {
				r.report.Blocked = append(r.report.Blocked, n.ID)
			}
		}
	}
	if blocked := len(r.report.Blocked); blocked > 0 {
		fmt.Fprintf(os.Stderr, "%d node(s) not reached (a dependency failed)\n", blocked)
	}
	return
}
