---
title: "Table"
description: "A responsive table component."
---

# Table

A responsive table component.

::::demo table-demo
<iframe class="demo" src="/demos/table-demo.html" title="table-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/table-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [table-demo.html]
<div data-slot="table-container" class="relative w-full overflow-x-auto">
  <table data-slot="table" class="w-full caption-bottom text-sm">
    <caption data-slot="table-caption" class="text-muted-foreground mt-4 text-sm">
      A list of your recent invoices.
    </caption>
    <thead data-slot="table-header" class="[&amp;_tr]:border-b">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 w-[100px]"
        >
          Invoice
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Status
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Method
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          Amount
        </th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&amp;_tr:last-child]:border-0">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV001
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Paid
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Credit Card
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $250.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV002
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Pending
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          PayPal
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $150.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV003
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Unpaid
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Bank Transfer
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $350.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV004
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Paid
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Credit Card
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $450.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV005
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Paid
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          PayPal
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $550.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV006
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Pending
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Bank Transfer
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $200.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV007
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Unpaid
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Credit Card
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $300.00
        </td>
      </tr>
    </tbody>
    <tfoot
      data-slot="table-footer"
      class="bg-muted/50 border-t font-medium [&amp;&gt;tr]:last:border-b-0"
    >
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
          colspan="3"
        >
          Total
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $2,500.00
        </td>
      </tr>
    </tfoot>
  </table>
</div>
```
:::

::::


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


**Copy the markup from `dist/components/table.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.



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

Use the `data-slot="table-footer"` component to add a footer to the table.

::::demo table-footer
<iframe class="demo" src="/demos/table-footer.html" title="table-footer" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/table-footer.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [table-footer.html]
<div data-slot="table-container" class="relative w-full overflow-x-auto">
  <table data-slot="table" class="w-full caption-bottom text-sm">
    <caption data-slot="table-caption" class="text-muted-foreground mt-4 text-sm">
      A list of your recent invoices.
    </caption>
    <thead data-slot="table-header" class="[&amp;_tr]:border-b">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 w-[100px]"
        >
          Invoice
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Status
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Method
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          Amount
        </th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&amp;_tr:last-child]:border-0">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV001
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Paid
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Credit Card
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $250.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV002
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Pending
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          PayPal
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $150.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          INV003
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Unpaid
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Bank Transfer
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $350.00
        </td>
      </tr>
    </tbody>
    <tfoot
      data-slot="table-footer"
      class="bg-muted/50 border-t font-medium [&amp;&gt;tr]:last:border-b-0"
    >
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
          colspan="3"
        >
          Total
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          $2,500.00
        </td>
      </tr>
    </tfoot>
  </table>
