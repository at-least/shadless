---
title: "Switch"
description: "A control that allows the user to toggle between checked and not checked."
---

# Switch

A control that allows the user to toggle between checked and not checked.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/switch" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/switch#api-reference" rel="noopener">api</a></p>

::::demo switch-demo
<iframe class="demo" src="/demos/switch-demo.html" title="switch-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/switch-demo.html">Open the demo page</a></p>

::: code-group
```text [switch-demo.html]
<div class="flex items-center space-x-2"><button type="button" role="switch" aria-checked="false" data-state="unchecked" value="on" data-slot="switch" data-size="default" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="airplane-mode"><span data-state="unchecked" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button><label data-slot="label" class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed" for="airplane-mode">Airplane Mode</label></div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/switch.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/switch.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/switch.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/switch.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/switch.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `role="switch"` root with `aria-checked` + `data-state`; the `switch-thumb` mirrors `data-state`. Keys: Space / click toggles.

The root dispatches `shadless:change` (`detail: { checked }`), bubbling, after the state change, whichever path caused it.

Forms: a `name` attribute submits its `value` (default `on`) while checked; `form.reset()` restores the initial state.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/switch.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Description

::::demo switch-description
<iframe class="demo" src="/demos/switch-description.html" title="switch-description" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/switch-description.html">Open the demo page</a></p>

::: code-group
```text [switch-description.html]
<div role="group" data-slot="field" data-orientation="horizontal" class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px max-w-sm"><div data-slot="field-content" class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"><label data-slot="field-label" class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col" for="switch-focus-mode">Share across devices</label><p data-slot="field-description" class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary">Focus is shared across devices, and turns off when you leave the app.</p></div><button type="button" role="switch" aria-checked="false" data-state="unchecked" value="on" data-slot="switch" data-size="default" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="switch-focus-mode"><span data-state="unchecked" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button></div>
```
:::

::::


## Choice Card

Card-style selection where `FieldLabel` wraps the entire `Field` for a clickable card pattern.

::::demo switch-choice-card
<iframe class="demo" src="/demos/switch-choice-card.html" title="switch-choice-card" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/switch-choice-card.html">Open the demo page</a></p>

::: code-group
```text [switch-choice-card.html]
<div data-slot="field-group" class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col w-full max-w-sm"><label data-slot="field-label" class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col" for="switch-share"><div role="group" data-slot="field" data-orientation="horizontal" class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"><div data-slot="field-content" class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"><div data-slot="field-label" class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center">Share across devices</div><p data-slot="field-description" class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary">Focus is shared across devices, and turns off when you leave the app.</p></div><button type="button" role="switch" aria-checked="false" data-state="unchecked" value="on" data-slot="switch" data-size="default" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="switch-share"><span data-state="unchecked" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button></div></label><label data-slot="field-label" class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col" for="switch-notifications"><div role="group" data-slot="field" data-orientation="horizontal" class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"><div data-slot="field-content" class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"><div data-slot="field-label" class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center">Enable notifications</div><p data-slot="field-description" class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary">Receive notifications when focus mode is enabled or disabled.</p></div><button type="button" role="switch" aria-checked="true" data-state="checked" value="on" data-slot="switch" data-size="default" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="switch-notifications"><span data-state="checked" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button></div></label></div>
```
:::

::::


## Disabled

Add the `disabled` prop to the `Switch` component to disable the switch. Add the `data-disabled` prop to the `Field` component for styling.

::::demo switch-disabled
<iframe class="demo" src="/demos/switch-disabled.html" title="switch-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/switch-disabled.html">Open the demo page</a></p>

::: code-group
```text [switch-disabled.html]
<div role="group" data-slot="field" data-orientation="horizontal" class="data-[invalid=true]:text-destructive gap-2 group/field flex flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px w-fit" data-disabled="true"><button type="button" role="switch" aria-checked="false" data-state="unchecked" data-disabled="" disabled="" value="on" data-slot="switch" data-size="default" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="switch-disabled-unchecked"><span data-state="unchecked" data-disabled="" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button><label data-slot="field-label" class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col" for="switch-disabled-unchecked">Disabled</label></div>
```
:::

::::


## Invalid

Add the `aria-invalid` prop to the `Switch` component to indicate an invalid state. Add the `data-invalid` prop to the `Field` component for styling.

::::demo switch-invalid
<iframe class="demo" src="/demos/switch-invalid.html" title="switch-invalid" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/switch-invalid.html">Open the demo page</a></p>

::: code-group
```text [switch-invalid.html]
<div role="group" data-slot="field" data-orientation="horizontal" class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px max-w-sm" data-invalid="true"><div data-slot="field-content" class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"><label data-slot="field-label" class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col" for="switch-terms">Accept terms and conditions</label><p data-slot="field-description" class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary">You must accept the terms and conditions to continue.</p></div><button type="button" role="switch" aria-checked="false" data-state="unchecked" value="on" data-slot="switch" data-size="default" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="switch-terms" aria-invalid="true"><span data-state="unchecked" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button></div>
```
:::

::::


## Size

Use the `size` prop to change the size of the switch.

::::demo switch-sizes
<iframe class="demo" src="/demos/switch-sizes.html" title="switch-sizes" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/switch-sizes.html">Open the demo page</a></p>

::: code-group
```text [switch-sizes.html]
<div data-slot="field-group" class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col w-full max-w-[10rem]"><div role="group" data-slot="field" data-orientation="horizontal" class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"><button type="button" role="switch" aria-checked="false" data-state="unchecked" value="on" data-slot="switch" data-size="sm" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="switch-size-sm"><span data-state="unchecked" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button><label data-slot="field-label" class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col" for="switch-size-sm">Small</label></div><div role="group" data-slot="field" data-orientation="horizontal" class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"><button type="button" role="switch" aria-checked="false" data-state="unchecked" value="on" data-slot="switch" data-size="default" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="switch-size-default"><span data-state="unchecked" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button><label data-slot="field-label" class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col" for="switch-size-default">Default</label></div></div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo switch-rtl
<iframe class="demo" src="/demos/switch-rtl.html" title="switch-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/switch-rtl.html">Open the demo page</a> · <a href="/demos/switch-rtl-he.html">HE</a> · <a href="/demos/switch-rtl-en.html">EN</a></p>

