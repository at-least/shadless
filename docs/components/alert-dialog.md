---
title: "Alert Dialog"
description: "A modal dialog that interrupts the user with important content and expects a response."
---

# Alert Dialog

A modal dialog that interrupts the user with important content and expects a response.

<p class="page-links">[doc](https://www.radix-ui.com/primitives/docs/components/alert-dialog) · [api](https://www.radix-ui.com/primitives/docs/components/alert-dialog#api-reference)</p>

<iframe class="demo" src="/demos/alert-dialog.html" title="alert-dialog-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/alert-dialog.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/alert-dialog.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/alert-dialog.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/alert-dialog.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/alert-dialog.js"></script>
```

**Copy the markup from                                     into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="alert-dialog-trigger" id="<k>-trigger">` | opens on clicking; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="alert-dialog-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/alert-dialog.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
alert-dialog
├── alert-dialog-trigger
└── alert-dialog-content
    ├── alert-dialog-header
    │   ├── alert-dialog-media
    │   ├── alert-dialog-title
    │   └── alert-dialog-description
    └── alert-dialog-footer
        ├── alert-dialog-cancel
        └── alert-dialog-action
```

## Basic

A basic alert dialog with a title, description, and cancel and continue buttons.

<iframe class="demo" src="/demos/alert-dialog-basic.html" title="alert-dialog-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Small

Use the `size="sm"` prop to make the alert dialog smaller.

<iframe class="demo" src="/demos/alert-dialog-small.html" title="alert-dialog-small" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Media

Use the `AlertDialogMedia` component to add a media element such as an icon or image to the alert dialog.

<iframe class="demo" src="/demos/alert-dialog-media.html" title="alert-dialog-media" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Small with Media

Use the `size="sm"` prop to make the alert dialog smaller and the `AlertDialogMedia` component to add a media element such as an icon or image to the alert dialog.

<iframe class="demo" src="/demos/alert-dialog-small-media.html" title="alert-dialog-small-media" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Destructive

Use the `AlertDialogAction` component to add a destructive action button to the alert dialog.

<iframe class="demo" src="/demos/alert-dialog-destructive.html" title="alert-dialog-destructive" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/alert-dialog-rtl.html" title="alert-dialog-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="alert-dialog"` |
| `data-slot="alert-dialog-trigger"` |
| `data-slot="alert-dialog-portal"` |
| `data-slot="alert-dialog-overlay"` |
| `data-slot="alert-dialog-content"` |
| `data-slot="alert-dialog-header"` |
| `data-slot="alert-dialog-footer"` |
| `data-slot="alert-dialog-media"` |
| `data-slot="alert-dialog-title"` |
| `data-slot="alert-dialog-description"` |
| `data-slot="alert-dialog-action"` |
| `data-slot="alert-dialog-cancel"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

### size

Use the `size` prop on the `AlertDialogContent` component to control the size of the alert dialog. It accepts the following values:

| Prop   | Type                | Default     |
| ------ | ------------------- | ----------- |
| `size` | `"default" \| "sm"` | `"default"` |

For more information about the other components and their props, see the [Radix UI documentation](https://www.radix-ui.com/primitives/docs/components/alert-dialog#api-reference).
