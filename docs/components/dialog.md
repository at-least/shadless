---
title: "Dialog"
description: "A window overlaid on either the primary window or another dialog window, rendering the content underneath inert."
---

# Dialog

A window overlaid on either the primary window or another dialog window, rendering the content underneath inert.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/dialog" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/dialog#api-reference" rel="noopener">api</a></p>

::::demo dialog-demo
<iframe class="demo" src="/demos/dialog.html" title="dialog-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dialog.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dialog.html]
<div>
  <button
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="d1"
    data-state="closed"
    data-slot="dialog-trigger"
    id="d1-trigger"
  >
    Open dialog
  </button>
</div>
<template id="d1-portal">
  <div
    data-state="open"
    data-slot="dialog-overlay"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50"
    style="pointer-events: auto"
  ></div>
  <div
    role="dialog"
    id="d1"
    aria-describedby="d1-desc"
    aria-labelledby="d1-title"
    data-state="open"
    data-slot="dialog-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-sm ring-1 duration-100 sm:max-w-sm fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 outline-none"
    tabindex="-1"
    style="pointer-events: auto"
  >
    <div data-slot="dialog-header" class="gap-2 flex flex-col">
      <h2
        id="d1-title"
        data-slot="dialog-title"
        class="text-base leading-none font-medium cn-font-heading"
      >
        Are you absolutely sure?
      </h2>
      <p
        id="d1-desc"
        data-slot="dialog-description"
        class="text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3"
      >
        This action cannot be undone.
      </p>
    </div>
    <button
      data-slot="dialog-close"
      data-variant="ghost"
      data-size="icon-sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg absolute top-2 right-2"
      type="button"
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
        class="lucide lucide-x"
      >
        <path d="M18 6 6 18"></path>
        <path d="m6 6 12 12"></path></svg
      ><span class="sr-only">Close</span>
    </button>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dialog.js
