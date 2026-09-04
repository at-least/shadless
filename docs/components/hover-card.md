---
title: "Hover Card"
description: "For sighted users to preview content available behind a link."
---

# Hover Card

For sighted users to preview content available behind a link.

::::demo hover-card-demo
<iframe class="demo" src="/demos/hover-card.html" title="hover-card-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/hover-card.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [hover-card.html]
<div>
  <a data-state="closed" data-slot="hover-card-trigger" id="d1-trigger">Hover me</a>
</div>
<template id="d1-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 29px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 691px;
      --radix-popper-anchor-width: 63.53125px;
      --radix-popper-anchor-height: 17px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground w-64 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      The React Framework for web and native user interfaces.
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/hover-card.js
// shadless hover-card behavior (wireHoverCard) — registers with the base; multi-instance: every
// [data-slot=hover-card-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("hover-card", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=hover-card-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, current = null;
      var wired = RadixKernel.wireHoverCard({
        trigger: trigger,
        openDelay: 700, closeDelay: 300,  // radix HoverCard.Root defaults
        popperOptions: { sideOffset: 4 }, // shadcn HoverCardContent default
        buildContent: function () {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=hover-card-content]");
          content.setAttribute("data-state", "open");
          current = content;
          return content;
        },
        onOpen: function () { open = true; shadless.h.emit(trigger, "open", "hover-card"); },
        onClosed: function () { open = false; current = null; shadless.h.emit(trigger, "close", "hover-card"); },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "hover-card",
        open: function () { if (!open) trigger.dispatchEvent(new PointerEvent("pointerenter", { pointerType: "mouse" })) },
        close: function () { if (open) wired.dismiss() },
        toggle: function () { open ? wired.dismiss() : this.open() },
        isOpen: function () { return open },
      })
      // radix HoverCardContent is a DismissableLayer (Escape + outside
      // pointerdown); kernel wireHoverCard has neither — absorb in glue.
      document.addEventListener("keydown", function (e) {
        if (open && e.key === "Escape") wired.dismiss();
      }, { signal: w.signal });
      document.addEventListener("pointerdown", function (e) {
        if (!open || !current) return;
        var t = e.target;
        if (!current.contains(t) && !trigger.contains(t)) wired.dismiss();
      }, { signal: w.signal });
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
@import "shadless/hover-card.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/hover-card.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/hover-card.js` | this component's behavior — registers with the base |

**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/hover-card.js"></script>
```

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="hover-card-trigger" id="<k>-trigger">` | opens on hovering; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="hover-card-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
hover-card
├── hover-card-trigger
└── hover-card-portal
```

## Trigger Delays

shadless's hover-card glue hardcodes `openDelay: 700, closeDelay: 300` (Radix's own defaults) in `dist/js/hover-card.js` — there is no markup prop to change them; edit that file directly if you need different timing.

## Positioning

shadless's hover-card glue doesn't expose `side`/`align` through markup either — `popperOptions` in `dist/js/hover-card.js` only sets `sideOffset: 4`, so placement is whatever the kernel's popper default is (auto-flipping on collision); edit that file to pass a preferred placement.

## Basic

::::demo hover-card-demo
<iframe class="demo" src="/demos/hover-card.html" title="hover-card-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/hover-card.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [hover-card.html]
<div>
  <a data-state="closed" data-slot="hover-card-trigger" id="d1-trigger">Hover me</a>
</div>
<template id="d1-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 29px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 691px;
      --radix-popper-anchor-width: 63.53125px;
      --radix-popper-anchor-height: 17px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground w-64 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      The React Framework for web and native user interfaces.
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/hover-card.js
// shadless hover-card behavior (wireHoverCard) — registers with the base; multi-instance: every
// [data-slot=hover-card-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("hover-card", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=hover-card-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, current = null;
      var wired = RadixKernel.wireHoverCard({
        trigger: trigger,
        openDelay: 700, closeDelay: 300,  // radix HoverCard.Root defaults
        popperOptions: { sideOffset: 4 }, // shadcn HoverCardContent default
        buildContent: function () {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=hover-card-content]");
          content.setAttribute("data-state", "open");
          current = content;
          return content;
        },
        onOpen: function () { open = true; shadless.h.emit(trigger, "open", "hover-card"); },
        onClosed: function () { open = false; current = null; shadless.h.emit(trigger, "close", "hover-card"); },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "hover-card",
        open: function () { if (!open) trigger.dispatchEvent(new PointerEvent("pointerenter", { pointerType: "mouse" })) },
        close: function () { if (open) wired.dismiss() },
        toggle: function () { open ? wired.dismiss() : this.open() },
        isOpen: function () { return open },
      })
      // radix HoverCardContent is a DismissableLayer (Escape + outside
      // pointerdown); kernel wireHoverCard has neither — absorb in glue.
      document.addEventListener("keydown", function (e) {
        if (open && e.key === "Escape") wired.dismiss();
      }, { signal: w.signal });
      document.addEventListener("pointerdown", function (e) {
        if (!open || !current) return;
        var t = e.target;
        if (!current.contains(t) && !trigger.contains(t)) wired.dismiss();
      }, { signal: w.signal });
    });
  } })
})()
```
:::

