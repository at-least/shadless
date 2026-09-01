package main

import "testing"

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
