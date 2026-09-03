package main

// The kernel-tier behavior protocol tables + docs templates, ported from
// tools/fixture-families.mjs. ONE table: example-fixture (still JS, Wave 3)
// generates what ships; docs-build tells the consumer how to author it.
//
// protocolMdx/trivialMdx/apiReferenceMdx must stay byte-identical to the JS
// originals — they land verbatim in built pages and docs-fidelity compares
// fences. Pinned by TestUnitFixtureFamiliesGolden.

import (
	"fmt"
	"strings"
)

type familyEnt struct {
	kind string
	open string
	attr string
	js   string
}

var family = map[string]familyEnt{
	"alert-dialog":    {kind: "dialog", js: "alert-dialog"},
	"dialog":          {kind: "dialog", js: "dialog"},
	"sheet":           {kind: "dialog", js: "sheet"},
	"popover":         {kind: "portal", open: "click", js: "popover"},
	"tooltip":         {kind: "portal", open: "hover", js: "tooltip"},
	"hover-card":      {kind: "portal", open: "hover", js: "hover-card"},
	"tabs":            {kind: "inline", js: "tabs"},
	"slider":          {kind: "none", js: "slider"},
	"scroll-area":     {kind: "none", js: "scroll-area"},
	"dropdown-menu":   {kind: "menu", open: "click", attr: "data-radixuigo-menu-trigger", js: "dropdown-menu"},
	"context-menu":    {kind: "menu", open: "contextmenu", attr: "data-radixuigo-context-trigger", js: "context-menu"},
	"menubar":         {kind: "menu", open: "click", attr: "data-radixuigo-menu-trigger", js: "menubar"},
	"select":          {kind: "select", js: "select"},
	"carousel":        {kind: "none", js: "carousel"},
	"navigation-menu": {kind: "nav", js: "navigation-menu"},
}

func gestureOf(open string) string {
	switch open {
	case "hover":
		return "hovering"
	case "contextmenu":
		return "right-clicking"
	}
	return "clicking"
}

