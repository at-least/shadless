---
title: "Label"
description: "Renders an accessible label associated with controls."
---

# Label

Renders an accessible label associated with controls.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/label) · [api](https://www.radix-ui.com/docs/primitives/components/label#api-reference)</p>

<iframe class="demo" src="/demos/label-demo.html" title="label-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

::: tip
For form fields, use the [Field](/components/field) component which
includes built-in label, description, and error handling.
:::

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/label.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/label.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/label.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
```

**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/label.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Label in Field

For form fields, use the [Field](/components/field) component which
includes built-in `FieldLabel`, `FieldDescription`, and `FieldError` components.

```tsx
<Field>
  <FieldLabel htmlFor="email">Your email address</FieldLabel>
  <Input id="email" />
</Field>
```

<iframe class="demo" src="/demos/field-demo.html" title="field-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/label-rtl.html" title="label-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="label"` |

See the [Radix UI Label](https://www.radix-ui.com/docs/primitives/components/label#api-reference) documentation for more information.
