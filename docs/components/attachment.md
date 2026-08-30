---
title: "Attachment"
description: "Displays a file or image attachment with media, metadata, upload state, and actions."
---

# Attachment

Displays a file or image attachment with media, metadata, upload state, and actions.

::::demo attachment-demo
<iframe class="demo" src="/demos/attachment-demo.html" title="attachment-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/attachment-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [attachment-demo.html]
<div class="mx-auto flex w-full max-w-sm flex-col gap-3 py-12">
  <div
    data-slot="attachment-group"
    class="gap-3 scroll-px-1 py-1 flex min-w-0 scroll-fade-x snap-x snap-mandatory scrollbar-none overflow-x-auto overscroll-x-contain *:data-[slot=attachment]:flex-none *:data-[slot=attachment]:snap-start"
  >
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="vertical"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full min-w-0 shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm w-24 has-data-[slot=attachment-content]:w-30 flex-col"
    >
      <div
        data-slot="attachment-media"
        data-variant="image"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
      >
        <img
          alt="Workspace"
          src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        />
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >workspace.png</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >PNG · 820 KB</span
        >
      </div>
    </div>
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="vertical"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full min-w-0 shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm w-24 has-data-[slot=attachment-content]:w-30 flex-col"
    >
      <div
        data-slot="attachment-media"
        data-variant="image"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
      >
        <img
          alt="Desk"
          src="https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        />
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >desk-reference.jpg</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >JPG · 1.1 MB</span
        >
      </div>
    </div>
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="vertical"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full min-w-0 shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm w-24 has-data-[slot=attachment-content]:w-30 flex-col"
    >
      <div
        data-slot="attachment-media"
        data-variant="image"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
      >
        <img
          alt="Office"
          src="https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        />
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >office-reference.jpg</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >JPG · 940 KB</span
        >
      </div>
    </div>
  </div>
  <div
    data-slot="attachment"
    data-state="uploading"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-loader-circle size-4 animate-spin"
        data-slot="spinner"
        role="status"
        aria-label="Loading"
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >sales-dashboard.pdf</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >Uploading · 64%</span
      >
    </div>
    <div
      data-slot="attachment-actions"
      class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
    >
      <button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Cancel upload"
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
          class="lucide lucide-x"
        >
          <path d="M18 6 6 18"></path>
          <path d="m6 6 12 12"></path>
        </svg>
      </button>
    </div>
  </div>
  <div
    data-slot="attachment"
    data-state="done"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-file-code"
      >
        <path d="M10 12.5 8 15l2 2.5"></path>
        <path d="m14 12.5 2 2.5-2 2.5"></path>
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >message-renderer.tsx</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >TypeScript · 12 KB</span
      >
    </div>
    <div
      data-slot="attachment-actions"
      class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
    >
      <button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Remove message-renderer.tsx"
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
          class="lucide lucide-x"
        >
          <path d="M18 6 6 18"></path>
          <path d="m6 6 12 12"></path>
        </svg>
      </button>
    </div>
  </div>
</div>
```
:::

::::


The `Attachment` component displays a file or image attachment, its media, name, and metadata, with optional actions and upload state. Use it for files and images in chat composers, message threads, and upload lists.

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/attachment.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/attachment.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/attachment.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                   into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/attachment.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `size="outline"` (JSX prop) | `data-size="outline"` (markup) |
| `orientation="outline"` (JSX prop) | `data-orientation="outline"` (markup) |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
attachment
├── attachment-media
├── attachment-content
│   ├── attachment-title
│   └── attachment-description
├── attachment-actions
│   └── attachment-action
└── attachment-trigger
```

## Features

- Icon and image media through `AttachmentMedia`
- Upload states: `idle`, `uploading`, `processing`, `error`, and `done` with built-in styling and a shimmer while in progress
- Three sizes and horizontal or vertical orientation
- A full-card `AttachmentTrigger` that opens a link or dialog while the actions stay independently clickable
- Scrollable, snapping `AttachmentGroup` with an edge fade
- Customizable styling through the `className` prop on every part

## Image

