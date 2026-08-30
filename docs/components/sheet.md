---
title: "Sheet"
description: "Extends the Dialog component to display content that complements the main content of the screen."
---

# Sheet

Extends the Dialog component to display content that complements the main content of the screen.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/dialog) · [api](https://www.radix-ui.com/docs/primitives/components/dialog#api-reference)</p>

<iframe class="demo" src="/demos/sheet.html" title="sheet-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/sheet.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/sheet.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/sheet.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/sheet.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/sheet.js"></script>
```

**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="sheet-trigger" id="<k>-trigger">` | opens on clicking; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="sheet-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/sheet.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
sheet
├── sheet-trigger
└── sheet-content
    ├── sheet-header
    │   ├── sheet-title
    │   └── sheet-description
    └── sheet-footer
```

## Side

Use the `side` prop on `SheetContent` to set the edge of the screen where the sheet appears. Values are `top`, `right`, `bottom`, or `left`.

<iframe class="demo" src="/demos/sheet-side.html" title="sheet-side" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## No Close Button

Use `showCloseButton={false}` on `SheetContent` to hide the close button.

<iframe class="demo" src="/demos/sheet-no-close-button.html" title="sheet-no-close-button" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/sheet-rtl.html" title="sheet-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="sheet"` |
| `data-slot="sheet-trigger"` |
| `data-slot="sheet-close"` |
| `data-slot="sheet-portal"` |
| `data-slot="sheet-overlay"` |
| `data-slot="sheet-content"` |
| `data-slot="sheet-header"` |
| `data-slot="sheet-footer"` |
| `data-slot="sheet-title"` |
| `data-slot="sheet-description"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Dialog](https://www.radix-ui.com/docs/primitives/components/dialog#api-reference) documentation.
