---
title: "Marker"
description: "Displays an inline status, system note, bordered row, or labeled separator in a conversation."
---

# Marker

Displays an inline status, system note, bordered row, or labeled separator in a conversation.

::::demo marker-demo
<iframe class="demo" src="/demos/marker-demo.html" title="marker-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/marker-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [marker-demo.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-git-branch"
      >
        <line x1="6" x2="6" y1="3" y2="15"></line>
        <circle cx="18" cy="6" r="3"></circle>
        <circle cx="6" cy="18" r="3"></circle>
        <path d="M18 9a9 9 0 0 1-9 9"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Switched to a new branch</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
    role="status"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-loader-circle size-4 animate-spin"
        data-slot="spinner"
        role="status"
        aria-label="Loading"
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word shimmer"
      >Thinking...</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="separator"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Conversation compacted</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-search"
      >
        <circle cx="11" cy="11" r="8"></circle>
        <path d="m21 21-4.3-4.3"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Explored 4 files</span
    >
  </div>
</div>
```
:::

::::

The `Marker` component displays inline conversation markers such as status updates, system notes, bordered rows, and labeled separators. Compose it with [`Message`](/components/message) in a conversation thread.

## Installation

**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/marker.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/marker.css` | this component's slot styles (`@apply` source — your build compiles it) |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
marker
├── marker-icon
└── marker-content
```

## Features

- Inline marker, bordered row, and labeled separator variants
- Decorative icon slot that is hidden from assistive tech
- Polymorphic root via `asChild` for link and button markers
- Pairs with the [`shimmer`](/guides/shimmer) utility for streaming status text
- Customizable styling through the `className` prop on every part

## Variants

Use `variant` to switch between an inline marker, bordered row, and labeled separator.

::::demo marker-variants
<iframe class="demo" src="/demos/marker-variants.html" title="marker-variants" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/marker-variants.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [marker-variants.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >A default marker for inline notes.</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="separator"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >A separator marker</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="border"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center border-b border-border pb-2"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >A border marker for row boundaries.</span
    >
  </div>
</div>
```
:::

::::

| Variant     | Description                                          |
| ----------- | ---------------------------------------------------- |
| `default`   | An inline marker for status, notes, and actions.     |
| `border`    | A default marker with a bottom border under the row. |
| `separator` | A centered label with divider lines on each side.    |

## Status

Set `role="status"` and include a [`Spinner`](/components/spinner) for streaming or in-progress markers so updates are announced.

