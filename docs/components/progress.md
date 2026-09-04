---
title: "Progress"
description: "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar."
---

# Progress

Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.

::::demo progress-demo
<iframe class="demo" src="/demos/progress-demo.html" title="progress-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/progress-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [progress-demo.html]
<div
  aria-valuemax="100"
  aria-valuemin="0"
  role="progressbar"
  data-state="indeterminate"
  data-max="100"
  data-slot="progress"
  class="bg-muted h-1 rounded-full relative flex items-center overflow-x-hidden w-[60%]"
>
  <div
    data-state="indeterminate"
    data-max="100"
    data-slot="progress-indicator"
    class="bg-primary size-full flex-1 transition-all"
    style="transform: translateX(-87%)"
  ></div>
</div>
```
:::

::::

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
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |

**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
```

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Label

Use a `Field` component to add a label to the progress bar.

::::demo progress-label
<iframe class="demo" src="/demos/progress-label.html" title="progress-label" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/progress-label.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [progress-label.html]
<div
  role="group"
  data-slot="field"
  data-orientation="vertical"
  class="data-[invalid=true]:text-destructive gap-2 group/field flex flex-col *:w-full [&amp;&gt;.sr-only]:w-auto w-full max-w-sm"
>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    for="progress-upload"
    ><span>Upload progress</span><span class="ml-auto">66%</span></label
  >
  <div
    aria-valuemax="100"
    aria-valuemin="0"
    role="progressbar"
    data-state="indeterminate"
    data-max="100"
    data-slot="progress"
    class="bg-muted h-1 rounded-full relative flex w-full items-center overflow-x-hidden"
    id="progress-upload"
  >
    <div
      data-state="indeterminate"
      data-max="100"
      data-slot="progress-indicator"
      class="bg-primary size-full flex-1 transition-all"
      style="transform: translateX(-34%)"
    ></div>
  </div>
</div>
```
:::

::::

## Controlled

A progress bar that can be controlled by a slider.

::::demo progress-controlled
<iframe class="demo" src="/demos/progress-controlled.html" title="progress-controlled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/progress-controlled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [progress-controlled.html]
<div class="flex w-full max-w-sm flex-col gap-4">
  <div
    aria-valuemax="100"
    aria-valuemin="0"
    role="progressbar"
    data-state="indeterminate"
    data-max="100"
    data-slot="progress"
    class="bg-muted h-1 rounded-full relative flex w-full items-center overflow-x-hidden"
  >
    <div
      data-state="indeterminate"
      data-max="100"
      data-slot="progress-indicator"
      class="bg-primary size-full flex-1 transition-all"
      style="transform: translateX(-50%)"
    ></div>
  </div>
  <span
    dir="ltr"
    data-orientation="horizontal"
    aria-disabled="false"
    data-slot="slider"
    class="data-vertical:min-h-40 relative flex w-full touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col"
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

```js:line-numbers [behavior]
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
:::

::::

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo progress-rtl
<iframe class="demo" src="/demos/progress-rtl.html" title="progress-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/progress-rtl.html">Open the demo page</a> · <a href="/demos/progress-rtl-he.html">HE</a> · <a href="/demos/progress-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [progress-rtl.html]
<div
  role="group"
  data-slot="field"
  data-orientation="vertical"
  class="data-[invalid=true]:text-destructive gap-2 group/field flex flex-col *:w-full [&amp;&gt;.sr-only]:w-auto w-full max-w-sm"
  dir="rtl"
>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    for="progress-upload"
    ><span>تقدم الرفع</span><span class="ms-auto">٦٦%</span></label
  >
  <div
    aria-valuemax="100"
    aria-valuemin="0"
    role="progressbar"
    data-state="indeterminate"
    data-max="100"
    data-slot="progress"
    class="bg-muted h-1 rounded-full relative flex w-full items-center overflow-x-hidden rtl:rotate-180"
    id="progress-upload"
  >
    <div
      data-state="indeterminate"
      data-max="100"
      data-slot="progress-indicator"
      class="bg-primary size-full flex-1 transition-all"
      style="transform: translateX(-34%)"
    ></div>
  </div>
</div>
```
:::

::::

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="progress"` |
| `data-slot="progress-indicator"` |

See the [Radix UI Progress](https://www.radix-ui.com/primitives/docs/components/progress#api-reference) documentation.
