---
title: "Tooltip"
description: "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."
---

# Tooltip

A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.

::::demo tooltip-demo
<iframe class="demo" src="/demos/tooltip.html" title="tooltip-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tooltip.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tooltip.html]
<div>
  <button data-state="closed" data-slot="tooltip-trigger" id="t1-trigger">Hover me</button>
</div>
<template id="t1-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 34px);
      min-width: max-content;
      --radix-popper-transform-origin: 45.062525px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 686px;
      --radix-popper-anchor-width: 73.78125px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="t1"
    >
      Add to library<span
        style="
          position: absolute;
          transform: rotate(180deg);
          left: 40.0625px;
          top: 0px;
          transform-origin: center 0px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="t1-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >Add to library</span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var content = shadless.h.mountFromTemplate(tpl, "[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      shadless.h.bindHandlers(trigger, wired.handlers, w.signal);
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
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
@import "shadless/tooltip.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/tooltip.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/tooltip.js` | this component's behavior — registers with the base |

**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/tooltip.js"></script>
```

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="tooltip-trigger" id="<k>-trigger">` | opens on hovering; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="tooltip-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
tooltip
├── tooltip-trigger
└── tooltip-content
```

## Side

Use the `side` prop to change the position of the tooltip.

::::demo tooltip-sides
<iframe class="demo" src="/demos/tooltip-sides.html" title="tooltip-sides" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tooltip-sides.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tooltip-sides.html]
<div class="flex flex-wrap gap-2">
  <button
    data-slot="tooltip-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-fit capitalize"
    data-state="closed"
    id="k0-trigger"
  >
    left</button
  ><button
    data-slot="tooltip-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-fit capitalize"
    data-state="closed"
    id="k1-trigger"
  >
    top</button
  ><button
    data-slot="tooltip-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-fit capitalize"
    data-state="closed"
    id="k2-trigger"
  >
    bottom</button
  ><button
    data-slot="tooltip-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-fit capitalize"
    data-state="closed"
    id="k3-trigger"
  >
    right
  </button>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(47px, 0px);
      min-width: max-content;
      --radix-popper-transform-origin: -5px 18.5px;
      z-index: auto;
      --radix-popper-available-width: 1233.21875px;
      --radix-popper-available-height: 720px;
      --radix-popper-anchor-width: 33.78125px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="right"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      <p>Add to library</p>
      <span
        style="
          position: absolute;
          transform-origin: 0px 0px;
          transform: translateY(50%) rotate(90deg) translateX(-50%);
          left: 0px;
          top: 16px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k0-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        ><p>Add to library</p></span
      >
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
      transform: translate(14px, 34px);
      min-width: max-content;
      --radix-popper-transform-origin: 45.5px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 686px;
      --radix-popper-anchor-width: 34.53125px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k1"
    >
      <p>Add to library</p>
      <span
        style="
          position: absolute;
          transform: rotate(180deg);
          left: 40.5px;
          top: 0px;
          transform-origin: center 0px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k1-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        ><p>Add to library</p></span
      >
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
      transform: translate(59px, 34px);
      min-width: max-content;
      --radix-popper-transform-origin: 45.5px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 686px;
      --radix-popper-anchor-width: 56.75px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k2"
    >
      <p>Add to library</p>
      <span
        style="
          position: absolute;
          top: 0px;
          transform-origin: center 0px;
          transform: rotate(180deg);
          left: 40.5px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k2-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        ><p>Add to library</p></span
      >
    </div>
  </div>
</template>
<template id="k3-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(180px, 0px);
      min-width: max-content;
      --radix-popper-transform-origin: -5px 18.5px;
      z-index: auto;
      --radix-popper-available-width: 1100px;
      --radix-popper-available-height: 720px;
      --radix-popper-anchor-width: 41.9375px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="right"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k3"
    >
      <p>Add to library</p>
      <span
        style="
          position: absolute;
          left: 0px;
          transform-origin: 0px 0px;
          transform: translateY(50%) rotate(90deg) translateX(-50%);
          top: 16px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k3-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        ><p>Add to library</p></span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var content = shadless.h.mountFromTemplate(tpl, "[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      shadless.h.bindHandlers(trigger, wired.handlers, w.signal);
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()
```
:::

::::

## With Keyboard Shortcut

::::demo tooltip-keyboard
<iframe class="demo" src="/demos/tooltip-keyboard.html" title="tooltip-keyboard" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tooltip-keyboard.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tooltip-keyboard.html]
<button
  data-slot="tooltip-trigger"
  data-variant="outline"
  data-size="icon-sm"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg"
  data-state="closed"
  id="k0-trigger"
>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class="lucide lucide-save"
  >
    <path
      d="M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"
    ></path>
    <path d="M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"></path>
    <path d="M7 3v4a1 1 0 0 0 1 1h7"></path>
  </svg>