Set `variant="image"` on `AttachmentMedia` and render an `<img>` inside it. Use `orientation="vertical"` to stack the media above the content.

::::demo attachment-image
<iframe class="demo" src="/demos/attachment-image.html" title="attachment-image" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/attachment-image.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [attachment-image.html]
<div class="mx-auto w-full max-w-sm py-12">
  <div
    data-slot="attachment-group"
    class="gap-3 scroll-px-1 py-1 flex min-w-0 scroll-fade-x snap-x snap-mandatory scrollbar-none overflow-x-auto overscroll-x-contain *:data-[slot=attachment]:flex-none *:data-[slot=attachment]:snap-start w-full"
  >
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="vertical"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full min-w-0 shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm w-24 has-data-[slot=attachment-content]:w-30 flex-col"
    >
      <div
        data-slot="attachment-media"
        data-variant="image"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
      >
        <img
          alt="Workspace"
          src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        />
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >workspace.png</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >PNG · 820 KB</span
        >
      </div>
      <div
        data-slot="attachment-actions"
        class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
      >
        <button
          data-slot="attachment-action"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
          aria-label="Remove workspace.png"
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
            class="lucide lucide-x"
          >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
          </svg>
        </button>
      </div>
      <a
        href="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        target="_blank"
        rel="noreferrer"
        aria-label="Open workspace.png"
        data-slot="attachment-trigger"
        class="absolute inset-0 z-10 outline-none"
      ></a>
    </div>
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="vertical"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full min-w-0 shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm w-24 has-data-[slot=attachment-content]:w-30 flex-col"
    >
      <div
        data-slot="attachment-media"
        data-variant="image"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
      >
        <img
          alt="Desk"
          src="https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        />
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >desk-reference.jpg</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >JPG · 1.1 MB</span
        >
      </div>
      <div
        data-slot="attachment-actions"
        class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
      >
        <button
          data-slot="attachment-action"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
          aria-label="Remove desk-reference.jpg"
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
            class="lucide lucide-x"
          >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
          </svg>
        </button>
      </div>
      <a
        href="https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        target="_blank"
        rel="noreferrer"
        aria-label="Open desk-reference.jpg"
        data-slot="attachment-trigger"
        class="absolute inset-0 z-10 outline-none"
      ></a>
    </div>
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="vertical"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full min-w-0 shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm w-24 has-data-[slot=attachment-content]:w-30 flex-col"
    >
      <div
        data-slot="attachment-media"
        data-variant="image"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
      >
        <img
          alt="Office"
          src="https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        />
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >office-reference.jpg</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >JPG · 940 KB</span
        >
      </div>
      <div
        data-slot="attachment-actions"
        class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
      >
        <button
          data-slot="attachment-action"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
          aria-label="Remove office-reference.jpg"
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
            class="lucide lucide-x"
          >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
          </svg>
        </button>
      </div>
      <a
        href="https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        target="_blank"
        rel="noreferrer"
        aria-label="Open office-reference.jpg"
        data-slot="attachment-trigger"
        class="absolute inset-0 z-10 outline-none"
      ></a>
    </div>
  </div>
