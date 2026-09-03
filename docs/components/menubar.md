---
title: "Menubar"
description: "A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands."
---

# Menubar

A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/menubar" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/menubar#api-reference" rel="noopener">api</a></p>

::::demo menubar-demo
<iframe class="demo" src="/demos/menubar-demo.html" title="menubar-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/menubar-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [menubar-demo.html]
<div
  role="menubar"
  data-slot="menubar"
  class="h-8 gap-0.5 rounded-lg border p-[3px] flex items-center w-72"
  tabindex="0"
  data-orientation="horizontal"
  style="outline: none"
>
  <button
    type="button"
    role="menuitem"
    id="m0-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m0"
  >
    File</button
  ><button
    type="button"
    role="menuitem"
    id="m1-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m1"
  >
    Edit</button
  ><button
    type="button"
    role="menuitem"
    id="m2-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m2"
  >
    View</button
  ><button
    type="button"
    role="menuitem"
    id="m3-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="0"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-highlighted=""
    data-radixuigo-menu-trigger="m3"
  >
    Profiles
  </button>
</div>
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
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        New Tab
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
          >⌘T</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        New Window
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
          >⌘N</span
        >
      </div>
      <div
        role="menuitem"
        aria-disabled="true"
        data-disabled=""
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        New Incognito Window
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        id="m0s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s0"
        data-state="closed"
        data-radix-menubar-subtrigger=""
        data-slot="menubar-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-none select-none"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s0"
      >
        Share<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right cn-rtl-flip ml-auto size-4"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Print...
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
          >⌘P</span
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
    data-radix-menubar-content=""
    data-slot="menubar-sub-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-32 rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Email link
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Messages
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Notes
      </div>
    </div>
  </div>
</template>
<template id="m1-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m1"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Undo
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
          >⌘Z</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Redo
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
          >⇧⌘Z</span
        >
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        id="m1s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m1s0"
        data-state="closed"
        data-radix-menubar-subtrigger=""
        data-slot="menubar-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-none select-none"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m1s0"
      >
        Find<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right cn-rtl-flip ml-auto size-4"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Cut
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Copy
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Paste
      </div>
    </div>
  </div>
</template>
<template id="m1s0-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m1s0"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-sub-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-32 rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Search the web
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Find...
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Find Next
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Find Previous
      </div>
    </div>
  </div>
</template>
<template id="m2-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m2"
    aria-labelledby="m2-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden w-44"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitemcheckbox"
        aria-checked="false"
        data-slot="menubar-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-inset:pl-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Bookmarks Bar
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="menubar-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-inset:pl-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
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
        >Full URLs
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Reload
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
          >⌘R</span
        >
      </div>
      <div
        role="menuitem"
        aria-disabled="true"
        data-disabled=""
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Force Reload
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
          >⇧⌘R</span
        >
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Toggle Fullscreen
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Hide Sidebar
      </div>
    </div>
  </div>
