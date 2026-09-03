---
title: "shimmer"
description: "Utilities for adding a shimmer effect to text elements."
---

# shimmer

Utilities for adding a shimmer effect to text elements.

<div class="demo-missing" data-demo="shimmer-demo" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-demo</code></div>

## Installation

In shadless, the `shimmer` utilities ship precompiled inside `dist/shadless-core.css`
(npm: bare `shadless`) — the same file every shadless component already needs, so
if you're using any component there is nothing extra to install or import.
Standalone, load `shadless-core.css` and use the classes directly (see the
[Installation](/guides/installation) guide).

## Usage

| Class                         | Styles                                                                                               |
| ----------------------------- | ---------------------------------------------------------------------------------------------------- |
| `shimmer`                     | `background-clip: text;` <br /> `animation: tw-shimmer var(--shimmer-duration, 2s) linear infinite;` |
| `shimmer-once`                | `animation-iteration-count: 1;`                                                                      |
| `shimmer-reverse`             | `animation-direction: reverse;`                                                                      |
| `shimmer-none`                | `--shimmer-image: none;` <br /> `--shimmer-text-fill: currentColor;`                                 |
| `shimmer-color-<color>`       | `--shimmer-color: <color>;`                                                                          |
| `shimmer-color-[<value>]`     | `--shimmer-color: <value>;`                                                                          |
| `shimmer-color-<color>/<pct>` | `--shimmer-color: color-mix(in oklch, <color> <pct>, transparent);`                                  |
| `shimmer-duration-<number>`   | `--shimmer-duration: calc(<number> * 1ms);`                                                          |
| `shimmer-spread-<number>`     | `--shimmer-spread: calc(var(--spacing) * <number>);`                                                 |
| `shimmer-spread-[<value>]`    | `--shimmer-spread: <value>;`                                                                         |
| `shimmer-angle-<number>`      | `--shimmer-angle: calc(<number> * 1deg);`                                                            |

Add `shimmer` to a text element.

```html
<p class="shimmer text-muted-foreground">Generating response&hellip;</p>
```

The shimmer is built on `currentColor`, so it adapts to the element:

- The highlight is derived from the text color, with no configuration needed.
- It works on any color, from `text-muted-foreground` to brand colors.
- In dark mode, the highlight automatically brightens to stay visible.

The effect is pure CSS. The text is painted with `background-clip: text`, and the highlight sweeps across it in a seamless loop.

## With Marker

The shimmer composes with any component that renders text. A common pattern is a [Marker](/components/marker) showing a live status while the assistant is working:

<div class="demo-missing" data-demo="shimmer-marker" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-marker</code></div>

```html
<div data-slot="marker" role="status">
  <span data-slot="marker-icon"><!-- Spinner markup — see /components/spinner --></span>
  <span data-slot="marker-content" class="shimmer">Thinking&hellip;</span>
</div>
```

## Color

Use `shimmer-color-<color>` to set the highlight color explicitly. It accepts theme colors with an optional opacity modifier, or any arbitrary color value.

<div class="demo-missing" data-demo="shimmer-color" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-color</code></div>

```html
<p class="shimmer shimmer-color-blue-500/60">Generating response&hellip;</p>
<p class="shimmer shimmer-color-[#378ADD]">Generating response&hellip;</p>
```

## Duration

Use `shimmer-duration-<number>` to set the duration of one sweep in milliseconds. The default is `2000`, i.e. `2s`.

<div class="demo-missing" data-demo="shimmer-duration" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-duration</code></div>

```html
<p class="shimmer shimmer-duration-1000">Generating response&hellip;</p>
```

## Spread

Use `shimmer-spread-<number>` to set the width of the highlight band using the spacing scale. The default is `calc(3ch + 40px)`: a fixed base plus a `3ch` term that scales with the font size.

<div class="demo-missing" data-demo="shimmer-spread" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-spread</code></div>

```html
<p class="shimmer shimmer-spread-24">Generating response&hellip;</p>
```

For one-off values, use an arbitrary length or percentage:

```html
<p class="shimmer shimmer-spread-[5rem]">Generating response&hellip;</p>
```

## Angle

Use `shimmer-angle-<number>` to set the tilt of the highlight band in degrees. The default is `20`.

<div class="demo-missing" data-demo="shimmer-angle" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-angle</code></div>

```html
<p class="shimmer shimmer-angle-45">Generating response&hellip;</p>
```

## Reverse

Use `shimmer-reverse` to sweep the highlight in the opposite direction. In RTL layouts the sweep already follows the reading direction. See [RTL](#rtl).

```html
<p class="shimmer shimmer-reverse">Generating response&hellip;</p>
```

## Play Once

Use `shimmer-once` to play a single sweep instead of looping, useful as a reveal when streaming completes. Pair it with `shimmer-duration-<number>` to control how long the sweep takes.

<div class="demo-missing" data-demo="shimmer-once" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-once</code></div>

```html
<p class="shimmer shimmer-duration-1100 shimmer-once">
  Response generated.
</p>
```

## Disabling the Shimmer

Use `shimmer-none` to turn the effect off and render the text normally. It works in any class order, so the typical use is responsive or stateful:

<div class="demo-missing" data-demo="shimmer-none" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-none</code></div>

```html
<p class="shimmer md:shimmer-none">Generating response&hellip;</p>
```

## Fallback

The shimmer is built on modern color features, [relative color syntax](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_colors/Relative_colors) and `color-mix()`, which are available in all current browsers. In older browsers without support, the highlight gradient is dropped and the text can render transparent. If you target older browsers, apply `shimmer` conditionally with a `supports-*` variant:

```html
<p class="supports-[color:oklch(from_white_l_c_h)]:shimmer">
  Generating response&hellip;
</p>
```

## Reduced Motion

When the user prefers reduced motion, the animation is disabled automatically and the text renders normally. There is nothing to configure.

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

The sweep follows the reading direction, left to right in LTR and right to left in RTL, with no extra classes. Use `shimmer-reverse` to flip the direction manually.

<div class="demo-missing" data-demo="shimmer-rtl" data-status="unavailable">demo not available in shadless (base-style demo) — <code>shimmer-rtl</code></div>