</div>
```
:::

::::


## States

Set `state` to reflect the upload lifecycle. `uploading` and `processing` shimmer the title, and `error` switches to a destructive treatment.

::::demo attachment-states
<iframe class="demo" src="/demos/attachment-states.html" title="attachment-states" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/attachment-states.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [attachment-states.html]
<div class="mx-auto flex w-full max-w-sm flex-col gap-2 py-12">
  <div
    data-slot="attachment"
    data-state="idle"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        <polyline points="12 6 12 12 16 14"></polyline>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >selected-file.pdf</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >Ready to upload</span
      >
    </div>
    <div
      data-slot="attachment-actions"
      class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
    >
      <button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Remove selected-file.pdf"
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
          class="lucide lucide-x"
        >
          <path d="M18 6 6 18"></path>
          <path d="m6 6 12 12"></path>
        </svg>
      </button>
    </div>
  </div>
  <div
    data-slot="attachment"
    data-state="uploading"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-loader-circle size-4 animate-spin"
        data-slot="spinner"
        role="status"
        aria-label="Loading"
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >design-system.zip</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >Uploading · 64%</span
      >
    </div>
    <div
      data-slot="attachment-actions"
      class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
    >
      <button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Cancel upload"
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
          class="lucide lucide-x"
        >
          <path d="M18 6 6 18"></path>
          <path d="m6 6 12 12"></path>
        </svg>
      </button>
    </div>
  </div>
  <div
    data-slot="attachment"
    data-state="processing"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-file-text"
      >
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M10 9H8"></path>
        <path d="M16 13H8"></path>
        <path d="M16 17H8"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >market-research.pdf</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >Processing document</span
      >
    </div>
    <div
      data-slot="attachment-actions"
      class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
    >
      <button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Remove market-research.pdf"
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
          class="lucide lucide-x"
        >
          <path d="M18 6 6 18"></path>
          <path d="m6 6 12 12"></path>
        </svg>
      </button>
    </div>
  </div>
  <div
    data-slot="attachment"
    data-state="error"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-file-warning"
      >
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
        <path d="M12 9v4"></path>
        <path d="M12 17h.01"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >financial-model.xlsx</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >Upload failed. Try again.</span
      >
    </div>
    <div
      data-slot="attachment-actions"
      class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
    >
      <button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Retry upload"
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
          class="lucide lucide-refresh-cw"
        >
          <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"></path>
          <path d="M21 3v5h-5"></path>
          <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"></path>
          <path d="M8 16H3v5"></path>
        </svg></button
      ><button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Remove financial-model.xlsx"
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
          class="lucide lucide-x"
        >
          <path d="M18 6 6 18"></path>
          <path d="m6 6 12 12"></path>
        </svg>
      </button>
    </div>
  </div>
  <div
    data-slot="attachment"
    data-state="done"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        <path d="M20 6 9 17l-5-5"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >uploaded-report.pdf</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >Uploaded · 1.8 MB</span
      >
    </div>
    <div
      data-slot="attachment-actions"
      class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
    >
      <button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Remove uploaded-report.pdf"
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
          class="lucide lucide-x"
        >
          <path d="M18 6 6 18"></path>
          <path d="m6 6 12 12"></path>
        </svg>
      </button>
    </div>
  </div>
</div>
```
:::

::::


## Sizes

Use `size` to switch between `default`, `sm`, and `xs`.

::::demo attachment-sizes
<iframe class="demo" src="/demos/attachment-sizes.html" title="attachment-sizes" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/attachment-sizes.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [attachment-sizes.html]
<div class="mx-auto flex w-full max-w-sm flex-col gap-3 py-12">
  <div
    data-slot="attachment"
    data-state="done"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-file-text"
      >
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M10 9H8"></path>
        <path d="M16 13H8"></path>
        <path d="M16 17H8"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >Default attachment</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >PDF · 2.4 MB</span
      >
    </div>
  </div>
  <div
    data-slot="attachment"
    data-state="done"
    data-size="sm"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2.5 has-data-[slot=attachment-content]:px-2 has-data-[slot=attachment-content]:py-1.5 has-data-[slot=attachment-media]:p-1.5 text-xs min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-file-text"
      >
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M10 9H8"></path>
        <path d="M16 13H8"></path>
        <path d="M16 17H8"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >Small attachment</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >PDF · 2.4 MB</span
      >
    </div>
  </div>
  <div
    data-slot="attachment"
    data-state="done"
    data-size="xs"
    data-orientation="horizontal"
    class="focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-1.5 has-data-[slot=attachment-content]:px-1.5 has-data-[slot=attachment-content]:py-1 has-data-[slot=attachment-media]:p-1 text-xs rounded-lg min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-file-text"
      >
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M10 9H8"></path>
        <path d="M16 13H8"></path>
        <path d="M16 17H8"></path>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >Extra small attachment</span
      >
    </div>
  </div>