</template>
<template id="m3-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m3"
    aria-labelledby="m3-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-radio-group">
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Andy
      </div>
      <div
        role="menuitemradio"
        aria-checked="true"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
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
        >Benoit
      </div>
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Luis
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Edit...
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Add Profile...
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/menubar.js
// generated: shadless menubar behavior — wireMenu + the menubar-specific layer
// (roving cursor between triggers, horizontal arrows switching between open
// menus, Enter/Space opening). wireMenu already covers click toggling,
// vertical item navigation, submenus, Escape/outside-close and trigger
// refocus.
//
// Measured radix semantics (oracle-driven):
//   - tabindex=0 rides the cursor trigger (root keeps tabindex=0 until one
//     is focused); data-highlighted appears on a trigger only when it holds
//     DOM focus (keyboard focus, post-Escape refocus) — not while its menu
//     is merely open with focus inside the content.
//   - closed ArrowLeft/Right/Home/End move focus between triggers, no
//     auto-open.
//   - open ArrowLeft/Right switch menus: close silently, open neighbor
//     WITHOUT first-item highlight (focus lands on the content, like a
//     click-open).
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menubar"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menubar"] = true
  shadless.register("menubar", { init: function () {
    if (shadless.__menuWired.installed_menubar) return
    shadless.__menuWired.installed_menubar = true
    // menubar.tsx wrapper defaults (align is hardcoded "start" in the kernel)
    var POSITION = { side: "bottom", sideOffset: 8, alignOffset: -4 };

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
      if (prev) shadless.h.emit(prev, "close", "menubar");
      if (t) shadless.h.emit(t, "open", "menubar");
    };
    var handles = RadixKernel.wireMenu({
      // triggers must stay out of the background aria-hidden set
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger");
      },
      onAllClosed: function () { sync(); },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });

    var triggers = function () {
      return Array.prototype.slice.call(document.querySelectorAll("[data-radixuigo-menu-trigger]"));
    };
    var idOf = function (t) { return t.getAttribute("data-radixuigo-menu-trigger"); };
    var setRoving = function (active) {
      triggers().forEach(function (t) { t.setAttribute("tabindex", t === active ? "0" : "-1"); });
    };
    // radix highlights the FOCUSED trigger (not merely the open one)
    var setCursor = function (active) {
      triggers().forEach(function (t) {
        t.setAttribute("tabindex", t === active ? "0" : "-1");
        if (t === active) t.setAttribute("data-highlighted", "")
        else t.removeAttribute("data-highlighted")
      });
    };
    // opening a menu moves focus into the content — any highlight left by the
    // trigger's mousedown focusin must go (measured: open ⇒ no trigger
    // highlight, even for the trigger whose menu is open)
    var clearHighlights = function () {
      triggers().forEach(function (t) { t.removeAttribute("data-highlighted") });
    };
    var switchMenu = function (dir) {
      var ts = triggers();
      var root = handles.rootLayer();
      var cur = root ? ts.indexOf(root.trigger) : -1;
      if (cur < 0) return;
      var next = ts[(cur + dir + ts.length) % ts.length];
      handles.closeAll(true);
      setRoving(next);
      handles.openMenu(idOf(next), next, POSITION);
      sync();
    };

    triggers().forEach(function (t) {
      var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
      shadless.instances.set(t, { component: "menubar",
        open: function () { if (!isOpen()) { setRoving(t); handles.openMenu(idOf(t), t, POSITION); sync() } },
        close: function () { if (isOpen()) { handles.closeAll(); sync() } },
        toggle: function () { setRoving(t); handles.toggleMenu(idOf(t), t, POSITION); sync() },
        isOpen: isOpen,
      })
    })
    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) {
        e.preventDefault();
        setRoving(trig);
        handles.toggleMenu(idOf(trig), trig, POSITION);
        if (handles.depth() > 0) clearHighlights();
        sync();
        return;
      }
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });

    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel opens it only on click / ArrowRight — route pointer entry through
    // the click path (idempotent for an open layer)
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });

    document.addEventListener("keydown", function (e) {
      var key = e.key;
      if (handles.depth() > 0) {
        // open: wireMenu first (items/submenus/Escape/Tab); whatever it does
        // not claim falls through to menubar-level horizontal switching
        var acted = false;
        handles.onKeydown(e.target, key, function () { acted = true; });
        sync();
        if (acted) return;
        if (key === "ArrowRight") { e.preventDefault(); switchMenu(1); }
        else if (key === "ArrowLeft") { e.preventDefault(); switchMenu(-1); }
        return;
      }
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (!trig) { handles.onKeydown(e.target, key, function () {}); return; }
      var ts = triggers();
      var i = ts.indexOf(trig);
      if (key === "ArrowRight" || key === "ArrowLeft" || key === "Home" || key === "End") {
        e.preventDefault();
        var next = key === "Home" ? ts[0]
          : key === "End" ? ts[ts.length - 1]
          : ts[(i + (key === "ArrowRight" ? 1 : ts.length - 1)) % ts.length];
        setCursor(next);
        next.focus();
        return;
      }
      if (key === "Enter" || key === " " || key === "ArrowDown") {
        e.preventDefault();
        setRoving(trig);
        handles.openMenu(idOf(trig), trig, Object.assign({ keyboard: true }, POSITION));
        clearHighlights();
        sync();
        return;
      }
      handles.onKeydown(e.target, key, function () { e.preventDefault(); });
      sync();
    });

    // focus landing on a trigger (kernel refocus after close, Tab, etc.) makes
    // it the cursor: tabindex=0 + data-highlighted
    document.addEventListener("focusin", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) setCursor(trig);
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
@import "shadless/menubar.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/menubar.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/menubar.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/menubar.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/menubar.js"></script>
```

**Copy the markup from                                into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="menubar-trigger" id="<k>-trigger" data-radixuigo-menu-trigger="<k>">` | opens on clicking |
| `<template id="<k>-tpl">` | holds the `data-slot="menubar-content"` subtree |
| `<… data-slot="menubar-sub-trigger" id="<k>s0-trigger" data-radixuigo-menu-subtrigger="<k>s0">` | a sub menu inside a layer; its own `<template id="<k>s0-tpl">` |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/menubar.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
menubar
├── menubar-menu
│   ├── menubar-trigger
│   └── menubar-content
│       ├── menubar-group
│       │   ├── menubar-label
│       │   ├── menubar-item
│       │   └── menubar-item
│       ├── menubar-separator
│       ├── menubar-group
│       │   ├── menubar-label
│       │   ├── menubar-checkbox-item
│       │   └── menubar-checkbox-item
│       ├── menubar-separator
│       ├── menubar-group
│       │   ├── menubar-label
│       │   └── menubar-radio-group
│       │       ├── menubar-radio-item
│       │       └── menubar-radio-item
│       └── menubar-sub
│           ├── menubar-sub-trigger
│           └── menubar-sub-content
│               └── menubar-group
│                   ├── menubar-label
│                   ├── menubar-item
│                   └── menubar-item
└── menubar-menu
    ├── menubar-trigger
    └── menubar-content
        └── menubar-group
            ├── menubar-label
            ├── menubar-item
            └── menubar-item
