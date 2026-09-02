---
title: "Avatar"
description: "An image element with a fallback for representing the user."
---

# Avatar

An image element with a fallback for representing the user.

<p class="page-links"><a href="https://www.radix-ui.com/primitives/docs/components/avatar" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/primitives/docs/components/avatar#api-reference" rel="noopener">api</a></p>

::::demo avatar-demo
<iframe class="demo" src="/demos/avatar-demo.html" title="avatar-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-demo.html]
<div class="flex flex-row flex-wrap items-center gap-6 md:gap-12">
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover grayscale"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png" /><span
      data-slot="avatar-badge"
      class="text-primary-foreground ring-background absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&amp;&gt;svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&amp;&gt;svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&amp;&gt;svg]:size-2 bg-green-600 dark:bg-green-800"
    ></span
  ></span>
  <div
    data-slot="avatar-group"
    class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
  >
    <span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@shadcn"
        src="https://github.com/shadcn.png" /></span
    ><span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@maxleiter"
        src="https://github.com/maxleiter.png" /></span
    ><span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@evilrabbit"
        src="https://github.com/evilrabbit.png"
    /></span>
    <div
      data-slot="avatar-group-count"
      class="bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&amp;&gt;svg]:size-4 group-has-data-[size=lg]/avatar-group:[&amp;&gt;svg]:size-5 group-has-data-[size=sm]/avatar-group:[&amp;&gt;svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background"
    >
      +3
    </div>
  </div>
</div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/avatar.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/avatar.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/avatar.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/avatar.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/avatar.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup.


No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/avatar.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
avatar
├── avatar-image
├── avatar-fallback
└── avatar-badge
```

## Basic

A basic avatar component with an image and a fallback.

::::demo avatar-basic
<iframe class="demo" src="/demos/avatar-basic.html" title="avatar-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-basic.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-basic.html]
<span
  data-slot="avatar"
  data-size="default"
  class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
  ><img
    data-slot="avatar-image"
    class="rounded-full aspect-square size-full object-cover grayscale"
    alt="@shadcn"
    src="https://github.com/shadcn.png"
/></span>
```
:::

::::


## Badge

Use the `AvatarBadge` component to add a badge to the avatar. The badge is positioned at the bottom right of the avatar.

::::demo avatar-badge
<iframe class="demo" src="/demos/avatar-badge.html" title="avatar-badge" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-badge.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-badge.html]
<span
  data-slot="avatar"
  data-size="default"
  class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
  ><img
    data-slot="avatar-image"
    class="rounded-full aspect-square size-full object-cover"
    alt="@shadcn"
    src="https://github.com/shadcn.png" /><span
    data-slot="avatar-badge"
    class="text-primary-foreground ring-background absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&amp;&gt;svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&amp;&gt;svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&amp;&gt;svg]:size-2 bg-green-600 dark:bg-green-800"
  ></span
></span>
```
:::

::::


Use the `className` prop to add custom styles to the badge such as custom colors, sizes, etc.

```tsx showLineNumbers
<Avatar>
  <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
  <AvatarFallback>CN</AvatarFallback>
  <AvatarBadge className="bg-green-600 dark:bg-green-800" />
</Avatar>
```

## Badge with Icon

You can also use an icon inside `<AvatarBadge>`.

::::demo avatar-badge-icon
<iframe class="demo" src="/demos/avatar-badge-icon.html" title="avatar-badge-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-badge-icon.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-badge-icon.html]
<span
  data-slot="avatar"
  data-size="default"
  class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten grayscale"
  ><img
    data-slot="avatar-image"
    class="rounded-full aspect-square size-full object-cover"
    alt="@pranathip"
    src="https://github.com/pranathip.png" /><span
    data-slot="avatar-badge"
    class="bg-primary text-primary-foreground ring-background absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&amp;&gt;svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&amp;&gt;svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&amp;&gt;svg]:size-2"
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
      class="lucide lucide-plus"
    >
      <path d="M5 12h14"></path>
      <path d="M12 5v14"></path></svg></span
></span>
```
:::

::::


## Avatar Group

Use the `AvatarGroup` component to add a group of avatars.

::::demo avatar-group
<iframe class="demo" src="/demos/avatar-group.html" title="avatar-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-group.html]
<div
  data-slot="avatar-group"
  class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
>
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@maxleiter"
      src="https://github.com/maxleiter.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png"
  /></span>