</button>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 46px);
      min-width: max-content;
      --radix-popper-transform-origin: 27.991999999999997px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 674px;
      --radix-popper-anchor-width: 40px;
      --radix-popper-anchor-height: 33px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      Save Changes
      <kbd
        data-slot="kbd"
        class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
        >S</kbd
      ><span
        style="
          position: absolute;
          transform: rotate(180deg);
          left: 22.992px;
          top: 0px;
          transform-origin: center 0px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k0-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >Save Changes
        <kbd
          data-slot="kbd"
          class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
          >S</kbd
        ></span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var content = shadless.h.mountFromTemplate(tpl, "[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      shadless.h.bindHandlers(trigger, wired.handlers, w.signal);
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()
```
:::

::::

## Disabled Button

Show a tooltip on a disabled button by wrapping it with a span.

::::demo tooltip-disabled
<iframe class="demo" src="/demos/tooltip-disabled.html" title="tooltip-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tooltip-disabled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tooltip-disabled.html]
<span class="inline-block w-fit" data-state="closed" data-slot="tooltip-trigger" id="k0-trigger"
  ><button
    data-slot="button"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    disabled=""
  >
    Disabled
  </button></span
>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 31px);
      min-width: max-content;
      --radix-popper-transform-origin: 41.929500000000004px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 689px;
      --radix-popper-anchor-width: 67.875px;
      --radix-popper-anchor-height: 17px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      <p>This feature is currently unavailable</p>
      <span
        style="
          position: absolute;
          transform: rotate(180deg);
          left: 36.9295px;
          top: 0px;
          transform-origin: center 0px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k0-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        ><p>This feature is currently unavailable</p></span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var content = shadless.h.mountFromTemplate(tpl, "[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      shadless.h.bindHandlers(trigger, wired.handlers, w.signal);
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()
```
:::

::::

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo tooltip-rtl
<iframe class="demo" src="/demos/tooltip-rtl.html" title="tooltip-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tooltip-rtl.html">Open the demo page</a> · <a href="/demos/tooltip-rtl-he.html">HE</a> · <a href="/demos/tooltip-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [tooltip-rtl.html]
<div class="flex flex-wrap gap-2">
  <button
    data-slot="tooltip-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-fit capitalize"
    data-state="closed"
    id="k0-trigger"
  >
    يسار</button
  ><button
    data-slot="tooltip-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-fit capitalize"
    data-state="closed"
    id="k1-trigger"
  >
    أعلى</button
  ><button
    data-slot="tooltip-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-fit capitalize"
    data-state="closed"
    id="k2-trigger"
  >
    أسفل</button
  ><button
    data-slot="tooltip-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-fit capitalize"
    data-state="closed"
    id="k3-trigger"
  >
    يمين
  </button>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(1109px, 9px);
      min-width: max-content;
      --radix-popper-transform-origin: 121px 9.5px;
      z-index: auto;
      --radix-popper-available-width: 1225px;
      --radix-popper-available-height: 720px;
      --radix-popper-anchor-width: 42px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="left"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pe-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      إضافة إلى المكتبة<span
        style="
          position: absolute;
          right: 0px;
          transform-origin: 100% 0px;
          transform: translateY(50%) rotate(-90deg) translateX(50%);
          top: 7px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k0-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >إضافة إلى المكتبة</span
      >
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
      transform: translate(1151px, 34px);
      min-width: max-content;
      --radix-popper-transform-origin: 58px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 686px;
      --radix-popper-anchor-width: 43px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pe-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k1"
    >
      إضافة إلى المكتبة<span
        style="
          position: absolute;
          transform: rotate(180deg);
          left: 53px;
          top: 0px;
          transform-origin: center 0px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k1-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >إضافة إلى المكتبة</span
      >
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
      transform: translate(1105px, 34px);
      min-width: max-content;
      --radix-popper-transform-origin: 58px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 686px;
      --radix-popper-anchor-width: 48px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pe-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k2"
    >
      إضافة إلى المكتبة<span
        style="
          position: absolute;
          top: 0px;
          transform-origin: center 0px;
          transform: rotate(180deg);
          left: 53px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k2-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >إضافة إلى المكتبة</span
      >
    </div>
  </div>
</template>
<template id="k3-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(1144px, 9px);
      min-width: max-content;
      --radix-popper-transform-origin: -5px 9.5px;
      z-index: auto;
      --radix-popper-available-width: 136px;
      --radix-popper-available-height: 720px;
      --radix-popper-anchor-width: 42px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="right"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pe-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k3"
    >
      إضافة إلى المكتبة<span
        style="
          position: absolute;
          left: 0px;
          transform-origin: 0px 0px;
          transform: translateY(50%) rotate(90deg) translateX(-50%);
          top: 7px;
        "
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k3-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >إضافة إلى المكتبة</span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var content = shadless.h.mountFromTemplate(tpl, "[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      shadless.h.bindHandlers(trigger, wired.handlers, w.signal);
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
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
| `data-slot="tooltip-trigger"` |
| `data-slot="tooltip-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix Tooltip](https://www.radix-ui.com/primitives/docs/components/tooltip#api-reference) documentation.
