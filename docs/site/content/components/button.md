---
title: "Button"
description: "Displays a button or a component that looks like a button."
weight: 10
---

# Button

Displays a button or a component that looks like a button.

{% <demo name="button-demo" status="authored"> %}
<iframe class="demo" src="/demos/button-demo.html" title="button-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-demo.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-demo.html
<div class="flex flex-wrap items-center gap-2 md:flex-row">
  <button
    data-slot="button"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
  >
    Button</button
  ><button
    data-slot="button"
    data-variant="outline"
    data-size="icon"
    class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
    aria-label="Submit"
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
      class="lucide lucide-arrow-up"
    >
      <path d="m5 12 7-7 7 7"></path>
      <path d="M12 19V5"></path>
    </svg>
  </button>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Installation

**Add shadless and this component to your Tailwind v4 (>= 4.3) entry:**

```css
@import "shadless";
@import "shadless/button.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build (>= 4.3) |
| `dist/css/button.css` | this component's slot styles (`@apply` source — your build compiles it) |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Cursor

Tailwind v4 [switched](https://tailwindcss.com/docs/upgrade-guide#buttons-use-the-default-cursor) from `cursor: pointer` to `cursor: default` for the button component.

If you want to keep the `cursor: pointer` behavior, add the following code to your CSS file:

shadless has no CLI, so the rule below is the whole of it — there is no flag that adds it for you.

```css showLineNumbers title="globals.css"
@layer base {
  button:not(:disabled),
  [role="button"]:not(:disabled) {
    cursor: pointer;
  }
}
```

## Size

Use the `size` prop to change the size of the button.

{% <demo name="button-size" status="authored"> %}
<iframe class="demo" src="/demos/button-size.html" title="button-size" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-size.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-size.html
<div class="flex flex-col items-start gap-8 sm:flex-row">
  <div class="flex items-start gap-2">
    <button
      data-slot="button"
      data-variant="outline"
      data-size="xs"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
    >
      Extra Small</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="icon-xs"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
      aria-label="Submit"
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
        class="lucide lucide-arrow-up-right"
      >
        <path d="M7 7h10v10"></path>
        <path d="M7 17 17 7"></path>
      </svg>
    </button>
  </div>
  <div class="flex items-start gap-2">
    <button
      data-slot="button"
      data-variant="outline"
      data-size="sm"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
    >
      Small</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="icon-sm"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
      aria-label="Submit"
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
        class="lucide lucide-arrow-up-right"
      >
        <path d="M7 7h10v10"></path>
        <path d="M7 17 17 7"></path>
      </svg>
    </button>
  </div>
  <div class="flex items-start gap-2">
    <button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
    >
      Default</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
      aria-label="Submit"
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
        class="lucide lucide-arrow-up-right"
      >
        <path d="M7 7h10v10"></path>
        <path d="M7 17 17 7"></path>
      </svg>
    </button>
  </div>
  <div class="flex items-start gap-2">
    <button
      data-slot="button"
      data-variant="outline"
      data-size="lg"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
    >
      Large</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="icon-lg"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
      aria-label="Submit"
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
        class="lucide lucide-arrow-up-right"
      >
        <path d="M7 7h10v10"></path>
        <path d="M7 17 17 7"></path>
      </svg>
    </button>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Default

{% <demo name="button-default" status="authored"> %}
<iframe class="demo" src="/demos/button-default.html" title="button-default" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-default.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-default.html
<button data-slot="button" data-variant="default" data-size="default" class="group/button">
  Button
</button>
```
{% </codegroup> %}

{% </demo> %}

## Outline

{% <demo name="button-outline" status="authored"> %}
<iframe class="demo" src="/demos/button-outline.html" title="button-outline" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-outline.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-outline.html
<button
  data-slot="button"
  data-variant="outline"
  data-size="default"
  class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
>
  Outline
