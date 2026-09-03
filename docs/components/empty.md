---
title: "Empty"
description: "Use the Empty component to display an empty state."
---

# Empty

Use the Empty component to display an empty state.

::::demo empty-demo
<iframe class="demo" src="/demos/empty-demo.html" title="empty-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/empty-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [empty-demo.html]
<div
  data-slot="empty"
  class="gap-4 rounded-xl border-dashed p-6 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance"
>
  <div data-slot="empty-header" class="gap-2 flex max-w-sm flex-col items-center">
    <div
      data-slot="empty-icon"
      data-variant="icon"
      class="mb-2 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-muted text-foreground flex size-8 shrink-0 items-center justify-center rounded-lg [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="tabler-icon tabler-icon-folder-code"
      >
        <path d="M11 19h-6a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4"></path>
        <path d="M20 21l2 -2l-2 -2"></path>
        <path d="M17 17l-2 2l2 2"></path>
      </svg>
    </div>
    <div data-slot="empty-title" class="text-sm font-medium tracking-tight cn-font-heading">
      No Projects Yet
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      You haven't created any projects yet. Get started by creating your first project.
    </div>
  </div>
  <div
    data-slot="empty-content"
    class="text-sm flex w-full max-w-sm min-w-0 items-center text-balance flex-row justify-center gap-2"
  >
    <button
      data-slot="button"
      data-variant="default"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Create Project</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    >
      Import Project
    </button>
  </div>
  <a
    href="#"
    data-slot="button"
    data-variant="link"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 text-muted-foreground"
    >Learn More
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
      class="lucide lucide-arrow-up-right"
    >
      <path d="M7 7h10v10"></path>
      <path d="M7 17 17 7"></path></svg
  ></a>
</div>
```
:::

::::

## Installation

**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/empty.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/empty.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/empty.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from `dist/components/empty.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
empty
├── empty-header
│   ├── empty-icon
│   ├── empty-title
│   └── empty-description
└── empty-content
```

## Outline

Use the `border` utility class to create an outline empty state.

::::demo empty-outline
<iframe class="demo" src="/demos/empty-outline.html" title="empty-outline" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/empty-outline.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [empty-outline.html]
<div
  data-slot="empty"
  class="gap-4 rounded-xl p-6 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance border border-dashed"
>
  <div data-slot="empty-header" class="gap-2 flex max-w-sm flex-col items-center">
    <div
      data-slot="empty-icon"
      data-variant="icon"
      class="mb-2 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-muted text-foreground flex size-8 shrink-0 items-center justify-center rounded-lg [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="tabler-icon tabler-icon-cloud"
      >
        <path
          d="M6.657 18c-2.572 0 -4.657 -2.007 -4.657 -4.483c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.913 0 3.464 1.56 3.464 3.486c0 1.927 -1.551 3.487 -3.465 3.487h-11.878"
        ></path>
      </svg>
    </div>
    <div data-slot="empty-title" class="text-sm font-medium tracking-tight cn-font-heading">
      Cloud Storage Empty
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Upload files to your cloud storage to access them anywhere.
    </div>
  </div>
  <div
    data-slot="empty-content"
    class="gap-2.5 text-sm flex w-full max-w-sm min-w-0 flex-col items-center text-balance"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    >
      Upload Files
    </button>
  </div>
</div>
```
:::

::::

## Background

Use the `bg-*` and `bg-gradient-*` utilities to add a background to the empty state.

::::demo empty-background
<iframe class="demo" src="/demos/empty-background.html" title="empty-background" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/empty-background.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [empty-background.html]
<div
  data-slot="empty"
  class="gap-4 rounded-xl border-dashed p-6 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance h-full bg-muted/30"
