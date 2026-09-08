package main

// The recorded-difference ledger. Ported from gates/ledger.mjs.
//
// Replaces tools/exemption-ledger.mjs + the prose EXEMPTIONS.md as the
// MACHINE surface. EXEMPTIONS.md is GENERATED from gates/ledger.json
// (`pipeline ledger --render`), so the human-readable file cannot drift from
// the checked one.
//
// Why this shape:
//
//	class      every exemption declares how it ENDS, not just why it exists.
//	           "permanent"     a real engine/by-design difference
//	           "auto-dissolve" expected to vanish at the next re-pin — the
//	                           upstream pipeline DELETES these wholesale and
//	                           lets whatever still fails come back with
//	                           evidence. No human "walks the Automation
//	                           column" any more; that was the review step
//	                           most likely to be skipped.
//	           "debt"          tracked work; carries a budget
//
//	budgets    the ratchet, done right. tools/style-parity.mjs recorded a
//	           COUNT per component but compared only PRESENCE, so 23 of 29
//	           components could accumulate unlimited new drift while the gate
//	           stayed green. Here a budget fails when the number GROWS *and*
//	           when it SHRINKS without being re-recorded — slack cannot
//	           silently accumulate.
//
//	recorded_at_pin  which upstream release the difference was observed
//	           against. An auto-dissolve entry older than the current pin is
//	           stale by definition and is reported.
//
// The VERIFY half is a gate, so it is a Go test (TestLedger). The three
// halves that WRITE — record, render, dissolve — are subcommands, because
// they mutate the tree and assert nothing.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

const (
	// ledgerPath lives in gate_coverage_budget.go — the coverage budget is
	// stored in the same file, and one path constant means one file.
	renderedPath  = "EXEMPTIONS.md"
	goldenExPath  = "src/registry/upstream-snapshot/exemptions.json"
	contractsDir  = "tools/contracts/components"
	emitterCSS    = "src/emitter/css.mjs"
	emitterSkin   = "src/emitter/skin.mjs"
	sweepPath     = "pipeline/interactivity_sweep.go"
	todoReasonPfx = "TODO"
)

var ledgerClasses = map[string]bool{"permanent": true, "auto-dissolve": true, "debt": true}

type ledgerEntry struct {
	Class    string `json:"class"`
	Reason   string `json:"reason"`
	Source   string `json:"source"`
	Recorded string `json:"recorded_at_pin"`
}

type ledgerBudget struct {
	Max    int    `json:"max"`
	Target int    `json:"target"`
	Class  string `json:"class"`
	Reason string `json:"reason"`
}

// ledgerFile keeps key order alongside the maps: gates/ledger.json is
// committed, and a Go map would reshuffle 66 entries on every write, turning
// a one-line change into a whole-file diff.
type ledgerFile struct {
	Pin         string
	EntryOrder  []string
	Entries     map[string]*ledgerEntry
	BudgetOrder []string
	Budgets     map[string]*ledgerBudget
	Notes       []string
}

func readLedger(root string) (*ledgerFile, error) {
	b, err := os.ReadFile(filepath.Join(root, ledgerPath))
	if err != nil {
		return nil, err
	}
	var raw struct {
		Pin     string                     `json:"pin"`
		Entries map[string]*ledgerEntry    `json:"entries"`
		Budgets map[string]*ledgerBudget   `json:"budgets"`
		Notes   []string                   `json:"notes"`
		_       map[string]json.RawMessage `json:"-"`
	}
	if err := json.Unmarshal(b, &raw); err != nil {
		return nil, fmt.Errorf("%s: %w", ledgerPath, err)
	}
	l := &ledgerFile{
		Pin: raw.Pin, Entries: raw.Entries, Budgets: raw.Budgets, Notes: raw.Notes,
	}
	if l.Entries == nil {
		l.Entries = map[string]*ledgerEntry{}
	}
	if l.Budgets == nil {
		l.Budgets = map[string]*ledgerBudget{}
	}
	l.EntryOrder, err = jsonKeyOrder(b, "entries")
	if err != nil {
		return nil, err
	}
	l.BudgetOrder, err = jsonKeyOrder(b, "budgets")
	if err != nil {
		return nil, err
	}
	return l, nil
}

