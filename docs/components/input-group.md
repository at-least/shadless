---
title: "Input Group"
description: "Add addons, buttons, and helper content to inputs."
---

# Input Group

Add addons, buttons, and helper content to inputs.

::::demo input-group-demo
<iframe class="demo" src="/demos/input-group-demo.html" title="input-group-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-demo.html]
<div
  data-slot="input-group"
  role="group"
  class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto max-w-xs"
>
  <input
    data-slot="input-group-control"
    class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
    placeholder="Search..."
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
    12 results
  </div>
</div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/input-group.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/input-group.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/input-group.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                    into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/input-group.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `align="outline"` (JSX prop) | `data-align="outline"` (markup) |
| `size="outline"` (JSX prop) | `data-size="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
input-group
├── input-group-control or input-group-control
├── input-group-addon
├── InputGroupButton
└── InputGroupText
```

## Align

Use the `align` prop on `InputGroupAddon` to position the addon relative to the input.

::: tip
For proper focus management, `InputGroupAddon` should always be placed after
`InputGroupInput` or `InputGroupTextarea` in the DOM. Use the `align` prop to
visually position the addon.
:::

### inline-start

Use `align="inline-start"` to position the addon at the start of the input. This is the default.

::::demo input-group-inline-start
<iframe class="demo" src="/demos/input-group-inline-start.html" title="input-group-inline-start" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-inline-start.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-inline-start.html]
<div
  role="group"
  data-slot="field"
  data-orientation="vertical"
  class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto max-w-sm"
>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    for="inline-start-input"
    >Input</label
  >
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      id="inline-start-input"
      placeholder="Search..."
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
        class="lucide lucide-search text-muted-foreground"
      >
        <circle cx="11" cy="11" r="8"></circle>
        <path d="m21 21-4.3-4.3"></path>
      </svg>
    </div>
  </div>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    Icon positioned at the start.
  </p>
</div>
```
:::

::::


### inline-end

Use `align="inline-end"` to position the addon at the end of the input.

::::demo input-group-inline-end
<iframe class="demo" src="/demos/input-group-inline-end.html" title="input-group-inline-end" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-inline-end.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-inline-end.html]
<div
  role="group"
  data-slot="field"
  data-orientation="vertical"
  class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto max-w-sm"
>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    for="inline-end-input"
    >Input</label
  >
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      id="inline-end-input"
      placeholder="Enter password"
      type="password"
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
        class="lucide lucide-eye-off"
      >
        <path
          d="M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49"
        ></path>
        <path d="M14.084 14.158a3 3 0 0 1-4.242-4.242"></path>
        <path
          d="M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143"
        ></path>
        <path d="m2 2 20 20"></path>
      </svg>
    </div>
  </div>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    Icon positioned at the end.
  </p>
</div>
```
:::

::::


### block-start

Use `align="block-start"` to position the addon above the input.

::::demo input-group-block-start
<iframe class="demo" src="/demos/input-group-block-start.html" title="input-group-block-start" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-block-start.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-block-start.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col max-w-sm"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="vertical"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
  >
    <label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="block-start-input"
      >Input</label
    >
    <div
      data-slot="input-group"
      role="group"
      class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto h-auto"
    >
      <input
        data-slot="input-group-control"
        class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
        id="block-start-input"
        placeholder="Enter your name"
      />
      <div
        role="group"
        data-slot="input-group-addon"
        data-align="block-start"
        class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pt-2 group-has-[&gt;input]/input-group:pt-2 [.border-b]:pb-2 order-first w-full justify-start"
      >
        <span
          class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
          >Full Name</span
        >
      </div>
    </div>
    <p
      data-slot="field-description"
      class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Header positioned above the input.
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
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="block-start-textarea"
      >Textarea</label
    >
    <div
      data-slot="input-group"
      role="group"
      class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
    >
      <textarea
        data-slot="input-group-control"
        class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 px-2.5 transition-colors md:text-sm flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent py-2 shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1 resize-none font-mono text-sm"
        id="block-start-textarea"
        placeholder="console.log('Hello, world!');"
      ></textarea>
      <div
        role="group"
        data-slot="input-group-addon"
        data-align="block-start"
        class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pt-2 group-has-[&gt;input]/input-group:pt-2 [.border-b]:pb-2 order-first w-full justify-start"
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
          class="lucide lucide-file-code text-muted-foreground"
        >
          <path d="M10 12.5 8 15l2 2.5"></path>
          <path d="m14 12.5 2 2.5-2 2.5"></path>
          <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
          <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z"></path></svg
        ><span
          class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none font-mono"
          >script.js</span
        ><button
          data-slot="button"
          data-variant="ghost"
          data-size="icon-xs"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[&gt;svg]:p-0 ml-auto"
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
            class="lucide lucide-copy"
          >
            <rect width="14" height="14" x="8" y="8" rx="2" ry="2"></rect>
            <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path></svg
          ><span class="sr-only">Copy</span>
        </button>
      </div>
    </div>
    <p
      data-slot="field-description"
      class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Header positioned above the textarea.
    </p>
  </div>
