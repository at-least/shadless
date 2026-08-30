package main

// The mutation harness, tested on scratch trees.
//
// This layer is worth testing precisely because it is what the meta-gate
// trusts. A helper that silently no-ops, or a restore that does not restore,
// turns the whole meta report into theatre — every mutation "CAUGHT" or,
// worse, a mutated file left behind in the working tree and committed.

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// tree writes files into a fresh temp dir and returns its path.
func tree(t *testing.T, files map[string]string) string {
	t.Helper()
	root := t.TempDir()
	for p, content := range files {
		full := filepath.Join(root, p)
		if err := os.MkdirAll(filepath.Dir(full), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(full, []byte(content), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

func read(t *testing.T, root, p string) string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(root, p))
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

func TestUnitMutEditRejectsNoOp(t *testing.T) {
	root := tree(t, map[string]string{"a.txt": "hello"})
	err := mutEdit(root, "a.txt", func(s string) (string, error) { return s, nil })
	if err == nil {
		t.Fatal("a no-op edit was accepted — the mutation would prove nothing")
	}
	if !strings.Contains(err.Error(), "no-op") {
		t.Errorf("error should name the no-op, got: %v", err)
	}
	if got := read(t, root, "a.txt"); got != "hello" {
		t.Errorf("a rejected edit still wrote: %q", got)
	}
}

func TestUnitMutEditRejectsMissingFile(t *testing.T) {
	root := tree(t, nil)
	err := mutEdit(root, "nope.txt", func(s string) (string, error) { return s + "x", nil })
	if err == nil {
		t.Fatal("editing a missing file succeeded")
	}
	if !strings.Contains(err.Error(), "build first") {
		t.Errorf("error should hint that the tree is unbuilt, got: %v", err)
	}
}

func TestUnitMutReplaceOnce(t *testing.T) {
	root := tree(t, map[string]string{"a.txt": "x y x y"})
	if err := mutReplaceOnce(root, "a.txt", "x", "Z"); err != nil {
		t.Fatal(err)
	}
	// JS String.replace with a string anchor replaces only the first
	if got, want := read(t, root, "a.txt"), "Z y x y"; got != want {
		t.Errorf("got %q, want %q", got, want)
	}
	if err := mutReplaceOnce(root, "a.txt", "absent", "Z"); err == nil {
		t.Error("a missing anchor was accepted")
	}
}

func TestUnitMutReplaceAll(t *testing.T) {
	root := tree(t, map[string]string{"a.txt": "x y x y"})
	if err := mutReplaceAll(root, "a.txt", "x", "Z"); err != nil {
		t.Fatal(err)
	}
	if got, want := read(t, root, "a.txt"), "Z y Z y"; got != want {
		t.Errorf("got %q, want %q", got, want)
	}
}

func TestUnitMutReplaceReFirstMatchOnly(t *testing.T) {
	root := tree(t, map[string]string{"a.css": "a px-2 b px-4"})
	if err := mutReplaceRe(root, "a.css", rePadding, ""); err != nil {
		t.Fatal(err)
	}
	// the JS this ports used a non-global regex: first match only
	if got, want := read(t, root, "a.css"), "a b px-4"; got != want {
		t.Errorf("got %q, want %q", got, want)
	}
}

func TestUnitMutReplaceReExpandsGroups(t *testing.T) {
	root := tree(t, map[string]string{
		"e.tsx": "export default function Demo() {\n  return null\n}\n",
	})
	if err := mutReplaceRe(root, "e.tsx", reExportDefaultFn, "$1\n  throw new Error(\"x\")"); err != nil {
		t.Fatal(err)
	}
	got := read(t, root, "e.tsx")
	if !strings.Contains(got, "export default function Demo() {\n  throw new Error(\"x\")") {
		t.Errorf("group expansion did not keep the signature: %q", got)
	}
}

func TestUnitMutReplaceReRejectsMissingPattern(t *testing.T) {
	root := tree(t, map[string]string{"a.json": `{"commit": "nope"}`})
	if err := mutReplaceRe(root, "a.json", reCommit, "x"); err == nil {
		t.Error("a pattern that does not match was accepted")
	}
}

func TestUnitMutFindFileIsDeterministic(t *testing.T) {
	root := tree(t, map[string]string{
		"d/b.html": `<i data-slot="badge">`,
		"d/a.html": `<i data-slot="badge">`,
		"d/c.html": `nothing`,
	})
	want := "d/a.html" // sorted, so the choice cannot drift with readdir order
	for i := 0; i < 3; i++ {
		got, err := mutFindFile(root, "d", func(s string) bool {
			return strings.Contains(s, `data-slot="badge"`)
		})
		if err != nil {
			t.Fatal(err)
		}
		if got != want {
			t.Fatalf("got %q, want %q", got, want)
		}
	}
	if _, err := mutFindFile(root, "d", func(s string) bool { return false }); err == nil {
		t.Error("a predicate matching nothing returned a file")
	}
}

func TestUnitInsertAfter(t *testing.T) {
	got, ok := insertAfter(`{"examples": {`+"\n"+`  "a": {}`, `"examples": {`, "\n  \"new\": {},")
	if !ok {
		t.Fatal("anchor not found")
	}
	if !strings.HasPrefix(got, `{"examples": {`+"\n  \"new\": {},") {
		t.Errorf("insert landed in the wrong place: %q", got)
	}
	if _, ok := insertAfter("abc", "zzz", "x"); ok {
		t.Error("a missing anchor reported success")
	}
}

// The JSON-shaped mutations must change exactly one thing. If they reformatted
// the file, the gate would go red over the reformat and the mutation would
// prove nothing.
func TestUnitJSONMutationsAreMinimal(t *testing.T) {
	const exemptions = `{
 "examples": {
  "attachment-demo": {
   "reason": "token drift vs live"
  }
 }
}
`
	for _, id := range []string{"ledger-undocumented-exemption", "ledger-budget-exceeded"} {
		m := mutationByID(t, id)
		root := tree(t, map[string]string{m.Files[0]: exemptions})
		if err := m.Apply(root, m.Files); err != nil {
			t.Fatalf("%s: %v", id, err)
		}
		got := read(t, root, m.Files[0])
		if !strings.Contains(got, `"attachment-demo"`) || !strings.Contains(got, `"reason": "token drift vs live"`) {
			t.Errorf("%s reformatted or dropped the existing entry:\n%s", id, got)
		}
		if !strings.Contains(got, "__mutation") {
			t.Errorf("%s did not add its entry:\n%s", id, got)
		}
	}

	// budget-exceeded must REUSE an existing reason, or it is just the
	// undocumented-exemption mutation under another name
	m := mutationByID(t, "ledger-budget-exceeded")
	root := tree(t, map[string]string{m.Files[0]: exemptions})
	if err := m.Apply(root, m.Files); err != nil {
		t.Fatal(err)
	}
	if n := strings.Count(read(t, root, m.Files[0]), `"reason": "token drift vs live"`); n != 2 {
		t.Errorf("budget mutation should reuse the existing reason (want 2 copies, got %d)", n)
	}
}

func TestUnitOverlayStaleAuthoredZeroesOnlyItsUnit(t *testing.T) {
	m := mutationByID(t, "overlay-stale-authored")
	const manifest = `{
  "units": {
    "behavior:dialog": {
      "hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    },
    "behavior:sheet": {
      "hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    }
  }
}
`
	root := tree(t, map[string]string{m.Files[0]: manifest})
	if err := m.Apply(root, m.Files); err != nil {
		t.Fatal(err)
	}
	got := read(t, root, m.Files[0])
	if !strings.Contains(got, strings.Repeat("0", 64)) {
		t.Error("the sheet hash was not zeroed")
	}
	if !strings.Contains(got, strings.Repeat("a", 64)) {
		t.Error("it also changed behavior:dialog — the mutation must touch one unit")
	}
}

func TestUnitStyleParityValueDriftNeedsRecordedValues(t *testing.T) {
	m := mutationByID(t, "style-parity-recorded-value-drift")
	// a baseline that still records bare ids proves nothing about whether the
	// gate reads values, so the mutation must refuse rather than no-op
	root := tree(t, map[string]string{m.Files[0]: `{"cells": ["some-id"]}`})
	if err := m.Apply(root, m.Files); err == nil {
		t.Error("a bare-id baseline was accepted")
	}
	root = tree(t, map[string]string{
		m.Files[0]: `{"cells": [{"id": "x", "shadless": "4px", "oracle": "8px"}]}`,
	})
	if err := m.Apply(root, m.Files); err != nil {
		t.Fatal(err)
	}
	got := read(t, root, m.Files[0])
	if !strings.Contains(got, "999px") {
		t.Errorf("the recorded value was not perturbed: %s", got)
	}
	if !strings.Contains(got, `"oracle": "8px"`) {
		t.Errorf("it changed more than the one cell value: %s", got)
	}
}

func mutationByID(t *testing.T, id string) Mutation {
	t.Helper()
	for _, m := range Mutations {
		if m.ID == id {
			return m
		}
	}
	t.Fatalf("no mutation %q", id)
	return Mutation{}
}

// ------------------------------------------------------------- snapshot

func TestUnitSnapshotRestoresContent(t *testing.T) {
	root := tree(t, map[string]string{"a.txt": "original"})
	snap, err := takeSnapshot(root, []string{"a.txt"})
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "a.txt"), []byte("mutated"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := snap.restore(); err != nil {
		t.Fatal(err)
	}
	if got := read(t, root, "a.txt"); got != "original" {
		t.Errorf("restore left %q", got)
	}
}

// coverage-drop-contract DELETES its target, so restore has to recreate it.
func TestUnitSnapshotRestoresDeletedFile(t *testing.T) {
	root := tree(t, map[string]string{"a.txt": "original"})
	snap, err := takeSnapshot(root, []string{"a.txt"})
	if err != nil {
		t.Fatal(err)
	}
	if err := os.Remove(filepath.Join(root, "a.txt")); err != nil {
		t.Fatal(err)
	}
	if err := snap.restore(); err != nil {
		t.Fatal(err)
	}
	if got := read(t, root, "a.txt"); got != "original" {
		t.Errorf("restore left %q", got)
	}
}

// The reverse: a file that did NOT exist when the snapshot was taken must be
// removed again, not left behind as an untracked artifact.
func TestUnitSnapshotRemovesCreatedFile(t *testing.T) {
	root := tree(t, nil)
	snap, err := takeSnapshot(root, []string{"new.txt"})
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "new.txt"), []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := snap.restore(); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(filepath.Join(root, "new.txt")); !os.IsNotExist(err) {
		t.Error("restore left behind a file the snapshot never had")
	}
}

func TestUnitActiveRestoreIsIdempotent(t *testing.T) {
	root := tree(t, map[string]string{"a.txt": "original"})
	snap, err := takeSnapshot(root, []string{"a.txt"})
	if err != nil {
		t.Fatal(err)
	}
	setActiveRestore(snap.restore)
	if err := os.WriteFile(filepath.Join(root, "a.txt"), []byte("mutated"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := RestoreActiveMutation(); err != nil {
		t.Fatal(err)
	}
	// the signal handler and the deferred cleanup can both fire; the second
	// must be a no-op rather than restoring a stale snapshot over later work
	if err := os.WriteFile(filepath.Join(root, "a.txt"), []byte("later"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := RestoreActiveMutation(); err != nil {
		t.Fatal(err)
	}
	if got := read(t, root, "a.txt"); got != "later" {
		t.Errorf("a second restore clobbered later work: %q", got)
	}
}

// ------------------------------------------------------------- selection

func TestUnitSelectMutations(t *testing.T) {
	g, err := AuthoredGraph()
	if err != nil {
		t.Fatal(err)
	}
	all, err := selectMutations(g, Mutations, "", "")
	if err != nil {
		t.Fatal(err)
	}
	if len(all) != len(Mutations) {
		t.Errorf("no filter selected %d of %d", len(all), len(Mutations))
	}
	one, err := selectMutations(g, Mutations, "pin-commit-drift", "")
	if err != nil {
		t.Fatal(err)
	}
	if len(one) != 1 || one[0].ID != "pin-commit-drift" {
		t.Errorf("--only selected %v", one)
	}
	if _, err := selectMutations(g, Mutations, "no-such-mutation", ""); err == nil {
		t.Error("an unknown mutation id was accepted")
	}
	fast, err := selectMutations(g, Mutations, "", "fast")
	if err != nil {
		t.Fatal(err)
	}
	if len(fast) == 0 || len(fast) >= len(Mutations) {
		t.Errorf("tier=fast selected %d of %d — expected a proper subset", len(fast), len(Mutations))
	}
	for _, m := range fast {
		if tierRank(g.EffectiveTier(m.Gate)) > tierRank("fast") {
			t.Errorf("%s targets %s, whose effective tier is %s", m.ID, m.Gate, g.EffectiveTier(m.Gate))
		}
	}
	if _, err := selectMutations(g, Mutations, "", "nonsense"); err == nil {
		t.Error("an unknown tier was accepted")
	}
}

// ------------------------------------------------------------- gate exec

func TestUnitRunGateDetectsRedAndGreen(t *testing.T) {
	root := t.TempDir()
	red, _ := runGate(root, Node{Run: [][]string{{"false"}}})
	if !red {
		t.Error("a failing command was not reported as red")
	}
	green, _ := runGate(root, Node{Run: [][]string{{"true"}}})
	if green {
		t.Error("a passing command was reported as red")
	}
	// commands run in order and the first failure stops the gate
	red, _ = runGate(root, Node{Run: [][]string{{"true"}, {"false"}, {"true"}}})
	if !red {
		t.Error("a failure in a later command was missed")
	}
}

func TestUnitMutationRegexesCompile(t *testing.T) {
	// each is a MustCompile at init, so this really asserts they still match
	// the shapes the real files carry
	for _, tc := range []struct {
		name string
		re   *regexp.Regexp
		in   string
	}{
		{"commit", reCommit, `"commit": "` + strings.Repeat("a", 40) + `"`},
		{"primary", rePrimaryToken, "--primary: oklch(0.2 0 0);"},
		{"padding", rePadding, "flex px-2.5 items-center"},
		{"dialog script", reDialogScript, `<script src="../js/dialog.js"></script>`},
		{"reason", reFirstReason, `"reason": "because"`},
		{"shadless cell", reFirstShadlessCell, `"shadless": "4px"`},
	} {
		if !tc.re.MatchString(tc.in) {
			t.Errorf("%s: %s does not match %q", tc.name, tc.re, tc.in)
		}
	}
}