</div>
```
:::

::::


## Group

Wrap attachments in `AttachmentGroup` to lay them out in a horizontally scrollable, snapping row with an edge fade.

::::demo attachment-group
<iframe class="demo" src="/demos/attachment-group.html" title="attachment-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/attachment-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [attachment-group.html]
<div class="mx-auto w-full max-w-sm py-12">
  <div
    data-slot="attachment-group"
    class="gap-3 scroll-px-1 py-1 flex min-w-0 scroll-fade-x snap-x snap-mandatory scrollbar-none overflow-x-auto overscroll-x-contain *:data-[slot=attachment]:flex-none *:data-[slot=attachment]:snap-start w-full"
  >
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="horizontal"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-64"
    >
      <div
        data-slot="attachment-media"
        data-variant="icon"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
          class="lucide lucide-file-text"
        >
          <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
          <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
          <path d="M10 9H8"></path>
          <path d="M16 13H8"></path>
          <path d="M16 17H8"></path>
        </svg>
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >briefing-notes.pdf</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >PDF · 1.4 MB</span
        >
      </div>
      <div
        data-slot="attachment-actions"
        class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
      >
        <button
          data-slot="attachment-action"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
          aria-label="Remove briefing-notes.pdf"
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
            class="lucide lucide-x"
          >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
          </svg>
        </button>
      </div>
    </div>
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="horizontal"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-64"
    >
      <div
        data-slot="attachment-media"
        data-variant="image"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
      >
        <img
          alt="workspace.png"
          src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&amp;auto=format&amp;fit=crop&amp;q=80"
        />
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >workspace.png</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >PNG · 820 KB</span
        >
      </div>
      <div
        data-slot="attachment-actions"
        class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
      >
        <button
          data-slot="attachment-action"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
          aria-label="Remove workspace.png"
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
            class="lucide lucide-x"
          >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
          </svg>
        </button>
      </div>
    </div>
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="horizontal"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-64"
    >
      <div
        data-slot="attachment-media"
        data-variant="icon"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
          class="lucide lucide-table"
        >
          <path d="M12 3v18"></path>
          <rect width="18" height="18" x="3" y="3" rx="2"></rect>
          <path d="M3 9h18"></path>
          <path d="M3 15h18"></path>
        </svg>
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >customers.csv</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >CSV · 18 KB</span
        >
      </div>
      <div
        data-slot="attachment-actions"
        class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
      >
        <button
          data-slot="attachment-action"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
          aria-label="Remove customers.csv"
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
            class="lucide lucide-x"
          >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
          </svg>
        </button>
      </div>
    </div>
    <div
      data-slot="attachment"
      data-state="done"
      data-size="default"
      data-orientation="horizontal"
      class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-64"
    >
      <div
        data-slot="attachment-media"
        data-variant="icon"
        class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
          class="lucide lucide-file-code"
        >
          <path d="M10 12.5 8 15l2 2.5"></path>
          <path d="m14 12.5 2 2.5-2 2.5"></path>
          <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
          <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z"></path>
        </svg>
      </div>
      <div
        data-slot="attachment-content"
        class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
      >
        <span
          data-slot="attachment-title"
          class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
          >renderer.tsx</span
        ><span
          data-slot="attachment-description"
          class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
          >TSX · 12 KB</span
        >
      </div>
      <div
        data-slot="attachment-actions"
        class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
      >
        <button
          data-slot="attachment-action"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
          aria-label="Remove renderer.tsx"
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
            class="lucide lucide-x"
          >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
          </svg>
        </button>
      </div>
    </div>
  </div>
</div>
```
:::

::::


## Trigger

Add an `AttachmentTrigger` to make the whole card open a link or dialog. It fills the card behind the actions, so the actions stay clickable.

