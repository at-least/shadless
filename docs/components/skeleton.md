---
title: "Skeleton"
description: "Use to show a placeholder while content is loading."
---

# Skeleton

Use to show a placeholder while content is loading.

<iframe class="demo" src="/demos/skeleton-demo.html" title="skeleton-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

::: details Source
```text
<div class="flex items-center gap-4"><div data-slot="skeleton" class="bg-muted animate-pulse h-12 w-12 rounded-full"></div><div class="space-y-2"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-[250px]"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-[200px]"></div></div></div>
```
:::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/skeleton.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/skeleton.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/skeleton.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                 into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/skeleton.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Avatar

<iframe class="demo" src="/demos/skeleton-avatar.html" title="skeleton-avatar" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

::: details Source
```text
<div class="flex w-fit items-center gap-4"><div data-slot="skeleton" class="bg-muted animate-pulse size-10 shrink-0 rounded-full"></div><div class="grid gap-2"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-[150px]"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-[100px]"></div></div></div>
```
:::


## Card

<iframe class="demo" src="/demos/skeleton-card.html" title="skeleton-card" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

::: details Source
```text
<div data-slot="card" data-size="default" class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col w-full max-w-xs"><div data-slot="card-header" class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-2/3"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-1/2"></div></div><div data-slot="card-content" class="px-(--card-spacing)"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse aspect-video w-full"></div></div></div>
```
:::


## Text

<iframe class="demo" src="/demos/skeleton-text.html" title="skeleton-text" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

::: details Source
```text
<div class="flex w-full max-w-xs flex-col gap-2"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-full"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-full"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-3/4"></div></div>
```
:::


## Form

<iframe class="demo" src="/demos/skeleton-form.html" title="skeleton-form" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

::: details Source
```text
<div class="flex w-full max-w-xs flex-col gap-7"><div class="flex flex-col gap-3"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-20"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-8 w-full"></div></div><div class="flex flex-col gap-3"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-24"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-8 w-full"></div></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-8 w-24"></div></div>
```
:::


## Table

<iframe class="demo" src="/demos/skeleton-table.html" title="skeleton-table" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

::: details Source
```text
<div class="flex w-full max-w-sm flex-col gap-2"><div class="flex gap-4"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 flex-1"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-24"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-20"></div></div><div class="flex gap-4"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 flex-1"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-24"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-20"></div></div><div class="flex gap-4"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 flex-1"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-24"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-20"></div></div><div class="flex gap-4"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 flex-1"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-24"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-20"></div></div><div class="flex gap-4"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 flex-1"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-24"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-20"></div></div></div>
```
:::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/skeleton-rtl.html" title="skeleton-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

::: details Source
```text
<div class="flex items-center gap-4" dir="rtl"><div data-slot="skeleton" class="bg-muted animate-pulse h-12 w-12 rounded-full"></div><div class="space-y-2"><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-[250px]"></div><div data-slot="skeleton" class="bg-muted rounded-md animate-pulse h-4 w-[200px]"></div></div></div>
```
:::
