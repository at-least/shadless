---
title: "Toggle Group"
description: "A set of two-state buttons that can be toggled on or off."
---

# Toggle Group

A set of two-state buttons that can be toggled on or off.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/toggle-group) · [api](https://www.radix-ui.com/docs/primitives/components/toggle-group#api-reference)</p>

<iframe class="demo" src="/demos/toggle-group-demo.html" title="toggle-group-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/toggle-group.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/toggle-group.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/toggle-group.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/toggle-group.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/toggle-group.js"></script>
```

**Copy the markup from                                     into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `role="group"` root; single mode items are `role="radio"` with `aria-checked`, multiple mode items carry `aria-pressed`; `data-state="on|off"` in both. Keys: arrows / Home / End move focus over enabled items (rtl-aware); Space / Enter / click selects.

The root dispatches `shadless:change` (`detail: { value, item }` — `value` is the on item's value, or an array in multiple mode), bubbling, after the state change, whichever path caused it.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/toggle-group.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
toggle-group
├── toggle-group-item
└── toggle-group-item
```

## Outline

Use `variant="outline"` for an outline style.

<iframe class="demo" src="/demos/toggle-group-outline.html" title="toggle-group-outline" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Size

Use the `size` prop to change the size of the toggle group.

<iframe class="demo" src="/demos/toggle-group-sizes.html" title="toggle-group-sizes" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Spacing

Use `spacing` to add spacing between toggle group items.

<iframe class="demo" src="/demos/toggle-group-spacing.html" title="toggle-group-spacing" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Vertical

Use `orientation="vertical"` for vertical toggle groups.

<iframe class="demo" src="/demos/toggle-group-vertical.html" title="toggle-group-vertical" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

<iframe class="demo" src="/demos/toggle-group-disabled.html" title="toggle-group-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Custom

A custom toggle group example.

<iframe class="demo" src="/demos/toggle-group-font-weight-selector.html" title="toggle-group-font-weight-selector" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/toggle-group-rtl.html" title="toggle-group-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="toggle-group"` |
| `data-slot="toggle-group-item"` |

**Runtime:** `role="group"` root; single mode items are `role="radio"` with `aria-checked`, multiple mode items carry `aria-pressed`; `data-state="on|off"` in both. Keys: arrows / Home / End move focus over enabled items (rtl-aware); Space / Enter / click selects. The root dispatches `shadless:change` (`detail: { value, item }` — `value` is the on item's value, or an array in multiple mode). No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix Toggle Group](https://www.radix-ui.com/docs/primitives/components/toggle-group#api-reference) documentation.

## Changelog

### 2026-05-17 Default Spacing

Changed the default `spacing` from `0` to `2` so toggle groups render with space between items by default. Use `spacing={0}` for connected items.
