package main

// ir-diff — slot-level semantic diff between two IR sets. Ported from
// gates/ir-diff.mjs.
//
// The upstream tsx diff is the wrong review surface for a re-pin: it shows
// React refactors, prop plumbing and comment changes that never reach the
// conversion. The IR is what the pipeline actually consumes, so a diff at that
// level answers the questions a re-pin review has:
//
//	which components appeared / vanished / changed tier
//	which slots appeared / vanished, which class lists changed
//	which cva axes / values / defaults changed
//
// and it is the routing signal gates/upstream.mjs uses to decide which overlay
// units and which gate failures are EXPECTED consequences of an upstream change
// versus regressions in our own pipeline.
//
//	pipeline ir-diff <git-ref>        committed IR at <ref> vs the working tree
//	pipeline ir-diff <dirA> <dirB>    two IR directories
//	pipeline ir-diff --json …         the same diff as JSON, for the drill
//
// This one is worth porting precisely because it reads the IR as DATA. It
// makes no judgement a converter or emitter rule already makes, so there is no
// second implementation of anything.

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
)

type irComponent struct {
	Tier       string `json:"tier"`
	Components []struct {
		Elements []struct {
			Slot    string   `json:"slot"`
			Classes []string `json:"classes"`
		} `json:"elements"`
	} `json:"components"`
	Cva map[string]struct {
		Base     any                       `json:"base"`
		Variants map[string]map[string]any `json:"variants"`
		Defaults map[string]any            `json:"defaults"`
	} `json:"cva"`
}

type irSet map[string]irComponent

func loadIrFromDir(dir string) irSet {
	out := irSet{}
	entries, err := os.ReadDir(dir)
	if err != nil {
		return out
	}
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		b, err := os.ReadFile(filepath.Join(dir, e.Name()))
		if err != nil {
			continue
		}
		var c irComponent
		if json.Unmarshal(b, &c) == nil {
			out[strings.TrimSuffix(e.Name(), ".json")] = c
		}
	}
	return out
}

func loadIrFromGit(root, ref string) irSet {
	out := irSet{}
	cmd := exec.Command("git", "ls-tree", "--name-only", ref, "generated/ir/")
	cmd.Dir = root
	b, err := cmd.Output()
	if err != nil {
		return out
	}
	for _, f := range strings.Split(string(b), "\n") {
		if !strings.HasSuffix(f, ".json") {
			continue
		}
		show := exec.Command("git", "show", ref+":"+f)
		show.Dir = root
		content, err := show.Output()
		if err != nil {
			continue
		}
		var c irComponent
		if json.Unmarshal(content, &c) == nil {
			name := f[strings.LastIndex(f, "/")+1:]
			out[strings.TrimSuffix(name, ".json")] = c
		}
	}
	return out
}

// orderedSet keeps insertion order, which is what the JS Set iteration gave
// and therefore what the added/removed lists print.
type orderedSet struct {
	order []string
	seen  map[string]bool
}

func newOrderedSet() *orderedSet { return &orderedSet{seen: map[string]bool{}} }
func (s *orderedSet) add(x string) {
	if !s.seen[x] {
		s.seen[x] = true
		s.order = append(s.order, x)
	}
}

func setDiff(a, b *orderedSet) (added, removed []string) {
	for _, x := range b.order {
		if !a.seen[x] {
			added = append(added, x)
		}
	}
	for _, x := range a.order {
		if !b.seen[x] {
			removed = append(removed, x)
		}
	}
	return
}

// slotsOf maps slot -> the union of class tokens declared on it.
func slotsOf(ir irComponent) (map[string]*orderedSet, []string) {
	m := map[string]*orderedSet{}
	var order []string
	for _, c := range ir.Components {
		for _, e := range c.Elements {
			if e.Slot == "" {
				continue
			}
			if _, ok := m[e.Slot]; !ok {
				m[e.Slot] = newOrderedSet()
				order = append(order, e.Slot)
			}
			for _, tok := range strings.Fields(strings.Join(e.Classes, " ")) {
				m[e.Slot].add(tok)
			}
		}
	}
	return m, order
}

// jsString mirrors JS String(): the comparison the predecessor made on cva
// values, which may legitimately be absent.
func jsString(v any) string {
	switch x := v.(type) {
	case nil:
		return "undefined"
	case string:
		return x
	case float64:
		if x == float64(int64(x)) {
			return fmt.Sprintf("%d", int64(x))
		}
		return fmt.Sprintf("%v", x)
	default:
		return fmt.Sprintf("%v", x)
	}
}

