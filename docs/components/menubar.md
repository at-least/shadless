---
title: "Menubar"
description: "A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands."
---

# Menubar

A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/menubar) · [api](https://www.radix-ui.com/docs/primitives/components/menubar#api-reference)</p>

<iframe class="demo" src="/demos/menubar-demo.html" title="menubar-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/menubar.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/menubar.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/menubar.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/menubar.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/menubar.js"></script>
```

**Copy the markup from                                into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="menubar-trigger" id="<k>-trigger" data-radixuigo-menu-trigger="<k>">` | opens on clicking |
| `<template id="<k>-tpl">` | holds the `data-slot="menubar-content"` subtree |
| `<… data-slot="menubar-sub-trigger" id="<k>s0-trigger" data-radixuigo-menu-subtrigger="<k>s0">` | a sub menu inside a layer; its own `<template id="<k>s0-tpl">` |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/menubar.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
menubar
├── menubar-menu
│   ├── menubar-trigger
│   └── menubar-content
│       ├── menubar-group
│       │   ├── menubar-label
│       │   ├── menubar-item
│       │   └── menubar-item
│       ├── menubar-separator
│       ├── menubar-group
│       │   ├── menubar-label
│       │   ├── menubar-checkbox-item
│       │   └── menubar-checkbox-item
│       ├── menubar-separator
│       ├── menubar-group
│       │   ├── menubar-label
│       │   └── menubar-radio-group
│       │       ├── menubar-radio-item
│       │       └── menubar-radio-item
│       └── menubar-sub
│           ├── menubar-sub-trigger
│           └── menubar-sub-content
│               └── menubar-group
│                   ├── menubar-label
│                   ├── menubar-item
│                   └── menubar-item
└── menubar-menu
    ├── menubar-trigger
    └── menubar-content
        └── menubar-group
            ├── menubar-label
            ├── menubar-item
            └── menubar-item
```

## Checkbox

Use `MenubarCheckboxItem` for toggleable options.

<iframe class="demo" src="/demos/menubar-checkbox.html" title="menubar-checkbox" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Radio

Use `MenubarRadioGroup` and `MenubarRadioItem` for single-select options.

<iframe class="demo" src="/demos/menubar-radio.html" title="menubar-radio" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Submenu

Use `MenubarSub`, `MenubarSubTrigger`, and `MenubarSubContent` for nested menus.

<iframe class="demo" src="/demos/menubar-submenu.html" title="menubar-submenu" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## With Icons

<iframe class="demo" src="/demos/menubar-icons.html" title="menubar-icons" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/menubar-rtl.html" title="menubar-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="menubar"` |
| `data-slot="menubar-menu"` |
| `data-slot="menubar-group"` |
| `data-slot="menubar-portal"` |
| `data-slot="menubar-radio-group"` |
| `data-slot="menubar-trigger"` |
| `data-slot="menubar-content"` |
| `data-slot="menubar-item"` |
| `data-slot="menubar-checkbox-item"` |
| `data-slot="menubar-radio-item"` |
| `data-slot="menubar-label"` |
| `data-slot="menubar-separator"` |
| `data-slot="menubar-shortcut"` |
| `data-slot="menubar-sub"` |
| `data-slot="menubar-sub-trigger"` |
| `data-slot="menubar-sub-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Menubar](https://www.radix-ui.com/docs/primitives/components/menubar#api-reference) documentation.
