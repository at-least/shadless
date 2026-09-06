package main

// Declared vs actual writes.
//
// The runner tracks a node's DECLARED outputs and is blind to anything else it
// writes. An undeclared write is not harmless: if some other node declares
// that file as an input, the graph's freshness is being driven by a file that
// no node admits to producing.
//
// The check exists because nothing else can see it: freshness is computed from
// `produces`, which has no way to notice what a node wrote outside it, so "the
// declarations are honest" was an assumption the pipeline had no way to test.
//
// The check is narrowed to the set that can actually do damage: the union of
// every file any node declares as an input. A write outside `produces` that
// nothing reads cannot affect freshness; a write that lands on someone's
// declared input can.

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// inputUniverse is every file the graph declares as an input, anywhere.
func inputUniverse(root string, g *Graph) (map[string]string, error) {
	snap := map[string]string{}
	for _, id := range g.IDs() {
		n, _ := g.Node(id)
		if n.NeverFresh() {
			continue
		}
		files, err := Files(root, n.Inputs)
		if err != nil {
			return nil, err
		}
		for _, f := range files {
			if _, done := snap[f]; done {
				continue
			}
			h, err := hashFile(filepath.Join(root, f))
			if err != nil {
				continue
			}
			snap[f] = h
		}
	}
	return snap, nil
}

type Violation struct {
	Path    string
	Readers []NodeID // nodes declaring Path as an input
}

// undeclaredWrites reports files that changed across a node's run, are read by
// somebody, and are not covered by that node's `produces`.
func undeclaredWrites(root string, g *Graph, n Node, before, after map[string]string) ([]Violation, error) {
	produced, err := Files(root, n.Produces)
	if err != nil {
		return nil, err
	}
	declared := map[string]bool{}
	for _, f := range produced {
		declared[f] = true
	}
	// `produces` may name a directory that did not exist before the run, so
	// also treat anything under a declared literal prefix as covered.
	covered := func(p string) bool {
		if declared[p] {
			return true
		}
		for _, pat := range n.Produces {
			re, err := globToRegexp(pat)
			if err == nil && re.MatchString(p) {
				return true
			}
			if lp := literalPrefix(pat); lp == pat && len(p) > len(lp)+1 && p[:len(lp)+1] == lp+"/" {
				return true
			}
		}
		return false
	}

	var out []Violation
	for path, h := range after {
		if b, existed := before[path]; existed && b == h {
			continue
		}
		if covered(path) {
			continue
		}
		var readers []NodeID
		for _, id := range g.IDs() {
			m, _ := g.Node(id)
			if m.NeverFresh() {
				continue
			}
			fs, err := Files(root, m.Inputs)
			if err != nil {
				continue
			}
			for _, f := range fs {
				if f == path {
					readers = append(readers, id)
					break
				}
			}
		}
		if len(readers) == 0 {
			continue // written but read by nobody: cannot affect freshness
		}
		out = append(out, Violation{Path: path, Readers: readers})
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Path < out[j].Path })
	return out, nil
}

func reportViolations(id NodeID, vs []Violation) {
	if len(vs) == 0 {
		return
	}
	fmt.Printf("  ⚠ %s wrote %d file(s) it does not declare in `produces`:\n", id, len(vs))
	for _, v := range vs {
		fmt.Printf("      %s   (declared as an input by: %v)\n", v.Path, v.Readers)
	}
}

// ------------------------------------------------------------------ reads
//
// Declared vs actual READS — the mirror of the check above, and it closes the
// same class of hole from the other side.
//
// A node's key hashes exactly the files its `inputs` globs resolve to. If the
// node actually reads a file that no glob covers, that file is not in the key:
// change it and the node stays "fresh" over a tree it would now judge
// differently. That is a stale GREEN, the most expensive kind of wrong answer
// this pipeline can give, and until now nothing could see it — `inputs` was
// hand-maintained prose that no test compared against reality.
//
// The evidence comes from the Go toolchain itself. `go test
// -test.testlogfile=F` makes the test binary record every file it opens, which
// is precisely "what this gate actually read". So the check costs no extra
// execution: the gate was going to run anyway.
//
// Scope, stated plainly because the gaps matter:
//
//   - only nodes whose commands are `go test` produce a testlog, so JS nodes
//     are not covered. Those keep the declaration discipline they had.
//   - a subprocess's reads are invisible: `pin` shells out to git, and git's
//     file access is not in the log. The check under-reports there.
//   - `open` only, not `stat`. An open is a content access, which is what the
//     key hashes; folding in stat would drown the report in repoRoot's own
//     walk up the tree looking for its marker.
//   - in a TESTLOG an `open` for WRITING looks the same as one for reading, so
//     a `go test` node that writes somewhere it has not declared shows up here
//     too. That is a feature — it catches undeclared writes on Go nodes even
//     at -j>1, where the write check above is switched off — but it means the
//     report has to offer both fixes, `inputs` and `produces`, rather than
//     assume a read. It does NOT extend to the JS half: fs-record.mjs hooks
//     the read surface and nothing else, so an undeclared write by a JS node
//     is only ever caught by the -j1 write check.
//
// Under-reporting is the safe direction: this finds real undeclared reads and
// never invents one.

