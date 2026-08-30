---
title: "Input"
description: "A text input component for forms and user data entry with built-in styling and accessibility features."
---

# Input

A text input component for forms and user data entry with built-in styling and accessibility features.

<iframe class="demo" src="/demos/input-demo.html" title="input-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/input.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/input.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/input.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/input.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Basic

<iframe class="demo" src="/demos/input-basic.html" title="input-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Field

Use `Field`, `FieldLabel`, and `FieldDescription` to create an input with a
label and description.

<iframe class="demo" src="/demos/input-field.html" title="input-field" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Field Group

Use `FieldGroup` to show multiple `Field` blocks and to build forms.

<iframe class="demo" src="/demos/input-fieldgroup.html" title="input-fieldgroup" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

Use the `disabled` prop to disable the input. To style the disabled state, add the `data-disabled` attribute to the `Field` component.

<iframe class="demo" src="/demos/input-disabled.html" title="input-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Invalid

Use the `aria-invalid` prop to mark the input as invalid. To style the invalid state, add the `data-invalid` attribute to the `Field` component.

<iframe class="demo" src="/demos/input-invalid.html" title="input-invalid" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## File

Use the `type="file"` prop to create a file input.

<iframe class="demo" src="/demos/input-file.html" title="input-file" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Inline

Use `Field` with `orientation="horizontal"` to create an inline input.
Pair with `Button` to create a search input with a button.

<iframe class="demo" src="/demos/input-inline.html" title="input-inline" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Grid

Use a grid layout to place multiple inputs side by side.

<iframe class="demo" src="/demos/input-grid.html" title="input-grid" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Required

Use the `required` attribute to indicate required inputs.

<iframe class="demo" src="/demos/input-required.html" title="input-required" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Badge

Use `Badge` in the label to highlight a recommended field.

<iframe class="demo" src="/demos/input-badge.html" title="input-badge" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Input Group

To add icons, text, or buttons inside an input, use the `InputGroup` component. See the [Input Group](/components/input-group) component for more examples.

<iframe class="demo" src="/demos/input-input-group.html" title="input-input-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Button Group

To add buttons to an input, use the `ButtonGroup` component. See the [Button Group](/components/button-group) component for more examples.

<iframe class="demo" src="/demos/input-button-group.html" title="input-button-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Form

A full form example with multiple inputs, a select, and a button.

<iframe class="demo" src="/demos/input-form.html" title="input-form" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/input-rtl.html" title="input-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>