```

## Checkbox

Use `MenubarCheckboxItem` for toggleable options.

::::demo menubar-checkbox
<iframe class="demo" src="/demos/menubar-checkbox.html" title="menubar-checkbox" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/menubar-checkbox.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [menubar-checkbox.html]
<div
  role="menubar"
  data-slot="menubar"
  class="h-8 gap-0.5 rounded-lg border p-[3px] flex items-center w-72"
  tabindex="0"
  data-orientation="horizontal"
  style="outline: none"
>
  <button
    type="button"
    role="menuitem"
    id="m0-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m0"
  >
    View</button
  ><button
    type="button"
    role="menuitem"
    id="m1-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="0"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-highlighted=""
    data-radixuigo-menu-trigger="m1"
  >
    Format
  </button>
</div>
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
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden w-64"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitemcheckbox"
      aria-checked="false"
      data-slot="menubar-checkbox-item"
      class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-inset:pl-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      data-state="unchecked"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      <span
        class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
      ></span
      >Always Show Bookmarks Bar
    </div>
    <div
      role="menuitemcheckbox"
      aria-checked="true"
      data-slot="menubar-checkbox-item"
      class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-inset:pl-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      data-state="checked"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      <span
        class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
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
      >Always Show Full URLs
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-inset="true"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Reload
      <span
        data-slot="menubar-shortcut"
        class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
        >⌘R</span
      >
    </div>
    <div
      role="menuitem"
      aria-disabled="true"
      data-disabled=""
      data-slot="menubar-item"
      data-inset="true"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Force Reload
      <span
        data-slot="menubar-shortcut"
        class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
        >⇧⌘R</span
      >
    </div>
  </div>
</template>
<template id="m1-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m1"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitemcheckbox"
      aria-checked="true"
      data-slot="menubar-checkbox-item"
      class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-inset:pl-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      data-state="checked"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      <span
        class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
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
      >Strikethrough
    </div>
    <div
      role="menuitemcheckbox"
      aria-checked="false"
      data-slot="menubar-checkbox-item"
      class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-inset:pl-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      data-state="unchecked"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      <span
        class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
      ></span
      >Code
    </div>
    <div
      role="menuitemcheckbox"
      aria-checked="false"
      data-slot="menubar-checkbox-item"
      class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-inset:pl-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      data-state="unchecked"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      <span
        class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
      ></span
      >Superscript
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/menubar.js
// generated: shadless menubar behavior — wireMenu + the menubar-specific layer
// (roving cursor between triggers, horizontal arrows switching between open
// menus, Enter/Space opening). wireMenu already covers click toggling,
// vertical item navigation, submenus, Escape/outside-close and trigger
// refocus.
//
// Measured radix semantics (oracle-driven):
//   - tabindex=0 rides the cursor trigger (root keeps tabindex=0 until one
//     is focused); data-highlighted appears on a trigger only when it holds
//     DOM focus (keyboard focus, post-Escape refocus) — not while its menu
//     is merely open with focus inside the content.
//   - closed ArrowLeft/Right/Home/End move focus between triggers, no
//     auto-open.
//   - open ArrowLeft/Right switch menus: close silently, open neighbor
//     WITHOUT first-item highlight (focus lands on the content, like a
//     click-open).
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menubar"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menubar"] = true
  shadless.register("menubar", { init: function () {
    if (shadless.__menuWired.installed_menubar) return
    shadless.__menuWired.installed_menubar = true
    // menubar.tsx wrapper defaults (align is hardcoded "start" in the kernel)
    var POSITION = { side: "bottom", sideOffset: 8, alignOffset: -4 };

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
      if (prev) shadless.h.emit(prev, "close", "menubar");
      if (t) shadless.h.emit(t, "open", "menubar");
    };
    var handles = RadixKernel.wireMenu({
      // triggers must stay out of the background aria-hidden set
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger");
      },
      onAllClosed: function () { sync(); },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });

    var triggers = function () {
      return Array.prototype.slice.call(document.querySelectorAll("[data-radixuigo-menu-trigger]"));
    };
    var idOf = function (t) { return t.getAttribute("data-radixuigo-menu-trigger"); };
    var setRoving = function (active) {
      triggers().forEach(function (t) { t.setAttribute("tabindex", t === active ? "0" : "-1"); });
    };
    // radix highlights the FOCUSED trigger (not merely the open one)
    var setCursor = function (active) {
      triggers().forEach(function (t) {
        t.setAttribute("tabindex", t === active ? "0" : "-1");
        if (t === active) t.setAttribute("data-highlighted", "")
        else t.removeAttribute("data-highlighted")
      });
    };
    // opening a menu moves focus into the content — any highlight left by the
    // trigger's mousedown focusin must go (measured: open ⇒ no trigger
    // highlight, even for the trigger whose menu is open)
    var clearHighlights = function () {
      triggers().forEach(function (t) { t.removeAttribute("data-highlighted") });
    };
    var switchMenu = function (dir) {
      var ts = triggers();
      var root = handles.rootLayer();
      var cur = root ? ts.indexOf(root.trigger) : -1;
      if (cur < 0) return;
      var next = ts[(cur + dir + ts.length) % ts.length];
      handles.closeAll(true);
      setRoving(next);
      handles.openMenu(idOf(next), next, POSITION);
      sync();
    };

    triggers().forEach(function (t) {
      var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
      shadless.instances.set(t, { component: "menubar",
        open: function () { if (!isOpen()) { setRoving(t); handles.openMenu(idOf(t), t, POSITION); sync() } },
        close: function () { if (isOpen()) { handles.closeAll(); sync() } },
        toggle: function () { setRoving(t); handles.toggleMenu(idOf(t), t, POSITION); sync() },
        isOpen: isOpen,
      })
    })
    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) {
        e.preventDefault();
        setRoving(trig);
        handles.toggleMenu(idOf(trig), trig, POSITION);
        if (handles.depth() > 0) clearHighlights();
        sync();
        return;
      }
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });

    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel opens it only on click / ArrowRight — route pointer entry through
    // the click path (idempotent for an open layer)
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });

    document.addEventListener("keydown", function (e) {
      var key = e.key;
      if (handles.depth() > 0) {
        // open: wireMenu first (items/submenus/Escape/Tab); whatever it does
        // not claim falls through to menubar-level horizontal switching
        var acted = false;
        handles.onKeydown(e.target, key, function () { acted = true; });
        sync();
        if (acted) return;
        if (key === "ArrowRight") { e.preventDefault(); switchMenu(1); }
        else if (key === "ArrowLeft") { e.preventDefault(); switchMenu(-1); }
        return;
      }
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (!trig) { handles.onKeydown(e.target, key, function () {}); return; }
      var ts = triggers();
      var i = ts.indexOf(trig);
      if (key === "ArrowRight" || key === "ArrowLeft" || key === "Home" || key === "End") {
        e.preventDefault();
        var next = key === "Home" ? ts[0]
          : key === "End" ? ts[ts.length - 1]
          : ts[(i + (key === "ArrowRight" ? 1 : ts.length - 1)) % ts.length];
        setCursor(next);
        next.focus();
        return;
      }
      if (key === "Enter" || key === " " || key === "ArrowDown") {
        e.preventDefault();
        setRoving(trig);
        handles.openMenu(idOf(trig), trig, Object.assign({ keyboard: true }, POSITION));
        clearHighlights();
        sync();
        return;
      }
      handles.onKeydown(e.target, key, function () { e.preventDefault(); });
      sync();
    });

    // focus landing on a trigger (kernel refocus after close, Tab, etc.) makes
    // it the cursor: tabindex=0 + data-highlighted
    document.addEventListener("focusin", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) setCursor(trig);
    });
  } })
})()
```
:::

::::


## Radio

Use `MenubarRadioGroup` and `MenubarRadioItem` for single-select options.

::::demo menubar-radio
<iframe class="demo" src="/demos/menubar-radio.html" title="menubar-radio" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/menubar-radio.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [menubar-radio.html]
<div
  role="menubar"
  data-slot="menubar"
  class="h-8 gap-0.5 rounded-lg border p-[3px] flex items-center w-72"
  tabindex="0"
  data-orientation="horizontal"
  style="outline: none"
>
  <button
    type="button"
    role="menuitem"
    id="m0-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m0"
  >
    Profiles</button
  ><button
    type="button"
    role="menuitem"
    id="m1-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="0"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-highlighted=""
    data-radixuigo-menu-trigger="m1"
  >
    Theme
  </button>
</div>
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
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-radio-group">
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Andy
      </div>
      <div
        role="menuitemradio"
        aria-checked="true"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
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
        >Benoit
      </div>
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Luis
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-inset="true"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Edit...
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-inset="true"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Add Profile...
    </div>
  </div>
