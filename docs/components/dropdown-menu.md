---
title: "Dropdown Menu"
description: "Displays a menu to the user — such as a set of actions or functions — triggered by a button."
---

# Dropdown Menu

Displays a menu to the user — such as a set of actions or functions — triggered by a button.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/dropdown-menu" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/dropdown-menu#api-reference" rel="noopener">api</a></p>

::::demo dropdown-menu-demo
<iframe class="demo" src="/demos/dropdown-menu.html" title="dropdown-menu-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu.html]
<div>
  <button
    type="button"
    id="d1-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="dropdown-menu-trigger"
    data-radixuigo-menu-trigger="d1"
  >
    Open menu
  </button>
</div>
<template id="d1-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="d1"
    aria-labelledby="d1-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      id="d1-item-1"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Item 1
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Item 2
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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
@import "shadless/dropdown-menu.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/dropdown-menu.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/dropdown-menu.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/dropdown-menu.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/dropdown-menu.js"></script>
```

**Copy the markup from `dist/components/dropdown-menu.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="dropdown-menu-trigger" id="<k>-trigger" data-radixuigo-menu-trigger="<k>">` | opens on clicking |
| `<template id="<k>-tpl">` | holds the `data-slot="dropdown-menu-content"` subtree |
| `<… data-slot="dropdown-menu-sub-trigger" id="<k>s0-trigger" data-radixuigo-menu-subtrigger="<k>s0">` | a sub menu inside a layer; its own `<template id="<k>s0-tpl">` |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/dropdown-menu.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
dropdown-menu
├── dropdown-menu-trigger
└── dropdown-menu-content
    ├── dropdown-menu-group
    │   ├── dropdown-menu-label
    │   ├── dropdown-menu-item
    │   └── dropdown-menu-item
    ├── dropdown-menu-separator
    ├── dropdown-menu-group
    │   ├── dropdown-menu-label
    │   ├── dropdown-menu-checkbox-item
    │   └── dropdown-menu-checkbox-item
    ├── dropdown-menu-separator
    ├── dropdown-menu-group
    │   ├── dropdown-menu-label
    │   └── dropdown-menu-radio-group
    │       ├── dropdown-menu-radio-item
    │       └── dropdown-menu-radio-item
    └── dropdown-menu-sub
        ├── dropdown-menu-sub-trigger
        └── dropdown-menu-sub-content
            └── dropdown-menu-group
                ├── dropdown-menu-label
                ├── dropdown-menu-item
                └── dropdown-menu-item
```

## Basic

A basic dropdown menu with labels and separators.

::::demo dropdown-menu-basic
<iframe class="demo" src="/demos/dropdown-menu-basic.html" title="dropdown-menu-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-basic.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-basic.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Open
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        My Account
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Profile
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Billing
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Settings
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      GitHub
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Support
    </div>
    <div
      role="menuitem"
      aria-disabled="true"
      data-disabled=""
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      API
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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

Use `DropdownMenuSub` to nest secondary actions.

::::demo dropdown-menu-submenu
<iframe class="demo" src="/demos/dropdown-menu-submenu.html" title="dropdown-menu-submenu" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-submenu.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-submenu.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Open
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Team
      </div>
      <div
        role="menuitem"
        id="m0s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s0"
        data-state="closed"
        data-slot="dropdown-menu-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s0"
      >
        Invite users<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right cn-rtl-flip ml-auto"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        New Team<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⌘+T</span
        >
      </div>
    </div>
  </div>
</template>
<template id="m0s0-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0s0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Email
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Message
    </div>
    <div
      role="menuitem"
      id="m0s0s0-trigger"
      aria-haspopup="menu"
      aria-expanded="false"
      aria-controls="m0s0s0"
      data-state="closed"
      data-slot="dropdown-menu-sub-trigger"
      class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
      data-radixuigo-menu-subtrigger="m0s0s0"
    >
      More options<svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="lucide lucide-chevron-right cn-rtl-flip ml-auto"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Advanced...
    </div>
  </div>
</template>
<template id="m0s0s0-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0s0s0"
    aria-labelledby="m0s0-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Calendly
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Slack
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Webhook
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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

Add `DropdownMenuShortcut` to show keyboard hints.

