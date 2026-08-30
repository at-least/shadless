---
title: "Select"
description: "Displays a list of options for the user to pick from—triggered by a button."
---

# Select

Displays a list of options for the user to pick from—triggered by a button.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/select) · [api](https://www.radix-ui.com/docs/primitives/components/select#api-reference)</p>

<iframe class="demo" src="/demos/select.html" title="select-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/select.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/select.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/select.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/select.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/select.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<button data-slot="select-trigger" id="<k>-trigger">` | opens on click / Enter / Space / arrows; the `data-slot="select-value"` child shows the selection |
| `<template id="<k>-tpl">` | holds the `data-slot="select-content"` listbox subtree |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`, `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger also dispatches `shadless:change` (`detail: { value, label, item }`). An option's value is its `value` / `data-value` attribute or id — React's value prop never reaches the DOM, so add `data-value` to options whose value differs from their label. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/select.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
select
├── select-trigger
│   └── select-value
└── select-content
    ├── select-group
    │   ├── select-label
    │   ├── select-item
    │   └── select-item
    ├── select-separator
    └── select-group
        ├── select-label
        ├── select-item
        └── select-item
```

## Align Item With Trigger

Use the `position` prop on `SelectContent` to control alignment. When `position="item-aligned"` (default), the popup positions so the selected item appears over the trigger. When `position="popper"`, the popup aligns to the trigger edge.

<iframe class="demo" src="/demos/select-align-item.html" title="select-align-item" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Groups

Use `SelectGroup`, `SelectLabel`, and `SelectSeparator` to organize items.

<iframe class="demo" src="/demos/select-groups.html" title="select-groups" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Scrollable

A select with many items that scrolls.

<iframe class="demo" src="/demos/select-scrollable.html" title="select-scrollable" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

<iframe class="demo" src="/demos/select-disabled.html" title="select-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Invalid

Add the `data-invalid` attribute to the `Field` component and the `aria-invalid` attribute to the `SelectTrigger` component to show an error state.

```tsx showLineNumbers /data-invalid/ /aria-invalid/
<Field data-invalid>
  <FieldLabel>Fruit</FieldLabel>
  <SelectTrigger aria-invalid>
    <SelectValue />
  </SelectTrigger>
</Field>
```

<iframe class="demo" src="/demos/select-invalid.html" title="select-invalid" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/select-rtl.html" title="select-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="select"` |
| `data-slot="select-group"` |
| `data-slot="select-value"` |
| `data-slot="select-trigger"` |
| `data-slot="select-content"` |
| `data-slot="select-label"` |
| `data-slot="select-item"` |
| `data-slot="select-separator"` |
| `data-slot="select-scroll-up-button"` |
| `data-slot="select-scroll-down-button"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`, `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger dispatches `shadless:change` (`detail: { value, label, item }`); a `name` attribute submits the selected value; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Select](https://www.radix-ui.com/docs/primitives/components/select#api-reference) documentation.
