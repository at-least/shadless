---
title: "Installation"
description: "shadless plugs into your Tailwind v4 build — two imports per component, vanilla JS where behavior is needed."
---

# Installation

shadless plugs into your Tailwind v4 build — two imports per component, vanilla JS where behavior is needed.

shadless is consumed as plain files: every component ships as HTML markup
carrying plain Tailwind utilities, a self-contained theme/stylesheet
source, and (only where needed) vanilla JavaScript. There is no CLI and no
framework requirement.

::: tip
The library is a mechanical conversion of the pinned
[shadcn/ui](https://ui.shadcn.com) registry — same markup and styling, zero
React. See the **Installation** section on any component page for its exact
file list.
:::

## Primary: your Tailwind v4 build

The intended path is the one you already know from shadcn itself: the
classes live in your markup, **your** Tailwind build compiles them.



**Add shadless to your Tailwind entry, then one import per component you use.**

```css
@import "shadless";     /* theme + animate layer, self-contained */
@import "shadless/button.css"; /* each component you use, one import    */
```

Import exactly the components you use — nothing else lands in your build
(this is machine-checked: the repo's `consumer-sim` gate compiles a scratch
consumer and asserts zero leakage from non-imported components).

**Copy the component markup you need from                               into your page.**

The markup carries plain utilities inline; your build's content scan picks
them up like any class you wrote yourself.

**Where the component has behavior, load its JavaScript exactly as its demo page does.**

Static components (alert, badge, card, table, …) need no JS at all.
Interactive ones load the base (`shadless.js`) plus one file per
component (`js/<name>.js`) — see the **Installation** section on the
component's page. From a bundler the same surface is ES modules:

```js
import shadless, { get } from "shadless/runtime" // dist/esm/shadless.mjs
import "shadless/js/dialog"                        // dist/esm/dialog.mjs
get("#d1-trigger").open()
```

Each component module imports the base itself, so import order does not
matter; `shadless/esm/<name>` names the module files explicitly.



## Alternative: no build

Not running a Tailwind build? Use the precompiled stylesheet instead of
the imports above:

```html
<link rel="stylesheet" href="out.css">
```

`dist/out.css` (or the npm export `shadless/full.min.css`) contains every
component — the trade-off is size: it is all-or-nothing, while the import
path emits only what you use.

## Get the artifacts

Build the distribution once from the repo:

```bash
npm run demo
```

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme vars, `@theme`, animate layer — the tailwind entry source |
| `dist/css/<name>.css` | per-component slot styles (`@apply` source) |
| `dist/components/<name>.html` | one demo page per component — the markup to copy |
| `dist/out.css` | precompiled styles for every component (no-build path) |
| `dist/shadless.js` / `.min.js` | the JS base: delegation engine, registry, theme, the vendored radix kernel |
| `dist/js/<name>.js` | one behavior file per interactive component (carousel bundles the embla engine) |
| `dist/esm/shadless.mjs`, `dist/esm/<name>.mjs` | the same two as ES modules, for bundlers and `<script type="module">` |

## Theming

The theme ships as CSS variables (`--background`, `--foreground`,
`--primary`, …) with a `.dark` override. See the
[Dark Mode](/guides/dark-mode) guide.

## Right-to-left

Logical utilities (`start-*`/`end-*`, logical animations) are supported —
see the [RTL](/guides/rtl) guide.
