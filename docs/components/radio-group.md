---
title: "Radio Group"
description: "A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time."
---

# Radio Group

A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/radio-group) · [api](https://www.radix-ui.com/docs/primitives/components/radio-group#api-reference)</p>

<iframe class="demo" src="/demos/radio-group-demo.html" title="radio-group-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/radio-group.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/radio-group.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/radio-group.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/radio-group.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/radio-group.js"></script>
```

**Copy the markup from                                    into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `role="radiogroup"` root; items are `role="radio"` with `aria-checked` + `data-state`, the checked one carries the `radio-group-indicator` mounted from `<template data-for="radio-group-indicator">`; an item's value is its `value` / `data-value` attribute or id. Keys: arrows / Home / End move focus over enabled items (rtl-aware); with nothing checked an arrow also checks its target (radix).

The root dispatches `shadless:change` (`detail: { value, item }`), bubbling, after the state change, whichever path caused it.

Forms: a `name` attribute on the root submits the checked item's value; `form.reset()` restores the initial state.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/radio-group.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
radio-group
├── radio-group-item
└── radio-group-item
```

## Description

Radio group items with a description using the `Field` component.

<iframe class="demo" src="/demos/radio-group-description.html" title="radio-group-description" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Choice Card

Use `FieldLabel` to wrap the entire `Field` for a clickable card-style selection.

<iframe class="demo" src="/demos/radio-group-choice-card.html" title="radio-group-choice-card" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Fieldset

Use `FieldSet` and `FieldLegend` to group radio items with a label and description.

<iframe class="demo" src="/demos/radio-group-fieldset.html" title="radio-group-fieldset" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

Use the `disabled` prop on `RadioGroupItem` to disable individual items.

<iframe class="demo" src="/demos/radio-group-disabled.html" title="radio-group-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Invalid

Use `aria-invalid` on `RadioGroupItem` and `data-invalid` on `Field` to show validation errors.

<iframe class="demo" src="/demos/radio-group-invalid.html" title="radio-group-invalid" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/radio-group-rtl.html" title="radio-group-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="radio-group"` |
| `data-slot="radio-group-item"` |
| `data-slot="radio-group-indicator"` |

**Runtime:** `role="radiogroup"` root; items are `role="radio"` with `aria-checked` + `data-state`, the checked one carries the `radio-group-indicator` mounted from `<template data-for="radio-group-indicator">`; an item's value is its `value` / `data-value` attribute or id. Keys: arrows / Home / End move focus over enabled items (rtl-aware); with nothing checked an arrow also checks its target (radix). The root dispatches `shadless:change` (`detail: { value, item }`). Forms: a `name` attribute on the root submits the checked item's value. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix UI Radio Group](https://www.radix-ui.com/docs/primitives/components/radio-group#api-reference) documentation.
