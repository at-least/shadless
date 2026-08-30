---
title: "Alert Dialog"
description: "A modal dialog that interrupts the user with important content and expects a response."
---

# Alert Dialog

A modal dialog that interrupts the user with important content and expects a response.

<p class="page-links"><a href="https://www.radix-ui.com/primitives/docs/components/alert-dialog" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/primitives/docs/components/alert-dialog#api-reference" rel="noopener">api</a></p>

<iframe class="demo" src="/demos/alert-dialog.html" title="alert-dialog-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [alert-dialog.html]
<div>
<button type="button" aria-haspopup="dialog" aria-expanded="false" aria-controls="d1" data-state="closed" data-slot="alert-dialog-trigger" id="d1-trigger">Show Dialog</button>
</div>
<template id="d1-portal">
<div data-state="open" data-slot="alert-dialog-overlay" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50" style="pointer-events: auto;"></div>
<div role="alertdialog" id="d1" aria-describedby="d1-desc" aria-labelledby="d1-title" data-state="open" data-slot="alert-dialog-content" data-size="default" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-4 rounded-xl p-4 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-sm group/alert-dialog-content fixed top-1/2 left-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 outline-none" tabindex="-1" style="pointer-events: auto;"><div data-slot="alert-dialog-header" class="grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-4 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-left sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]"><h2 id="d1-title" data-slot="alert-dialog-title" class="text-base font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 cn-font-heading">Are you absolutely sure?</h2><p id="d1-desc" data-slot="alert-dialog-description" class="text-muted-foreground *:[a]:hover:text-foreground text-sm text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3">This action cannot be undone. This will permanently delete your account and remove your data from our servers.</p></div><div data-slot="alert-dialog-footer" class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 sm:flex-row sm:justify-end"><button type="button" data-slot="alert-dialog-cancel" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="outline" data-size="default">Cancel</button><button type="button" data-slot="alert-dialog-action" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="default" data-size="default">Continue</button></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/alert-dialog.js
// shadless alert-dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=alert-dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("alert-dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=alert-dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=alert-dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=alert-dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "alert-dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=alert-dialog-overlay]");
        var content = portal.querySelector("[data-slot=alert-dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);
        // alert-dialog semantics: clicking the overlay must NOT dismiss —
        // swallow overlay clicks so wireDialog's portal click-close never fires
        if (overlay) overlay.addEventListener("click", function (e) { e.stopPropagation(); });
        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "alert-dialog");
          },
        });
        // Action / Cancel both dismiss (radix Action/Cancel close the dialog)
        portal.querySelectorAll("[data-slot=alert-dialog-action], [data-slot=alert-dialog-cancel]")
          .forEach(function (b) {
            b.addEventListener("click", function () { handles && handles.close(true); });
          });
        open = true;
        shadless.h.emit(trigger, "open", "alert-dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "alert-dialog",
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
@import "shadless/alert-dialog.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/alert-dialog.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/alert-dialog.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/alert-dialog.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/alert-dialog.js"></script>
```

**Copy the markup from                                     into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="alert-dialog-trigger" id="<k>-trigger">` | opens on clicking; `<k>` names the instance |
| `<template id="<k>-portal">` | holds the overlay/content subtree (`data-slot="alert-dialog-content"` …) that the glue mounts into `<body>` while open |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/alert-dialog.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
alert-dialog
├── alert-dialog-trigger
└── alert-dialog-content
    ├── alert-dialog-header
    │   ├── alert-dialog-media
    │   ├── alert-dialog-title
    │   └── alert-dialog-description
    └── alert-dialog-footer
        ├── alert-dialog-cancel
        └── alert-dialog-action
```

## Basic

A basic alert dialog with a title, description, and cancel and continue buttons.

<iframe class="demo" src="/demos/alert-dialog-basic.html" title="alert-dialog-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [alert-dialog-basic.html]
<button data-slot="alert-dialog-trigger" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" type="button" aria-haspopup="dialog" aria-expanded="false" aria-controls="d1" data-state="closed" id="d1-trigger">Show Dialog</button>
<template id="d1-portal">
<div data-state="open" data-slot="alert-dialog-overlay" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50" style="pointer-events: auto;"></div>
<div role="alertdialog" id="d1" aria-describedby="d1-desc" aria-labelledby="d1-title" data-state="open" data-slot="alert-dialog-content" data-size="default" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-4 rounded-xl p-4 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-sm group/alert-dialog-content fixed top-1/2 left-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 outline-none" tabindex="-1" style="pointer-events: auto;"><div data-slot="alert-dialog-header" class="grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-4 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-left sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]"><h2 id="d1-title" data-slot="alert-dialog-title" class="text-base font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 cn-font-heading">Are you absolutely sure?</h2><p id="d1-desc" data-slot="alert-dialog-description" class="text-muted-foreground *:[a]:hover:text-foreground text-sm text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3">This action cannot be undone. This will permanently delete your account and remove your data from our servers.</p></div><div data-slot="alert-dialog-footer" class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 sm:flex-row sm:justify-end"><button type="button" data-slot="alert-dialog-cancel" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="outline" data-size="default">Cancel</button><button type="button" data-slot="alert-dialog-action" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="default" data-size="default">Continue</button></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/alert-dialog.js
// shadless alert-dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=alert-dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("alert-dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=alert-dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=alert-dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=alert-dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "alert-dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=alert-dialog-overlay]");
        var content = portal.querySelector("[data-slot=alert-dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);
        // alert-dialog semantics: clicking the overlay must NOT dismiss —
        // swallow overlay clicks so wireDialog's portal click-close never fires
        if (overlay) overlay.addEventListener("click", function (e) { e.stopPropagation(); });
        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "alert-dialog");
          },
        });
        // Action / Cancel both dismiss (radix Action/Cancel close the dialog)
        portal.querySelectorAll("[data-slot=alert-dialog-action], [data-slot=alert-dialog-cancel]")
          .forEach(function (b) {
            b.addEventListener("click", function () { handles && handles.close(true); });
          });
        open = true;
        shadless.h.emit(trigger, "open", "alert-dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "alert-dialog",
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


## Small

Use the `size="sm"` prop to make the alert dialog smaller.

<iframe class="demo" src="/demos/alert-dialog-small.html" title="alert-dialog-small" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [alert-dialog-small.html]
<button data-slot="alert-dialog-trigger" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" type="button" aria-haspopup="dialog" aria-expanded="false" aria-controls="d1" data-state="closed" id="d1-trigger">Show Dialog</button>
<template id="d1-portal">
<div data-state="open" data-slot="alert-dialog-overlay" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50" style="pointer-events: auto;"></div>
<div role="alertdialog" id="d1" aria-describedby="d1-desc" aria-labelledby="d1-title" data-state="open" data-slot="alert-dialog-content" data-size="sm" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-4 rounded-xl p-4 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-sm group/alert-dialog-content fixed top-1/2 left-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 outline-none" tabindex="-1" style="pointer-events: auto;"><div data-slot="alert-dialog-header" class="grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-4 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-left sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]"><h2 id="d1-title" data-slot="alert-dialog-title" class="text-base font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 cn-font-heading">Allow accessory to connect?</h2><p id="d1-desc" data-slot="alert-dialog-description" class="text-muted-foreground *:[a]:hover:text-foreground text-sm text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3">Do you want to allow the USB accessory to connect to this device?</p></div><div data-slot="alert-dialog-footer" class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 sm:flex-row sm:justify-end"><button type="button" data-slot="alert-dialog-cancel" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="outline" data-size="default">Don't allow</button><button type="button" data-slot="alert-dialog-action" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="default" data-size="default">Allow</button></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/alert-dialog.js
// shadless alert-dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=alert-dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("alert-dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=alert-dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=alert-dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=alert-dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "alert-dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=alert-dialog-overlay]");
        var content = portal.querySelector("[data-slot=alert-dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);
        // alert-dialog semantics: clicking the overlay must NOT dismiss —
        // swallow overlay clicks so wireDialog's portal click-close never fires
        if (overlay) overlay.addEventListener("click", function (e) { e.stopPropagation(); });
        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "alert-dialog");
          },
        });
        // Action / Cancel both dismiss (radix Action/Cancel close the dialog)
        portal.querySelectorAll("[data-slot=alert-dialog-action], [data-slot=alert-dialog-cancel]")
          .forEach(function (b) {
            b.addEventListener("click", function () { handles && handles.close(true); });
          });
        open = true;
        shadless.h.emit(trigger, "open", "alert-dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "alert-dialog",
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


## Media

Use the `AlertDialogMedia` component to add a media element such as an icon or image to the alert dialog.

<iframe class="demo" src="/demos/alert-dialog-media.html" title="alert-dialog-media" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [alert-dialog-media.html]
<button data-slot="alert-dialog-trigger" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" type="button" aria-haspopup="dialog" aria-expanded="false" aria-controls="d1" data-state="closed" id="d1-trigger">Share Project</button>
<template id="d1-portal">
<div data-state="open" data-slot="alert-dialog-overlay" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50" style="pointer-events: auto;"></div>
<div role="alertdialog" id="d1" aria-describedby="d1-desc" aria-labelledby="d1-title" data-state="open" data-slot="alert-dialog-content" data-size="default" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-4 rounded-xl p-4 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-sm group/alert-dialog-content fixed top-1/2 left-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 outline-none" tabindex="-1" style="pointer-events: auto;"><div data-slot="alert-dialog-header" class="grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-4 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-left sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]"><div data-slot="alert-dialog-media" class="bg-muted mb-2 inline-flex size-10 items-center justify-center rounded-md sm:group-data-[size=default]/alert-dialog-content:row-span-2 *:[svg:not([class*='size-'])]:size-6"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-circle-fading-plus"><path d="M12 2a10 10 0 0 1 7.38 16.75"></path><path d="M12 8v8"></path><path d="M16 12H8"></path><path d="M2.5 8.875a10 10 0 0 0-.5 3"></path><path d="M2.83 16a10 10 0 0 0 2.43 3.4"></path><path d="M4.636 5.235a10 10 0 0 1 .891-.857"></path><path d="M8.644 21.42a10 10 0 0 0 7.631-.38"></path></svg></div><h2 id="d1-title" data-slot="alert-dialog-title" class="text-base font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 cn-font-heading">Share this project?</h2><p id="d1-desc" data-slot="alert-dialog-description" class="text-muted-foreground *:[a]:hover:text-foreground text-sm text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3">Anyone with the link will be able to view and edit this project.</p></div><div data-slot="alert-dialog-footer" class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 sm:flex-row sm:justify-end"><button type="button" data-slot="alert-dialog-cancel" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="outline" data-size="default">Cancel</button><button type="button" data-slot="alert-dialog-action" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="default" data-size="default">Share</button></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/alert-dialog.js
// shadless alert-dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=alert-dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("alert-dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=alert-dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=alert-dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=alert-dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "alert-dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=alert-dialog-overlay]");
        var content = portal.querySelector("[data-slot=alert-dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);
        // alert-dialog semantics: clicking the overlay must NOT dismiss —
        // swallow overlay clicks so wireDialog's portal click-close never fires
        if (overlay) overlay.addEventListener("click", function (e) { e.stopPropagation(); });
        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "alert-dialog");
          },
        });
        // Action / Cancel both dismiss (radix Action/Cancel close the dialog)
        portal.querySelectorAll("[data-slot=alert-dialog-action], [data-slot=alert-dialog-cancel]")
          .forEach(function (b) {
            b.addEventListener("click", function () { handles && handles.close(true); });
          });
        open = true;
        shadless.h.emit(trigger, "open", "alert-dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "alert-dialog",
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


## Small with Media

Use the `size="sm"` prop to make the alert dialog smaller and the `AlertDialogMedia` component to add a media element such as an icon or image to the alert dialog.

<iframe class="demo" src="/demos/alert-dialog-small-media.html" title="alert-dialog-small-media" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [alert-dialog-small-media.html]
<button data-slot="alert-dialog-trigger" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" type="button" aria-haspopup="dialog" aria-expanded="false" aria-controls="d1" data-state="closed" id="d1-trigger">Show Dialog</button>
<template id="d1-portal">
<div data-state="open" data-slot="alert-dialog-overlay" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50" style="pointer-events: auto;"></div>
<div role="alertdialog" id="d1" aria-describedby="d1-desc" aria-labelledby="d1-title" data-state="open" data-slot="alert-dialog-content" data-size="sm" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-4 rounded-xl p-4 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-sm group/alert-dialog-content fixed top-1/2 left-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 outline-none" tabindex="-1" style="pointer-events: auto;"><div data-slot="alert-dialog-header" class="grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-4 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-left sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]"><div data-slot="alert-dialog-media" class="bg-muted mb-2 inline-flex size-10 items-center justify-center rounded-md sm:group-data-[size=default]/alert-dialog-content:row-span-2 *:[svg:not([class*='size-'])]:size-6"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-bluetooth"><path d="m7 7 10 10-5 5V2l5 5L7 17"></path></svg></div><h2 id="d1-title" data-slot="alert-dialog-title" class="text-base font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 cn-font-heading">Allow accessory to connect?</h2><p id="d1-desc" data-slot="alert-dialog-description" class="text-muted-foreground *:[a]:hover:text-foreground text-sm text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3">Do you want to allow the USB accessory to connect to this device?</p></div><div data-slot="alert-dialog-footer" class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 sm:flex-row sm:justify-end"><button type="button" data-slot="alert-dialog-cancel" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="outline" data-size="default">Don't allow</button><button type="button" data-slot="alert-dialog-action" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="default" data-size="default">Allow</button></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/alert-dialog.js
// shadless alert-dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=alert-dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("alert-dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=alert-dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=alert-dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=alert-dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "alert-dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=alert-dialog-overlay]");
        var content = portal.querySelector("[data-slot=alert-dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);
        // alert-dialog semantics: clicking the overlay must NOT dismiss —
        // swallow overlay clicks so wireDialog's portal click-close never fires
        if (overlay) overlay.addEventListener("click", function (e) { e.stopPropagation(); });
        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "alert-dialog");
          },
        });
        // Action / Cancel both dismiss (radix Action/Cancel close the dialog)
        portal.querySelectorAll("[data-slot=alert-dialog-action], [data-slot=alert-dialog-cancel]")
          .forEach(function (b) {
            b.addEventListener("click", function () { handles && handles.close(true); });
          });
        open = true;
        shadless.h.emit(trigger, "open", "alert-dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "alert-dialog",
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


## Destructive

Use the `AlertDialogAction` component to add a destructive action button to the alert dialog.

<iframe class="demo" src="/demos/alert-dialog-destructive.html" title="alert-dialog-destructive" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [alert-dialog-destructive.html]
<button data-slot="alert-dialog-trigger" data-variant="destructive" data-size="default" class="aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-destructive/10 hover:bg-destructive/20 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/20 text-destructive focus-visible:border-destructive/40 dark:hover:bg-destructive/30 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" type="button" aria-haspopup="dialog" aria-expanded="false" aria-controls="d1" data-state="closed" id="d1-trigger">Delete Chat</button>
<template id="d1-portal">
<div data-state="open" data-slot="alert-dialog-overlay" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50" style="pointer-events: auto;"></div>
<div role="alertdialog" id="d1" aria-describedby="d1-desc" aria-labelledby="d1-title" data-state="open" data-slot="alert-dialog-content" data-size="sm" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-4 rounded-xl p-4 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-sm group/alert-dialog-content fixed top-1/2 left-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 outline-none" tabindex="-1" style="pointer-events: auto;"><div data-slot="alert-dialog-header" class="grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-4 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-left sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]"><div data-slot="alert-dialog-media" class="mb-2 inline-flex size-10 items-center justify-center rounded-md sm:group-data-[size=default]/alert-dialog-content:row-span-2 *:[svg:not([class*='size-'])]:size-6 bg-destructive/10 text-destructive dark:bg-destructive/20 dark:text-destructive"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-trash2"><path d="M3 6h18"></path><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path><line x1="10" x2="10" y1="11" y2="17"></line><line x1="14" x2="14" y1="11" y2="17"></line></svg></div><h2 id="d1-title" data-slot="alert-dialog-title" class="text-base font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 cn-font-heading">Delete chat?</h2><p id="d1-desc" data-slot="alert-dialog-description" class="text-muted-foreground *:[a]:hover:text-foreground text-sm text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3">This will permanently delete this chat conversation. View <a href="#">Settings</a> delete any memories saved during this chat.</p></div><div data-slot="alert-dialog-footer" class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 sm:flex-row sm:justify-end"><button type="button" data-slot="alert-dialog-cancel" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="outline" data-size="default">Cancel</button><button type="button" data-slot="alert-dialog-action" class="aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-destructive/10 hover:bg-destructive/20 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/20 text-destructive focus-visible:border-destructive/40 dark:hover:bg-destructive/30 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="destructive" data-size="default">Delete</button></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/alert-dialog.js
// shadless alert-dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=alert-dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("alert-dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=alert-dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=alert-dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=alert-dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "alert-dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=alert-dialog-overlay]");
        var content = portal.querySelector("[data-slot=alert-dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);
        // alert-dialog semantics: clicking the overlay must NOT dismiss —
        // swallow overlay clicks so wireDialog's portal click-close never fires
        if (overlay) overlay.addEventListener("click", function (e) { e.stopPropagation(); });
        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "alert-dialog");
          },
        });
        // Action / Cancel both dismiss (radix Action/Cancel close the dialog)
        portal.querySelectorAll("[data-slot=alert-dialog-action], [data-slot=alert-dialog-cancel]")
          .forEach(function (b) {
            b.addEventListener("click", function () { handles && handles.close(true); });
          });
        open = true;
        shadless.h.emit(trigger, "open", "alert-dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "alert-dialog",
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

<iframe class="demo" src="/demos/alert-dialog-rtl.html" title="alert-dialog-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

:::: details Source

::: code-group
```text [alert-dialog-rtl.html]
<div class="flex gap-4" dir="rtl"><button data-slot="alert-dialog-trigger" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2" type="button" aria-haspopup="dialog" aria-expanded="false" aria-controls="d1" data-state="closed" id="d1-trigger">إظهار الحوار</button><button data-slot="alert-dialog-trigger" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2" type="button" aria-haspopup="dialog" aria-expanded="false" data-state="closed">إظهار الحوار (صغير)</button></div>
<template id="d1-portal">
<div data-state="open" data-slot="alert-dialog-overlay" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50" style="pointer-events: auto;"></div>
<div role="alertdialog" id="d1" aria-describedby="d1-desc" aria-labelledby="d1-title" data-state="open" data-slot="alert-dialog-content" data-size="default" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 bg-popover text-popover-foreground ring-foreground/10 gap-4 rounded-xl p-4 ring-1 duration-100 data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-sm group/alert-dialog-content fixed top-1/2 start-1/2 z-50 grid w-full -translate-x-1/2 rtl:translate-x-1/2 -translate-y-1/2 outline-none" dir="rtl" data-lang="ar" tabindex="-1" style="pointer-events: auto;"><div data-slot="alert-dialog-header" class="grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-4 sm:group-data-[size=default]/alert-dialog-content:place-items-start sm:group-data-[size=default]/alert-dialog-content:text-start sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr]"><h2 id="d1-title" data-slot="alert-dialog-title" class="text-base font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 cn-font-heading">هل أنت متأكد تمامًا؟</h2><p id="d1-desc" data-slot="alert-dialog-description" class="text-muted-foreground *:[a]:hover:text-foreground text-sm text-balance md:text-pretty *:[a]:underline *:[a]:underline-offset-3">لا يمكن التراجع عن هذا الإجراء. سيؤدي هذا إلى حذف حسابك نهائيًا من خوادمنا.</p></div><div data-slot="alert-dialog-footer" class="bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 sm:flex-row sm:justify-end"><button type="button" data-slot="alert-dialog-cancel" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="outline" data-size="default">إلغاء</button><button type="button" data-slot="alert-dialog-action" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2" data-variant="default" data-size="default">متابعة</button></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/alert-dialog.js
// shadless alert-dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=alert-dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("alert-dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=alert-dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=alert-dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=alert-dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "alert-dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=alert-dialog-overlay]");
        var content = portal.querySelector("[data-slot=alert-dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);
        // alert-dialog semantics: clicking the overlay must NOT dismiss —
        // swallow overlay clicks so wireDialog's portal click-close never fires
        if (overlay) overlay.addEventListener("click", function (e) { e.stopPropagation(); });
        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "alert-dialog");
          },
        });
        // Action / Cancel both dismiss (radix Action/Cancel close the dialog)
        portal.querySelectorAll("[data-slot=alert-dialog-action], [data-slot=alert-dialog-cancel]")
          .forEach(function (b) {
            b.addEventListener("click", function () { handles && handles.close(true); });
          });
        open = true;
        shadless.h.emit(trigger, "open", "alert-dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "alert-dialog",
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
| `data-slot="alert-dialog"` |
| `data-slot="alert-dialog-trigger"` |
| `data-slot="alert-dialog-portal"` |
| `data-slot="alert-dialog-overlay"` |
| `data-slot="alert-dialog-content"` |
| `data-slot="alert-dialog-header"` |
| `data-slot="alert-dialog-footer"` |
| `data-slot="alert-dialog-media"` |
| `data-slot="alert-dialog-title"` |
| `data-slot="alert-dialog-description"` |
| `data-slot="alert-dialog-action"` |
| `data-slot="alert-dialog-cancel"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

### size

Use the `size` prop on the `AlertDialogContent` component to control the size of the alert dialog. It accepts the following values:

| Prop   | Type                | Default     |
| ------ | ------------------- | ----------- |
| `size` | `"default" \| "sm"` | `"default"` |

For more information about the other components and their props, see the [Radix UI documentation](https://www.radix-ui.com/primitives/docs/components/alert-dialog#api-reference).