</button>
```
{% </codegroup> %}

{% </demo> %}

## Secondary

{% <demo name="button-secondary" status="authored"> %}
<iframe class="demo" src="/demos/button-secondary.html" title="button-secondary" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-secondary.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-secondary.html
<button data-slot="button" data-variant="secondary" data-size="default" class="group/button">
  Secondary
</button>
```
{% </codegroup> %}

{% </demo> %}

## Ghost

{% <demo name="button-ghost" status="authored"> %}
<iframe class="demo" src="/demos/button-ghost.html" title="button-ghost" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-ghost.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-ghost.html
<button data-slot="button" data-variant="ghost" data-size="default" class="group/button">
  Ghost
</button>
```
{% </codegroup> %}

{% </demo> %}

## Destructive

{% <demo name="button-destructive" status="authored"> %}
<iframe class="demo" src="/demos/button-destructive.html" title="button-destructive" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-destructive.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-destructive.html
<button
  data-slot="button"
  data-variant="destructive"
  data-size="default"
  class="aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border-transparent group/button focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 focus-visible:border-destructive/40"
>
  Destructive
</button>
```
{% </codegroup> %}

{% </demo> %}

## Link

{% <demo name="button-link" status="authored"> %}
<iframe class="demo" src="/demos/button-link.html" title="button-link" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-link.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-link.html
<button data-slot="button" data-variant="link" data-size="default" class="group/button">
  Link
</button>
```
{% </codegroup> %}

{% </demo> %}

## Icon

{% <demo name="button-icon" status="authored"> %}
<iframe class="demo" src="/demos/button-icon.html" title="button-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-icon.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-icon.html
<button
  data-slot="button"
  data-variant="outline"
  data-size="icon"
  class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
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
    class="lucide lucide-circle-fading-arrow-up"
  >
    <path d="M12 2a10 10 0 0 1 7.38 16.75"></path>
    <path d="m16 12-4-4-4 4"></path>
    <path d="M12 16V8"></path>
    <path d="M2.5 8.875a10 10 0 0 0-.5 3"></path>
    <path d="M2.83 16a10 10 0 0 0 2.43 3.4"></path>
    <path d="M4.636 5.235a10 10 0 0 1 .891-.857"></path>
    <path d="M8.644 21.42a10 10 0 0 0 7.631-.38"></path>
  </svg>
</button>
```
{% </codegroup> %}

{% </demo> %}

## With Icon

Remember to add the `data-icon="inline-start"` or `data-icon="inline-end"` attribute to the icon for the correct spacing.

{% <demo name="button-with-icon" status="authored"> %}
<iframe class="demo" src="/demos/button-with-icon.html" title="button-with-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-with-icon.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-with-icon.html
<button
  data-slot="button"
  data-variant="outline"
  data-size="sm"
  class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
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
    class="tabler-icon tabler-icon-git-branch"
  >
    <path d="M5 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path>
    <path d="M5 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path>
    <path d="M15 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path>
    <path d="M7 8l0 8"></path>
    <path d="M9 18h6a2 2 0 0 0 2 -2v-5"></path>
    <path d="M14 14l3 -3l3 3"></path>
  </svg>
  New Branch