// shadless dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=dialog-overlay]");
        var content = portal.querySelector("[data-slot=dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);

        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "dialog");
          },
        });
        portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
          b.addEventListener("click", function () { handles && handles.close(true); });
        });
        open = true;
        shadless.h.emit(trigger, "open", "dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "dialog",
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
@import "shadless/dialog.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/dialog.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/dialog.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/dialog.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/dialog.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="dialog-trigger" id="<k>-trigger">` | opens on clicking; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="dialog-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/dialog.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
dialog
├── dialog-trigger
└── dialog-content
    ├── dialog-header
    │   ├── dialog-title
    │   └── dialog-description
    └── dialog-footer
```

## Custom Close Button

Replace the default close control with your own button.

::::demo dialog-close-button
<iframe class="demo" src="/demos/dialog-close-button.html" title="dialog-close-button" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dialog-close-button.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dialog-close-button.html]
<button
  data-slot="dialog-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  aria-haspopup="dialog"
  aria-expanded="false"
  aria-controls="d1"
  data-state="closed"
  id="d1-trigger"
>
  Share
</button>
<template id="d1-portal">
  <div
    data-state="open"
    data-slot="dialog-overlay"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50"
    style="pointer-events: auto"
  ></div>
  <div
    role="dialog"
    id="d1"
    aria-describedby="d1-desc"
    aria-labelledby="d1-title"
    data-state="open"
    data-slot="dialog-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-sm ring-1 duration-100 fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 outline-none sm:max-w-md"
    tabindex="-1"
    style="pointer-events: auto"
  >
    <div data-slot="dialog-header" class="gap-2 flex flex-col">
      <h2
        id="d1-title"
        data-slot="dialog-title"
        class="text-base leading-none font-medium cn-font-heading"
      >
        Share link
      </h2>
      <p
        id="d1-desc"
        data-slot="dialog-description"
        class="text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3"
      >
        Anyone who has this link will be able to view this.
      </p>
    </div>
    <div class="flex items-center gap-2">
      <div class="grid flex-1 gap-2">
        <label
          data-slot="label"
          class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed sr-only"
          for="link"
          >Link</label
        ><input
          data-slot="input"
          class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
          id="link"
          readonly=""
          value="https://ui.shadcn.com/docs/installation"
        />
      </div>
    </div>
    <div
      data-slot="dialog-footer"
      class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 sm:flex-row sm:justify-start"
    >
      <button
        data-slot="dialog-close"
        data-variant="default"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
        type="button"
      >
        Close
      </button>
    </div>
    <button
      data-slot="dialog-close"
      data-variant="ghost"
      data-size="icon-sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg absolute top-2 right-2"
      type="button"
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
        class="lucide lucide-x"
      >
        <path d="M18 6 6 18"></path>
        <path d="m6 6 12 12"></path></svg
      ><span class="sr-only">Close</span>
    </button>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dialog.js
// shadless dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=dialog-overlay]");
        var content = portal.querySelector("[data-slot=dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);

        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "dialog");
          },
        });
        portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
          b.addEventListener("click", function () { handles && handles.close(true); });
        });
        open = true;
        shadless.h.emit(trigger, "open", "dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "dialog",
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


## No Close Button

Use `showCloseButton={false}` to hide the close button.

::::demo dialog-no-close-button
<iframe class="demo" src="/demos/dialog-no-close-button.html" title="dialog-no-close-button" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dialog-no-close-button.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dialog-no-close-button.html]
<button
  data-slot="dialog-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  aria-haspopup="dialog"
  aria-expanded="false"
  aria-controls="d1"
  data-state="closed"
  id="d1-trigger"
>
  No Close Button
</button>
<template id="d1-portal">
  <div
    data-state="open"
    data-slot="dialog-overlay"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50"
    style="pointer-events: auto"
  ></div>
  <div
    role="dialog"
    id="d1"
    aria-describedby="d1-desc"
    aria-labelledby="d1-title"
    data-state="open"
    data-slot="dialog-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-sm ring-1 duration-100 sm:max-w-sm fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 outline-none"
    tabindex="-1"
    style="pointer-events: auto"
  >
    <div data-slot="dialog-header" class="gap-2 flex flex-col">
      <h2
        id="d1-title"
        data-slot="dialog-title"
        class="text-base leading-none font-medium cn-font-heading"
      >
        No Close Button
      </h2>
      <p
        id="d1-desc"
        data-slot="dialog-description"
        class="text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3"
      >
        This dialog doesn't have a close button in the top-right corner.
      </p>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dialog.js
// shadless dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=dialog-overlay]");
        var content = portal.querySelector("[data-slot=dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);

        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "dialog");
          },
        });
        portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
          b.addEventListener("click", function () { handles && handles.close(true); });
        });
        open = true;
        shadless.h.emit(trigger, "open", "dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "dialog",
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


## Sticky Footer

Keep actions visible while the content scrolls.

::::demo dialog-sticky-footer
<iframe class="demo" src="/demos/dialog-sticky-footer.html" title="dialog-sticky-footer" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dialog-sticky-footer.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dialog-sticky-footer.html]
<button
  data-slot="dialog-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  aria-haspopup="dialog"
  aria-expanded="false"
  aria-controls="d1"
  data-state="closed"
  id="d1-trigger"
>
  Sticky Footer
</button>
<template id="d1-portal">
  <div
    data-state="open"
    data-slot="dialog-overlay"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50"
    style="pointer-events: auto"
  ></div>
  <div
    role="dialog"
    id="d1"
    aria-describedby="d1-desc"
    aria-labelledby="d1-title"
    data-state="open"
    data-slot="dialog-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-sm ring-1 duration-100 sm:max-w-sm fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 outline-none"
    tabindex="-1"
    style="pointer-events: auto"
  >
    <div data-slot="dialog-header" class="gap-2 flex flex-col">
      <h2
        id="d1-title"
        data-slot="dialog-title"
        class="text-base leading-none font-medium cn-font-heading"
      >
        Sticky Footer
      </h2>
      <p
        id="d1-desc"
        data-slot="dialog-description"
        class="text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3"
      >
        This dialog has a sticky footer that stays visible while the content scrolls.
      </p>
    </div>
    <div class="-mx-4 no-scrollbar max-h-[50vh] overflow-y-auto px-4">
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
    </div>
    <div
      data-slot="dialog-footer"
      class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"
    >
      <button
        data-slot="dialog-close"
        data-variant="outline"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
        type="button"
      >
        Close
      </button>
    </div>
    <button
      data-slot="dialog-close"
      data-variant="ghost"
      data-size="icon-sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg absolute top-2 right-2"
      type="button"
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
        class="lucide lucide-x"
      >
        <path d="M18 6 6 18"></path>
        <path d="m6 6 12 12"></path></svg
      ><span class="sr-only">Close</span>
    </button>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dialog.js
// shadless dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=dialog-overlay]");
        var content = portal.querySelector("[data-slot=dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);

        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "dialog");
          },
        });
        portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
          b.addEventListener("click", function () { handles && handles.close(true); });
        });
        open = true;
        shadless.h.emit(trigger, "open", "dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "dialog",
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


## Scrollable Content

Long content can scroll while the header stays in view.

::::demo dialog-scrollable-content
<iframe class="demo" src="/demos/dialog-scrollable-content.html" title="dialog-scrollable-content" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dialog-scrollable-content.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dialog-scrollable-content.html]
<button
  data-slot="dialog-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  aria-haspopup="dialog"
  aria-expanded="false"
  aria-controls="d1"
  data-state="closed"
  id="d1-trigger"
>
  Scrollable Content
</button>
<template id="d1-portal">
  <div
    data-state="open"
    data-slot="dialog-overlay"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50"
    style="pointer-events: auto"
  ></div>
  <div
    role="dialog"
    id="d1"
    aria-describedby="d1-desc"
    aria-labelledby="d1-title"
    data-state="open"
    data-slot="dialog-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-sm ring-1 duration-100 sm:max-w-sm fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 outline-none"
    tabindex="-1"
    style="pointer-events: auto"
  >
    <div data-slot="dialog-header" class="gap-2 flex flex-col">
      <h2
        id="d1-title"
        data-slot="dialog-title"
        class="text-base leading-none font-medium cn-font-heading"
      >
        Scrollable Content
      </h2>
      <p
        id="d1-desc"
        data-slot="dialog-description"
        class="text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3"
      >
        This is a dialog with scrollable content.
      </p>
    </div>
    <div class="-mx-4 no-scrollbar max-h-[50vh] overflow-y-auto px-4">
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
      <p class="mb-4 leading-normal">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
      </p>
    </div>
    <button
      data-slot="dialog-close"
      data-variant="ghost"
      data-size="icon-sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg absolute top-2 right-2"
      type="button"
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
        class="lucide lucide-x"
      >
        <path d="M18 6 6 18"></path>
        <path d="m6 6 12 12"></path></svg
      ><span class="sr-only">Close</span>
    </button>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dialog.js
// shadless dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=dialog-overlay]");
        var content = portal.querySelector("[data-slot=dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);

        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "dialog");
          },
        });
        portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
          b.addEventListener("click", function () { handles && handles.close(true); });
        });
        open = true;
        shadless.h.emit(trigger, "open", "dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "dialog",
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

::::demo dialog-rtl
<iframe class="demo" src="/demos/dialog-rtl.html" title="dialog-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dialog-rtl.html">Open the demo page</a> · <a href="/demos/dialog-rtl-he.html">HE</a> · <a href="/demos/dialog-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [dialog-rtl.html]
<form>
  <button
    data-slot="dialog-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="d1"
    data-state="closed"
    id="d1-trigger"
  >
    فتح الحوار
  </button>
</form>
<template id="d1-portal">
  <div
    data-state="open"
    data-slot="dialog-overlay"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50"
    style="pointer-events: auto"
  ></div>
  <div
    role="dialog"
    id="d1"
    aria-describedby="d1-desc"
    aria-labelledby="d1-title"
    data-state="open"
    data-slot="dialog-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-sm ring-1 duration-100 fixed top-1/2 start-1/2 z-50 w-full -translate-x-1/2 rtl:translate-x-1/2 -translate-y-1/2 outline-none sm:max-w-sm"
    dir="rtl"
    data-lang="ar"
    tabindex="-1"
    style="pointer-events: auto"
  >
    <div data-slot="dialog-header" class="gap-2 flex flex-col">
      <h2
        id="d1-title"
        data-slot="dialog-title"
        class="text-base leading-none font-medium cn-font-heading"
      >
        تعديل الملف الشخصي
      </h2>
      <p
        id="d1-desc"
        data-slot="dialog-description"
        class="text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3"
      >
        قم بإجراء تغييرات على ملفك الشخصي هنا. انقر فوق حفظ عند الانتهاء.
      </p>
    </div>
    <div
      data-slot="field-group"
      class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
    >
      <div
        role="group"
        data-slot="field"
        data-orientation="vertical"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
      >
        <label
          data-slot="label"
          class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
          for="name-1"
          >الاسم</label
        ><input
          data-slot="input"
          class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
          id="name-1"
          value="Pedro Duarte"
          name="name"
        />
      </div>
      <div
        role="group"
        data-slot="field"
        data-orientation="vertical"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
      >
        <label
          data-slot="label"
          class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
          for="username-1"
          >اسم المستخدم</label
        ><input
          data-slot="input"
          class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
          id="username-1"
          value="@peduarte"
          name="username"
        />
      </div>
    </div>
    <div
      data-slot="dialog-footer"
      class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"
    >
      <button
        data-slot="dialog-close"
        data-variant="outline"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
        type="button"
      >
        إلغاء</button
      ><button
        data-slot="button"
        data-variant="default"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
        type="submit"
      >
        حفظ التغييرات
      </button>
    </div>
    <button
      data-slot="dialog-close"
      data-variant="ghost"
      data-size="icon-sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg absolute top-2 end-2"
      type="button"
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
        class="lucide lucide-x"
      >
        <path d="M18 6 6 18"></path>
        <path d="m6 6 12 12"></path></svg
      ><span class="sr-only">Close</span>
    </button>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dialog.js
// shadless dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=dialog-overlay]");
        var content = portal.querySelector("[data-slot=dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);

        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "dialog");
          },
        });
        portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
          b.addEventListener("click", function () { handles && handles.close(true); });
        });
        open = true;
        shadless.h.emit(trigger, "open", "dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "dialog",
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
| `data-slot="dialog"` |
| `data-slot="dialog-trigger"` |
| `data-slot="dialog-portal"` |
| `data-slot="dialog-close"` |
| `data-slot="dialog-overlay"` |
| `data-slot="dialog-content"` |
| `data-slot="dialog-header"` |
| `data-slot="dialog-footer"` |
| `data-slot="dialog-title"` |
| `data-slot="dialog-description"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI](https://www.radix-ui.com/docs/primitives/components/dialog#api-reference) documentation for more information.
