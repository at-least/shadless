---
title: "Popover"
description: "Displays rich content in a portal, triggered by a button."
---

# Popover

Displays rich content in a portal, triggered by a button.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/popover) · [api](https://www.radix-ui.com/docs/primitives/components/popover#api-reference)</p>

<iframe class="demo" src="/demos/popover.html" title="popover-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/popover.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/popover.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/popover.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/popover.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/popover.js"></script>
```

**Copy the markup from                                into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="popover-trigger" id="<k>-trigger">` | opens on clicking; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="popover-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/popover.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
popover
├── popover-trigger
└── popover-content
```

## Basic

A simple popover with a header, title, and description.

<iframe class="demo" src="/demos/popover-basic.html" title="popover-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Align

Use the `align` prop on `PopoverContent` to control the horizontal alignment.

<iframe class="demo" src="/demos/popover-alignments.html" title="popover-alignments" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## With Form

A popover with form fields inside.

<iframe class="demo" src="/demos/popover-form.html" title="popover-form" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/popover-rtl.html" title="popover-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="popover"` |
| `data-slot="popover-trigger"` |
| `data-slot="popover-content"` |
| `data-slot="popover-anchor"` |
| `data-slot="popover-header"` |
| `data-slot="popover-title"` |
| `data-slot="popover-description"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Popover](https://www.radix-ui.com/docs/primitives/components/popover#api-reference) documentation.
