---
title: "Tooltip"
description: "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."
---

# Tooltip

A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/tooltip) · [api](https://www.radix-ui.com/docs/primitives/components/tooltip#api-reference)</p>

<iframe class="demo" src="/demos/tooltip.html" title="tooltip-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/tooltip.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/tooltip.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/tooltip.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/tooltip.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/tooltip.js"></script>
```

**Copy the markup from                                into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="tooltip-trigger" id="<k>-trigger">` | opens on hovering; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="tooltip-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/tooltip.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
tooltip
├── tooltip-trigger
└── tooltip-content
```

## Side

Use the `side` prop to change the position of the tooltip.

<iframe class="demo" src="/demos/tooltip-sides.html" title="tooltip-sides" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## With Keyboard Shortcut

<iframe class="demo" src="/demos/tooltip-keyboard.html" title="tooltip-keyboard" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled Button

Show a tooltip on a disabled button by wrapping it with a span.

<iframe class="demo" src="/demos/tooltip-disabled.html" title="tooltip-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/tooltip-rtl.html" title="tooltip-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="tooltip-provider"` |
| `data-slot="tooltip"` |
| `data-slot="tooltip-trigger"` |
| `data-slot="tooltip-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix Tooltip](https://www.radix-ui.com/docs/primitives/components/tooltip#api-reference) documentation.
