---
title: "Spinner"
description: "An indicator that can be used to show a loading state."
weight: 44
---

# Spinner

An indicator that can be used to show a loading state.

{% <demo name="spinner-demo" status="authored"> %}
<iframe class="demo" src="/demos/spinner-demo.html" title="spinner-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/spinner-demo.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=spinner-demo.html
<div class="flex w-full max-w-xs flex-col gap-4 [--radius:1rem]">
  <div
    data-slot="item"
    data-variant="muted"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors bg-muted/50 border-transparent gap-2.5 px-3 py-2.5"
  >
    <div
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
        class="lucide lucide-loader-circle size-4 animate-spin"
        data-slot="spinner"
        role="status"
        aria-label="Loading"
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 w-fit items-center line-clamp-1"
      >
        Processing payment...
      </div>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-col [&amp;+[data-slot=item-content]]:flex-none flex-none justify-end"
    >
      <span class="text-sm tabular-nums">$100.00</span>
    </div>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Installation

**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/spinner.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/spinner.css` | this component's slot styles (`@apply` source — your build compiles it) |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Customization

You can replace the default spinner icon with any other icon — swap the inner `<svg>` in the spinner markup for a different one.

{% <demo name="spinner-custom" status="authored"> %}
<iframe class="demo" src="/demos/spinner-custom.html" title="spinner-custom" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/spinner-custom.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=spinner-custom.html
<div class="flex items-center gap-4">
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
    class="lucide lucide-loader size-4 animate-spin"
    role="status"
    aria-label="Loading"
  >
    <path d="M12 2v4"></path>
    <path d="m16.2 7.8 2.9-2.9"></path>
    <path d="M18 12h4"></path>
    <path d="m16.2 16.2 2.9 2.9"></path>
    <path d="M12 18v4"></path>
    <path d="m4.9 19.1 2.9-2.9"></path>
    <path d="M2 12h4"></path>
    <path d="m4.9 4.9 2.9 2.9"></path>
  </svg>
</div>
```
{% </codegroup> %}

{% </demo> %}

```html showLineNumbers
<svg data-slot="spinner">
  <!-- swap this inner <svg> for a different icon's markup -->
  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="..."/></svg>
</svg>
```

The `size-4 animate-spin` styling ships as a `[data-slot="spinner"]` CSS rule (`dist/css/spinner.css`), not an inline class — override it the same way, by targeting that selector.

## Size

Use the `size-*` utility class to change the size of the spinner.

{% <demo name="spinner-size" status="authored"> %}
<iframe class="demo" src="/demos/spinner-size.html" title="spinner-size" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/spinner-size.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=spinner-size.html
<div class="flex items-center gap-6">
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
    class="lucide lucide-loader-circle animate-spin size-3"
    data-slot="spinner"
    role="status"
    aria-label="Loading"
  >
    <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
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
    class="lucide lucide-loader-circle animate-spin size-4"
    data-slot="spinner"
    role="status"
    aria-label="Loading"
  >
    <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
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
    class="lucide lucide-loader-circle animate-spin size-6"
    data-slot="spinner"
    role="status"
    aria-label="Loading"
  >
    <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
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
    class="lucide lucide-loader-circle animate-spin size-8"
    data-slot="spinner"
    role="status"
    aria-label="Loading"
  >
    <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
  </svg>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Button

Add a spinner to a button to indicate a loading state. Place the `data-slot="spinner"` before the label with `data-icon="inline-start"` for a start position, or after the label with `data-icon="inline-end"` for an end position.

{% <demo name="spinner-button" status="authored"> %}
<iframe class="demo" src="/demos/spinner-button.html" title="spinner-button" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/spinner-button.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=spinner-button.html
<div class="flex flex-col items-center gap-4">
  <button
    data-slot="button"
    data-variant="default"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
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
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
    >Loading...</button
  ><button
    data-slot="button"
    data-variant="outline"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
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
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
    >Please wait</button
  ><button
    data-slot="button"
    data-variant="secondary"
    data-size="sm"
    class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
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
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
    >Processing
  </button>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Badge

Add a spinner to a badge to indicate a loading state. Place the `data-slot="spinner"` before the label with `data-icon="inline-start"` for a start position, or after the label with `data-icon="inline-end"` for an end position.