</template>
<template id="m1-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m1"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-radio-group">
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Light
      </div>
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Dark
      </div>
      <div
        role="menuitemradio"
        aria-checked="true"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="left-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
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
        >System
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/menubar.js
// generated: shadless menubar behavior — wireMenu + the menubar-specific layer
// (roving cursor between triggers, horizontal arrows switching between open
// menus, Enter/Space opening). wireMenu already covers click toggling,
// vertical item navigation, submenus, Escape/outside-close and trigger
// refocus.
//
// Measured radix semantics (oracle-driven):
//   - tabindex=0 rides the cursor trigger (root keeps tabindex=0 until one
//     is focused); data-highlighted appears on a trigger only when it holds
//     DOM focus (keyboard focus, post-Escape refocus) — not while its menu
//     is merely open with focus inside the content.
//   - closed ArrowLeft/Right/Home/End move focus between triggers, no
//     auto-open.
//   - open ArrowLeft/Right switch menus: close silently, open neighbor
//     WITHOUT first-item highlight (focus lands on the content, like a
//     click-open).
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menubar"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menubar"] = true
  shadless.register("menubar", { init: function () {
    if (shadless.__menuWired.installed_menubar) return
    shadless.__menuWired.installed_menubar = true
    // menubar.tsx wrapper defaults (align is hardcoded "start" in the kernel)
    var POSITION = { side: "bottom", sideOffset: 8, alignOffset: -4 };

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
      if (prev) shadless.h.emit(prev, "close", "menubar");
      if (t) shadless.h.emit(t, "open", "menubar");
    };
    var handles = RadixKernel.wireMenu({
      // triggers must stay out of the background aria-hidden set
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger");
      },
      onAllClosed: function () { sync(); },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });

    var triggers = function () {
      return Array.prototype.slice.call(document.querySelectorAll("[data-radixuigo-menu-trigger]"));
    };
    var idOf = function (t) { return t.getAttribute("data-radixuigo-menu-trigger"); };
    var setRoving = function (active) {
      triggers().forEach(function (t) { t.setAttribute("tabindex", t === active ? "0" : "-1"); });
    };
    // radix highlights the FOCUSED trigger (not merely the open one)
    var setCursor = function (active) {
      triggers().forEach(function (t) {
        t.setAttribute("tabindex", t === active ? "0" : "-1");
        if (t === active) t.setAttribute("data-highlighted", "")
        else t.removeAttribute("data-highlighted")
      });
    };
    // opening a menu moves focus into the content — any highlight left by the
    // trigger's mousedown focusin must go (measured: open ⇒ no trigger
    // highlight, even for the trigger whose menu is open)
    var clearHighlights = function () {
      triggers().forEach(function (t) { t.removeAttribute("data-highlighted") });
    };
    var switchMenu = function (dir) {
      var ts = triggers();
      var root = handles.rootLayer();
      var cur = root ? ts.indexOf(root.trigger) : -1;
      if (cur < 0) return;
      var next = ts[(cur + dir + ts.length) % ts.length];
      handles.closeAll(true);
      setRoving(next);
      handles.openMenu(idOf(next), next, POSITION);
      sync();
    };

    triggers().forEach(function (t) {
      var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
      shadless.instances.set(t, { component: "menubar",
        open: function () { if (!isOpen()) { setRoving(t); handles.openMenu(idOf(t), t, POSITION); sync() } },
        close: function () { if (isOpen()) { handles.closeAll(); sync() } },
        toggle: function () { setRoving(t); handles.toggleMenu(idOf(t), t, POSITION); sync() },
        isOpen: isOpen,
      })
    })
    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) {
        e.preventDefault();
        setRoving(trig);
        handles.toggleMenu(idOf(trig), trig, POSITION);
        if (handles.depth() > 0) clearHighlights();
        sync();
        return;
      }
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });

    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel opens it only on click / ArrowRight — route pointer entry through
    // the click path (idempotent for an open layer)
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });

    document.addEventListener("keydown", function (e) {
      var key = e.key;
      if (handles.depth() > 0) {
        // open: wireMenu first (items/submenus/Escape/Tab); whatever it does
        // not claim falls through to menubar-level horizontal switching
        var acted = false;
        handles.onKeydown(e.target, key, function () { acted = true; });
        sync();
        if (acted) return;
        if (key === "ArrowRight") { e.preventDefault(); switchMenu(1); }
        else if (key === "ArrowLeft") { e.preventDefault(); switchMenu(-1); }
        return;
      }
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (!trig) { handles.onKeydown(e.target, key, function () {}); return; }
      var ts = triggers();
      var i = ts.indexOf(trig);
      if (key === "ArrowRight" || key === "ArrowLeft" || key === "Home" || key === "End") {
        e.preventDefault();
        var next = key === "Home" ? ts[0]
          : key === "End" ? ts[ts.length - 1]
          : ts[(i + (key === "ArrowRight" ? 1 : ts.length - 1)) % ts.length];
        setCursor(next);
        next.focus();
        return;
      }
      if (key === "Enter" || key === " " || key === "ArrowDown") {
        e.preventDefault();
        setRoving(trig);
        handles.openMenu(idOf(trig), trig, Object.assign({ keyboard: true }, POSITION));
        clearHighlights();
        sync();
        return;
      }
      handles.onKeydown(e.target, key, function () { e.preventDefault(); });
      sync();
    });

    // focus landing on a trigger (kernel refocus after close, Tab, etc.) makes
    // it the cursor: tabindex=0 + data-highlighted
    document.addEventListener("focusin", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) setCursor(trig);
    });
  } })
})()
```
:::

::::


## Submenu

Use `MenubarSub`, `MenubarSubTrigger`, and `MenubarSubContent` for nested menus.

::::demo menubar-submenu
<iframe class="demo" src="/demos/menubar-submenu.html" title="menubar-submenu" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/menubar-submenu.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [menubar-submenu.html]
<div
  role="menubar"
  data-slot="menubar"
  class="h-8 gap-0.5 rounded-lg border p-[3px] flex items-center w-72"
  tabindex="0"
  data-orientation="horizontal"
  style="outline: none"
>
  <button
    type="button"
    role="menuitem"
    id="m0-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m0"
  >
    File</button
  ><button
    type="button"
    role="menuitem"
    id="m1-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="0"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-highlighted=""
    data-radixuigo-menu-trigger="m1"
  >
    Edit
  </button>
</div>
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
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      id="m0s0-trigger"
      aria-haspopup="menu"
      aria-expanded="false"
      aria-controls="m0s0"
      data-state="closed"
      data-radix-menubar-subtrigger=""
      data-slot="menubar-sub-trigger"
      class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-none select-none"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
      data-radixuigo-menu-subtrigger="m0s0"
    >
      Share<svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="lucide lucide-chevron-right cn-rtl-flip ml-auto size-4"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Print...
      <span
        data-slot="menubar-shortcut"
        class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
        >⌘P</span
      >
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
    data-radix-menubar-content=""
    data-slot="menubar-sub-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-32 rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Email link
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Messages
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Notes
    </div>
  </div>
</template>
<template id="m1-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m1"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Undo
      <span
        data-slot="menubar-shortcut"
        class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
        >⌘Z</span
      >
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Redo
      <span
        data-slot="menubar-shortcut"
        class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
        >⇧⌘Z</span
      >
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      id="m1s0-trigger"
      aria-haspopup="menu"
      aria-expanded="false"
      aria-controls="m1s0"
      data-state="closed"
      data-radix-menubar-subtrigger=""
      data-slot="menubar-sub-trigger"
      class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-none select-none"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
      data-radixuigo-menu-subtrigger="m1s0"
    >
      Find<svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="lucide lucide-chevron-right cn-rtl-flip ml-auto size-4"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Cut
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Copy
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Paste
    </div>
  </div>
</template>
<template id="m1s0-tpl">
  <div
    data-side="right"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m1s0"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-sub-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-32 rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Find...
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Find Next
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Find Previous
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/menubar.js
// generated: shadless menubar behavior — wireMenu + the menubar-specific layer
// (roving cursor between triggers, horizontal arrows switching between open
// menus, Enter/Space opening). wireMenu already covers click toggling,
// vertical item navigation, submenus, Escape/outside-close and trigger
// refocus.
//
// Measured radix semantics (oracle-driven):
//   - tabindex=0 rides the cursor trigger (root keeps tabindex=0 until one
//     is focused); data-highlighted appears on a trigger only when it holds
//     DOM focus (keyboard focus, post-Escape refocus) — not while its menu
//     is merely open with focus inside the content.
//   - closed ArrowLeft/Right/Home/End move focus between triggers, no
//     auto-open.
//   - open ArrowLeft/Right switch menus: close silently, open neighbor
//     WITHOUT first-item highlight (focus lands on the content, like a
//     click-open).
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menubar"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menubar"] = true
  shadless.register("menubar", { init: function () {
    if (shadless.__menuWired.installed_menubar) return
    shadless.__menuWired.installed_menubar = true
    // menubar.tsx wrapper defaults (align is hardcoded "start" in the kernel)
    var POSITION = { side: "bottom", sideOffset: 8, alignOffset: -4 };

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
      if (prev) shadless.h.emit(prev, "close", "menubar");
      if (t) shadless.h.emit(t, "open", "menubar");
    };
    var handles = RadixKernel.wireMenu({
      // triggers must stay out of the background aria-hidden set
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger");
      },
      onAllClosed: function () { sync(); },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });

    var triggers = function () {
      return Array.prototype.slice.call(document.querySelectorAll("[data-radixuigo-menu-trigger]"));
    };
    var idOf = function (t) { return t.getAttribute("data-radixuigo-menu-trigger"); };
    var setRoving = function (active) {
      triggers().forEach(function (t) { t.setAttribute("tabindex", t === active ? "0" : "-1"); });
    };
    // radix highlights the FOCUSED trigger (not merely the open one)
    var setCursor = function (active) {
      triggers().forEach(function (t) {
        t.setAttribute("tabindex", t === active ? "0" : "-1");
        if (t === active) t.setAttribute("data-highlighted", "")
        else t.removeAttribute("data-highlighted")
      });
    };
    // opening a menu moves focus into the content — any highlight left by the
    // trigger's mousedown focusin must go (measured: open ⇒ no trigger
    // highlight, even for the trigger whose menu is open)
    var clearHighlights = function () {
      triggers().forEach(function (t) { t.removeAttribute("data-highlighted") });
    };
    var switchMenu = function (dir) {
      var ts = triggers();
      var root = handles.rootLayer();
      var cur = root ? ts.indexOf(root.trigger) : -1;
      if (cur < 0) return;
      var next = ts[(cur + dir + ts.length) % ts.length];
      handles.closeAll(true);
      setRoving(next);
      handles.openMenu(idOf(next), next, POSITION);
      sync();
    };

    triggers().forEach(function (t) {
      var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
      shadless.instances.set(t, { component: "menubar",
        open: function () { if (!isOpen()) { setRoving(t); handles.openMenu(idOf(t), t, POSITION); sync() } },
        close: function () { if (isOpen()) { handles.closeAll(); sync() } },
        toggle: function () { setRoving(t); handles.toggleMenu(idOf(t), t, POSITION); sync() },
        isOpen: isOpen,
      })
    })
    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) {
        e.preventDefault();
        setRoving(trig);
        handles.toggleMenu(idOf(trig), trig, POSITION);
        if (handles.depth() > 0) clearHighlights();
        sync();
        return;
      }
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });

    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel opens it only on click / ArrowRight — route pointer entry through
    // the click path (idempotent for an open layer)
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });

    document.addEventListener("keydown", function (e) {
      var key = e.key;
      if (handles.depth() > 0) {
        // open: wireMenu first (items/submenus/Escape/Tab); whatever it does
        // not claim falls through to menubar-level horizontal switching
        var acted = false;
        handles.onKeydown(e.target, key, function () { acted = true; });
        sync();
        if (acted) return;
        if (key === "ArrowRight") { e.preventDefault(); switchMenu(1); }
        else if (key === "ArrowLeft") { e.preventDefault(); switchMenu(-1); }
        return;
      }
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (!trig) { handles.onKeydown(e.target, key, function () {}); return; }
      var ts = triggers();
      var i = ts.indexOf(trig);
      if (key === "ArrowRight" || key === "ArrowLeft" || key === "Home" || key === "End") {
        e.preventDefault();
        var next = key === "Home" ? ts[0]
          : key === "End" ? ts[ts.length - 1]
          : ts[(i + (key === "ArrowRight" ? 1 : ts.length - 1)) % ts.length];
        setCursor(next);
        next.focus();
        return;
      }
      if (key === "Enter" || key === " " || key === "ArrowDown") {
        e.preventDefault();
        setRoving(trig);
        handles.openMenu(idOf(trig), trig, Object.assign({ keyboard: true }, POSITION));
        clearHighlights();
        sync();
        return;
      }
      handles.onKeydown(e.target, key, function () { e.preventDefault(); });
      sync();
    });

    // focus landing on a trigger (kernel refocus after close, Tab, etc.) makes
    // it the cursor: tabindex=0 + data-highlighted
    document.addEventListener("focusin", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) setCursor(trig);
    });
  } })
})()
```
:::

