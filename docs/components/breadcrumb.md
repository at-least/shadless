---
title: "Breadcrumb"
description: "Displays the path to the current resource using a hierarchy of links."
---

# Breadcrumb

Displays the path to the current resource using a hierarchy of links.

::::demo breadcrumb-demo
<iframe class="demo" src="/demos/breadcrumb-demo.html" title="breadcrumb-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/breadcrumb-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [breadcrumb-demo.html]
<nav aria-label="breadcrumb" data-slot="breadcrumb" class="">
  <ol
    data-slot="breadcrumb-list"
    class="text-muted-foreground gap-1.5 text-sm flex flex-wrap items-center wrap-break-word"
  >
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a href="#" data-slot="breadcrumb-link" class="hover:text-foreground transition-colors"
        >Home</a
      >
    </li>
    <li data-slot="breadcrumb-separator" role="presentation" class="[&amp;&gt;svg]:size-3.5">
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <button
        data-slot="dropdown-menu-trigger"
        data-variant="ghost"
        data-size="icon-sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg"
        type="button"
        id="m0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        data-state="closed"
        data-radixuigo-menu-trigger="m0"
      >
        <span
          data-slot="breadcrumb-ellipsis"
          role="presentation"
          class="size-5 [&amp;&gt;svg]:size-4 flex items-center justify-center"
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
          ><span class="sr-only">More</span></span
        ><span class="sr-only">Toggle menu</span>
      </button>
    </li>
    <li data-slot="breadcrumb-separator" role="presentation" class="[&amp;&gt;svg]:size-3.5">
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a href="#" data-slot="breadcrumb-link" class="hover:text-foreground transition-colors"
        >Components</a
      >
    </li>
    <li data-slot="breadcrumb-separator" role="presentation" class="[&amp;&gt;svg]:size-3.5">
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <span
        data-slot="breadcrumb-page"
        role="link"
        aria-disabled="true"
        aria-current="page"
        class="text-foreground font-normal"
        >Breadcrumb</span
      >
    </li>
  </ol>
</nav>
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
        Documentation
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
        Themes
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
        GitHub
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// shadless dropdown-menu behavior — registers with shadless.h.installMenuFamily,
// the shared wireMenu glue also used by context-menu.js (core.js has the full
// story: the two files' bodies were byte-identical but for this line, so the
// body now lives once and both files just point at it).
(function () {
  shadless.register("dropdown-menu", { init: shadless.h.installMenuFamily })
})()
```
:::

::::

## Installation

**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/breadcrumb.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/breadcrumb.css` | this component's slot styles (`@apply` source — your build compiles it) |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
breadcrumb
└── breadcrumb-list
    ├── breadcrumb-item
    │   └── breadcrumb-link
    ├── breadcrumb-separator
    ├── breadcrumb-item
    │   └── breadcrumb-link
    ├── breadcrumb-separator
    └── breadcrumb-item
        └── breadcrumb-page
```

## Basic

A basic breadcrumb with a home link and a components link.

::::demo breadcrumb-basic
<iframe class="demo" src="/demos/breadcrumb-basic.html" title="breadcrumb-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/breadcrumb-basic.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [breadcrumb-basic.html]
<nav aria-label="breadcrumb" data-slot="breadcrumb" class="">
  <ol
    data-slot="breadcrumb-list"
    class="text-muted-foreground gap-1.5 text-sm flex flex-wrap items-center wrap-break-word"
  >
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a data-slot="breadcrumb-link" class="hover:text-foreground transition-colors" href="#"
        >Home</a
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a data-slot="breadcrumb-link" class="hover:text-foreground transition-colors" href="#"
        >Components</a
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <span
        data-slot="breadcrumb-page"
        role="link"
        aria-disabled="true"
        aria-current="page"
        class="text-foreground font-normal"
        >Breadcrumb</span
      >
    </li>
  </ol>
</nav>
```
:::

::::

## Custom separator

Use a custom component as `children` for `data-slot="breadcrumb-separator"` to create a custom separator.