</div>
```
:::

::::


### block-end

Use `align="block-end"` to position the addon below the input.

::::demo input-group-block-end
<iframe class="demo" src="/demos/input-group-block-end.html" title="input-group-block-end" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-block-end.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-block-end.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col max-w-sm"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="vertical"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
  >
    <label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="block-end-input"
      >Input</label
    >
    <div
      data-slot="input-group"
      role="group"
      class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto h-auto"
    >
      <input
        data-slot="input-group-control"
        class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
        id="block-end-input"
        placeholder="Enter amount"
      />
      <div
        role="group"
        data-slot="input-group-addon"
        data-align="block-end"
        class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pb-2 group-has-[&gt;input]/input-group:pb-2 [.border-t]:pt-2 order-last w-full justify-start"
      >
        <span
          class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
          >USD</span
        >
      </div>
    </div>
    <p
      data-slot="field-description"
      class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Footer positioned below the input.
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
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="block-end-textarea"
      >Textarea</label
    >
    <div
      data-slot="input-group"
      role="group"
      class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
    >
      <textarea
        data-slot="input-group-control"
        class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 px-2.5 text-base transition-colors md:text-sm flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent py-2 shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1 resize-none"
        id="block-end-textarea"
        placeholder="Write a comment..."
      ></textarea>
      <div
        role="group"
        data-slot="input-group-addon"
        data-align="block-end"
        class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pb-2 group-has-[&gt;input]/input-group:pb-2 [.border-t]:pt-2 order-last w-full justify-start"
      >
        <span
          class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
          >0/280</span
        ><button
          data-slot="button"
          data-variant="default"
          data-size="sm"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none ml-auto"
          type="button"
        >
          Post
        </button>
      </div>
    </div>
    <p
      data-slot="field-description"
      class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Footer positioned below the textarea.
    </p>
  </div>
</div>
```
:::

::::


## Icon

::::demo input-group-icon
<iframe class="demo" src="/demos/input-group-icon.html" title="input-group-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-icon.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-icon.html]
<div class="grid w-full max-w-sm gap-6">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Search..."
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
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Enter your email"
      type="email"
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
        class="lucide lucide-mail"
      >
        <rect width="20" height="16" x="2" y="4" rx="2"></rect>
        <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
      </svg>
    </div>
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Card number"
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
        class="lucide lucide-credit-card"
      >
        <rect width="20" height="14" x="2" y="5" rx="2"></rect>
        <line x1="2" x2="22" y1="10" y2="10"></line>
      </svg>
    </div>
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
        class="lucide lucide-check"
      >
        <path d="M20 6 9 17l-5-5"></path>
      </svg>
    </div>
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Card number"
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
        class="lucide lucide-star"
      >
        <path
          d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"
        ></path></svg
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
        class="lucide lucide-info"
      >
        <circle cx="12" cy="12" r="10"></circle>
        <path d="M12 16v-4"></path>
        <path d="M12 8h.01"></path>
      </svg>
    </div>
  </div>
</div>
```
:::

::::


## Text

::::demo input-group-text
<iframe class="demo" src="/demos/input-group-text.html" title="input-group-text" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-text.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-text.html]
<div class="grid w-full max-w-sm gap-6">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-start"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pl-2 has-[&gt;button]:ml-[-0.3rem] has-[&gt;kbd]:ml-[-0.15rem] order-first"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
        >$</span
      >
    </div>
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="0.00"
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
        >USD</span
      >
    </div>
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-start"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pl-2 has-[&gt;button]:ml-[-0.3rem] has-[&gt;kbd]:ml-[-0.15rem] order-first"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
        >https://</span
      >
    </div>
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1 pl-0.5!"
      placeholder="example.com"
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
        >.com</span
      >
    </div>
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Enter your username"
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
        >@company.com</span
      >
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
      placeholder="Enter your message"
    ></textarea>
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="block-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pb-2 group-has-[&gt;input]/input-group:pb-2 [.border-t]:pt-2 order-last w-full justify-start"
    >
      <span
        class="gap-2 [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none text-xs text-muted-foreground"
        >120 characters left</span
      >
    </div>
  </div>
</div>
```
:::

