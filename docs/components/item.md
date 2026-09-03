---
title: "Item"
description: "A versatile component for displaying content with media, title, description, and actions."
---

# Item

A versatile component for displaying content with media, title, description, and actions.

::::demo item-demo
<iframe class="demo" src="/demos/item-demo.html" title="item-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-demo.html]
<div class="flex w-full max-w-md flex-col gap-6">
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Basic Item
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        A simple item with title and description.
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
      <button
        data-slot="button"
        data-variant="outline"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
      >
        Action
      </button>
    </div>
  </div>
  <a
    href="#"
    data-slot="item"
    data-variant="outline"
    data-size="sm"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
    ><div
      data-slot="item-media"
      data-variant="default"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
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
        class="lucide lucide-badge-check size-5"
      >
        <path
          d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"
        ></path>
        <path d="m9 12 2 2 4-4"></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Your profile has been verified.
      </div>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
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
        class="lucide lucide-chevron-right size-4"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg></div
  ></a>
</div>
```
:::

::::


The `Item` component is a straightforward flex container that can house nearly any type of content. Use it to display a title, description, and actions. Group it with the `ItemGroup` component to create a list of items.

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/item.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/item.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/item.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from `dist/components/item.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
item-group
└── item
    ├── item-header
    ├── item-media
    ├── item-content
    │   ├── item-title
    │   └── item-description
    ├── item-actions
    └── item-footer
```

## Item vs Field

Use `Field` if you need to display a form input such as a checkbox, input, radio, or select.

If you only need to display content such as a title, description, and actions, use `Item`.

## Variant

Use the `variant` prop to change the visual style of the item.

::::demo item-variant
<iframe class="demo" src="/demos/item-variant.html" title="item-variant" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-variant.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-variant.html]
<div class="flex w-full max-w-md flex-col gap-6">
  <div
    data-slot="item"
    data-variant="default"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-transparent gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="icon"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="lucide lucide-inbox"
      >
        <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
        <path
          d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
        ></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Default Variant
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Transparent background with no border.
      </p>
    </div>
  </div>
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="icon"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="lucide lucide-inbox"
      >
        <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
        <path
          d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
        ></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Outline Variant
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Outlined style with a visible border.
      </p>
    </div>
  </div>
  <div
    data-slot="item"
    data-variant="muted"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors bg-muted/50 border-transparent gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="icon"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="lucide lucide-inbox"
      >
        <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
        <path
          d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
        ></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Muted Variant
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Muted background for secondary content.
      </p>
    </div>
  </div>
</div>
```
:::

::::


## Size

Use the `size` prop to change the size of the item. Available sizes are `default`, `sm`, and `xs`.

::::demo item-size
<iframe class="demo" src="/demos/item-size.html" title="item-size" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-size.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-size.html]
<div class="flex w-full max-w-md flex-col gap-6">
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="icon"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="lucide lucide-inbox"
      >
        <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
        <path
          d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
        ></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Default Size
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        The standard size for most use cases.
      </p>
    </div>
  </div>
  <div
    data-slot="item"
    data-variant="outline"
    data-size="sm"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="icon"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="lucide lucide-inbox"
      >
        <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
        <path
          d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
        ></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Small Size
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        A compact size for dense layouts.
      </p>
    </div>
  </div>
  <div
    data-slot="item"
    data-variant="outline"
    data-size="xs"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2 px-2.5 py-2 in-data-[slot=dropdown-menu-content]:p-0"
  >
    <div
      data-slot="item-media"
      data-variant="icon"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="lucide lucide-inbox"
      >
        <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
        <path
          d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
        ></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Extra Small Size
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        The most compact size available.
      </p>
    </div>
  </div>
</div>
```
:::

::::


## Icon

Use `ItemMedia` with `variant="icon"` to display an icon.

::::demo item-icon
<iframe class="demo" src="/demos/item-icon.html" title="item-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-icon.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-icon.html]
<div class="flex w-full max-w-lg flex-col gap-6">
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="icon"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4"
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
        class="lucide lucide-shield-alert"
      >
        <path
          d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"
        ></path>
        <path d="M12 8v4"></path>
        <path d="M12 16h.01"></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Security Alert
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        New login detected from unknown device.
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
      <button
        data-slot="button"
        data-variant="outline"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
      >
        Review
      </button>
    </div>
  </div>
</div>
```
:::

