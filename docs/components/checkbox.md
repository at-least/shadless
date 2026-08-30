---
title: "Checkbox"
description: "A control that allows the user to toggle between checked and not checked."
---

# Checkbox

A control that allows the user to toggle between checked and not checked.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/checkbox" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/checkbox#api-reference" rel="noopener">api</a></p>

::::demo checkbox-demo
<iframe class="demo" src="/demos/checkbox-demo.html" title="checkbox-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/checkbox-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [checkbox-demo.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col max-w-sm"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="checkbox"
      aria-checked="false"
      data-state="unchecked"
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="terms-checkbox"
    ></button
    ><label
      data-slot="label"
      class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
      for="terms-checkbox"
      >Accept terms and conditions</label
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
      role="checkbox"
      aria-checked="true"
      data-state="checked"
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="terms-checkbox-2"
    >
      <span
        data-state="checked"
        data-slot="checkbox-indicator"
        class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
        style="pointer-events: none"
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
    </button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="terms-checkbox-2"
        >Accept terms and conditions</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        By clicking this checkbox, you agree to the terms.
      </p>
    </div>
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    data-disabled="true"
  >
    <button
      type="button"
      role="checkbox"
      aria-checked="false"
      data-state="unchecked"
      data-disabled=""
      disabled=""
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="toggle-checkbox"
    ></button
    ><label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="toggle-checkbox"
      >Enable notifications</label
    >
  </div>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    ><div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="checkbox"
        aria-checked="false"
        data-state="unchecked"
        value="on"
        data-slot="checkbox"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="toggle-checkbox-2"
      ></button>
      <div
        data-slot="field-content"
        class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
      >
        <div
          data-slot="field-label"
          class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center"
        >
          Enable notifications
        </div>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          You can enable or disable notifications at any time.
        </p>
      </div>
    </div></label
  >
</div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/checkbox.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/checkbox.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/checkbox.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/checkbox.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/checkbox.js"></script>
```

**Copy the markup from                                 into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `role="checkbox"` root with `aria-checked` + `data-state="checked|unchecked"`; the `checkbox-indicator` mounts from a `<template data-for="checkbox-indicator">` while checked (radix Presence). Keys: Space / click toggles.

The root dispatches `shadless:change` (`detail: { checked }`), bubbling, after the state change, whichever path caused it.

Forms: a `name` attribute submits its `value` (default `on`) while checked; `form.reset()` restores the initial state.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/checkbox.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Checked State

Use `defaultChecked` for uncontrolled checkboxes, or `checked` and
`onCheckedChange` to control the state.



## Invalid State

Set `aria-invalid` on the checkbox and `data-invalid` on the field wrapper to
show the invalid styles.

::::demo checkbox-invalid
<iframe class="demo" src="/demos/checkbox-invalid.html" title="checkbox-invalid" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/checkbox-invalid.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [checkbox-invalid.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col mx-auto w-56"
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
      role="checkbox"
      aria-checked="false"
      data-state="unchecked"
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="terms-checkbox-invalid"
      aria-invalid="true"
    ></button
    ><label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="terms-checkbox-invalid"
      >Accept terms and conditions</label
    >
  </div>
</div>
```
:::

::::


## Basic

Pair the checkbox with `Field` and `FieldLabel` for proper layout and labeling.

::::demo checkbox-basic
<iframe class="demo" src="/demos/checkbox-basic.html" title="checkbox-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/checkbox-basic.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [checkbox-basic.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col mx-auto w-56"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="checkbox"
      aria-checked="false"
      data-state="unchecked"
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="terms-checkbox-basic"
    ></button
    ><label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="terms-checkbox-basic"
      >Accept terms and conditions</label
    >
  </div>
</div>
```
:::

::::


## Description

Use `FieldContent` and `FieldDescription` for helper text.

::::demo checkbox-description
<iframe class="demo" src="/demos/checkbox-description.html" title="checkbox-description" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/checkbox-description.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [checkbox-description.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col mx-auto w-72"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="checkbox"
      aria-checked="true"
      data-state="checked"
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="terms-checkbox-desc"
    >
      <span
        data-state="checked"
        data-slot="checkbox-indicator"
        class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
        style="pointer-events: none"
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
    </button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="terms-checkbox-desc"
        >Accept terms and conditions</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        By clicking this checkbox, you agree to the terms and conditions.
      </p>
    </div>
  </div>
</div>
```
:::

::::


## Disabled

Use the `disabled` prop to prevent interaction and add the `data-disabled` attribute to the `<Field>` component for disabled styles.

::::demo checkbox-disabled
<iframe class="demo" src="/demos/checkbox-disabled.html" title="checkbox-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/checkbox-disabled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [checkbox-disabled.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col mx-auto w-56"
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
      role="checkbox"
      aria-checked="false"
      data-state="unchecked"
      data-disabled=""
      disabled=""
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="toggle-checkbox-disabled"
    ></button
    ><label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="toggle-checkbox-disabled"
      >Enable notifications</label
    >
  </div>
