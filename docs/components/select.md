---
title: "Select"
description: "Displays a list of options for the user to pick from—triggered by a button."
---

# Select

Displays a list of options for the user to pick from—triggered by a button.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/select" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/select#api-reference" rel="noopener">api</a></p>

::::demo select-demo
<iframe class="demo" src="/demos/select.html" title="select-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/select.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [select.html]
<div>
  <button
    type="button"
    role="combobox"
    aria-controls="s1"
    aria-expanded="false"
    aria-autocomplete="none"
    dir="ltr"
    data-state="closed"
    data-slot="select-trigger"
    data-size="default"
    class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
    id="s1-trigger"
  >
    <span data-slot="select-value" style="pointer-events: none">Apple</span
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
      class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
    >
      <path d="m6 9 6 6 6-6"></path>
    </svg>
  </button>
</div>
<template id="s1-tpl">
  <div
    role="listbox"
    id="s1"
    data-state="open"
    dir="ltr"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div
        role="option"
        aria-labelledby="s1-e0"
        aria-selected="true"
        data-state="checked"
        tabindex="-1"
        data-slot="select-item"
        class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-radix-collection-item=""
        data-highlighted=""
      >
        <span class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ><span
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
              class="lucide lucide-check pointer-events-none"
            >
              <path d="M20 6 9 17l-5-5"></path></svg></span></span
        ><span id="s1-e0">Apple</span>
      </div>
      <div
        role="option"
        aria-labelledby="s1-e1"
        aria-selected="false"
        data-state="unchecked"
        tabindex="-1"
        data-slot="select-item"
        class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-radix-collection-item=""
      >
        <span
          class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
        ></span
        ><span id="s1-e1">Banana</span>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
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
@import "shadless/select.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/select.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/select.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/select.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/select.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<button data-slot="select-trigger" id="<k>-trigger">` | opens on click / Enter / Space / arrows; the `data-slot="select-value"` child shows the selection |
| `<template id="<k>-tpl">` | holds the `data-slot="select-content"` listbox subtree |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`, `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger also dispatches `shadless:change` (`detail: { value, label, item }`). An option's value is its `value` / `data-value` attribute or id — React's value prop never reaches the DOM, so add `data-value` to options whose value differs from their label. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/select.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
select
├── select-trigger
│   └── select-value
└── select-content
    ├── select-group
    │   ├── select-label
    │   ├── select-item
    │   └── select-item
    ├── select-separator
    └── select-group
        ├── select-label
        ├── select-item
        └── select-item
```

## Align Item With Trigger

Use the `position` prop on `SelectContent` to control alignment. When `position="item-aligned"` (default), the popup positions so the selected item appears over the trigger. When `position="popper"`, the popup aligns to the trigger edge.

::::demo select-align-item
<iframe class="demo" src="/demos/select-align-item.html" title="select-align-item" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/select-align-item.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [select-align-item.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col w-full max-w-xs"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="align-item"
        >Align Item</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Toggle to align the item with the trigger.
      </p>
    </div>
    <button
      type="button"
      role="switch"
      aria-checked="true"
      data-state="checked"
      value="on"
      data-slot="switch"
      data-size="default"
      class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50"
      id="align-item"
    >
      <span
        data-state="checked"
        data-slot="switch-thumb"
        class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"
      ></span>
    </button>
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="vertical"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
  >
    <button
      type="button"
      role="combobox"
      aria-controls="s0"
      aria-expanded="false"
      aria-autocomplete="none"
      dir="ltr"
      data-state="closed"
      data-slot="select-trigger"
      data-size="default"
      class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      id="s0-trigger"
    >
      <span data-slot="select-value" style="pointer-events: none">Banana</span
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
        class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
      >
        <path d="m6 9 6 6 6-6"></path>
      </svg>
    </button>
  </div>
</div>
<template id="s0-tpl">
  <div
    role="listbox"
    id="s0"
    data-state="open"
    dir="ltr"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="-trigger" data-slot="select-group" class="scroll-my-1 p-1">
        <div
          role="option"
          aria-labelledby="s0-e0"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e0">Apple</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e1"
          aria-selected="true"
          data-state="checked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
            ><span
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
                class="lucide lucide-check pointer-events-none"
              >
                <path d="M20 6 9 17l-5-5"></path></svg></span></span
          ><span id="s0-e1">Banana</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e2">Blueberry</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e3">Grapes</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e4"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e4">Pineapple</span>
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()

// js/switch.js
// shadless switch behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  function set(root, checked) {
    h.setChecked(root, checked)
    var thumb = root.querySelector("[data-slot=switch-thumb]")
    if (thumb) thumb.setAttribute("data-state", checked ? "checked" : "unchecked")
    h.syncForm(root)
  }
  shadless.register("switch", { slots: {
    switch: {
      init: function (root) {
        h.formMirror(root, {
          read: function () { return root.getAttribute("aria-checked") === "true" },
          write: function (v) { set(root, v) },
        })
      },
      onClick: function (root) {
        var checked = root.getAttribute("aria-checked") !== "true"
        set(root, checked)
        h.emit(root, "change", "switch", { checked: checked })
      },
    },
    // toggle: aria-pressed + data-state on/off (radix Toggle).,
  } })
})()
```
:::