::: code-group
```text [switch-rtl.html]
<div role="group" data-slot="field" data-orientation="horizontal" class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px max-w-sm" dir="rtl"><div data-slot="field-content" class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"><label data-slot="field-label" class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col" for="switch-focus-mode-rtl" dir="rtl">المشاركة عبر الأجهزة</label><p data-slot="field-description" class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary" dir="rtl">يتم مشاركة التركيز عبر الأجهزة، ويتم إيقاف تشغيله عند مغادرة التطبيق.</p></div><button type="button" role="switch" aria-checked="false" data-state="unchecked" value="on" data-slot="switch" data-size="default" class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50" id="switch-focus-mode-rtl" dir="rtl"><span data-state="unchecked" data-slot="switch-thumb" class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] rtl:group-data-[size=default]/switch:data-checked:-translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] rtl:group-data-[size=sm]/switch:data-checked:-translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 rtl:group-data-[size=default]/switch:data-unchecked:-translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 rtl:group-data-[size=sm]/switch:data-unchecked:-translate-x-0 pointer-events-none block ring-0 transition-transform"></span></button></div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="switch"` |
| `data-slot="switch-thumb"` |

**Runtime:** `role="switch"` root with `aria-checked` + `data-state`; the `switch-thumb` mirrors `data-state`. Keys: Space / click toggles. The root dispatches `shadless:change` (`detail: { checked }`). Forms: a `name` attribute submits its `value` (default `on`) while checked. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix Switch](https://www.radix-ui.com/docs/primitives/components/switch#api-reference) documentation.
