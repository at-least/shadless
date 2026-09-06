---
title: "Navigation Menu"
description: "A collection of links for navigating websites."
---

# Navigation Menu

A collection of links for navigating websites.

::::demo navigation-menu-demo
<iframe class="demo" src="/demos/navigation-menu-demo.html" title="navigation-menu-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/navigation-menu-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [navigation-menu-demo.html]
<nav
  aria-label="Main"
  data-orientation="horizontal"
  dir="ltr"
  data-slot="navigation-menu"
  data-viewport="true"
  class="group/navigation-menu relative flex max-w-max flex-1 items-center justify-center"
>
  <div style="position: relative">
    <ul
      data-orientation="horizontal"
      data-slot="navigation-menu-list"
      class="gap-0 group flex flex-1 list-none items-center justify-center"
      dir="ltr"
    >
      <li data-slot="navigation-menu-item" class="relative">
        <button
          id="n0-trigger"
          data-state="closed"
          aria-expanded="false"
          aria-controls="n0-content"
          data-slot="navigation-menu-trigger"
          class="hover:bg-muted focus:bg-muted data-open:hover:bg-muted data-open:focus:bg-muted data-open:bg-muted/50 focus-visible:ring-ring/50 data-popup-open:bg-muted/50 data-popup-open:hover:bg-muted rounded-lg px-2.5 py-1.5 text-sm font-medium transition-all focus-visible:ring-3 focus-visible:outline-1 disabled:opacity-50 group/navigation-menu-trigger inline-flex h-9 w-max items-center justify-center outline-none disabled:pointer-events-none group"
          data-radix-collection-item=""
          data-radixuigo-nav-trigger="n0"
        >
          Getting started
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
            class="lucide lucide-chevron-down relative top-px ml-1 size-3 transition duration-300 group-data-open/navigation-menu-trigger:rotate-180 group-data-popup-open/navigation-menu-trigger:rotate-180"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
        </button>
      </li>
      <li data-slot="navigation-menu-item" class="relative hidden md:flex">
        <button
          id="n1-trigger"
          data-state="closed"
          aria-expanded="false"
          aria-controls="n1-content"
          data-slot="navigation-menu-trigger"
          class="hover:bg-muted focus:bg-muted data-open:hover:bg-muted data-open:focus:bg-muted data-open:bg-muted/50 focus-visible:ring-ring/50 data-popup-open:bg-muted/50 data-popup-open:hover:bg-muted rounded-lg px-2.5 py-1.5 text-sm font-medium transition-all focus-visible:ring-3 focus-visible:outline-1 disabled:opacity-50 group/navigation-menu-trigger inline-flex h-9 w-max items-center justify-center outline-none disabled:pointer-events-none group"
          data-radix-collection-item=""
          data-radixuigo-nav-trigger="n1"
        >
          Components
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
            class="lucide lucide-chevron-down relative top-px ml-1 size-3 transition duration-300 group-data-open/navigation-menu-trigger:rotate-180 group-data-popup-open/navigation-menu-trigger:rotate-180"
          >
            <path d="m6 9 6 6 6-6"></path>
          </svg>
        </button>
      </li>
      <li data-slot="navigation-menu-item" class="relative">
        <a
          href="/docs"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 gap-2 p-2 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 hover:bg-muted focus:bg-muted data-open:hover:bg-muted data-open:focus:bg-muted data-open:bg-muted/50 focus-visible:ring-ring/50 data-popup-open:bg-muted/50 data-popup-open:hover:bg-muted rounded-lg px-2.5 py-1.5 text-sm font-medium transition-all focus-visible:ring-3 focus-visible:outline-1 disabled:opacity-50 group/navigation-menu-trigger inline-flex h-9 w-max items-center justify-center outline-none disabled:pointer-events-none"
          data-radix-collection-item=""
          >Docs</a
        >
      </li>
    </ul>
  </div>
  <div class="absolute top-full left-0 isolate z-50 flex justify-center"></div>
