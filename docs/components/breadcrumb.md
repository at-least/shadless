---
title: "Breadcrumb"
description: "Displays the path to the current resource using a hierarchy of links."
---

# Breadcrumb

Displays the path to the current resource using a hierarchy of links.

<iframe class="demo" src="/demos/breadcrumb-demo.html" title="breadcrumb-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/breadcrumb.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/breadcrumb.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/breadcrumb.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                   into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/breadcrumb.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
breadcrumb
└── breadcrumb-list
    ├── breadcrumb-item
    │   └── breadcrumb-link
    ├── breadcrumb-separator
    ├── breadcrumb-item
    │   └── breadcrumb-link
    ├── breadcrumb-separator
    └── breadcrumb-item
        └── breadcrumb-page
```

## Basic

A basic breadcrumb with a home link and a components link.

<iframe class="demo" src="/demos/breadcrumb-basic.html" title="breadcrumb-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Custom separator

Use a custom component as `children` for `<BreadcrumbSeparator />` to create a custom separator.

<iframe class="demo" src="/demos/breadcrumb-separator.html" title="breadcrumb-separator" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Dropdown

You can compose `<BreadcrumbItem />` with a `<DropdownMenu />` to create a dropdown in the breadcrumb.

<iframe class="demo" src="/demos/breadcrumb-dropdown.html" title="breadcrumb-dropdown" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Collapsed

We provide a `<BreadcrumbEllipsis />` component to show a collapsed state when the breadcrumb is too long.

<iframe class="demo" src="/demos/breadcrumb-ellipsis.html" title="breadcrumb-ellipsis" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Link component

To use a custom link component from your routing library, you can use the `asChild` prop on `<BreadcrumbLink />`.

<iframe class="demo" src="/demos/breadcrumb-link.html" title="breadcrumb-link" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/breadcrumb-rtl.html" title="breadcrumb-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="breadcrumb"` |
| `data-slot="breadcrumb-list"` |
| `data-slot="breadcrumb-item"` |
| `data-slot="breadcrumb-link"` |
| `data-slot="breadcrumb-page"` |
| `data-slot="breadcrumb-separator"` |
| `data-slot="breadcrumb-ellipsis"` |

### Breadcrumb

The `Breadcrumb` component is the root navigation element that wraps all breadcrumb components.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### BreadcrumbList

The `BreadcrumbList` component displays the ordered list of breadcrumb items.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### BreadcrumbItem

The `BreadcrumbItem` component wraps individual breadcrumb items.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### BreadcrumbLink

The `BreadcrumbLink` component displays a clickable link in the breadcrumb.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### BreadcrumbPage

The `BreadcrumbPage` component displays the current page in the breadcrumb (non-clickable).

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### BreadcrumbSeparator

The `BreadcrumbSeparator` component displays a separator between breadcrumb items. You can pass custom children to override the default separator icon.

| Prop        | Type              | Default |
| ----------- | ----------------- | ------- |
| `children`  | `React.ReactNode` | -       |
| `className` | `string`          | -       |

### BreadcrumbEllipsis

The `BreadcrumbEllipsis` component displays an ellipsis indicator for collapsed breadcrumb items.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |
