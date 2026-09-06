---
title: "Slider"
description: "An input where the user selects a value from within a given range."
weight: 43
---

# Slider

An input where the user selects a value from within a given range.

{% <demo name="slider-demo" status="existing-dist"> %}
<iframe class="demo" src="/demos/slider.html" title="slider-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/slider.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=slider.html
<div>
  <span
    dir="ltr"
    data-orientation="horizontal"
    aria-disabled="false"
    data-slot="slider"
    class="data-vertical:min-h-40 relative flex w-full touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col"
    id="s1"
    style="--radix-slider-thumb-transform: translateX(-50%)"
    ><span
      data-orientation="horizontal"
      data-slot="slider-track"
      class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
      ><span
        data-orientation="horizontal"
        data-slot="slider-range"
        class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
        style="left: 0%; right: 50%"
      ></span></span
    ><span
      style="
        transform: var(--radix-slider-thumb-transform);
        position: absolute;
        left: calc(50% + 0px);
      "
      ><span
        role="slider"
        aria-valuemin="0"
        aria-valuemax="100"
        aria-orientation="horizontal"
        data-orientation="horizontal"
        tabindex="0"
        data-slot="slider-thumb"
        class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
        aria-valuenow="50"
        style=""
      ></span></span
  ></span>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/slider.js
// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Installation