::::


## Groups

Use `SelectGroup`, `SelectLabel`, and `SelectSeparator` to organize items.

::::demo select-groups
<iframe class="demo" src="/demos/select-groups.html" title="select-groups" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/select-groups.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [select-groups.html]
<button
  type="button"
  role="combobox"
  aria-controls="s0"
  aria-expanded="false"
  aria-autocomplete="none"
  dir="ltr"
  data-state="closed"
  data-placeholder=""
  data-slot="select-trigger"
  data-size="default"
  class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 w-full max-w-48"
  id="s0-trigger"
>
  <span data-slot="select-value" style="pointer-events: none">Select a fruit</span
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
    class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</button>
<template id="s0-tpl">
  <div
    role="listbox"
    id="s0"
    data-state="open"
    dir="ltr"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <div
      data-slot="select-scroll-up-button"
      class="bg-popover z-10 flex cursor-default items-center justify-center py-1 [&amp;_svg:not([class*='size-'])]:size-4"
      style="flex-shrink: 0"
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
        class="lucide lucide-chevron-up"
      >
        <path d="m18 15-6-6-6 6"></path>
      </svg>
    </div>
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="s0-e0" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e0" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          Fruits
        </div>
        <div
          role="option"
          aria-labelledby="s0-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e1">Apple</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e2">Banana</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e3">Blueberry</span>
        </div>
      </div>
      <div data-slot="select-separator" class="bg-border -mx-1 my-1 h-px pointer-events-none"></div>
      <div role="group" aria-labelledby="s0-e4" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e4" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          Vegetables
        </div>
        <div
          role="option"
          aria-labelledby="s0-e5"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e5">Carrot</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e6"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e6">Broccoli</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e7"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e7">Spinach</span>
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()
```
:::

::::


## Scrollable

A select with many items that scrolls.

::::demo select-scrollable
<iframe class="demo" src="/demos/select-scrollable.html" title="select-scrollable" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/select-scrollable.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [select-scrollable.html]
<button
  type="button"
  role="combobox"
  aria-controls="s0"
  aria-expanded="false"
  aria-autocomplete="none"
  dir="ltr"
  data-state="closed"
  data-placeholder=""
  data-slot="select-trigger"
  data-size="default"
  class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 w-full max-w-64"
  id="s0-trigger"
>
  <span data-slot="select-value" style="pointer-events: none">Select a timezone</span
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
    class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</button>
<template id="s0-tpl">
  <div
    role="listbox"
    id="s0"
    data-state="open"
    dir="ltr"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <div
      data-slot="select-scroll-up-button"
      class="bg-popover z-10 flex cursor-default items-center justify-center py-1 [&amp;_svg:not([class*='size-'])]:size-4"
      style="flex-shrink: 0"
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
        class="lucide lucide-chevron-up"
      >
        <path d="m18 15-6-6-6 6"></path>
      </svg>
    </div>
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="s0-e0" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e0" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          North America
        </div>
        <div
          role="option"
          aria-labelledby="s0-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e1">Eastern Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e2">Central Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e3">Mountain Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e4"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e4">Pacific Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e5"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e5">Alaska Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e6"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e6">Hawaii Standard Time</span>
        </div>
      </div>
      <div role="group" aria-labelledby="s0-e7" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e7" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          Europe &amp; Africa
        </div>
        <div
          role="option"
          aria-labelledby="s0-e8"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e8">Greenwich Mean Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e9"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e9">Central European Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e10"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e10">Eastern European Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e11"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e11">Western European Summer Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e12"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e12">Central Africa Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e13"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e13">East Africa Time</span>
        </div>
      </div>
      <div role="group" aria-labelledby="s0-e14" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e14" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          Asia
        </div>
        <div
          role="option"
          aria-labelledby="s0-e15"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e15">Moscow Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e16"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e16">India Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e17"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e17">China Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e18"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e18">Japan Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e19"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e19">Korea Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e20"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e20">Indonesia Central Standard Time</span>
        </div>
      </div>
      <div role="group" aria-labelledby="s0-e21" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e21" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          Australia &amp; Pacific
        </div>
        <div
          role="option"
          aria-labelledby="s0-e22"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e22">Australian Western Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e23"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e23">Australian Central Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e24"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e24">Australian Eastern Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e25"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e25">New Zealand Standard Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e26"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e26">Fiji Time</span>
        </div>
      </div>
      <div role="group" aria-labelledby="s0-e27" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e27" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          South America
        </div>
        <div
          role="option"
          aria-labelledby="s0-e28"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e28">Argentina Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e29"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e29">Bolivia Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e30"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e30">Brasilia Time</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e31"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e31">Chile Standard Time</span>
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()
```
:::

::::


## Disabled