// jsonKeyOrder returns the keys of a top-level object field in file order.
// encoding/json discards it, and the committed file's order is the diff.
func jsonKeyOrder(data []byte, field string) ([]string, error) {
	var top map[string]json.RawMessage
	if err := json.Unmarshal(data, &top); err != nil {
		return nil, err
	}
	raw, ok := top[field]
	if !ok {
		return nil, nil
	}
	dec := json.NewDecoder(strings.NewReader(string(raw)))
	tok, err := dec.Token()
	if err != nil {
		return nil, err
	}
	if d, ok := tok.(json.Delim); !ok || d != '{' {
		return nil, fmt.Errorf("%s is not an object", field)
	}
	var keys []string
	for dec.More() {
		k, err := dec.Token()
		if err != nil {
			return nil, err
		}
		keys = append(keys, k.(string))
		// skip the value, whatever its shape
		var skip json.RawMessage
		if err := dec.Decode(&skip); err != nil {
			return nil, err
		}
	}
	return keys, nil
}

// writeLedger emits JSON.stringify(l, null, 2) + "\n" — the bytes the JS
// wrote, so the port does not reformat a committed file on its first run.
func (l *ledgerFile) write(root string) error {
	entries := jsonObj{}
	for _, id := range l.EntryOrder {
		e := l.Entries[id]
		entries = entries.add(id, jsonObj{}.
			add("class", e.Class).
			add("reason", e.Reason).
			add("source", e.Source).
			add("recorded_at_pin", e.Recorded))
	}
	budgets := jsonObj{}
	for _, n := range l.BudgetOrder {
		b := l.Budgets[n]
		budgets = budgets.add(n, jsonObj{}.
			add("max", b.Max).
			add("target", b.Target).
			add("class", b.Class).
			add("reason", b.Reason))
	}
	notes := make([]any, len(l.Notes))
	for i, n := range l.Notes {
		notes[i] = n
	}
	out := jsonObj{}.
		add("pin", l.Pin).
		add("entries", entries).
		add("budgets", budgets).
		add("notes", notes)
	return os.WriteFile(filepath.Join(root, ledgerPath),
		[]byte(marshalJS(out, "")+"\n"), 0o644)
}

// ---------------------------------------------------------------- sources

// sourceID is one exemption the codebase currently claims, and where from.
type sourceID struct {
	ID     string
	Source string
}

// collectSourceIds derives every id the sources currently claim. The ids ARE
// the contract with the sources, so this must stay the same derivation the
// JS used — an id that changes shape reads as "one exemption vanished, an
// undocumented one appeared" and fails the gate in both directions at once.
func collectSourceIds(root string) ([]sourceID, error) {
	var out []sourceID
	seen := map[string]bool{}
	add := func(id, src string) {
		if seen[id] { // a Map in JS: later writes do not duplicate the key
			return
		}
		seen[id] = true
		out = append(out, sourceID{id, src})
	}

	entries, err := os.ReadDir(filepath.Join(root, contractsDir))
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		if strings.HasSuffix(e.Name(), ".mjs") {
			names = append(names, e.Name())
		}
	}
	sort.Strings(names) // readdirSync order in JS is already sorted on ext4/APFS
	for _, f := range names {
		name := strings.TrimSuffix(f, ".mjs")
		src, err := os.ReadFile(filepath.Join(root, contractsDir, f))
		if err != nil {
			return nil, err
		}
		body := string(src)
		if jsFieldIsFalse(body, "mountedCheck") {
			add("mounted-check:"+name, "contracts")
		}
		if jsFieldIsFalse(body, "mountedClasses") {
			add("mounted-classes:"+name, "contracts")
		}
		attrs, ok, err := jsObjectField(body, "ignoreAttrs")
		if err != nil {
			return nil, fmt.Errorf("%s/%s: %w", contractsDir, f, err)
		}
		if ok {
			for _, e := range jsAttrMap(attrs) {
				for _, a := range e.Values {
					add(fmt.Sprintf("ignore-attrs:%s:%s:%s", name, e.Key, a), "contracts")
				}
			}
		}
	}

	reasons, err := goldenReasons(root)
	if err != nil {
		return nil, err
	}
	for _, r := range reasons {
		add("golden:"+r, "golden")
	}

	cssSrc, err := os.ReadFile(filepath.Join(root, emitterCSS))
	if err != nil {
		return nil, err
	}
	dead, err := jsSetLiteral(string(cssSrc), "DEAD_UTILITIES")
	if err != nil {
		return nil, fmt.Errorf("%s: %w", emitterCSS, err)
	}
	for _, t := range dead {
		add("dead-utility:"+t, "emitter")
	}

	skinSrc, err := os.ReadFile(filepath.Join(root, emitterSkin))
	if err != nil {
		return nil, err
	}
	skin, err := jsSetLiteral(string(skinSrc), "SKIN_ALLOWLIST")
	if err != nil {
		return nil, fmt.Errorf("%s: %w", emitterSkin, err)
	}
	for _, t := range skin {
		add("skin-allowlist:"+t, "emitter")
	}
	return out, nil
}

