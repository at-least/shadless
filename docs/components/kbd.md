---
title: "Kbd"
description: "Used to display textual user input from keyboard."
---

# Kbd

Used to display textual user input from keyboard.

<iframe class="demo" src="/demos/kbd-demo.html" title="kbd-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/kbd.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/kbd.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/kbd.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                            into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/kbd.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
kbd
kbd-group
├── kbd
└── kbd
```

## Group

Use the `KbdGroup` component to group keyboard keys together.

<iframe class="demo" src="/demos/kbd-group.html" title="kbd-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Button

Use the `Kbd` component inside a `Button` component to display a keyboard key inside a button.

<iframe class="demo" src="/demos/kbd-button.html" title="kbd-button" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Tooltip

You can use the `Kbd` component inside a `Tooltip` component to display a tooltip with a keyboard key.

<iframe class="demo" src="/demos/kbd-tooltip.html" title="kbd-tooltip" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Input Group

You can use the `Kbd` component inside a `InputGroupAddon` component to display a keyboard key inside an input group.

<iframe class="demo" src="/demos/kbd-input-group.html" title="kbd-input-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/kbd-rtl.html" title="kbd-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="kbd"` |
| `data-slot="kbd-group"` |

### Kbd

Use the `Kbd` component to display a keyboard key.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | ``      |

```tsx
<Kbd>Ctrl</Kbd>
```

### KbdGroup

Use the `KbdGroup` component to group `Kbd` components together.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | ``      |

```tsx
<KbdGroup>
  <Kbd>Ctrl</Kbd>
  <Kbd>B</Kbd>
</KbdGroup>
```
