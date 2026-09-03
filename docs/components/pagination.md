---
title: "Pagination"
description: "Pagination with page navigation, next and previous links."
---

# Pagination

Pagination with page navigation, next and previous links.

::::demo pagination-demo
<iframe class="demo" src="/demos/pagination-demo.html" title="pagination-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/pagination-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [pagination-demo.html]
<nav
  role="navigation"
  aria-label="pagination"
  data-slot="pagination"
  class="mx-auto flex w-full justify-center"
>
  <ul data-slot="pagination-content" class="gap-0.5 flex items-center">
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        aria-label="Go to previous page"
        href="#"
        data-variant="ghost"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 pl-1.5!"
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
          class="lucide lucide-chevron-left cn-rtl-flip"
          data-icon="inline-start"
        >
          <path d="m15 18-6-6 6-6"></path></svg
        ><span class="hidden sm:block">Previous</span></a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        href="#"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >1</a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        aria-current="page"
        data-slot="pagination-link"
        data-active="true"
        href="#"
        data-variant="outline"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >2</a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        href="#"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >3</a
      >
    </li>
    <li data-slot="pagination-item">
      <span
        aria-hidden="true"
        data-slot="pagination-ellipsis"
        class="size-8 [&amp;_svg:not([class*='size-'])]:size-4 flex items-center justify-center"
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
          class="lucide lucide-ellipsis"
        >
          <circle cx="12" cy="12" r="1"></circle>
          <circle cx="19" cy="12" r="1"></circle>
          <circle cx="5" cy="12" r="1"></circle></svg
        ><span class="sr-only">More pages</span></span
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        aria-label="Go to next page"
        href="#"
        data-variant="ghost"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 pr-1.5!"
        ><span class="hidden sm:block">Next</span
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
          class="lucide lucide-chevron-right cn-rtl-flip"
          data-icon="inline-end"
        >
          <path d="m9 18 6-6-6-6"></path></svg
      ></a>
    </li>
  </ul>
</nav>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/pagination.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/pagination.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/pagination.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from `dist/components/pagination.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/pagination.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
pagination
└── pagination-content
    ├── pagination-item
    │   └── PaginationPrevious
    ├── pagination-item
    │   └── pagination-link
    ├── pagination-item
    │   └── pagination-ellipsis
    └── pagination-item
        └── PaginationNext
```

## Simple

A simple pagination with only page numbers.

::::demo pagination-simple
<iframe class="demo" src="/demos/pagination-simple.html" title="pagination-simple" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/pagination-simple.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [pagination-simple.html]
<nav
  role="navigation"
  aria-label="pagination"
  data-slot="pagination"
  class="mx-auto flex w-full justify-center"
>
  <ul data-slot="pagination-content" class="gap-0.5 flex items-center">
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        href="#"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >1</a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        aria-current="page"
        data-slot="pagination-link"
        data-active="true"
        href="#"
        data-variant="outline"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >2</a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        href="#"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >3</a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        href="#"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >4</a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        href="#"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >5</a
      >
    </li>
  </ul>
</nav>
```
:::

::::


## Icons Only

Use just the previous and next buttons without page numbers. This is useful for data tables with a rows per page selector.

::::demo pagination-icons-only
<iframe class="demo" src="/demos/pagination-icons-only.html" title="pagination-icons-only" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/pagination-icons-only.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [pagination-icons-only.html]
<div class="flex items-center justify-between gap-4">
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px w-fit"
  >
    <label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="select-rows-per-page"
      >Rows per page</label
    ><button
      type="button"
      role="combobox"
      aria-controls="s0"
      aria-expanded="false"
      aria-autocomplete="none"
      dir="ltr"
      data-state="closed"
      data-slot="select-trigger"
      data-size="default"
      class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 w-20"
      id="s0-trigger"
    >
      <span data-slot="select-value" style="pointer-events: none">25</span
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
  <nav
    role="navigation"
    aria-label="pagination"
    data-slot="pagination"
    class="flex justify-center mx-0 w-auto"
  >
    <ul data-slot="pagination-content" class="gap-0.5 flex items-center">
      <li data-slot="pagination-item">
        <a
          data-slot="pagination-link"
          aria-label="Go to previous page"
          href="#"
          data-variant="ghost"
          data-size="default"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 pl-1.5!"
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
            class="lucide lucide-chevron-left cn-rtl-flip"
            data-icon="inline-start"
          >
            <path d="m15 18-6-6 6-6"></path></svg
          ><span class="hidden sm:block">Previous</span></a
        >
      </li>
      <li data-slot="pagination-item">
        <a
          data-slot="pagination-link"
          aria-label="Go to next page"
          href="#"
          data-variant="ghost"
          data-size="default"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 pr-1.5!"
          ><span class="hidden sm:block">Next</span
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
            class="lucide lucide-chevron-right cn-rtl-flip"
            data-icon="inline-end"
          >
            <path d="m9 18 6-6-6-6"></path></svg
        ></a>
      </li>
    </ul>
  </nav>
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
          ><span id="s0-e0">10</span>
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
          ><span id="s0-e1">25</span>
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
          ><span id="s0-e2">50</span>
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
          ><span id="s0-e3">100</span>
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


shadless has no CLI or framework Link component — `PaginationLink` always renders a real `<a>`; point `href` at whatever your router expects.

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo pagination-rtl
<iframe class="demo" src="/demos/pagination-rtl.html" title="pagination-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/pagination-rtl.html">Open the demo page</a> · <a href="/demos/pagination-rtl-he.html">HE</a> · <a href="/demos/pagination-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [pagination-rtl.html]
<nav
  role="navigation"
  aria-label="pagination"
  data-slot="pagination"
  class="mx-auto flex w-full justify-center"
  dir="rtl"
>
  <ul data-slot="pagination-content" class="gap-0.5 flex items-center">
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        aria-label="Go to previous page"
        href="#"
        data-variant="ghost"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 ps-1.5!"
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
          class="lucide lucide-chevron-left rtl:rotate-180"
          data-icon="inline-start"
        >
          <path d="m15 18-6-6 6-6"></path></svg
        ><span class="hidden sm:block">السابق</span></a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        href="#"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >١</a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        aria-current="page"
        data-slot="pagination-link"
        data-active="true"
        href="#"
        data-variant="outline"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >٢</a
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        href="#"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        >٣</a
      >
    </li>
    <li data-slot="pagination-item">
      <span
        aria-hidden="true"
        data-slot="pagination-ellipsis"
        class="size-8 [&amp;_svg:not([class*='size-'])]:size-4 flex items-center justify-center"
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
          class="lucide lucide-ellipsis"
        >
          <circle cx="12" cy="12" r="1"></circle>
          <circle cx="19" cy="12" r="1"></circle>
          <circle cx="5" cy="12" r="1"></circle></svg
        ><span class="sr-only">More pages</span></span
      >
    </li>
    <li data-slot="pagination-item">
      <a
        data-slot="pagination-link"
        aria-label="Go to next page"
        href="#"
        data-variant="ghost"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 pe-1.5!"
        ><span class="hidden sm:block">التالي</span
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
          class="lucide lucide-chevron-right rtl:rotate-180"
          data-icon="inline-end"
        >
          <path d="m9 18 6-6-6-6"></path></svg
      ></a>
    </li>
  </ul>
</nav>
```
:::

::::
