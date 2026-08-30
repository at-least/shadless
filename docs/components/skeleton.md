---
title: "Skeleton"
description: "Use to show a placeholder while content is loading."
---

# Skeleton

Use to show a placeholder while content is loading.

<iframe class="demo" src="/demos/skeleton-demo.html" title="skeleton-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/skeleton.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/skeleton.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/skeleton.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                 into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/skeleton.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Avatar

<iframe class="demo" src="/demos/skeleton-avatar.html" title="skeleton-avatar" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Card

<iframe class="demo" src="/demos/skeleton-card.html" title="skeleton-card" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Text

<iframe class="demo" src="/demos/skeleton-text.html" title="skeleton-text" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Form

<iframe class="demo" src="/demos/skeleton-form.html" title="skeleton-form" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Table

<iframe class="demo" src="/demos/skeleton-table.html" title="skeleton-table" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/skeleton-rtl.html" title="skeleton-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>
