---
title: "Context Menu"
description: "Displays a menu of actions triggered by a right click."
---

# Context Menu

Displays a menu of actions triggered by a right click.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/context-menu" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/context-menu#api-reference" rel="noopener">api</a></p>

<iframe class="demo" src="/demos/context-menu.html" title="context-menu-demo" data-status="existing-dist" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu.html]
<div>
<span data-state="closed" data-slot="context-menu-trigger" class="select-none" id="d1-trigger" data-radixuigo-context-trigger="d1">Right click here</span>
</div>
<template id="d1-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" id="d1-item-1" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Item 1</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Item 2</div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
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
@import "shadless/context-menu.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/context-menu.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/context-menu.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/context-menu.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/context-menu.js"></script>
```

**Copy the markup from                                     into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="context-menu-trigger" id="<k>-trigger" data-radixuigo-context-trigger="<k>">` | opens on right-clicking |
| `<template id="<k>-tpl">` | holds the `data-slot="context-menu-content"` subtree |
| `<… data-slot="context-menu-sub-trigger" id="<k>s0-trigger" data-radixuigo-menu-subtrigger="<k>s0">` | a sub menu inside a layer; its own `<template id="<k>s0-tpl">` |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/context-menu.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
context-menu
├── context-menu-trigger
└── context-menu-content
    ├── context-menu-group
    │   ├── context-menu-label
    │   ├── context-menu-item
    │   └── context-menu-item
    ├── context-menu-separator
    ├── context-menu-group
    │   ├── context-menu-label
    │   ├── context-menu-checkbox-item
    │   └── context-menu-checkbox-item
    ├── context-menu-separator
    ├── context-menu-group
    │   ├── context-menu-label
    │   └── context-menu-radio-group
    │       ├── context-menu-radio-item
    │       └── context-menu-radio-item
    └── context-menu-sub
        ├── context-menu-sub-trigger
        └── context-menu-sub-content
            └── context-menu-group
                ├── context-menu-item
                └── context-menu-item
```

## Basic

A simple context menu with a few actions.

<iframe class="demo" src="/demos/context-menu-basic.html" title="context-menu-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu-basic.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">Right click here</span><span class="hidden pointer-coarse:inline-block">Long press here</span></span>
<template id="m0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Back</div><div role="menuitem" aria-disabled="true" data-disabled="" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Forward</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Reload</div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()
```
:::

::::


## Submenu

Use `ContextMenuSub` to nest secondary actions.

<iframe class="demo" src="/demos/context-menu-submenu.html" title="context-menu-submenu" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu-submenu.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">Right click here</span><span class="hidden pointer-coarse:inline-block">Long press here</span></span>
<template id="m0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Copy<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘C</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Cut<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘X</span></div></div><div role="menuitem" id="m0s0-trigger" aria-haspopup="menu" aria-expanded="false" aria-controls="m0s0" data-state="closed" data-slot="context-menu-sub-trigger" class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="" data-radixuigo-menu-subtrigger="m0s0">More Tools<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevron-right cn-rtl-flip ml-auto"><path d="m9 18 6-6-6-6"></path></svg></div></div>
</template>
<template id="m0s0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" id="m0s0" aria-labelledby="m0-trigger" data-slot="context-menu-sub-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 bg-popover text-popover-foreground min-w-32 rounded-lg border p-1 shadow-lg duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-context-menu-content-transform-origin) overflow-hidden" tabindex="-1" data-orientation="vertical" style="outline: none; pointer-events: auto; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height);"><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Save Page...</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Create Shortcut...</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Name Window...</div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Developer Tools</div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="destructive" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Delete</div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()
```
:::

::::


## Shortcuts

Add `ContextMenuShortcut` to show keyboard hints.

<iframe class="demo" src="/demos/context-menu-shortcuts.html" title="context-menu-shortcuts" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu-shortcuts.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">Right click here</span><span class="hidden pointer-coarse:inline-block">Long press here</span></span>
<template id="m0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Back<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘[</span></div><div role="menuitem" aria-disabled="true" data-disabled="" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Forward<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘]</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Reload<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘R</span></div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Save<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘S</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Save As...<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⇧⌘S</span></div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()
```
:::

::::


## Groups

Group related actions and separate them with dividers.