::::

## Sides

::::demo hover-card-sides
<iframe class="demo" src="/demos/hover-card-sides.html" title="hover-card-sides" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/hover-card-sides.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [hover-card-sides.html]
<div class="flex flex-wrap justify-center gap-2">
  <button
    data-slot="hover-card-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 capitalize"
    data-state="closed"
    id="k0-trigger"
  >
    left</button
  ><button
    data-slot="hover-card-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 capitalize"
    data-state="closed"
    id="k1-trigger"
  >
    top</button
  ><button
    data-slot="hover-card-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 capitalize"
    data-state="closed"
    id="k2-trigger"
  >
    bottom</button
  ><button
    data-slot="hover-card-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 capitalize"
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
      transform: translate(46px, 0px);
      min-width: max-content;
      --radix-popper-transform-origin: 0px 50%;
      z-index: auto;
      --radix-popper-available-width: 1234.21875px;
      --radix-popper-available-height: 720px;
      --radix-popper-anchor-width: 33.78125px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="right"
      data-align="center"
      data-state="open"
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground w-64 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      <div class="flex flex-col gap-1">
        <h4 class="font-medium">Hover Card</h4>
        <p>This hover card appears on the left side of the trigger.</p>
      </div>
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
      transform: translate(0px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 34.53125px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground w-64 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      <div class="flex flex-col gap-1">
        <h4 class="font-medium">Hover Card</h4>
        <p>This hover card appears on the top side of the trigger.</p>
      </div>
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
      transform: translate(0px, 33px);
      min-width: max-content;
      --radix-popper-transform-origin: 50% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 687px;
      --radix-popper-anchor-width: 56.75px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="bottom"
      data-align="center"
      data-state="open"
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground w-64 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      <div class="flex flex-col gap-1">
        <h4 class="font-medium">Hover Card</h4>
        <p>This hover card appears on the bottom side of the trigger.</p>
      </div>
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
      transform: translate(179px, 0px);
      min-width: max-content;
      --radix-popper-transform-origin: 0px 50%;
      z-index: auto;
      --radix-popper-available-width: 1101px;
      --radix-popper-available-height: 720px;
      --radix-popper-anchor-width: 41.9375px;
      --radix-popper-anchor-height: 21px;
    "
  >
    <div
      data-side="right"
      data-align="center"
      data-state="open"
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground w-64 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      <div class="flex flex-col gap-1">
        <h4 class="font-medium">Hover Card</h4>
        <p>This hover card appears on the right side of the trigger.</p>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/hover-card.js
// shadless hover-card behavior (wireHoverCard) — registers with the base; multi-instance: every
// [data-slot=hover-card-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("hover-card", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=hover-card-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, current = null;
      var wired = RadixKernel.wireHoverCard({
        trigger: trigger,
        openDelay: 700, closeDelay: 300,  // radix HoverCard.Root defaults
        popperOptions: { sideOffset: 4 }, // shadcn HoverCardContent default
        buildContent: function () {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=hover-card-content]");
          content.setAttribute("data-state", "open");
          current = content;
          return content;
        },
        onOpen: function () { open = true; shadless.h.emit(trigger, "open", "hover-card"); },
        onClosed: function () { open = false; current = null; shadless.h.emit(trigger, "close", "hover-card"); },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "hover-card",
        open: function () { if (!open) trigger.dispatchEvent(new PointerEvent("pointerenter", { pointerType: "mouse" })) },
        close: function () { if (open) wired.dismiss() },
        toggle: function () { open ? wired.dismiss() : this.open() },
        isOpen: function () { return open },
      })
      // radix HoverCardContent is a DismissableLayer (Escape + outside
      // pointerdown); kernel wireHoverCard has neither — absorb in glue.
      document.addEventListener("keydown", function (e) {
        if (open && e.key === "Escape") wired.dismiss();
      }, { signal: w.signal });
      document.addEventListener("pointerdown", function (e) {
        if (!open || !current) return;
        var t = e.target;
        if (!current.contains(t) && !trigger.contains(t)) wired.dismiss();
      }, { signal: w.signal });
    });
  } })
})()
```
:::

::::

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo hover-card-rtl
<iframe class="demo" src="/demos/hover-card-rtl.html" title="hover-card-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/hover-card-rtl.html">Open the demo page</a> · <a href="/demos/hover-card-rtl-he.html">HE</a> · <a href="/demos/hover-card-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [hover-card-rtl.html]
<div class="flex flex-wrap justify-center gap-2">
  <button
    data-slot="hover-card-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    data-state="closed"
    id="k0-trigger"
  >
    يسار</button
  ><button
    data-slot="hover-card-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    data-state="closed"
    id="k1-trigger"
  >
    أعلى</button
  ><button
    data-slot="hover-card-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    data-state="closed"
    id="k2-trigger"
  >
    أسفل</button
  ><button
    data-slot="hover-card-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
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
      transform: translate(1115px, 0px);
      min-width: max-content;
      --radix-popper-transform-origin: 111px 50%;
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
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden flex w-64 flex-col gap-1"
      dir="rtl"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      <div class="font-semibold">سماعات لاسلكية</div>
      <div class="text-sm text-muted-foreground">٩٩.٩٩ $</div>
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
      transform: translate(1153px, 33px);
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
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden flex w-64 flex-col gap-1"
      dir="rtl"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      <div class="font-semibold">سماعات لاسلكية</div>
      <div class="text-sm text-muted-foreground">٩٩.٩٩ $</div>
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
      transform: translate(1108px, 33px);
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
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden flex w-64 flex-col gap-1"
      dir="rtl"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      <div class="font-semibold">سماعات لاسلكية</div>
      <div class="text-sm text-muted-foreground">٩٩.٩٩ $</div>
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
      data-slot="hover-card-content"
      class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden flex w-64 flex-col gap-1"
      dir="rtl"
      style="
        --radix-hover-card-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-hover-card-content-available-width: var(--radix-popper-available-width);
        --radix-hover-card-content-available-height: var(--radix-popper-available-height);
        --radix-hover-card-trigger-width: var(--radix-popper-anchor-width);
        --radix-hover-card-trigger-height: var(--radix-popper-anchor-height);
      "
    >
      <div class="font-semibold">سماعات لاسلكية</div>
      <div class="text-sm text-muted-foreground">٩٩.٩٩ $</div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/hover-card.js
// shadless hover-card behavior (wireHoverCard) — registers with the base; multi-instance: every
// [data-slot=hover-card-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("hover-card", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=hover-card-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, current = null;
      var wired = RadixKernel.wireHoverCard({
        trigger: trigger,
        openDelay: 700, closeDelay: 300,  // radix HoverCard.Root defaults
        popperOptions: { sideOffset: 4 }, // shadcn HoverCardContent default
        buildContent: function () {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=hover-card-content]");
          content.setAttribute("data-state", "open");
          current = content;
          return content;
        },
        onOpen: function () { open = true; shadless.h.emit(trigger, "open", "hover-card"); },
        onClosed: function () { open = false; current = null; shadless.h.emit(trigger, "close", "hover-card"); },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "hover-card",
        open: function () { if (!open) trigger.dispatchEvent(new PointerEvent("pointerenter", { pointerType: "mouse" })) },
        close: function () { if (open) wired.dismiss() },
        toggle: function () { open ? wired.dismiss() : this.open() },
        isOpen: function () { return open },
      })
      // radix HoverCardContent is a DismissableLayer (Escape + outside
      // pointerdown); kernel wireHoverCard has neither — absorb in glue.
      document.addEventListener("keydown", function (e) {
        if (open && e.key === "Escape") wired.dismiss();
      }, { signal: w.signal });
      document.addEventListener("pointerdown", function (e) {
        if (!open || !current) return;
        var t = e.target;
        if (!current.contains(t) && !trigger.contains(t)) wired.dismiss();
      }, { signal: w.signal });
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
| `data-slot="hover-card"` |
| `data-slot="hover-card-trigger"` |
| `data-slot="hover-card-portal"` |
| `data-slot="hover-card-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI](https://www.radix-ui.com/primitives/docs/components/hover-card#api-reference) documentation for more information.
