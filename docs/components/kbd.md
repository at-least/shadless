---
title: "Kbd"
description: "Used to display textual user input from keyboard."
---

# Kbd

Used to display textual user input from keyboard.

::::demo kbd-demo
<iframe class="demo" src="/demos/kbd-demo.html" title="kbd-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/kbd-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [kbd-demo.html]
<div class="flex flex-col items-center gap-4">
  <kbd data-slot="kbd-group" class="gap-1 inline-flex items-center"
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >⌘</kbd
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >⇧</kbd
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >⌥</kbd
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >⌃</kbd
    ></kbd
  ><kbd data-slot="kbd-group" class="gap-1 inline-flex items-center"
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >Ctrl</kbd
    ><span>+</span
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >B</kbd
    ></kbd
  >
</div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/kbd.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/kbd.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/kbd.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                            into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/kbd.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
kbd
kbd-group
├── kbd
└── kbd
```

## Group

Use the `KbdGroup` component to group keyboard keys together.

::::demo kbd-group
<iframe class="demo" src="/demos/kbd-group.html" title="kbd-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/kbd-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [kbd-group.html]
<div class="flex flex-col items-center gap-4">
  <p class="text-sm text-muted-foreground">
    Use
    <kbd data-slot="kbd-group" class="gap-1 inline-flex items-center"
      ><kbd
        data-slot="kbd"
        class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
        >Ctrl + B</kbd
      ><kbd
        data-slot="kbd"
        class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
        >Ctrl + K</kbd
      ></kbd
    >
    to open the command palette
  </p>
</div>
```
:::

::::


## Button

Use the `Kbd` component inside a `Button` component to display a keyboard key inside a button.

::::demo kbd-button
<iframe class="demo" src="/demos/kbd-button.html" title="kbd-button" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/kbd-button.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [kbd-button.html]
<button
  data-slot="button"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
>
  Accept
  <kbd
    data-slot="kbd"
    class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none translate-x-0.5"
    data-icon="inline-end"
    >⏎</kbd
  >
</button>
```
:::

::::


## Tooltip

You can use the `Kbd` component inside a `Tooltip` component to display a tooltip with a keyboard key.

::::demo kbd-tooltip
<iframe class="demo" src="/demos/kbd-tooltip.html" title="kbd-tooltip" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/kbd-tooltip.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [kbd-tooltip.html]
<div class="flex flex-wrap gap-4">
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="tooltip-trigger"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
      data-state="closed"
      id="k0-trigger"
    >
      Save</button
    ><button
      data-slot="tooltip-trigger"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
      data-state="closed"
      id="k1-trigger"
    >
      Print
    </button>
  </div>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 34px);
      min-width: max-content;
      --radix-popper-transform-origin: 31.187312499999997px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 686px;
      --radix-popper-anchor-width: 46.390625px;
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
          left: 26.1873px;
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
<template id="k1-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(4px, 34px);
      min-width: max-content;
      --radix-popper-transform-origin: 72.5px -5px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 686px;
      --radix-popper-anchor-width: 43.40625px;
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
      Print Document
      <kbd data-slot="kbd-group" class="gap-1 inline-flex items-center"
        ><kbd
          data-slot="kbd"
          class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
          >Ctrl</kbd
        ><kbd
          data-slot="kbd"
          class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
          >P</kbd
        ></kbd
      ><span
        style="
          position: absolute;
          transform: rotate(180deg);
          left: 67.5px;
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
        >Print Document
        <kbd data-slot="kbd-group" class="gap-1 inline-flex items-center"
          ><kbd
            data-slot="kbd"
            class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
            >Ctrl</kbd
          ><kbd
            data-slot="kbd"
            class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
            >P</kbd
          ></kbd
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
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
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


## Input Group

You can use the `Kbd` component inside a `InputGroupAddon` component to display a keyboard key inside an input group.

::::demo kbd-input-group
<iframe class="demo" src="/demos/kbd-input-group.html" title="kbd-input-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/kbd-input-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [kbd-input-group.html]
<div class="flex w-full max-w-xs flex-col gap-6">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Search..."
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-start"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pl-2 has-[&gt;button]:ml-[-0.3rem] has-[&gt;kbd]:ml-[-0.15rem] order-first"
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
        class="lucide lucide-search"
      >
        <circle cx="11" cy="11" r="8"></circle>
        <path d="m21 21-4.3-4.3"></path>
      </svg>
    </div>
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <kbd
        data-slot="kbd"
        class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
        >⌘</kbd
      ><kbd
        data-slot="kbd"
        class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
        >K</kbd
      >
    </div>
  </div>
</div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo kbd-rtl
<iframe class="demo" src="/demos/kbd-rtl.html" title="kbd-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/kbd-rtl.html">Open the demo page</a> · <a href="/demos/kbd-rtl-he.html">HE</a> · <a href="/demos/kbd-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [kbd-rtl.html]
<div class="flex flex-col items-center gap-4" dir="rtl">
  <kbd data-slot="kbd-group" class="gap-1 inline-flex items-center"
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >⌘</kbd
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >⇧</kbd
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >⌥</kbd
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >⌃</kbd
    ></kbd
  ><kbd data-slot="kbd-group" class="gap-1 inline-flex items-center"
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >Ctrl</kbd
    ><span>+</span
    ><kbd
      data-slot="kbd"
      class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
      >B</kbd
    ></kbd
  >
</div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="kbd"` |
| `data-slot="kbd-group"` |

**Runtime:** no JavaScript — this is markup + CSS. No `cva`-declared variants. Check `dist/css/kbd.css` for any `data-*` attribute this slot's styling depends on.
