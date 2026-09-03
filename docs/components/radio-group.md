---
title: "Radio Group"
description: "A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time."
---

# Radio Group

A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/radio-group" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/radio-group#api-reference" rel="noopener">api</a></p>

::::demo radio-group-demo
<iframe class="demo" src="/demos/radio-group-demo.html" title="radio-group-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/radio-group-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [radio-group-demo.html]
<div
  role="radiogroup"
  aria-required="false"
  dir="ltr"
  data-slot="radio-group"
  class="grid gap-2 w-fit"
  tabindex="0"
  style="outline: none"
>
  <div class="flex items-center gap-3">
    <button
      type="button"
      role="radio"
      aria-checked="false"
      data-state="unchecked"
      value="default"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="r1"
      tabindex="-1"
      data-radix-collection-item=""
    ></button
    ><label
      data-slot="label"
      class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
      for="r1"
      >Default</label
    >
  </div>
  <div class="flex items-center gap-3">
    <button
      type="button"
      role="radio"
      aria-checked="true"
      data-state="checked"
      value="comfortable"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="r2"
      tabindex="-1"
      data-radix-collection-item=""
    >
      <span
        data-state="checked"
        data-slot="radio-group-indicator"
        class="flex size-4 items-center justify-center"
        ><span
          class="bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full"
        ></span
      ></span></button
    ><label
      data-slot="label"
      class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
      for="r2"
      >Comfortable</label
    >
  </div>
  <div class="flex items-center gap-3">
    <button
      type="button"
      role="radio"
      aria-checked="false"
      data-state="unchecked"
      value="compact"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="r3"
      tabindex="-1"
      data-radix-collection-item=""
    ></button
    ><label
      data-slot="label"
      class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
      for="r3"
      >Compact</label
    >
  </div>
</div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/radio-group.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/radio-group.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/radio-group.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/radio-group.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/radio-group.js"></script>
```

**Copy the markup from `dist/components/radio-group.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `role="radiogroup"` root; items are `role="radio"` with `aria-checked` + `data-state`, the checked one carries the `radio-group-indicator` mounted from `<template data-for="radio-group-indicator">`; an item's value is its `value` / `data-value` attribute or id. Keys: arrows / Home / End move focus over enabled items (rtl-aware); with nothing checked an arrow also checks its target (radix).

The root dispatches `shadless:change` (`detail: { value, item }`), bubbling, after the state change, whichever path caused it.

Forms: a `name` attribute on the root submits the checked item's value; `form.reset()` restores the initial state.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/radio-group.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
radio-group
├── radio-group-item
└── radio-group-item
```

## Description

Radio group items with a description using the `Field` component.

::::demo radio-group-description
<iframe class="demo" src="/demos/radio-group-description.html" title="radio-group-description" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/radio-group-description.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [radio-group-description.html]
<div
  role="radiogroup"
  aria-required="false"
  dir="ltr"
  data-slot="radio-group"
  class="grid gap-2 w-fit"
  tabindex="0"
  style="outline: none"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="radio"
      aria-checked="false"
      data-state="unchecked"
      value="default"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="desc-r1"
      tabindex="-1"
      data-radix-collection-item=""
    ></button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="desc-r1"
        >Default</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Standard spacing for most use cases.
      </p>
    </div>
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="radio"
      aria-checked="true"
      data-state="checked"
      value="comfortable"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="desc-r2"
      tabindex="-1"
      data-radix-collection-item=""
    >
      <span
        data-state="checked"
        data-slot="radio-group-indicator"
        class="flex size-4 items-center justify-center"
        ><span
          class="bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full"
        ></span
      ></span>
    </button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="desc-r2"
        >Comfortable</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        More space between elements.
      </p>
    </div>
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="radio"
      aria-checked="false"
      data-state="unchecked"
      value="compact"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="desc-r3"
      tabindex="-1"
      data-radix-collection-item=""
    ></button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="desc-r3"
        >Compact</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Minimal spacing for dense layouts.
      </p>
    </div>
  </div>
</div>
```
:::

::::


## Choice Card

Use `FieldLabel` to wrap the entire `Field` for a clickable card-style selection.

::::demo radio-group-choice-card
<iframe class="demo" src="/demos/radio-group-choice-card.html" title="radio-group-choice-card" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/radio-group-choice-card.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [radio-group-choice-card.html]
<div
  role="radiogroup"
  aria-required="false"
  dir="ltr"
  data-slot="radio-group"
  class="grid gap-2 w-full max-w-sm"
  tabindex="0"
  style="outline: none"