::::demo dropdown-menu-shortcuts
<iframe class="demo" src="/demos/dropdown-menu-shortcuts.html" title="dropdown-menu-shortcuts" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-shortcuts.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-shortcuts.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Open
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        My Account
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Profile<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⇧⌘P</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Billing<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⌘B</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Settings<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⌘S</span
        >
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Log out<span
        data-slot="dropdown-menu-shortcut"
        class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
        >⇧⌘Q</span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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

::::demo dropdown-menu-icons
<iframe class="demo" src="/demos/dropdown-menu-icons.html" title="dropdown-menu-icons" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-icons.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-icons.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Open
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
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
        class="lucide lucide-user"
      >
        <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
        <circle cx="12" cy="7" r="4"></circle></svg
      >Profile
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
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
        class="lucide lucide-credit-card"
      >
        <rect width="20" height="14" x="2" y="5" rx="2"></rect>
        <line x1="2" x2="22" y1="10" y2="10"></line></svg
      >Billing
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
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
        class="lucide lucide-settings"
      >
        <path
          d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
        ></path>
        <circle cx="12" cy="12" r="3"></circle></svg
      >Settings
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="destructive"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
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
        class="lucide lucide-log-out"
      >
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
        <polyline points="16 17 21 12 16 7"></polyline>
        <line x1="21" x2="9" y1="12" y2="12"></line></svg
      >Log out
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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

Use `DropdownMenuCheckboxItem` for toggles.

::::demo dropdown-menu-checkboxes
<iframe class="demo" src="/demos/dropdown-menu-checkboxes.html" title="dropdown-menu-checkboxes" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-checkboxes.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-checkboxes.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Open
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden w-40"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Appearance
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
          ><span data-state="checked"
            ><svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-check"
            >
              <path d="M20 6 9 17l-5-5"></path></svg></span></span
        >Status Bar
      </div>
      <div
        role="menuitemcheckbox"
        aria-disabled="true"
        data-disabled=""
        aria-checked="false"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
        ></span
        >Activity Bar
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="false"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
        ></span
        >Panel
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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


## Checkboxes Icons

Add icons to checkbox items.

::::demo dropdown-menu-checkboxes-icons
<iframe class="demo" src="/demos/dropdown-menu-checkboxes-icons.html" title="dropdown-menu-checkboxes-icons" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-checkboxes-icons.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-checkboxes-icons.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Notifications
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden w-48"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Notification Preferences
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
          ><span data-state="checked"
            ><svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-check"
            >
              <path d="M20 6 9 17l-5-5"></path></svg></span></span
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-mail"
        >
          <rect width="20" height="16" x="2" y="4" rx="2"></rect>
          <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path></svg
        >Email notifications
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="false"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
        ></span
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-message-square"
        >
          <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path></svg
        >SMS notifications
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
          ><span data-state="checked"
            ><svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-check"
            >
              <path d="M20 6 9 17l-5-5"></path></svg></span></span
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-bell"
        >
          <path d="M10.268 21a2 2 0 0 0 3.464 0"></path>
          <path
            d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"
          ></path></svg
        >Push notifications
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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


## Radio Group

Use `DropdownMenuRadioGroup` for exclusive choices.

::::demo dropdown-menu-radio-group
<iframe class="demo" src="/demos/dropdown-menu-radio-group.html" title="dropdown-menu-radio-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-radio-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-radio-group.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Open
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden w-32"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Panel Position
      </div>
      <div role="group" data-slot="dropdown-menu-radio-group">
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          >Top
        </div>
        <div
          role="menuitemradio"
          aria-checked="true"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="checked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
            ><span data-state="checked"
              ><svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-check"
              >
                <path d="M20 6 9 17l-5-5"></path></svg></span></span
          >Bottom
        </div>
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          >Right
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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


## Radio Icons

Show radio options with icons.

