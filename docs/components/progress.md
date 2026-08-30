---
title: "Progress"
description: "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar."
---

# Progress

Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/progress) · [api](https://www.radix-ui.com/docs/primitives/components/progress#api-reference)</p>

<iframe class="demo" src="/demos/progress-demo.html" title="progress-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/progress.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/progress.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/progress.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
```

**Copy the markup from                                 into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/progress.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Label

Use a `Field` component to add a label to the progress bar.

<iframe class="demo" src="/demos/progress-label.html" title="progress-label" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Controlled

A progress bar that can be controlled by a slider.

<iframe class="demo" src="/demos/progress-controlled.html" title="progress-controlled" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/progress-rtl.html" title="progress-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="progress"` |
| `data-slot="progress-indicator"` |

See the [Radix UI Progress](https://www.radix-ui.com/docs/primitives/components/progress#api-reference) documentation.