>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    for="plus-plan"
    ><div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <div
        data-slot="field-content"
        class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
      >
        <div
          data-slot="field-label"
          class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center"
        >
          Plus
        </div>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          For individuals and small teams.
        </p>
      </div>
      <button
        type="button"
        role="radio"
        aria-checked="true"
        data-state="checked"
        value="plus"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="plus-plan"
        tabindex="-1"
        data-radix-collection-item=""
      >
        <span
          data-state="checked"
          data-slot="radio-group-indicator"
          class="flex size-4 items-center justify-center"
          ><span
            class="bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full"
          ></span
        ></span>
      </button></div></label
  ><label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    for="pro-plan"
    ><div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <div
        data-slot="field-content"
        class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
      >
        <div
          data-slot="field-label"
          class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center"
        >
          Pro
        </div>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          For growing businesses.
        </p>
      </div>
      <button
        type="button"
        role="radio"
        aria-checked="false"
        data-state="unchecked"
        value="pro"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="pro-plan"
        tabindex="-1"
        data-radix-collection-item=""
      ></button></div></label
  ><label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    for="enterprise-plan"
    ><div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <div
        data-slot="field-content"
        class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
      >
        <div
          data-slot="field-label"
          class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center"
        >
          Enterprise
        </div>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          For large teams and enterprises.
        </p>
      </div>
      <button
        type="button"
        role="radio"
        aria-checked="false"
        data-state="unchecked"
        value="enterprise"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="enterprise-plan"
        tabindex="-1"
        data-radix-collection-item=""
      ></button></div
  ></label>
</div>
```
:::

::::


## Fieldset

Use `FieldSet` and `FieldLegend` to group radio items with a label and description.

::::demo radio-group-fieldset
<iframe class="demo" src="/demos/radio-group-fieldset.html" title="radio-group-fieldset" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/radio-group-fieldset.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [radio-group-fieldset.html]
<fieldset
  data-slot="field-set"
  class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col w-full max-w-xs"
>
  <legend
    data-slot="field-legend"
    data-variant="label"
    class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
  >
    Subscription Plan
  </legend>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    Yearly and lifetime plans offer significant savings.
  </p>
  <div
    role="radiogroup"
    aria-required="false"
    dir="ltr"
    data-slot="radio-group"
    class="grid gap-2 w-full"
    tabindex="0"
    style="outline: none"
  >
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="radio"
        aria-checked="true"
        data-state="checked"
        value="monthly"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="plan-monthly"
        tabindex="-1"
        data-radix-collection-item=""
      >
        <span
          data-state="checked"
          data-slot="radio-group-indicator"
          class="flex size-4 items-center justify-center"
          ><span
            class="bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full"
          ></span
        ></span></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="plan-monthly"
        >Monthly ($9.99/month)</label
      >
    </div>
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="radio"
        aria-checked="false"
        data-state="unchecked"
        value="yearly"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="plan-yearly"
        tabindex="-1"
        data-radix-collection-item=""
      ></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="plan-yearly"
        >Yearly ($99.99/year)</label
      >
    </div>
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="radio"
        aria-checked="false"
        data-state="unchecked"
        value="lifetime"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="plan-lifetime"
        tabindex="-1"
        data-radix-collection-item=""
      ></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="plan-lifetime"
        >Lifetime ($299.99)</label
      >
    </div>
  </div>
</fieldset>
```
:::

::::


## Disabled

Use the `disabled` prop on `RadioGroupItem` to disable individual items.

::::demo radio-group-disabled
<iframe class="demo" src="/demos/radio-group-disabled.html" title="radio-group-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/radio-group-disabled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [radio-group-disabled.html]
<div
  role="radiogroup"
  aria-required="false"
  dir="ltr"
  data-slot="radio-group"
  class="grid gap-2 w-fit"
  tabindex="0"
  style="outline: none"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    data-disabled="true"
  >
    <button
      type="button"
      role="radio"
      aria-checked="false"
      data-state="unchecked"
      data-disabled=""
      disabled=""
      value="option1"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="disabled-1"
      tabindex="-1"
      data-radix-collection-item=""
    ></button
    ><label
      data-slot="field-label"
      class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
      for="disabled-1"
      >Disabled</label
    >
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="radio"
      aria-checked="true"
      data-state="checked"
      value="option2"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="disabled-2"
      tabindex="-1"
      data-radix-collection-item=""
    >
      <span
        data-state="checked"
        data-slot="radio-group-indicator"
        class="flex size-4 items-center justify-center"
        ><span
          class="bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full"
        ></span
      ></span></button
    ><label
      data-slot="field-label"
      class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
      for="disabled-2"
      >Option 2</label
    >
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="radio"
      aria-checked="false"
      data-state="unchecked"
      value="option3"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="disabled-3"
      tabindex="-1"
      data-radix-collection-item=""
    ></button
    ><label
      data-slot="field-label"
      class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
      for="disabled-3"
      >Option 3</label
    >
  </div>
