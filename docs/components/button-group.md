---
title: "Button Group"
description: "A container that groups related buttons together with consistent styling."
---

# Button Group

A container that groups related buttons together with consistent styling.

<iframe class="demo" src="/demos/button-group-demo.html" title="button-group-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/button-group.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/button-group.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/button-group.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                     into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/button-group.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `orientation="outline"` (JSX prop) | `data-orientation="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
button-group
├── Button or Input
├── button-group-separator
└── ButtonGroupText
```

## Accessibility

- The `ButtonGroup` component has the `role` attribute set to `group`.
- Use <kbd>Tab</kbd> to navigate between the buttons in the group.
- Use `aria-label` or `aria-labelledby` to label the button group.

```tsx showLineNumbers
<ButtonGroup aria-label="Button group">
  <Button>Button 1</Button>
  <Button>Button 2</Button>
</ButtonGroup>
```

## ButtonGroup vs ToggleGroup

- Use the `ButtonGroup` component when you want to group buttons that perform an action.
- Use the `ToggleGroup` component when you want to group buttons that toggle a state.

## Orientation

Set the `orientation` prop to change the button group layout.

<iframe class="demo" src="/demos/button-group-orientation.html" title="button-group-orientation" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Size

Control the size of buttons using the `size` prop on individual buttons.

<iframe class="demo" src="/demos/button-group-size.html" title="button-group-size" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Nested

Nest `<ButtonGroup>` components to create button groups with spacing.

<iframe class="demo" src="/demos/button-group-nested.html" title="button-group-nested" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Separator

The `ButtonGroupSeparator` component visually divides buttons within a group.

Buttons with variant `outline` do not need a separator since they have a border. For other variants, a separator is recommended to improve the visual hierarchy.

<iframe class="demo" src="/demos/button-group-separator.html" title="button-group-separator" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Split

Create a split button group by adding two buttons separated by a `ButtonGroupSeparator`.

<iframe class="demo" src="/demos/button-group-split.html" title="button-group-split" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Input

Wrap an `Input` component with buttons.

<iframe class="demo" src="/demos/button-group-input.html" title="button-group-input" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Input Group

Wrap an `InputGroup` component to create complex input layouts.

<iframe class="demo" src="/demos/button-group-input-group.html" title="button-group-input-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Dropdown Menu

Create a split button group with a `DropdownMenu` component.

<iframe class="demo" src="/demos/button-group-dropdown.html" title="button-group-dropdown" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Select

Pair with a `Select` component.

<iframe class="demo" src="/demos/button-group-select.html" title="button-group-select" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Popover

Use with a `Popover` component.

<iframe class="demo" src="/demos/button-group-popover.html" title="button-group-popover" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/button-group-rtl.html" title="button-group-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="button-group"` |
| `data-slot="button-group-separator"` |

### ButtonGroup

The `ButtonGroup` component is a container that groups related buttons together with consistent styling.

| Prop          | Type                         | Default        |
| ------------- | ---------------------------- | -------------- |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` |

```tsx
<ButtonGroup>
  <Button>Button 1</Button>
  <Button>Button 2</Button>
</ButtonGroup>
```

Nest multiple button groups to create complex layouts with spacing. See the [nested](#nested) example for more details.

```tsx
<ButtonGroup>
  <ButtonGroup />
  <ButtonGroup />
</ButtonGroup>
```

### ButtonGroupSeparator

The `ButtonGroupSeparator` component visually divides buttons within a group.

| Prop          | Type                         | Default      |
| ------------- | ---------------------------- | ------------ |
| `orientation` | `"horizontal" \| "vertical"` | `"vertical"` |

```tsx
<ButtonGroup>
  <Button>Button 1</Button>
  <ButtonGroupSeparator />
  <Button>Button 2</Button>
</ButtonGroup>
```

### ButtonGroupText

Use this component to display text within a button group.

| Prop      | Type      | Default |
| --------- | --------- | ------- |
| `asChild` | `boolean` | `false` |

```tsx
<ButtonGroup>
  <ButtonGroupText>Text</ButtonGroupText>
  <Button>Button</Button>
</ButtonGroup>
```

Use the `asChild` prop to render a custom component as the text, for example a label.

```tsx showLineNumbers

export function ButtonGroupTextDemo() {
  return (
    <ButtonGroup>
      <ButtonGroupText asChild>
        <Label htmlFor="name">Text</Label>
      </ButtonGroupText>
      <Input placeholder="Type something here..." id="name" />
    </ButtonGroup>
  )
}
```
