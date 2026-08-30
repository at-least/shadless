---
title: "Textarea"
description: "Displays a form textarea or a component that looks like a textarea."
---

# Textarea

Displays a form textarea or a component that looks like a textarea.

<iframe class="demo" src="/demos/textarea-demo.html" title="textarea-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/textarea.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/textarea.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/textarea.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                 into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/textarea.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Field

Use `Field`, `FieldLabel`, and `FieldDescription` to create a textarea with a label and description.

<iframe class="demo" src="/demos/textarea-field.html" title="textarea-field" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

Use the `disabled` prop to disable the textarea. To style the disabled state, add the `data-disabled` attribute to the `Field` component.

<iframe class="demo" src="/demos/textarea-disabled.html" title="textarea-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Invalid

Use the `aria-invalid` prop to mark the textarea as invalid. To style the invalid state, add the `data-invalid` attribute to the `Field` component.

<iframe class="demo" src="/demos/textarea-invalid.html" title="textarea-invalid" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Button

Pair with `Button` to create a textarea with a submit button.

<iframe class="demo" src="/demos/textarea-button.html" title="textarea-button" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/textarea-rtl.html" title="textarea-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>