>
  <div data-slot="empty-header" class="gap-2 flex max-w-sm flex-col items-center">
    <div
      data-slot="empty-icon"
      data-variant="icon"
      class="mb-2 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-muted text-foreground flex size-8 shrink-0 items-center justify-center rounded-lg [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="tabler-icon tabler-icon-bell"
      >
        <path
          d="M10 5a2 2 0 1 1 4 0a7 7 0 0 1 4 6v3a4 4 0 0 0 2 3h-16a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6"
        ></path>
        <path d="M9 17v1a3 3 0 0 0 6 0v-1"></path>
      </svg>
    </div>
    <div data-slot="empty-title" class="text-sm font-medium tracking-tight cn-font-heading">
      No Notifications
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary max-w-xs text-pretty"
    >
      You're all caught up. New notifications will appear here.
    </div>
  </div>
  <div
    data-slot="empty-content"
    class="gap-2.5 text-sm flex w-full max-w-sm min-w-0 flex-col items-center text-balance"
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
        class="lucide lucide-refresh-ccw"
      >
        <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
        <path d="M3 3v5h5"></path>
        <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"></path>
        <path d="M16 16h5v5"></path></svg
      >Refresh
    </button>
  </div>
</div>
```
:::

::::

## Avatar

Use the `EmptyMedia` component to display an avatar in the empty state.

::::demo empty-avatar
<iframe class="demo" src="/demos/empty-avatar.html" title="empty-avatar" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/empty-avatar.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [empty-avatar.html]
<div
  data-slot="empty"
  class="gap-4 rounded-xl border-dashed p-6 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance"
>
  <div data-slot="empty-header" class="gap-2 flex max-w-sm flex-col items-center">
    <div
      data-slot="empty-icon"
      data-variant="default"
      class="mb-2 flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-transparent"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten size-12"
        ><img
          data-slot="avatar-image"
          class="rounded-full aspect-square size-full object-cover grayscale"
          src="https://github.com/shadcn.png"
      /></span>
    </div>
    <div data-slot="empty-title" class="text-sm font-medium tracking-tight cn-font-heading">
      User Offline
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      This user is currently offline. You can leave a message to notify them or try again later.
    </div>
  </div>
  <div
    data-slot="empty-content"
    class="gap-2.5 text-sm flex w-full max-w-sm min-w-0 flex-col items-center text-balance"
  >
    <button
      data-slot="button"
      data-variant="default"
      data-size="sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    >
      Leave Message
    </button>
  </div>
</div>
```
:::

::::

## Avatar Group

Use the `EmptyMedia` component to display an avatar group in the empty state.

::::demo empty-avatar-group
<iframe class="demo" src="/demos/empty-avatar-group.html" title="empty-avatar-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/empty-avatar-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [empty-avatar-group.html]
<div
  data-slot="empty"
  class="gap-4 rounded-xl border-dashed p-6 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance"
>
  <div data-slot="empty-header" class="gap-2 flex max-w-sm flex-col items-center">
    <div
      data-slot="empty-icon"
      data-variant="default"
      class="mb-2 flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-transparent"
    >
      <div
        class="flex -space-x-2 *:data-[slot=avatar]:size-12 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background *:data-[slot=avatar]:grayscale"
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
    </div>
    <div data-slot="empty-title" class="text-sm font-medium tracking-tight cn-font-heading">
      No Team Members
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Invite your team to collaborate on this project.
    </div>
  </div>
  <div
    data-slot="empty-content"
    class="gap-2.5 text-sm flex w-full max-w-sm min-w-0 flex-col items-center text-balance"
  >
    <button
      data-slot="button"
      data-variant="default"
      data-size="sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
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
        <path d="M12 5v14"></path></svg
      >Invite Members
    </button>
  </div>