::::demo dropdown-menu-radio-icons
<iframe class="demo" src="/demos/dropdown-menu-radio-icons.html" title="dropdown-menu-radio-icons" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-radio-icons.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-radio-icons.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Payment Method
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden min-w-56"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Select Payment Method
      </div>
      <div role="group" data-slot="dropdown-menu-radio-group">
        <div
          role="menuitemradio"
          aria-checked="true"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="checked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
            ><span data-state="checked"
              ><svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-check"
              >
                <path d="M20 6 9 17l-5-5"></path></svg></span></span
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-credit-card"
          >
            <rect width="20" height="14" x="2" y="5" rx="2"></rect>
            <line x1="2" x2="22" y1="10" y2="10"></line></svg
          >Credit Card
        </div>
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-wallet"
          >
            <path
              d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1"
            ></path>
            <path d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4"></path></svg
          >PayPal
        </div>
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-building2"
          >
            <path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z"></path>
            <path d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2"></path>
            <path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2"></path>
            <path d="M10 6h4"></path>
            <path d="M10 10h4"></path>
            <path d="M10 14h4"></path>
            <path d="M10 18h4"></path></svg
          >Bank Transfer
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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

Use `variant="destructive"` for irreversible actions.

::::demo dropdown-menu-destructive
<iframe class="demo" src="/demos/dropdown-menu-destructive.html" title="dropdown-menu-destructive" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-destructive.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-destructive.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Actions
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-pencil"
        >
          <path
            d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"
          ></path>
          <path d="m15 5 4 4"></path></svg
        >Edit
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-share"
        >
          <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path>
          <polyline points="16 6 12 2 8 6"></polyline>
          <line x1="12" x2="12" y1="2" y2="15"></line></svg
        >Share
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="destructive"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-trash"
        >
          <path d="M3 6h18"></path>
          <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
          <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path></svg
        >Delete
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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


## Avatar

An account switcher dropdown triggered by an avatar.

::::demo dropdown-menu-avatar
<iframe class="demo" src="/demos/dropdown-menu-avatar.html" title="dropdown-menu-avatar" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-avatar.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-avatar.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="ghost"
  data-size="icon"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8 rounded-full"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><span
      data-slot="avatar-fallback"
      class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
      >LR</span
    ></span
  >
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="end"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-badge-check"
        >
          <path
            d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"
          ></path>
          <path d="m9 12 2 2 4-4"></path></svg
        >Account
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-credit-card"
        >
          <rect width="20" height="14" x="2" y="5" rx="2"></rect>
          <line x1="2" x2="22" y1="10" y2="10"></line></svg
        >Billing
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-bell"
        >
          <path d="M10.268 21a2 2 0 0 0 3.464 0"></path>
          <path
            d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"
          ></path></svg
        >Notifications
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
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
        class="lucide lucide-log-out"
      >
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
        <polyline points="16 17 21 12 16 7"></polyline>
        <line x1="21" x2="9" y1="12" y2="12"></line></svg
      >Sign Out
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
:::

::::


## Complex

A richer example combining groups, icons, and submenus.