::::


## Avatar

You can use `ItemMedia` with `variant="avatar"` to display an avatar.

::::demo item-avatar
<iframe class="demo" src="/demos/item-avatar.html" title="item-avatar" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-avatar.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-avatar.html]
<div class="flex w-full max-w-lg flex-col gap-6">
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="default"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten size-10"
        ><img
          data-slot="avatar-image"
          class="rounded-full aspect-square size-full object-cover"
          src="https://github.com/evilrabbit.png"
      /></span>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Evil Rabbit
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Last seen 5 months ago
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
      <button
        data-slot="button"
        data-variant="outline"
        data-size="icon-sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-7 in-data-[slot=button-group]:rounded-lg rounded-full"
        aria-label="Invite"
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
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="default"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
    >
      <div
        class="flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background *:data-[slot=avatar]:grayscale"
      >
        <span
          data-slot="avatar"
          data-size="default"
          class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten hidden sm:flex"
          ><img
            data-slot="avatar-image"
            class="rounded-full aspect-square size-full object-cover"
            alt="@shadcn"
            src="https://github.com/shadcn.png" /></span
        ><span
          data-slot="avatar"
          data-size="default"
          class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten hidden sm:flex"
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
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        No Team Members
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Invite your team to collaborate on this project.
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
      <button
        data-slot="button"
        data-variant="outline"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
      >
        Invite
      </button>
    </div>
  </div>
</div>
```
:::

::::


## Image

Use `ItemMedia` with `variant="image"` to display an image.

::::demo item-image
<iframe class="demo" src="/demos/item-image.html" title="item-image" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-image.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-image.html]
<div class="flex w-full max-w-md flex-col gap-6">
  <div
    role="list"
    data-slot="item-group"
    class="has-data-[size=sm]:gap-2.5 has-data-[size=xs]:gap-2 group/item-group flex w-full flex-col gap-4"
  >
    <a
      href="#"
      data-slot="item"
      data-variant="outline"
      data-size="default"
      class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
      role="listitem"
      ><div
        data-slot="item-media"
        data-variant="image"
        class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none size-10 overflow-hidden rounded-sm group-data-[size=sm]/item:size-8 group-data-[size=xs]/item:size-6 [&amp;_img]:size-full [&amp;_img]:object-cover"
      >
        <img
          alt="Midnight City Lights"
          width="32"
          height="32"
          data-nimg="1"
          decoding="async"
          loading="lazy"
          class="object-cover grayscale"
          src="https://avatar.vercel.sh/Midnight City Lights"
          style="color: transparent"
        />
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
      >
        <div
          data-slot="item-title"
          class="gap-2 text-sm leading-snug font-medium underline-offset-4 w-fit items-center line-clamp-1"
        >
          Midnight City Lights - <span class="text-muted-foreground">Electric Nights</span>
        </div>
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          Neon Dreams
        </p>
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-col [&amp;+[data-slot=item-content]]:flex-none flex-none text-center"
      >
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          3:45
        </p>
      </div></a
    ><a
      href="#"
      data-slot="item"
      data-variant="outline"
      data-size="default"
      class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
      role="listitem"
      ><div
        data-slot="item-media"
        data-variant="image"
        class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none size-10 overflow-hidden rounded-sm group-data-[size=sm]/item:size-8 group-data-[size=xs]/item:size-6 [&amp;_img]:size-full [&amp;_img]:object-cover"
      >
        <img
          alt="Coffee Shop Conversations"
          width="32"
          height="32"
          data-nimg="1"
          decoding="async"
          loading="lazy"
          class="object-cover grayscale"
          src="https://avatar.vercel.sh/Coffee Shop Conversations"
          style="color: transparent"
        />
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
      >
        <div
          data-slot="item-title"
          class="gap-2 text-sm leading-snug font-medium underline-offset-4 w-fit items-center line-clamp-1"
        >
          Coffee Shop Conversations - <span class="text-muted-foreground">Urban Stories</span>
        </div>
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          The Morning Brew
        </p>
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-col [&amp;+[data-slot=item-content]]:flex-none flex-none text-center"
      >
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          4:05
        </p>
      </div></a
    ><a
      href="#"
      data-slot="item"
      data-variant="outline"
      data-size="default"
      class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
      role="listitem"
      ><div
        data-slot="item-media"
        data-variant="image"
        class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none size-10 overflow-hidden rounded-sm group-data-[size=sm]/item:size-8 group-data-[size=xs]/item:size-6 [&amp;_img]:size-full [&amp;_img]:object-cover"
      >
        <img
          alt="Digital Rain"
          width="32"
          height="32"
          data-nimg="1"
          decoding="async"
          loading="lazy"
          class="object-cover grayscale"
          src="https://avatar.vercel.sh/Digital Rain"
          style="color: transparent"
        />
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
      >
        <div
          data-slot="item-title"
          class="gap-2 text-sm leading-snug font-medium underline-offset-4 w-fit items-center line-clamp-1"
        >
          Digital Rain - <span class="text-muted-foreground">Binary Beats</span>
        </div>
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          Cyber Symphony
        </p>
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-col [&amp;+[data-slot=item-content]]:flex-none flex-none text-center"
      >
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          3:30
        </p>
      </div></a
    >
  </div>
</div>
```
:::