</nav>
<template id="n0-content-tpl">
  <div
    id="n0-content"
    aria-labelledby="n0-trigger"
    data-orientation="horizontal"
    data-slot="navigation-menu-content"
    class="data-[motion^=from-]:animate-in data-[motion^=to-]:animate-out data-[motion^=from-]:fade-in data-[motion^=to-]:fade-out data-[motion=from-end]:slide-in-from-right-52 data-[motion=from-start]:slide-in-from-left-52 data-[motion=to-end]:slide-out-to-right-52 data-[motion=to-start]:slide-out-to-left-52 group-data-[viewport=false]/navigation-menu:bg-popover group-data-[viewport=false]/navigation-menu:text-popover-foreground group-data-[viewport=false]/navigation-menu:data-open:animate-in group-data-[viewport=false]/navigation-menu:data-closed:animate-out group-data-[viewport=false]/navigation-menu:data-closed:zoom-out-95 group-data-[viewport=false]/navigation-menu:data-open:zoom-in-95 group-data-[viewport=false]/navigation-menu:data-open:fade-in-0 group-data-[viewport=false]/navigation-menu:data-closed:fade-out-0 group-data-[viewport=false]/navigation-menu:ring-foreground/10 p-1 ease-[cubic-bezier(0.22,1,0.36,1)] group-data-[viewport=false]/navigation-menu:rounded-lg group-data-[viewport=false]/navigation-menu:shadow group-data-[viewport=false]/navigation-menu:ring-1 group-data-[viewport=false]/navigation-menu:duration-300 top-0 left-0 w-full group-data-[viewport=false]/navigation-menu:top-full group-data-[viewport=false]/navigation-menu:mt-1.5 group-data-[viewport=false]/navigation-menu:overflow-hidden **:data-[slot=navigation-menu-link]:focus:ring-0 **:data-[slot=navigation-menu-link]:focus:outline-none md:absolute md:w-auto"
    dir="ltr"
  >
    <ul class="w-96">
      <li>
        <a
          href="/docs"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Introduction</div>
            <div class="line-clamp-2 text-muted-foreground">
              Re-usable components built with Tailwind CSS.
            </div>
          </div></a
        >
      </li>
      <li>
        <a
          href="/docs/installation"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Installation</div>
            <div class="line-clamp-2 text-muted-foreground">
              How to install dependencies and structure your app.
            </div>
          </div></a
        >
      </li>
      <li>
        <a
          href="/docs/primitives/typography"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Typography</div>
            <div class="line-clamp-2 text-muted-foreground">
              Styles for headings, paragraphs, lists...etc
            </div>
          </div></a
        >
      </li>
    </ul>
  </div>
</template>
<template id="n1-content-tpl">
  <div
    id="n1-content"
    aria-labelledby="n1-trigger"
    data-orientation="horizontal"
    data-slot="navigation-menu-content"
    class="data-[motion^=from-]:animate-in data-[motion^=to-]:animate-out data-[motion^=from-]:fade-in data-[motion^=to-]:fade-out data-[motion=from-end]:slide-in-from-right-52 data-[motion=from-start]:slide-in-from-left-52 data-[motion=to-end]:slide-out-to-right-52 data-[motion=to-start]:slide-out-to-left-52 group-data-[viewport=false]/navigation-menu:bg-popover group-data-[viewport=false]/navigation-menu:text-popover-foreground group-data-[viewport=false]/navigation-menu:data-open:animate-in group-data-[viewport=false]/navigation-menu:data-closed:animate-out group-data-[viewport=false]/navigation-menu:data-closed:zoom-out-95 group-data-[viewport=false]/navigation-menu:data-open:zoom-in-95 group-data-[viewport=false]/navigation-menu:data-open:fade-in-0 group-data-[viewport=false]/navigation-menu:data-closed:fade-out-0 group-data-[viewport=false]/navigation-menu:ring-foreground/10 p-1 ease-[cubic-bezier(0.22,1,0.36,1)] group-data-[viewport=false]/navigation-menu:rounded-lg group-data-[viewport=false]/navigation-menu:shadow group-data-[viewport=false]/navigation-menu:ring-1 group-data-[viewport=false]/navigation-menu:duration-300 top-0 left-0 w-full group-data-[viewport=false]/navigation-menu:top-full group-data-[viewport=false]/navigation-menu:mt-1.5 group-data-[viewport=false]/navigation-menu:overflow-hidden **:data-[slot=navigation-menu-link]:focus:ring-0 **:data-[slot=navigation-menu-link]:focus:outline-none md:absolute md:w-auto"
    dir="ltr"
  >
    <ul class="grid w-[400px] gap-2 md:w-[500px] md:grid-cols-2 lg:w-[600px]">
      <li>
        <a
          href="/docs/primitives/alert-dialog"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Alert Dialog</div>
            <div class="line-clamp-2 text-muted-foreground">
              A modal dialog that interrupts the user with important content and expects a response.
            </div>
          </div></a
        >
      </li>
      <li>
        <a
          href="/docs/primitives/hover-card"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Hover Card</div>
            <div class="line-clamp-2 text-muted-foreground">
              For sighted users to preview content available behind a link.
            </div>
          </div></a
        >
      </li>
      <li>
        <a
          href="/docs/primitives/progress"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Progress</div>
            <div class="line-clamp-2 text-muted-foreground">
              Displays an indicator showing the completion progress of a task, typically displayed
              as a progress bar.
            </div>
          </div></a
        >
      </li>
      <li>
        <a
          href="/docs/primitives/scroll-area"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Scroll-area</div>
            <div class="line-clamp-2 text-muted-foreground">
              Visually or semantically separates content.
            </div>
          </div></a
        >
      </li>
      <li>
        <a
          href="/docs/primitives/tabs"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Tabs</div>
            <div class="line-clamp-2 text-muted-foreground">
              A set of layered sections of content—known as tab panels—that are displayed one at a
              time.
            </div>
          </div></a
        >
      </li>
      <li>
        <a
          href="/docs/primitives/tooltip"
          data-slot="navigation-menu-link"
          class="data-active:focus:bg-muted data-active:hover:bg-muted data-active:bg-muted/50 focus-visible:ring-ring/50 hover:bg-muted focus:bg-muted flex items-center gap-2 rounded-lg p-2 text-sm transition-all outline-none focus-visible:ring-3 focus-visible:outline-1 in-data-[slot=navigation-menu-content]:rounded-md [&amp;_svg:not([class*='size-'])]:size-4"
          data-radix-collection-item=""
          ><div class="flex flex-col gap-1 text-sm">
            <div class="leading-none font-medium">Tooltip</div>
            <div class="line-clamp-2 text-muted-foreground">
              A popup that displays information related to an element when the element receives
              keyboard focus or the mouse hovers over it.
            </div>
          </div></a
        >
      </li>
    </ul>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/navigation-menu.js
