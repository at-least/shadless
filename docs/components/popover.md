---
title: "Popover"
description: "Displays rich content in a portal, triggered by a button."
---

# Popover

Displays rich content in a portal, triggered by a button.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/popover" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/popover#api-reference" rel="noopener">api</a></p>

::::demo popover-demo
<iframe class="demo" src="/demos/popover.html" title="popover-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/popover.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [popover.html]
<div>
  <button
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="p1"
    data-state="closed"
    data-slot="popover-trigger"
    id="p1-trigger"
  >
    Open popover
  </button>
</div>
<template id="p1-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 100.484375px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      role="dialog"
      id="p1"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="p1"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium">Dimensions</div>
        <p data-slot="popover-description" class="text-muted-foreground">Set the dimensions.</p>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/popover.js
// shadless popover behavior (wirePopover) — registers with the base; multi-instance: every
// [data-slot=popover-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("popover", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=popover-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, content = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        if (content) content.setAttribute("data-state", s);
      }
      function mount() {
        var frag = tpl.content.cloneNode(true);
        var host = document.createElement("div");
        host.appendChild(frag);
        content = host.querySelector("[data-slot=popover-content]");
        document.body.appendChild(content);
        setState("open");
        handles = RadixKernel.wirePopover({
          content: content,
          trigger: trigger,
          contentStateFlip: true,
          onClosed: function () {
            setState("closed");
            if (content) content.remove();
            content = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "popover");
          },
        });
        open = true;
        shadless.h.emit(trigger, "open", "popover");
      }
      function unmount(restoreFocus) { if (handles) handles.close(restoreFocus !== false); }
      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "popover",
        open: function () { if (!open) mount() },
        close: function (restoreFocus) { if (open) unmount(restoreFocus) },
        toggle: function () { open ? unmount() : mount() },
        isOpen: function () { return open },
      })
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
@import "shadless/popover.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/popover.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/popover.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/popover.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/popover.js"></script>
```

**Copy the markup from                                into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="popover-trigger" id="<k>-trigger">` | opens on clicking; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="popover-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/popover.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
popover
├── popover-trigger
└── popover-content
```

## Basic

A simple popover with a header, title, and description.

::::demo popover-basic
<iframe class="demo" src="/demos/popover-basic.html" title="popover-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/popover-basic.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [popover-basic.html]
<button
  data-slot="popover-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  aria-haspopup="dialog"
  aria-expanded="false"
  aria-controls="k0"
  data-state="closed"
  id="k0-trigger"
>
  Open Popover
</button>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(8px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 0% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 101.953125px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="start"
      data-state="open"
      role="dialog"
      id="k0"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium">Dimensions</div>
        <p data-slot="popover-description" class="text-muted-foreground">
          Set the dimensions for the layer.
        </p>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/popover.js
// shadless popover behavior (wirePopover) — registers with the base; multi-instance: every
// [data-slot=popover-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("popover", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=popover-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, content = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        if (content) content.setAttribute("data-state", s);
      }
      function mount() {
        var frag = tpl.content.cloneNode(true);
        var host = document.createElement("div");
        host.appendChild(frag);
        content = host.querySelector("[data-slot=popover-content]");
        document.body.appendChild(content);
        setState("open");
        handles = RadixKernel.wirePopover({
          content: content,
          trigger: trigger,
          contentStateFlip: true,
          onClosed: function () {
            setState("closed");
            if (content) content.remove();
            content = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "popover");
          },
        });
        open = true;
        shadless.h.emit(trigger, "open", "popover");
      }
      function unmount(restoreFocus) { if (handles) handles.close(restoreFocus !== false); }
      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "popover",
        open: function () { if (!open) mount() },
        close: function (restoreFocus) { if (open) unmount(restoreFocus) },
        toggle: function () { open ? unmount() : mount() },
        isOpen: function () { return open },
      })
    });
  } })
})()
```
:::

::::


## Align

Use the `align` prop on `PopoverContent` to control the horizontal alignment.

