---
title: "Input Group"
description: "Add addons, buttons, and helper content to inputs."
---

# Input Group

Add addons, buttons, and helper content to inputs.

<iframe class="demo" src="/demos/input-group-demo.html" title="input-group-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/input-group.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/input-group.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/input-group.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                    into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/input-group.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `align="outline"` (JSX prop) | `data-align="outline"` (markup) |
| `size="outline"` (JSX prop) | `data-size="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
input-group
├── input-group-control or input-group-control
├── input-group-addon
├── InputGroupButton
└── InputGroupText
```

## Align

Use the `align` prop on `InputGroupAddon` to position the addon relative to the input.

::: tip
For proper focus management, `InputGroupAddon` should always be placed after
`InputGroupInput` or `InputGroupTextarea` in the DOM. Use the `align` prop to
visually position the addon.
:::

### inline-start

Use `align="inline-start"` to position the addon at the start of the input. This is the default.

<iframe class="demo" src="/demos/input-group-inline-start.html" title="input-group-inline-start" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

### inline-end

Use `align="inline-end"` to position the addon at the end of the input.

<iframe class="demo" src="/demos/input-group-inline-end.html" title="input-group-inline-end" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

### block-start

Use `align="block-start"` to position the addon above the input.

<iframe class="demo" src="/demos/input-group-block-start.html" title="input-group-block-start" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

### block-end

Use `align="block-end"` to position the addon below the input.

<iframe class="demo" src="/demos/input-group-block-end.html" title="input-group-block-end" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Icon

<iframe class="demo" src="/demos/input-group-icon.html" title="input-group-icon" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Text

<iframe class="demo" src="/demos/input-group-text.html" title="input-group-text" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Button

<iframe class="demo" src="/demos/input-group-button.html" title="input-group-button" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Kbd

<iframe class="demo" src="/demos/input-group-kbd.html" title="input-group-kbd" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Dropdown

<iframe class="demo" src="/demos/input-group-dropdown.html" title="input-group-dropdown" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Spinner

<iframe class="demo" src="/demos/input-group-spinner.html" title="input-group-spinner" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Textarea

<iframe class="demo" src="/demos/input-group-textarea.html" title="input-group-textarea" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Custom Input

Add the `data-slot="input-group-control"` attribute to your custom input for automatic focus state handling.

Here's an example of a custom resizable textarea from a third-party library.

<iframe class="demo" src="/demos/input-group-custom.html" title="input-group-custom" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/input-group-rtl.html" title="input-group-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="input-group"` |
| `data-slot="input-group-addon"` |
| `data-slot="input-group-control"` |

### InputGroup

The main component that wraps inputs and addons.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<InputGroup>
  <InputGroupInput />
  <InputGroupAddon />
</InputGroup>
```

### InputGroupAddon

Displays icons, text, buttons, or other content alongside inputs.

::: tip
} title="Focus Navigation">
For proper focus navigation, the `InputGroupAddon` component should be placed
after the input. Set the `align` prop to position the addon.
:::

| Prop        | Type                                                             | Default          |
| ----------- | ---------------------------------------------------------------- | ---------------- |
| `align`     | `"inline-start" \| "inline-end" \| "block-start" \| "block-end"` | `"inline-start"` |
| `className` | `string`                                                         |                  |

```tsx
<InputGroupAddon align="inline-end">
  <SearchIcon />
</InputGroupAddon>
```

**For `<InputGroupInput />`, use the `inline-start` or `inline-end` alignment. For `<InputGroupTextarea />`, use the `block-start` or `block-end` alignment.**

The `InputGroupAddon` component can have multiple `InputGroupButton` components and icons.

```tsx
<InputGroupAddon>
  <InputGroupButton>Button</InputGroupButton>
  <InputGroupButton>Button</InputGroupButton>
</InputGroupAddon>
```

### InputGroupButton

Displays buttons within input groups.

| Prop        | Type                                                                          | Default   |
| ----------- | ----------------------------------------------------------------------------- | --------- |
| `size`      | `"xs" \| "icon-xs" \| "sm" \| "icon-sm"`                                      | `"xs"`    |
| `variant`   | `"default" \| "destructive" \| "outline" \| "secondary" \| "ghost" \| "link"` | `"ghost"` |
| `className` | `string`                                                                      |           |

```tsx
<InputGroupButton>Button</InputGroupButton>
<InputGroupButton size="icon-xs" aria-label="Copy">
  <CopyIcon />
</InputGroupButton>
```

### InputGroupInput

Replacement for `<Input />` when building input groups. This component has the input group styles pre-applied and uses the unified `data-slot="input-group-control"` for focus state handling.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

All other props are passed through to the underlying `<Input />` component.

```tsx
<InputGroup>
  <InputGroupInput placeholder="Enter text..." />
  <InputGroupAddon>
    <SearchIcon />
  </InputGroupAddon>
</InputGroup>
```

### InputGroupTextarea

Replacement for `<Textarea />` when building input groups. This component has the textarea group styles pre-applied and uses the unified `data-slot="input-group-control"` for focus state handling.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

All other props are passed through to the underlying `<Textarea />` component.

```tsx
<InputGroup>
  <InputGroupTextarea placeholder="Enter message..." />
  <InputGroupAddon align="block-end">
    <InputGroupButton>Send</InputGroupButton>
  </InputGroupAddon>
</InputGroup>
```

## Changelog

### 2025-10-06 `InputGroup`

Add the `min-w-0` class to the `InputGroup` component. See [diff](https://github.com/shadcn-ui/ui/pull/8341/files#diff-0e2ee95d0050ca4c5d82339df86c54e14a6739dc4638fdda0eec8f73aebc2da9).