::::


## Group

Use `ItemGroup` to group related items together.

::::demo item-group
<iframe class="demo" src="/demos/item-group.html" title="item-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-group.html]
<div
  role="list"
  data-slot="item-group"
  class="gap-4 has-data-[size=sm]:gap-2.5 has-data-[size=xs]:gap-2 group/item-group flex w-full flex-col max-w-sm"
>
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="default"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><img
          data-slot="avatar-image"
          class="rounded-full aspect-square size-full object-cover grayscale"
          src="https://github.com/shadcn.png"
      /></span>
    </div>
    <div
      data-slot="item-content"
      class="group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none gap-1"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        shadcn
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        shadcn@vercel.com
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
      <button
        data-slot="button"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8 rounded-full"
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
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="default"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><img
          data-slot="avatar-image"
          class="rounded-full aspect-square size-full object-cover grayscale"
          src="https://github.com/maxleiter.png"
      /></span>
    </div>
    <div
      data-slot="item-content"
      class="group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none gap-1"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        maxleiter
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        maxleiter@vercel.com
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
      <button
        data-slot="button"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8 rounded-full"
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
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
  >
    <div
      data-slot="item-media"
      data-variant="default"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
    >
      <span
        data-slot="avatar"
        data-size="default"
        class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
        ><img
          data-slot="avatar-image"
          class="rounded-full aspect-square size-full object-cover grayscale"
          src="https://github.com/evilrabbit.png"
      /></span>
    </div>
    <div
      data-slot="item-content"
      class="group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none gap-1"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        evilrabbit
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        evilrabbit@vercel.com
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
      <button
        data-slot="button"
        data-variant="ghost"
        data-size="icon"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8 rounded-full"
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
</div>
```
:::

::::


## Header

Use `ItemHeader` to add a header above the item content.

::::demo item-header
<iframe class="demo" src="/demos/item-header.html" title="item-header" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-header.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-header.html]
<div class="flex w-full max-w-xl flex-col gap-6">
  <div
    role="list"
    data-slot="item-group"
    class="has-data-[size=sm]:gap-2.5 has-data-[size=xs]:gap-2 group/item-group w-full flex-col grid grid-cols-3 gap-4"
  >
    <div
      data-slot="item"
      data-variant="outline"
      data-size="default"
      class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
    >
      <div data-slot="item-header" class="gap-2 flex basis-full items-center justify-between">
        <img
          alt="v0-1.5-sm"
          width="128"
          height="128"
          data-nimg="1"
          decoding="async"
          loading="lazy"
          class="aspect-square w-full rounded-sm object-cover"
          src="https://images.unsplash.com/photo-1650804068570-7fb2e3dbf888?q=80&amp;w=640&amp;auto=format&amp;fit=crop"
          style="color: transparent"
        />
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
      >
        <div
          data-slot="item-title"
          class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
        >
          v0-1.5-sm
        </div>
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          Everyday tasks and UI generation.
        </p>
      </div>
    </div>
    <div
      data-slot="item"
      data-variant="outline"
      data-size="default"
      class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
    >
      <div data-slot="item-header" class="gap-2 flex basis-full items-center justify-between">
        <img
          alt="v0-1.5-lg"
          width="128"
          height="128"
          data-nimg="1"
          decoding="async"
          loading="lazy"
          class="aspect-square w-full rounded-sm object-cover"
          src="https://images.unsplash.com/photo-1610280777472-54133d004c8c?q=80&amp;w=640&amp;auto=format&amp;fit=crop"
          style="color: transparent"
        />
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
      >
        <div
          data-slot="item-title"
          class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
        >
          v0-1.5-lg
        </div>
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          Advanced thinking or reasoning.
        </p>
      </div>
    </div>
    <div
      data-slot="item"
      data-variant="outline"
      data-size="default"
      class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
    >
      <div data-slot="item-header" class="gap-2 flex basis-full items-center justify-between">
        <img
          alt="v0-2.0-mini"
          width="128"
          height="128"
          data-nimg="1"
          decoding="async"
          loading="lazy"
          class="aspect-square w-full rounded-sm object-cover"
          src="https://images.unsplash.com/photo-1602146057681-08560aee8cde?q=80&amp;w=640&amp;auto=format&amp;fit=crop"
          style="color: transparent"
        />
      </div>
      <div
        data-slot="item-content"
        class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
      >
        <div
          data-slot="item-title"
          class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
        >
          v0-2.0-mini
        </div>
        <p
          data-slot="item-description"
          class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          Open Source model for everyone.
        </p>
      </div>
    </div>
  </div>
</div>
```
:::

