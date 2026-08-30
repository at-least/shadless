---
title: "Collapsible"
description: "An interactive component which expands/collapses a panel."
---

# Collapsible

An interactive component which expands/collapses a panel.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/collapsible" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/collapsible#api-reference" rel="noopener">api</a></p>

::::demo collapsible-demo
<iframe class="demo" src="/demos/collapsible-demo.html" title="collapsible-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/collapsible-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [collapsible-demo.html]
<div data-state="closed" data-slot="collapsible" class="flex w-[350px] flex-col gap-2">
  <div class="flex items-center justify-between gap-4 px-4">
    <h4 class="text-sm font-semibold">Order #4189</h4>
    <button
      data-slot="collapsible-trigger"
      data-variant="ghost"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
      type="button"
      aria-controls="radix-<auto>_"
      aria-expanded="false"
      data-state="closed"
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
        class="lucide lucide-chevrons-up-down"
      >
        <path d="m7 15 5 5 5-5"></path>
        <path d="m7 9 5-5 5 5"></path></svg
      ><span class="sr-only">Toggle details</span>
    </button>
  </div>
  <div class="flex items-center justify-between rounded-md border px-4 py-2 text-sm">
    <span class="text-muted-foreground">Status</span><span class="font-medium">Shipped</span>
  </div>
  <div
    data-state="closed"
    id="radix-<auto>_"
    hidden=""
    data-slot="collapsible-content"
    class="flex flex-col gap-2"
    style=""
  ></div>
</div>
```
:::

::::


## Installation



**Add shadless to your Tailwind v4 entry:**

```css
@import "shadless";
```

This component has no stylesheet of its own — its styling rides the core theme and utilities in `shadless`.

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/components/collapsible.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/collapsible.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/collapsible.js"></script>
```

**Copy the markup from                                    into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `collapsible-trigger` carries `aria-expanded` + `data-state` and `aria-controls` → the `collapsible-content`, `hidden` while closed. Keys: Enter / Space / click toggles.

The trigger dispatches `shadless:open` / `shadless:close`, bubbling, after the state change, whichever path caused it.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/collapsible.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
collapsible
├── collapsible-trigger
└── collapsible-content
```

## Controlled State

Use the `open` and `onOpenChange` props to control the state.



## Basic

::::demo collapsible-basic
<iframe class="demo" src="/demos/collapsible-basic.html" title="collapsible-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/collapsible-basic.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [collapsible-basic.html]
<div
  data-slot="card"
  data-size="default"
  class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col mx-auto w-full max-w-sm"
>
  <div data-slot="card-content" class="px-(--card-spacing)">
    <div data-state="closed" data-slot="collapsible" class="rounded-md data-[state=open]:bg-muted">
      <button
        data-slot="collapsible-trigger"
        data-variant="ghost"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 group w-full"
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
      >
        Product details<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down ml-auto group-data-[state=open]:rotate-180"
        >
          <path d="m6 9 6 6 6-6"></path>
        </svg>
      </button>
      <div
        data-state="closed"
        id="radix-<auto>_"
        hidden=""
        data-slot="collapsible-content"
        class="flex flex-col items-start gap-2 p-2.5 pt-0 text-sm"
        style=""
      ></div>
    </div>
  </div>
</div>
```
:::

::::


## Settings Panel

Use a trigger button to reveal additional settings.

::::demo collapsible-settings
<iframe class="demo" src="/demos/collapsible-settings.html" title="collapsible-settings" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/collapsible-settings.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [collapsible-settings.html]
<div
  data-slot="card"
  data-size="sm"
  class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col mx-auto w-full max-w-xs"
>
  <div
    data-slot="card-header"
    class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"
  >
    <div
      data-slot="card-title"
      class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading"
    >
      Radius
    </div>
    <div data-slot="card-description" class="text-muted-foreground text-sm">
      Set the corner radius of the element.
    </div>
  </div>
  <div data-slot="card-content" class="px-(--card-spacing)">
    <div data-state="closed" data-slot="collapsible" class="flex items-start gap-2">
      <div
        data-slot="field-group"
        class="data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex-col grid w-full grid-cols-2 gap-2"
      >
        <div
          role="group"
          data-slot="field"
          data-orientation="vertical"
          class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
        >
          <label
            data-slot="field-label"
            class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col sr-only"
            for="radius-x"
            >Radius X</label
          ><input
            data-slot="input"
            class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
            id="radius"
            placeholder="0"
            value="0"
          />
        </div>
        <div
          role="group"
          data-slot="field"
          data-orientation="vertical"
          class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
        >
          <label
            data-slot="field-label"
            class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col sr-only"
            for="radius-y"
            >Radius Y</label
          ><input
            data-slot="input"
            class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
            id="radius"
            placeholder="0"
            value="0"
          />
        </div>
        <div
          data-state="closed"
          id="radix-<auto>_"
          hidden=""
          data-slot="collapsible-content"
          class="col-span-full grid grid-cols-subgrid gap-2"
          style=""
        ></div>
      </div>
      <button
        data-slot="collapsible-trigger"
        data-variant="outline"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
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
          class="lucide lucide-maximize"
        >
          <path d="M8 3H5a2 2 0 0 0-2 2v3"></path>
          <path d="M21 8V5a2 2 0 0 0-2-2h-3"></path>
          <path d="M3 16v3a2 2 0 0 0 2 2h3"></path>
          <path d="M16 21h3a2 2 0 0 0 2-2v-3"></path>
        </svg>
      </button>
    </div>
  </div>