</div>
```
:::

::::


## Actions

A table showing actions for each row using a dropdown-menu trigger (see the [Dropdown Menu](/components/dropdown-menu) page).

::::demo table-actions
<iframe class="demo" src="/demos/table-actions.html" title="table-actions" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/table-actions.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [table-actions.html]
<div data-slot="table-container" class="relative w-full overflow-x-auto">
  <table data-slot="table" class="w-full caption-bottom text-sm">
    <thead data-slot="table-header" class="[&amp;_tr]:border-b">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Product
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          Price
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          Actions
        </th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&amp;_tr:last-child]:border-0">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          Wireless Mouse
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          $29.99
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          <button
            data-slot="dropdown-menu-trigger"
            data-variant="ghost"
            data-size="icon"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
            type="button"
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
              <circle cx="5" cy="12" r="1"></circle></svg
            ><span class="sr-only">Open menu</span>
          </button>
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          Mechanical Keyboard
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          $129.99
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          <button
            data-slot="dropdown-menu-trigger"
            data-variant="ghost"
            data-size="icon"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
            type="button"
            id="m1-trigger"
            aria-haspopup="menu"
            aria-expanded="false"
            data-state="closed"
            data-radixuigo-menu-trigger="m1"
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
              <circle cx="5" cy="12" r="1"></circle></svg
            ><span class="sr-only">Open menu</span>
          </button>
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 font-medium"
        >
          USB-C Hub
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0"
        >
          $49.99
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pr-0 text-right"
        >
          <button
            data-slot="dropdown-menu-trigger"
            data-variant="ghost"
            data-size="icon"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
            type="button"
            id="m2-trigger"
            aria-haspopup="menu"
            aria-expanded="false"
            data-state="closed"
            data-radixuigo-menu-trigger="m2"
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
              <circle cx="5" cy="12" r="1"></circle></svg
            ><span class="sr-only">Open menu</span>
          </button>
        </td>
      </tr>
    </tbody>
  </table>
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
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Edit
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
      Duplicate
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="destructive"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Delete
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
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Edit
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
      Duplicate
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="destructive"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Delete
    </div>
  </div>
</template>
<template id="m2-tpl">
  <div
    data-side="bottom"
    data-align="end"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m2"
    aria-labelledby="m2-trigger"
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
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="default"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Edit
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
      Duplicate
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="destructive"
      class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Delete
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
// finds the listeners already there). dropdown-menu.js and context-menu.js
// share that single installation (shadless.__menuWired.installed_menu) —
// whichever file's init runs first does the real RadixKernel.wireMenu()
// setup, the other reuses it via shadless.__menuWired.fileTriggers. Each
// file still registers its OWN component name unconditionally, and
// fileTriggers labels every element by its actual
// data-radixuigo-{menu,context}-trigger attribute rather than by which file
// happened to install — otherwise, on a page carrying both component types,
// whichever file loaded second would never call shadless.register at all
// (its component type invisible to the engine) and every trigger, of both
// kinds, would be mislabelled with the first file's component name.
(function () {
  shadless.register("dropdown-menu", { init: function (root) {
    shadless.__menuWired = shadless.__menuWired || {}
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
        var isContext = t.hasAttribute("data-radixuigo-context-trigger")
        var isOpen = function () { var l = handles.rootLayer(); return !!l && l.trigger === t }
        var openIt = function () {
          if (isOpen()) return
          if (isContext) {
            var r = t.getBoundingClientRect()
            handles.onContextmenu(t, r.left + r.width / 2, r.top + r.height / 2, noop)
          } else handles.onDocumentClick(t, noop)
          sync()
        }
        shadless.instances.set(t, { component: isContext ? "context-menu" : "dropdown-menu",
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


## Data Table

You can use the `data-slot="table-container"` component to build more complex data tables. Combine it with [@tanstack/react-table](https://tanstack.com/table/latest) to create tables with sorting, filtering and pagination.

See the Data Table documentation for more information.

You can also see an example of a data table in the Tasks demo.

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo table-rtl
<iframe class="demo" src="/demos/table-rtl.html" title="table-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/table-rtl.html">Open the demo page</a> · <a href="/demos/table-rtl-he.html">HE</a> · <a href="/demos/table-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [table-rtl.html]
<div data-slot="table-container" class="relative w-full overflow-x-auto">
  <table data-slot="table" class="w-full caption-bottom text-sm" dir="rtl">
    <caption data-slot="table-caption" class="text-muted-foreground mt-4 text-sm">
      قائمة بفواتيرك الأخيرة.
    </caption>
    <thead data-slot="table-header" class="[&amp;_tr]:border-b">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-start align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 w-[100px]"
        >
          الفاتورة
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-start align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          الحالة
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-start align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          الطريقة
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 align-middle font-medium whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          المبلغ
        </th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&amp;_tr:last-child]:border-0">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 font-medium"
        >
          INV001
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          بطاقة ائتمانية
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          $250.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 font-medium"
        >
          INV002
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          قيد الانتظار
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          PayPal
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          $150.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 font-medium"
        >
          INV003
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          غير مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          تحويل بنكي
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          $350.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 font-medium"
        >
          INV004
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          بطاقة ائتمانية
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          $450.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 font-medium"
        >
          INV005
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          PayPal
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          $550.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 font-medium"
        >
          INV006
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          قيد الانتظار
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          تحويل بنكي
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          $200.00
        </td>
      </tr>
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 font-medium"
        >
          INV007
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          غير مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
        >
          بطاقة ائتمانية
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          $300.00
        </td>
      </tr>
    </tbody>
    <tfoot
      data-slot="table-footer"
      class="bg-muted/50 border-t font-medium [&amp;&gt;tr]:last:border-b-0"
    >
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0"
          colspan="3"
        >
          المجموع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&amp;:has([role=checkbox])]:pe-0 text-right"
        >
          $2,500.00
        </td>
      </tr>
    </tfoot>
  </table>
</div>
```
:::

::::
