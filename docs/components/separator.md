---
title: "Separator"
description: "Visually or semantically separates content."
---

# Separator

Visually or semantically separates content.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/separator) · [api](https://www.radix-ui.com/docs/primitives/components/separator#api-reference)</p>

<iframe class="demo" src="/demos/separator-demo.html" title="separator-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/separator.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/separator.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/separator.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
```

**Copy the markup from                                  into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/separator.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Vertical

Use `orientation="vertical"` for a vertical separator.

<iframe class="demo" src="/demos/separator-vertical.html" title="separator-vertical" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Menu

Vertical separators between menu items with descriptions.

<iframe class="demo" src="/demos/separator-menu.html" title="separator-menu" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## List

Horizontal separators between list items.

<iframe class="demo" src="/demos/separator-list.html" title="separator-list" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/separator-rtl.html" title="separator-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="separator"` |

See the [Radix UI Separator](https://www.radix-ui.com/docs/primitives/components/separator#api-reference) documentation.
