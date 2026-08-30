---
title: "Context Menu"
description: "Displays a menu of actions triggered by a right click."
---

# Context Menu

Displays a menu of actions triggered by a right click.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/context-menu) · [api](https://www.radix-ui.com/docs/primitives/components/context-menu#api-reference)</p>

<iframe class="demo" src="/demos/context-menu.html" title="context-menu-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/context-menu.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/context-menu.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/context-menu.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/context-menu.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/context-menu.js"></script>
```

**Copy the markup from                                     into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="context-menu-trigger" id="<k>-trigger" data-radixuigo-context-trigger="<k>">` | opens on right-clicking |
| `<template id="<k>-tpl">` | holds the `data-slot="context-menu-content"` subtree |
| `<… data-slot="context-menu-sub-trigger" id="<k>s0-trigger" data-radixuigo-menu-subtrigger="<k>s0">` | a sub menu inside a layer; its own `<template id="<k>s0-tpl">` |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/context-menu.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
context-menu
├── context-menu-trigger
└── context-menu-content
    ├── context-menu-group
    │   ├── context-menu-label
    │   ├── context-menu-item
    │   └── context-menu-item
    ├── context-menu-separator
    ├── context-menu-group
    │   ├── context-menu-label
    │   ├── context-menu-checkbox-item
    │   └── context-menu-checkbox-item
    ├── context-menu-separator
    ├── context-menu-group
    │   ├── context-menu-label
    │   └── context-menu-radio-group
    │       ├── context-menu-radio-item
    │       └── context-menu-radio-item
    └── context-menu-sub
        ├── context-menu-sub-trigger
        └── context-menu-sub-content
            └── context-menu-group
                ├── context-menu-item
                └── context-menu-item
```

## Basic

A simple context menu with a few actions.

<iframe class="demo" src="/demos/context-menu-basic.html" title="context-menu-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Submenu

Use `ContextMenuSub` to nest secondary actions.

<iframe class="demo" src="/demos/context-menu-submenu.html" title="context-menu-submenu" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Shortcuts

Add `ContextMenuShortcut` to show keyboard hints.

<iframe class="demo" src="/demos/context-menu-shortcuts.html" title="context-menu-shortcuts" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Groups

Group related actions and separate them with dividers.

<iframe class="demo" src="/demos/context-menu-groups.html" title="context-menu-groups" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Icons

Combine icons with labels for quick scanning.

<iframe class="demo" src="/demos/context-menu-icons.html" title="context-menu-icons" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Checkboxes

Use `ContextMenuCheckboxItem` for toggles.

<iframe class="demo" src="/demos/context-menu-checkboxes.html" title="context-menu-checkboxes" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Radio

Use `ContextMenuRadioItem` for exclusive choices.

<iframe class="demo" src="/demos/context-menu-radio.html" title="context-menu-radio" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Destructive

Use `variant="destructive"` to style the menu item as destructive.

<iframe class="demo" src="/demos/context-menu-destructive.html" title="context-menu-destructive" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/context-menu-rtl.html" title="context-menu-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="context-menu"` |
| `data-slot="context-menu-trigger"` |
| `data-slot="context-menu-group"` |
| `data-slot="context-menu-portal"` |
| `data-slot="context-menu-sub"` |
| `data-slot="context-menu-radio-group"` |
| `data-slot="context-menu-content"` |
| `data-slot="context-menu-item"` |
| `data-slot="context-menu-sub-trigger"` |
| `data-slot="context-menu-sub-content"` |
| `data-slot="context-menu-checkbox-item"` |
| `data-slot="context-menu-radio-item"` |
| `data-slot="context-menu-label"` |
| `data-slot="context-menu-separator"` |
| `data-slot="context-menu-shortcut"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI](https://www.radix-ui.com/docs/primitives/components/context-menu#api-reference) documentation for more information.
