---
title: "Tabs"
description: "A set of layered sections of content—known as tab panels—that are displayed one at a time."
---

# Tabs

A set of layered sections of content—known as tab panels—that are displayed one at a time.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/tabs" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/tabs#api-reference" rel="noopener">api</a></p>

::::demo tabs-demo
<iframe class="demo" src="/demos/tabs.html" title="tabs-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tabs.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tabs.html]
<div>
  <div
    dir="ltr"
    data-orientation="horizontal"
    data-slot="tabs"
    class="gap-2 group/tabs flex data-horizontal:flex-col"
  >
    <div
      role="tablist"
      aria-orientation="horizontal"
      data-slot="tabs-list"
      data-variant="default"
      class="rounded-lg p-[3px] group-data-horizontal/tabs:h-8 data-[variant=line]:rounded-none group/tabs-list inline-flex w-fit items-center justify-center text-muted-foreground group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col bg-muted"
      tabindex="0"
      data-orientation="horizontal"
      style="outline: none"
    >
      <button
        type="button"
        role="tab"
        aria-selected="true"
        aria-controls="panel0-0"
        data-state="active"
        id="t1"
        data-slot="tabs-trigger"
        class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
        tabindex="-1"
        data-orientation="horizontal"
        data-radix-collection-item=""
      >
        Account</button
      ><button
        type="button"
        role="tab"
        aria-selected="false"
        aria-controls="panel0-1"
        data-state="inactive"
        id="t2"
        data-slot="tabs-trigger"
        class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
        tabindex="-1"
        data-orientation="horizontal"
        data-radix-collection-item=""
      >
        Password
      </button>
    </div>
    <div
      data-state="active"
      data-orientation="horizontal"
      role="tabpanel"
      aria-labelledby="t1"
      id="panel0-0"
      tabindex="0"
      data-slot="tabs-content"
      class="text-sm flex-1 outline-none"
      style="animation-duration: 0s"
    >
      Make changes to your account here.
    </div>
    <div
      data-state="inactive"
      data-orientation="horizontal"
      role="tabpanel"
      aria-labelledby="t2"
      id="panel0-1"
      tabindex="0"
      data-slot="tabs-content"
      class="text-sm flex-1 outline-none"
      style="animation-duration: 0s"
      hidden=""
    >
      Make changes to your account here.
    </div>
  </div>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tabs.js