::::


## With Icons

::::demo menubar-icons
<iframe class="demo" src="/demos/menubar-icons.html" title="menubar-icons" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/menubar-icons.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [menubar-icons.html]
<div
  role="menubar"
  data-slot="menubar"
  class="h-8 gap-0.5 rounded-lg border p-[3px] flex items-center w-72"
  tabindex="0"
  data-orientation="horizontal"
  style="outline: none"
>
  <button
    type="button"
    role="menuitem"
    id="m0-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m0"
  >
    File</button
  ><button
    type="button"
    role="menuitem"
    id="m1-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="0"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-highlighted=""
    data-radixuigo-menu-trigger="m1"
  >
    More
  </button>
</div>
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
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
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
      >New File
      <span
        data-slot="menubar-shortcut"
        class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
        >⌘N</span
      >
    </div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
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
      >Open Folder
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="menubar-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
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
      >Save
      <span
        data-slot="menubar-shortcut"
        class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto"
        >⌘S</span
      >
    </div>
  </div>
</template>
<template id="m1-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m1"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
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
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
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
        >Help
      </div>
      <div
        role="separator"
        aria-orientation="horizontal"
        data-slot="menubar-separator"
        class="bg-border -mx-1 my-1 h-px"
      ></div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="destructive"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
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

// js/menubar.js
// generated: shadless menubar behavior — wireMenu + the menubar-specific layer
// (roving cursor between triggers, horizontal arrows switching between open
// menus, Enter/Space opening). wireMenu already covers click toggling,
// vertical item navigation, submenus, Escape/outside-close and trigger
// refocus.
//
// Measured radix semantics (oracle-driven):
//   - tabindex=0 rides the cursor trigger (root keeps tabindex=0 until one
//     is focused); data-highlighted appears on a trigger only when it holds
//     DOM focus (keyboard focus, post-Escape refocus) — not while its menu
//     is merely open with focus inside the content.
//   - closed ArrowLeft/Right/Home/End move focus between triggers, no
//     auto-open.
//   - open ArrowLeft/Right switch menus: close silently, open neighbor
//     WITHOUT first-item highlight (focus lands on the content, like a
//     click-open).
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menubar"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menubar"] = true
  shadless.register("menubar", { init: function () {
    if (shadless.__menuWired.installed_menubar) return
    shadless.__menuWired.installed_menubar = true
    // menubar.tsx wrapper defaults (align is hardcoded "start" in the kernel)
    var POSITION = { side: "bottom", sideOffset: 8, alignOffset: -4 };

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
      if (prev) shadless.h.emit(prev, "close", "menubar");
      if (t) shadless.h.emit(t, "open", "menubar");
    };
    var handles = RadixKernel.wireMenu({
      // triggers must stay out of the background aria-hidden set
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger");
      },
      onAllClosed: function () { sync(); },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });

    var triggers = function () {
      return Array.prototype.slice.call(document.querySelectorAll("[data-radixuigo-menu-trigger]"));
    };
    var idOf = function (t) { return t.getAttribute("data-radixuigo-menu-trigger"); };
    var setRoving = function (active) {
      triggers().forEach(function (t) { t.setAttribute("tabindex", t === active ? "0" : "-1"); });
    };
    // radix highlights the FOCUSED trigger (not merely the open one)
    var setCursor = function (active) {
      triggers().forEach(function (t) {
        t.setAttribute("tabindex", t === active ? "0" : "-1");
        if (t === active) t.setAttribute("data-highlighted", "")
        else t.removeAttribute("data-highlighted")
      });
    };
    // opening a menu moves focus into the content — any highlight left by the
    // trigger's mousedown focusin must go (measured: open ⇒ no trigger
    // highlight, even for the trigger whose menu is open)
    var clearHighlights = function () {
      triggers().forEach(function (t) { t.removeAttribute("data-highlighted") });
    };
    var switchMenu = function (dir) {
      var ts = triggers();
      var root = handles.rootLayer();
      var cur = root ? ts.indexOf(root.trigger) : -1;
      if (cur < 0) return;
      var next = ts[(cur + dir + ts.length) % ts.length];
      handles.closeAll(true);
      setRoving(next);
      handles.openMenu(idOf(next), next, POSITION);
      sync();
    };

    triggers().forEach(function (t) {
      var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
      shadless.instances.set(t, { component: "menubar",
        open: function () { if (!isOpen()) { setRoving(t); handles.openMenu(idOf(t), t, POSITION); sync() } },
        close: function () { if (isOpen()) { handles.closeAll(); sync() } },
        toggle: function () { setRoving(t); handles.toggleMenu(idOf(t), t, POSITION); sync() },
        isOpen: isOpen,
      })
    })
    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) {
        e.preventDefault();
        setRoving(trig);
        handles.toggleMenu(idOf(trig), trig, POSITION);
        if (handles.depth() > 0) clearHighlights();
        sync();
        return;
      }
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });

    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel opens it only on click / ArrowRight — route pointer entry through
    // the click path (idempotent for an open layer)
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });

    document.addEventListener("keydown", function (e) {
      var key = e.key;
      if (handles.depth() > 0) {
        // open: wireMenu first (items/submenus/Escape/Tab); whatever it does
        // not claim falls through to menubar-level horizontal switching
        var acted = false;
        handles.onKeydown(e.target, key, function () { acted = true; });
        sync();
        if (acted) return;
        if (key === "ArrowRight") { e.preventDefault(); switchMenu(1); }
        else if (key === "ArrowLeft") { e.preventDefault(); switchMenu(-1); }
        return;
      }
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (!trig) { handles.onKeydown(e.target, key, function () {}); return; }
      var ts = triggers();
      var i = ts.indexOf(trig);
      if (key === "ArrowRight" || key === "ArrowLeft" || key === "Home" || key === "End") {
        e.preventDefault();
        var next = key === "Home" ? ts[0]
          : key === "End" ? ts[ts.length - 1]
          : ts[(i + (key === "ArrowRight" ? 1 : ts.length - 1)) % ts.length];
        setCursor(next);
        next.focus();
        return;
      }
      if (key === "Enter" || key === " " || key === "ArrowDown") {
        e.preventDefault();
        setRoving(trig);
        handles.openMenu(idOf(trig), trig, Object.assign({ keyboard: true }, POSITION));
        clearHighlights();
        sync();
        return;
      }
      handles.onKeydown(e.target, key, function () { e.preventDefault(); });
      sync();
    });

    // focus landing on a trigger (kernel refocus after close, Tab, etc.) makes
    // it the cursor: tabindex=0 + data-highlighted
    document.addEventListener("focusin", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) setCursor(trig);
    });
  } })
})()
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo menubar-rtl
<iframe class="demo" src="/demos/menubar-rtl.html" title="menubar-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/menubar-rtl.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [menubar-rtl.html]
<div
  role="menubar"
  data-slot="menubar"
  class="h-8 gap-0.5 rounded-lg border p-[3px] flex items-center w-72"
  tabindex="0"
  data-orientation="horizontal"
  style="outline: none"
