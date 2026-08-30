---
title: "Aspect Ratio"
description: "Displays content within a desired ratio."
---

# Aspect Ratio

Displays content within a desired ratio.

<p class="page-links">[doc](https://www.radix-ui.com/primitives/docs/components/aspect-ratio) · [api](https://www.radix-ui.com/primitives/docs/components/aspect-ratio#api-reference)</p>

<iframe class="demo" src="/demos/aspect-ratio-demo.html" title="aspect-ratio-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless to your Tailwind v4 entry:**

```css
@import "shadless";
```

This component has no stylesheet of its own — its styling rides the core theme and utilities in `shadless`.

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/components/aspect-ratio.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
```

**Copy the markup from                                     into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/aspect-ratio.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Square

A square aspect ratio component using the `ratio={1 / 1}` prop. This is useful for displaying images in a square format.

<iframe class="demo" src="/demos/aspect-ratio-square.html" title="aspect-ratio-square" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Portrait

A portrait aspect ratio component using the `ratio={9 / 16}` prop. This is useful for displaying images in a portrait format.

<iframe class="demo" src="/demos/aspect-ratio-portrait.html" title="aspect-ratio-portrait" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/aspect-ratio-rtl.html" title="aspect-ratio-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="aspect-ratio"` |

### AspectRatio

The `AspectRatio` component displays content within a desired ratio.

| Prop        | Type     | Default | Required |
| ----------- | -------- | ------- | -------- |
| `ratio`     | `number` | -       | Yes      |
| `className` | `string` | -       | No       |

For more information, see the [Radix UI documentation](https://www.radix-ui.com/primitives/docs/components/aspect-ratio#api-reference).