// parseOpenedFiles is the shared skeleton behind both file-access logs: read
// logPath, pull a path out of each line with parseLine, resolve it against
// root, drop anything outside the repo or excluded by exclude, drop
// directories, dedupe, and sort. testlogOpens and fsRecordOpens differ only in
// how a line is parsed and what (if anything) gets excluded.
func parseOpenedFiles(root, logPath string, parseLine func(string) (string, bool), exclude func(string) bool) ([]string, error) {
	b, err := os.ReadFile(logPath)
	if err != nil {
		return nil, err
	}
	seen := map[string]bool{}
	var out []string
	for _, line := range strings.Split(string(b), "\n") {
		path, ok := parseLine(line)
		if !ok {
			continue
		}
		if !filepath.IsAbs(path) {
			continue // relative entries are the test's own cwd noise
		}
		rel, err := filepath.Rel(root, path)
		if err != nil || strings.HasPrefix(rel, "..") {
			continue // outside the repo
		}
		if seen[rel] || exclude(rel) {
			continue
		}
		info, err := os.Stat(path)
		if err != nil || info.IsDir() {
			continue
		}
		seen[rel] = true
		out = append(out, rel)
	}
	sort.Strings(out)
	return out, nil
}

// testlogOpens returns the repo-relative regular files a testlog records as
// opened. Directories, absent paths and anything outside the repo (GOROOT,
// the module cache) are dropped.
func testlogOpens(root, logPath string) ([]string, error) {
	return parseOpenedFiles(root, logPath, func(line string) (string, bool) {
		return strings.CutPrefix(line, "open ")
	}, func(string) bool { return false })
}

// undeclaredReads reports files the node opened that nothing puts in its key.
//
// Three things count as covered, and the third is the one that makes this
// check usable rather than merely noisy:
//
//   - the node's own `inputs`, obviously.
//
//   - the node's own `produces`: reading back what you just wrote is not a
//     missing declaration, and declaring your own output as your own input
//     would be circular.
//
//   - anything a node in its dependency CLOSURE declares, as an input OR as an
//     output. The key folds in each dependency's key, and that key already
//     hashes that dependency's inputs — so a change to either reaches this
//     node transitively. That is the merkle chain the whole design rests on,
//     and pipeline/README.md states half of it: "outputs of `needs` are
//     implied". The inputs half follows from the same identity.
//
//     Without this, `emit` reading generated/ir (which `convert` produces
//     and `emit` needs) would be reported, and declaring it would be
//     redundant noise that teaches people to ignore the report. It is the
//     difference between 64 findings on `emit` and 1 real one.
func undeclaredReads(root string, g *Graph, n Node, opens []string) ([]string, error) {
	if n.NeverFresh() {
		return nil, nil // it can never be skipped, so nothing can go stale-green
	}
	covered := map[string]bool{}
	add := func(patterns []string) error {
		fs, err := Files(root, patterns)
		if err != nil {
			return err
		}
		for _, f := range fs {
			covered[f] = true
		}
		return nil
	}
	if err := add(n.Inputs); err != nil {
		return nil, err
	}

	// everything the closure declares, in either direction
	producePatterns := append([]string(nil), n.Produces...)
	if g != nil {
		closure, err := g.Plan([]NodeID{n.ID})
		if err == nil {
			for _, d := range closure {
				if d.ID == n.ID {
					continue
				}
				producePatterns = append(producePatterns, d.Produces...)
				if err := add(d.Inputs); err != nil {
					return nil, err
				}
			}
		}
	}
	if err := add(producePatterns); err != nil {
		return nil, err
	}
	// a `produces` entry may name a directory that did not exist when the
	// patterns were expanded, so also treat anything under a literal prefix
	// as covered — the same allowance the write check makes
	underProduces := func(p string) bool {
		for _, pat := range producePatterns {
			if lp := literalPrefix(pat); lp == pat && strings.HasPrefix(p, lp+"/") {
				return true
			}
		}
		return false
	}

	var out []string
	for _, p := range opens {
		if covered[p] || underProduces(p) {
			continue
		}
		out = append(out, p)
	}
	return out, nil
}

func reportUndeclaredReads(id NodeID, paths []string) {
	if len(paths) == 0 {
		return
	}
	fmt.Printf("  ⚠ %s opened %d file(s) it declares in neither `inputs` nor `produces`:\n", id, len(paths))
	for _, p := range paths {
		fmt.Printf("      %s\n", p)
	}
	fmt.Printf("      a file it READS belongs in `inputs` (it is not in the node's key, so a\n" +
		"      change to it leaves the node falsely fresh); a file it WRITES belongs in `produces`\n")
}

// fsRecordOpens parses the log tools/fs-record.mjs writes: one absolute path
// per line, already de-duplicated per process, possibly appended to by several
// processes in a node's command list.
//
// Two exclusions, and both are judgement calls worth stating:
//
//   - node_modules. A tool that bundles (esbuild) or compiles mdx reads
//     thousands of files under it, and requiring a node to enumerate its npm
//     dependency tree in `inputs` would be absurd. package-lock.json is this
//     repo's accepted proxy for "the dependency set changed" and several
//     nodes already declare it. The cost is real: the tailwind CLI binary that
//     the Go side flagged for consumer-sim would be invisible here.
//   - build/. The pipeline's own scratch space, where a node's intermediate
//     output is neither an input nor something worth declaring.
func fsRecordOpens(root, logPath string) ([]string, error) {
	return parseOpenedFiles(root, logPath, func(line string) (string, bool) {
		p := strings.TrimSpace(line)
		return p, p != ""
	}, excludedFromAccessCheck)
}

func excludedFromAccessCheck(rel string) bool {
	for _, prefix := range []string{"node_modules/", "build/", ".git/"} {
		if strings.HasPrefix(rel, prefix) {
			return true
		}
	}
	return false
}
