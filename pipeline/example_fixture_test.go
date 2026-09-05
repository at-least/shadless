package main

import (
	"strings"
	"testing"
)

// The id-stabilization half of example-fixture, pinned: learn maps radix
// auto ids to stable fixture ids (slot ids first, then derived ids for the
// rest, then dangling aria-labelledby references to the trigger), remap
// rewrites tokens, ensureContentId inserts an id only when the tag carries
// none after its data-slot attribute (the JS tempered pattern's exact
// semantics), triggerWithId annotates the closed-markup trigger.
func TestUnitExampleFixtureIds(t *testing.T) {
	idMap := map[string]string{}
	efLearn(`<div><button data-slot="dialog-content" id="radix-:r2:" aria-labelledby="radix-:r3:"><h2 data-slot="dialog-title" id="radix-:r5:">T</h2><span id="radix-:r6:">d</span></button></div>`,
		[]efSlotStable{{"dialog-content", "d1"}, {"dialog-title", "d1-title"}, {"dialog-description", "d1-desc"}}, idMap)
	want := map[string]string{
		"radix-:r2:": "d1",
		"radix-:r5:": "d1-title",
		"radix-:r6:": "d1-e0",
		"radix-:r3:": "d1-trigger",
	}
	for k, v := range want {
		if idMap[k] != v {
			t.Errorf("idMap[%s] = %q, want %q", k, idMap[k], v)
		}
	}
	got := efRemap(`<button id="radix-:r2:" aria-labelledby="radix-:r3:" data-x="radix-:r9:">`, idMap)
	wantRemap := `<button id="d1" aria-labelledby="d1-trigger" data-x="radix-:r9:">`
	if got != wantRemap {
		t.Errorf("remap:\n got %q\nwant %q", got, wantRemap)
	}

	// strip: radix-referencing attrs and aria-hidden marks go
	got = efStripRadixIds(`<div id="radix-:r1:" aria-controls="radix-:r2:" aria-hidden="true" data-aria-hidden="true" class="k">`)
	if got != `<div class="k">` {
		t.Errorf("strip: got %q", got)
	}

	// ensureContentId: no id after data-slot → insert; id present → keep
	got = efEnsureContentId(`<div data-slot="popover-content" class="x"><p>hi</p></div>`, "popover", "k0")
	if got != `<div data-slot="popover-content" class="x" id="k0"><p>hi</p></div>` {
		t.Errorf("ensureContentId insert: got %q", got)
	}
	got = efEnsureContentId(`<div data-slot="popover-content" class="x" id="already"><p>hi</p></div>`, "popover", "k0")
	if got != `<div data-slot="popover-content" class="x" id="already"><p>hi</p></div>` {
		t.Errorf("ensureContentId keep: got %q", got)
	}

	// triggerWithId: only when the tag carries no id at all
	got = efTriggerWithId(`<div><button data-slot="dialog-trigger" class="t">o</button></div>`, "dialog", "d1-trigger")
	if got != `<div><button data-slot="dialog-trigger" class="t" id="d1-trigger">o</button></div>` {
		t.Errorf("triggerWithId insert: got %q", got)
	}
	got = efTriggerWithId(`<div><button data-slot="dialog-trigger" id="own">o</button></div>`, "dialog", "d1-trigger")
	if got != `<div><button data-slot="dialog-trigger" id="own">o</button></div>` {
		t.Errorf("triggerWithId keep: got %q", got)
	}
}

// Two shapes the shipped pages proved the old code got wrong.
//
// radix renders `id` BEFORE `data-slot`; the old check looked only after
// the data-slot attribute, so a content tag that efRemap had already given
// id="k0" received a second id="k0" — every popover-family fixture page
// shipped one element with two id attributes.
//
// A select instance's stable prefix is "s0"; `s\d+$` (meant for "k0s1" →
// "k0") stripped it to "", and nine pages shipped aria-labelledby="-trigger".
func TestUnitExampleFixtureIdRegressions(t *testing.T) {
	in := `<div role="dialog" id="k0" data-slot="popover-content" class="x"><p>hi</p></div>`
	if got := efEnsureContentId(in, "popover", "k0"); got != in {
		t.Errorf("id before data-slot was not seen:\n%s", got)
	}
	in = `<div data-slot="popover-content" class="x"><p>hi</p></div>`
	if got := efEnsureContentId(in, "popover", "k0"); strings.Count(got, ` id="`) != 1 {
		t.Errorf("tag without id did not gain exactly one: %s", got)
	}
	for base, want := range map[string]string{"k0s1": "k0", "k0": "k0", "s0": "s0", "s12": "s12", "d1s0": "d1"} {
		if got := efReTrailingSub.ReplaceAllString(base, "$1"); got != want {
			t.Errorf("stripper(%q) = %q, want %q", base, got, want)
		}
	}
	idMap := map[string]string{}
	efLearn(`<div data-slot="select-content" id="radix-:r1:" aria-labelledby="radix-:r0:"></div>`, []efSlotStable{{"select-content", "s0"}}, idMap)
	if idMap["radix-:r0:"] != "s0-trigger" {
		t.Errorf("select layer's dangling labelledby → %q, want s0-trigger", idMap["radix-:r0:"])
	}
}