>
  <button
    type="button"
    role="menuitem"
    id="m0-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m0"
  >
    ملف</button
  ><button
    type="button"
    role="menuitem"
    id="m1-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m1"
  >
    تعديل</button
  ><button
    type="button"
    role="menuitem"
    id="m2-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="-1"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-radixuigo-menu-trigger="m2"
  >
    عرض</button
  ><button
    type="button"
    role="menuitem"
    id="m3-trigger"
    aria-haspopup="menu"
    aria-expanded="false"
    data-state="closed"
    data-slot="menubar-trigger"
    class="hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none"
    tabindex="0"
    data-orientation="horizontal"
    data-radix-collection-item=""
    data-highlighted=""
    data-radixuigo-menu-trigger="m3"
  >
    الملفات الشخصية
  </button>
</div>
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
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        علامة تبويب جديدة
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ms-auto"
          >⌘T</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        نافذة جديدة
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ms-auto"
          >⌘N</span
        >
      </div>
      <div
        role="menuitem"
        aria-disabled="true"
        data-disabled=""
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        نافذة التصفح المتخفي الجديدة
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        id="m0s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m0s0"
        data-state="closed"
        data-radix-menubar-subtrigger=""
        data-slot="menubar-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-none select-none"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m0s0"
      >
        مشاركة<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right rtl:rotate-180 ms-auto size-4"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        طباعة...
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ms-auto"
          >⌘P</span
        >
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
    data-radix-menubar-content=""
    data-slot="menubar-sub-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-32 rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        رابط البريد الإلكتروني
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        الرسائل
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        الملاحظات
      </div>
    </div>
  </div>