func protocolMdx(comp string) string {
	f, ok := family[comp]
	if !ok {
		return ""
	}
	t, c := comp+"-trigger", comp+"-content"
	gesture := gestureOf(f.open)
	type row struct{ a, b string }
	var rows []row
	switch f.kind {
	case "dialog", "portal":
		rows = append(rows,
			row{"`<… data-slot=\"" + t + "\" id=\"<k>-trigger\">`", "opens on " + gesture + "; `<k>` names the instance"},
			row{"`<template id=\"<k>-portal\">`", "holds the overlay/content subtree (`data-slot=\"" + c + "\"` …) that the glue mounts into `<body>` while open"},
		)
	case "menu":
		rows = append(rows,
			row{"`<… data-slot=\"" + t + "\" id=\"<k>-trigger\" " + f.attr + "=\"<k>\">`", "opens on " + gesture},
			row{"`<template id=\"<k>-tpl\">`", "holds the `data-slot=\"" + c + "\"` subtree"},
			row{"`<… data-slot=\"" + comp + "-sub-trigger\" id=\"<k>s0-trigger\" data-radixuigo-menu-subtrigger=\"<k>s0\">`", "a sub menu inside a layer; its own `<template id=\"<k>s0-tpl\">`"},
		)
	case "select":
		rows = append(rows,
			row{"`<button data-slot=\"" + t + "\" id=\"<k>-trigger\">`", "opens on click / Enter / Space / arrows; the `data-slot=\"select-value\"` child shows the selection"},
			row{"`<template id=\"<k>-tpl\">`", "holds the `data-slot=\"" + c + "\"` listbox subtree"},
		)
	case "nav":
		rows = append(rows,
			row{"`<… data-slot=\"" + t + "\" id=\"<k>-trigger\" data-radixuigo-nav-trigger=\"<k>\">`", "opens on click"},
			row{"`<template id=\"<k>-content-tpl\">`", "holds the `data-slot=\"" + c + "\"` subtree; the glue creates the shared viewport inside the root"},
		)
	case "inline":
		rows = append(rows,
			row{"`<div data-slot=\"tabs\">` with `data-slot=\"tabs-trigger\" aria-controls=\"<panel-id>\"` and `data-slot=\"tabs-content\" id=\"<panel-id>\"`", "no template: every panel is in the markup, inactive ones `hidden`; the glue wires every root it finds"},
		)
	case "none":
		rows = append(rows,
			row{"`<… data-slot=\"" + comp + "\">`", "no ids, no templates: the glue wires every root it finds"},
		)
	}
	var api string
	switch {
	case f.kind == "inline":
		api = "`shadless.get(rootEl)` → `activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`)"
	case f.kind == "none" && comp == "slider":
		api = "`shadless.get(rootEl)` → `values()`, `setValue(value, index)`; the root dispatches `shadless:change` (`detail: { values }`, live) and `shadless:commit` (once per gesture)"
	case f.kind == "none" && comp == "carousel":
		api = "`shadless.get(rootEl)` → the embla api (`scrollNext()`, `scrollTo(i)`, `on(\"select\", …)`)"
	case f.kind == "none":
		api = ""
	default:
		api = "`shadless.get(\"#<k>-trigger\")` → `open()`, `close()`, `toggle()`, `isOpen()`"
		if f.kind == "select" {
			api += ", `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger also dispatches `shadless:change` (`detail: { value, label, item }`). An option's value is its `value` / `data-value` attribute or id — React's value prop never reaches the DOM, so add `data-value` to options whose value differs from their label"
		}
	}
	var b strings.Builder
	b.WriteString("\n**Behavior protocol**\n\n")
	b.WriteString("The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.\n\n")
	b.WriteString("| Markup | Meaning |\n| --- | --- |\n")
	for _, r := range rows {
		b.WriteString("| " + r.a + " | " + r.b + " |\n")
	}
	b.WriteString("\nContent that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.\n")
	if api != "" {
		b.WriteString("\n**From code:** " + api + ". `shadless.get` accepts an element or a selector and walks up from any element inside the instance.")
		if f.kind != "inline" && f.kind != "none" {
			b.WriteString(" The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.")
		}
		b.WriteString("\n")
	}
	return b.String()
}

type trivialEnt struct {
	state  string
	events string
	form   string
	keys   string
}

var trivial = map[string]trivialEnt{
	"checkbox": {
		state:  "`role=\"checkbox\"` root with `aria-checked` + `data-state=\"checked|unchecked\"`; the `checkbox-indicator` mounts from a `<template data-for=\"checkbox-indicator\">` while checked (radix Presence)",
		events: "the root dispatches `shadless:change` (`detail: { checked }`)",
		form:   "a `name` attribute submits its `value` (default `on`) while checked",
		keys:   "Space / click toggles",
	},
	"switch": {
		state:  "`role=\"switch\"` root with `aria-checked` + `data-state`; the `switch-thumb` mirrors `data-state`",
		events: "the root dispatches `shadless:change` (`detail: { checked }`)",
		form:   "a `name` attribute submits its `value` (default `on`) while checked",
		keys:   "Space / click toggles",
	},
	"toggle": {
		state:  "`aria-pressed` + `data-state=\"on|off\"` on the root",
		events: "the root dispatches `shadless:change` (`detail: { pressed }`)",
		keys:   "Space / click toggles",
	},
	"radio-group": {
		state:  "`role=\"radiogroup\"` root; items are `role=\"radio\"` with `aria-checked` + `data-state`, the checked one carries the `radio-group-indicator` mounted from `<template data-for=\"radio-group-indicator\">`; an item's value is its `value` / `data-value` attribute or id",
		events: "the root dispatches `shadless:change` (`detail: { value, item }`)",
		form:   "a `name` attribute on the root submits the checked item's value",
		keys:   "arrows / Home / End move focus over enabled items (rtl-aware); with nothing checked an arrow also checks its target (radix)",
	},
	"toggle-group": {
		state:  "`role=\"group\"` root; single mode items are `role=\"radio\"` with `aria-checked`, multiple mode items carry `aria-pressed`; `data-state=\"on|off\"` in both",
		events: "the root dispatches `shadless:change` (`detail: { value, item }` — `value` is the on item's value, or an array in multiple mode)",
		keys:   "arrows / Home / End move focus over enabled items (rtl-aware); Space / Enter / click selects",
	},
	"accordion": {
		state:  "root `data-type=\"single|multiple\"`; each `accordion-trigger` carries `aria-expanded` + `data-state=\"open|closed\"` and `aria-controls` → its `accordion-content`, which is `hidden` while closed (kept in the DOM — radix unmounts it)",
		events: "each trigger dispatches `shadless:open` / `shadless:close` (a sibling closed by single mode gets its own `close`)",
		keys:   "arrows / Home / End move focus between triggers (rtl-aware); Enter / Space / click toggles",
	},
	"collapsible": {
		state:  "`collapsible-trigger` carries `aria-expanded` + `data-state` and `aria-controls` → the `collapsible-content`, `hidden` while closed",
		events: "the trigger dispatches `shadless:open` / `shadless:close`",
		keys:   "Enter / Space / click toggles",
	},
	"avatar": {
		state: "`avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup",
	},
}

