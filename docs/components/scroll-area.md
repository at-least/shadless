---
title: "Scroll Area"
description: "Augments native scroll functionality for custom, cross-browser styling."
---

# Scroll Area

Augments native scroll functionality for custom, cross-browser styling.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/scroll-area" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/scroll-area#api-reference" rel="noopener">api</a></p>

<iframe class="demo" src="/demos/scroll-area.html" title="scroll-area-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [scroll-area.html]
<div>
<div dir="ltr" data-slot="scroll-area" class="relative" style="position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px; height: 200px; width: 200px;"><style>[data-radix-scroll-area-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-scroll-area-viewport]::-webkit-scrollbar{display:none}</style><div data-radix-scroll-area-viewport="" data-slot="scroll-area-viewport" class="size-full rounded-[inherit] transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1" style="overflow: hidden scroll;"><div style="min-width: 100%; display: table;"><div style="height: 20px; line-height: 19px;">1</div><div style="height: 20px; line-height: 19px;">2</div><div style="height: 20px; line-height: 19px;">3</div><div style="height: 20px; line-height: 19px;">4</div><div style="height: 20px; line-height: 19px;">5</div><div style="height: 20px; line-height: 19px;">6</div><div style="height: 20px; line-height: 19px;">7</div><div style="height: 20px; line-height: 19px;">8</div><div style="height: 20px; line-height: 19px;">9</div><div style="height: 20px; line-height: 19px;">10</div><div style="height: 20px; line-height: 19px;">11</div><div style="height: 20px; line-height: 19px;">12</div><div style="height: 20px; line-height: 19px;">13</div><div style="height: 20px; line-height: 19px;">14</div><div style="height: 20px; line-height: 19px;">15</div><div style="height: 20px; line-height: 19px;">16</div><div style="height: 20px; line-height: 19px;">17</div><div style="height: 20px; line-height: 19px;">18</div><div style="height: 20px; line-height: 19px;">19</div><div style="height: 20px; line-height: 19px;">20</div></div></div><div data-orientation="vertical" data-state="visible" data-slot="scroll-area-scrollbar" class="data-horizontal:h-2.5 data-horizontal:flex-col data-horizontal:border-t data-horizontal:border-t-transparent data-vertical:h-full data-vertical:w-2.5 data-vertical:border-l data-vertical:border-l-transparent flex touch-none p-px transition-colors select-none" style="position: absolute; top: 0px; right: 0px; bottom: var(--radix-scroll-area-corner-height); --radix-scroll-area-thumb-height: 100px;"><div data-state="visible" data-slot="scroll-area-thumb" class="rounded-full relative flex-1 bg-border" style="width: var(--radix-scroll-area-thumb-width); height: var(--radix-scroll-area-thumb-height); transform: translate3d(0px, 0px, 0px);"></div></div></div>
</div>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/scroll-area.js
// shadless scroll-area behavior (wireScrollArea, type=hover default) — registers with the base; multi-instance
(function () {
  shadless.register("scroll-area", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=scroll-area]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var viewport = root.querySelector("[data-slot=scroll-area-viewport]");
      if (!viewport) return;
      var scrollbars = {};
      Array.prototype.forEach.call(root.querySelectorAll("[data-slot=scroll-area-scrollbar]"), function (bar) {
        var o = bar.getAttribute("data-orientation") === "horizontal" ? "horizontal" : "vertical";
        scrollbars[o] = { scrollbar: bar, thumb: bar.querySelector("[data-slot=scroll-area-thumb]") };
      });
      var wired = RadixKernel.wireScrollArea({
        root: root,
        viewport: viewport,
        content: viewport.firstElementChild,
        scrollbars: scrollbars,
      });
      w.teardown = function () { if (wired && wired.destroy) wired.destroy() };
    });
  } })
})()
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/scroll-area.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/scroll-area.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/scroll-area.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/scroll-area.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/scroll-area.js"></script>
```

**Copy the markup from                                    into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="scroll-area">` | no ids, no templates: the glue wires every root it finds |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/scroll-area.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
scroll-area
└── scroll-area-scrollbar
```

## Horizontal

Use `ScrollBar` with `orientation="horizontal"` for horizontal scrolling.

<iframe class="demo" src="/demos/scroll-area-horizontal-demo.html" title="scroll-area-horizontal-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [scroll-area-horizontal-demo.html]
<div dir="ltr" data-slot="scroll-area" class="relative w-96 rounded-md border whitespace-nowrap" style="position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;"><style>[data-radix-scroll-area-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-scroll-area-viewport]::-webkit-scrollbar{display:none}</style><div data-radix-scroll-area-viewport="" data-slot="scroll-area-viewport" class="size-full rounded-[inherit] transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1" style="overflow: scroll;"><div style="min-width: 100%; display: table;"><div class="flex w-max space-x-4 p-4"><figure class="shrink-0"><div class="overflow-hidden rounded-md"><img alt="Photo by Ornella Binni" width="300" height="400" data-nimg="1" decoding="async" loading="lazy" class="aspect-[3/4] h-fit w-fit object-cover" src="https://images.unsplash.com/photo-1465869185982-5a1a7522cbcb?auto=format&amp;fit=crop&amp;w=300&amp;q=80" style="color: transparent;"></div><figcaption class="pt-2 text-xs text-muted-foreground">Photo by <span class="font-semibold text-foreground">Ornella Binni</span></figcaption></figure><figure class="shrink-0"><div class="overflow-hidden rounded-md"><img alt="Photo by Tom Byrom" width="300" height="400" data-nimg="1" decoding="async" loading="lazy" class="aspect-[3/4] h-fit w-fit object-cover" src="https://images.unsplash.com/photo-1548516173-3cabfa4607e9?auto=format&amp;fit=crop&amp;w=300&amp;q=80" style="color: transparent;"></div><figcaption class="pt-2 text-xs text-muted-foreground">Photo by <span class="font-semibold text-foreground">Tom Byrom</span></figcaption></figure><figure class="shrink-0"><div class="overflow-hidden rounded-md"><img alt="Photo by Vladimir Malyavko" width="300" height="400" data-nimg="1" decoding="async" loading="lazy" class="aspect-[3/4] h-fit w-fit object-cover" src="https://images.unsplash.com/photo-1494337480532-3725c85fd2ab?auto=format&amp;fit=crop&amp;w=300&amp;q=80" style="color: transparent;"></div><figcaption class="pt-2 text-xs text-muted-foreground">Photo by <span class="font-semibold text-foreground">Vladimir Malyavko</span></figcaption></figure></div></div></div></div>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/scroll-area.js
// shadless scroll-area behavior (wireScrollArea, type=hover default) — registers with the base; multi-instance
(function () {
  shadless.register("scroll-area", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=scroll-area]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var viewport = root.querySelector("[data-slot=scroll-area-viewport]");
      if (!viewport) return;
      var scrollbars = {};
      Array.prototype.forEach.call(root.querySelectorAll("[data-slot=scroll-area-scrollbar]"), function (bar) {
        var o = bar.getAttribute("data-orientation") === "horizontal" ? "horizontal" : "vertical";
        scrollbars[o] = { scrollbar: bar, thumb: bar.querySelector("[data-slot=scroll-area-thumb]") };
      });
      var wired = RadixKernel.wireScrollArea({
        root: root,
        viewport: viewport,
        content: viewport.firstElementChild,
        scrollbars: scrollbars,
      });
      w.teardown = function () { if (wired && wired.destroy) wired.destroy() };
    });
  } })
})()
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/scroll-area-rtl.html" title="scroll-area-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

:::: details Source

::: code-group
```text [scroll-area-rtl.html]
<div dir="rtl" data-slot="scroll-area" class="relative h-72 w-48 rounded-md border" style="position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;"><style>[data-radix-scroll-area-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-scroll-area-viewport]::-webkit-scrollbar{display:none}</style><div data-radix-scroll-area-viewport="" data-slot="scroll-area-viewport" class="size-full rounded-[inherit] transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1" style="overflow: hidden scroll;"><div style="min-width: 100%; display: table;"><div class="p-4"><h4 class="mb-4 text-sm leading-none font-medium">العلامات</h4><div class="text-sm">v1.2.0-beta.50</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.49</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.48</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.47</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.46</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.45</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.44</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.43</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.42</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.41</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.40</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.39</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.38</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.37</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.36</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.35</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.34</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.33</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.32</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.31</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.30</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.29</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.28</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.27</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.26</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.25</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.24</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.23</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.22</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.21</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.20</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.19</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.18</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.17</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.16</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.15</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.14</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.13</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.12</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.11</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.10</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.9</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.8</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.7</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.6</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.5</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.4</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.3</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.2</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div><div class="text-sm">v1.2.0-beta.1</div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch my-2"></div></div></div></div></div>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/scroll-area.js
// shadless scroll-area behavior (wireScrollArea, type=hover default) — registers with the base; multi-instance
(function () {
  shadless.register("scroll-area", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=scroll-area]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var viewport = root.querySelector("[data-slot=scroll-area-viewport]");
      if (!viewport) return;
      var scrollbars = {};
      Array.prototype.forEach.call(root.querySelectorAll("[data-slot=scroll-area-scrollbar]"), function (bar) {
        var o = bar.getAttribute("data-orientation") === "horizontal" ? "horizontal" : "vertical";
        scrollbars[o] = { scrollbar: bar, thumb: bar.querySelector("[data-slot=scroll-area-thumb]") };
      });
      var wired = RadixKernel.wireScrollArea({
        root: root,
        viewport: viewport,
        content: viewport.firstElementChild,
        scrollbars: scrollbars,
      });
      w.teardown = function () { if (wired && wired.destroy) wired.destroy() };
    });
  } })
})()
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="scroll-area"` |
| `data-slot="scroll-area-viewport"` |
| `data-slot="scroll-area-scrollbar"` |
| `data-slot="scroll-area-thumb"` |

**Runtime:** wired from `data-slot` alone — no handle, no events; see Installation → Behavior protocol.

See the [Radix UI Scroll Area](https://www.radix-ui.com/docs/primitives/components/scroll-area#api-reference) documentation.