// goldenExemptions is src/registry/upstream-snapshot/exemptions.json, whose
// key order is load-bearing (it is committed) and whose values carry the
// reason each demo is exempt from the golden dual hop.
type goldenExemptions struct {
	Order   []string
	Reasons map[string]string
}

func readGoldenExemptions(root string) (*goldenExemptions, error) {
	b, err := os.ReadFile(filepath.Join(root, goldenExPath))
	if err != nil {
		return nil, err
	}
	var raw struct {
		Examples map[string]struct {
			Reason string `json:"reason"`
		} `json:"examples"`
	}
	if err := json.Unmarshal(b, &raw); err != nil {
		return nil, fmt.Errorf("%s: %w", goldenExPath, err)
	}
	order, err := jsonKeyOrder(b, "examples")
	if err != nil {
		return nil, err
	}
	g := &goldenExemptions{Order: order, Reasons: map[string]string{}}
	for k, v := range raw.Examples {
		g.Reasons[k] = v.Reason
	}
	return g, nil
}

// goldenReasons is the distinct reasons, in file order.
func goldenReasons(root string) ([]string, error) {
	g, err := readGoldenExemptions(root)
	if err != nil {
		return nil, err
	}
	var out []string
	seen := map[string]bool{}
	for _, demo := range g.Order {
		r := g.Reasons[demo]
		if !seen[r] {
			seen[r] = true
			out = append(out, r)
		}
	}
	return out, nil
}

// collectBudgetValues reads the live value of every budgeted number from the
// same place the gate that reports it reads. This is the ONLY definition of
// each metric; a second one is how a ratchet silently stops ratcheting.
//
// A value of -1 means "could not read", which fails. Absence from the map is
// different and means "not checked here" — the coverage.* budgets need the
// IR, so TestCoverage owns them.
func collectBudgetValues(root string) (map[string]int, error) {
	v := map[string]int{}

	g, err := readGoldenExemptions(root)
	if err != nil {
		return nil, err
	}
	v["golden.exempt-demos"] = len(g.Order)

	sweep, err := os.ReadFile(filepath.Join(root, sweepPath))
	if err != nil {
		return nil, err
	}
	// sweepKnownDead is a Go map literal now; count its keys
	n := goMapKeyCount(string(sweep), "sweepKnownDead")
	v["interactivity.dead-families"] = n

	for name, path := range map[string]string{
		"demo-parity.dirty-cells":  "gates/demo-parity-baseline.json",
		"path-parity.dirty-cells":  "gates/path-parity-baseline.json",
		"style-parity.dirty-cells": "gates/style-parity-baseline.json",
	} {
		v[name] = baselineCellCount(root, path)
	}
	return v, nil
}

func baselineCellCount(root, path string) int {
	b, err := os.ReadFile(filepath.Join(root, path))
	if err != nil {
		return -1
	}
	var doc struct {
		Cells []json.RawMessage `json:"cells"`
	}
	if err := json.Unmarshal(b, &doc); err != nil {
		return -1
	}
	return len(doc.Cells)
}

func currentPin(root string) (string, error) {
	p, err := readPin(root)
	if err != nil {
		return "", err
	}
	return p.ShadcnUI.Tag, nil
}

// classOfGoldenReason: golden reasons carry their own end-condition in the
// text; trust it so a re-pin dissolves them without anyone re-classifying
// 147 rows by hand.
func classOfGoldenReason(r string) string {
	l := strings.ToLower(r)
	for _, sig := range []string{"re-check on re-pin", "deploy lag", "frame lag"} {
		if strings.Contains(l, sig) {
			return "auto-dissolve"
		}
	}
	return "permanent"
}

// ---------------------------------------------------------------- verify

