---
title: "Native Select"
description: "A styled native HTML select element with consistent design system integration."
---

# Native Select

A styled native HTML select element with consistent design system integration.

::: tip
}>
For a styled select component, see the [Select](/components/select)
component.
:::

<iframe class="demo" src="/demos/native-select-demo.html" title="native-select-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/native-select.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/native-select.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/native-select.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                      into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/native-select.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
native-select-wrapper
├── native-select-option
├── native-select-option
├── native-select-option
└── native-select-option
```

## Groups

Use `NativeSelectOptGroup` to organize options into categories.

<iframe class="demo" src="/demos/native-select-groups.html" title="native-select-groups" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

Add the `disabled` prop to the `NativeSelect` component to disable the select.

<iframe class="demo" src="/demos/native-select-disabled.html" title="native-select-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Invalid

Use `aria-invalid` to show validation errors and the `data-invalid` attribute to the `Field` component for styling.

<iframe class="demo" src="/demos/native-select-invalid.html" title="native-select-invalid" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Native Select vs Select

- Use `NativeSelect` for native browser behavior, better performance, or mobile-optimized dropdowns.
- Use `Select` for custom styling, animations, or complex interactions.

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/native-select-rtl.html" title="native-select-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="native-select-wrapper"` |
| `data-slot="native-select"` |
| `data-slot="native-select-icon"` |
| `data-slot="native-select-option"` |
| `data-slot="native-select-optgroup"` |

### NativeSelect

The main select component that wraps the native HTML select element.

```tsx
<NativeSelect>
  <NativeSelectOption value="option1">Option 1</NativeSelectOption>
  <NativeSelectOption value="option2">Option 2</NativeSelectOption>
</NativeSelect>
```

### NativeSelectOption

Represents an individual option within the select.

| Prop       | Type      | Default |
| ---------- | --------- | ------- |
| `value`    | `string`  |         |
| `disabled` | `boolean` | `false` |

### NativeSelectOptGroup

Groups related options together for better organization.

| Prop       | Type      | Default |
| ---------- | --------- | ------- |
| `label`    | `string`  |         |
| `disabled` | `boolean` | `false` |

```tsx
<NativeSelectOptGroup label="Fruits">
  <NativeSelectOption value="apple">Apple</NativeSelectOption>
  <NativeSelectOption value="banana">Banana</NativeSelectOption>
</NativeSelectOptGroup>
```