</button>
```
{% </codegroup> %}

{% </demo> %}

## Rounded

Use the `rounded-full` class to make the button rounded.

{% <demo name="button-rounded" status="authored"> %}
<iframe class="demo" src="/demos/button-rounded.html" title="button-rounded" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-rounded.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-rounded.html
<div class="flex flex-col gap-8">
  <button
    data-slot="button"
    data-variant="outline"
    data-size="icon"
    class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input rounded-full"
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
      class="lucide lucide-arrow-up"
    >
      <path d="m5 12 7-7 7 7"></path>
      <path d="M12 19V5"></path>
    </svg>
  </button>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Spinner

Render a `data-slot="spinner"` component inside the button to show a loading state. Remember to add the `data-icon="inline-start"` or `data-icon="inline-end"` attribute to the spinner for the correct spacing.

{% <demo name="button-spinner" status="authored"> %}
<iframe class="demo" src="/demos/button-spinner.html" title="button-spinner" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-spinner.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-spinner.html
<div class="flex gap-2">
  <button
    data-slot="button"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input"
    disabled=""
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
      class="lucide lucide-loader-circle"
      data-slot="spinner"
      role="status"
      aria-label="Loading"
      data-icon="inline-start"
    >
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
    >Generating</button
  ><button
    data-slot="button"
    data-variant="secondary"
    data-size="default"
    class="group/button"
    disabled=""
  >
    Downloading<svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="lucide lucide-loader-circle"
      data-slot="spinner"
      role="status"
      aria-label="Loading"
      data-icon="inline-start"
    >
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
    </svg>
  </button>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Button Group

To create a button group, use the `ButtonGroup` component. See the [Button Group](/components/button-group) documentation for more details.

{% <demo name="button-group-demo" status="authored"> %}
<iframe class="demo" src="/demos/button-group-demo.html" title="button-group-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-group-demo.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-group-demo.html
<div
  role="group"
  data-slot="button-group"
  class="has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1 [&>[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none"
>
  <div
    role="group"
    data-slot="button-group"
    class="has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1 [&>[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none hidden sm:flex"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border [&_svg:not([class*='size-'])]:size-4 group/button shrink-0 [&_svg]:shrink-0 border-border dark:border-input size-8"
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
    class="has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1 [&>[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border [&_svg:not([class*='size-'])]:size-4 group/button shrink-0 [&_svg]:shrink-0 border-border dark:border-input"
    >
      Archive</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border [&_svg:not([class*='size-'])]:size-4 group/button shrink-0 [&_svg]:shrink-0 border-border dark:border-input"
    >
      Report
    </button>
  </div>
  <div
    role="group"
    data-slot="button-group"
    class="has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-r-lg group/button-group [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1 [&>[data-slot]:not(:has(~[data-slot]))]:rounded-r-lg! [&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none"
  >
    <button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border [&_svg:not([class*='size-'])]:size-4 group/button shrink-0 [&_svg]:shrink-0 border-border dark:border-input"
    >
      Snooze</button
    ><button
      data-slot="dropdown-menu-trigger"
      data-variant="outline"
      data-size="icon"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
    class="cn-menu-target cn-menu-translucent w-40"
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
        class="group/dropdown-menu-item"
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
        class="group/dropdown-menu-item"
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
    <div role="separator" aria-orientation="horizontal" data-slot="dropdown-menu-separator"></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="group/dropdown-menu-item"
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
        class="group/dropdown-menu-item"
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
        class="group/dropdown-menu-item"
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
    <div role="separator" aria-orientation="horizontal" data-slot="dropdown-menu-separator"></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="destructive"
        class="group/dropdown-menu-item"
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
    class="cn-menu-target cn-menu-translucent"
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
        data-state="checked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span data-slot="dropdown-menu-radio-item-indicator"
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
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span data-slot="dropdown-menu-radio-item-indicator"></span>Work
      </div>
      <div
        role="menuitemradio"
        aria-checked="false"
        data-slot="dropdown-menu-radio-item"
        data-state="unchecked"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        <span data-slot="dropdown-menu-radio-item-indicator"></span>Other
      </div>
    </div>
  </div>
</template>
<template data-for="menu-item-indicator">
  <span data-state="checked"
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
      <path d="M20 6 9 17l-5-5"></path></svg
  ></span>
</template>
```

```js,name=behavior
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
{% </codegroup> %}

{% </demo> %}

## As Child

You can use the `asChild` prop on `data-slot="button"` to make another component look like a button. Here's an example of a link that looks like a button.

{% <demo name="button-aschild" status="authored"> %}
<iframe class="demo" src="/demos/button-aschild.html" title="button-aschild" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-aschild.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=button-aschild.html
<a href="/login" data-slot="button" data-variant="default" data-size="default" class="group/button"
  >Login</a
>
```
{% </codegroup> %}

{% </demo> %}

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

