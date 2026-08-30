---
title: "Slider"
description: "An input where the user selects a value from within a given range."
---

# Slider

An input where the user selects a value from within a given range.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/slider) · [api](https://www.radix-ui.com/docs/primitives/components/slider#api-reference)</p>

<iframe class="demo" src="/demos/slider.html" title="slider-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/slider.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/slider.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/slider.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/slider.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/slider.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="slider">` | no ids, no templates: the glue wires every root it finds |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get(rootEl)` → `values()`, `setValue(value, index)`; the root dispatches `shadless:change` (`detail: { values }`, live) and `shadless:commit` (once per gesture). `shadless.get` accepts an element or a selector and walks up from any element inside the instance.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/slider.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Range

Use an array with two values for a range slider.

<iframe class="demo" src="/demos/slider-range.html" title="slider-range" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Multiple Thumbs

Use an array with multiple values for multiple thumbs.

<iframe class="demo" src="/demos/slider-multiple.html" title="slider-multiple" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Vertical

Use `orientation="vertical"` for a vertical slider.

<iframe class="demo" src="/demos/slider-vertical.html" title="slider-vertical" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Controlled

<iframe class="demo" src="/demos/slider-controlled.html" title="slider-controlled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

Use the `disabled` prop to disable the slider.

<iframe class="demo" src="/demos/slider-disabled.html" title="slider-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/slider-rtl.html" title="slider-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="slider"` |
| `data-slot="slider-track"` |
| `data-slot="slider-range"` |
| `data-slot="slider-thumb"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `values()`, `setValue(value, index)`; the root dispatches `shadless:change` (live) and `shadless:commit` (once per gesture) with `detail: { values }`; a `name` attribute submits one input per thumb. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Slider](https://www.radix-ui.com/docs/primitives/components/slider#api-reference) documentation.
