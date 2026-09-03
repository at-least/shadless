---
title: "Aspect Ratio"
description: "Displays content within a desired ratio."
---

# Aspect Ratio

Displays content within a desired ratio.

<p class="page-links"><a href="https://www.radix-ui.com/primitives/docs/components/aspect-ratio" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/primitives/docs/components/aspect-ratio#api-reference" rel="noopener">api</a></p>

::::demo aspect-ratio-demo
<iframe class="demo" src="/demos/aspect-ratio-demo.html" title="aspect-ratio-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/aspect-ratio-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [aspect-ratio-demo.html]
<div class="w-full max-w-sm">
  <div
    data-radix-aspect-ratio-wrapper=""
    style="position: relative; width: 100%; padding-bottom: 56.25%"
  >
    <div
      data-slot="aspect-ratio"
      class="rounded-lg bg-muted"
      style="position: absolute; inset: 0px"
    >
      <img
        alt="Photo"
        data-nimg="fill"
        sizes="100vw"
        decoding="async"
        loading="lazy"
        class="w-full rounded-lg object-cover grayscale dark:brightness-20"
        src="https://avatar.vercel.sh/shadcn1"
        style="inset: 0px; color: transparent; height: 100%; position: absolute; width: 100%"
      />
    </div>
  </div>
</div>
```
:::

::::


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
| `dist/components/aspect-ratio.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
```

**Copy the markup from `dist/components/aspect-ratio.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/aspect-ratio.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Square

A square aspect ratio component using the `ratio={1 / 1}` prop. This is useful for displaying images in a square format.

::::demo aspect-ratio-square
<iframe class="demo" src="/demos/aspect-ratio-square.html" title="aspect-ratio-square" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/aspect-ratio-square.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [aspect-ratio-square.html]
<div class="w-full max-w-[12rem]">
  <div
    data-radix-aspect-ratio-wrapper=""
    style="position: relative; width: 100%; padding-bottom: 100%"
  >
    <div
      data-slot="aspect-ratio"
      class="rounded-lg bg-muted"
      style="position: absolute; inset: 0px"
    >
      <img
        alt="Photo"
        data-nimg="fill"
        sizes="100vw"
        decoding="async"
        loading="lazy"
        class="rounded-lg object-cover grayscale dark:brightness-20"
        src="https://avatar.vercel.sh/shadcn1"
        style="inset: 0px; color: transparent; height: 100%; position: absolute; width: 100%"
      />
    </div>
  </div>
</div>
```
:::

::::


## Portrait

A portrait aspect ratio component using the `ratio={9 / 16}` prop. This is useful for displaying images in a portrait format.

::::demo aspect-ratio-portrait
<iframe class="demo" src="/demos/aspect-ratio-portrait.html" title="aspect-ratio-portrait" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/aspect-ratio-portrait.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [aspect-ratio-portrait.html]
<div class="w-full max-w-[10rem]">
  <div
    data-radix-aspect-ratio-wrapper=""
    style="position: relative; width: 100%; padding-bottom: 177.778%"
  >
    <div
      data-slot="aspect-ratio"
      class="rounded-lg bg-muted"
      style="position: absolute; inset: 0px"
    >
      <img
        alt="Photo"
        data-nimg="fill"
        sizes="100vw"
        decoding="async"
        loading="lazy"
        class="rounded-lg object-cover grayscale dark:brightness-20"
        src="https://avatar.vercel.sh/shadcn1"
        style="inset: 0px; color: transparent; height: 100%; position: absolute; width: 100%"
      />
    </div>
  </div>
</div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo aspect-ratio-rtl
<iframe class="demo" src="/demos/aspect-ratio-rtl.html" title="aspect-ratio-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/aspect-ratio-rtl.html">Open the demo page</a> · <a href="/demos/aspect-ratio-rtl-he.html">HE</a> · <a href="/demos/aspect-ratio-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [aspect-ratio-rtl.html]
<figure class="w-full max-w-sm" dir="rtl">
  <div
    data-radix-aspect-ratio-wrapper=""
    style="position: relative; width: 100%; padding-bottom: 56.25%"
  >
    <div
      data-slot="aspect-ratio"
      class="rounded-lg bg-muted"
      style="position: absolute; inset: 0px"
    >
      <img
        alt="Photo"
        data-nimg="fill"
        sizes="100vw"
        decoding="async"
        loading="lazy"
        class="w-full rounded-lg object-cover grayscale dark:brightness-20"
        src="https://avatar.vercel.sh/shadcn1"
        style="inset: 0px; color: transparent; height: 100%; position: absolute; width: 100%"
      />
    </div>
  </div>
  <figcaption class="mt-2 text-center text-sm text-muted-foreground">منظر طبيعي جميل</figcaption>
</figure>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="aspect-ratio"` |

**Runtime:** No `cva`-declared variants. Check `dist/css/aspect-ratio.css` for any `data-*` attribute this slot's styling depends on.
See Installation → Files this component needs for the JavaScript this component requires.