::::demo select-disabled
<iframe class="demo" src="/demos/select-disabled.html" title="select-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/select-disabled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [select-disabled.html]
<button
  type="button"
  role="combobox"
  aria-expanded="false"
  aria-autocomplete="none"
  dir="ltr"
  data-state="closed"
  disabled=""
  data-disabled=""
  data-placeholder=""
  data-slot="select-trigger"
  data-size="default"
  class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 w-full max-w-48"
  id="s0-trigger"
>
  <span data-slot="select-value" style="pointer-events: none">Select a fruit</span
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
    class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</button>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()
```
:::

::::


## Invalid

Add the `data-invalid` attribute to the `Field` component and the `aria-invalid` attribute to the `SelectTrigger` component to show an error state.

```tsx showLineNumbers /data-invalid/ /aria-invalid/
<Field data-invalid>
  <FieldLabel>Fruit</FieldLabel>
  <SelectTrigger aria-invalid>
    <SelectValue />
  </SelectTrigger>
</Field>
```

::::demo select-invalid
<iframe class="demo" src="/demos/select-invalid.html" title="select-invalid" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/select-invalid.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [select-invalid.html]
<div
  role="group"
  data-slot="field"
  data-orientation="vertical"
  class="data-[invalid=true]:text-destructive gap-2 group/field flex flex-col *:w-full [&amp;&gt;.sr-only]:w-auto w-full max-w-48"
  data-invalid="true"
>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    >Fruit</label
  ><button
    type="button"
    role="combobox"
    aria-controls="s0"
    aria-expanded="false"
    aria-autocomplete="none"
    dir="ltr"
    data-state="closed"
    data-placeholder=""
    data-slot="select-trigger"
    data-size="default"
    class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
    aria-invalid="true"
    id="s0-trigger"
  >
    <span data-slot="select-value" style="pointer-events: none">Select a fruit</span
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
      class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
    >
      <path d="m6 9 6 6 6-6"></path>
    </svg>
  </button>
  <div role="alert" data-slot="field-error" class="text-destructive text-sm font-normal">
    Please select a fruit.
  </div>
</div>
<template id="s0-tpl">
  <div
    role="listbox"
    id="s0"
    data-state="open"
    dir="ltr"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="-trigger" data-slot="select-group" class="scroll-my-1 p-1">
        <div
          role="option"
          aria-labelledby="s0-e0"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e0">Apple</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e1">Banana</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e2">Blueberry</span>
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo select-rtl
<iframe class="demo" src="/demos/select-rtl.html" title="select-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/select-rtl.html">Open the demo page</a> · <a href="/demos/select-rtl-he.html">HE</a> · <a href="/demos/select-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [select-rtl.html]
<button
  type="button"
  role="combobox"
  aria-controls="s0"
  aria-expanded="false"
  aria-autocomplete="none"
  dir="rtl"
  data-state="closed"
  data-placeholder=""
  data-slot="select-trigger"
  data-size="default"
  class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pe-2 ps-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 w-32"
  id="s0-trigger"
>
  <span data-slot="select-value" style="pointer-events: none">اختر فاكهة</span
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
    class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</button>
<template id="s0-tpl">
  <div
    role="listbox"
    id="s0"
    data-state="open"
    dir="rtl"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    data-lang="ar"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <div
      data-slot="select-scroll-up-button"
      class="bg-popover z-10 flex cursor-default items-center justify-center py-1 [&amp;_svg:not([class*='size-'])]:size-4"
      style="flex-shrink: 0"
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
        class="lucide lucide-chevron-up"
      >
        <path d="m18 15-6-6-6 6"></path>
      </svg>
    </div>
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="s0-e0" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e0" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          الفواكه
        </div>
        <div
          role="option"
          aria-labelledby="s0-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e1">تفاح</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e2">موز</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e3">توت أزرق</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e4"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e4">عنب</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e5"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e5">أناناس</span>
        </div>
      </div>
      <div data-slot="select-separator" class="bg-border -mx-1 my-1 h-px pointer-events-none"></div>
      <div role="group" aria-labelledby="s0-e6" data-slot="select-group" class="scroll-my-1 p-1">
        <div id="s0-e6" data-slot="select-label" class="text-muted-foreground px-1.5 py-1 text-xs">
          الخضروات
        </div>
        <div
          role="option"
          aria-labelledby="s0-e7"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e7">جزر</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e8"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e8">بروكلي</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e9"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e9">سبانخ</span>
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
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
| `data-slot="select"` |
| `data-slot="select-group"` |
| `data-slot="select-value"` |
| `data-slot="select-trigger"` |
| `data-slot="select-content"` |
| `data-slot="select-label"` |
| `data-slot="select-item"` |
| `data-slot="select-separator"` |
| `data-slot="select-scroll-up-button"` |
| `data-slot="select-scroll-down-button"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`, `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger dispatches `shadless:change` (`detail: { value, label, item }`); a `name` attribute submits the selected value; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Select](https://www.radix-ui.com/docs/primitives/components/select#api-reference) documentation.