{% <demo name="button-rtl" status="authored"> %}
<iframe class="demo" src="/demos/button-rtl.html" title="button-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/button-rtl.html">Open the demo page</a> · <a href="/demos/button-rtl-he.html">HE</a> · <a href="/demos/button-rtl-en.html">EN</a></p>

{% <codegroup> %}
```text,name=button-rtl.html
<div class="flex flex-wrap items-center gap-2 md:flex-row" dir="rtl">
  <button
    data-slot="button"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
  >
    زر</button
  ><button
    data-slot="button"
    data-variant="destructive"
    data-size="default"
    class="aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 bg-destructive/10 hover:bg-destructive/20 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/20 text-destructive focus-visible:border-destructive/40 dark:hover:bg-destructive/30 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
  >
    حذف</button
  ><button
    data-slot="button"
    data-variant="outline"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
  >
    إرسال
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
      class="lucide lucide-arrow-right rtl:rotate-180"
      data-icon="inline-end"
    >
      <path d="M5 12h14"></path>
      <path d="m12 5 7 7-7 7"></path>
    </svg></button
  ><button
    data-slot="button"
    data-variant="outline"
    data-size="icon"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
    aria-label="Add"
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
    data-variant="secondary"
    data-size="default"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
    disabled=""
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
      data-icon="inline-start"
    >
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
    </svg>
    جاري التحميل
  </button>
</div>
```
{% </codegroup> %}

{% </demo> %}

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="button"` |

**Runtime:** no JavaScript — this is markup + CSS. Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — the component stylesheet (`shadless/button.css`) embedded below carries any other `data-*` selector on these slots.

| Slot | Attribute | Values | Default |
| --- | --- | --- | --- |
| `button` | `data-variant` | `default`, `outline`, `secondary`, `ghost`, `destructive`, `link` | `default` |
| `button` | `data-size` | `default`, `xs`, `sm`, `lg`, `icon`, `icon-xs`, `icon-sm`, `icon-lg` | `default` |

**The stylesheet** — `shadless/button.css`, the exact bytes of `dist/css/button.css`, verbatim. Every rule keys on the `data-slot` / `data-*` attributes in the tables above, so this is the authoritative default look of each slot:

```css
/* button */
@layer components {
  [data-slot="button"] { @apply focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0; }
  [data-slot="button"]:where(:not([data-variant]), [data-variant="default"], [data-variant=""]) { @apply bg-primary text-primary-foreground hover:bg-primary/80; }
  [data-slot="button"]:where([data-variant="outline"]) { @apply border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground; }
  [data-slot="button"]:where([data-variant="secondary"]) { @apply bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground; }
  [data-slot="button"]:where([data-variant="ghost"]) { @apply hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground; }
  [data-slot="button"]:where([data-variant="destructive"]) { @apply bg-destructive/10 hover:bg-destructive/20 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/20 text-destructive focus-visible:border-destructive/40 dark:hover:bg-destructive/30; }
  [data-slot="button"]:where([data-variant="link"]) { @apply text-primary underline-offset-4 hover:underline; }
  [data-slot="button"]:where(:not([data-size]), [data-size="default"], [data-size=""]) { @apply h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2; }
  [data-slot="button"]:where([data-size="xs"]) { @apply h-6 gap-1 rounded-[min(var(--radius-md),10px)] px-2 text-xs in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3; }
  [data-slot="button"]:where(.text-\[0\.8rem\]) { @apply leading-[inherit]; }
  [data-slot="button"]:where([data-size="sm"]) { @apply h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5 leading-[inherit]; }
  [data-slot="button"]:where([data-size="lg"]) { @apply h-9 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2; }
  [data-slot="button"]:where([data-size="icon"]) { @apply size-8; }
  [data-slot="button"]:where([data-size="icon-xs"]) { @apply size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&_svg:not([class*='size-'])]:size-3; }
  [data-slot="button"]:where([data-size="icon-sm"]) { @apply size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg; }
  [data-slot="button"]:where([data-size="icon-lg"]) { @apply size-9; }
}
```