func gateLedger(root string) error {
	l, err := readLedger(root)
	if err != nil {
		return err
	}
	sources, err := collectSourceIds(root)
	if err != nil {
		return err
	}
	inSources := map[string]bool{}
	for _, s := range sources {
		inSources[s.ID] = true
	}

	var problems []string
	for _, id := range l.EntryOrder {
		e := l.Entries[id]
		if !ledgerClasses[e.Class] {
			problems = append(problems, fmt.Sprintf("%s: unknown class %q", id, e.Class))
		}
		if len(e.Reason) < 8 {
			problems = append(problems, fmt.Sprintf("%s: missing or trivial reason", id))
		}
		if e.Recorded == "" {
			problems = append(problems, fmt.Sprintf("%s: no recorded_at_pin", id))
		}
	}

	var undocumented []string
	for _, s := range sources {
		if _, ok := l.Entries[s.ID]; !ok {
			undocumented = append(undocumented, s.ID)
		}
	}
	sort.Strings(undocumented)
	var stale []string
	for _, id := range l.EntryOrder {
		if !inSources[id] {
			stale = append(stale, id)
		}
	}
	sort.Strings(stale)
	if len(undocumented) > 0 {
		problems = append(problems,
			"exemptions in the sources with no ledger entry (add them, with a class):\n    "+
				strings.Join(undocumented, "\n    "))
	}
	if len(stale) > 0 {
		problems = append(problems,
			"ledger entries whose source flag is gone (delete them):\n    "+
				strings.Join(stale, "\n    "))
	}

	values, err := collectBudgetValues(root)
	if err != nil {
		return err
	}
	for _, name := range l.BudgetOrder {
		b := l.Budgets[name]
		actual, known := values[name]
		if !known && strings.HasPrefix(name, "coverage.") {
			continue // checked by `go test -run TestCoverage`
		}
		if !known {
			problems = append(problems, fmt.Sprintf("budget %s: no live value known", name))
			continue
		}
		if actual < 0 {
			problems = append(problems, fmt.Sprintf("budget %s: could not read the live value", name))
			continue
		}
		switch {
		case actual > b.Max:
			problems = append(problems, fmt.Sprintf(
				"budget %s: %d > recorded max %d — this number may only shrink", name, actual, b.Max))
		case actual < b.Max:
			problems = append(problems, fmt.Sprintf(
				"budget %s: %d < recorded max %d — it improved; "+
					"re-record so the slack cannot be silently re-spent:  pipeline ledger --record",
				name, actual, b.Max))
		}
	}

	if len(problems) > 0 {
		return fmt.Errorf("FAIL  ledger\n  %s", strings.Join(problems, "\n  "))
	}

	pin, err := currentPin(root)
	if err != nil {
		return err
	}
	var staleAuto []string
	byClass := map[string]int{}
	for _, id := range l.EntryOrder {
		e := l.Entries[id]
		byClass[e.Class]++
		if e.Class == "auto-dissolve" && e.Recorded != pin {
			staleAuto = append(staleAuto, id)
		}
	}
	var counts []string
	for _, c := range []string{"permanent", "auto-dissolve", "debt"} {
		if byClass[c] > 0 {
			counts = append(counts, fmt.Sprintf("%d %s", byClass[c], c))
		}
	}
	msg := fmt.Sprintf("PASS  ledger (%d exemptions: %s; %d budgets at their recorded max)",
		len(l.EntryOrder), strings.Join(counts, ", "), len(l.BudgetOrder))
	if len(staleAuto) > 0 {
		msg += fmt.Sprintf("\n  note: %d auto-dissolve entries predate pin %s "+
			"— they should have dissolved at the last re-pin", len(staleAuto), pin)
	}
	fmt.Println(msg)
	return nil
}

// ---------------------------------------------------------------- record

// ledgerRecord reconciles the ledger with the sources: add missing ids, drop
// stale ones, re-record budgets. Reasons for new ids are harvested from the id
// itself where it carries one (golden), otherwise flagged TODO so the human
// writes the reason rather than the tool inventing it.
func ledgerRecord(root string) error {
	l, err := readLedger(root)
	if err != nil {
		return err
	}
	pin, err := currentPin(root)
	if err != nil {
		return err
	}
	l.Pin = pin
	sources, err := collectSourceIds(root)
	if err != nil {
		return err
	}
	inSources := map[string]bool{}
	for _, s := range sources {
		inSources[s.ID] = true
	}

	added, dropped := 0, 0
	for _, s := range sources {
		if _, ok := l.Entries[s.ID]; ok {
			continue
		}
		reason := "TODO: state why this difference is accepted"
		class := "permanent"
		if r, ok := strings.CutPrefix(s.ID, "golden:"); ok {
			reason, class = r, classOfGoldenReason(r)
		}
		l.Entries[s.ID] = &ledgerEntry{Class: class, Reason: reason, Source: s.Source, Recorded: pin}
		l.EntryOrder = append(l.EntryOrder, s.ID)
		added++
	}
	kept := l.EntryOrder[:0]
	for _, id := range l.EntryOrder {
		if !inSources[id] {
			delete(l.Entries, id)
			dropped++
			continue
		}
		kept = append(kept, id)
	}
	l.EntryOrder = kept

	values, err := collectBudgetValues(root)
	if err != nil {
		return err
	}
	for _, name := range l.BudgetOrder {
		b := l.Budgets[name]
		if v, ok := values[name]; ok && v >= 0 && v != b.Max {
			fmt.Printf("  budget %s: %d -> %d\n", name, b.Max, v)
			b.Max = v
		}
	}
	if err := l.write(root); err != nil {
		return err
	}
	todo := 0
	for _, e := range l.Entries {
		if strings.HasPrefix(e.Reason, todoReasonPfx) {
			todo++
		}
	}
	fmt.Printf("ledger recorded: +%d -%d, %d entries\n", added, dropped, len(l.EntryOrder))
	if todo > 0 {
		fmt.Printf("  %d entries still need a real reason (search for TODO in %s)\n", todo, ledgerPath)
	}
	return nil
}