</div>
```
:::

::::


## File Tree

Use nested collapsibles to build a file tree.

::::demo collapsible-file-tree
<iframe class="demo" src="/demos/collapsible-file-tree.html" title="collapsible-file-tree" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/collapsible-file-tree.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [collapsible-file-tree.html]
<div
  data-slot="card"
  data-size="sm"
  class="ring-foreground/10 bg-card text-card-foreground overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col mx-auto w-full max-w-[16rem] gap-2"
>
  <div
    data-slot="card-header"
    class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"
  >
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
        class="rounded-lg p-[3px] group-data-horizontal/tabs:h-8 data-[variant=line]:rounded-none group/tabs-list inline-flex items-center justify-center text-muted-foreground group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col bg-muted w-full"
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
          Explorer</button
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
          Outline
        </button>
      </div>
    </div>
  </div>
  <div data-slot="card-content" class="px-(--card-spacing)">
    <div class="flex flex-col gap-1">
      <div data-state="closed" data-slot="collapsible">
        <button
          data-slot="collapsible-trigger"
          data-variant="ghost"
          data-size="sm"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 group w-full justify-start transition-none hover:bg-accent hover:text-accent-foreground"
          type="button"
          aria-controls="radix-_r_3_"
          aria-expanded="false"
          data-state="closed"
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
            class="lucide lucide-chevron-right transition-transform group-data-[state=open]:rotate-90"
          >
            <path d="m9 18 6-6-6-6"></path></svg
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
            class="lucide lucide-folder"
          >
            <path
              d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
            ></path></svg
          >components
        </button>
        <div
          data-state="closed"
          id="radix-_r_3_"
          hidden=""
          data-slot="collapsible-content"
          class="mt-1 ml-5 style-lyra:ml-4"
          style=""
        ></div>
      </div>
      <div data-state="closed" data-slot="collapsible">
        <button
          data-slot="collapsible-trigger"
          data-variant="ghost"
          data-size="sm"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 group w-full justify-start transition-none hover:bg-accent hover:text-accent-foreground"
          type="button"
          aria-controls="radix-_r_4_"
          aria-expanded="false"
          data-state="closed"
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
            class="lucide lucide-chevron-right transition-transform group-data-[state=open]:rotate-90"
          >
            <path d="m9 18 6-6-6-6"></path></svg
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
            class="lucide lucide-folder"
          >
            <path
              d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
            ></path></svg
          >lib
        </button>
        <div
          data-state="closed"
          id="radix-_r_4_"
          hidden=""
          data-slot="collapsible-content"
          class="mt-1 ml-5 style-lyra:ml-4"
          style=""
        ></div>
      </div>
      <div data-state="closed" data-slot="collapsible">
        <button
          data-slot="collapsible-trigger"
          data-variant="ghost"
          data-size="sm"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 group w-full justify-start transition-none hover:bg-accent hover:text-accent-foreground"
          type="button"
          aria-controls="radix-_r_5_"
          aria-expanded="false"
          data-state="closed"
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
            class="lucide lucide-chevron-right transition-transform group-data-[state=open]:rotate-90"
          >
            <path d="m9 18 6-6-6-6"></path></svg
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
            class="lucide lucide-folder"
          >
            <path
              d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
            ></path></svg
          >hooks
        </button>
        <div
          data-state="closed"
          id="radix-_r_5_"
          hidden=""
          data-slot="collapsible-content"
          class="mt-1 ml-5 style-lyra:ml-4"
          style=""
        ></div>
      </div>
      <div data-state="closed" data-slot="collapsible">
        <button
          data-slot="collapsible-trigger"
          data-variant="ghost"
          data-size="sm"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 group w-full justify-start transition-none hover:bg-accent hover:text-accent-foreground"
          type="button"
          aria-controls="radix-_r_6_"
          aria-expanded="false"
          data-state="closed"
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
            class="lucide lucide-chevron-right transition-transform group-data-[state=open]:rotate-90"
          >
            <path d="m9 18 6-6-6-6"></path></svg
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
            class="lucide lucide-folder"
          >
            <path
              d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
            ></path></svg
          >types
        </button>
        <div
          data-state="closed"
          id="radix-_r_6_"
          hidden=""
          data-slot="collapsible-content"
          class="mt-1 ml-5 style-lyra:ml-4"
          style=""
        ></div>
      </div>
      <div data-state="closed" data-slot="collapsible">
        <button
          data-slot="collapsible-trigger"
          data-variant="ghost"
          data-size="sm"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 group w-full justify-start transition-none hover:bg-accent hover:text-accent-foreground"
          type="button"
          aria-controls="radix-_r_7_"
          aria-expanded="false"
          data-state="closed"
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
            class="lucide lucide-chevron-right transition-transform group-data-[state=open]:rotate-90"
          >
            <path d="m9 18 6-6-6-6"></path></svg
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
            class="lucide lucide-folder"
          >
            <path
              d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
            ></path></svg
          >public
        </button>
        <div
          data-state="closed"
          id="radix-_r_7_"
          hidden=""
          data-slot="collapsible-content"
          class="mt-1 ml-5 style-lyra:ml-4"
          style=""
        ></div>
      </div>
      <button
        data-slot="button"
        data-variant="link"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 w-full justify-start gap-2 text-foreground"
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
        ><span>app.tsx</span></button
      ><button
        data-slot="button"
        data-variant="link"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 w-full justify-start gap-2 text-foreground"
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
        ><span>layout.tsx</span></button
      ><button
        data-slot="button"
        data-variant="link"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 w-full justify-start gap-2 text-foreground"
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
        ><span>globals.css</span></button
      ><button
        data-slot="button"
        data-variant="link"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 w-full justify-start gap-2 text-foreground"
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
        ><span>package.json</span></button
      ><button
        data-slot="button"
        data-variant="link"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 w-full justify-start gap-2 text-foreground"
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
        ><span>tsconfig.json</span></button
      ><button
        data-slot="button"
        data-variant="link"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 w-full justify-start gap-2 text-foreground"
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
        ><span>README.md</span></button
      ><button
        data-slot="button"
        data-variant="link"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 w-full justify-start gap-2 text-foreground"
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
        ><span>.gitignore</span>
      </button>
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

// js/collapsible.js
// shadless collapsible behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("collapsible", { slots: {
    "collapsible-trigger": {
      init: function (trigger) {
        var root = trigger.closest("[data-slot=collapsible]")
        h.linkControls(trigger, root && root.querySelector("[data-slot=collapsible-content]"))
      },
      onClick: function (trigger) {
        var root = trigger.closest("[data-slot=collapsible]")
        var content = root && root.querySelector("[data-slot=collapsible-content]")
        if (!root) return
        var open = trigger.getAttribute("data-state") !== "open"
        h.setDisclosed(trigger, content, open)
        root.setAttribute("data-state", open ? "open" : "closed")
        h.emit(trigger, open ? "open" : "close", "collapsible")
      },
    },
    // accordion trigger: type=single (default) closes siblings; data-type=
    // multiple toggles items independently. Siblings without an item/content
    // ancestor are skipped, not crashed on.,
  } })
})()
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo collapsible-rtl
<iframe class="demo" src="/demos/collapsible-rtl.html" title="collapsible-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/collapsible-rtl.html">Open the demo page</a> · <a href="/demos/collapsible-rtl-he.html">HE</a> · <a href="/demos/collapsible-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [collapsible-rtl.html]
<div data-state="closed" data-slot="collapsible" class="flex w-[350px] flex-col gap-2" dir="rtl">
  <div class="flex items-center justify-between gap-4 px-4">
    <h4 class="text-sm font-semibold">الطلب #4189</h4>
    <button
      data-slot="collapsible-trigger"
      data-variant="ghost"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
      type="button"
      aria-controls="radix-<auto>_"
      aria-expanded="false"
      data-state="closed"
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
        class="lucide lucide-chevrons-up-down"
      >
        <path d="m7 15 5 5 5-5"></path>
        <path d="m7 9 5-5 5 5"></path></svg
      ><span class="sr-only">Toggle details</span>
    </button>
  </div>
  <div class="flex items-center justify-between rounded-md border px-4 py-2 text-sm">
    <span class="text-muted-foreground">الحالة</span><span class="font-medium">تم الشحن</span>
  </div>
  <div
    data-state="closed"
    id="radix-<auto>_"
    hidden=""
    data-slot="collapsible-content"
    class="flex flex-col gap-2"
    style=""
  ></div>
</div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="collapsible"` |
| `data-slot="collapsible-trigger"` |
| `data-slot="collapsible-content"` |

**Runtime:** `collapsible-trigger` carries `aria-expanded` + `data-state` and `aria-controls` → the `collapsible-content`, `hidden` while closed. Keys: Enter / Space / click toggles. The trigger dispatches `shadless:open` / `shadless:close`. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix UI](https://www.radix-ui.com/docs/primitives/components/collapsible#api-reference) documentation for more information.