</div>
```
:::

::::

## InputGroup

You can add an `InputGroup` component to the `EmptyContent` component.

::::demo empty-input-group
<iframe class="demo" src="/demos/empty-input-group.html" title="empty-input-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/empty-input-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [empty-input-group.html]
<div
  data-slot="empty"
  class="gap-4 rounded-xl border-dashed p-6 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance"
>
  <div data-slot="empty-header" class="gap-2 flex max-w-sm flex-col items-center">
    <div data-slot="empty-title" class="text-sm font-medium tracking-tight cn-font-heading">
      404 - Not Found
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      The page you're looking for doesn't exist. Try searching for what you need below.
    </div>
  </div>
  <div
    data-slot="empty-content"
    class="gap-2.5 text-sm flex w-full max-w-sm min-w-0 flex-col items-center text-balance"
  >
    <div
      data-slot="input-group"
      role="group"
      class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto sm:w-3/4"
    >
      <input
        data-slot="input-group-control"
        class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
        placeholder="Try searching for pages..."
      />
      <div
        role="group"
        data-slot="input-group-addon"
        data-align="inline-start"
        class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pl-2 has-[&gt;button]:ml-[-0.3rem] has-[&gt;kbd]:ml-[-0.15rem] order-first"
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
      </div>
      <div
        role="group"
        data-slot="input-group-addon"
        data-align="inline-end"
        class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
      >
        <kbd
          data-slot="kbd"
          class="bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-sm px-1 font-sans text-xs font-medium [&amp;_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none"
          >/</kbd
        >
      </div>
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Need help? <a href="#">Contact support</a>
    </div>
  </div>
</div>
```
:::

::::

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo empty-rtl
<iframe class="demo" src="/demos/empty-rtl.html" title="empty-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/empty-rtl.html">Open the demo page</a> · <a href="/demos/empty-rtl-he.html">HE</a> · <a href="/demos/empty-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [empty-rtl.html]
<div
  data-slot="empty"
  class="gap-4 rounded-xl border-dashed p-6 flex w-full min-w-0 flex-1 flex-col items-center justify-center text-center text-balance"
  dir="rtl"
>
  <div data-slot="empty-header" class="gap-2 flex max-w-sm flex-col items-center">
    <div
      data-slot="empty-icon"
      data-variant="icon"
      class="mb-2 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-muted text-foreground flex size-8 shrink-0 items-center justify-center rounded-lg [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="tabler-icon tabler-icon-folder-code"
      >
        <path d="M11 19h-6a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4"></path>
        <path d="M20 21l2 -2l-2 -2"></path>
        <path d="M17 17l-2 2l2 2"></path>
      </svg>
    </div>
    <div data-slot="empty-title" class="text-sm font-medium tracking-tight cn-font-heading">
      لا توجد مشاريع بعد
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      لم تقم بإنشاء أي مشاريع بعد. ابدأ بإنشاء مشروعك الأول.
    </div>
  </div>
  <div
    data-slot="empty-content"
    class="text-sm flex w-full max-w-sm min-w-0 items-center text-balance flex-row justify-center gap-2"
  >
    <button
      data-slot="button"
      data-variant="default"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    >
      إنشاء مشروع</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    >
      استيراد مشروع
    </button>
  </div>
  <a
    href="#"
    data-slot="button"
    data-variant="link"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 underline-offset-4 hover:underline h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pe-1.5 has-data-[icon=inline-start]:ps-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 text-muted-foreground"
    >تعرف على المزيد
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
      class="lucide lucide-arrow-up-right rtl:rotate-270"
      data-icon="inline-end"
    >
      <path d="M7 7h10v10"></path>
      <path d="M7 17 17 7"></path></svg
  ></a>
</div>
```
:::

::::

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="empty"` |
| `data-slot="empty-header"` |
| `data-slot="empty-icon"` |
| `data-slot="empty-title"` |
| `data-slot="empty-description"` |
| `data-slot="empty-content"` |

**Runtime:** no JavaScript — this is markup + CSS. Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — check `dist/css/empty.css` for any other `data-*` selector on these slots.

| Slot | Attribute | Values | Default |
| --- | --- | --- | --- |
| `empty-icon` | `data-variant` | `default`, `icon` | `default` |