::::


## Button

::::demo input-group-button
<iframe class="demo" src="/demos/input-group-button.html" title="input-group-button" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-button.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-button.html]
<div class="grid w-full max-w-sm gap-6">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="https://x.com/shadcn"
      readonly=""
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <button
        data-slot="button"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[&gt;svg]:p-0"
        type="button"
        aria-label="Copy"
        title="Copy"
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
          class="tabler-icon tabler-icon-copy"
        >
          <path
            d="M7 9.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666"
          ></path>
          <path
            d="M4.012 16.737a2.005 2.005 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1"
          ></path>
        </svg>
      </button>
    </div>
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto [--radius:9999px]"
  >
    <div
      role="group"
      data-slot="popover-trigger"
      data-align="inline-start"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pl-2 has-[&gt;button]:ml-[-0.3rem] has-[&gt;kbd]:ml-[-0.15rem] order-first"
      type="button"
      aria-haspopup="dialog"
      aria-expanded="false"
      aria-controls="k0"
      data-state="closed"
      id="k0-trigger"
    >
      <button
        data-slot="button"
        data-variant="secondary"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[&gt;svg]:p-0"
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
          class="tabler-icon tabler-icon-info-circle"
        >
          <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0"></path>
          <path d="M12 9h.01"></path>
          <path d="M11 12h1v4h1"></path>
        </svg>
      </button>
    </div>
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-start"
      class="h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none has-[&gt;button]:ml-[-0.3rem] has-[&gt;kbd]:ml-[-0.15rem] order-first pl-1.5 text-muted-foreground"
    >
      https://
    </div>
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      id="input-secure-19"
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <button
        data-slot="button"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[&gt;svg]:p-0"
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
          class="tabler-icon tabler-icon-star data-[favorite=true]:fill-blue-600 data-[favorite=true]:stroke-blue-600"
          data-favorite="false"
        >
          <path
            d="M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873l-6.158 -3.245"
          ></path>
        </svg>
      </button>
    </div>
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Type to search..."
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <button
        data-slot="button"
        data-variant="secondary"
        data-size="xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 text-sm flex items-center shadow-none h-6 gap-1 rounded-[calc(var(--radius)-3px)] px-1.5 [&amp;&gt;svg:not([class*='size-'])]:size-3.5"
        type="button"
      >
        Search
      </button>
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
      transform: translate(8px, 99px);
      min-width: max-content;
      --radix-popper-transform-origin: 0% 0px;
      z-index: auto;
      --radix-popper-available-width: 1280px;
      --radix-popper-available-height: 621px;
      --radix-popper-anchor-width: 1264px;
      --radix-popper-anchor-height: 33px;
    "
  >
    <div
      data-side="bottom"
      data-align="start"
      data-state="open"
      role="dialog"
      id="k0"
      data-slot="popover-content"
      class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 p-2.5 shadow-md ring-1 duration-100 z-50 w-72 origin-(--radix-popover-content-transform-origin) outline-hidden flex flex-col gap-1 rounded-xl text-sm"
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
      <p class="font-medium">Your connection is not secure.</p>
      <p>You should not enter any sensitive information on this site.</p>
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


## Kbd

::::demo input-group-kbd
<iframe class="demo" src="/demos/input-group-kbd.html" title="input-group-kbd" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-kbd.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-kbd.html]
<div
  data-slot="input-group"
  role="group"
  class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto max-w-sm"
>
  <input
    data-slot="input-group-control"
    class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
    placeholder="Search..."
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
      class="lucide lucide-search text-muted-foreground"
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
      >⌘K</kbd
    >
  </div>