func upperFirst(s string) string {
	if s == "" {
		return s
	}
	return strings.ToUpper(s[:1]) + s[1:]
}

func trivialMdx(comp string) string {
	t, ok := trivial[comp]
	if !ok {
		return ""
	}
	var b strings.Builder
	b.WriteString("\n**Behavior**\n\n")
	b.WriteString("Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. " + t.state + ".")
	if t.keys != "" {
		b.WriteString(" Keys: " + t.keys + ".")
	}
	b.WriteString("\n")
	if t.events != "" {
		b.WriteString("\n" + upperFirst(t.events) + ", bubbling, after the state change, whichever path caused it.")
	}
	if t.form != "" {
		b.WriteString("\n\nForms: " + t.form + "; `form.reset()` restores the initial state.")
	}
	b.WriteString("\n")
	return b.String()
}

type cvaAxisRow struct {
	slot   string
	attr   string
	values []string
	def    string
}

// cvaAxisRows: cva variant axes flattened to (slot, attribute, values,
// default) rows, in the IR's declaration order. This is the only vanilla-JS
// equivalent a plain-HTML wrapper (shadcn's own component, not a Radix
// primitive) has for a React prop like `variant`/`size`: no events, no
// state, just a `data-*` attribute the shipped CSS switches on.
func cvaAxisRows(ir cssIrComponent) []cvaAxisRow {
	bySlot := cvaSlot(ir)
	var out []cvaAxisRow
	for _, varName := range ir.Cva.keys {
		entry, ok := bySlot[varName]
		if !ok || entry.slot == "" {
			continue
		}
		for _, axis := range entry.table.axisOrder {
			out = append(out, cvaAxisRow{
				slot:   entry.slot,
				attr:   axis,
				values: entry.table.valueOrder[axis],
				def:    entry.table.defaults[axis],
			})
		}
	}
	return out
}

// cvaAxisTableMdx: the one rendering of a component's cva axes, sourced from
// the IR's own value order and defaults. Every component that declares axes
// gets it — the family/trivial ones (tabs, toggle) too, which used to reach
// it only through the `leaked` branch and so shipped no axis documentation at
// all. It replaced a per-page table in the Usage section that spelled every
// axis's example value as `outline`, a value 16 of its 21 rows did not have.
func cvaAxisTableMdx(comp string, axes []cvaAxisRow) string {
	var b strings.Builder
	b.WriteString("Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — check `dist/css/" + comp + ".css` for any other `data-*` selector on these slots.\n\n")
	b.WriteString("| Slot | Attribute | Values | Default |\n| --- | --- | --- | --- |\n")
	for _, a := range axes {
		var vals []string
		for _, v := range a.values {
			vals = append(vals, "`"+v+"`")
		}
		b.WriteString("| `" + a.slot + "` | `data-" + a.attr + "` | " + strings.Join(vals, ", ") + " | `" + a.def + "` |\n")
	}
	return b.String()
}