// -------------------------------------------------------------- dissolve

// ledgerDissolve is called by the upstream drill right after a re-pin: delete
// every auto-dissolve entry recorded against a DIFFERENT pin, so the rebuild
// has to re-earn each one. Whatever the gates still flag comes back as a real,
// evidenced failure instead of hiding under an exemption recorded against an
// older upstream. Entries recorded at the current pin stay — a same-tag drill
// (the self-test) must be a no-op here.
//
// The golden exemptions are ALSO a source file (one row per demo): the demos
// whose reason dissolved are pruned there too, otherwise the ledger and the
// source disagree and the verify step fails for the wrong reason.
func ledgerDissolve(root string) error {
	l, err := readLedger(root)
	if err != nil {
		return err
	}
	pin, err := currentPin(root)
	if err != nil {
		return err
	}
	var gone []string
	for _, id := range l.EntryOrder {
		e := l.Entries[id]
		if e.Class == "auto-dissolve" && e.Recorded != pin {
			gone = append(gone, id)
		}
	}
	goneSet := map[string]bool{}
	for _, id := range gone {
		goneSet[id] = true
		delete(l.Entries, id)
	}
	kept := l.EntryOrder[:0]
	for _, id := range l.EntryOrder {
		if !goneSet[id] {
			kept = append(kept, id)
		}
	}
	l.EntryOrder = kept
	l.Pin = pin

	goneReasons := map[string]bool{}
	for _, id := range gone {
		if r, ok := strings.CutPrefix(id, "golden:"); ok {
			goneReasons[r] = true
		}
	}
	g, err := readGoldenExemptions(root)
	if err != nil {
		return err
	}
	pruned := 0
	keptDemos := g.Order[:0]
	for _, demo := range g.Order {
		if goneReasons[g.Reasons[demo]] {
			delete(g.Reasons, demo)
			pruned++
			continue
		}
		keptDemos = append(keptDemos, demo)
	}
	g.Order = keptDemos
	if err := g.write(root); err != nil {
		return err
	}
	// pruning is a deliberate shrink of the golden budget — record it here,
	// otherwise the drill's own verify step reports the improvement as an
	// UNEXPECTED failure (seen on the first cross-tag drill: 147 -> 90)
	if b, ok := l.Budgets["golden.exempt-demos"]; ok {
		b.Max = len(g.Order)
	}
	if err := l.write(root); err != nil {
		return err
	}
	fmt.Printf("ledger dissolved: removed %d auto-dissolve entries recorded before %s "+
		"(%d golden demo exemptions pruned); re-run the gates and re-record whatever legitimately survives\n",
		len(gone), pin, pruned)
	for i, id := range gone {
		if i >= 10 {
			fmt.Printf("  … +%d more\n", len(gone)-10)
			break
		}
		fmt.Printf("  - %s\n", id)
	}
	return nil
}

// write emits JSON.stringify(golden, null, 1) + "\n" — indent 1, as the JS
// wrote it, so dissolving does not reformat the whole file.
func (g *goldenExemptions) write(root string) error {
	examples := jsonObj{}
	for _, demo := range g.Order {
		examples = examples.add(demo, jsonObj{}.add("reason", g.Reasons[demo]))
	}
	out := jsonObj{}.add("examples", examples)
	return os.WriteFile(filepath.Join(root, goldenExPath),
		[]byte(marshalJSStep(out, "", " ")+"\n"), 0o644)
}