</div>
```
:::

::::


## Dropdown

::::demo input-group-dropdown
<iframe class="demo" src="/demos/input-group-dropdown.html" title="input-group-dropdown" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-dropdown.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-dropdown.html]
<div class="grid w-full max-w-sm gap-4">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Enter file name"
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <button
        data-slot="dropdown-menu-trigger"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[&gt;svg]:p-0"
        type="button"
        aria-label="More"
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
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto [--radius:1rem]"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Enter search query"
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <button
        data-slot="dropdown-menu-trigger"
        data-variant="ghost"
        data-size="xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 flex items-center shadow-none h-6 gap-1 rounded-[calc(var(--radius)-3px)] px-1.5 [&amp;&gt;svg:not([class*='size-'])]:size-3.5 pr-1.5! text-xs"
        type="button"
        id="m1-trigger"
        aria-haspopup="menu"
        aria-expanded="false"
        data-state="closed"
        data-radixuigo-menu-trigger="m1"
      >
        Search In...
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
          class="lucide lucide-chevron-down size-3"
        >
          <path d="m6 9 6 6 6-6"></path>
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
        Settings
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
        Copy path
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
        Open location
      </div>
    </div>
  </div>
</template>
<template id="m1-tpl">
  <div
    data-side="bottom"
    data-align="end"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m1"
    aria-labelledby="m1-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) w-(--radix-dropdown-menu-trigger-width) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden [--radius:0.95rem]"
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
        Blog Posts
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
        Changelog
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
// finds the listeners already there).
(function () {
  if (shadless.__menuWired && shadless.__menuWired["menu"]) return
  shadless.__menuWired = shadless.__menuWired || {}
  shadless.__menuWired["menu"] = true
  shadless.register("dropdown-menu", { init: function (root) {
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
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (t.hasAttribute("data-radixuigo-context-trigger")) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: "dropdown-menu",
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


## Spinner

::::demo input-group-spinner
<iframe class="demo" src="/demos/input-group-spinner.html" title="input-group-spinner" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-spinner.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-spinner.html]
<div class="grid w-full max-w-sm gap-4">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Searching..."
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
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Processing..."
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
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Saving changes..."
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
        >Saving...</span
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
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
      </svg>
    </div>
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="Refreshing data..."
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
        class="lucide lucide-loader animate-spin"
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
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pr-2 has-[&gt;button]:mr-[-0.3rem] has-[&gt;kbd]:mr-[-0.15rem] order-last"
    >
      <span
        class="gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none text-muted-foreground"
        >Please wait...</span
      >
    </div>
  </div>
</div>
```
:::

::::


## Textarea

::::demo input-group-textarea
<iframe class="demo" src="/demos/input-group-textarea.html" title="input-group-textarea" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-textarea.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-textarea.html]
<div class="grid w-full max-w-md gap-4">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <textarea
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 px-2.5 text-base transition-colors md:text-sm flex field-sizing-content w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent py-2 shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1 resize-none min-h-[200px]"
      id="textarea-code-32"
      placeholder="console.log('Hello, world!');"
    ></textarea>
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="block-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pb-2 group-has-[&gt;input]/input-group:pb-2 [.border-t]:pt-2 order-last w-full justify-start border-t"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
        >Line 1, Column 1</span
      ><button
        data-slot="button"
        data-variant="default"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none ml-auto"
        type="button"
      >
        Run
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
          class="tabler-icon tabler-icon-corner-down-left"
        >
          <path d="M18 6v6a3 3 0 0 1 -3 3h-10l4 -4m0 8l-4 -4"></path>
        </svg>
      </button>
    </div>
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="block-start"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pt-2 group-has-[&gt;input]/input-group:pt-2 [.border-b]:pb-2 order-first w-full justify-start border-b"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none font-mono font-medium"
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
          class="tabler-icon tabler-icon-brand-javascript"
        >
          <path d="M20 4l-2 14.5l-6 2l-6 -2l-2 -14.5l16 0"></path>
          <path d="M7.5 8h3v8l-2 -1"></path>
          <path
            d="M16.5 8h-2.5a.5 .5 0 0 0 -.5 .5v3a.5 .5 0 0 0 .5 .5h1.423a.5 .5 0 0 1 .495 .57l-.418 2.93l-2 .5"
          ></path></svg
        >script.js</span
      ><button
        data-slot="button"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[&gt;svg]:p-0 ml-auto"
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
          class="tabler-icon tabler-icon-refresh"
        >
          <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4"></path>
          <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 4v-4h-4"></path>
        </svg></button
      ><button
        data-slot="button"
        data-variant="ghost"
        data-size="icon-xs"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[&gt;svg]:p-0"
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
          class="tabler-icon tabler-icon-copy"
        >
          <path
            d="M7 9.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666"
          ></path>
          <path
            d="M4.012 16.737a2.005 2.005 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1"
          ></path>
        </svg>
      </button>
    </div>
  </div>