::::demo dropdown-menu-complex
<iframe class="demo" src="/demos/dropdown-menu-complex.html" title="dropdown-menu-complex" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-complex.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [dropdown-menu-complex.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Complex Menu
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden w-44"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        File
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-file"
        >
          <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
          <path d="M14 2v4a2 2 0 0 0 2 2h4"></path></svg
        >New File<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⌘N</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-folder"
        >
          <path
            d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
          ></path></svg
        >New Folder<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⇧⌘N</span
        >
      </div>
      <div
        role="menuitem"
        id="m0s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s0"
        data-state="closed"
        data-slot="dropdown-menu-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s0"
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
          class="lucide lucide-folder-open"
        >
          <path
            d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"
          ></path></svg
        >Open Recent<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right cn-rtl-flip ml-auto"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
      <div
        role="separator"
        aria-orientation="horizontal"
        data-slot="dropdown-menu-separator"
        class="bg-border -mx-1 my-1 h-px"
      ></div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          <path d="M7 3v4a1 1 0 0 0 1 1h7"></path></svg
        >Save<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⌘S</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-download"
        >
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
          <polyline points="7 10 12 15 17 10"></polyline>
          <line x1="12" x2="12" y1="15" y2="3"></line></svg
        >Export<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⇧⌘E</span
        >
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        View
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
          ><span data-state="checked"
            ><svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-check"
            >
              <path d="M20 6 9 17l-5-5"></path></svg></span></span
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-eye"
        >
          <path
            d="M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0"
          ></path>
          <circle cx="12" cy="12" r="3"></circle></svg
        >Show Sidebar
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="false"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
        ></span
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-panels-top-left"
        >
          <rect width="18" height="18" x="3" y="3" rx="2"></rect>
          <path d="M3 9h18"></path>
          <path d="M9 21V9"></path></svg
        >Show Status Bar
      </div>
      <div
        role="menuitem"
        id="m0s1-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s1"
        data-state="closed"
        data-slot="dropdown-menu-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s1"
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
          class="lucide lucide-palette"
        >
          <circle cx="13.5" cy="6.5" r=".5" fill="currentColor"></circle>
          <circle cx="17.5" cy="10.5" r=".5" fill="currentColor"></circle>
          <circle cx="8.5" cy="7.5" r=".5" fill="currentColor"></circle>
          <circle cx="6.5" cy="12.5" r=".5" fill="currentColor"></circle>
          <path
            d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"
          ></path></svg
        >Theme<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right cn-rtl-flip ml-auto"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Account
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-user"
        >
          <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
          <circle cx="12" cy="7" r="4"></circle></svg
        >Profile<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⇧⌘P</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-credit-card"
        >
          <rect width="20" height="14" x="2" y="5" rx="2"></rect>
          <line x1="2" x2="22" y1="10" y2="10"></line></svg
        >Billing
      </div>
      <div
        role="menuitem"
        id="m0s2-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s2"
        data-state="closed"
        data-slot="dropdown-menu-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s2"
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
          class="lucide lucide-settings"
        >
          <path
            d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
          ></path>
          <circle cx="12" cy="12" r="3"></circle></svg
        >Settings<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right cn-rtl-flip ml-auto"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-circle-help"
        >
          <circle cx="12" cy="12" r="10"></circle>
          <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
          <path d="M12 17h.01"></path></svg
        >Help &amp; Support
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-file-text"
        >
          <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
          <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
          <path d="M10 9H8"></path>
          <path d="M16 13H8"></path>
          <path d="M16 17H8"></path></svg
        >Documentation
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="destructive"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-log-out"
        >
          <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
          <polyline points="16 17 21 12 16 7"></polyline>
          <line x1="21" x2="9" y1="12" y2="12"></line></svg
        >Sign Out<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ml-auto text-xs tracking-widest"
          >⇧⌘Q</span
        >
      </div>
    </div>
  </div>
</template>
<template id="m0s0-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0s0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Recent Projects
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-file-code"
        >
          <path d="M10 12.5 8 15l2 2.5"></path>
          <path d="m14 12.5 2 2.5-2 2.5"></path>
          <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
          <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z"></path></svg
        >Project Alpha
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-file-code"
        >
          <path d="M10 12.5 8 15l2 2.5"></path>
          <path d="m14 12.5 2 2.5-2 2.5"></path>
          <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
          <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z"></path></svg
        >Project Beta
      </div>
      <div
        role="menuitem"
        id="m0s0s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s0s0"
        data-state="closed"
        data-slot="dropdown-menu-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s0s0"
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
          class="lucide lucide-ellipsis"
        >
          <circle cx="12" cy="12" r="1"></circle>
          <circle cx="19" cy="12" r="1"></circle>
          <circle cx="5" cy="12" r="1"></circle></svg
        >More Projects<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right cn-rtl-flip ml-auto"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-folder-search"
        >
          <path
            d="M10.7 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v4.1"
          ></path>
          <path d="m21 21-1.9-1.9"></path>
          <circle cx="17" cy="17" r="3"></circle></svg
        >Browse...
      </div>
    </div>
  </div>
</template>
<template id="m0s0s0-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0s0s0"
    aria-labelledby="m0s0-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
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
        class="lucide lucide-file-code"
      >
        <path d="M10 12.5 8 15l2 2.5"></path>
        <path d="m14 12.5 2 2.5-2 2.5"></path>
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z"></path></svg
      >Project Gamma
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
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
        class="lucide lucide-file-code"
      >
        <path d="M10 12.5 8 15l2 2.5"></path>
        <path d="m14 12.5 2 2.5-2 2.5"></path>
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z"></path></svg
      >Project Delta
    </div>
  </div>
</template>
<template id="m0s1-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0s1"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Appearance
      </div>
      <div role="group" data-slot="dropdown-menu-radio-group">
        <div
          role="menuitemradio"
          aria-checked="true"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="checked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
            ><span data-state="checked"
              ><svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-check"
              >
                <path d="M20 6 9 17l-5-5"></path></svg></span></span
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-sun"
          >
            <circle cx="12" cy="12" r="4"></circle>
            <path d="M12 2v2"></path>
            <path d="M12 20v2"></path>
            <path d="m4.93 4.93 1.41 1.41"></path>
            <path d="m17.66 17.66 1.41 1.41"></path>
            <path d="M2 12h2"></path>
            <path d="M20 12h2"></path>
            <path d="m6.34 17.66-1.41 1.41"></path>
            <path d="m19.07 4.93-1.41 1.41"></path></svg
          >Light
        </div>
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-moon"
          >
            <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path></svg
          >Dark
        </div>
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute right-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-monitor"
          >
            <rect width="20" height="14" x="2" y="3" rx="2"></rect>
            <line x1="8" x2="16" y1="21" y2="21"></line>
            <line x1="12" x2="12" y1="17" y2="21"></line></svg
          >System
        </div>
      </div>
    </div>
  </div>
</template>
<template id="m0s2-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0s2"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Preferences
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-keyboard"
        >
          <path d="M10 8h.01"></path>
          <path d="M12 12h.01"></path>
          <path d="M14 8h.01"></path>
          <path d="M16 12h.01"></path>
          <path d="M18 8h.01"></path>
          <path d="M6 8h.01"></path>
          <path d="M7 16h10"></path>
          <path d="M8 12h.01"></path>
          <rect width="20" height="16" x="2" y="4" rx="2"></rect></svg
        >Keyboard Shortcuts
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-languages"
        >
          <path d="m5 8 6 6"></path>
          <path d="m4 14 6-6 2-3"></path>
          <path d="M2 5h12"></path>
          <path d="M7 2h1"></path>
          <path d="m22 22-5-10-5 10"></path>
          <path d="M14 18h6"></path></svg
        >Language
      </div>
      <div
        role="menuitem"
        id="m0s2s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s2s0"
        data-state="closed"
        data-slot="dropdown-menu-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s2s0"
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
          class="lucide lucide-bell"
        >
          <path d="M10.268 21a2 2 0 0 0 3.464 0"></path>
          <path
            d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"
          ></path></svg
        >Notifications<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right cn-rtl-flip ml-auto"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-shield"
        >
          <path
            d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"
          ></path></svg
        >Privacy &amp; Security
      </div>
    </div>
  </div>
</template>
<template id="m0s2s0-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0s2s0"
    aria-labelledby="m0s2-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7"
      >
        Notification Types
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
          ><span data-state="checked"
            ><svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-check"
            >
              <path d="M20 6 9 17l-5-5"></path></svg></span></span
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-bell"
        >
          <path d="M10.268 21a2 2 0 0 0 3.464 0"></path>
          <path
            d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"
          ></path></svg
        >Push Notifications
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute right-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
          ><span data-state="checked"
            ><svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-check"
            >
              <path d="M20 6 9 17l-5-5"></path></svg></span></span
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-mail"
        >
          <rect width="20" height="16" x="2" y="4" rx="2"></rect>
          <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path></svg
        >Email Notifications
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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

::::demo dropdown-menu-rtl
<iframe class="demo" src="/demos/dropdown-menu-rtl.html" title="dropdown-menu-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/dropdown-menu-rtl.html">Open the demo page</a> · <a href="/demos/dropdown-menu-rtl-he.html">HE</a> · <a href="/demos/dropdown-menu-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [dropdown-menu-rtl.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  افتح القائمة
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden w-36"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        id="m0s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s0"
        data-state="closed"
        data-slot="dropdown-menu-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s0"
      >
        الحساب<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right rtl:rotate-180 ms-auto"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:ps-7"
      >
        الفريق
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        الفريق
      </div>
      <div
        role="menuitem"
        id="m0s1-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s1"
        data-state="closed"
        data-slot="dropdown-menu-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s1"
      >
        دعوة المستخدمين<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right rtl:rotate-180 ms-auto"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        فريق جديد<span
          data-slot="dropdown-menu-shortcut"
          class="text-muted-foreground group-focus/dropdown-menu-item:text-accent-foreground ms-auto text-xs tracking-widest"
          >⌘+T</span
        >
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:ps-7"
      >
        عرض
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute end-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
          ><span data-state="checked"
            ><svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-check"
            >
              <path d="M20 6 9 17l-5-5"></path></svg></span></span
        >شريط الحالة
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="false"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute end-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
        ></span
        >شريط النشاط
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="false"
        data-slot="dropdown-menu-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="absolute end-2 flex items-center justify-center pointer-events-none"
          data-slot="dropdown-menu-checkbox-item-indicator"
        ></span
        >اللوحة
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        data-slot="dropdown-menu-label"
        class="text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:ps-7"
      >
        الموضع
      </div>
      <div role="group" data-slot="dropdown-menu-radio-group">
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute end-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          >أعلى
        </div>
        <div
          role="menuitemradio"
          aria-checked="true"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="checked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute end-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
            ><span data-state="checked"
              ><svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-check"
              >
                <path d="M20 6 9 17l-5-5"></path></svg></span></span
          >أسفل
        </div>
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute end-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          >يمين
        </div>
        <div
          role="menuitemradio"
          aria-checked="false"
          data-slot="dropdown-menu-radio-item"
          class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-state="unchecked"
          tabindex="-1"
          data-orientation="vertical"
          data-radix-collection-item=""
        >
          <span
            class="absolute end-2 flex items-center justify-center pointer-events-none"
            data-slot="dropdown-menu-radio-item-indicator"
          ></span
          >يسار
        </div>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="destructive"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        تسجيل الخروج
      </div>
    </div>
  </div>
</template>
<template id="m0s0-tpl">
  <div
    data-side="left"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m0s0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-user"
        >
          <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
          <circle cx="12" cy="7" r="4"></circle></svg
        >الملف الشخصي
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-credit-card"
        >
          <rect width="20" height="14" x="2" y="5" rx="2"></rect>
          <line x1="2" x2="22" y1="10" y2="10"></line></svg
        >الفوترة
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
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
          class="lucide lucide-settings"
        >
          <path
            d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
          ></path>
          <circle cx="12" cy="12" r="3"></circle></svg
        >الإعدادات
      </div>
    </div>
  </div>
</template>
<template id="m0s1-tpl">
  <div
    data-side="left"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m0s1"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      البريد الإلكتروني
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      رسالة
    </div>
    <div
      role="menuitem"
      id="m0s1s0-trigger"
      aria-haspopup="menu"
      aria-expanded="false"
      aria-controls="m0s1s0"
      data-state="closed"
      data-slot="dropdown-menu-sub-trigger"
      class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
      data-radixuigo-menu-subtrigger="m0s1s0"
    >
      المزيد<svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="lucide lucide-chevron-right rtl:rotate-180 ms-auto"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      متقدم...
    </div>
  </div>
</template>
<template id="m0s1s0-tpl">
  <div
    data-side="left"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m0s1s0"
    aria-labelledby="m0s1-trigger"
    data-slot="dropdown-menu-sub-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-[96px] rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-dropdown-menu-content-transform-origin) overflow-hidden"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      pointer-events: auto;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      تقويم
    </div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      دردشة
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      خطاف ويب
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// generated: shadless menu behavior (wireMenu) — dropdown-menu & context-menu
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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
| `data-slot="dropdown-menu"` |
| `data-slot="dropdown-menu-portal"` |
| `data-slot="dropdown-menu-trigger"` |
| `data-slot="dropdown-menu-content"` |
| `data-slot="dropdown-menu-group"` |
| `data-slot="dropdown-menu-item"` |
| `data-slot="dropdown-menu-checkbox-item"` |
| `data-slot="dropdown-menu-checkbox-item-indicator"` |
| `data-slot="dropdown-menu-radio-group"` |
| `data-slot="dropdown-menu-radio-item"` |
| `data-slot="dropdown-menu-radio-item-indicator"` |
| `data-slot="dropdown-menu-label"` |
| `data-slot="dropdown-menu-separator"` |
| `data-slot="dropdown-menu-shortcut"` |
| `data-slot="dropdown-menu-sub"` |
| `data-slot="dropdown-menu-sub-trigger"` |
| `data-slot="dropdown-menu-sub-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI documentation](https://www.radix-ui.com/docs/primitives/components/dropdown-menu) for the full API reference.