// shadless tabs behavior (wireTabs) — registers with the base; multi-instance: every [data-slot=tabs] root.
// kernel handles click/arrow activation (aria-selected, data-state, hidden,
// roving tabindex on interaction); behavior wires the static a11y surface:
// roving tabindex initial value (kernel only sets it on interactive activation)
(function () {
  shadless.register("tabs", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=tabs]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var list = root.querySelector("[data-slot=tabs-list]");
      if (!list) return;
      var triggers = Array.prototype.slice.call(list.querySelectorAll("[data-slot=tabs-trigger]"));
      var panels = triggers.map(function (t) {
        var id = t.getAttribute("aria-controls");
        return (id && document.getElementById(id)) || null;
      });
      if (!triggers.length) return;
      var initial = Math.max(0, triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active"; }));
      // roving step: skips disabled triggers (radix), horizontal arrows swap under dir=rtl
      function step(idx, key) {
        var last = triggers.length - 1;
        if (key === "Home" || key === "End") {
          var i = key === "Home" ? 0 : last, d = key === "Home" ? 1 : -1;
          for (var n = 0; n <= last; n++, i += d) if (!shadless.h.isDisabled(triggers[i])) return i;
          return -1;
        }
        var fwd = key === "ArrowRight" || key === "ArrowDown";
        if (shadless.h.isRtl(list) && (key === "ArrowRight" || key === "ArrowLeft")) fwd = !fwd;
        for (var k = 1; k <= last; k++) {
          var j = (idx + (fwd ? k : -k) + triggers.length) % triggers.length;
          if (!shadless.h.isDisabled(triggers[j])) return j;
        }
        return -1;
      }
      // panel-less tab lists (tabs-icons, tabs-line, …: triggers only): the
      // kernel's wireTabs needs panels, so activation is wired here — click
      // and arrow keys flip data-state / aria-selected / roving tabindex
      if (panels.some(function (p) { return !p; })) {
        var current = -1;
        var activate = function (i) {
          triggers.forEach(function (t, k) {
            var on = k === i;
            t.setAttribute("data-state", on ? "active" : "inactive");
            t.setAttribute("aria-selected", on ? "true" : "false");
            t.setAttribute("tabindex", on ? "0" : "-1");
          });
          var changed = current !== -1 && current !== i;
          current = i;
          if (changed) shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        };
        triggers.forEach(function (t, i) {
          t.addEventListener("click", function () { if (!t.disabled) activate(i); }, { signal: w.signal });
        });
        list.addEventListener("keydown", function (e) {
          var idx = triggers.indexOf(document.activeElement);
          if (idx === -1) return;
          if (["ArrowRight", "ArrowDown", "ArrowLeft", "ArrowUp", "Home", "End"].indexOf(e.key) < 0) return;
          var next = step(idx, e.key);
          if (next === -1) return;
          e.preventDefault();
          activate(next); triggers[next].focus();
        }, { signal: w.signal });
        activate(initial);
        shadless.instances.set(root, { component: "tabs",
          activate: function (i) { activate(i) },
          active: function () { return triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active" }) },
        })
        return;
      }
      w.persistent = true // kernel wireTabs holds list/trigger listeners with no unwire
      triggers.forEach(function (t) { t.setAttribute("tabindex", "-1"); }); // radix roving: all -1 until focus enters
      // kernel calls onChange on wire (its initial activate) and on every
      // activate, same index included; radix onValueChange fires only when
      // the value CHANGES — emit on real transitions after wiring
      var last = initial, ready = false;
      var wired = RadixKernel.wireTabs({ list: list, triggers: triggers, panels: panels, initial: initial,
        onChange: function (i) {
          if (!ready || i === last) { last = i; return; }
          last = i;
          shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        } });
      ready = true;
      shadless.instances.set(root, { component: "tabs",
        activate: function (i, focus) { wired.activate(i, !!focus) },
        active: function () { return wired.active() },
      })
      // kernel computes ArrowLeft/ArrowRight from the ACTIVE index; radix moves
      // relative to the FOCUSED trigger (focus alone doesn't activate) — absorb in
      // glue with a capture-phase handler; kernel keeps click/Home/End.
      list.addEventListener("keydown", function (e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var idx = triggers.indexOf(document.activeElement);
        if (idx === -1) return;
        e.preventDefault(); e.stopPropagation();
        var next = step(idx, e.key);
        if (next === -1) return;
        wired.activate(next, true);
        triggers[next].focus();
      }, true);
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
@import "shadless/tabs.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/tabs.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/tabs.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/tabs.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/tabs.js"></script>
```

**Copy the markup from                             into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<div data-slot="tabs">` with `data-slot="tabs-trigger" aria-controls="<panel-id>"` and `data-slot="tabs-content" id="<panel-id>"` | no template: every panel is in the markup, inactive ones `hidden`; the glue wires every root it finds |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get(rootEl)` → `activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`). `shadless.get` accepts an element or a selector and walks up from any element inside the instance.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/tabs.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
tabs
├── tabs-list
│   ├── tabs-trigger
│   └── tabs-trigger
├── tabs-content
└── tabs-content
```

## Line

Use the `variant="line"` prop on `TabsList` for a line style.

::::demo tabs-line
<iframe class="demo" src="/demos/tabs-line.html" title="tabs-line" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tabs-line.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tabs-line.html]
<div
  dir="ltr"
  data-orientation="horizontal"
  data-slot="tabs"
  class="gap-2 group/tabs flex data-horizontal:flex-col"
>
  <div
    role="tablist"
    aria-orientation="horizontal"
    data-slot="tabs-list"
    data-variant="line"
    class="rounded-lg p-[3px] group-data-horizontal/tabs:h-8 data-[variant=line]:rounded-none group/tabs-list inline-flex w-fit items-center justify-center text-muted-foreground group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col gap-1 bg-transparent"
    tabindex="0"
    data-orientation="horizontal"
    style="outline: none"
  >
    <button
      type="button"
      role="tab"
      aria-selected="true"
      aria-controls="panel0-0"
      data-state="active"
      id="tab0-0"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      Overview</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-1"
      data-state="inactive"
      id="tab0-1"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      Analytics</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-2"
      data-state="inactive"
      id="tab0-2"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      Reports
    </button>
  </div>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tabs.js
// shadless tabs behavior (wireTabs) — registers with the base; multi-instance: every [data-slot=tabs] root.
// kernel handles click/arrow activation (aria-selected, data-state, hidden,
// roving tabindex on interaction); behavior wires the static a11y surface:
// roving tabindex initial value (kernel only sets it on interactive activation)
(function () {
  shadless.register("tabs", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=tabs]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var list = root.querySelector("[data-slot=tabs-list]");
      if (!list) return;
      var triggers = Array.prototype.slice.call(list.querySelectorAll("[data-slot=tabs-trigger]"));
      var panels = triggers.map(function (t) {
        var id = t.getAttribute("aria-controls");
        return (id && document.getElementById(id)) || null;
      });
      if (!triggers.length) return;
      var initial = Math.max(0, triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active"; }));
      // roving step: skips disabled triggers (radix), horizontal arrows swap under dir=rtl
      function step(idx, key) {
        var last = triggers.length - 1;
        if (key === "Home" || key === "End") {
          var i = key === "Home" ? 0 : last, d = key === "Home" ? 1 : -1;
          for (var n = 0; n <= last; n++, i += d) if (!shadless.h.isDisabled(triggers[i])) return i;
          return -1;
        }
        var fwd = key === "ArrowRight" || key === "ArrowDown";
        if (shadless.h.isRtl(list) && (key === "ArrowRight" || key === "ArrowLeft")) fwd = !fwd;
        for (var k = 1; k <= last; k++) {
          var j = (idx + (fwd ? k : -k) + triggers.length) % triggers.length;
          if (!shadless.h.isDisabled(triggers[j])) return j;
        }
        return -1;
      }
      // panel-less tab lists (tabs-icons, tabs-line, …: triggers only): the
      // kernel's wireTabs needs panels, so activation is wired here — click
      // and arrow keys flip data-state / aria-selected / roving tabindex
      if (panels.some(function (p) { return !p; })) {
        var current = -1;
        var activate = function (i) {
          triggers.forEach(function (t, k) {
            var on = k === i;
            t.setAttribute("data-state", on ? "active" : "inactive");
            t.setAttribute("aria-selected", on ? "true" : "false");
            t.setAttribute("tabindex", on ? "0" : "-1");
          });
          var changed = current !== -1 && current !== i;
          current = i;
          if (changed) shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        };
        triggers.forEach(function (t, i) {
          t.addEventListener("click", function () { if (!t.disabled) activate(i); }, { signal: w.signal });
        });
        list.addEventListener("keydown", function (e) {
          var idx = triggers.indexOf(document.activeElement);
          if (idx === -1) return;
          if (["ArrowRight", "ArrowDown", "ArrowLeft", "ArrowUp", "Home", "End"].indexOf(e.key) < 0) return;
          var next = step(idx, e.key);
          if (next === -1) return;
          e.preventDefault();
          activate(next); triggers[next].focus();
        }, { signal: w.signal });
        activate(initial);
        shadless.instances.set(root, { component: "tabs",
          activate: function (i) { activate(i) },
          active: function () { return triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active" }) },
        })
        return;
      }
      w.persistent = true // kernel wireTabs holds list/trigger listeners with no unwire
      triggers.forEach(function (t) { t.setAttribute("tabindex", "-1"); }); // radix roving: all -1 until focus enters
      // kernel calls onChange on wire (its initial activate) and on every
      // activate, same index included; radix onValueChange fires only when
      // the value CHANGES — emit on real transitions after wiring
      var last = initial, ready = false;
      var wired = RadixKernel.wireTabs({ list: list, triggers: triggers, panels: panels, initial: initial,
        onChange: function (i) {
          if (!ready || i === last) { last = i; return; }
          last = i;
          shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        } });
      ready = true;
      shadless.instances.set(root, { component: "tabs",
        activate: function (i, focus) { wired.activate(i, !!focus) },
        active: function () { return wired.active() },
      })
      // kernel computes ArrowLeft/ArrowRight from the ACTIVE index; radix moves
      // relative to the FOCUSED trigger (focus alone doesn't activate) — absorb in
      // glue with a capture-phase handler; kernel keeps click/Home/End.
      list.addEventListener("keydown", function (e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var idx = triggers.indexOf(document.activeElement);
        if (idx === -1) return;
        e.preventDefault(); e.stopPropagation();
        var next = step(idx, e.key);
        if (next === -1) return;
        wired.activate(next, true);
        triggers[next].focus();
      }, true);
    });
  } })
})()
```
:::

::::


## Vertical

Use `orientation="vertical"` for vertical tabs.

::::demo tabs-vertical
<iframe class="demo" src="/demos/tabs-vertical.html" title="tabs-vertical" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tabs-vertical.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tabs-vertical.html]
<div
  dir="ltr"
  data-orientation="vertical"
  data-slot="tabs"
  class="gap-2 group/tabs flex data-horizontal:flex-col"
>
  <div
    role="tablist"
    aria-orientation="horizontal"
    data-slot="tabs-list"
    data-variant="default"
    class="rounded-lg p-[3px] group-data-horizontal/tabs:h-8 data-[variant=line]:rounded-none group/tabs-list inline-flex w-fit items-center justify-center text-muted-foreground group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col bg-muted"
    tabindex="0"
    data-orientation="horizontal"
    style="outline: none"
  >
    <button
      type="button"
      role="tab"
      aria-selected="true"
      aria-controls="panel0-0"
      data-state="active"
      id="tab0-0"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      Account</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-1"
      data-state="inactive"
      id="tab0-1"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      Password</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-2"
      data-state="inactive"
      id="tab0-2"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      Notifications
    </button>
  </div>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tabs.js
// shadless tabs behavior (wireTabs) — registers with the base; multi-instance: every [data-slot=tabs] root.
// kernel handles click/arrow activation (aria-selected, data-state, hidden,
// roving tabindex on interaction); behavior wires the static a11y surface:
// roving tabindex initial value (kernel only sets it on interactive activation)
(function () {
  shadless.register("tabs", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=tabs]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var list = root.querySelector("[data-slot=tabs-list]");
      if (!list) return;
      var triggers = Array.prototype.slice.call(list.querySelectorAll("[data-slot=tabs-trigger]"));
      var panels = triggers.map(function (t) {
        var id = t.getAttribute("aria-controls");
        return (id && document.getElementById(id)) || null;
      });
      if (!triggers.length) return;
      var initial = Math.max(0, triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active"; }));
      // roving step: skips disabled triggers (radix), horizontal arrows swap under dir=rtl
      function step(idx, key) {
        var last = triggers.length - 1;
        if (key === "Home" || key === "End") {
          var i = key === "Home" ? 0 : last, d = key === "Home" ? 1 : -1;
          for (var n = 0; n <= last; n++, i += d) if (!shadless.h.isDisabled(triggers[i])) return i;
          return -1;
        }
        var fwd = key === "ArrowRight" || key === "ArrowDown";
        if (shadless.h.isRtl(list) && (key === "ArrowRight" || key === "ArrowLeft")) fwd = !fwd;
        for (var k = 1; k <= last; k++) {
          var j = (idx + (fwd ? k : -k) + triggers.length) % triggers.length;
          if (!shadless.h.isDisabled(triggers[j])) return j;
        }
        return -1;
      }
      // panel-less tab lists (tabs-icons, tabs-line, …: triggers only): the
      // kernel's wireTabs needs panels, so activation is wired here — click
      // and arrow keys flip data-state / aria-selected / roving tabindex
      if (panels.some(function (p) { return !p; })) {
        var current = -1;
        var activate = function (i) {
          triggers.forEach(function (t, k) {
            var on = k === i;
            t.setAttribute("data-state", on ? "active" : "inactive");
            t.setAttribute("aria-selected", on ? "true" : "false");
            t.setAttribute("tabindex", on ? "0" : "-1");
          });
          var changed = current !== -1 && current !== i;
          current = i;
          if (changed) shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        };
        triggers.forEach(function (t, i) {
          t.addEventListener("click", function () { if (!t.disabled) activate(i); }, { signal: w.signal });
        });
        list.addEventListener("keydown", function (e) {
          var idx = triggers.indexOf(document.activeElement);
          if (idx === -1) return;
          if (["ArrowRight", "ArrowDown", "ArrowLeft", "ArrowUp", "Home", "End"].indexOf(e.key) < 0) return;
          var next = step(idx, e.key);
          if (next === -1) return;
          e.preventDefault();
          activate(next); triggers[next].focus();
        }, { signal: w.signal });
        activate(initial);
        shadless.instances.set(root, { component: "tabs",
          activate: function (i) { activate(i) },
          active: function () { return triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active" }) },
        })
        return;
      }
      w.persistent = true // kernel wireTabs holds list/trigger listeners with no unwire
      triggers.forEach(function (t) { t.setAttribute("tabindex", "-1"); }); // radix roving: all -1 until focus enters
      // kernel calls onChange on wire (its initial activate) and on every
      // activate, same index included; radix onValueChange fires only when
      // the value CHANGES — emit on real transitions after wiring
      var last = initial, ready = false;
      var wired = RadixKernel.wireTabs({ list: list, triggers: triggers, panels: panels, initial: initial,
        onChange: function (i) {
          if (!ready || i === last) { last = i; return; }
          last = i;
          shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        } });
      ready = true;
      shadless.instances.set(root, { component: "tabs",
        activate: function (i, focus) { wired.activate(i, !!focus) },
        active: function () { return wired.active() },
      })
      // kernel computes ArrowLeft/ArrowRight from the ACTIVE index; radix moves
      // relative to the FOCUSED trigger (focus alone doesn't activate) — absorb in
      // glue with a capture-phase handler; kernel keeps click/Home/End.
      list.addEventListener("keydown", function (e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var idx = triggers.indexOf(document.activeElement);
        if (idx === -1) return;
        e.preventDefault(); e.stopPropagation();
        var next = step(idx, e.key);
        if (next === -1) return;
        wired.activate(next, true);
        triggers[next].focus();
      }, true);
    });
  } })
})()
```
:::