</div>
```
:::

::::


## Custom Input

Add the `data-slot="input-group-control"` attribute to your custom input for automatic focus state handling.

Here's an example of a custom resizable textarea from a third-party library.

::::demo input-group-custom
<iframe class="demo" src="/demos/input-group-custom.html" title="input-group-custom" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-custom.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [input-group-custom.html]
<div class="grid w-full max-w-sm gap-6">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pr-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:pl-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
  >
    <textarea
      data-slot="input-group-control"
      class="flex field-sizing-content min-h-16 w-full resize-none rounded-md bg-transparent px-3 py-2.5 text-base transition-[color,box-shadow] outline-none md:text-sm"
      placeholder="Autoresize textarea..."
    ></textarea>
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="block-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pb-2 group-has-[&gt;input]/input-group:pb-2 [.border-t]:pt-2 order-last w-full justify-start"
    >
      <button
        data-slot="button"
        data-variant="default"
        data-size="sm"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none ml-auto"
        type="button"
      >
        Submit
      </button>
    </div>
  </div>
</div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo input-group-rtl
<iframe class="demo" src="/demos/input-group-rtl.html" title="input-group-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/input-group-rtl.html">Open the demo page</a> · <a href="/demos/input-group-rtl-he.html">HE</a> · <a href="/demos/input-group-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [input-group-rtl.html]
<div class="grid w-full max-w-sm gap-6">
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pe-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:ps-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto max-w-xs"
    dir="rtl"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="بحث..."
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-start"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none ps-2 has-[&gt;button]:ms-[-0.3rem] has-[&gt;kbd]:ms-[-0.15rem] order-first"
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
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pe-2 has-[&gt;button]:me-[-0.3rem] has-[&gt;kbd]:me-[-0.15rem] order-last"
    >
      ١٢ نتيجة
    </div>
  </div>
  <div
    data-slot="input-group"
    role="group"
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pe-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:ps-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
    dir="rtl"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="جاري البحث..."
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pe-2 has-[&gt;button]:me-[-0.3rem] has-[&gt;kbd]:me-[-0.15rem] order-last"
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
    class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pe-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:ps-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
    dir="rtl"
  >
    <input
      data-slot="input-group-control"
      class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1"
      placeholder="جاري حفظ التغييرات..."
    />
    <div
      role="group"
      data-slot="input-group-addon"
      data-align="inline-end"
      class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center justify-center select-none pe-2 has-[&gt;button]:me-[-0.3rem] has-[&gt;kbd]:me-[-0.15rem] order-last"
    >
      <span
        class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
        >جاري الحفظ...</span
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
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
      </svg>
    </div>
  </div>
  <div
    data-slot="field-group"
    class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col max-w-sm"
    dir="rtl"
  >
    <div
      role="group"
      data-slot="field"
      data-orientation="vertical"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="rtl-textarea"
        >منطقة النص</label
      >
      <div
        data-slot="input-group"
        role="group"
        class="group/input-group border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 has-disabled:bg-input/50 dark:has-disabled:bg-input/80 h-8 rounded-lg border transition-colors in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[&gt;[data-align=block-end]]:h-auto has-[&gt;[data-align=block-end]]:flex-col has-[&gt;[data-align=block-start]]:h-auto has-[&gt;[data-align=block-start]]:flex-col has-[&gt;[data-align=block-end]]:[&amp;&gt;input]:pt-3 has-[&gt;[data-align=block-start]]:[&amp;&gt;input]:pb-3 has-[&gt;[data-align=inline-end]]:[&amp;&gt;input]:pe-1.5 has-[&gt;[data-align=inline-start]]:[&amp;&gt;input]:ps-1.5 relative flex w-full min-w-0 items-center outline-none has-[&gt;textarea]:h-auto"
        dir="rtl"
      >
        <textarea
          data-slot="input-group-control"
          class="border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 px-2.5 text-base transition-colors md:text-sm flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 rounded-none border-0 bg-transparent py-2 shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent flex-1 resize-none"
          id="rtl-textarea"
          placeholder="اكتب تعليقًا..."
        ></textarea>
        <div
          role="group"
          data-slot="input-group-addon"
          data-align="block-end"
          class="text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&amp;&gt;kbd]:rounded-[calc(var(--radius)-5px)] [&amp;&gt;svg:not([class*='size-'])]:size-4 flex cursor-text items-center select-none px-2.5 pb-2 group-has-[&gt;input]/input-group:pb-2 [.border-t]:pt-2 order-last w-full justify-start"
        >
          <span
            class="text-muted-foreground gap-2 text-sm [&amp;_svg:not([class*='size-'])]:size-4 flex items-center [&amp;_svg]:pointer-events-none"
            >٠/٢٨٠</span
          ><button
            data-slot="button"
            data-variant="default"
            data-size="sm"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button shrink-0 justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 gap-2 text-sm flex items-center shadow-none ml-auto"
            type="button"
          >
            نشر
          </button>
        </div>
      </div>
      <p
        data-slot="field-description"
        class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        تذييل موضع أسفل منطقة النص.
      </p>
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
| `data-slot="input-group"` |
| `data-slot="input-group-addon"` |
| `data-slot="input-group-control"` |

### InputGroup

The main component that wraps inputs and addons.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<InputGroup>
  <InputGroupInput />
  <InputGroupAddon />
</InputGroup>
```

