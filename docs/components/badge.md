---
title: "Badge"
description: "Displays a badge or a component that looks like a badge."
---

# Badge

Displays a badge or a component that looks like a badge.

<iframe class="demo" src="/demos/badge-demo.html" title="badge-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/badge.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/badge.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/badge.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/badge.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
## Variants

Use the `variant` prop to change the variant of the badge.

<iframe class="demo" src="/demos/badge-variants.html" title="badge-variants" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## With Icon

You can render an icon inside the badge. Use `data-icon="inline-start"` to render the icon on the left and `data-icon="inline-end"` to render the icon on the right.

<iframe class="demo" src="/demos/badge-icon.html" title="badge-icon" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## With Spinner

You can render a spinner inside the badge. Remember to add the `data-icon="inline-start"` or `data-icon="inline-end"` prop to the spinner.

<iframe class="demo" src="/demos/badge-spinner.html" title="badge-spinner" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Link

Use the `asChild` prop to render a link as a badge.

<iframe class="demo" src="/demos/badge-link.html" title="badge-link" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Custom Colors

You can customize the colors of a badge by adding custom classes such as `bg-green-50 dark:bg-green-800` to the `Badge` component.

<iframe class="demo" src="/demos/badge-colors.html" title="badge-colors" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/badge-rtl.html" title="badge-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="badge"` |

### Badge

The `Badge` component displays a badge or a component that looks like a badge.

| Prop        | Type                                                                          | Default     |
| ----------- | ----------------------------------------------------------------------------- | ----------- |
| `variant`   | `"default" \| "secondary" \| "destructive" \| "outline" \| "ghost" \| "link"` | `"default"` |
| `className` | `string`                                                                      | -           |
