---
title: "Button Group"
description: "A container that groups related buttons together with consistent styling."
---

# Button Group

A container that groups related buttons together with consistent styling.

::::demo button-group-demo
<iframe class="demo" src="/demos/button-group-demo.html" title="button-group-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-demo.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none hidden sm:flex"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
      aria-label="Go Back"
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
        class="lucide lucide-arrow-left"
      >
        <path d="m12 19-7-7 7-7"></path>
        <path d="M19 12H5"></path>
      </svg>
    </button>
  </div>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Archive</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Report
    </button>
  </div>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Snooze</button
    ><button
      data-slot="dropdown-menu-trigger"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
      aria-label="More Options"
      type="button"
      id="m0-trigger"
      aria-haspopup="menu"
      aria-expanded="false"
      data-state="closed"
      data-radixuigo-menu-trigger="m0"
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
        <circle cx="5" cy="12" r="1"></circle>
      </svg>
    </button>
  </div>
</div>
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
          class="lucide lucide-mail-check"
        >
          <path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"></path>
          <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
          <path d="m16 19 2 2 4-4"></path></svg
        >Mark as Read
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
          class="lucide lucide-archive"
        >
          <rect width="20" height="5" x="2" y="3" rx="1"></rect>
          <path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"></path>
          <path d="M10 12h4"></path></svg
        >Archive
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
          class="lucide lucide-clock"
        >
          <circle cx="12" cy="12" r="10"></circle>
          <polyline points="12 6 12 12 16 14"></polyline></svg
        >Snooze
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
          class="lucide lucide-calendar-plus"
        >
          <path d="M8 2v4"></path>
          <path d="M16 2v4"></path>
          <path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
          <path d="M3 10h18"></path>
          <path d="M16 19h6"></path>
          <path d="M19 16v6"></path></svg
        >Add to Calendar
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
          class="lucide lucide-list-filter"
        >
          <path d="M3 6h18"></path>
          <path d="M7 12h10"></path>
          <path d="M10 18h4"></path></svg
        >Add to List
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
          class="lucide lucide-tag"
        >
          <path
            d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z"
          ></path>
          <circle cx="7.5" cy="7.5" r=".5" fill="currentColor"></circle></svg
        >Label As...<svg
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
          class="lucide lucide-trash2"
        >
          <path d="M3 6h18"></path>
          <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
          <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
          <line x1="10" x2="10" y1="11" y2="17"></line>
          <line x1="14" x2="14" y1="11" y2="17"></line></svg
        >Trash
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
        >Personal
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
        >Work
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
        >Other
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


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/button-group.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/button-group.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/button-group.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                     into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/button-group.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `orientation="outline"` (JSX prop) | `data-orientation="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
button-group
├── Button or Input
├── button-group-separator
└── ButtonGroupText
```

## Accessibility

- The `ButtonGroup` component has the `role` attribute set to `group`.
- Use <kbd>Tab</kbd> to navigate between the buttons in the group.
- Use `aria-label` or `aria-labelledby` to label the button group.

```tsx showLineNumbers
<ButtonGroup aria-label="Button group">
  <Button>Button 1</Button>
  <Button>Button 2</Button>
</ButtonGroup>
```

## ButtonGroup vs ToggleGroup

- Use the `ButtonGroup` component when you want to group buttons that perform an action.
- Use the `ToggleGroup` component when you want to group buttons that toggle a state.

## Orientation

Set the `orientation` prop to change the button group layout.

::::demo button-group-orientation
<iframe class="demo" src="/demos/button-group-orientation.html" title="button-group-orientation" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-orientation.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-orientation.html]
<div
  role="group"
  data-slot="button-group"
  data-orientation="vertical"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-b-lg! flex-col [&amp;&gt;*:not(:first-child)]:rounded-t-none [&amp;&gt;*:not(:first-child)]:border-t-0 [&amp;&gt;*:not(:last-child)]:rounded-b-none h-fit"
  aria-label="Media controls"
>
  <button
    data-slot="button"
    data-variant="outline"
    data-size="icon"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
      class="lucide lucide-plus"
    >
      <path d="M5 12h14"></path>
      <path d="M12 5v14"></path>
    </svg></button
  ><button
    data-slot="button"
    data-variant="outline"
    data-size="icon"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
      class="lucide lucide-minus"
    >
      <path d="M5 12h14"></path>
    </svg>
  </button>
