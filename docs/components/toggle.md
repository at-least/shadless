---
title: "Toggle"
description: "A two-state button that can be either on or off."
---

# Toggle

A two-state button that can be either on or off.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/toggle) · [api](https://www.radix-ui.com/docs/primitives/components/toggle#api-reference)</p>

<iframe class="demo" src="/demos/toggle-demo.html" title="toggle-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/toggle.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/toggle.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/toggle.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/toggle.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/toggle.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `aria-pressed` + `data-state="on|off"` on the root. Keys: Space / click toggles.

The root dispatches `shadless:change` (`detail: { pressed }`), bubbling, after the state change, whichever path caused it.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/toggle.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
| `size="outline"` (JSX prop) | `data-size="outline"` (markup) |
## Outline

Use `variant="outline"` for an outline style.

<iframe class="demo" src="/demos/toggle-outline.html" title="toggle-outline" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## With Text

<iframe class="demo" src="/demos/toggle-text.html" title="toggle-text" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Size

Use the `size` prop to change the size of the toggle.

<iframe class="demo" src="/demos/toggle-sizes.html" title="toggle-sizes" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

<iframe class="demo" src="/demos/toggle-disabled.html" title="toggle-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/toggle-rtl.html" title="toggle-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="toggle"` |

**Runtime:** `aria-pressed` + `data-state="on|off"` on the root. Keys: Space / click toggles. The root dispatches `shadless:change` (`detail: { pressed }`). No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix Toggle](https://www.radix-ui.com/docs/primitives/components/toggle#api-reference) documentation.