type irChange struct {
	What, Slot, Table, Axis, Value, From, To string
	Added, Removed                           []string
}

type irEntry struct {
	Kind    string // added | removed | changed
	Tier    string
	Changes []irChange
}

func sortedKeys[V any](m map[string]V) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

// diffIr returns only the components that changed.
func diffIr(before, after irSet) ([]string, map[string]irEntry) {
	names := map[string]bool{}
	for n := range before {
		names[n] = true
	}
	for n := range after {
		names[n] = true
	}
	var order []string
	components := map[string]irEntry{}
	for _, name := range sortedKeys(names) {
		a, okA := before[name]
		b, okB := after[name]
		if !okA {
			order = append(order, name)
			components[name] = irEntry{Kind: "added", Tier: b.Tier}
			continue
		}
		if !okB {
			order = append(order, name)
			components[name] = irEntry{Kind: "removed", Tier: a.Tier}
			continue
		}
		d := irEntry{Kind: "changed"}
		if a.Tier != b.Tier {
			d.Changes = append(d.Changes, irChange{What: "tier", From: a.Tier, To: b.Tier})
		}
		sa, orderA := slotsOf(a)
		sb, orderB := slotsOf(b)
		for _, s := range orderB {
			if _, ok := sa[s]; !ok {
				d.Changes = append(d.Changes, irChange{What: "slot-added", Slot: s})
			}
		}
		for _, s := range orderA {
			if _, ok := sb[s]; !ok {
				d.Changes = append(d.Changes, irChange{What: "slot-removed", Slot: s})
			}
		}
		for _, s := range orderA {
			if _, ok := sb[s]; !ok {
				continue
			}
			added, removed := setDiff(sa[s], sb[s])
			if len(added) > 0 || len(removed) > 0 {
				d.Changes = append(d.Changes, irChange{What: "classes", Slot: s, Added: added, Removed: removed})
			}
		}
		for _, tbl := range sortedKeys(b.Cva) {
			if _, ok := a.Cva[tbl]; !ok {
				d.Changes = append(d.Changes, irChange{What: "cva-added", Table: tbl})
			}
		}
		for _, tbl := range sortedKeys(a.Cva) {
			if _, ok := b.Cva[tbl]; !ok {
				d.Changes = append(d.Changes, irChange{What: "cva-removed", Table: tbl})
			}
		}
		for _, tbl := range sortedKeys(a.Cva) {
			cb, ok := b.Cva[tbl]
			if !ok {
				continue
			}
			ca := a.Cva[tbl]
			for _, ax := range sortedKeys(cb.Variants) {
				if _, ok := ca.Variants[ax]; !ok {
					d.Changes = append(d.Changes, irChange{What: "cva-axis-added", Table: tbl, Axis: ax,
						Added: sortedKeys(cb.Variants[ax])})
				}
			}
			for _, ax := range sortedKeys(ca.Variants) {
				if _, ok := cb.Variants[ax]; !ok {
					d.Changes = append(d.Changes, irChange{What: "cva-axis-removed", Table: tbl, Axis: ax})
				}
			}
			for _, ax := range sortedKeys(ca.Variants) {
				vb, ok := cb.Variants[ax]
				if !ok {
					continue
				}
				va := ca.Variants[ax]
				sA, sB := newOrderedSet(), newOrderedSet()
				for _, k := range sortedKeys(va) {
					sA.add(k)
				}
				for _, k := range sortedKeys(vb) {
					sB.add(k)
				}
				added, removed := setDiff(sA, sB)
				if len(added) > 0 || len(removed) > 0 {
					d.Changes = append(d.Changes, irChange{What: "cva-values", Table: tbl, Axis: ax,
						Added: added, Removed: removed})
				}
				for _, v := range sortedKeys(va) {
					bv, present := vb[v]
					if present && jsString(va[v]) != jsString(bv) {
						d.Changes = append(d.Changes, irChange{What: "cva-value-classes", Table: tbl, Axis: ax, Value: v})
					}
				}
			}
			axes := map[string]bool{}
			for k := range ca.Defaults {
				axes[k] = true
			}
			for k := range cb.Defaults {
				axes[k] = true
			}
			for _, ax := range sortedKeys(axes) {
				if jsString(ca.Defaults[ax]) != jsString(cb.Defaults[ax]) {
					d.Changes = append(d.Changes, irChange{What: "cva-default", Table: tbl, Axis: ax,
						From: jsString(ca.Defaults[ax]), To: jsString(cb.Defaults[ax])})
				}
			}
			if jsString(ca.Base) != jsString(cb.Base) {
				d.Changes = append(d.Changes, irChange{What: "cva-base", Table: tbl})
			}
		}
		if len(d.Changes) > 0 {
			order = append(order, name)
			components[name] = d
		}
	}
	return order, components
}