::::demo breadcrumb-separator
<iframe class="demo" src="/demos/breadcrumb-separator.html" title="breadcrumb-separator" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/breadcrumb-separator.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [breadcrumb-separator.html]
<nav aria-label="breadcrumb" data-slot="breadcrumb" class="">
  <ol
    data-slot="breadcrumb-list"
    class="text-muted-foreground gap-1.5 text-sm flex flex-wrap items-center wrap-break-word"
  >
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a href="/" data-slot="breadcrumb-link" class="hover:text-foreground transition-colors"
        >Home</a
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-dot"
      >
        <circle cx="12.1" cy="12.1" r="1"></circle>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a
        href="/components"
        data-slot="breadcrumb-link"
        class="hover:text-foreground transition-colors"
        >Components</a
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-dot"
      >
        <circle cx="12.1" cy="12.1" r="1"></circle>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <span
        data-slot="breadcrumb-page"
        role="link"
        aria-disabled="true"
        aria-current="page"
        class="text-foreground font-normal"
        >Breadcrumb</span
      >
    </li>
  </ol>
</nav>
```
:::

::::

## Dropdown

You can compose a `data-slot="breadcrumb-item"` with a dropdown-menu trigger (see the [Dropdown Menu](/components/dropdown-menu) page) to create a dropdown in the breadcrumb.

::::demo breadcrumb-dropdown
<iframe class="demo" src="/demos/breadcrumb-dropdown.html" title="breadcrumb-dropdown" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/breadcrumb-dropdown.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [breadcrumb-dropdown.html]
<nav aria-label="breadcrumb" data-slot="breadcrumb" class="">
  <ol
    data-slot="breadcrumb-list"
    class="text-muted-foreground gap-1.5 text-sm flex flex-wrap items-center wrap-break-word"
  >
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a href="/" data-slot="breadcrumb-link" class="hover:text-foreground transition-colors"
        >Home</a
      >
    </li>
    <li data-slot="breadcrumb-separator" role="presentation" class="[&amp;&gt;svg]:size-3.5">
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
        class="lucide lucide-dot"
      >
        <circle cx="12.1" cy="12.1" r="1"></circle>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <button
        class="flex items-center gap-1"
        type="button"
        id="m0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        data-state="closed"
        data-slot="dropdown-menu-trigger"
        data-radixuigo-menu-trigger="m0"
      >
        Components<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down size-3.5"
        >
          <path d="m6 9 6 6 6-6"></path>
        </svg>
      </button>
    </li>
    <li data-slot="breadcrumb-separator" role="presentation" class="[&amp;&gt;svg]:size-3.5">
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
        class="lucide lucide-dot"
      >
        <circle cx="12.1" cy="12.1" r="1"></circle>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <span
        data-slot="breadcrumb-page"
        role="link"
        aria-disabled="true"
        aria-current="page"
        class="text-foreground font-normal"
        >Breadcrumb</span
      >
    </li>
  </ol>
</nav>
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
        Documentation
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
        Themes
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
        GitHub
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// shadless dropdown-menu behavior — registers with shadless.h.installMenuFamily,
// the shared wireMenu glue also used by context-menu.js (core.js has the full
// story: the two files' bodies were byte-identical but for this line, so the
// body now lives once and both files just point at it).
(function () {
  shadless.register("dropdown-menu", { init: shadless.h.installMenuFamily })
})()
```
:::

::::

## Collapsed

We provide a `data-slot="breadcrumb-ellipsis"` component to show a collapsed state when the breadcrumb is too long.

::::demo breadcrumb-ellipsis
<iframe class="demo" src="/demos/breadcrumb-ellipsis.html" title="breadcrumb-ellipsis" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/breadcrumb-ellipsis.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [breadcrumb-ellipsis.html]
<nav aria-label="breadcrumb" data-slot="breadcrumb" class="">
  <ol
    data-slot="breadcrumb-list"
    class="text-muted-foreground gap-1.5 text-sm flex flex-wrap items-center wrap-break-word"
  >
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a href="/" data-slot="breadcrumb-link" class="hover:text-foreground transition-colors"
        >Home</a
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <span
        data-slot="breadcrumb-ellipsis"
        role="presentation"
        aria-hidden="true"
        class="size-5 [&amp;&gt;svg]:size-4 flex items-center justify-center"
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
        ><span class="sr-only">More</span></span
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a
        href="/docs/components"
        data-slot="breadcrumb-link"
        class="hover:text-foreground transition-colors"
        >Components</a
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <span
        data-slot="breadcrumb-page"
        role="link"
        aria-disabled="true"
        aria-current="page"
        class="text-foreground font-normal"
        >Breadcrumb</span
      >
    </li>
  </ol>