**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/slider.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/slider.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/slider.js` | this component's behavior — registers with the base |

**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/slider.js"></script>
```

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="slider">` | no ids, no templates: the glue wires every root it finds |

**From code:** `shadless.get(rootEl)` → `values()`, `setValue(value, index)`; the root dispatches `shadless:change` (`detail: { values }`, live) and `shadless:commit` (once per gesture). `shadless.get` accepts an element or a selector and walks up from any element inside the instance.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Range

Use an array with two values for a range slider.

{% <demo name="slider-range" status="authored"> %}
<iframe class="demo" src="/demos/slider-range.html" title="slider-range" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/slider-range.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=slider-range.html
<span
  dir="ltr"
  data-orientation="horizontal"
  aria-disabled="false"
  data-slot="slider"
  class="data-vertical:min-h-40 relative flex touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col mx-auto w-full max-w-xs"
  style="--radix-slider-thumb-transform: translateX(-50%)"
  ><span
    data-orientation="horizontal"
    data-slot="slider-track"
    class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
    ><span
      data-orientation="horizontal"
      data-slot="slider-range"
      class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
      style="left: 25%; right: 50%"
    ></span></span
  ><span
    style="
      transform: var(--radix-slider-thumb-transform);
      position: absolute;
      left: calc(25% + 0px);
    "
    ><span
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-orientation="horizontal"
      data-orientation="horizontal"
      tabindex="0"
      data-slot="slider-thumb"
      class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
      data-radix-collection-item=""
      aria-label="Minimum"
      aria-valuenow="25"
      style=""
    ></span></span
  ><span
    style="
      transform: var(--radix-slider-thumb-transform);
      position: absolute;
      left: calc(50% + 0px);
    "
    ><span
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-orientation="horizontal"
      data-orientation="horizontal"
      tabindex="0"
      data-slot="slider-thumb"
      class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
      data-radix-collection-item=""
      aria-label="Maximum"
      aria-valuenow="50"
      style=""
    ></span></span
></span>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/slider.js
// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Multiple Thumbs

Use an array with multiple values for multiple thumbs.

{% <demo name="slider-multiple" status="authored"> %}
<iframe class="demo" src="/demos/slider-multiple.html" title="slider-multiple" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/slider-multiple.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=slider-multiple.html
<span
  dir="ltr"
  data-orientation="horizontal"
  aria-disabled="false"
  data-slot="slider"
  class="data-vertical:min-h-40 relative flex touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col mx-auto w-full max-w-xs"
  style="--radix-slider-thumb-transform: translateX(-50%)"
  ><span
    data-orientation="horizontal"
    data-slot="slider-track"
    class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
    ><span
      data-orientation="horizontal"
      data-slot="slider-range"
      class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
      style="left: 10%; right: 30%"
    ></span></span
  ><span
    style="
      transform: var(--radix-slider-thumb-transform);
      position: absolute;
      left: calc(10% + 0px);
    "
    ><span
      role="slider"
      aria-label="Value 1 of 3"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-orientation="horizontal"
      data-orientation="horizontal"
      tabindex="0"
      data-slot="slider-thumb"
      class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
      data-radix-collection-item=""
      aria-valuenow="10"
      style=""
    ></span></span
  ><span
    style="
      transform: var(--radix-slider-thumb-transform);
      position: absolute;
      left: calc(20% + 0px);
    "
    ><span
      role="slider"
      aria-label="Value 2 of 3"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-orientation="horizontal"
      data-orientation="horizontal"
      tabindex="0"
      data-slot="slider-thumb"
      class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
      data-radix-collection-item=""
      aria-valuenow="20"
      style=""
    ></span></span
  ><span
    style="
      transform: var(--radix-slider-thumb-transform);
      position: absolute;
      left: calc(70% + 0px);
    "
    ><span
      role="slider"
      aria-label="Value 3 of 3"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-orientation="horizontal"
      data-orientation="horizontal"
      tabindex="0"
      data-slot="slider-thumb"
      class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
      data-radix-collection-item=""
      aria-valuenow="70"
      style=""
    ></span></span
></span>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/slider.js
// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Vertical

Use `orientation="vertical"` for a vertical slider.

{% <demo name="slider-vertical" status="authored"> %}
<iframe class="demo" src="/demos/slider-vertical.html" title="slider-vertical" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/slider-vertical.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=slider-vertical.html
<div class="mx-auto flex w-full max-w-xs items-center justify-center gap-6">
  <span
    data-orientation="vertical"
    aria-disabled="false"
    data-slot="slider"
    class="data-vertical:min-h-40 relative flex w-full touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col h-40"
    style="--radix-slider-thumb-transform: translateY(50%)"
    ><span
      data-orientation="vertical"
      data-slot="slider-track"
      class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
      ><span
        data-orientation="vertical"
        data-slot="slider-range"
        class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
        style="bottom: 0%; top: 50%"
      ></span></span
    ><span
      style="
        transform: var(--radix-slider-thumb-transform);
        position: absolute;
        bottom: calc(50% + 0px);
      "
      ><span
        role="slider"
        aria-valuemin="0"
        aria-valuemax="100"
        aria-orientation="vertical"
        data-orientation="vertical"
        tabindex="0"
        data-slot="slider-thumb"
        class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
        aria-valuenow="50"
        style=""
      ></span></span></span
  ><span
    data-orientation="vertical"
    aria-disabled="false"
    data-slot="slider"
    class="data-vertical:min-h-40 relative flex w-full touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col h-40"
    style="--radix-slider-thumb-transform: translateY(50%)"
    ><span
      data-orientation="vertical"
      data-slot="slider-track"
      class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
      ><span
        data-orientation="vertical"
        data-slot="slider-range"
        class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
        style="bottom: 0%; top: 75%"
      ></span></span
    ><span
      style="
        transform: var(--radix-slider-thumb-transform);
        position: absolute;
        bottom: calc(25% + 0px);
      "
      ><span
        role="slider"
        aria-valuemin="0"
        aria-valuemax="100"
        aria-orientation="vertical"
        data-orientation="vertical"
        tabindex="0"
        data-slot="slider-thumb"
        class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
        aria-valuenow="25"
        style=""
      ></span></span
  ></span>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/slider.js
// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Controlled

{% <demo name="slider-controlled" status="authored"> %}
<iframe class="demo" src="/demos/slider-controlled.html" title="slider-controlled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/slider-controlled.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=slider-controlled.html
<div class="mx-auto grid w-full max-w-xs gap-3">
  <div class="flex items-center justify-between gap-2">
    <label
      data-slot="label"
      class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
      for="slider-demo-temperature"
      >Temperature</label
    ><span class="text-sm text-muted-foreground">0.3, 0.7</span>
  </div>
  <span
    dir="ltr"
    data-orientation="horizontal"
    aria-disabled="false"
    data-slot="slider"
    class="data-vertical:min-h-40 relative flex w-full touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col"
    id="slider-demo-temperature"
    style="--radix-slider-thumb-transform: translateX(-50%)"
    ><span
      data-orientation="horizontal"
      data-slot="slider-track"
      class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
      ><span
        data-orientation="horizontal"
        data-slot="slider-range"
        class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
        style="left: 30%; right: 30%"
      ></span></span
    ><span
      style="
        transform: var(--radix-slider-thumb-transform);
        position: absolute;
        left: calc(30% + 0px);
      "
      ><span
        role="slider"
        aria-valuemin="0"
        aria-valuemax="1"
        aria-orientation="horizontal"
        data-orientation="horizontal"
        tabindex="0"
        data-slot="slider-thumb"
        class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
        aria-label="Minimum"
        aria-valuenow="0.3"
        style=""
      ></span></span
    ><span
      style="
        transform: var(--radix-slider-thumb-transform);
        position: absolute;
        left: calc(70% + 0px);
      "
      ><span
        role="slider"
        aria-valuemin="0"
        aria-valuemax="1"
        aria-orientation="horizontal"
        data-orientation="horizontal"
        tabindex="0"
        data-slot="slider-thumb"
        class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
        aria-label="Maximum"
        aria-valuenow="0.7"
        style=""
      ></span></span
  ></span>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/slider.js
// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Disabled

Use the `disabled` prop to disable the slider.

{% <demo name="slider-disabled" status="authored"> %}
<iframe class="demo" src="/demos/slider-disabled.html" title="slider-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/slider-disabled.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=slider-disabled.html
<span
  dir="ltr"
  data-orientation="horizontal"
  aria-disabled="true"
  data-disabled=""
  data-slot="slider"
  class="data-vertical:min-h-40 relative flex touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col mx-auto w-full max-w-xs"
  style="--radix-slider-thumb-transform: translateX(-50%)"
  ><span
    data-disabled=""
    data-orientation="horizontal"
    data-slot="slider-track"
    class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
    ><span
      data-orientation="horizontal"
      data-disabled=""
      data-slot="slider-range"
      class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
      style="left: 0%; right: 50%"
    ></span></span
  ><span
    style="
      transform: var(--radix-slider-thumb-transform);
      position: absolute;
      left: calc(50% + 0px);
    "
    ><span
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-orientation="horizontal"
      data-orientation="horizontal"
      data-disabled=""
      data-slot="slider-thumb"
      class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
      data-radix-collection-item=""
      aria-valuenow="50"
      style=""
    ></span></span
></span>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/slider.js
// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

{% <demo name="slider-rtl" status="authored"> %}
<iframe class="demo" src="/demos/slider-rtl.html" title="slider-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/slider-rtl.html">Open the demo page</a> · <a href="/demos/slider-rtl-he.html">HE</a> · <a href="/demos/slider-rtl-en.html">EN</a></p>

{% <codegroup> %}
```text,name=slider-rtl.html
<span
  dir="rtl"
  data-orientation="horizontal"
  aria-disabled="false"
  data-slot="slider"
  class="data-vertical:min-h-40 relative flex touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col mx-auto w-full max-w-xs"
  style="--radix-slider-thumb-transform: translateX(-50%)"
  ><span
    data-orientation="horizontal"
    data-slot="slider-track"
    class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
    ><span
      data-orientation="horizontal"
      data-slot="slider-range"
      class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
      style="right: 0%; left: 25%"
    ></span></span
  ><span
    style="
      transform: var(--radix-slider-thumb-transform);
      position: absolute;
      right: calc(75% + 0px);
    "
    ><span
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-orientation="horizontal"
      data-orientation="horizontal"
      tabindex="0"
      data-slot="slider-thumb"
      class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
      data-radix-collection-item=""
      aria-valuenow="75"
      style=""
    ></span></span
></span>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/slider.js
// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="slider"` |
| `data-slot="slider-track"` |
| `data-slot="slider-range"` |
| `data-slot="slider-thumb"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `values()`, `setValue(value, index)`; the root dispatches `shadless:change` (live) and `shadless:commit` (once per gesture) with `detail: { values }`; a `name` attribute submits one input per thumb. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Slider](https://www.radix-ui.com/primitives/docs/components/slider#api-reference) documentation.