### InputGroupAddon

Displays icons, text, buttons, or other content alongside inputs.

::: tip
} title="Focus Navigation">
For proper focus navigation, the `InputGroupAddon` component should be placed
after the input. Set the `align` prop to position the addon.
:::

| Prop        | Type                                                             | Default          |
| ----------- | ---------------------------------------------------------------- | ---------------- |
| `align`     | `"inline-start" \| "inline-end" \| "block-start" \| "block-end"` | `"inline-start"` |
| `className` | `string`                                                         |                  |

```tsx
<InputGroupAddon align="inline-end">
  <SearchIcon />
</InputGroupAddon>
```

**For `<InputGroupInput />`, use the `inline-start` or `inline-end` alignment. For `<InputGroupTextarea />`, use the `block-start` or `block-end` alignment.**

The `InputGroupAddon` component can have multiple `InputGroupButton` components and icons.

```tsx
<InputGroupAddon>
  <InputGroupButton>Button</InputGroupButton>
  <InputGroupButton>Button</InputGroupButton>
</InputGroupAddon>
```

### InputGroupButton

Displays buttons within input groups.

| Prop        | Type                                                                          | Default   |
| ----------- | ----------------------------------------------------------------------------- | --------- |
| `size`      | `"xs" \| "icon-xs" \| "sm" \| "icon-sm"`                                      | `"xs"`    |
| `variant`   | `"default" \| "destructive" \| "outline" \| "secondary" \| "ghost" \| "link"` | `"ghost"` |
| `className` | `string`                                                                      |           |

```tsx
<InputGroupButton>Button</InputGroupButton>
<InputGroupButton size="icon-xs" aria-label="Copy">
  <CopyIcon />
</InputGroupButton>
```

### InputGroupInput

Replacement for `<Input />` when building input groups. This component has the input group styles pre-applied and uses the unified `data-slot="input-group-control"` for focus state handling.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

All other props are passed through to the underlying `<Input />` component.

```tsx
<InputGroup>
  <InputGroupInput placeholder="Enter text..." />
  <InputGroupAddon>
    <SearchIcon />
  </InputGroupAddon>
</InputGroup>
```

### InputGroupTextarea

Replacement for `<Textarea />` when building input groups. This component has the textarea group styles pre-applied and uses the unified `data-slot="input-group-control"` for focus state handling.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

All other props are passed through to the underlying `<Textarea />` component.

```tsx
<InputGroup>
  <InputGroupTextarea placeholder="Enter message..." />
  <InputGroupAddon align="block-end">
    <InputGroupButton>Send</InputGroupButton>
  </InputGroupAddon>
</InputGroup>
```

## Changelog

### 2025-10-06 `InputGroup`

Add the `min-w-0` class to the `InputGroup` component. See [diff](https://github.com/shadcn-ui/ui/pull/8341/files#diff-0e2ee95d0050ca4c5d82339df86c54e14a6739dc4638fdda0eec8f73aebc2da9).