</nav>
```
:::

::::

## Link component

To use a custom link component from your routing library, you can use the `asChild` prop on `data-slot="breadcrumb-link"`.

::::demo breadcrumb-link
<iframe class="demo" src="/demos/breadcrumb-link.html" title="breadcrumb-link" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/breadcrumb-link.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [breadcrumb-link.html]
<nav aria-label="breadcrumb" data-slot="breadcrumb" class="">
  <ol
    data-slot="breadcrumb-list"
    class="text-muted-foreground gap-1.5 text-sm flex flex-wrap items-center wrap-break-word"
  >
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a href="/" data-slot="breadcrumb-link" class="hover:text-foreground transition-colors"
        >Home</a
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a
        href="/components"
        data-slot="breadcrumb-link"
        class="hover:text-foreground transition-colors"
        >Components</a
      >
    </li>
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      class="[&amp;&gt;svg]:size-3.5"
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
        class="lucide lucide-chevron-right cn-rtl-flip"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <span
        data-slot="breadcrumb-page"
        role="link"
        aria-disabled="true"
        aria-current="page"
        class="text-foreground font-normal"
        >Breadcrumb</span
      >
    </li>
  </ol>
</nav>
```
:::

::::

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo breadcrumb-rtl
<iframe class="demo" src="/demos/breadcrumb-rtl.html" title="breadcrumb-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/breadcrumb-rtl.html">Open the demo page</a> · <a href="/demos/breadcrumb-rtl-he.html">HE</a> · <a href="/demos/breadcrumb-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [breadcrumb-rtl.html]
<nav aria-label="breadcrumb" data-slot="breadcrumb" class="" dir="rtl">
  <ol
    data-slot="breadcrumb-list"
    class="text-muted-foreground gap-1.5 text-sm flex flex-wrap items-center wrap-break-word"
  >
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <a href="/" data-slot="breadcrumb-link" class="hover:text-foreground transition-colors"
        >الرئيسية</a
      >
    </li>
    <li data-slot="breadcrumb-separator" role="presentation" class="[&amp;&gt;svg]:size-3.5">
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
        class="lucide lucide-dot"
      >
        <circle cx="12.1" cy="12.1" r="1"></circle>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <button
        class="flex items-center gap-1"
        type="button"
        id="m0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        data-state="closed"
        data-slot="dropdown-menu-trigger"
        data-radixuigo-menu-trigger="m0"
      >
        المكونات<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down size-3.5"
        >
          <path d="m6 9 6 6 6-6"></path>
        </svg>
      </button>
    </li>
    <li data-slot="breadcrumb-separator" role="presentation" class="[&amp;&gt;svg]:size-3.5">
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
        class="lucide lucide-dot"
      >
        <circle cx="12.1" cy="12.1" r="1"></circle>
      </svg>
    </li>
    <li data-slot="breadcrumb-item" class="gap-1 inline-flex items-center">
      <span
        data-slot="breadcrumb-page"
        role="link"
        aria-disabled="true"
        aria-current="page"
        class="text-foreground font-normal"
        >مسار التنقل</span
      >
    </li>
  </ol>
</nav>
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
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden"
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
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        التوثيق
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
        السمات
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
        جيت هاب
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// shadless dropdown-menu behavior — registers with shadless.h.installMenuFamily,
// the shared wireMenu glue also used by context-menu.js (core.js has the full
// story: the two files' bodies were byte-identical but for this line, so the
// body now lives once and both files just point at it).
(function () {
  shadless.register("dropdown-menu", { init: shadless.h.installMenuFamily })
})()
```
:::

::::

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="breadcrumb"` |
| `data-slot="breadcrumb-list"` |
| `data-slot="breadcrumb-item"` |
| `data-slot="breadcrumb-link"` |
| `data-slot="breadcrumb-page"` |
| `data-slot="breadcrumb-separator"` |
| `data-slot="breadcrumb-ellipsis"` |

**Runtime:** no JavaScript — this is markup + CSS. No `cva`-declared variants. Check `dist/css/breadcrumb.css` for any `data-*` attribute this slot's styling depends on.