{% <demo name="spinner-badge" status="authored"> %}
<iframe class="demo" src="/demos/spinner-badge.html" title="spinner-badge" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/spinner-badge.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=spinner-badge.html
<div class="flex items-center gap-4 [--radius:1.2rem]">
  <span
    data-slot="badge"
    data-variant="default"
    class="h-5 gap-1 rounded-4xl border border-transparent px-2 py-0.5 text-xs font-medium transition-all has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;&gt;svg]:size-3! group/badge inline-flex w-fit shrink-0 items-center justify-center overflow-hidden whitespace-nowrap focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&amp;&gt;svg]:pointer-events-none bg-primary text-primary-foreground [a]:hover:bg-primary/80"
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
      data-icon="inline-start"
    >
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
    >Syncing</span
  ><span
    data-slot="badge"
    data-variant="secondary"
    class="h-5 gap-1 rounded-4xl border border-transparent px-2 py-0.5 text-xs font-medium transition-all has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;&gt;svg]:size-3! group/badge inline-flex w-fit shrink-0 items-center justify-center overflow-hidden whitespace-nowrap focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&amp;&gt;svg]:pointer-events-none bg-secondary text-secondary-foreground [a]:hover:bg-secondary/80"
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
      data-icon="inline-start"
    >
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
    >Updating</span
  ><span
    data-slot="badge"
    data-variant="outline"
    class="h-5 gap-1 rounded-4xl border px-2 py-0.5 text-xs font-medium transition-all has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;&gt;svg]:size-3! group/badge inline-flex w-fit shrink-0 items-center justify-center overflow-hidden whitespace-nowrap focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&amp;&gt;svg]:pointer-events-none border-border text-foreground [a]:hover:bg-muted [a]:hover:text-muted-foreground"
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
      data-icon="inline-start"
    >
      <path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg
    >Processing</span
  >
</div>
```
{% </codegroup> %}

{% </demo> %}

## Input Group

{% <demo name="spinner-input-group" status="authored"> %}
<iframe class="demo" src="/demos/spinner-input-group.html" title="spinner-input-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/spinner-input-group.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=spinner-input-group.html
<div class="flex w-full max-w-md flex-col gap-4">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Send a message..."
      disabled=""
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
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
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <textarea
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 px-2.5 text-base transition-colors md:text-sm flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent py-2 shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1 resize-none"
      placeholder="Send a message..."
      disabled=""
    ></textarea>
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="block-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pb-2 group-has-[&gt;input]/input-group:pb-2 [.border-t]:pt-2 order-last w-full justify-start"
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
      Validating...<button
        data-slot="button"
        data-variant="default"
        data-size="xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 text-sm flex items-center shadow-none h-6 gap-1 rounded-[calc(var(--radius)-3px)] px-1.5 [&amp;&gt;svg:not([class*='size-'])]:size-3.5 ml-auto"
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
          class="lucide lucide-arrow-up"
        >
          <path d="m5 12 7-7 7 7"></path>
          <path d="M12 19V5"></path></svg
        ><span class="sr-only">Send</span>
      </button>
    </div>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Empty

{% <demo name="spinner-empty" status="authored"> %}
<iframe class="demo" src="/demos/spinner-empty.html" title="spinner-empty" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/spinner-empty.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=spinner-empty.html
<div
  data-slot="empty"
  class="gap-4 rounded-xl border-dashed p-6 flex min-w-0 flex-1 flex-col items-center justify-center text-center text-balance w-full"
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
        class="lucide lucide-loader-circle size-4 animate-spin"
        data-slot="spinner"
        role="status"
        aria-label="Loading"
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
      </svg>
    </div>
    <div data-slot="empty-title" class="text-sm font-medium tracking-tight cn-font-heading">
      Processing your request
    </div>
    <div
      data-slot="empty-description"
      class="text-sm/relaxed text-muted-foreground [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Please wait while we process your request. Do not refresh the page.
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
      Cancel
    </button>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

{% <demo name="spinner-rtl" status="authored"> %}
<iframe class="demo" src="/demos/spinner-rtl.html" title="spinner-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/spinner-rtl.html">Open the demo page</a> · <a href="/demos/spinner-rtl-he.html">HE</a> · <a href="/demos/spinner-rtl-en.html">EN</a></p>

{% <codegroup> %}
```text,name=spinner-rtl.html
<div class="flex w-full max-w-xs flex-col gap-4 [--radius:1rem]" dir="rtl">
  <div
    data-slot="item"
    data-variant="muted"
    data-size="default"
    class="[a]:hover:bg-muted rounded-lg border text-sm group/item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors bg-muted/50 border-transparent gap-2.5 px-3 py-2.5"
    dir="rtl"
  >
    <div
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
        class="lucide lucide-loader-circle size-4 animate-spin"
        data-slot="spinner"
        role="status"
        aria-label="Loading"
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
      </svg>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-1 flex-col [&amp;+[data-slot=item-content]]:flex-none"
    >
      <div
        data-slot="item-title"
        class="gap-2 text-sm leading-snug font-medium underline-offset-4 w-fit items-center line-clamp-1"
      >
        جاري معالجة الدفع...
      </div>
    </div>
    <div
      data-slot="item-content"
      class="gap-1 group-data-[size=xs]/item:gap-0 flex flex-col [&amp;+[data-slot=item-content]]:flex-none flex-none justify-end"
    >
      <span class="text-sm tabular-nums">١٠٠.٠٠ دولار</span>
    </div>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}