::::demo marker-status
<iframe class="demo" src="/demos/marker-status.html" title="marker-status" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/marker-status.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [marker-status.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
    role="status"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-loader-circle size-4 animate-spin"
        data-slot="spinner"
        role="status"
        aria-label="Loading"
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Compacting conversation</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="separator"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
    role="status"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-loader-circle size-4 animate-spin"
        data-slot="spinner"
        role="status"
        aria-label="Loading"
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Running tests</span
    >
  </div>
</div>
```
:::

::::

## Shimmer

Add the [`shimmer`](/guides/shimmer) utility class to `MarkerContent` for an animated streaming-text effect. The utility ships with the `shadcn` package — see the shimmer docs for installation.

::::demo marker-shimmer
<iframe class="demo" src="/demos/marker-shimmer.html" title="marker-shimmer" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/marker-shimmer.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [marker-shimmer.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
    role="status"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word shimmer"
      >Thinking...</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="separator"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
    role="status"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word shimmer"
      >Reading 4 files</span
    >
  </div>
</div>
```
:::

::::

## Separator

Use the `separator` variant for labeled dividers, such as dates or section breaks, in a conversation.

::::demo marker-separator
<iframe class="demo" src="/demos/marker-separator.html" title="marker-separator" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/marker-separator.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [marker-separator.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <div
    data-slot="marker"
    data-variant="separator"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Today</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="separator"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Worked for 42s</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="separator"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
  >
    <span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Conversation compacted</span
    >
  </div>
</div>
```
:::

::::

## Border

Use the `border` variant for status rows that should keep the default marker alignment while separating the next row.

::::demo marker-border
<iframe class="demo" src="/demos/marker-border.html" title="marker-border" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/marker-border.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [marker-border.html]
<div class="flex w-full max-w-sm flex-col gap-3 py-12">
  <div
    data-slot="marker"
    data-variant="border"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center border-b border-border pb-2"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-git-branch"
      >
        <line x1="6" x2="6" y1="3" y2="15"></line>
        <circle cx="18" cy="6" r="3"></circle>
        <circle cx="6" cy="18" r="3"></circle>
        <path d="M18 9a9 9 0 0 1-9 9"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Switched to release-candidate</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="border"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center border-b border-border pb-2"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-search"
      >
        <circle cx="11" cy="11" r="8"></circle>
        <path d="m21 21-4.3-4.3"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Reviewed 8 related files</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="border"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center border-b border-border pb-2"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-file-text"
      >
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M10 9H8"></path>
        <path d="M16 13H8"></path>
        <path d="M16 17H8"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Opened implementation notes</span
    >
  </div>
</div>
```
:::

::::

## With Icon

Use `MarkerIcon` to render an icon alongside the content. Use `flex-col` to stack the icon above the content.

::::demo marker-icon
<iframe class="demo" src="/demos/marker-icon.html" title="marker-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/marker-icon.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [marker-icon.html]
<div class="flex w-full max-w-sm flex-col gap-12 py-12">
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-git-branch"
      >
        <line x1="6" x2="6" y1="3" y2="15"></line>
        <circle cx="18" cy="6" r="3"></circle>
        <circle cx="6" cy="18" r="3"></circle>
        <path d="M18 9a9 9 0 0 1-9 9"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Switched to a new branch</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="separator"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center before:h-px before:min-w-0 before:flex-1 before:bg-border after:h-px after:min-w-0 after:flex-1 after:bg-border before:mr-1 after:ml-1"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-search"
      >
        <circle cx="11" cy="11" r="8"></circle>
        <path d="m21 21-4.3-4.3"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Explored 4 files</span
    >
  </div>
  <div
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center flex-col"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-book-open-check"
      >
        <path d="M12 21V7"></path>
        <path d="m16 12 2 2 4-4"></path>
        <path
          d="M22 6V4a1 1 0 0 0-1-1h-5a4 4 0 0 0-4 4 4 4 0 0 0-4-4H3a1 1 0 0 0-1 1v13a1 1 0 0 0 1 1h6a3 3 0 0 1 3 3 3 3 0 0 1 3-3h6a1 1 0 0 0 1-1v-1.3"
        ></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Syncing completed</span
    >
  </div>
</div>
```
:::

::::

## Links and Buttons

Turn a marker into a link or button by putting `data-slot="marker"` on the `<a>` or `<button>` itself — shadless has no `asChild`.

::::demo marker-link-button
<iframe class="demo" src="/demos/marker-link-button.html" title="marker-link-button" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/marker-link-button.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [marker-link-button.html]
<div class="flex w-full max-w-sm flex-col gap-8 py-12">
  <a
    href="#links-and-buttons"
    data-slot="marker"
    data-variant="default"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center"
    ><span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-git-branch"
      >
        <line x1="6" x2="6" y1="3" y2="15"></line>
        <circle cx="18" cy="6" r="3"></circle>
        <circle cx="6" cy="18" r="3"></circle>
        <path d="M18 9a9 9 0 0 1-9 9"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >View the pull request</span
    ></a
  ><button
    type="button"
    class="gap-2 text-sm text-muted-foreground [a]:hover:text-foreground [a]:underline-offset-3 [a]:underline [&amp;_svg:not([class*='size-'])]:size-4 min-h-4 text-left group/marker relative flex w-full items-center transition-colors hover:text-foreground"
    data-slot="marker"
    data-variant="default"
  >
    <span
      data-slot="marker-icon"
      aria-hidden="true"
      class="size-4 [&amp;_svg:not([class*='size-'])]:size-4 shrink-0"
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
        class="lucide lucide-rotate-ccw"
      >
        <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
        <path d="M3 3v5h5"></path></svg></span
    ><span
      data-slot="marker-content"
      class="group-data-[variant=separator]/marker:flex-none group-data-[variant=separator]/marker:text-center *:[a]:hover:text-foreground *:[a]:underline *:[a]:underline-offset-3 min-w-0 wrap-break-word"
      >Revert this change</span
    >
  </button>
</div>
```
:::

::::

```html showLineNumbers
<a data-slot="marker" href="#">
  <span data-slot="marker-content">View the pull request</span>
</a>
```

## Accessibility

`Marker` is presentational by default. The correct semantics depend on how you use it, so choose the role based on intent rather than relying on a single default.

### Status and Progress

For streaming or progress markers such as "Thinking..." or a running tool, set `role="status"` so assistive tech announces the update as it appears. `Marker` forwards `role` to the underlying element.

```html
<div data-slot="marker" role="status">
  <!-- lucide "marker" icon -->
    <svg data-slot="spinner" />
  <!-- lucide "marker" icon -->
  <span data-slot="marker-content">Compacting conversation</span>
</div>
```

### Labeled Separators

A separator that carries text, such as a date or a section label, needs no role. The divider lines are decorative CSS pseudo-elements, and the text is announced as ordinary content.

```html
<div data-slot="marker" data-variant="separator">
  <span data-slot="marker-content">Today</span>
</div>
```

::: tip
**Note:** Do not add `role="separator"` to a labeled divider. A separator
takes its accessible name from `aria-label`, not from its text, and its
contents are treated as presentational, so the visible label would not be
announced. Reserve `role="separator"` for a divider with no meaningful text.
:::

### Bordered Markers

A bordered marker keeps the same semantics as the default marker. The bottom border is decorative, so choose `role="status"`, `asChild`, or no role based on the marker's purpose.

```html
<div data-slot="marker" data-variant="border">
  <!-- lucide "marker" icon -->
    <!-- lucide "file-text" icon -->
  <!-- lucide "marker" icon -->
  <span data-slot="marker-content">Opened implementation notes</span>
</div>
```

### Decorative Icons

`MarkerIcon` is decorative and hidden from assistive tech with `aria-hidden`, so the adjacent `MarkerContent` carries the meaning. For an icon-only marker, provide an `aria-label` or visible text so it is not announced as empty.

```html
<div data-slot="marker" aria-label="Synced">
  <!-- lucide "marker" icon -->
    <!-- lucide "check" icon -->
  <!-- lucide "marker" icon -->
</div>
```

### Interactive Markers

When a marker links or triggers an action, put `data-slot="marker"` on a real `<button>` or `<a>` so it is focusable and exposes the correct role. The accessible name comes from the marker text.

```html showLineNumbers
<a data-slot="marker" href="/files">
  <span data-slot="marker-icon"><!-- lucide "file-text" icon --></span>
  <span data-slot="marker-content">Explored 4 files</span>
</a>
```

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="marker"` |
| `data-slot="marker-icon"` |
| `data-slot="marker-content"` |

**Runtime:** no JavaScript — this is markup + CSS. Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — check `dist/css/marker.css` for any other `data-*` selector on these slots.

| Slot | Attribute | Values | Default |
| --- | --- | --- | --- |
| `marker` | `data-variant` | `default`, `separator`, `border` | `default` |