// generated: shadless navigation-menu behavior — shared-viewport menu on the
// kernel. Measured radix semantics (oracle-driven):
//   - the viewport lives INSIDE the nav root (in flow, below the list) and
//     is fully unmounted when closed (no empty container in the DOM);
//   - content mounts into the viewport from the item's template, carries
//     data-state + aria-labelledby and receives aria-controls on the trigger;
//   - focus STAYS on the trigger while open (unlike menubar, which moves it
//     into the content); horizontal arrows only move focus between triggers
//     (no auto-switch, no roving tabindex — triggers keep natural focus);
//   - Escape / outside click close; size reaches CSS through the
//     --radix-navigation-menu-viewport-{width,height} vars.
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["navigation-menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["navigation-menu"] = true
  shadless.register("navigation-menu", { init: function () {
    if (shadless.__menuWired.installed_navigation_menu) return
    shadless.__menuWired.installed_navigation_menu = true
    var EXIT = 120;
    var idOf = function (t) { return t.getAttribute("data-radixuigo-nav-trigger"); };

    // One state set PER ROOT. This used to be a single
    // document.querySelector("[data-slot=navigation-menu]"), so a second nav
    // on the page got no handle and clicking its trigger mounted the content
    // into the FIRST nav's viewport — while the protocol table promised
    // "wires every instance it finds — several per page".
    var navs = [];
    function navOf(el) {
      var r = el && el.closest && el.closest("[data-slot=navigation-menu]");
      for (var i = 0; i < navs.length; i++) if (navs[i].root === r) return navs[i];
      return null;
    }

    function wire(root) {
    var triggers = function () {
      return Array.prototype.slice.call(root.querySelectorAll("[data-radixuigo-nav-trigger]"));
    };
    var viewport = function () { return root.querySelector("[data-slot=navigation-menu-viewport]"); };
    var openId = null;
    var exitTimer = null;

    function ensureViewport() {
      var vp = viewport()
      if (vp) return vp
      vp = document.createElement("div")
      // data-slot alone is enough: dist/css/navigation-menu.css already
      // ships [data-slot="navigation-menu-viewport"] { @apply … } — the
      // rule is compiled from upstream regardless of whether any static
      // page happens to render this element, same as every other
      // dynamically-mounted node in this codebase (no className to keep in
      // sync by hand, and hand-syncing it had already drifted: it carried
      // origin-top-center, which the shipped rule does not).
      vp.setAttribute("data-slot", "navigation-menu-viewport")
      vp.id = (openId || idOf(triggers()[0]) || "nav") + "-viewport"
      vp.setAttribute("data-orientation", "horizontal")
      vp.setAttribute("data-state", "closed")
      root.appendChild(vp)
      return vp
    }

    function close(silent) {
      if (!openId) return
      var trig = root.querySelector('[data-radixuigo-nav-trigger="' + openId + '"]')
      var content = viewport() && viewport().querySelector('[data-slot=navigation-menu-content]')
      if (trig) {
        trig.setAttribute("data-state", "closed")
        trig.setAttribute("aria-expanded", "false")
        trig.removeAttribute("aria-controls")
      }
      if (content) content.setAttribute("data-state", "closed")
      var vp = viewport()
      if (vp) {
        vp.setAttribute("data-state", "closed")
        clearTimeout(exitTimer)
        exitTimer = setTimeout(function () { if (vp.parentNode && vp.getAttribute("data-state") === "closed") vp.remove() }, EXIT)
      }
      openId = null
      if (trig) shadless.h.emit(trig, "close", "navigation-menu")
    }

    function open(id, trig) {
      clearTimeout(exitTimer)
      if (openId && openId !== id) close(true)
      var tpl = document.getElementById(id + "-content-tpl")
      if (!tpl) return
      var vp = ensureViewport()
      vp.setAttribute("data-state", "open")
      var content = tpl.content.firstElementChild.cloneNode(true)
      vp.replaceChildren(content)
      content.setAttribute("data-state", "open")
      trig.setAttribute("data-state", "open")
      trig.setAttribute("aria-expanded", "true")
      trig.setAttribute("aria-controls", content.id)
      var w = content.offsetWidth, h = content.offsetHeight
      vp.style.setProperty("--radix-navigation-menu-viewport-width", w + "px")
      vp.style.setProperty("--radix-navigation-menu-viewport-height", h + "px")
      openId = id
      shadless.h.emit(trig, "open", "navigation-menu")
    }

    triggers().forEach(function (t) {
      var id = idOf(t)
      shadless.instances.set(t, { component: "navigation-menu",
        open: function () { if (openId !== id) open(id, t) },
        close: function () { if (openId === id) close() },
        toggle: function () { openId === id ? close() : open(id, t) },
        isOpen: function () { return openId === id },
      })
    })
      return { root: root, triggers: triggers,
        openId: function () { return openId }, open: open, close: close };
    }

    Array.prototype.forEach.call(document.querySelectorAll("[data-slot=navigation-menu]"), function (r) {
      navs.push(wire(r));
    });

    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-nav-trigger]")
      if (trig) {
        var n = navOf(trig)
        if (!n) return
        e.preventDefault()
        var id = idOf(trig)
        if (id === n.openId()) n.close()
        else n.open(id, trig)
        return
      }
      var link = e.target.closest && e.target.closest("[data-slot=navigation-menu-link]")
      if (link) { var ln = navOf(link); if (ln && ln.openId()) ln.close(); return }
      navs.forEach(function (n) { if (n.openId() && !n.root.contains(e.target)) n.close() })
    })

    document.addEventListener("keydown", function (e) {
      var key = e.key
      if (key === "Escape") {
        var opened = navs.filter(function (n) { return n.openId() })
        if (opened.length) { e.preventDefault(); opened.forEach(function (n) { n.close() }) }
        return
      }
      if (key !== "ArrowRight" && key !== "ArrowLeft" && key !== "Home" && key !== "End") return
      var trig = e.target.closest && e.target.closest("[data-radixuigo-nav-trigger]")
      if (!trig) return
      var kn = navOf(trig)
      if (!kn) return
      e.preventDefault()
      var ts = kn.triggers()
      // nextIndex also skips disabled triggers and swaps the horizontal
      // arrows under dir=rtl — the shipped oracle-navigation-menu-rtl
      // fixture exercises exactly this, which the old hand-rolled formula
      // never did
      var n = shadless.h.nextIndex(e, ts)
      if (n < 0) return
      ts[n].focus()
    })
  } })
})()
```
:::

::::

## Installation

**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/navigation-menu.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/navigation-menu.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/navigation-menu.js` | this component's behavior — registers with the base |