</div>
```
:::

::::


## Size

Control the size of buttons using the `size` prop on individual buttons.

::::demo button-group-size
<iframe class="demo" src="/demos/button-group-size.html" title="button-group-size" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-size.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-size.html]
<div class="flex flex-col items-start gap-8">
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    >
      Small</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    >
      Button</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    >
      Group</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="icon-sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg"
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
        class="lucide lucide-plus"
      >
        <path d="M5 12h14"></path>
        <path d="M12 5v14"></path>
      </svg>
    </button>
  </div>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Default</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Button</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Group</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
        class="lucide lucide-plus"
      >
        <path d="M5 12h14"></path>
        <path d="M12 5v14"></path>
      </svg>
    </button>
  </div>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="lg"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-9 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Large</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="lg"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-9 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Button</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="lg"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-9 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Group</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="icon-lg"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-9"
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
        class="lucide lucide-plus"
      >
        <path d="M5 12h14"></path>
        <path d="M12 5v14"></path>
      </svg>
    </button>
  </div>
</div>
```
:::

::::


## Nested

Nest `<ButtonGroup>` components to create button groups with spacing.

::::demo button-group-nested
<iframe class="demo" src="/demos/button-group-nested.html" title="button-group-nested" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-nested.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-nested.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
        class="lucide lucide-plus"
      >
        <path d="M5 12h14"></path>
        <path d="M12 5v14"></path>
      </svg>
    </button>
  </div>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <div
      data-slot="input-group"
      role="group"
      class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
    >
      <input
        data-slot="input-group-control"
        class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
        placeholder="Send a message..."
      />
      <div
        role="group"
        data-slot="tooltip-trigger"
        data-align="inline-end"
        class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
        data-state="closed"
        id="k0-trigger"
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
          class="lucide lucide-audio-lines"
        >
          <path d="M2 10v3"></path>
          <path d="M6 6v11"></path>
          <path d="M10 3v18"></path>
          <path d="M14 8v7"></path>
          <path d="M18 5v13"></path>
          <path d="M22 10v3"></path>
        </svg>
      </div>
    </div>
  </div>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(601px, 39px);
      min-width: max-content;
      --radix-popper-transform-origin: 38.50000000000006px 23px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 57px;
      --radix-popper-anchor-width: 1264px;
      --radix-popper-anchor-height: 28px;
    "
  >
    <div
      data-side="top"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      Voice Mode<span
        style="position: absolute; bottom: 0px; transform: translateY(100%); left: 33.5px"
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k0-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >Voice Mode</span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()
```
:::

::::


## Separator

The `ButtonGroupSeparator` component visually divides buttons within a group.

Buttons with variant `outline` do not need a separator since they have a border. For other variants, a separator is recommended to improve the visual hierarchy.

::::demo button-group-separator
<iframe class="demo" src="/demos/button-group-separator.html" title="button-group-separator" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-separator.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-separator.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
>
  <button
    data-slot="button"
    data-variant="secondary"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
  >
    Copy
  </button>
  <div
    data-orientation="vertical"
    role="none"
    data-slot="button-group-separator"
    class="shrink-0 data-horizontal:h-px data-vertical:w-px data-vertical:self-stretch bg-input relative self-stretch data-horizontal:mx-px data-horizontal:w-auto data-vertical:my-px data-vertical:h-auto"
  ></div>
  <button
    data-slot="button"
    data-variant="secondary"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
  >
    Paste
  </button>
</div>
```
:::

::::


## Split

Create a split button group by adding two buttons separated by a `ButtonGroupSeparator`.

::::demo button-group-split
<iframe class="demo" src="/demos/button-group-split.html" title="button-group-split" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-split.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-split.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
>
  <button
    data-slot="button"
    data-variant="secondary"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  >
    Button
  </button>
  <div
    data-orientation="vertical"
    role="none"
    data-slot="button-group-separator"
    class="shrink-0 data-horizontal:h-px data-vertical:w-px data-vertical:self-stretch bg-input relative self-stretch data-horizontal:mx-px data-horizontal:w-auto data-vertical:my-px data-vertical:h-auto"
  ></div>
  <button
    data-slot="button"
    data-variant="secondary"
    data-size="icon"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground size-8"
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
      class="tabler-icon tabler-icon-plus"
    >
      <path d="M12 5l0 14"></path>
      <path d="M5 12l14 0"></path>
    </svg>
  </button>
