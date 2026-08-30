---
title: "Checkbox"
description: "A control that allows the user to toggle between checked and not checked."
---

# Checkbox

A control that allows the user to toggle between checked and not checked.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/checkbox) · [api](https://www.radix-ui.com/docs/primitives/components/checkbox#api-reference)</p>

<iframe class="demo" src="/demos/checkbox-demo.html" title="checkbox-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/checkbox.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/checkbox.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/checkbox.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/checkbox.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/checkbox.js"></script>
```

**Copy the markup from                                 into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `role="checkbox"` root with `aria-checked` + `data-state="checked|unchecked"`; the `checkbox-indicator` mounts from a `<template data-for="checkbox-indicator">` while checked (radix Presence). Keys: Space / click toggles.

The root dispatches `shadless:change` (`detail: { checked }`), bubbling, after the state change, whichever path caused it.

Forms: a `name` attribute submits its `value` (default `on`) while checked; `form.reset()` restores the initial state.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/checkbox.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Checked State

Use `defaultChecked` for uncontrolled checkboxes, or `checked` and
`onCheckedChange` to control the state.



## Invalid State

Set `aria-invalid` on the checkbox and `data-invalid` on the field wrapper to
show the invalid styles.

<iframe class="demo" src="/demos/checkbox-invalid.html" title="checkbox-invalid" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Basic

Pair the checkbox with `Field` and `FieldLabel` for proper layout and labeling.

<iframe class="demo" src="/demos/checkbox-basic.html" title="checkbox-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Description

Use `FieldContent` and `FieldDescription` for helper text.

<iframe class="demo" src="/demos/checkbox-description.html" title="checkbox-description" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

Use the `disabled` prop to prevent interaction and add the `data-disabled` attribute to the `<Field>` component for disabled styles.

<iframe class="demo" src="/demos/checkbox-disabled.html" title="checkbox-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Group

Use multiple fields to create a checkbox list.

<iframe class="demo" src="/demos/checkbox-group.html" title="checkbox-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Table

<iframe class="demo" src="/demos/checkbox-table.html" title="checkbox-table" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/checkbox-rtl.html" title="checkbox-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="checkbox"` |
| `data-slot="checkbox-indicator"` |

**Runtime:** `role="checkbox"` root with `aria-checked` + `data-state="checked|unchecked"`; the `checkbox-indicator` mounts from a `<template data-for="checkbox-indicator">` while checked (radix Presence). Keys: Space / click toggles. The root dispatches `shadless:change` (`detail: { checked }`). Forms: a `name` attribute submits its `value` (default `on`) while checked. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix UI](https://www.radix-ui.com/docs/primitives/components/checkbox#api-reference) documentation for more information.