**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/navigation-menu.js"></script>
```

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="navigation-menu-trigger" id="<k>-trigger" data-radixuigo-nav-trigger="<k>">` | opens on click |
| `<template id="<k>-content-tpl">` | holds the `data-slot="navigation-menu-content"` subtree; the glue creates the shared viewport inside the root |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
navigation-menu
├── navigation-menu-list
│   ├── navigation-menu-item
│   │   ├── navigation-menu-trigger
│   │   └── navigation-menu-content
│   │       ├── navigation-menu-link
│   │       └── navigation-menu-link
│   └── navigation-menu-item
│       └── navigation-menu-link
└── navigation-menu-indicator
```

## Link Component

There's no `asChild`/framework-Link composition in shadless — `navigation-menu-link` is a plain `<a>`, so just point `href` at whatever route your app uses:

```html showLineNumbers
<li data-slot="navigation-menu-item">
  <a data-slot="navigation-menu-link" href="/docs">Documentation</a>
</li>
```

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<div class="demo-missing" data-demo="navigation-menu-rtl" data-status="to-author">demo not yet available — <code>navigation-menu-rtl</code></div>

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="navigation-menu"` |
| `data-slot="navigation-menu-list"` |
| `data-slot="navigation-menu-item"` |
| `data-slot="navigation-menu-trigger"` |
| `data-slot="navigation-menu-content"` |
| `data-slot="navigation-menu-viewport"` |
| `data-slot="navigation-menu-link"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Navigation Menu](https://www.radix-ui.com/primitives/docs/components/navigation-menu#api-reference) documentation for more information.
