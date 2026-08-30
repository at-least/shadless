---
title: "Switch"
description: "A control that allows the user to toggle between checked and not checked."
---

# Switch

A control that allows the user to toggle between checked and not checked.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/switch) · [api](https://www.radix-ui.com/docs/primitives/components/switch#api-reference)</p>

<iframe class="demo" src="/demos/switch-demo.html" title="switch-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/switch.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/switch.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/switch.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/switch.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/switch.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `role="switch"` root with `aria-checked` + `data-state`; the `switch-thumb` mirrors `data-state`. Keys: Space / click toggles.

The root dispatches `shadless:change` (`detail: { checked }`), bubbling, after the state change, whichever path caused it.

Forms: a `name` attribute submits its `value` (default `on`) while checked; `form.reset()` restores the initial state.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/switch.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Description

<iframe class="demo" src="/demos/switch-description.html" title="switch-description" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Choice Card

Card-style selection where `FieldLabel` wraps the entire `Field` for a clickable card pattern.

<iframe class="demo" src="/demos/switch-choice-card.html" title="switch-choice-card" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

Add the `disabled` prop to the `Switch` component to disable the switch. Add the `data-disabled` prop to the `Field` component for styling.

<iframe class="demo" src="/demos/switch-disabled.html" title="switch-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Invalid

Add the `aria-invalid` prop to the `Switch` component to indicate an invalid state. Add the `data-invalid` prop to the `Field` component for styling.

<iframe class="demo" src="/demos/switch-invalid.html" title="switch-invalid" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Size

Use the `size` prop to change the size of the switch.

<iframe class="demo" src="/demos/switch-sizes.html" title="switch-sizes" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/switch-rtl.html" title="switch-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="switch"` |
| `data-slot="switch-thumb"` |

**Runtime:** `role="switch"` root with `aria-checked` + `data-state`; the `switch-thumb` mirrors `data-state`. Keys: Space / click toggles. The root dispatches `shadless:change` (`detail: { checked }`). Forms: a `name` attribute submits its `value` (default `on`) while checked. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix Switch](https://www.radix-ui.com/docs/primitives/components/switch#api-reference) documentation.