</template>
<template id="m1-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m1"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        تراجع
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ms-auto"
          >⌘Z</span
        >
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        إعادة
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ms-auto"
          >⇧⌘Z</span
        >
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        id="m1s0-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-controls="m1s0"
        data-state="closed"
        data-radix-menubar-subtrigger=""
        data-slot="menubar-sub-trigger"
        class="focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-none select-none"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
        data-radixuigo-menu-subtrigger="m1s0"
      >
        بحث<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-right rtl:rotate-180 ms-auto size-4"
        >
          <path d="m9 18 6-6-6-6"></path>
        </svg>
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        قص
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        نسخ
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        لصق
      </div>
    </div>
  </div>
</template>
<template id="m1s0-tpl">
  <div
    data-side="left"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m1s0"
    aria-labelledby="m1-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-sub-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-32 rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        البحث على الويب
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        بحث...
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        البحث التالي
      </div>
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        البحث السابق
      </div>
    </div>
  </div>
</template>
<template id="m2-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m2"
    aria-labelledby="m2-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden w-44"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitemcheckbox"
        aria-checked="false"
        data-slot="menubar-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-1.5 ps-7 text-sm data-inset:ps-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="start-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >شريط الإشارات المرجعية
      </div>
      <div
        role="menuitemcheckbox"
        aria-checked="true"
        data-slot="menubar-checkbox-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-1.5 ps-7 text-sm data-inset:ps-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="start-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
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
        >عناوين URL الكاملة
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        إعادة تحميل
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ms-auto"
          >⌘R</span
        >
      </div>
      <div
        role="menuitem"
        aria-disabled="true"
        data-disabled=""
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        إعادة تحميل قسري
        <span
          data-slot="menubar-shortcut"
          class="text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ms-auto"
          >⇧⌘R</span
        >
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        تبديل وضع ملء الشاشة
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        إخفاء الشريط الجانبي
      </div>
    </div>
  </div>
