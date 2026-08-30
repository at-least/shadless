---
title: "Button"
description: "Displays a button or a component that looks like a button."
---

# Button

Displays a button or a component that looks like a button.

<iframe class="demo" src="/demos/button-demo.html" title="button-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/button.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/button.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/button.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/button.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
| `size="outline"` (JSX prop) | `data-size="outline"` (markup) |
## Cursor

Tailwind v4 [switched](https://tailwindcss.com/docs/upgrade-guide#buttons-use-the-default-cursor) from `cursor: pointer` to `cursor: default` for the button component.

If you want to keep the `cursor: pointer` behavior, add the following code to your CSS file:

In shadless just keep the CSS rule above — there is no CLI flag to set.

```css showLineNumbers title="globals.css"
@layer base {
  button:not(:disabled),
  [role="button"]:not(:disabled) {
    cursor: pointer;
  }
}
```

## Size

Use the `size` prop to change the size of the button.

<iframe class="demo" src="/demos/button-size.html" title="button-size" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Default

<iframe class="demo" src="/demos/button-default.html" title="button-default" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Outline

<iframe class="demo" src="/demos/button-outline.html" title="button-outline" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Secondary

<iframe class="demo" src="/demos/button-secondary.html" title="button-secondary" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Ghost

<iframe class="demo" src="/demos/button-ghost.html" title="button-ghost" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Destructive

<iframe class="demo" src="/demos/button-destructive.html" title="button-destructive" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Link

<iframe class="demo" src="/demos/button-link.html" title="button-link" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Icon

<iframe class="demo" src="/demos/button-icon.html" title="button-icon" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## With Icon

Remember to add the `data-icon="inline-start"` or `data-icon="inline-end"` attribute to the icon for the correct spacing.

<iframe class="demo" src="/demos/button-with-icon.html" title="button-with-icon" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Rounded

Use the `rounded-full` class to make the button rounded.

<iframe class="demo" src="/demos/button-rounded.html" title="button-rounded" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Spinner

Render a `<Spinner />` component inside the button to show a loading state. Remember to add the `data-icon="inline-start"` or `data-icon="inline-end"` attribute to the spinner for the correct spacing.

<iframe class="demo" src="/demos/button-spinner.html" title="button-spinner" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Button Group

To create a button group, use the `ButtonGroup` component. See the [Button Group](/components/button-group) documentation for more details.

<iframe class="demo" src="/demos/button-group-demo.html" title="button-group-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## As Child

You can use the `asChild` prop on `<Button />` to make another component look like a button. Here's an example of a link that looks like a button.

<iframe class="demo" src="/demos/button-aschild.html" title="button-aschild" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/button-rtl.html" title="button-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="button"` |

### Button

The `Button` component is a wrapper around the `button` element that adds a variety of styles and functionality.

| Prop      | Type                                                                                 | Default     |
| --------- | ------------------------------------------------------------------------------------ | ----------- |
| `variant` | `"default" \| "outline" \| "ghost" \| "destructive" \| "secondary" \| "link"`        | `"default"` |
| `size`    | `"default" \| "xs" \| "sm" \| "lg" \| "icon" \| "icon-xs" \| "icon-sm" \| "icon-lg"` | `"default"` |
| `asChild` | `boolean`                                                                            | `false`     |