::::


## Link

shadless has no `asChild` — put `data-slot="item"` on the anchor itself. The hover and focus states apply to the anchor.

::::demo item-link
<iframe class="demo" src="/demos/item-link.html" title="item-link" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-link.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-link.html]
<div class="flex w-full max-w-md flex-col gap-4">
  <a
    href="#"
    data-slot="item"
    data-variant="default"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-transparent gap-2.5 px-3 py-2.5"
    ><div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        Visit our documentation
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Learn how to get started with our components.
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
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
        class="lucide lucide-chevron-right size-4"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg></div></a
  ><a
    href="#"
    target="_blank"
    rel="noopener noreferrer"
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
    ><div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        External resource
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-left text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Opens in a new tab with security attributes.
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
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
        class="lucide lucide-external-link size-4"
      >
        <path d="M15 3h6v6"></path>
        <path d="M10 14 21 3"></path>
        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
      </svg></div
  ></a>
</div>
```
:::

::::


```html showLineNumbers
<a data-slot="item" href="/dashboard">
  <div data-slot="item-media" data-variant="icon">
    <!-- lucide "home" icon -->
  </div>
  <div data-slot="item-content">
    <div data-slot="item-title">Dashboard</div>
    <p data-slot="item-description">Overview of your account and activity.</p>
  </div>