::::


## Disabled

::::demo tabs-disabled
<iframe class="demo" src="/demos/tabs-disabled.html" title="tabs-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tabs-disabled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tabs-disabled.html]
<div
  dir="ltr"
  data-orientation="horizontal"
  data-slot="tabs"
  class="gap-2 group/tabs flex data-horizontal:flex-col"
>
  <div
    role="tablist"
    aria-orientation="horizontal"
    data-slot="tabs-list"
    data-variant="default"
    class="rounded-lg p-[3px] group-data-horizontal/tabs:h-8 data-[variant=line]:rounded-none group/tabs-list inline-flex w-fit items-center justify-center text-muted-foreground group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col bg-muted"
    tabindex="0"
    data-orientation="horizontal"
    style="outline: none"
  >
    <button
      type="button"
      role="tab"
      aria-selected="true"
      aria-controls="panel0-0"
      data-state="active"
      id="tab0-0"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      Home</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-1"
      data-state="inactive"
      data-disabled=""
      disabled=""
      id="tab0-1"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      Disabled
    </button>
  </div>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tabs.js
// shadless tabs behavior (wireTabs) — registers with the base; multi-instance: every [data-slot=tabs] root.
// kernel handles click/arrow activation (aria-selected, data-state, hidden,
// roving tabindex on interaction); behavior wires the static a11y surface:
// roving tabindex initial value (kernel only sets it on interactive activation)
(function () {
  shadless.register("tabs", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=tabs]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var list = root.querySelector("[data-slot=tabs-list]");
      if (!list) return;
      var triggers = Array.prototype.slice.call(list.querySelectorAll("[data-slot=tabs-trigger]"));
      var panels = triggers.map(function (t) {
        var id = t.getAttribute("aria-controls");
        return (id && document.getElementById(id)) || null;
      });
      if (!triggers.length) return;
      var initial = Math.max(0, triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active"; }));
      // roving step: skips disabled triggers (radix), horizontal arrows swap under dir=rtl
      function step(idx, key) {
        var last = triggers.length - 1;
        if (key === "Home" || key === "End") {
          var i = key === "Home" ? 0 : last, d = key === "Home" ? 1 : -1;
          for (var n = 0; n <= last; n++, i += d) if (!shadless.h.isDisabled(triggers[i])) return i;
          return -1;
        }
        var fwd = key === "ArrowRight" || key === "ArrowDown";
        if (shadless.h.isRtl(list) && (key === "ArrowRight" || key === "ArrowLeft")) fwd = !fwd;
        for (var k = 1; k <= last; k++) {
          var j = (idx + (fwd ? k : -k) + triggers.length) % triggers.length;
          if (!shadless.h.isDisabled(triggers[j])) return j;
        }
        return -1;
      }
      // panel-less tab lists (tabs-icons, tabs-line, …: triggers only): the
      // kernel's wireTabs needs panels, so activation is wired here — click
      // and arrow keys flip data-state / aria-selected / roving tabindex
      if (panels.some(function (p) { return !p; })) {
        var current = -1;
        var activate = function (i) {
          triggers.forEach(function (t, k) {
            var on = k === i;
            t.setAttribute("data-state", on ? "active" : "inactive");
            t.setAttribute("aria-selected", on ? "true" : "false");
            t.setAttribute("tabindex", on ? "0" : "-1");
          });
          var changed = current !== -1 && current !== i;
          current = i;
          if (changed) shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        };
        triggers.forEach(function (t, i) {
          t.addEventListener("click", function () { if (!t.disabled) activate(i); }, { signal: w.signal });
        });
        list.addEventListener("keydown", function (e) {
          var idx = triggers.indexOf(document.activeElement);
          if (idx === -1) return;
          if (["ArrowRight", "ArrowDown", "ArrowLeft", "ArrowUp", "Home", "End"].indexOf(e.key) < 0) return;
          var next = step(idx, e.key);
          if (next === -1) return;
          e.preventDefault();
          activate(next); triggers[next].focus();
        }, { signal: w.signal });
        activate(initial);
        shadless.instances.set(root, { component: "tabs",
          activate: function (i) { activate(i) },
          active: function () { return triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active" }) },
        })
        return;
      }
      w.persistent = true // kernel wireTabs holds list/trigger listeners with no unwire
      triggers.forEach(function (t) { t.setAttribute("tabindex", "-1"); }); // radix roving: all -1 until focus enters
      // kernel calls onChange on wire (its initial activate) and on every
      // activate, same index included; radix onValueChange fires only when
      // the value CHANGES — emit on real transitions after wiring
      var last = initial, ready = false;
      var wired = RadixKernel.wireTabs({ list: list, triggers: triggers, panels: panels, initial: initial,
        onChange: function (i) {
          if (!ready || i === last) { last = i; return; }
          last = i;
          shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        } });
      ready = true;
      shadless.instances.set(root, { component: "tabs",
        activate: function (i, focus) { wired.activate(i, !!focus) },
        active: function () { return wired.active() },
      })
      // kernel computes ArrowLeft/ArrowRight from the ACTIVE index; radix moves
      // relative to the FOCUSED trigger (focus alone doesn't activate) — absorb in
      // glue with a capture-phase handler; kernel keeps click/Home/End.
      list.addEventListener("keydown", function (e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var idx = triggers.indexOf(document.activeElement);
        if (idx === -1) return;
        e.preventDefault(); e.stopPropagation();
        var next = step(idx, e.key);
        if (next === -1) return;
        wired.activate(next, true);
        triggers[next].focus();
      }, true);
    });
  } })
})()
```
:::

::::


## Icons

::::demo tabs-icons
<iframe class="demo" src="/demos/tabs-icons.html" title="tabs-icons" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tabs-icons.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [tabs-icons.html]
<div
  dir="ltr"
  data-orientation="horizontal"
  data-slot="tabs"
  class="gap-2 group/tabs flex data-horizontal:flex-col"
>
  <div
    role="tablist"
    aria-orientation="horizontal"
    data-slot="tabs-list"
    data-variant="default"
    class="rounded-lg p-[3px] group-data-horizontal/tabs:h-8 data-[variant=line]:rounded-none group/tabs-list inline-flex w-fit items-center justify-center text-muted-foreground group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col bg-muted"
    tabindex="0"
    data-orientation="horizontal"
    style="outline: none"
  >
    <button
      type="button"
      role="tab"
      aria-selected="true"
      aria-controls="panel0-0"
      data-state="active"
      id="tab0-0"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
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
        class="lucide lucide-app-window"
      >
        <rect x="2" y="4" width="20" height="16" rx="2"></rect>
        <path d="M10 4v4"></path>
        <path d="M2 8h20"></path>
        <path d="M6 4v4"></path></svg
      >Preview</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-1"
      data-state="inactive"
      id="tab0-1"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
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
        class="lucide lucide-code"
      >
        <polyline points="16 18 22 12 16 6"></polyline>
        <polyline points="8 6 2 12 8 18"></polyline></svg
      >Code
    </button>
  </div>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tabs.js
// shadless tabs behavior (wireTabs) — registers with the base; multi-instance: every [data-slot=tabs] root.
// kernel handles click/arrow activation (aria-selected, data-state, hidden,
// roving tabindex on interaction); behavior wires the static a11y surface:
// roving tabindex initial value (kernel only sets it on interactive activation)
(function () {
  shadless.register("tabs", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=tabs]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var list = root.querySelector("[data-slot=tabs-list]");
      if (!list) return;
      var triggers = Array.prototype.slice.call(list.querySelectorAll("[data-slot=tabs-trigger]"));
      var panels = triggers.map(function (t) {
        var id = t.getAttribute("aria-controls");
        return (id && document.getElementById(id)) || null;
      });
      if (!triggers.length) return;
      var initial = Math.max(0, triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active"; }));
      // roving step: skips disabled triggers (radix), horizontal arrows swap under dir=rtl
      function step(idx, key) {
        var last = triggers.length - 1;
        if (key === "Home" || key === "End") {
          var i = key === "Home" ? 0 : last, d = key === "Home" ? 1 : -1;
          for (var n = 0; n <= last; n++, i += d) if (!shadless.h.isDisabled(triggers[i])) return i;
          return -1;
        }
        var fwd = key === "ArrowRight" || key === "ArrowDown";
        if (shadless.h.isRtl(list) && (key === "ArrowRight" || key === "ArrowLeft")) fwd = !fwd;
        for (var k = 1; k <= last; k++) {
          var j = (idx + (fwd ? k : -k) + triggers.length) % triggers.length;
          if (!shadless.h.isDisabled(triggers[j])) return j;
        }
        return -1;
      }
      // panel-less tab lists (tabs-icons, tabs-line, …: triggers only): the
      // kernel's wireTabs needs panels, so activation is wired here — click
      // and arrow keys flip data-state / aria-selected / roving tabindex
      if (panels.some(function (p) { return !p; })) {
        var current = -1;
        var activate = function (i) {
          triggers.forEach(function (t, k) {
            var on = k === i;
            t.setAttribute("data-state", on ? "active" : "inactive");
            t.setAttribute("aria-selected", on ? "true" : "false");
            t.setAttribute("tabindex", on ? "0" : "-1");
          });
          var changed = current !== -1 && current !== i;
          current = i;
          if (changed) shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        };
        triggers.forEach(function (t, i) {
          t.addEventListener("click", function () { if (!t.disabled) activate(i); }, { signal: w.signal });
        });
        list.addEventListener("keydown", function (e) {
          var idx = triggers.indexOf(document.activeElement);
          if (idx === -1) return;
          if (["ArrowRight", "ArrowDown", "ArrowLeft", "ArrowUp", "Home", "End"].indexOf(e.key) < 0) return;
          var next = step(idx, e.key);
          if (next === -1) return;
          e.preventDefault();
          activate(next); triggers[next].focus();
        }, { signal: w.signal });
        activate(initial);
        shadless.instances.set(root, { component: "tabs",
          activate: function (i) { activate(i) },
          active: function () { return triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active" }) },
        })
        return;
      }
      w.persistent = true // kernel wireTabs holds list/trigger listeners with no unwire
      triggers.forEach(function (t) { t.setAttribute("tabindex", "-1"); }); // radix roving: all -1 until focus enters
      // kernel calls onChange on wire (its initial activate) and on every
      // activate, same index included; radix onValueChange fires only when
      // the value CHANGES — emit on real transitions after wiring
      var last = initial, ready = false;
      var wired = RadixKernel.wireTabs({ list: list, triggers: triggers, panels: panels, initial: initial,
        onChange: function (i) {
          if (!ready || i === last) { last = i; return; }
          last = i;
          shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        } });
      ready = true;
      shadless.instances.set(root, { component: "tabs",
        activate: function (i, focus) { wired.activate(i, !!focus) },
        active: function () { return wired.active() },
      })
      // kernel computes ArrowLeft/ArrowRight from the ACTIVE index; radix moves
      // relative to the FOCUSED trigger (focus alone doesn't activate) — absorb in
      // glue with a capture-phase handler; kernel keeps click/Home/End.
      list.addEventListener("keydown", function (e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var idx = triggers.indexOf(document.activeElement);
        if (idx === -1) return;
        e.preventDefault(); e.stopPropagation();
        var next = step(idx, e.key);
        if (next === -1) return;
        wired.activate(next, true);
        triggers[next].focus();
      }, true);
    });
  } })
})()
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo tabs-rtl
<iframe class="demo" src="/demos/tabs-rtl.html" title="tabs-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/tabs-rtl.html">Open the demo page</a> · <a href="/demos/tabs-rtl-he.html">HE</a> · <a href="/demos/tabs-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [tabs-rtl.html]
<div
  dir="rtl"
  data-orientation="horizontal"
  data-slot="tabs"
  class="gap-2 group/tabs flex data-horizontal:flex-col w-full max-w-sm"
>
  <div
    role="tablist"
    aria-orientation="horizontal"
    data-slot="tabs-list"
    data-variant="default"
    class="rounded-lg p-[3px] group-data-horizontal/tabs:h-8 data-[variant=line]:rounded-none group/tabs-list inline-flex w-fit items-center justify-center text-muted-foreground group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col bg-muted"
    dir="rtl"
    tabindex="0"
    data-orientation="horizontal"
    style="outline: none"
  >
    <button
      type="button"
      role="tab"
      aria-selected="true"
      aria-controls="panel0-0"
      data-state="active"
      id="tab0-0"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pe-1 has-data-[icon=inline-start]:ps-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      نظرة عامة</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-1"
      data-state="inactive"
      id="tab0-1"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pe-1 has-data-[icon=inline-start]:ps-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      التحليلات</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-2"
      data-state="inactive"
      id="tab0-2"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pe-1 has-data-[icon=inline-start]:ps-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      التقارير</button
    ><button
      type="button"
      role="tab"
      aria-selected="false"
      aria-controls="panel0-3"
      data-state="inactive"
      id="tab0-3"
      data-slot="tabs-trigger"
      class="gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&amp;_svg:not([class*='size-'])]:size-4 has-data-[icon=inline-end]:pe-1 has-data-[icon=inline-start]:ps-1 relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 dark:text-muted-foreground dark:hover:text-foreground [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100"
      tabindex="-1"
      data-orientation="horizontal"
      data-radix-collection-item=""
    >
      الإعدادات
    </button>
  </div>
  <div
    data-state="active"
    data-orientation="horizontal"
    role="tabpanel"
    aria-labelledby="tab0-0"
    id="panel0-0"
    tabindex="0"
    data-slot="tabs-content"
    class="text-sm flex-1 outline-none"
    style="animation-duration: 0s"
  >
    <div
      data-slot="card"
      data-size="default"
      class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col"
      dir="rtl"
    >
      <div
        data-slot="card-header"
        class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"
      >
        <div
          data-slot="card-title"
          class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading"
        >
          نظرة عامة
        </div>
        <div data-slot="card-description" class="text-muted-foreground text-sm">
          عرض مقاييسك الرئيسية وأنشطة المشروع الأخيرة. تتبع التقدم عبر جميع مشاريعك النشطة.
        </div>
      </div>
      <div data-slot="card-content" class="px-(--card-spacing) text-sm text-muted-foreground">
        لديك ١٢ مشروعًا نشطًا و٣ مهام معلقة.
      </div>
    </div>
  </div>
  <div
    data-state="inactive"
    data-orientation="horizontal"
    role="tabpanel"
    aria-labelledby="tab0-1"
    id="panel0-1"
    tabindex="0"
    data-slot="tabs-content"
    class="text-sm flex-1 outline-none"
    style="animation-duration: 0s"
    hidden=""
  >
    <div
      data-slot="card"
      data-size="default"
      class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col"
      dir="rtl"
    >
      <div
        data-slot="card-header"
        class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"
      >
        <div
          data-slot="card-title"
          class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading"
        >
          نظرة عامة
        </div>
        <div data-slot="card-description" class="text-muted-foreground text-sm">
          عرض مقاييسك الرئيسية وأنشطة المشروع الأخيرة. تتبع التقدم عبر جميع مشاريعك النشطة.
        </div>
      </div>
      <div data-slot="card-content" class="px-(--card-spacing) text-sm text-muted-foreground">
        لديك ١٢ مشروعًا نشطًا و٣ مهام معلقة.
      </div>
    </div>
  </div>
  <div
    data-state="inactive"
    data-orientation="horizontal"
    role="tabpanel"
    aria-labelledby="tab0-2"
    id="panel0-2"
    tabindex="0"
    data-slot="tabs-content"
    class="text-sm flex-1 outline-none"
    style="animation-duration: 0s"
    hidden=""
  >
    <div
      data-slot="card"
      data-size="default"
      class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col"
      dir="rtl"
    >
      <div
        data-slot="card-header"
        class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"
      >
        <div
          data-slot="card-title"
          class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading"
        >
          نظرة عامة
        </div>
        <div data-slot="card-description" class="text-muted-foreground text-sm">
          عرض مقاييسك الرئيسية وأنشطة المشروع الأخيرة. تتبع التقدم عبر جميع مشاريعك النشطة.
        </div>
      </div>
      <div data-slot="card-content" class="px-(--card-spacing) text-sm text-muted-foreground">
        لديك ١٢ مشروعًا نشطًا و٣ مهام معلقة.
      </div>
    </div>
  </div>
  <div
    data-state="inactive"
    data-orientation="horizontal"
    role="tabpanel"
    aria-labelledby="tab0-3"
    id="panel0-3"
    tabindex="0"
    data-slot="tabs-content"
    class="text-sm flex-1 outline-none"
    style="animation-duration: 0s"
    hidden=""
  >
    <div
      data-slot="card"
      data-size="default"
      class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col"
      dir="rtl"
    >
      <div
        data-slot="card-header"
        class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"
      >
        <div
          data-slot="card-title"
          class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading"
        >
          نظرة عامة
        </div>
        <div data-slot="card-description" class="text-muted-foreground text-sm">
          عرض مقاييسك الرئيسية وأنشطة المشروع الأخيرة. تتبع التقدم عبر جميع مشاريعك النشطة.
        </div>
      </div>
      <div data-slot="card-content" class="px-(--card-spacing) text-sm text-muted-foreground">
        لديك ١٢ مشروعًا نشطًا و٣ مهام معلقة.
      </div>
    </div>
  </div>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tabs.js
// shadless tabs behavior (wireTabs) — registers with the base; multi-instance: every [data-slot=tabs] root.
// kernel handles click/arrow activation (aria-selected, data-state, hidden,
// roving tabindex on interaction); behavior wires the static a11y surface:
// roving tabindex initial value (kernel only sets it on interactive activation)
(function () {
  shadless.register("tabs", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=tabs]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var list = root.querySelector("[data-slot=tabs-list]");
      if (!list) return;
      var triggers = Array.prototype.slice.call(list.querySelectorAll("[data-slot=tabs-trigger]"));
      var panels = triggers.map(function (t) {
        var id = t.getAttribute("aria-controls");
        return (id && document.getElementById(id)) || null;
      });
      if (!triggers.length) return;
      var initial = Math.max(0, triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active"; }));
      // roving step: skips disabled triggers (radix), horizontal arrows swap under dir=rtl
      function step(idx, key) {
        var last = triggers.length - 1;
        if (key === "Home" || key === "End") {
          var i = key === "Home" ? 0 : last, d = key === "Home" ? 1 : -1;
          for (var n = 0; n <= last; n++, i += d) if (!shadless.h.isDisabled(triggers[i])) return i;
          return -1;
        }
        var fwd = key === "ArrowRight" || key === "ArrowDown";
        if (shadless.h.isRtl(list) && (key === "ArrowRight" || key === "ArrowLeft")) fwd = !fwd;
        for (var k = 1; k <= last; k++) {
          var j = (idx + (fwd ? k : -k) + triggers.length) % triggers.length;
          if (!shadless.h.isDisabled(triggers[j])) return j;
        }
        return -1;
      }
      // panel-less tab lists (tabs-icons, tabs-line, …: triggers only): the
      // kernel's wireTabs needs panels, so activation is wired here — click
      // and arrow keys flip data-state / aria-selected / roving tabindex
      if (panels.some(function (p) { return !p; })) {
        var current = -1;
        var activate = function (i) {
          triggers.forEach(function (t, k) {
            var on = k === i;
            t.setAttribute("data-state", on ? "active" : "inactive");
            t.setAttribute("aria-selected", on ? "true" : "false");
            t.setAttribute("tabindex", on ? "0" : "-1");
          });
          var changed = current !== -1 && current !== i;
          current = i;
          if (changed) shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        };
        triggers.forEach(function (t, i) {
          t.addEventListener("click", function () { if (!t.disabled) activate(i); }, { signal: w.signal });
        });
        list.addEventListener("keydown", function (e) {
          var idx = triggers.indexOf(document.activeElement);
          if (idx === -1) return;
          if (["ArrowRight", "ArrowDown", "ArrowLeft", "ArrowUp", "Home", "End"].indexOf(e.key) < 0) return;
          var next = step(idx, e.key);
          if (next === -1) return;
          e.preventDefault();
          activate(next); triggers[next].focus();
        }, { signal: w.signal });
        activate(initial);
        shadless.instances.set(root, { component: "tabs",
          activate: function (i) { activate(i) },
          active: function () { return triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active" }) },
        })
        return;
      }
      w.persistent = true // kernel wireTabs holds list/trigger listeners with no unwire
      triggers.forEach(function (t) { t.setAttribute("tabindex", "-1"); }); // radix roving: all -1 until focus enters
      // kernel calls onChange on wire (its initial activate) and on every
      // activate, same index included; radix onValueChange fires only when
      // the value CHANGES — emit on real transitions after wiring
      var last = initial, ready = false;
      var wired = RadixKernel.wireTabs({ list: list, triggers: triggers, panels: panels, initial: initial,
        onChange: function (i) {
          if (!ready || i === last) { last = i; return; }
          last = i;
          shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        } });
      ready = true;
      shadless.instances.set(root, { component: "tabs",
        activate: function (i, focus) { wired.activate(i, !!focus) },
        active: function () { return wired.active() },
      })
      // kernel computes ArrowLeft/ArrowRight from the ACTIVE index; radix moves
      // relative to the FOCUSED trigger (focus alone doesn't activate) — absorb in
      // glue with a capture-phase handler; kernel keeps click/Home/End.
      list.addEventListener("keydown", function (e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var idx = triggers.indexOf(document.activeElement);
        if (idx === -1) return;
        e.preventDefault(); e.stopPropagation();
        var next = step(idx, e.key);
        if (next === -1) return;
        wired.activate(next, true);
        triggers[next].focus();
      }, true);
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
| `data-slot="tabs"` |
| `data-slot="tabs-list"` |
| `data-slot="tabs-trigger"` |
| `data-slot="tabs-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`). Markup protocol: see Installation → Behavior protocol.

See the [Radix Tabs](https://www.radix-ui.com/docs/primitives/components/tabs#api-reference) documentation.
