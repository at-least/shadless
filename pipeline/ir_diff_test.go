package main

// The semantic-diff algorithm the re-pin drill relies on to decide which
// components moved upstream, tested directly since it is wired into no
// automated gate — only the CLI driver (`pipeline ir-diff`, via upstream.go).

import (
	"encoding/json"
	"fmt"
	"testing"
)

func mustIrComponent(t *testing.T, src string) irComponent {
	t.Helper()
	var c irComponent
	if err := json.Unmarshal([]byte(src), &c); err != nil {
		t.Fatalf("bad fixture json: %v", err)
	}
	return c
}

// wantChange is the subset of irChange a case cares about; unset string
// fields compare against "" and unset slices against nil-or-empty.
type wantChange struct {
	what, slot, table, axis, value, from, to string
	added, removed                           []string
}

func sameStrSlice(a, b []string) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func (w wantChange) matches(c irChange) bool {
	return w.what == c.What && w.slot == c.Slot && w.table == c.Table && w.axis == c.Axis &&
		w.value == c.Value && w.from == c.From && w.to == c.To &&
		sameStrSlice(w.added, c.Added) && sameStrSlice(w.removed, c.Removed)
}

// diffIr end to end: a component added, a component removed, and a component
// that changed in every way the algorithm tracks — tier, a slot added, a slot
// removed, a class token added/removed on a shared slot, a cva table added, a
// cva table removed, a cva axis added, a cva axis removed, a cva value added
// alongside a changed class on a shared value, and a changed default.
func TestUnitDiffIr(t *testing.T) {
	before := irSet{
		"widget": mustIrComponent(t, `{
			"tier": "core",
			"components": [{"elements": [
				{"slot": "root", "classes": ["flex", "gap-2"]},
				{"slot": "icon", "classes": ["size-4"]}
			]}],
			"cva": {
				"variant": {
					"base": "inline-flex",
					"variants": {
						"size": {"sm": "h-8", "lg": "h-10"},
						"tone": {"solid": "bg-black"}
					},
					"defaults": {"size": "sm"}
				},
				"color": {"base": "", "variants": {"c": {"red": "text-red"}}, "defaults": {}}
			}
		}`),
		"gone": mustIrComponent(t, `{"tier": "core", "components": [], "cva": {}}`),
	}
	after := irSet{
		"widget": mustIrComponent(t, `{
			"tier": "extended",
			"components": [{"elements": [
				{"slot": "root", "classes": ["flex", "px-2"]},
				{"slot": "label", "classes": ["text-sm"]}
			]}],
			"cva": {
				"variant": {
					"base": "inline-flex",
					"variants": {
						"size": {"sm": "h-9", "lg": "h-10", "xl": "h-12"},
						"state": {"active": "opacity-100"}
					},
					"defaults": {"size": "lg"}
				},
				"shape": {"base": "", "variants": {"s": {"round": "rounded"}}, "defaults": {}}
			}
		}`),
		"fresh": mustIrComponent(t, `{"tier": "core", "components": [], "cva": {}}`),
	}

	order, components := diffIr(before, after)

	if got := fmt.Sprint(order); got != "[fresh gone widget]" {
		t.Fatalf("order = %v, want sorted [fresh gone widget]", order)
	}
	if e := components["fresh"]; e.Kind != "added" || e.Tier != "core" {
		t.Errorf("fresh = %+v, want added/core", e)
	}
	if e := components["gone"]; e.Kind != "removed" || e.Tier != "core" {
		t.Errorf("gone = %+v, want removed/core", e)
	}

	w := components["widget"]
	if w.Kind != "changed" {
		t.Fatalf("widget kind = %q, want changed", w.Kind)
	}
	want := []wantChange{
		{what: "tier", from: "core", to: "extended"},
		{what: "slot-added", slot: "label"},
		{what: "slot-removed", slot: "icon"},
		{what: "classes", slot: "root", added: []string{"px-2"}, removed: []string{"gap-2"}},
		{what: "cva-added", table: "shape"},
		{what: "cva-removed", table: "color"},
		{what: "cva-axis-added", table: "variant", axis: "state", added: []string{"active"}},
		{what: "cva-axis-removed", table: "variant", axis: "tone"},
		{what: "cva-values", table: "variant", axis: "size", added: []string{"xl"}},
		{what: "cva-value-classes", table: "variant", axis: "size", value: "sm"},
		{what: "cva-default", table: "variant", axis: "size", from: "sm", to: "lg"},
	}
	if len(w.Changes) != len(want) {
		t.Fatalf("widget.Changes has %d entries, want %d:\n%+v", len(w.Changes), len(want), w.Changes)
	}
	for i, c := range w.Changes {
		if !want[i].matches(c) {
			t.Errorf("change[%d] = %+v, want %+v", i, c, want[i])
		}
	}
}

// setDiff must follow INSERTION order, not sorted order — proven with two
// multi-element diffs where sorted and insertion order disagree.
func TestUnitSetDiffFollowsInsertionOrder(t *testing.T) {
	a, b := newOrderedSet(), newOrderedSet()
	for _, x := range []string{"shared", "z", "q"} {
		a.add(x)
	}
	for _, x := range []string{"b", "shared", "a"} {
		b.add(x)
	}
	added, removed := setDiff(a, b)
	if fmt.Sprint(added) != "[b a]" {
		t.Errorf("added = %v, want insertion order [b a] (sorted would be [a b])", added)
	}
	if fmt.Sprint(removed) != "[z q]" {
		t.Errorf("removed = %v, want insertion order [z q] (sorted would be [q z])", removed)
	}
}

// jsString mirrors JS String(): undefined for a missing (nil) cva value, the
// bare integer for a whole-number float (JSON's only number representation),
// and the decimal form for a fractional one.
func TestUnitJsString(t *testing.T) {
	cases := []struct {
		in   any
		want string
	}{
		{nil, "undefined"},
		{"solid", "solid"},
		{float64(4), "4"},
		{float64(3.5), "3.5"},
	}
	for _, c := range cases {
		if got := jsString(c.in); got != c.want {
			t.Errorf("jsString(%#v) = %q, want %q", c.in, got, c.want)
		}
	}
}