::::demo attachment-trigger
<iframe class="demo" src="/demos/attachment-trigger.html" title="attachment-trigger" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/attachment-trigger.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [attachment-trigger.html]
<div class="mx-auto w-full max-w-sm py-12">
  <div
    data-slot="attachment"
    data-state="done"
    data-size="default"
    data-orientation="horizontal"
    class="rounded-xl focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[&gt;a,&gt;button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm min-w-40 items-center w-full"
  >
    <div
      data-slot="attachment-media"
      data-variant="icon"
      class="bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&amp;_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&amp;_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&amp;_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&amp;_svg]:pointer-events-none"
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
        class="lucide lucide-file-search"
      >
        <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
        <path d="M4.268 21a2 2 0 0 0 1.727 1H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3"></path>
        <path d="m9 18-1.5-1.5"></path>
        <circle cx="5" cy="14" r="3"></circle>
      </svg>
    </div>
    <div
      data-slot="attachment-content"
      class="leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1"
    >
      <span
        data-slot="attachment-title"
        class="font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer"
        >research-summary.pdf</span
      ><span
        data-slot="attachment-description"
        class="mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80 max-w-full"
        >Open preview dialog</span
      >
    </div>
    <div
      data-slot="attachment-actions"
      class="group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center"
    >
      <button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Copy link"
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
          <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>
        </svg></button
      ><button
        data-slot="attachment-action"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&amp;_svg:not([class*='size-'])]:size-3"
        aria-label="Remove research-summary.pdf"
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
          class="lucide lucide-x"
        >
          <path d="M18 6 6 18"></path>
          <path d="m6 6 12 12"></path>
        </svg>
      </button>
    </div>
    <button
      data-slot="dialog-trigger"
      type="button"
      class="absolute inset-0 z-10 outline-none"
      aria-label="Preview research-summary.pdf"
      aria-haspopup="dialog"
      aria-expanded="false"
      aria-controls="d1"
      data-state="closed"
      id="d1-trigger"
    ></button>
  </div>
</div>
<template id="d1-portal">
  <div
    data-state="open"
    data-slot="dialog-overlay"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50"
    style="pointer-events: auto"
  ></div>
  <div
    role="dialog"
    id="d1"
    aria-describedby="d1-desc"
    aria-labelledby="d1-title"
    data-state="open"
    data-slot="dialog-content"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-sm ring-1 duration-100 fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 outline-none sm:max-w-md"
    tabindex="-1"
    style="pointer-events: auto"
  >
    <div data-slot="dialog-header" class="gap-2 flex flex-col">
      <h2
        id="d1-title"
        data-slot="dialog-title"
        class="text-base leading-none font-medium cn-font-heading"
      >
        research-summary.pdf
      </h2>
      <p
        id="d1-desc"
        data-slot="dialog-description"
        class="text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3"
      >
        The attachment trigger fills the card and opens the dialog, while the actions stay
        independently clickable above it.
      </p>
    </div>
    <button
      data-slot="dialog-close"
      data-variant="ghost"
      data-size="icon-sm"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg absolute top-2 right-2"
      type="button"
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
        class="lucide lucide-x"
      >
        <path d="M18 6 6 18"></path>
        <path d="m6 6 12 12"></path></svg
      ><span class="sr-only">Close</span>
    </button>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dialog.js
