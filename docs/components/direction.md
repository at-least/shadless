---
title: "Direction"
description: "A provider component that sets the text direction for your application."
---

# Direction

A provider component that sets the text direction for your application.

<p class="page-links">[doc](https://www.radix-ui.com/primitives/docs/utilities/direction-provider) · [api](https://www.radix-ui.com/primitives/docs/utilities/direction-provider#api-reference)</p>

The `DirectionProvider` component is used to set the text direction (`ltr` or `rtl`) for your application. This is essential for supporting right-to-left languages like Arabic, Hebrew, and Persian.

Here's a preview of the component in RTL mode. Use the language selector to switch the language. To see more examples, look for the RTL section on components pages.

<iframe class="demo" src="/demos/card-rtl.html" title="card-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

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
| `dist/components/direction.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                  into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/direction.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## useDirection

The `useDirection` hook is used to get the current direction of the application.