</a>
```

## Dropdown

::::demo item-dropdown
<iframe class="demo" src="/demos/item-dropdown.html" title="item-dropdown" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-dropdown.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [item-dropdown.html]
<button
  data-slot="dropdown-menu-trigger"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  Select
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
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden w-48"
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
        <div
          data-slot="item"
          data-variant="default"
          data-size="xs"
          class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-transparent gap-2 in-data-[slot=dropdown-menu-content]:p-0 w-full p-2"
        >
          <div
            data-slot="item-media"
            data-variant="default"
            class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
          >
            <span
              data-slot="avatar"
              data-size="default"
              class="rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten size-[--spacing(6.5)]"
              ><span
                data-slot="avatar-fallback"
                class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
                >s</span
              ></span
            >
          </div>
          <div
            data-slot="item-content"
            class="group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none gap-0"
          >
            <div
              data-slot="item-title"
              class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
            >
              shadcn
            </div>
            <p
              data-slot="item-description"
              class="text-muted-foreground text-left text-sm group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary leading-none"
            >
              shadcn@vercel.com
            </p>
          </div>
        </div>
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
        <div
          data-slot="item"
          data-variant="default"
          data-size="xs"
          class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-transparent gap-2 in-data-[slot=dropdown-menu-content]:p-0 w-full p-2"
        >
          <div
            data-slot="item-media"
            data-variant="default"
            class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
          >
            <span
              data-slot="avatar"
              data-size="default"
              class="rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten size-[--spacing(6.5)]"
              ><span
                data-slot="avatar-fallback"
                class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
                >m</span
              ></span
            >
          </div>
          <div
            data-slot="item-content"
            class="group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none gap-0"
          >
            <div
              data-slot="item-title"
              class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
            >
              maxleiter
            </div>
            <p
              data-slot="item-description"
              class="text-muted-foreground text-left text-sm group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary leading-none"
            >
              maxleiter@vercel.com
            </p>
          </div>
        </div>
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
        <div
          data-slot="item"
          data-variant="default"
          data-size="xs"
          class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-transparent gap-2 in-data-[slot=dropdown-menu-content]:p-0 w-full p-2"
        >
          <div
            data-slot="item-media"
            data-variant="default"
            class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
          >
            <span
              data-slot="avatar"
              data-size="default"
              class="rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten size-[--spacing(6.5)]"
              ><span
                data-slot="avatar-fallback"
                class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
                >e</span
              ></span
            >
          </div>
          <div
            data-slot="item-content"
            class="group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none gap-0"
          >
            <div
              data-slot="item-title"
              class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
            >
              evilrabbit
            </div>
            <p
              data-slot="item-description"
              class="text-muted-foreground text-left text-sm group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary leading-none"
            >
              evilrabbit@vercel.com
            </p>
          </div>
        </div>
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


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo item-rtl
<iframe class="demo" src="/demos/item-rtl.html" title="item-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/item-rtl.html">Open the demo page</a> · <a href="/demos/item-rtl-he.html">HE</a> · <a href="/demos/item-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [item-rtl.html]
<div class="flex w-full max-w-md flex-col gap-6" dir="rtl">
  <div
    data-slot="item"
    data-variant="outline"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
    dir="rtl"
  >
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        عنصر أساسي
      </div>
      <p
        data-slot="item-description"
        class="text-muted-foreground text-start text-sm leading-normal group-data-[size=xs]/item:text-xs line-clamp-2 font-normal [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        عنصر بسيط يحتوي على عنوان ووصف.
      </p>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
      <button
        data-slot="button"
        data-variant="outline"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pe-1.5 has-data-[icon=inline-start]:ps-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
      >
        إجراء
      </button>
    </div>
  </div>
  <a
    href="#"
    data-slot="item"
    data-variant="outline"
    data-size="sm"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors border-border gap-2.5 px-3 py-2.5"
    dir="rtl"
    ><div
      data-slot="item-media"
      data-variant="default"
      class="gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start flex shrink-0 items-center justify-center [&amp;_svg]:pointer-events-none bg-transparent"
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
        class="lucide lucide-badge-check size-5"
      >
        <path
          d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"
        ></path>
        <path d="m9 12 2 2 4-4"></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center"
      >
        تم التحقق من ملفك الشخصي.
      </div>
    </div>
    <div data-slot="item-actions" class="gap-2 flex items-center">
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
        class="lucide lucide-chevron-right size-4"
      >
        <path d="m9 18 6-6-6-6"></path>
      </svg></div
  ></a>
</div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="item-group"` |
| `data-slot="item-separator"` |
| `data-slot="item"` |
| `data-slot="item-media"` |
| `data-slot="item-content"` |
| `data-slot="item-title"` |
| `data-slot="item-description"` |
| `data-slot="item-actions"` |
| `data-slot="item-header"` |
| `data-slot="item-footer"` |

**Runtime:** no JavaScript — this is markup + CSS. Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — check `dist/css/item.css` for any other `data-*` selector on these slots.

| Slot | Attribute | Values | Default |
| --- | --- | --- | --- |
| `item` | `data-variant` | `default`, `outline`, `muted` | `default` |
| `item` | `data-size` | `default`, `sm`, `xs` | `default` |
| `item-media` | `data-variant` | `default`, `icon`, `image` | `default` |