</div>
```
:::

::::


## Invalid

Use `aria-invalid` on `RadioGroupItem` and `data-invalid` on `Field` to show validation errors.

::::demo radio-group-invalid
<iframe class="demo" src="/demos/radio-group-invalid.html" title="radio-group-invalid" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/radio-group-invalid.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [radio-group-invalid.html]
<fieldset
  data-slot="field-set"
  class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col w-full max-w-xs"
>
  <legend
    data-slot="field-legend"
    data-variant="label"
    class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
  >
    Notification Preferences
  </legend>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    Choose how you want to receive notifications.
  </p>
  <div
    role="radiogroup"
    aria-required="false"
    dir="ltr"
    data-slot="radio-group"
    class="grid gap-2 w-full"
    tabindex="0"
    style="outline: none"
  >
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      data-invalid="true"
    >
      <button
        type="button"
        role="radio"
        aria-checked="true"
        data-state="checked"
        value="email"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="invalid-email"
        aria-invalid="true"
        tabindex="-1"
        data-radix-collection-item=""
      >
        <span
          data-state="checked"
          data-slot="radio-group-indicator"
          class="flex size-4 items-center justify-center"
          ><span
            class="bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full"
          ></span
        ></span></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="invalid-email"
        >Email only</label
      >
    </div>
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      data-invalid="true"
    >
      <button
        type="button"
        role="radio"
        aria-checked="false"
        data-state="unchecked"
        value="sms"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="invalid-sms"
        aria-invalid="true"
        tabindex="-1"
        data-radix-collection-item=""
      ></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="invalid-sms"
        >SMS only</label
      >
    </div>
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      data-invalid="true"
    >
      <button
        type="button"
        role="radio"
        aria-checked="false"
        data-state="unchecked"
        value="both"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="invalid-both"
        aria-invalid="true"
        tabindex="-1"
        data-radix-collection-item=""
      ></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="invalid-both"
        >Both Email &amp; SMS</label
      >
    </div>
  </div>
</fieldset>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo radio-group-rtl
<iframe class="demo" src="/demos/radio-group-rtl.html" title="radio-group-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/radio-group-rtl.html">Open the demo page</a> · <a href="/demos/radio-group-rtl-he.html">HE</a> · <a href="/demos/radio-group-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [radio-group-rtl.html]
<div
  role="radiogroup"
  aria-required="false"
  dir="rtl"
  data-slot="radio-group"
  class="grid gap-2 w-fit"
  tabindex="0"
  style="outline: none"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="radio"
      aria-checked="false"
      data-state="unchecked"
      value="default"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="r1-rtl"
      dir="rtl"
      tabindex="-1"
      data-radix-collection-item=""
    ></button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="r1-rtl"
        dir="rtl"
        >افتراضي</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        dir="rtl"
      >
        تباعد قياسي لمعظم حالات الاستخدام.
      </p>
    </div>
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="radio"
      aria-checked="true"
      data-state="checked"
      value="comfortable"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="r2-rtl"
      dir="rtl"
      tabindex="-1"
      data-radix-collection-item=""
    >
      <span
        data-state="checked"
        data-slot="radio-group-indicator"
        class="flex size-4 items-center justify-center"
        ><span
          class="bg-primary-foreground absolute top-1/2 start-1/2 size-2 -translate-x-1/2 rtl:translate-x-1/2 -translate-y-1/2 rounded-full"
        ></span
      ></span>
    </button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="r2-rtl"
        dir="rtl"
        >مريح</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        dir="rtl"
      >
        مساحة أكبر بين العناصر.
      </p>
    </div>
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="radio"
      aria-checked="false"
      data-state="unchecked"
      value="compact"
      data-slot="radio-group-item"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="r3-rtl"
      dir="rtl"
      tabindex="-1"
      data-radix-collection-item=""
    ></button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="r3-rtl"
        dir="rtl"
        >مضغوط</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        dir="rtl"
      >
        تباعد أدنى للتخطيطات الكثيفة.
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
| `data-slot="radio-group"` |
| `data-slot="radio-group-item"` |
| `data-slot="radio-group-indicator"` |

**Runtime:** `role="radiogroup"` root; items are `role="radio"` with `aria-checked` + `data-state`, the checked one carries the `radio-group-indicator` mounted from `<template data-for="radio-group-indicator">`; an item's value is its `value` / `data-value` attribute or id. Keys: arrows / Home / End move focus over enabled items (rtl-aware); with nothing checked an arrow also checks its target (radix). The root dispatches `shadless:change` (`detail: { value, item }`). Forms: a `name` attribute on the root submits the checked item's value. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix UI Radio Group](https://www.radix-ui.com/docs/primitives/components/radio-group#api-reference) documentation.
