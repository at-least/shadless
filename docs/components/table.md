---
title: "Table"
description: "A responsive table component."
---

# Table

A responsive table component.

<iframe class="demo" src="/demos/table-demo.html" title="table-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/table.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/table.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/table.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/table.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
table-container
├── table-caption
├── table-header
│   └── table-row
│       ├── table-head
│       ├── table-head
│       ├── table-head
│       └── table-head
├── table-body
│   ├── table-row
│   │   ├── table-cell
│   │   ├── table-cell
│   │   ├── table-cell
│   │   └── table-cell
│   └── table-row
│       ├── table-cell
│       ├── table-cell
│       ├── table-cell
│       └── table-cell
└── table-footer
```

## Footer

Use the `<TableFooter />` component to add a footer to the table.

<iframe class="demo" src="/demos/table-footer.html" title="table-footer" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Actions

A table showing actions for each row using a `<DropdownMenu />` component.

<iframe class="demo" src="/demos/table-actions.html" title="table-actions" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Data Table

You can use the `<Table />` component to build more complex data tables. Combine it with [@tanstack/react-table](https://tanstack.com/table/latest) to create tables with sorting, filtering and pagination.

See the Data Table documentation for more information.

You can also see an example of a data table in the Tasks demo.

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/table-rtl.html" title="table-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>