func apiReferenceMdx(comp string, slots []string, axes []cvaAxisRow, tier string, leaked bool) string {
	t := trivial[comp]
	f := family[comp]
	var rows []string
	for _, s := range slots {
		rows = append(rows, "| `data-slot=\""+s+"\"` |")
	}
	slotTable := ""
	if len(rows) > 0 {
		slotTable = "\n| Slot |\n| --- |\n" + strings.Join(rows, "\n") + "\n"
	}
	runtime := ""
	if _, isTrivial := trivial[comp]; isTrivial {
		runtime = "\n**Runtime:** " + t.state + "."
		if t.keys != "" {
			runtime += " Keys: " + t.keys + "."
		}
		if t.events != "" {
			runtime += " " + upperFirst(t.events) + "."
		}
		if t.form != "" {
			runtime += " Forms: " + t.form + "."
		}
		runtime += " No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.\n"
	} else if _, isFamily := family[comp]; isFamily {
		var api string
		switch {
		case f.kind == "inline":
			api = "`activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`)"
		case comp == "slider":
			api = "`values()`, `setValue(value, index)`; the root dispatches `shadless:change` (live) and `shadless:commit` (once per gesture) with `detail: { values }`; a `name` attribute submits one input per thumb"
		case comp == "carousel":
			api = "the embla api (`scrollNext()`, `scrollTo(i)`, `on(\"select\", …)`)"
		case comp == "scroll-area":
			api = ""
		default:
			api = "`open()`, `close()`, `toggle()`, `isOpen()`"
			if f.kind == "select" {
				api += ", `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger dispatches `shadless:change` (`detail: { value, label, item }`); a `name` attribute submits the selected value"
			}
			api += "; the trigger dispatches `shadless:open` / `shadless:close`"
		}
		if api != "" {
			runtime = "\n**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → " + api + ". Markup protocol: see Installation → Behavior protocol.\n"
		} else {
			runtime = "\n**Runtime:** wired from `data-slot` alone — no handle, no events; see Installation → Behavior protocol.\n"
		}
	} else if leaked {
		var b strings.Builder
		b.WriteString("\n**Runtime:** ")
		if tier == "static" {
			b.WriteString("no JavaScript — this is markup + CSS. ")
		}
		if len(axes) > 0 {
			b.WriteString(cvaAxisTableMdx(comp, axes))
		} else {
			b.WriteString("No `cva`-declared variants. Check `dist/css/" + comp + ".css` for any `data-*` attribute this slot's styling depends on.\n")
		}
		if tier != "static" {
			b.WriteString("See Installation → Files this component needs for the JavaScript this component requires.\n")
		}
		runtime = b.String()
	}
	// The `leaked` branch already wrote it; the other two never did, so tabs
	// and toggle documented their variants nowhere.
	if !leaked && len(axes) > 0 {
		runtime += "\n" + cvaAxisTableMdx(comp, axes)
	}
	if slotTable == "" && runtime == "" {
		return ""
	}
	preamble := "\n**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.\n"
	if leaked {
		preamble = "\n**shadless surface** — every node is a `data-slot` attribute in the shipped markup.\n"
	}
	return preamble + slotTable + runtime
}

// sortedFamilyKeys / sortedTrivialKeys for deterministic golden dumps.
func sortedFamilyKeys() []string {
	var out []string
	for k := range family {
		out = append(out, k)
	}
	sortStrings(out)
	return out
}

func sortStrings(s []string) {
	for i := 1; i < len(s); i++ {
		for j := i; j > 0 && s[j] < s[j-1]; j-- {
			s[j], s[j-1] = s[j-1], s[j]
		}
	}
}

var _ = fmt.Sprintf // keep fmt for future use in this file
