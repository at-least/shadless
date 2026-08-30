---
title: "Tabs"
description: "A set of layered sections of content—known as tab panels—that are displayed one at a time."
---

# Tabs

A set of layered sections of content—known as tab panels—that are displayed one at a time.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/tabs) · [api](https://www.radix-ui.com/docs/primitives/components/tabs#api-reference)</p>

<iframe class="demo" src="/demos/tabs.html" title="tabs-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/tabs.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/tabs.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/tabs.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/tabs.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/tabs.js"></script>
```

**Copy the markup from                             into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<div data-slot="tabs">` with `data-slot="tabs-trigger" aria-controls="<panel-id>"` and `data-slot="tabs-content" id="<panel-id>"` | no template: every panel is in the markup, inactive ones `hidden`; the glue wires every root it finds |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get(rootEl)` → `activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`). `shadless.get` accepts an element or a selector and walks up from any element inside the instance.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/tabs.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
tabs
├── tabs-list
│   ├── tabs-trigger
│   └── tabs-trigger
├── tabs-content
└── tabs-content
```

## Line

Use the `variant="line"` prop on `TabsList` for a line style.

<iframe class="demo" src="/demos/tabs-line.html" title="tabs-line" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Vertical

Use `orientation="vertical"` for vertical tabs.

<iframe class="demo" src="/demos/tabs-vertical.html" title="tabs-vertical" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

<iframe class="demo" src="/demos/tabs-disabled.html" title="tabs-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Icons

<iframe class="demo" src="/demos/tabs-icons.html" title="tabs-icons" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/tabs-rtl.html" title="tabs-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="tabs"` |
| `data-slot="tabs-list"` |
| `data-slot="tabs-trigger"` |
| `data-slot="tabs-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`). Markup protocol: see Installation → Behavior protocol.

See the [Radix Tabs](https://www.radix-ui.com/docs/primitives/components/tabs#api-reference) documentation.