// ---------------------------------------------------------------- render

func ledgerRender(root string) error {
	l, err := readLedger(root)
	if err != nil {
		return err
	}
	groups := map[string][]string{}
	for _, id := range l.EntryOrder {
		c := l.Entries[id].Class
		groups[c] = append(groups[c], id)
	}
	section := func(title, key, blurb string) string {
		rows := append([]string(nil), groups[key]...)
		// Byte order, where the JS used localeCompare. Matching ICU collation
		// would mean golang.org/x/text, and this runner is deliberately
		// stdlib-only. The rendered file is not compared against anything
		// (it is not in `reproducible`'s set and no gate reads it), so the
		// one-time reorder costs nothing and byte order is stable forever.
		sort.Strings(rows)
		lines := []string{
			fmt.Sprintf("## %s (%d)", title, len(rows)), "", blurb, "",
			"| Id | Reason | Recorded at |", "|---|---|---|",
		}
		for _, id := range rows {
			e := l.Entries[id]
			lines = append(lines, fmt.Sprintf("| `%s` | %s | %s |",
				id, strings.ReplaceAll(e.Reason, "|", `\|`), e.Recorded))
		}
		return strings.Join(append(lines, ""), "\n")
	}
	md := []string{
		"# EXEMPTIONS — the recorded-difference ledger",
		"",
		"<!-- GENERATED from gates/ledger.json by `pipeline ledger --render`.",
		"     Do not edit by hand: the `ledger` gate checks the JSON, not",
		"     this file, and the next render will overwrite whatever you wrote. -->",
		"",
		fmt.Sprintf("Pin: `%s` · %d exemptions · %d budgets", l.Pin, len(l.EntryOrder), len(l.BudgetOrder)),
		"",
		`Every "known difference, accepted for a reason" lives here, and every entry`,
		"declares **how it ends**. The `ledger` gate keeps this list in lockstep",
		"with the sources in both directions: a new exemption with no entry fails,",
		"an entry whose source flag vanished fails.",
		"",
		section("Permanent", "permanent",
			"Real engine or by-design differences. These do not dissolve; upstream would have to change."),
		section("Auto-dissolve on re-pin", "auto-dissolve",
			"Deploy lag, SSR-frame lag and other pin-relative differences. `make upstream` "+
				"DELETES every one of these after a re-pin and lets the gates re-earn them — "+
				"nobody reviews this section by hand."),
		section("Debt", "debt",
			"Accepted for now, tracked to zero. Governed by the budgets below."),
		"## Budgets",
		"",
		"A budget may only shrink. Growing fails the `ledger` gate; shrinking without",
		"re-recording also fails, so slack cannot be silently re-spent.",
		"",
		"| Metric | Max | Target | Reason |", "|---|---|---|---|",
	}
	for _, n := range l.BudgetOrder {
		b := l.Budgets[n]
		md = append(md, fmt.Sprintf("| `%s` | %d | %d | %s |",
			n, b.Max, b.Target, strings.ReplaceAll(b.Reason, "|", `\|`)))
	}
	md = append(md, "", "## Work items", "",
		"Not cross-checked — these track work, not accepted differences.", "")
	for _, n := range l.Notes {
		md = append(md, "- [ ] "+n)
	}
	md = append(md, "")
	if err := os.WriteFile(filepath.Join(root, renderedPath),
		[]byte(strings.Join(md, "\n")), 0o644); err != nil {
		return err
	}
	fmt.Printf("rendered %s from %s\n", renderedPath, ledgerPath)
	return nil
}

// runLedger is the writing half, as a subcommand: these mutate the tree and
// assert nothing, so they are not tests. --verify is TestLedger.
func runLedger(args []string) int {
	root, err := os.Getwd()
	if err != nil {
		fmt.Fprintln(os.Stderr, "pipeline:", err)
		return 1
	}
	var fn func(string) error
	switch {
	case has(args, "--record"):
		fn = ledgerRecord
	case has(args, "--render"):
		fn = ledgerRender
	case has(args, "--dissolve"):
		fn = ledgerDissolve
	case has(args, "--verify"):
		fmt.Fprintln(os.Stderr, "the ledger GATE is a Go test: go test -C pipeline -run '^TestLedger$'\n"+
			"this subcommand only writes: pipeline ledger --record|--render|--dissolve")
		return 2
	default:
		fmt.Fprintln(os.Stderr, "usage: pipeline ledger --record|--render|--dissolve")
		return 2
	}
	if err := fn(root); err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 1
	}
	return 0
}