</div>
```
:::

::::


## Input

Wrap an `Input` component with buttons.

::::demo button-group-input
<iframe class="demo" src="/demos/button-group-input.html" title="button-group-input" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-input.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-input.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
>
  <input
    data-slot="input"
    class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
    placeholder="Search..."
  /><button
    data-slot="button"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    aria-label="Search"
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
      class="lucide lucide-search"
    >
      <circle cx="11" cy="11" r="8"></circle>
      <path d="m21 21-4.3-4.3"></path>
    </svg>
  </button>
</div>
```
:::

::::


## Input Group

Wrap an `InputGroup` component to create complex input layouts.

::::demo button-group-input-group
<iframe class="demo" src="/demos/button-group-input-group.html" title="button-group-input-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-input-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-input-group.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none [--radius:9999rem]"
>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
        class="lucide lucide-plus"
      >
        <path d="M5 12h14"></path>
        <path d="M12 5v14"></path>
      </svg>
    </button>
  </div>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <div
      data-slot="input-group"
      role="group"
      class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
    >
      <input
        data-slot="input-group-control"
        class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
        placeholder="Send a message..."
      />
      <div
        role="group"
        data-slot="input-group-addon"
        data-align="inline-end"
        class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
      >
        <button
          data-slot="tooltip-trigger"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[&gt;svg]:p-0 data-[active=true]:bg-orange-100 data-[active=true]:text-orange-700 dark:data-[active=true]:bg-orange-800 dark:data-[active=true]:text-orange-100"
          type="button"
          data-active="false"
          aria-pressed="false"
          data-state="closed"
          id="k0-trigger"
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
            class="lucide lucide-audio-lines"
          >
            <path d="M2 10v3"></path>
            <path d="M6 6v11"></path>
            <path d="M10 3v18"></path>
            <path d="M14 8v7"></path>
            <path d="M18 5v13"></path>
            <path d="M22 10v3"></path>
          </svg>
        </button>
      </div>
    </div>
  </div>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 39px);
      min-width: max-content;
      --radix-popper-transform-origin: 27.7656px 23px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 57px;
      --radix-popper-anchor-width: 40px;
      --radix-popper-anchor-height: 33px;
    "
  >
    <div
      data-side="top"
      data-align="center"
      data-state="delayed-open"
      data-slot="tooltip-content"
      class="data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background"
      style="
        --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-tooltip-content-available-width: var(--radix-popper-available-width);
        --radix-tooltip-content-available-height: var(--radix-popper-available-height);
        --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);
        --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      Voice Mode<span
        style="position: absolute; bottom: 0px; transform: translateY(100%); left: 22.7656px"
        ><svg
          class="size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground"
          width="10"
          height="5"
          viewBox="0 0 30 10"
          preserveAspectRatio="none"
          style="display: block"
        >
          <polygon points="0,0 30,0 15,10"></polygon></svg></span
      ><span
        id="k0-e0"
        role="tooltip"
        style="
          position: absolute;
          border: 0px;
          width: 1px;
          height: 1px;
          padding: 0px;
          margin: -1px;
          overflow: hidden;
          clip: rect(0px, 0px, 0px, 0px);
          white-space: nowrap;
          overflow-wrap: normal;
        "
        >Voice Mode</span
      >
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/tooltip.js
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()
```
:::

::::


## Dropdown Menu

Create a split button group with a `DropdownMenu` component.

::::demo button-group-dropdown
<iframe class="demo" src="/demos/button-group-dropdown.html" title="button-group-dropdown" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-dropdown.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-dropdown.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
>
  <button
    data-slot="button"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  >
    Follow</button
  ><button
    data-slot="dropdown-menu-trigger"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 pl-2!"
    type="button"
    id="m0-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-radixuigo-menu-trigger="m0"
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
      class="lucide lucide-chevron-down"
    >
      <path d="m6 9 6 6 6-6"></path>
    </svg>
  </button>
</div>
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
          class="lucide lucide-volume-off"
        >
          <path d="M16 9a5 5 0 0 1 .95 2.293"></path>
          <path d="M19.364 5.636a9 9 0 0 1 1.889 9.96"></path>
          <path d="m2 2 20 20"></path>
          <path
            d="m7 7-.587.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298V11"
          ></path>
          <path d="M9.828 4.172A.686.686 0 0 1 11 4.657v.686"></path></svg
        >Mute Conversation
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
          class="lucide lucide-check"
        >
          <path d="M20 6 9 17l-5-5"></path></svg
        >Mark as Read
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
          class="lucide lucide-triangle-alert"
        >
          <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"></path>
          <path d="M12 9v4"></path>
          <path d="M12 17h.01"></path></svg
        >Report Conversation
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
          class="lucide lucide-user-round-x"
        >
          <path d="M2 21a8 8 0 0 1 11.873-7"></path>
          <circle cx="10" cy="8" r="5"></circle>
          <path d="m17 17 5 5"></path>
          <path d="m22 17-5 5"></path></svg
        >Block User
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
        >Share Conversation
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
          class="lucide lucide-copy"
        >
          <rect width="14" height="14" x="8" y="8" rx="2" ry="2"></rect>
          <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path></svg
        >Copy Conversation
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
        >Delete Conversation
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


## Select

Pair with a `Select` component.

::::demo button-group-select
<iframe class="demo" src="/demos/button-group-select.html" title="button-group-select" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-select.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-select.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
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
      class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 font-mono"
      id="s0-trigger"
    >
      $<svg
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
      </svg></button
    ><input
      data-slot="input"
      class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
      placeholder="10.00"
      pattern="[0-9]*"
    />
  </div>
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
      aria-label="Send"
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
        class="lucide lucide-arrow-right"
      >
        <path d="M5 12h14"></path>
        <path d="m12 5 7 7-7 7"></path>
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
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none min-w-24"
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
          data-state="checked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
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
          ><span id="s0-e0">$ <span class="text-muted-foreground">US Dollar</span></span>
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
          ><span id="s0-e1">€ <span class="text-muted-foreground">Euro</span></span>
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
          ><span id="s0-e2">£ <span class="text-muted-foreground">British Pound</span></span>
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


## Popover

Use with a `Popover` component.

::::demo button-group-popover
<iframe class="demo" src="/demos/button-group-popover.html" title="button-group-popover" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-popover.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [button-group-popover.html]
<div
  role="group"
  data-slot="button-group"
  class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&amp;&gt;*:not(:first-child)]:rounded-l-none [&amp;&gt;*:not(:first-child)]:border-l-0 [&amp;&gt;*:not(:last-child)]:rounded-r-none"
>
  <button
    data-slot="button"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
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
      class="lucide lucide-bot"
    >
      <path d="M12 8V4H8"></path>
      <rect width="16" height="12" x="4" y="8" rx="2"></rect>
      <path d="M2 14h2"></path>
      <path d="M20 14h2"></path>
      <path d="M15 13v2"></path>
      <path d="M9 13v2"></path>
    </svg>
    Copilot</button
  ><button
    data-slot="popover-trigger"
    data-variant="outline"
    data-size="icon"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
    aria-label="Open Popover"
    type="button"
    aria-haspopup="dialog"
    aria-expanded="false"
    aria-controls="k0"
    data-state="closed"
    id="k0-trigger"
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
      class="lucide lucide-chevron-down"
    >
      <path d="m6 9 6 6 6-6"></path>
    </svg>
  </button>
</div>
<template id="k0-portal">
  <div
    data-radix-popper-content-wrapper=""
    style="
      position: fixed;
      left: 0px;
      top: 0px;
      transform: translate(0px, 45px);
      min-width: max-content;
      --radix-popper-transform-origin: 100% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 675px;
      --radix-popper-anchor-width: 40px;
      --radix-popper-anchor-height: 33px;
    "
  >
    <div
      data-side="bottom"
      data-align="end"
      data-state="open"
      role="dialog"
      id="k0"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 p-2.5 shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden rounded-xl text-sm"
      tabindex="-1"
      style="
        --radix-popover-content-transform-origin: var(--radix-popper-transform-origin);
        --radix-popover-content-available-width: var(--radix-popper-available-width);
        --radix-popover-content-available-height: var(--radix-popper-available-height);
        --radix-popover-trigger-width: var(--radix-popper-anchor-width);
        --radix-popover-trigger-height: var(--radix-popper-anchor-height);
      "
      id="k0"
    >
      <div data-slot="popover-header" class="flex flex-col gap-0.5 text-sm">
        <div data-slot="popover-title" class="font-medium">Start a new task with Copilot</div>
        <p data-slot="popover-description" class="text-muted-foreground">
          Describe your task in natural language.
        </p>
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
          for="task"
          >Task Description</label
        ><textarea
          data-slot="textarea"
          class="border-input dark:bg-input/30 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 rounded-lg border bg-transparent px-2.5 py-2 text-base transition-colors focus-visible:ring-3 aria-invalid:ring-3 md:text-sm flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 resize-none"
          id="task"
          placeholder="I need to..."
        ></textarea>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          Copilot will open a pull request for review.
        </p>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/popover.js
// shadless popover behavior (wirePopover) — registers with the base; multi-instance: every
// [data-slot=popover-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("popover", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=popover-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, content = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        if (content) content.setAttribute("data-state", s);
      }
      function mount() {
        var frag = tpl.content.cloneNode(true);
        var host = document.createElement("div");
        host.appendChild(frag);
        content = host.querySelector("[data-slot=popover-content]");
        document.body.appendChild(content);
        setState("open");
        handles = RadixKernel.wirePopover({
          content: content,
          trigger: trigger,
          contentStateFlip: true,
          onClosed: function () {
            setState("closed");
            if (content) content.remove();
            content = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "popover");
          },
        });
        open = true;
        shadless.h.emit(trigger, "open", "popover");
      }
      function unmount() { if (handles) handles.close(); }
      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "popover",
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

::::demo button-group-rtl
<iframe class="demo" src="/demos/button-group-rtl.html" title="button-group-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-rtl.html">Open the demo page</a> · <a href="/demos/button-group-rtl-he.html">HE</a> · <a href="/demos/button-group-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [button-group-rtl.html]
<div dir="rtl">
  <div
    role="group"
    data-slot="button-group"
    class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-e-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-e-lg! [&amp;&gt;*:not(:first-child)]:rounded-s-none [&amp;&gt;*:not(:first-child)]:border-s-0 [&amp;&gt;*:not(:last-child)]:rounded-e-none"
  >
    <div
      role="group"
      data-slot="button-group"
      class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-e-lg group/button-group w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-e-lg! [&amp;&gt;*:not(:first-child)]:rounded-s-none [&amp;&gt;*:not(:first-child)]:border-s-0 [&amp;&gt;*:not(:last-child)]:rounded-e-none hidden sm:flex"
    >
      <button
        data-slot="button"
        data-variant="outline"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        aria-label="Go Back"
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
          class="lucide lucide-arrow-left rtl:rotate-180"
        >
          <path d="m12 19-7-7 7-7"></path>
          <path d="M19 12H5"></path>
        </svg>
      </button>
    </div>
    <div
      role="group"
      data-slot="button-group"
      class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-e-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-e-lg! [&amp;&gt;*:not(:first-child)]:rounded-s-none [&amp;&gt;*:not(:first-child)]:border-s-0 [&amp;&gt;*:not(:last-child)]:rounded-e-none"
    >
      <button
        data-slot="button"
        data-variant="outline"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
      >
        أرشفة</button
      ><button
        data-slot="button"
        data-variant="outline"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
      >
        تقرير
      </button>
    </div>
    <div
      role="group"
      data-slot="button-group"
      class="has-[&gt;[data-slot=button-group]]:gap-2 has-[select[aria-hidden=true]:last-child]:[&amp;&gt;[data-slot=select-trigger]:last-of-type]:rounded-e-lg group/button-group flex w-fit items-stretch *:focus-visible:relative *:focus-visible:z-10 [&amp;&gt;[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&amp;&gt;input]:flex-1 [&amp;&gt;[data-slot]:not(:has(~[data-slot]))]:rounded-e-lg! [&amp;&gt;*:not(:first-child)]:rounded-s-none [&amp;&gt;*:not(:first-child)]:border-s-0 [&amp;&gt;*:not(:last-child)]:rounded-e-none"
    >
      <button
        data-slot="button"
        data-variant="outline"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
      >
        تأجيل</button
      ><button
        data-slot="dropdown-menu-trigger"
        data-variant="outline"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
        aria-label="More Options"
        type="button"
        id="m0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        data-state="closed"
        data-radixuigo-menu-trigger="m0"
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
          <circle cx="5" cy="12" r="1"></circle>
        </svg>
      </button>
    </div>
  </div>
</div>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="end"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden w-40"
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
          class="lucide lucide-mail-check"
        >
          <path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"></path>
          <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
          <path d="m16 19 2 2 4-4"></path></svg
        >وضع علامة كمقروء
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
          class="lucide lucide-archive"
        >
          <rect width="20" height="5" x="2" y="3" rx="1"></rect>
          <path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"></path>
          <path d="M10 12h4"></path></svg
        >أرشفة
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
          class="lucide lucide-clock"
        >
          <circle cx="12" cy="12" r="10"></circle>
          <polyline points="12 6 12 12 16 14"></polyline></svg
        >تأجيل
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
          class="lucide lucide-calendar-plus"
        >
          <path d="M8 2v4"></path>
          <path d="M16 2v4"></path>
          <path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
          <path d="M3 10h18"></path>
          <path d="M16 19h6"></path>
          <path d="M19 16v6"></path></svg
        >إضافة إلى التقويم
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
          class="lucide lucide-list-filter"
        >
          <path d="M3 6h18"></path>
          <path d="M7 12h10"></path>
          <path d="M10 18h4"></path></svg
        >إضافة إلى القائمة
      </div>
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
          class="lucide lucide-tag"
        >
          <path
            d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z"
          ></path>
          <circle cx="7.5" cy="7.5" r=".5" fill="currentColor"></circle></svg
        >تصنيف كـ...<svg
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
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="destructive"
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
          class="lucide lucide-trash2"
        >
          <path d="M3 6h18"></path>
          <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
          <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
          <line x1="10" x2="10" y1="11" y2="17"></line>
          <line x1="14" x2="14" y1="11" y2="17"></line></svg
        >سلة المهملات
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
    <div role="group" data-slot="dropdown-menu-radio-group">
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
        >شخصي
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
        >عمل
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
        >آخر
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


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="button-group"` |
| `data-slot="button-group-separator"` |

