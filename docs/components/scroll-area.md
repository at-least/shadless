---
title: "Scroll Area"
description: "Augments native scroll functionality for custom, cross-browser styling."
---

# Scroll Area

Augments native scroll functionality for custom, cross-browser styling.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/scroll-area) · [api](https://www.radix-ui.com/docs/primitives/components/scroll-area#api-reference)</p>

<iframe class="demo" src="/demos/scroll-area.html" title="scroll-area-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/scroll-area.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/scroll-area.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/scroll-area.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/scroll-area.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/scroll-area.js"></script>
```

**Copy the markup from                                    into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="scroll-area">` | no ids, no templates: the glue wires every root it finds |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/scroll-area.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
scroll-area
└── scroll-area-scrollbar
```

## Horizontal

Use `ScrollBar` with `orientation="horizontal"` for horizontal scrolling.

<iframe class="demo" src="/demos/scroll-area-horizontal-demo.html" title="scroll-area-horizontal-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/scroll-area-rtl.html" title="scroll-area-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="scroll-area"` |
| `data-slot="scroll-area-viewport"` |
| `data-slot="scroll-area-scrollbar"` |
| `data-slot="scroll-area-thumb"` |

**Runtime:** wired from `data-slot` alone — no handle, no events; see Installation → Behavior protocol.

See the [Radix UI Scroll Area](https://www.radix-ui.com/docs/primitives/components/scroll-area#api-reference) documentation.