</div>
```
:::

::::


## Group

Use multiple fields to create a checkbox list.

::::demo checkbox-group
<iframe class="demo" src="/demos/checkbox-group.html" title="checkbox-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/checkbox-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [checkbox-group.html]
<fieldset
  data-slot="field-set"
  class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
>
  <legend
    data-slot="field-legend"
    data-variant="label"
    class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
  >
    Show these items on the desktop:
  </legend>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    Select the items you want to show on the desktop.
  </p>
  <div
    data-slot="field-group"
    class="data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col gap-3"
  >
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="checkbox"
        aria-checked="true"
        data-state="checked"
        value="on"
        data-slot="checkbox"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="finder-pref-9k2-hard-disks-ljj-checkbox"
      >
        <span
          data-state="checked"
          data-slot="checkbox-indicator"
          class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
          style="pointer-events: none"
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
        ></span></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="finder-pref-9k2-hard-disks-ljj-checkbox"
        >Hard disks</label
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
        role="checkbox"
        aria-checked="true"
        data-state="checked"
        value="on"
        data-slot="checkbox"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="finder-pref-9k2-external-disks-1yg-checkbox"
      >
        <span
          data-state="checked"
          data-slot="checkbox-indicator"
          class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
          style="pointer-events: none"
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
        ></span></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="finder-pref-9k2-external-disks-1yg-checkbox"
        >External disks</label
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
        role="checkbox"
        aria-checked="false"
        data-state="unchecked"
        value="on"
        data-slot="checkbox"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="finder-pref-9k2-cds-dvds-fzt-checkbox"
      ></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="finder-pref-9k2-cds-dvds-fzt-checkbox"
        >CDs, DVDs, and iPods</label
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
        role="checkbox"
        aria-checked="false"
        data-state="unchecked"
        value="on"
        data-slot="checkbox"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="finder-pref-9k2-connected-servers-6l2-checkbox"
      ></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="finder-pref-9k2-connected-servers-6l2-checkbox"
        >Connected servers</label
      >
    </div>
  </div>
</fieldset>
```
:::

::::


## Table

::::demo checkbox-table
<iframe class="demo" src="/demos/checkbox-table.html" title="checkbox-table" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/checkbox-table.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [checkbox-table.html]
<div data-slot="table-container" class="relative w-full overflow-x-auto">
  <table data-slot="table" class="w-full caption-bottom text-sm">
    <thead data-slot="table-header" class="[&amp;_tr]:border-b">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 w-8"
        >
          <button
            type="button"
            role="checkbox"
            aria-checked="false"
            data-state="unchecked"
            value="on"
            data-slot="checkbox"
            class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
            id="select-all-checkbox"
          ></button>
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Name
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Email
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Role
        </th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&amp;_tr:last-child]:border-0">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
        data-state="selected"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          <button
            type="button"
            role="checkbox"
            aria-checked="true"
            data-state="checked"
            value="on"
            data-slot="checkbox"
            class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
            id="row-1-checkbox"
          >
            <span
              data-state="checked"
              data-slot="checkbox-indicator"
              class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
              style="pointer-events: none"
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
          </button>
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          Sarah Chen
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          sarah.chen@example.com
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Admin
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          <button
            type="button"
            role="checkbox"
            aria-checked="false"
            data-state="unchecked"
            value="on"
            data-slot="checkbox"
            class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
            id="row-2-checkbox"
          ></button>
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          Marcus Rodriguez
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          marcus.rodriguez@example.com
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          User
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          <button
            type="button"
            role="checkbox"
            aria-checked="false"
            data-state="unchecked"
            value="on"
            data-slot="checkbox"
            class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
            id="row-3-checkbox"
          ></button>
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          Priya Patel
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          priya.patel@example.com
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          User
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          <button
            type="button"
            role="checkbox"
            aria-checked="false"
            data-state="unchecked"
            value="on"
            data-slot="checkbox"
            class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
            id="row-4-checkbox"
          ></button>
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          David Kim
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          david.kim@example.com
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Editor
        </td>
      </tr>
    </tbody>
  </table>
</div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo checkbox-rtl
<iframe class="demo" src="/demos/checkbox-rtl.html" title="checkbox-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/checkbox-rtl.html">Open the demo page</a> · <a href="/demos/checkbox-rtl-he.html">HE</a> · <a href="/demos/checkbox-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [checkbox-rtl.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col max-w-sm"
  dir="rtl"
>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="checkbox"
      aria-checked="false"
      data-state="unchecked"
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="terms-checkbox-rtl"
    ></button
    ><label
      data-slot="label"
      class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
      for="terms-checkbox-rtl"
      >قبول الشروط والأحكام</label
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
      role="checkbox"
      aria-checked="true"
      data-state="checked"
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="terms-checkbox-2-rtl"
    >
      <span
        data-state="checked"
        data-slot="checkbox-indicator"
        class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
        style="pointer-events: none"
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
    </button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="terms-checkbox-2-rtl"
        >قبول الشروط والأحكام</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        بالنقر على هذا المربع، فإنك توافق على الشروط.
      </p>
    </div>
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    data-disabled="true"
  >
    <button
      type="button"
      role="checkbox"
      aria-checked="false"
      data-state="unchecked"
      data-disabled=""
      disabled=""
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="toggle-checkbox-rtl"
    ></button
    ><label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      for="toggle-checkbox-rtl"
      >تفعيل الإشعارات</label
    >
  </div>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    ><div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="checkbox"
        aria-checked="false"
        data-state="unchecked"
        value="on"
        data-slot="checkbox"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="toggle-checkbox-2"
      ></button>
      <div
        data-slot="field-content"
        class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
      >
        <div
          data-slot="field-label"
          class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center"
        >
          تفعيل الإشعارات
        </div>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          يمكنك تفعيل أو إلغاء تفعيل الإشعارات في أي وقت.
        </p>
      </div>
    </div></label
  >
</div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="checkbox"` |
| `data-slot="checkbox-indicator"` |

**Runtime:** `role="checkbox"` root with `aria-checked` + `data-state="checked|unchecked"`; the `checkbox-indicator` mounts from a `<template data-for="checkbox-indicator">` while checked (radix Presence). Keys: Space / click toggles. The root dispatches `shadless:change` (`detail: { checked }`). Forms: a `name` attribute submits its `value` (default `on`) while checked. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix UI](https://www.radix-ui.com/docs/primitives/components/checkbox#api-reference) documentation for more information.