<iframe class="demo" src="/demos/context-menu-groups.html" title="context-menu-groups" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu-groups.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">Right click here</span><span class="hidden pointer-coarse:inline-block">Long press here</span></span>
<template id="m0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div data-slot="context-menu-label" class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7">File</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">New File<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘N</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Open File<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘O</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Save<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘S</span></div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div data-slot="context-menu-label" class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7">Edit</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Undo<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘Z</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Redo<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⇧⌘Z</span></div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Cut<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘X</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Copy<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘C</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Paste<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌘V</span></div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="destructive" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">Delete<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest">⌫</span></div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()
```
:::

::::


## Icons

Combine icons with labels for quick scanning.

<iframe class="demo" src="/demos/context-menu-icons.html" title="context-menu-icons" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu-icons.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">Right click here</span><span class="hidden pointer-coarse:inline-block">Long press here</span></span>
<template id="m0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-copy"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"></rect><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path></svg>Copy</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-scissors"><circle cx="6" cy="6" r="3"></circle><path d="M8.12 8.12 12 12"></path><path d="M20 4 8.12 15.88"></path><circle cx="6" cy="18" r="3"></circle><path d="M14.8 14.8 20 20"></path></svg>Cut</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-clipboard-paste"><path d="M15 2H9a1 1 0 0 0-1 1v2c0 .6.4 1 1 1h6c.6 0 1-.4 1-1V3c0-.6-.4-1-1-1Z"></path><path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2M16 4h2a2 2 0 0 1 2 2v2M11 14h10"></path><path d="m17 10 4 4-4 4"></path></svg>Paste</div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="destructive" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-trash"><path d="M3 6h18"></path><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path></svg>Delete</div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()
```
:::

::::


## Checkboxes

Use `ContextMenuCheckboxItem` for toggles.

<iframe class="demo" src="/demos/context-menu-checkboxes.html" title="context-menu-checkboxes" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu-checkboxes.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">Right click here</span><span class="hidden pointer-coarse:inline-block">Long press here</span></span>
<template id="m0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div role="menuitemcheckbox" aria-checked="false" data-slot="context-menu-checkbox-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="unchecked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute right-2 pointer-events-none"></span>Show Bookmarks Bar</div><div role="menuitemcheckbox" aria-checked="false" data-slot="context-menu-checkbox-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="unchecked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute right-2 pointer-events-none"></span>Show Full URLs</div><div role="menuitemcheckbox" aria-checked="false" data-slot="context-menu-checkbox-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="unchecked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute right-2 pointer-events-none"></span>Show Developer Tools</div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()
```
:::

::::


## Radio

Use `ContextMenuRadioItem` for exclusive choices.

<iframe class="demo" src="/demos/context-menu-radio.html" title="context-menu-radio" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu-radio.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">Right click here</span><span class="hidden pointer-coarse:inline-block">Long press here</span></span>
<template id="m0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div data-slot="context-menu-label" class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7">People</div><div role="group" data-slot="context-menu-radio-group"><div role="menuitemradio" aria-checked="true" data-slot="context-menu-radio-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="checked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute right-2 pointer-events-none"><span data-state="checked"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><path d="M20 6 9 17l-5-5"></path></svg></span></span>Pedro Duarte</div><div role="menuitemradio" aria-checked="false" data-slot="context-menu-radio-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="unchecked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute right-2 pointer-events-none"></span>Colm Tuite</div></div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div data-slot="context-menu-label" class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7">Theme</div><div role="group" data-slot="context-menu-radio-group"><div role="menuitemradio" aria-checked="true" data-slot="context-menu-radio-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="checked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute right-2 pointer-events-none"><span data-state="checked"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><path d="M20 6 9 17l-5-5"></path></svg></span></span>Light</div><div role="menuitemradio" aria-checked="false" data-slot="context-menu-radio-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="unchecked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute right-2 pointer-events-none"></span>Dark</div><div role="menuitemradio" aria-checked="false" data-slot="context-menu-radio-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="unchecked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute right-2 pointer-events-none"></span>System</div></div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()
```
:::

::::


## Destructive

Use `variant="destructive"` to style the menu item as destructive.

<iframe class="demo" src="/demos/context-menu-destructive.html" title="context-menu-destructive" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

:::: details Source

