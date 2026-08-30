---
title: "Accordion"
description: "A vertically stacked set of interactive headings that each reveal a section of content."
---

# Accordion

A vertically stacked set of interactive headings that each reveal a section of content.

<p class="page-links">[doc](https://www.radix-ui.com/primitives/docs/components/accordion) · [api](https://www.radix-ui.com/primitives/docs/components/accordion#api-reference)</p>

<iframe class="demo" src="/demos/accordion-demo.html" title="accordion-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/accordion.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/accordion.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/accordion.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/accordion.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/accordion.js"></script>
```

**Copy the markup from                                  into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. root `data-type="single|multiple"`; each `accordion-trigger` carries `aria-expanded` + `data-state="open|closed"` and `aria-controls` → its `accordion-content`, which is `hidden` while closed (kept in the DOM — radix unmounts it). Keys: arrows / Home / End move focus between triggers (rtl-aware); Enter / Space / click toggles.

Each trigger dispatches `shadless:open` / `shadless:close` (a sibling closed by single mode gets its own `close`), bubbling, after the state change, whichever path caused it.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/accordion.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
accordion
├── accordion-item
│   ├── accordion-trigger
│   └── accordion-content
└── accordion-item
    ├── accordion-trigger
    └── accordion-content
```

## Basic

A basic accordion that shows one item at a time. The first item is open by default.

<iframe class="demo" src="/demos/accordion-basic.html" title="accordion-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Multiple

Use `type="multiple"` to allow multiple items to be open at the same time.

<iframe class="demo" src="/demos/accordion-multiple.html" title="accordion-multiple" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Disabled

Use the `disabled` prop on `AccordionItem` to disable individual items.

<iframe class="demo" src="/demos/accordion-disabled.html" title="accordion-disabled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Borders

Add `border` to the `Accordion` and `border-b last:border-b-0` to the `AccordionItem` to add borders to the items.

<iframe class="demo" src="/demos/accordion-borders.html" title="accordion-borders" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Card

Wrap the `Accordion` in a `Card` component.

<iframe class="demo" src="/demos/accordion-card.html" title="accordion-card" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/accordion-rtl.html" title="accordion-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="accordion"` |
| `data-slot="accordion-item"` |
| `data-slot="accordion-trigger"` |
| `data-slot="accordion-trigger-icon"` |
| `data-slot="accordion-content"` |

**Runtime:** root `data-type="single|multiple"`; each `accordion-trigger` carries `aria-expanded` + `data-state="open|closed"` and `aria-controls` → its `accordion-content`, which is `hidden` while closed (kept in the DOM — radix unmounts it). Keys: arrows / Home / End move focus between triggers (rtl-aware); Enter / Space / click toggles. Each trigger dispatches `shadless:open` / `shadless:close` (a sibling closed by single mode gets its own `close`). No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix UI](https://www.radix-ui.com/primitives/docs/components/accordion#api-reference) documentation for more information.