// shadless dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=dialog-overlay]");
        var content = portal.querySelector("[data-slot=dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);

        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "dialog");
          },
        });
        portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
          b.addEventListener("click", function () { handles && handles.close(true); });
        });
        open = true;
        shadless.h.emit(trigger, "open", "dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "dialog",
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


```tsx showLineNumbers
<Dialog>
  <Attachment>
    {/* media, content, actions */}
    <DialogTrigger asChild>
      <AttachmentTrigger aria-label="Preview research-summary.pdf" />
    </DialogTrigger>
  </Attachment>
  <DialogContent>{/* ... */}</DialogContent>
</Dialog>
```

## Accessibility

`AttachmentAction` renders a `Button`, and `AttachmentTrigger` renders a real `<button>` (or your element via `asChild`). Follow the guidance below so both are operable and announced.

### Label icon-only actions

`AttachmentAction` is usually icon-only, so give each one an `aria-label` describing the action and its target.

```tsx showLineNumbers
<AttachmentAction aria-label="Remove sales-dashboard.pdf">
  <XIcon />
</AttachmentAction>
```

### Label the trigger

`AttachmentTrigger` covers the card with no text of its own, so give it an `aria-label` for what activating it does.

```tsx showLineNumbers
<AttachmentTrigger asChild>
  <a
    href={url}
    target="_blank"
    rel="noreferrer"
    aria-label="Open workspace.png"
  />
</AttachmentTrigger>
```

The trigger sits behind the actions in the stacking order, so an `AttachmentAction` and the `AttachmentTrigger` never trap each other — both remain separately focusable and clickable.

### Keyboard scrolling

An `AttachmentGroup` scrolls horizontally. When its attachments are interactive: a trigger or actions, keyboard users reach off-screen items by tabbing to them. For a row of presentational attachments, make the group itself focusable and scrollable by adding `tabIndex={0}`, `role="group"`, and an `aria-label`.

### Meaning beyond color

The `error` state uses a destructive color. Keep the failure reason in `AttachmentDescription` so the state is not conveyed by color alone.

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="attachment"` |
| `data-slot="attachment-media"` |
| `data-slot="attachment-content"` |
| `data-slot="attachment-title"` |
| `data-slot="attachment-description"` |
| `data-slot="attachment-actions"` |
| `data-slot="attachment-action"` |
| `data-slot="attachment-trigger"` |
| `data-slot="attachment-group"` |

### Attachment

The root attachment container.

| Prop          | Type                                                         | Default        | Description                                       |
| ------------- | ------------------------------------------------------------ | -------------- | ------------------------------------------------- |
| `state`       | `"idle" \| "uploading" \| "processing" \| "error" \| "done"` | `"done"`       | The upload state. Drives styling and the shimmer. |
| `size`        | `"default" \| "sm" \| "xs"`                                  | `"default"`    | The attachment size.                              |
| `orientation` | `"horizontal" \| "vertical"`                                 | `"horizontal"` | Lay the media beside or above the content.        |
| `className`   | `string`                                                     | -              | Additional classes to apply to the root element.  |

### AttachmentMedia

The media slot for an icon or image preview.

| Prop        | Type                | Default  | Description                                    |
| ----------- | ------------------- | -------- | ---------------------------------------------- |
| `variant`   | `"icon" \| "image"` | `"icon"` | Whether the media holds an icon or an `<img>`. |
| `className` | `string`            | -        | Additional classes to apply to the media slot. |

### AttachmentContent

Wraps the title and description.

| Prop        | Type     | Default | Description                                      |
| ----------- | -------- | ------- | ------------------------------------------------ |
| `className` | `string` | -       | Additional classes to apply to the content slot. |

### AttachmentTitle

The attachment name. Shimmers while the attachment is `uploading` or `processing`.

| Prop        | Type     | Default | Description                               |
| ----------- | -------- | ------- | ----------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the title. |

### AttachmentDescription

Secondary metadata such as the file type, size, or upload status.

| Prop        | Type     | Default | Description                                     |
| ----------- | -------- | ------- | ----------------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the description. |

### AttachmentActions

A container for one or more actions, aligned to the end of the attachment.

| Prop        | Type     | Default | Description                                 |
| ----------- | -------- | ------- | ------------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the actions. |

### AttachmentAction

An action button. Renders a [        ](/components/button) and accepts all of its props.

| Prop       | Type                                  | Default     | Description                              |
| ---------- | ------------------------------------- | ----------- | ---------------------------------------- |
| `size`     | `Button["size"]`                      | `"icon-xs"` | The button size.                         |
| `...props` | `React.ComponentProps<typeof Button>` | -           | Props spread to the underlying `Button`. |

### AttachmentTrigger

A full-card overlay that activates the attachment. Renders a `<button>` by default.

| Prop       | Type                             | Default | Description                                  |
| ---------- | -------------------------------- | ------- | -------------------------------------------- |
| `asChild`  | `boolean`                        | `false` | Render as the child element, such as a link. |
| `...props` | `React.ComponentProps<"button">` | -       | Props spread to the trigger element.         |

### AttachmentGroup

Lays out attachments in a horizontally scrollable, snapping row.

| Prop        | Type     | Default | Description                               |
| ----------- | -------- | ------- | ----------------------------------------- |
| `className` | `string` | -       | Additional classes to apply to the group. |
