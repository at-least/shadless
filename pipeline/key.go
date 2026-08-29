package main

// Freshness. The whole rule:
//
//	key(n) = H( n.id, n.run, contents of every file n declares as input,
//	            key(d) for every d in n.needs )
//
// A node is fresh iff its recorded key equals its computed key. Because a
// dependency's key is folded in, "something upstream changed" invalidates a
// node without anyone having to say so — that is the only propagation
// mechanism, and there is no second one to disagree with it.
//
// A node with no declared input set (Inputs == nil) judges state outside the
// tree, so it has no key, and neither does anything downstream of it.
//
// A matching key is necessary but not sufficient: the node's declared outputs
// have to still BE there. On a fresh clone every stamp could match — the
// inputs are all committed — while the gitignored half of the outputs does not
// exist yet. Checking the key alone would skip the work and hand the next node
// an empty directory.

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// ---------------------------------------------------------------- globbing
// Patterns are the registry's: "*" within a segment, "**" across segments,
// a leading "!" excluding. filepath.Glob has no "**", so patterns compile to
// regexps and the literal prefix decides where the walk starts.

var metaChars = regexp.MustCompile(`[*?\[]`)

func globToRegexp(pat string) (*regexp.Regexp, error) {
	var b strings.Builder
	b.WriteString("^")
	for i := 0; i < len(pat); i++ {
		switch {
		case strings.HasPrefix(pat[i:], "**/"):
			b.WriteString(`(?:[^/]+/)*`)
			i += 2
		case strings.HasPrefix(pat[i:], "**"):
			b.WriteString(`.*`)
			i++
		case pat[i] == '*':
			b.WriteString(`[^/]*`)
		case pat[i] == '?':
			b.WriteString(`[^/]`)
		default:
			b.WriteString(regexp.QuoteMeta(string(pat[i])))
		}
	}
	b.WriteString("$")
	return regexp.Compile(b.String())
}

// literalPrefix is the deepest directory of pat that contains no wildcard —
// where the walk can start instead of scanning the repo.
func literalPrefix(pat string) string {
	segs := strings.Split(pat, "/")
	var keep []string
	for _, s := range segs {
		if metaChars.MatchString(s) {
			break
		}
		keep = append(keep, s)
	}
	if len(keep) == len(segs) {
		return pat // fully literal
	}
	return strings.Join(keep, "/")
}

// expand resolves one pattern to the sorted set of files it names, relative
// to root. A fully literal pattern naming a directory expands to that whole
// directory: the registry writes `produces: ["dist/css"]` meaning all of it.
func expand(root, pat string) ([]string, error) {
	prefix := literalPrefix(pat)
	fullyLiteral := prefix == pat
	start := filepath.Join(root, prefix)
	info, err := os.Stat(start)
	if err != nil {
		return nil, nil // a pattern naming nothing contributes nothing
	}
	if fullyLiteral && !info.IsDir() {
		return []string{prefix}, nil
	}
	re, err := globToRegexp(pat)
	if err != nil {
		return nil, err
	}
	var out []string
	err = filepath.WalkDir(start, func(p string, d os.DirEntry, err error) error {
		if err != nil {
			return nil // unreadable entries are not silently-passing inputs; they are absent
		}
		if d.IsDir() {
			return nil
		}
		rel, err := filepath.Rel(root, p)
		if err != nil {
			return nil
		}
		if fullyLiteral || re.MatchString(rel) {
			out = append(out, rel)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	sort.Strings(out)
	return out, nil
}

// Files resolves a node's whole pattern list, applying "!" exclusions last so
// order in the registry does not matter.
func Files(root string, patterns []string) ([]string, error) {
	include := map[string]bool{}
	var excludes []*regexp.Regexp
	for _, p := range patterns {
		if strings.HasPrefix(p, "!") {
			re, err := globToRegexp(strings.TrimPrefix(p, "!"))
			if err != nil {
				return nil, err
			}
			excludes = append(excludes, re)
			continue
		}
		fs, err := expand(root, p)
		if err != nil {
			return nil, err
		}
		for _, f := range fs {
			include[f] = true
		}
	}
	var out []string
	for f := range include {
		skip := false
		for _, re := range excludes {
			if re.MatchString(f) {
				skip = true
				break
			}
		}
		if !skip {
			out = append(out, f)
		}
	}
	sort.Strings(out)
	return out, nil
}

// ------------------------------------------------------------------- keys

func hashFile(path string) (string, error) {
	f, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer f.Close()
	h := sha256.New()
	if _, err := io.Copy(h, f); err != nil {
		return "", err
	}
	return hex.EncodeToString(h.Sum(nil)), nil
}

type Keyer struct {
	root  string
	graph *Graph
	memo  map[NodeID]string // id -> key, "" = never fresh
}

func NewKeyer(root string, g *Graph) *Keyer {
	return &Keyer{root: root, graph: g, memo: map[NodeID]string{}}
}

// Key returns the node's content key, or ok=false when the node (or anything
// upstream of it) declares no input set and therefore can never be skipped.
func (k *Keyer) Key(id NodeID) (string, bool, error) {
	if v, seen := k.memo[id]; seen {
		return v, v != "", nil
	}
	n, ok := k.graph.Node(id)
	if !ok {
		return "", false, fmt.Errorf("unknown node id: %s", id)
	}
	if n.NeverFresh() {
		k.memo[id] = ""
		return "", false, nil
	}
	h := sha256.New()
	fmt.Fprintf(h, "node\x00%s\n", n.ID)
	for _, cmd := range n.Run {
		fmt.Fprintf(h, "run\x00%s\n", strings.Join(cmd, "\x00"))
	}

	deps := append([]NodeID(nil), n.Needs...)
	sort.Slice(deps, func(i, j int) bool { return deps[i] < deps[j] })
	for _, d := range deps {
		dk, ok, err := k.Key(d)
		if err != nil {
			return "", false, err
		}
		if !ok {
			k.memo[id] = "" // an unskippable dependency makes this node unskippable
			return "", false, nil
		}
		fmt.Fprintf(h, "dep\x00%s\x00%s\n", d, dk)
	}

	files, err := Files(k.root, n.Inputs)
	if err != nil {
		return "", false, err
	}
	for _, f := range files {
		fh, err := hashFile(filepath.Join(k.root, f))
		if err != nil {
			return "", false, err
		}
		fmt.Fprintf(h, "in\x00%s\x00%s\n", f, fh)
	}

	key := hex.EncodeToString(h.Sum(nil))
	k.memo[id] = key
	return key, true, nil
}

// OutputsPresent reports whether everything the node claims to produce is on
// disk. Checked against the literal prefix of each pattern rather than an
// exact file list: the question is "did this get built", not "is every file
// byte-for-byte what it was", which is `reproducible`'s job.
func OutputsPresent(root string, n Node) (bool, string) {
	for _, pat := range n.Produces {
		if strings.HasPrefix(pat, "!") {
			continue
		}
		if p := literalPrefix(pat); p != "" {
			if _, err := os.Stat(filepath.Join(root, p)); err != nil {
				return false, p
			}
		}
	}
	return true, ""
}
