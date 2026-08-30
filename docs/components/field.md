---
title: "Field"
description: "Combine labels, controls, and help text to compose accessible form fields and grouped inputs."
---

# Field

Combine labels, controls, and help text to compose accessible form fields and grouped inputs.

<iframe class="demo" src="/demos/field-demo.html" title="field-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/field.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/field.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/field.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/field.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `orientation="outline"` (JSX prop) | `data-orientation="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
field
├── field-label
├── Input / Textarea / Switch / Select
├── field-description
└── field-error
```

## Anatomy

The `Field` family is designed for composing accessible forms. A typical field is structured as follows:

```tsx showLineNumbers
<Field>
  <FieldLabel htmlFor="input-id">Label</FieldLabel>
  {/* Input, Select, Switch, etc. */}
  <FieldDescription>Optional helper text.</FieldDescription>
  <FieldError>Validation message.</FieldError>
</Field>
```

- `Field` is the core wrapper for a single field.
- `FieldContent` is a flex column that groups label and description. Not required if you have no description.
- Wrap related fields with `FieldGroup`, and use `FieldSet` with `FieldLegend` for semantic grouping.

## Form

See the Form documentation for building forms with the `Field` component and React Hook Form, Tanstack Form, or Formisch.

## Input

<iframe class="demo" src="/demos/field-input.html" title="field-input" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Textarea

<iframe class="demo" src="/demos/field-textarea.html" title="field-textarea" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Select

<iframe class="demo" src="/demos/field-select.html" title="field-select" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Slider

<iframe class="demo" src="/demos/field-slider.html" title="field-slider" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Fieldset

<iframe class="demo" src="/demos/field-fieldset.html" title="field-fieldset" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Checkbox

<iframe class="demo" src="/demos/field-checkbox.html" title="field-checkbox" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Radio

<iframe class="demo" src="/demos/field-radio.html" title="field-radio" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Switch

<iframe class="demo" src="/demos/field-switch.html" title="field-switch" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Choice Card

Wrap `Field` components inside `FieldLabel` to create selectable field groups. This works with `RadioItem`, `Checkbox` and `Switch` components.

<iframe class="demo" src="/demos/field-choice-card.html" title="field-choice-card" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Field Group

Stack `Field` components with `FieldGroup`. Add `FieldSeparator` to divide them.

<iframe class="demo" src="/demos/field-group.html" title="field-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/field-rtl.html" title="field-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## Responsive Layout

- **Vertical fields:** Default orientation stacks label, control, and helper text—ideal for mobile-first layouts.
- **Horizontal fields:** Set `orientation="horizontal"` on `Field` to align the label and control side-by-side. Pair with `FieldContent` to keep descriptions aligned.
- **Responsive fields:** Set `orientation="responsive"` for automatic column layouts inside container-aware parents. Apply `@container/field-group` classes on `FieldGroup` to switch orientations at specific breakpoints.

<iframe class="demo" src="/demos/field-responsive.html" title="field-responsive" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Validation and Errors

- Add `data-invalid` to `Field` to switch the entire block into an error state.
- Add `aria-invalid` on the input itself for assistive technologies.
- Render `FieldError` immediately after the control or inside `FieldContent` to keep error messages aligned with the field.

```tsx showLineNumbers /data-invalid/ /aria-invalid/
<Field data-invalid>
  <FieldLabel htmlFor="email">Email</FieldLabel>
  <Input id="email" type="email" aria-invalid />
  <FieldError>Enter a valid email address.</FieldError>
</Field>
```

## Accessibility

- `FieldSet` and `FieldLegend` keep related controls grouped for keyboard and assistive tech users.
- `Field` outputs `role="group"` so nested controls inherit labeling from `FieldLabel` and `FieldLegend` when combined.
- Apply `FieldSeparator` sparingly to ensure screen readers encounter clear section boundaries.

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="field-set"` |
| `data-slot="field-legend"` |
| `data-slot="field-group"` |
| `data-slot="field"` |
| `data-slot="field-content"` |
| `data-slot="field-label"` |
| `data-slot="field-description"` |
| `data-slot="field-separator"` |
| `data-slot="field-separator-content"` |
| `data-slot="field-error"` |

### FieldSet

Container that renders a semantic `fieldset` with spacing presets.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<FieldSet>
  <FieldLegend>Delivery</FieldLegend>
  <FieldGroup>{/* Fields */}</FieldGroup>
</FieldSet>
```

### FieldLegend

Legend element for a `FieldSet`. Switch to the `label` variant to align with label sizing.

| Prop        | Type                  | Default    |
| ----------- | --------------------- | ---------- |
| `variant`   | `"legend" \| "label"` | `"legend"` |
| `className` | `string`              |            |

```tsx
<FieldLegend variant="label">Notification Preferences</FieldLegend>
```

The `FieldLegend` has two variants: `legend` and `label`. The `label` variant applies label sizing and alignment. Handy if you have nested `FieldSet`.

### FieldGroup

Layout wrapper that stacks `Field` components and enables container queries for responsive orientations.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<FieldGroup className="@container/field-group flex flex-col gap-6">
  <Field>{/* ... */}</Field>
  <Field>{/* ... */}</Field>
</FieldGroup>
```

### Field

The core wrapper for a single field. Provides orientation control, invalid state styling, and spacing.

| Prop           | Type                                         | Default      |
| -------------- | -------------------------------------------- | ------------ |
| `orientation`  | `"vertical" \| "horizontal" \| "responsive"` | `"vertical"` |
| `className`    | `string`                                     |              |
| `data-invalid` | `boolean`                                    |              |

```tsx
<Field orientation="horizontal">
  <FieldLabel htmlFor="remember">Remember me</FieldLabel>
  <Switch id="remember" />
</Field>
```

### FieldContent

Flex column that groups control and descriptions when the label sits beside the control. Not required if you have no description.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<Field>
  <Checkbox id="notifications" />
  <FieldContent>
    <FieldLabel htmlFor="notifications">Notifications</FieldLabel>
    <FieldDescription>Email, SMS, and push options.</FieldDescription>
  </FieldContent>
</Field>
```

### FieldLabel

Label styled for both direct inputs and nested `Field` children.

| Prop        | Type      | Default |
| ----------- | --------- | ------- |
| `className` | `string`  |         |
| `asChild`   | `boolean` | `false` |

```tsx
<FieldLabel htmlFor="email">Email</FieldLabel>
```

### FieldTitle

Renders a title with label styling inside `FieldContent`.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<FieldContent>
  <FieldTitle>Enable Touch ID</FieldTitle>
  <FieldDescription>Unlock your device faster.</FieldDescription>
</FieldContent>
```

### FieldDescription

Helper text slot that automatically balances long lines in horizontal layouts.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<FieldDescription>We never share your email with anyone.</FieldDescription>
```

### FieldSeparator

Visual divider to separate sections inside a `FieldGroup`. Accepts optional inline content.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<FieldSeparator>Or continue with</FieldSeparator>
```

### FieldError

Accessible error container that accepts children or an `errors` array (e.g., from `react-hook-form`).

| Prop        | Type                                       | Default |
| ----------- | ------------------------------------------ | ------- |
| `errors`    | `Array<{ message?: string } \| undefined>` |         |
| `className` | `string`                                   |         |

```tsx
<FieldError errors={errors.username} />
```

When the `errors` array contains multiple messages, the component renders a list automatically.

`FieldError` also accepts issues produced by any validator that implements [Standard Schema](https://standardschema.dev/), including Zod, Valibot, and ArkType. Pass the `issues` array from the schema result directly to render a unified error list across libraries.