### ButtonGroup

The `ButtonGroup` component is a container that groups related buttons together with consistent styling.

| Prop          | Type                         | Default        |
| ------------- | ---------------------------- | -------------- |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` |

```tsx
<ButtonGroup>
  <Button>Button 1</Button>
  <Button>Button 2</Button>
</ButtonGroup>
```

Nest multiple button groups to create complex layouts with spacing. See the [nested](#nested) example for more details.

```tsx
<ButtonGroup>
  <ButtonGroup />
  <ButtonGroup />
</ButtonGroup>
```

### ButtonGroupSeparator

The `ButtonGroupSeparator` component visually divides buttons within a group.

| Prop          | Type                         | Default      |
| ------------- | ---------------------------- | ------------ |
| `orientation` | `"horizontal" \| "vertical"` | `"vertical"` |

```tsx
<ButtonGroup>
  <Button>Button 1</Button>
  <ButtonGroupSeparator />
  <Button>Button 2</Button>
</ButtonGroup>
```

### ButtonGroupText

Use this component to display text within a button group.

| Prop      | Type      | Default |
| --------- | --------- | ------- |
| `asChild` | `boolean` | `false` |

```tsx
<ButtonGroup>
  <ButtonGroupText>Text</ButtonGroupText>
  <Button>Button</Button>
</ButtonGroup>
```

Use the `asChild` prop to render a custom component as the text, for example a label.

```tsx showLineNumbers

export function ButtonGroupTextDemo() {
  return (
    <ButtonGroup>
      <ButtonGroupText asChild>
        <Label htmlFor="name">Text</Label>
      </ButtonGroupText>
      <Input placeholder="Type something here..." id="name" />
    </ButtonGroup>
  )
}
```
