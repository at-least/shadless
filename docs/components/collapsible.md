---
title: "Collapsible"
description: "An interactive component which expands/collapses a panel."
---

# Collapsible

An interactive component which expands/collapses a panel.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/collapsible) · [api](https://www.radix-ui.com/docs/primitives/components/collapsible#api-reference)</p>

<iframe class="demo" src="/demos/collapsible-demo.html" title="collapsible-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless to your Tailwind v4 entry:**

```css
@import "shadless";
```

This component has no stylesheet of its own — its styling rides the core theme and utilities in `shadless`.

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/components/collapsible.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/collapsible.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/collapsible.js"></script>
```

**Copy the markup from                                    into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `collapsible-trigger` carries `aria-expanded` + `data-state` and `aria-controls` → the `collapsible-content`, `hidden` while closed. Keys: Enter / Space / click toggles.

The trigger dispatches `shadless:open` / `shadless:close`, bubbling, after the state change, whichever path caused it.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/collapsible.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
collapsible
├── collapsible-trigger
└── collapsible-content
```

## Controlled State

Use the `open` and `onOpenChange` props to control the state.



## Basic

<iframe class="demo" src="/demos/collapsible-basic.html" title="collapsible-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Settings Panel

Use a trigger button to reveal additional settings.

<iframe class="demo" src="/demos/collapsible-settings.html" title="collapsible-settings" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## File Tree

Use nested collapsibles to build a file tree.

<iframe class="demo" src="/demos/collapsible-file-tree.html" title="collapsible-file-tree" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/collapsible-rtl.html" title="collapsible-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="collapsible"` |
| `data-slot="collapsible-trigger"` |
| `data-slot="collapsible-content"` |

**Runtime:** `collapsible-trigger` carries `aria-expanded` + `data-state` and `aria-controls` → the `collapsible-content`, `hidden` while closed. Keys: Enter / Space / click toggles. The trigger dispatches `shadless:open` / `shadless:close`. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix UI](https://www.radix-ui.com/docs/primitives/components/collapsible#api-reference) documentation for more information.
