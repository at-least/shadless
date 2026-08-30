---
title: "Marker"
description: "Displays an inline status, system note, bordered row, or labeled separator in a conversation."
---

# Marker

Displays an inline status, system note, bordered row, or labeled separator in a conversation.

<iframe class="demo" src="/demos/marker-demo.html" title="marker-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

The `Marker` component displays inline conversation markers such as status updates, system notes, bordered rows, and labeled separators. Compose it with [         ](/components/message) in a conversation thread.

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/marker.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/marker.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/marker.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/marker.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
marker
├── marker-icon
└── marker-content
```

## Features

- Inline marker, bordered row, and labeled separator variants
- Decorative icon slot that is hidden from assistive tech
- Polymorphic root via `asChild` for link and button markers
- Pairs with the [         ](/guides/shimmer) utility for streaming status text
- Customizable styling through the `className` prop on every part

## Variants

Use `variant` to switch between an inline marker, bordered row, and labeled separator.

<iframe class="demo" src="/demos/marker-variants.html" title="marker-variants" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

| Variant     | Description                                          |
| ----------- | ---------------------------------------------------- |
| `default`   | An inline marker for status, notes, and actions.     |
| `border`    | A default marker with a bottom border under the row. |
| `separator` | A centered label with divider lines on each side.    |

## Status

Set `role="status"` and include a [         ](/components/spinner) for streaming or in-progress markers so updates are announced.

<iframe class="demo" src="/demos/marker-status.html" title="marker-status" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Shimmer

Add the [         ](/guides/shimmer) utility class to `MarkerContent` for an animated streaming-text effect. The utility ships with the `shadcn` package — see the shimmer docs for installation.

<iframe class="demo" src="/demos/marker-shimmer.html" title="marker-shimmer" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Separator

Use the `separator` variant for labeled dividers, such as dates or section breaks, in a conversation.

<iframe class="demo" src="/demos/marker-separator.html" title="marker-separator" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Border

Use the `border` variant for status rows that should keep the default marker alignment while separating the next row.

<iframe class="demo" src="/demos/marker-border.html" title="marker-border" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## With Icon

Use `MarkerIcon` to render an icon alongside the content. Use `flex-col` to stack the icon above the content.

<iframe class="demo" src="/demos/marker-icon.html" title="marker-icon" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Links and Buttons

Turn a marker into a link or button with the `asChild` prop on `Marker`.

<iframe class="demo" src="/demos/marker-link-button.html" title="marker-link-button" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>



## Accessibility

`Marker` is presentational by default. The correct semantics depend on how you use it, so choose the role based on intent rather than relying on a single default.

### Status and Progress

For streaming or progress markers such as "Thinking..." or a running tool, set `role="status"` so assistive tech announces the update as it appears. `Marker` forwards `role` to the underlying element.

```tsx showLineNumbers
<Marker role="status">
  <MarkerIcon>
    <Spinner />
  </MarkerIcon>
  <MarkerContent>Compacting conversation</MarkerContent>
</Marker>
```

### Labeled Separators

A separator that carries text, such as a date or a section label, needs no role. The divider lines are decorative CSS pseudo-elements, and the text is announced as ordinary content.

```tsx showLineNumbers
<Marker variant="separator">
  <MarkerContent>Today</MarkerContent>
</Marker>
```

::: tip
**Note:** Do not add `role="separator"` to a labeled divider. A separator
takes its accessible name from `aria-label`, not from its text, and its
contents are treated as presentational, so the visible label would not be
announced. Reserve `role="separator"` for a divider with no meaningful text.
:::

### Bordered Markers

A bordered marker keeps the same semantics as the default marker. The bottom border is decorative, so choose `role="status"`, `asChild`, or no role based on the marker's purpose.

```tsx showLineNumbers
<Marker variant="border">
  <MarkerIcon>
    <FileTextIcon />
  </MarkerIcon>
  <MarkerContent>Opened implementation notes</MarkerContent>
</Marker>
```

### Decorative Icons

`MarkerIcon` is decorative and hidden from assistive tech with `aria-hidden`, so the adjacent `MarkerContent` carries the meaning. For an icon-only marker, provide an `aria-label` or visible text so it is not announced as empty.

```tsx showLineNumbers
<Marker aria-label="Synced">
  <MarkerIcon>
    <CheckIcon />
  </MarkerIcon>
</Marker>
```

### Interactive Markers

When a marker links or triggers an action, render it as a real `<button>` or `<a>` with the `asChild` prop so it is focusable and exposes the correct role. The accessible name comes from the marker text.

```tsx showLineNumbers
<Marker asChild>
  <a href="/files">
    <MarkerIcon>
      <FileTextIcon />
    </MarkerIcon>
    <MarkerContent>Explored 4 files</MarkerContent>
  </a>
</Marker>
```

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="marker"` |
| `data-slot="marker-icon"` |
| `data-slot="marker-content"` |

### Marker

The root marker element. The file also exports `markerVariants` for composing the marker styles into custom components.

| Prop        | Type                                   | Default     | Description                                      |
| ----------- | -------------------------------------- | ----------- | ------------------------------------------------ |
| `variant`   | `"default" \| "border" \| "separator"` | `"default"` | The marker layout.                               |
| `asChild`   | `boolean`                              | `false`     | Render as the child element, such as a link.     |
| `className` | `string`                               | -           | Additional classes to apply to the root element. |

### MarkerIcon

A decorative icon slot. Hidden from assistive tech with `aria-hidden`.

| Prop        | Type     | Default | Description                                   |
| ----------- | -------- | ------- | --------------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the icon slot. |

### MarkerContent

The marker text content.

| Prop        | Type     | Default | Description                                      |
| ----------- | -------- | ------- | ------------------------------------------------ |
| `className` | `string` | -       | Additional classes to apply to the content slot. |
