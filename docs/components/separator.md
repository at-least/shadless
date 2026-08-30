---
title: "Separator"
description: "Visually or semantically separates content."
---

# Separator

Visually or semantically separates content.

<p class="page-links"><a href="https://www.radix-ui.com/docs/primitives/components/separator" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/docs/primitives/components/separator#api-reference" rel="noopener">api</a></p>

::::demo separator-demo
<iframe class="demo" src="/demos/separator-demo.html" title="separator-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/separator-demo.html">Open the demo page</a></p>

::: code-group
```text [separator-demo.html]
<div class="flex max-w-sm flex-col gap-4 text-sm"><div class="flex flex-col gap-1.5"><div class="leading-none font-medium">shadcn/ui</div><div class="text-muted-foreground">The Foundation for your Design System</div></div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch"></div><div>A set of beautifully designed components that you can customize, extend, and build on.</div></div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/separator.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/separator.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/separator.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
```

**Copy the markup from                                  into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/separator.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Vertical

Use `orientation="vertical"` for a vertical separator.

::::demo separator-vertical
<iframe class="demo" src="/demos/separator-vertical.html" title="separator-vertical" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/separator-vertical.html">Open the demo page</a></p>

::: code-group
```text [separator-vertical.html]
<div class="flex h-5 items-center gap-4 text-sm"><div>Blog</div><div data-orientation="vertical" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch"></div><div>Docs</div><div data-orientation="vertical" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch"></div><div>Source</div></div>
```
:::

::::


## Menu

Vertical separators between menu items with descriptions.

::::demo separator-menu
<iframe class="demo" src="/demos/separator-menu.html" title="separator-menu" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/separator-menu.html">Open the demo page</a></p>

::: code-group
```text [separator-menu.html]
<div class="flex items-center gap-2 text-sm md:gap-4"><div class="flex flex-col gap-1"><span class="font-medium">Settings</span><span class="text-xs text-muted-foreground">Manage preferences</span></div><div data-orientation="vertical" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch"></div><div class="flex flex-col gap-1"><span class="font-medium">Account</span><span class="text-xs text-muted-foreground">Profile &amp; security</span></div><div data-orientation="vertical" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch hidden md:block"></div><div class="hidden flex-col gap-1 md:flex"><span class="font-medium">Help</span><span class="text-xs text-muted-foreground">Support &amp; docs</span></div></div>
```
:::

::::


## List

Horizontal separators between list items.

::::demo separator-list
<iframe class="demo" src="/demos/separator-list.html" title="separator-list" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/separator-list.html">Open the demo page</a></p>

::: code-group
```text [separator-list.html]
<div class="flex w-full max-w-sm flex-col gap-2 text-sm"><dl class="flex items-center justify-between"><dt>Item 1</dt><dd class="text-muted-foreground">Value 1</dd></dl><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch"></div><dl class="flex items-center justify-between"><dt>Item 2</dt><dd class="text-muted-foreground">Value 2</dd></dl><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch"></div><dl class="flex items-center justify-between"><dt>Item 3</dt><dd class="text-muted-foreground">Value 3</dd></dl></div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo separator-rtl
<iframe class="demo" src="/demos/separator-rtl.html" title="separator-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/separator-rtl.html">Open the demo page</a> · <a href="/demos/separator-rtl-he.html">HE</a> · <a href="/demos/separator-rtl-en.html">EN</a></p>

::: code-group
```text [separator-rtl.html]
<div class="flex max-w-sm flex-col gap-4 text-sm" dir="rtl"><div class="flex flex-col gap-1.5"><div class="leading-none font-medium">shadcn/ui</div><div class="text-muted-foreground">الأساس لنظام التصميم الخاص بك</div></div><div data-orientation="horizontal" role="none" data-slot="separator" class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch"></div><div>مجموعة من المكونات المصممة بشكل جميل يمكنك تخصيصها وتوسيعها والبناء عليها.</div></div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="separator"` |

See the [Radix UI Separator](https://www.radix-ui.com/docs/primitives/components/separator#api-reference) documentation.
