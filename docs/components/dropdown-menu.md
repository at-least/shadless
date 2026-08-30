---
title: "Dropdown Menu"
description: "Displays a menu to the user — such as a set of actions or functions — triggered by a button."
---

# Dropdown Menu

Displays a menu to the user — such as a set of actions or functions — triggered by a button.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/dropdown-menu) · [api](https://www.radix-ui.com/docs/primitives/components/dropdown-menu#api-reference)</p>

<iframe class="demo" src="/demos/dropdown-menu.html" title="dropdown-menu-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/dropdown-menu.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/dropdown-menu.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/dropdown-menu.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/dropdown-menu.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/dropdown-menu.js"></script>
```

**Copy the markup from                                      into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="dropdown-menu-trigger" id="<k>-trigger" data-radixuigo-menu-trigger="<k>">` | opens on clicking |
| `<template id="<k>-tpl">` | holds the `data-slot="dropdown-menu-content"` subtree |
| `<… data-slot="dropdown-menu-sub-trigger" id="<k>s0-trigger" data-radixuigo-menu-subtrigger="<k>s0">` | a sub menu inside a layer; its own `<template id="<k>s0-tpl">` |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/dropdown-menu.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
dropdown-menu
├── dropdown-menu-trigger
└── dropdown-menu-content
    ├── dropdown-menu-group
    │   ├── dropdown-menu-label
    │   ├── dropdown-menu-item
    │   └── dropdown-menu-item
    ├── dropdown-menu-separator
    ├── dropdown-menu-group
    │   ├── dropdown-menu-label
    │   ├── dropdown-menu-checkbox-item
    │   └── dropdown-menu-checkbox-item
    ├── dropdown-menu-separator
    ├── dropdown-menu-group
    │   ├── dropdown-menu-label
    │   └── dropdown-menu-radio-group
    │       ├── dropdown-menu-radio-item
    │       └── dropdown-menu-radio-item
    └── dropdown-menu-sub
        ├── dropdown-menu-sub-trigger
        └── dropdown-menu-sub-content
            └── dropdown-menu-group
                ├── dropdown-menu-label
                ├── dropdown-menu-item
                └── dropdown-menu-item
```

## Basic

A basic dropdown menu with labels and separators.

<iframe class="demo" src="/demos/dropdown-menu-basic.html" title="dropdown-menu-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Submenu

Use `DropdownMenuSub` to nest secondary actions.

<iframe class="demo" src="/demos/dropdown-menu-submenu.html" title="dropdown-menu-submenu" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Shortcuts

Add `DropdownMenuShortcut` to show keyboard hints.

<iframe class="demo" src="/demos/dropdown-menu-shortcuts.html" title="dropdown-menu-shortcuts" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Icons

Combine icons with labels for quick scanning.

<iframe class="demo" src="/demos/dropdown-menu-icons.html" title="dropdown-menu-icons" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Checkboxes

Use `DropdownMenuCheckboxItem` for toggles.

<iframe class="demo" src="/demos/dropdown-menu-checkboxes.html" title="dropdown-menu-checkboxes" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Checkboxes Icons

Add icons to checkbox items.

<iframe class="demo" src="/demos/dropdown-menu-checkboxes-icons.html" title="dropdown-menu-checkboxes-icons" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Radio Group

Use `DropdownMenuRadioGroup` for exclusive choices.

<iframe class="demo" src="/demos/dropdown-menu-radio-group.html" title="dropdown-menu-radio-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Radio Icons

Show radio options with icons.

<iframe class="demo" src="/demos/dropdown-menu-radio-icons.html" title="dropdown-menu-radio-icons" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Destructive

Use `variant="destructive"` for irreversible actions.

<iframe class="demo" src="/demos/dropdown-menu-destructive.html" title="dropdown-menu-destructive" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Avatar

An account switcher dropdown triggered by an avatar.

<iframe class="demo" src="/demos/dropdown-menu-avatar.html" title="dropdown-menu-avatar" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Complex

A richer example combining groups, icons, and submenus.

<iframe class="demo" src="/demos/dropdown-menu-complex.html" title="dropdown-menu-complex" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/dropdown-menu-rtl.html" title="dropdown-menu-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="dropdown-menu"` |
| `data-slot="dropdown-menu-portal"` |
| `data-slot="dropdown-menu-trigger"` |
| `data-slot="dropdown-menu-content"` |
| `data-slot="dropdown-menu-group"` |
| `data-slot="dropdown-menu-item"` |
| `data-slot="dropdown-menu-checkbox-item"` |
| `data-slot="dropdown-menu-checkbox-item-indicator"` |
| `data-slot="dropdown-menu-radio-group"` |
| `data-slot="dropdown-menu-radio-item"` |
| `data-slot="dropdown-menu-radio-item-indicator"` |
| `data-slot="dropdown-menu-label"` |
| `data-slot="dropdown-menu-separator"` |
| `data-slot="dropdown-menu-shortcut"` |
| `data-slot="dropdown-menu-sub"` |
| `data-slot="dropdown-menu-sub-trigger"` |
| `data-slot="dropdown-menu-sub-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI documentation](https://www.radix-ui.com/docs/primitives/components/dropdown-menu) for the full API reference.