</template>
<template id="m3-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="rtl"
    id="m3"
    aria-labelledby="m3-trigger"
    data-radix-menubar-content=""
    data-slot="menubar-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden"
    data-lang="ar"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-menubar-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-menubar-content-available-width: var(--radix-popper-available-width);
      --radix-menubar-content-available-height: var(--radix-popper-available-height);
      --radix-menubar-trigger-width: var(--radix-popper-anchor-width);
      --radix-menubar-trigger-height: var(--radix-popper-anchor-height);
    "
  >
    <div role="group" data-slot="menubar-radio-group">
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-1.5 ps-7 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="start-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Andy
      </div>
      <div
        role="menuitemradio"
        aria-checked="true"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-1.5 ps-7 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="start-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
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
        >Benoit
      </div>
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="menubar-radio-item"
        class="focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-1.5 ps-7 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span
          class="start-1.5 size-4 [&amp;_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center"
        ></span
        >Luis
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        تعديل...
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="menubar-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="menubar-group">
      <div
        role="menuitem"
        data-slot="menubar-item"
        data-inset="true"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:ps-7 [&amp;_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        إضافة ملف شخصي...
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/menubar.js
// generated: shadless menubar behavior — wireMenu + the menubar-specific layer
// (roving cursor between triggers, horizontal arrows switching between open
// menus, Enter/Space opening). wireMenu already covers click toggling,
// vertical item navigation, submenus, Escape/outside-close and trigger
// refocus.
//
// Measured radix semantics (oracle-driven):
//   - tabindex=0 rides the cursor trigger (root keeps tabindex=0 until one
//     is focused); data-highlighted appears on a trigger only when it holds
//     DOM focus (keyboard focus, post-Escape refocus) — not while its menu
//     is merely open with focus inside the content.
//   - closed ArrowLeft/Right/Home/End move focus between triggers, no
//     auto-open.
//   - open ArrowLeft/Right switch menus: close silently, open neighbor
//     WITHOUT first-item highlight (focus lands on the content, like a
//     click-open).
// Document-level: wireMenu owns every trigger on the page through its
// data-radixuigo-* protocol, so the behavior installs ONCE per page and
// ignores the root it is initialised with (a later shadless.init(subtree)
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menubar"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menubar"] = true
  shadless.register("menubar", { init: function () {
    if (shadless.__menuWired.installed_menubar) return
    shadless.__menuWired.installed_menubar = true
    // menubar.tsx wrapper defaults (align is hardcoded "start" in the kernel)
    var POSITION = { side: "bottom", sideOffset: 8, alignOffset: -4 };

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
      if (prev) shadless.h.emit(prev, "close", "menubar");
      if (t) shadless.h.emit(t, "open", "menubar");
    };
    var handles = RadixKernel.wireMenu({
      // triggers must stay out of the background aria-hidden set
      isPortalMarker: function (el) {
        return el.hasAttribute("data-radixuigo-menu-trigger");
      },
      onAllClosed: function () { sync(); },
      mountLayer: function (id) {
        var tpl = document.getElementById(id + "-tpl");
        if (!tpl) return null;
        var content = tpl.content.firstElementChild.cloneNode(true);
        var wrapper = document.createElement("div");
        wrapper.setAttribute("data-radix-popper-content-wrapper", "");
        wrapper.appendChild(content);
        document.body.appendChild(wrapper);
        content.style.setProperty("pointer-events", "auto");
        return {
          id: id,
          wrapper: wrapper,
          content: content,
          trigger: document.getElementById(id + "-trigger"),
        };
      },
    });

    var triggers = function () {
      return Array.prototype.slice.call(document.querySelectorAll("[data-radixuigo-menu-trigger]"));
    };
    var idOf = function (t) { return t.getAttribute("data-radixuigo-menu-trigger"); };
    var setRoving = function (active) {
      triggers().forEach(function (t) { t.setAttribute("tabindex", t === active ? "0" : "-1"); });
    };
    // radix highlights the FOCUSED trigger (not merely the open one)
    var setCursor = function (active) {
      triggers().forEach(function (t) {
        t.setAttribute("tabindex", t === active ? "0" : "-1");
        if (t === active) t.setAttribute("data-highlighted", "")
        else t.removeAttribute("data-highlighted")
      });
    };
    // opening a menu moves focus into the content — any highlight left by the
    // trigger's mousedown focusin must go (measured: open ⇒ no trigger
    // highlight, even for the trigger whose menu is open)
    var clearHighlights = function () {
      triggers().forEach(function (t) { t.removeAttribute("data-highlighted") });
    };
    var switchMenu = function (dir) {
      var ts = triggers();
      var root = handles.rootLayer();
      var cur = root ? ts.indexOf(root.trigger) : -1;
      if (cur < 0) return;
      var next = ts[(cur + dir + ts.length) % ts.length];
      handles.closeAll(true);
      setRoving(next);
      handles.openMenu(idOf(next), next, POSITION);
      sync();
    };

    triggers().forEach(function (t) {
      var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
      shadless.instances.set(t, { component: "menubar",
        open: function () { if (!isOpen()) { setRoving(t); handles.openMenu(idOf(t), t, POSITION); sync() } },
        close: function () { if (isOpen()) { handles.closeAll(); sync() } },
        toggle: function () { setRoving(t); handles.toggleMenu(idOf(t), t, POSITION); sync() },
        isOpen: isOpen,
      })
    })
    document.addEventListener("click", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) {
        e.preventDefault();
        setRoving(trig);
        handles.toggleMenu(idOf(trig), trig, POSITION);
        if (handles.depth() > 0) clearHighlights();
        sync();
        return;
      }
      handles.onDocumentClick(e.target, function () { e.preventDefault(); });
      sync();
    });

    // radix opens a sub menu when the pointer moves onto its trigger; the
    // kernel opens it only on click / ArrowRight — route pointer entry through
    // the click path (idempotent for an open layer)
    document.addEventListener("pointerover", function (e) {
      if (e.pointerType && e.pointerType !== "mouse") return;
      var sub = e.target.closest && e.target.closest("[data-radixuigo-menu-subtrigger]");
      if (sub && sub.getAttribute("data-state") !== "open") handles.onDocumentClick(sub, function () {});
    });

    document.addEventListener("keydown", function (e) {
      var key = e.key;
      if (handles.depth() > 0) {
        // open: wireMenu first (items/submenus/Escape/Tab); whatever it does
        // not claim falls through to menubar-level horizontal switching
        var acted = false;
        handles.onKeydown(e.target, key, function () { acted = true; });
        sync();
        if (acted) return;
        if (key === "ArrowRight") { e.preventDefault(); switchMenu(1); }
        else if (key === "ArrowLeft") { e.preventDefault(); switchMenu(-1); }
        return;
      }
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (!trig) { handles.onKeydown(e.target, key, function () {}); return; }
      var ts = triggers();
      var i = ts.indexOf(trig);
      if (key === "ArrowRight" || key === "ArrowLeft" || key === "Home" || key === "End") {
        e.preventDefault();
        var next = key === "Home" ? ts[0]
          : key === "End" ? ts[ts.length - 1]
          : ts[(i + (key === "ArrowRight" ? 1 : ts.length - 1)) % ts.length];
        setCursor(next);
        next.focus();
        return;
      }
      if (key === "Enter" || key === " " || key === "ArrowDown") {
        e.preventDefault();
        setRoving(trig);
        handles.openMenu(idOf(trig), trig, Object.assign({ keyboard: true }, POSITION));
        clearHighlights();
        sync();
        return;
      }
      handles.onKeydown(e.target, key, function () { e.preventDefault(); });
      sync();
    });

    // focus landing on a trigger (kernel refocus after close, Tab, etc.) makes
    // it the cursor: tabindex=0 + data-highlighted
    document.addEventListener("focusin", function (e) {
      var trig = e.target.closest && e.target.closest("[data-radixuigo-menu-trigger]");
      if (trig) setCursor(trig);
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
| `data-slot="menubar"` |
| `data-slot="menubar-menu"` |
| `data-slot="menubar-group"` |
| `data-slot="menubar-portal"` |
| `data-slot="menubar-radio-group"` |
| `data-slot="menubar-trigger"` |
| `data-slot="menubar-content"` |
| `data-slot="menubar-item"` |
| `data-slot="menubar-checkbox-item"` |
| `data-slot="menubar-radio-item"` |
| `data-slot="menubar-label"` |
| `data-slot="menubar-separator"` |
| `data-slot="menubar-shortcut"` |
| `data-slot="menubar-sub"` |
| `data-slot="menubar-sub-trigger"` |
| `data-slot="menubar-sub-content"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Menubar](https://www.radix-ui.com/docs/primitives/components/menubar#api-reference) documentation.
