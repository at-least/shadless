---
title: "Navigation Menu"
description: "A collection of links for navigating websites."
---

# Navigation Menu

A collection of links for navigating websites.

<p class="page-links">[doc](https://www.radix-ui.com/docs/primitives/components/navigation-menu) · [api](https://www.radix-ui.com/docs/primitives/components/navigation-menu#api-reference)</p>

<iframe class="demo" src="/demos/navigation-menu-demo.html" title="navigation-menu-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/navigation-menu.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/navigation-menu.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/navigation-menu.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/navigation-menu.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/navigation-menu.js"></script>
```

**Copy the markup from                                        into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="navigation-menu-trigger" id="<k>-trigger" data-radixuigo-nav-trigger="<k>">` | opens on click |
| `<template id="<k>-content-tpl">` | holds the `data-slot="navigation-menu-content"` subtree; the glue creates the shared viewport inside the root |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get("#<k>-trigger")` → `open()`, `close()`, `toggle()`, `isOpen()`. `shadless.get` accepts an element or a selector and walks up from any element inside the instance. The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/navigation-menu.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
navigation-menu
├── navigation-menu-list
│   ├── navigation-menu-item
│   │   ├── navigation-menu-trigger
│   │   └── navigation-menu-content
│   │       ├── navigation-menu-link
│   │       └── navigation-menu-link
│   └── navigation-menu-item
│       └── navigation-menu-link
└── navigation-menu-indicator
```

## Link Component

Use the `asChild` prop to compose a custom link component such as Next.js `Link`.

```tsx showLineNumbers

export function NavigationMenuDemo() {
  return (
    <NavigationMenuItem>
      <NavigationMenuLink asChild className={navigationMenuTriggerStyle()}>
        <Link href="/docs">Documentation</Link>
      </NavigationMenuLink>
    </NavigationMenuItem>
  )
}
```

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<div class="demo-missing" data-demo="navigation-menu-rtl" data-status="tombstoned">demo not available in shadless (component greyed) — <code>navigation-menu-rtl</code></div>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="navigation-menu"` |
| `data-slot="navigation-menu-list"` |
| `data-slot="navigation-menu-item"` |
| `data-slot="navigation-menu-trigger"` |
| `data-slot="navigation-menu-content"` |
| `data-slot="navigation-menu-viewport"` |
| `data-slot="navigation-menu-link"` |
| `data-slot="navigation-menu-indicator"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → `open()`, `close()`, `toggle()`, `isOpen()`; the trigger dispatches `shadless:open` / `shadless:close`. Markup protocol: see Installation → Behavior protocol.

See the [Radix UI Navigation Menu](https://www.radix-ui.com/docs/primitives/components/navigation-menu#api-reference) documentation for more information.