::::demo popover-alignments
<iframe class="demo" src="/demos/popover-alignments.html" title="popover-alignments" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/popover-alignments.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [popover-alignments.html]
<div class="flex gap-6">
  <button
    data-slot="popover-trigger"
    data-variant="outline"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="k0"
    data-state="closed"
    id="k0-trigger"
  >
    Start</button
  ><button
    data-slot="popover-trigger"
    data-variant="outline"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="k1"
    data-state="closed"
    id="k1-trigger"
  >
    Center</button
  ><button
    data-slot="popover-trigger"
    data-variant="outline"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="k2"
    data-state="closed"
    id="k2-trigger"
  >
    End
  </button>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(8px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 0% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 44.15625px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="start"
      data-state="open"
      role="dialog"
      id="k0"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-popover-content-transform-origin) outline-hidden w-40"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      Aligned to start
    </div>
  </div>
</template>
<template id="k1-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(25px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 56.015625px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      role="dialog"
      id="k1"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-popover-content-transform-origin) outline-hidden w-40"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k1"
    >
      Aligned to center
    </div>
  </div>
</template>
<template id="k2-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(53px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 100% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 39.71875px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="end"
      data-state="open"
      role="dialog"
      id="k2"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-popover-content-transform-origin) outline-hidden w-40"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k2"
    >
      Aligned to end
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/popover.js
// shadless popover behavior (wirePopover) — registers with the base; multi-instance: every
// [data-slot=popover-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("popover", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=popover-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, content = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        if (content) content.setAttribute("data-state", s);
      }
      function mount() {
        var frag = tpl.content.cloneNode(true);
        var host = document.createElement("div");
        host.appendChild(frag);
        content = host.querySelector("[data-slot=popover-content]");
        document.body.appendChild(content);
        setState("open");
        handles = RadixKernel.wirePopover({
          content: content,
          trigger: trigger,
          contentStateFlip: true,
          onClosed: function () {
            setState("closed");
            if (content) content.remove();
            content = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "popover");
          },
        });
        open = true;
        shadless.h.emit(trigger, "open", "popover");
      }
      function unmount(restoreFocus) { if (handles) handles.close(restoreFocus !== false); }
      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "popover",
        open: function () { if (!open) mount() },
        close: function (restoreFocus) { if (open) unmount(restoreFocus) },
        toggle: function () { open ? unmount() : mount() },
        isOpen: function () { return open },
      })
    });
  } })
})()
```
:::

::::


## With Form

A popover with form fields inside.

::::demo popover-form
<iframe class="demo" src="/demos/popover-form.html" title="popover-form" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/popover-form.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [popover-form.html]
<button
  data-slot="popover-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  aria-haspopup="dialog"
  aria-expanded="false"
  aria-controls="k0"
  data-state="closed"
  id="k0-trigger"
>
  Open Popover
</button>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(8px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 0% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 101.953125px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="start"
      data-state="open"
      role="dialog"
      id="k0"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-popover-content-transform-origin) outline-hidden w-64"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium">Dimensions</div>
        <p data-slot="popover-description" class="text-muted-foreground">
          Set the dimensions for the layer.
        </p>
      </div>
      <div
        data-slot="field-group"
        class="data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col gap-4"
      >
        <div
          role="group"
          data-slot="field"
          data-orientation="horizontal"
          class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
        >
          <label
            data-slot="field-label"
            class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col w-1/2"
            for="width"
            >Width</label
          ><input
            data-slot="input"
            class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
            id="width"
            value="100%"
          />
        </div>
        <div
          role="group"
          data-slot="field"
          data-orientation="horizontal"
          class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
        >
          <label
            data-slot="field-label"
            class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col w-1/2"
            for="height"
            >Height</label
          ><input
            data-slot="input"
            class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
            id="height"
            value="25px"
          />
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/popover.js
// shadless popover behavior (wirePopover) — registers with the base; multi-instance: every
// [data-slot=popover-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("popover", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=popover-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, content = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        if (content) content.setAttribute("data-state", s);
      }
      function mount() {
        var frag = tpl.content.cloneNode(true);
        var host = document.createElement("div");
        host.appendChild(frag);
        content = host.querySelector("[data-slot=popover-content]");
        document.body.appendChild(content);
        setState("open");
        handles = RadixKernel.wirePopover({
          content: content,
          trigger: trigger,
          contentStateFlip: true,
          onClosed: function () {
            setState("closed");
            if (content) content.remove();
            content = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "popover");
          },
        });
        open = true;
        shadless.h.emit(trigger, "open", "popover");
      }
      function unmount(restoreFocus) { if (handles) handles.close(restoreFocus !== false); }
      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "popover",
        open: function () { if (!open) mount() },
        close: function (restoreFocus) { if (open) unmount(restoreFocus) },
        toggle: function () { open ? unmount() : mount() },
        isOpen: function () { return open },
      })
    });
  } })
})()
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo popover-rtl
<iframe class="demo" src="/demos/popover-rtl.html" title="popover-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/popover-rtl.html">Open the demo page</a> · <a href="/demos/popover-rtl-he.html">HE</a> · <a href="/demos/popover-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [popover-rtl.html]
<div class="flex flex-wrap justify-center gap-2">
  <button
    data-slot="popover-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="k0"
    data-state="closed"
    id="k0-trigger"
  >
    يسار</button
  ><button
    data-slot="popover-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="k1"
    data-state="closed"
    id="k1-trigger"
  >
    أعلى</button
  ><button
    data-slot="popover-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="k2"
    data-state="closed"
    id="k2-trigger"
  >
    أسفل</button
  ><button
    data-slot="popover-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="k3"
    data-state="closed"
    id="k3-trigger"
  >
    يمين
  </button>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    dir="rtl"
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(1098px, 0px);
      min-width: max-content;
      --radix-popper-transform-origin: 128px 50%;
      z-index: auto;
      --radix-popper-available-width: 1226px;
      --radix-popper-available-height: 720px;
      --radix-popper-anchor-width: 42px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="left"
      data-align="center"
      data-state="open"
      role="dialog"
      id="k0"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden"
      dir="rtl"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium">الأبعاد</div>
        <p data-slot="popover-description" class="text-muted-foreground">تعيين الأبعاد للطبقة.</p>
      </div>
    </div>
  </div>
</template>
<template id="k1-portal">
  <div
    data-radix-popper-content-wrapper=""
    dir="rtl"
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(1145px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 43px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      role="dialog"
      id="k1"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden"
      dir="rtl"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k1"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium">الأبعاد</div>
        <p data-slot="popover-description" class="text-muted-foreground">تعيين الأبعاد للطبقة.</p>
      </div>
    </div>
  </div>
</template>
<template id="k2-portal">
  <div
    data-radix-popper-content-wrapper=""
    dir="rtl"
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(1099px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 48px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      role="dialog"
      id="k2"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden"
      dir="rtl"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k2"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium">الأبعاد</div>
        <p data-slot="popover-description" class="text-muted-foreground">تعيين الأبعاد للطبقة.</p>
      </div>
    </div>
  </div>
</template>
<template id="k3-portal">
  <div
    data-radix-popper-content-wrapper=""
    dir="rtl"
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(1143px, 0px);
      min-width: max-content;
      --radix-popper-transform-origin: 0px 50%;
      z-index: auto;
      --radix-popper-available-width: 137px;
      --radix-popper-available-height: 720px;
      --radix-popper-anchor-width: 42px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="right"
      data-align="center"
      data-state="open"
      role="dialog"
      id="k3"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden"
      dir="rtl"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k3"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium">الأبعاد</div>
        <p data-slot="popover-description" class="text-muted-foreground">تعيين الأبعاد للطبقة.</p>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/popover.js
// shadless popover behavior (wirePopover) — registers with the base; multi-instance: every
// [data-slot=popover-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("popover", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=popover-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, content = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        if (content) content.setAttribute("data-state", s);
      }
      function mount() {
        var frag = tpl.content.cloneNode(true);
        var host = document.createElement("div");
        host.appendChild(frag);
        content = host.querySelector("[data-slot=popover-content]");
        document.body.appendChild(content);
        setState("open");
        handles = RadixKernel.wirePopover({
          content: content,
          trigger: trigger,
          contentStateFlip: true,
          onClosed: function () {
            setState("closed");
            if (content) content.remove();
            content = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "popover");
          },
        });
        open = true;
        shadless.h.emit(trigger, "open", "popover");
      }
      function unmount(restoreFocus) { if (handles) handles.close(restoreFocus !== false); }
      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "popover",
        open: function () { if (!open) mount() },
        close: function (restoreFocus) { if (open) unmount(restoreFocus) },
        toggle: function () { open ? unmount() : mount() },
        isOpen: function () { return open },
      })
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
| `data-slot="popover"` |
| `data-slot="popover-trigger"` |
| `data-slot="popover-content"` |
| `data-slot="popover-anchor"` |
| `data-slot="popover-header"` |
| `data-slot="popover-title"` |
| `data-slot="popover-description"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Popover](https://www.radix-ui.com/docs/primitives/components/popover#api-reference) documentation.