</div>
```
:::

::::


## Avatar Group Count

Use `<AvatarGroupCount>` to add a count to the group.

::::demo avatar-group-count
<iframe class="demo" src="/demos/avatar-group-count.html" title="avatar-group-count" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-group-count.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-group-count.html]
<div
  data-slot="avatar-group"
  class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
>
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@maxleiter"
      src="https://github.com/maxleiter.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png"
  /></span>
  <div
    data-slot="avatar-group-count"
    class="bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&amp;&gt;svg]:size-4 group-has-data-[size=lg]/avatar-group:[&amp;&gt;svg]:size-5 group-has-data-[size=sm]/avatar-group:[&amp;&gt;svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background"
  >
    +3
  </div>
</div>
```
:::

::::


## Avatar Group with Icon

You can also use an icon inside `<AvatarGroupCount>`.

::::demo avatar-group-count-icon
<iframe class="demo" src="/demos/avatar-group-count-icon.html" title="avatar-group-count-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-group-count-icon.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-group-count-icon.html]
<div
  data-slot="avatar-group"
  class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
>
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@maxleiter"
      src="https://github.com/maxleiter.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png"
  /></span>
  <div
    data-slot="avatar-group-count"
    class="bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&amp;&gt;svg]:size-4 group-has-data-[size=lg]/avatar-group:[&amp;&gt;svg]:size-5 group-has-data-[size=sm]/avatar-group:[&amp;&gt;svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background"
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
  </div>
</div>
```
:::

::::


## Sizes

Use the `size` prop to change the size of the avatar.

::::demo avatar-size
<iframe class="demo" src="/demos/avatar-size.html" title="avatar-size" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-size.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-size.html]
<div class="flex flex-wrap items-center gap-2 grayscale">
  <span
    data-slot="avatar"
    data-size="sm"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="lg"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png"
  /></span>
</div>
```
:::

::::


## Dropdown

You can use the `Avatar` component as a trigger for a dropdown menu.

::::demo avatar-dropdown
<iframe class="demo" src="/demos/avatar-dropdown.html" title="avatar-dropdown" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-dropdown.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [avatar-dropdown.html]
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
      >CN</span
    ></span
  >
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
        Log out
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


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo avatar-rtl
<iframe class="demo" src="/demos/avatar-rtl.html" title="avatar-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-rtl.html">Open the demo page</a> · <a href="/demos/avatar-rtl-he.html">HE</a> · <a href="/demos/avatar-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [avatar-rtl.html]
<div class="flex flex-row flex-wrap items-center gap-6 md:gap-12" dir="rtl">
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover grayscale"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png" /><span
      data-slot="avatar-badge"
      class="text-primary-foreground ring-background absolute end-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&amp;&gt;svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&amp;&gt;svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&amp;&gt;svg]:size-2 bg-green-600 dark:bg-green-800"
    ></span
  ></span>
  <div
    data-slot="avatar-group"
    class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
  >
    <span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@shadcn"
        src="https://github.com/shadcn.png" /></span
    ><span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@maxleiter"
        src="https://github.com/maxleiter.png" /></span
    ><span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@evilrabbit"
        src="https://github.com/evilrabbit.png"
    /></span>
    <div
      data-slot="avatar-group-count"
      class="bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&amp;&gt;svg]:size-4 group-has-data-[size=lg]/avatar-group:[&amp;&gt;svg]:size-5 group-has-data-[size=sm]/avatar-group:[&amp;&gt;svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background"
    >
      +٣
    </div>
  </div>
</div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="avatar"` |
| `data-slot="avatar-image"` |
| `data-slot="avatar-fallback"` |
| `data-slot="avatar-badge"` |
| `data-slot="avatar-group"` |
| `data-slot="avatar-group-count"` |

**Runtime:** `avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

### Avatar

The `Avatar` component is the root component that wraps the avatar image and fallback.

| Prop        | Type                        | Default     |
| ----------- | --------------------------- | ----------- |
| `size`      | `"default" \| "sm" \| "lg"` | `"default"` |
| `className` | `string`                    | -           |

### AvatarImage

The `AvatarImage` component displays the avatar image. It is a plain `<img data-slot="avatar-image">` — the shadless runtime switches to the fallback from its load state.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `src`       | `string` | -       |
| `alt`       | `string` | -       |
| `className` | `string` | -       |

### AvatarFallback

The `AvatarFallback` component displays a fallback when the image fails to load. It is a plain `<span data-slot="avatar-fallback">` shown by the shadless runtime while the image is loading or failed.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### AvatarBadge

The `AvatarBadge` component displays a badge indicator on the avatar, typically positioned at the bottom right.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### AvatarGroup

The `AvatarGroup` component displays a group of avatars with overlapping styling.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### AvatarGroupCount

The `AvatarGroupCount` component displays a count indicator in an avatar group, typically showing the number of additional avatars.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

For more information about Radix UI Avatar props, see the [Radix UI documentation](https://www.radix-ui.com/primitives/docs/components/avatar#api-reference).