func signed(added, removed []string, limit int) string {
	var parts []string
	for _, x := range added {
		parts = append(parts, "+"+x)
	}
	for _, x := range removed {
		parts = append(parts, "-"+x)
	}
	if limit > 0 && len(parts) > limit {
		parts = parts[:limit]
	}
	return strings.Join(parts, " ")
}

func renderIrDiff(order []string, components map[string]irEntry) string {
	if len(order) == 0 {
		return "no semantic change in the IR"
	}
	var lines []string
	for _, name := range order {
		d := components[name]
		if d.Kind != "changed" {
			lines = append(lines, fmt.Sprintf("%-20s %s (tier %s)", name, strings.ToUpper(d.Kind), d.Tier))
			continue
		}
		lines = append(lines, name)
		for _, c := range d.Changes {
			var s string
			switch c.What {
			case "classes":
				s = fmt.Sprintf("classes[%s] +%d -%d: %s", c.Slot, len(c.Added), len(c.Removed), signed(c.Added, c.Removed, 8))
			case "cva-values":
				s = fmt.Sprintf("cva %s.%s: %s", c.Table, c.Axis, signed(c.Added, c.Removed, 0))
			case "cva-default":
				s = fmt.Sprintf("cva %s.%s default %s -> %s", c.Table, c.Axis, c.From, c.To)
			case "cva-value-classes":
				s = fmt.Sprintf("cva %s.%s=%s classes changed", c.Table, c.Axis, c.Value)
			case "tier":
				s = fmt.Sprintf("tier %s -> %s", c.From, c.To)
			default:
				tail := c.Slot
				if tail == "" {
					tail = c.Table
				}
				if c.Axis != "" {
					tail += "." + c.Axis
				}
				s = strings.TrimSpace(c.What + " " + tail)
			}
			lines = append(lines, "  "+s)
		}
	}
	return strings.Join(lines, "\n")
}

func runIrDiff(args []string) int {
	asJSON := has(args, "--json")
	var pos []string
	for _, a := range args {
		if !strings.HasPrefix(a, "--") {
			pos = append(pos, a)
		}
	}
	if len(pos) == 0 {
		fmt.Fprintln(os.Stderr, "usage: pipeline ir-diff <git-ref> | <dirA> <dirB> [--json]")
		return 2
	}
	wd, _ := os.Getwd()
	root, err := findRepoRoot(wd)
	if err != nil {
		fmt.Fprintln(os.Stderr, "ir-diff:", err)
		return 1
	}
	resolve := func(p string) string {
		if filepath.IsAbs(p) {
			return p
		}
		return filepath.Join(root, p)
	}
	var before irSet
	if len(pos) > 1 {
		before = loadIrFromDir(resolve(pos[0]))
	} else {
		before = loadIrFromGit(root, pos[0])
	}
	after := loadIrFromDir(resolve("generated/ir"))
	if len(pos) > 1 {
		after = loadIrFromDir(resolve(pos[1]))
	}
	order, components := diffIr(before, after)

	if !asJSON {
		fmt.Println(renderIrDiff(order, components))
		return 0
	}
	obj := jsonObj{}
	for _, name := range order {
		d := components[name]
		e := jsonObj{}.add("kind", d.Kind)
		if d.Kind != "changed" {
			e = e.add("tier", d.Tier)
		} else {
			changes := make([]any, len(d.Changes))
			for i, c := range d.Changes {
				co := jsonObj{}.add("what", c.What)
				for _, kv := range []struct{ k, v string }{
					{"slot", c.Slot}, {"table", c.Table}, {"axis", c.Axis},
					{"value", c.Value}, {"from", c.From}, {"to", c.To},
				} {
					if kv.v != "" {
						co = co.add(kv.k, kv.v)
					}
				}
				if c.Added != nil {
					co = co.add("added", c.Added)
				}
				if c.Removed != nil {
					co = co.add("removed", c.Removed)
				}
				changes[i] = co
			}
			e = e.add("changes", changes)
		}
		obj = obj.add(name, e)
	}
	fmt.Println(marshalJS(jsonObj{}.add("components", obj), ""))
	return 0
}