::: code-group
```text [context-menu-destructive.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">Right click here</span><span class="hidden pointer-coarse:inline-block">Long press here</span></span>
<template id="m0-tpl">
<div data-side="right" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="ltr" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-pencil"><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"></path><path d="m15 5 4 4"></path></svg>Edit</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-share"><path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path><polyline points="16 6 12 2 8 6"></polyline><line x1="12" x2="12" y1="2" y2="15"></line></svg>Share</div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="destructive" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-trash"><path d="M3 6h18"></path><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path></svg>Delete</div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
    });
  } })
})()
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/context-menu-rtl.html" title="context-menu-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

:::: details Source

::: code-group
```text [context-menu-rtl.html]
<span data-state="closed" data-slot="context-menu-trigger" class="select-none flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm" id="m0-trigger" data-radixuigo-context-trigger="m0"><span class="hidden pointer-fine:inline-block">انقر بزر الماوس الأيمن هنا</span><span class="hidden pointer-coarse:inline-block">اضغط مطولاً هنا</span></span>
<template id="m0-tpl">
<div data-side="left" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="rtl" data-slot="context-menu-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto w-48" data-lang="ar" tabindex="-1" data-orientation="vertical" style="outline: none; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height); pointer-events: auto;"><div role="group" data-slot="context-menu-group"><div role="menuitem" id="m0s0-trigger" aria-haspopup="menu" aria-expanded="false" aria-controls="m0s0" data-state="closed" data-slot="context-menu-sub-trigger" class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="" data-radixuigo-menu-subtrigger="m0s0">التنقل<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevron-right rtl:rotate-180 ms-auto"><path d="m9 18 6-6-6-6"></path></svg></div><div role="menuitem" id="m0s1-trigger" aria-haspopup="menu" aria-expanded="false" aria-controls="m0s1" data-state="closed" data-slot="context-menu-sub-trigger" class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="" data-radixuigo-menu-subtrigger="m0s1">المزيد من الأدوات<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevron-right rtl:rotate-180 ms-auto"><path d="m9 18 6-6-6-6"></path></svg></div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitemcheckbox" aria-checked="true" data-slot="context-menu-checkbox-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="checked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute end-2 pointer-events-none"><span data-state="checked"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><path d="M20 6 9 17l-5-5"></path></svg></span></span>إظهار الإشارات المرجعية</div><div role="menuitemcheckbox" aria-checked="false" data-slot="context-menu-checkbox-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="unchecked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute end-2 pointer-events-none"></span>إظهار عناوين URL الكاملة</div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="group" data-slot="context-menu-radio-group"><div data-slot="context-menu-label" class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:ps-7">الأشخاص</div><div role="menuitemradio" aria-checked="true" data-slot="context-menu-radio-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="checked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute end-2 pointer-events-none"><span data-state="checked"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><path d="M20 6 9 17l-5-5"></path></svg></span></span>Pedro Duarte</div><div role="menuitemradio" aria-checked="false" data-slot="context-menu-radio-item" class="focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" data-state="unchecked" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><span class="absolute end-2 pointer-events-none"></span>Colm Tuite</div></div></div></div>
</template>
<template id="m0s0-tpl">
<div data-side="left" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="rtl" id="m0s0" aria-labelledby="m0-trigger" data-slot="context-menu-sub-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 bg-popover text-popover-foreground min-w-32 rounded-lg border p-1 shadow-lg duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-context-menu-content-transform-origin) overflow-hidden w-44" data-lang="ar" tabindex="-1" data-orientation="vertical" style="outline: none; pointer-events: auto; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height);"><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-left"><path d="m12 19-7-7 7-7"></path><path d="M19 12H5"></path></svg>رجوع<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ms-auto text-xs tracking-widest">⌘[</span></div><div role="menuitem" aria-disabled="true" data-disabled="" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-right"><path d="M5 12h14"></path><path d="m12 5 7 7-7 7"></path></svg>تقدم<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ms-auto text-xs tracking-widest">⌘]</span></div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item=""><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-rotate-cw"><path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path><path d="M21 3v5h-5"></path></svg>إعادة تحميل<span data-slot="context-menu-shortcut" class="text-muted-foreground group-focus/context-menu-item:text-accent-foreground ms-auto text-xs tracking-widest">⌘R</span></div></div></div>
</template>
<template id="m0s1-tpl">
<div data-side="left" data-align="start" role="menu" aria-orientation="vertical" data-state="open" data-radix-menu-content="" dir="rtl" id="m0s1" aria-labelledby="m0-trigger" data-slot="context-menu-sub-content" class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 bg-popover text-popover-foreground min-w-32 rounded-lg border p-1 shadow-lg duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-context-menu-content-transform-origin) overflow-hidden w-44" data-lang="ar" tabindex="-1" data-orientation="vertical" style="outline: none; pointer-events: auto; --radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height);"><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">حفظ الصفحة...</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">إنشاء اختصار...</div><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">تسمية النافذة...</div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="default" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">أدوات المطور</div></div><div role="separator" aria-orientation="horizontal" data-slot="context-menu-separator" class="bg-border -mx-1 my-1 h-px"></div><div role="group" data-slot="context-menu-group"><div role="menuitem" data-slot="context-menu-item" data-variant="destructive" class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0" tabindex="-1" data-orientation="vertical" data-radix-collection-item="">حذف</div></div></div>
</template>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/context-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("context-menu", { init: function (root) {
    if (shadless.__menuWired.installed_menu) { if (shadless.__menuWired.fileTriggers) shadless.__menuWired.fileTriggers(root); return }
    shadless.__menuWired.installed_menu = true
    // shadless:open / shadless:close: the kernel has no open hook, so every
    // path into it is followed by sync(), which diffs the ROOT layer's
    // trigger (sub menus do not emit) and dispatches the edges
    var openTrigger = null;
    var sync = function () {
      var l = handles.rootLayer();
      var t = l ? l.trigger : null;
      if (t === openTrigger) return;
      var prev = openTrigger;
      openTrigger = t;
      if (prev) shadless.h.emit(prev, "close", prev.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
      if (t) shadless.h.emit(t, "open", t.hasAttribute("data-radixuigo-context-trigger") ? "context-menu" : "dropdown-menu");
    };
    var handles = RadixKernel.wireMenu({
      // radix keeps the trigger out of the background aria-hidden set and locks
      // scroll while a root menu is open — kernel wireMenu does neither
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger")
          || el.hasAttribute("data-radixuigo-context-trigger");
      },
      onAllClosed: function () {
        document.body.removeAttribute("data-scroll-locked");
        document.body.style.removeProperty("pointer-events");
        sync();
      },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        document.body.setAttribute("data-scroll-locked", "1");
        document.body.style.setProperty("pointer-events", "none");
        // modal body lock inherits down — re-enable hit-testing in the layer
        // (radix sets pointer-events:auto on content while open)
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });
    document.addEventListener("click", function (e) {
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });
    // programmatic handles, one per trigger (found by the kernel's protocol
    // attributes; a later shadless.init(root) files any new ones)
    var noop = function () {}
    var fileTriggers = function (root) {
      Array.prototype.forEach.call((root || document).querySelectorAll("[data-radixuigo-menu-trigger], [data-radixuigo-context-trigger]"), function (t) {
        if (shadless.instances.has(t)) return
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "context-menu",
          open: openIt,
          close: function () { if (isOpen()) { handles.closeAll(); sync() } },
          toggle: function () { if (isOpen()) { handles.closeAll(); sync() } else openIt() },
          isOpen: isOpen,
        })
      })
    }
    fileTriggers(document)
    shadless.__menuWired.fileTriggers = fileTriggers
    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel only opens it on click / ArrowRight (onDocumentClick has the sub
    // path) — route pointer entry through the same path. openMenuLayer is
    // idempotent for an already-open layer.
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });
    document.addEventListener("contextmenu", function (e) {
      handles.onContextmenu(e.target, e.clientX, e.clientY, function () { e.preventDefault(); });
      sync();
    });
    document.addEventListener("keydown", function (e) {
      handles.onKeydown(e.target, e.key, function () { e.preventDefault(); });
      sync();
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
| `data-slot="context-menu"` |
| `data-slot="context-menu-trigger"` |
| `data-slot="context-menu-group"` |
| `data-slot="context-menu-portal"` |
| `data-slot="context-menu-sub"` |
| `data-slot="context-menu-radio-group"` |
| `data-slot="context-menu-content"` |
| `data-slot="context-menu-item"` |
| `data-slot="context-menu-sub-trigger"` |
| `data-slot="context-menu-sub-content"` |
| `data-slot="context-menu-checkbox-item"` |
| `data-slot="context-menu-radio-item"` |
| `data-slot="context-menu-label"` |
| `data-slot="context-menu-separator"` |
| `data-slot="context-menu-shortcut"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI](https://www.radix-ui.com/docs/primitives/components/context-menu#api-reference) documentation for more information.
