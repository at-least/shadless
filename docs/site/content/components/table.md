---
title: "Table"
description: "A responsive table component."
weight: 46
---

# Table

A responsive table component.

{% <demo name="table-demo" status="authored"> %}
<iframe class="demo" src="/demos/table-demo.html" title="table-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/table-demo.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=table-demo.html
<div data-slot="table-container">
  <table data-slot="table">
    <caption data-slot="table-caption">
      A list of your recent invoices.
    </caption>
    <thead data-slot="table-header">
      <tr data-slot="table-row">
        <th data-slot="table-head" class="w-[100px]">Invoice</th>
        <th data-slot="table-head">Status</th>
        <th data-slot="table-head">Method</th>
        <th data-slot="table-head" class="text-right">Amount</th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&_tr:last-child]:border-0">
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV001</td>
        <td data-slot="table-cell">Paid</td>
        <td data-slot="table-cell">Credit Card</td>
        <td data-slot="table-cell" class="text-right">$250.00</td>
      </tr>
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV002</td>
        <td data-slot="table-cell">Pending</td>
        <td data-slot="table-cell">PayPal</td>
        <td data-slot="table-cell" class="text-right">$150.00</td>
      </tr>
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV003</td>
        <td data-slot="table-cell">Unpaid</td>
        <td data-slot="table-cell">Bank Transfer</td>
        <td data-slot="table-cell" class="text-right">$350.00</td>
      </tr>
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV004</td>
        <td data-slot="table-cell">Paid</td>
        <td data-slot="table-cell">Credit Card</td>
        <td data-slot="table-cell" class="text-right">$450.00</td>
      </tr>
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV005</td>
        <td data-slot="table-cell">Paid</td>
        <td data-slot="table-cell">PayPal</td>
        <td data-slot="table-cell" class="text-right">$550.00</td>
      </tr>
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV006</td>
        <td data-slot="table-cell">Pending</td>
        <td data-slot="table-cell">Bank Transfer</td>
        <td data-slot="table-cell" class="text-right">$200.00</td>
      </tr>
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV007</td>
        <td data-slot="table-cell">Unpaid</td>
        <td data-slot="table-cell">Credit Card</td>
        <td data-slot="table-cell" class="text-right">$300.00</td>
      </tr>
    </tbody>
    <tfoot data-slot="table-footer" class="[&>tr]:last:border-b-0">
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" colspan="3">Total</td>
        <td data-slot="table-cell" class="text-right">$2,500.00</td>
      </tr>
    </tfoot>
  </table>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Installation

**Add shadless and this component to your Tailwind v4 (>= 4.3) entry:**

```css
@import "shadless";
@import "shadless/table.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build (>= 4.3) |
| `dist/css/table.css` | this component's slot styles (`@apply` source — your build compiles it) |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

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

{% <demo name="table-footer" status="authored"> %}
<iframe class="demo" src="/demos/table-footer.html" title="table-footer" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/table-footer.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=table-footer.html
<div data-slot="table-container">
  <table data-slot="table">
    <caption data-slot="table-caption">
      A list of your recent invoices.
    </caption>
    <thead data-slot="table-header">
      <tr data-slot="table-row">
        <th data-slot="table-head" class="w-[100px]">Invoice</th>
        <th data-slot="table-head">Status</th>
        <th data-slot="table-head">Method</th>
        <th data-slot="table-head" class="text-right">Amount</th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&_tr:last-child]:border-0">
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV001</td>
        <td data-slot="table-cell">Paid</td>
        <td data-slot="table-cell">Credit Card</td>
        <td data-slot="table-cell" class="text-right">$250.00</td>
      </tr>
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV002</td>
        <td data-slot="table-cell">Pending</td>
        <td data-slot="table-cell">PayPal</td>
        <td data-slot="table-cell" class="text-right">$150.00</td>
      </tr>
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">INV003</td>
        <td data-slot="table-cell">Unpaid</td>
        <td data-slot="table-cell">Bank Transfer</td>
        <td data-slot="table-cell" class="text-right">$350.00</td>
      </tr>
    </tbody>
    <tfoot data-slot="table-footer" class="[&>tr]:last:border-b-0">
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" colspan="3">Total</td>
        <td data-slot="table-cell" class="text-right">$2,500.00</td>
      </tr>
    </tfoot>
  </table>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Actions

A table showing actions for each row using a dropdown-menu trigger (see the [Dropdown Menu](/components/dropdown-menu) page).

{% <demo name="table-actions" status="authored"> %}
<iframe class="demo" src="/demos/table-actions.html" title="table-actions" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/table-actions.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=table-actions.html
<div data-slot="table-container">
  <table data-slot="table">
    <thead data-slot="table-header">
      <tr data-slot="table-row">
        <th data-slot="table-head">Product</th>
        <th data-slot="table-head">Price</th>
        <th data-slot="table-head" class="text-right">Actions</th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&_tr:last-child]:border-0">
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">Wireless Mouse</td>
        <td data-slot="table-cell">$29.99</td>
        <td data-slot="table-cell" class="text-right">
          <button
            data-slot="dropdown-menu-trigger"
            data-variant="ghost"
            data-size="icon"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">Mechanical Keyboard</td>
        <td data-slot="table-cell">$129.99</td>
        <td data-slot="table-cell" class="text-right">
          <button
            data-slot="dropdown-menu-trigger"
            data-variant="ghost"
            data-size="icon"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
      <tr data-slot="table-row" class="border-b">
        <td data-slot="table-cell" class="font-medium">USB-C Hub</td>
        <td data-slot="table-cell">$49.99</td>
        <td data-slot="table-cell" class="text-right">
          <button
            data-slot="dropdown-menu-trigger"
            data-variant="ghost"
            data-size="icon"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8"
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
    class="cn-menu-target cn-menu-translucent"
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
      class="group/dropdown-menu-item"
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
      class="group/dropdown-menu-item"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Duplicate
    </div>
    <div role="separator" aria-orientation="horizontal" data-slot="dropdown-menu-separator"></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="destructive"
      class="group/dropdown-menu-item"
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
    class="cn-menu-target cn-menu-translucent"
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
      class="group/dropdown-menu-item"
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
      class="group/dropdown-menu-item"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Duplicate
    </div>
    <div role="separator" aria-orientation="horizontal" data-slot="dropdown-menu-separator"></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="destructive"
      class="group/dropdown-menu-item"
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
    class="cn-menu-target cn-menu-translucent"
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
      class="group/dropdown-menu-item"
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
      class="group/dropdown-menu-item"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Duplicate
    </div>
    <div role="separator" aria-orientation="horizontal" data-slot="dropdown-menu-separator"></div>
    <div
      role="menuitem"
      data-slot="dropdown-menu-item"
      data-variant="destructive"
      class="group/dropdown-menu-item"
      tabindex="-1"
      data-orientation="vertical"
      data-radix-collection-item=""
    >
      Delete
    </div>
  </div>
</template>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// shadless dropdown-menu behavior — registers with shadless.h.installMenuFamily,
// the shared wireMenu glue also used by context-menu.js (core.js has the full
// story: the two files' bodies were byte-identical but for this line, so the
// body now lives once and both files just point at it).
(function () {
  shadless.register("dropdown-menu", { init: shadless.h.installMenuFamily })
})()
```
{% </codegroup> %}

{% </demo> %}

## Data Table

You can use the `data-slot="table-container"` component to build more complex data tables. Combine it with [@tanstack/react-table](https://tanstack.com/table/latest) to create tables with sorting, filtering and pagination.

See the Data Table documentation for more information.

You can also see an example of a data table in the Tasks demo.

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

{% <demo name="table-rtl" status="authored"> %}
<iframe class="demo" src="/demos/table-rtl.html" title="table-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/table-rtl.html">Open the demo page</a> · <a href="/demos/table-rtl-he.html">HE</a> · <a href="/demos/table-rtl-en.html">EN</a></p>

{% <codegroup> %}
```text,name=table-rtl.html
<div data-slot="table-container" class="relative w-full overflow-x-auto">
  <table data-slot="table" class="w-full caption-bottom text-sm" dir="rtl">
    <caption data-slot="table-caption" class="text-muted-foreground mt-4 text-sm">
      قائمة بفواتيرك الأخيرة.
    </caption>
    <thead data-slot="table-header" class="[&_tr]:border-b">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-start align-middle font-medium whitespace-nowrap [&:has([role=checkbox])]:pe-0 w-[100px]"
        >
          الفاتورة
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-start align-middle font-medium whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          الحالة
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 text-start align-middle font-medium whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          الطريقة
        </th>
        <th
          data-slot="table-head"
          class="text-foreground h-10 px-2 align-middle font-medium whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
        >
          المبلغ
        </th>
      </tr>
    </thead>
    <tbody data-slot="table-body" class="[&_tr:last-child]:border-0">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 font-medium"
        >
          INV001
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          بطاقة ائتمانية
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
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
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 font-medium"
        >
          INV002
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          قيد الانتظار
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          PayPal
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
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
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 font-medium"
        >
          INV003
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          غير مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          تحويل بنكي
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
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
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 font-medium"
        >
          INV004
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          بطاقة ائتمانية
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
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
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 font-medium"
        >
          INV005
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          PayPal
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
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
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 font-medium"
        >
          INV006
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          قيد الانتظار
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          تحويل بنكي
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
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
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 font-medium"
        >
          INV007
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          غير مدفوع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
        >
          بطاقة ائتمانية
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
        >
          $300.00
        </td>
      </tr>
    </tbody>
    <tfoot data-slot="table-footer" class="bg-muted/50 border-t font-medium [&>tr]:last:border-b-0">
      <tr
        data-slot="table-row"
        class="hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors has-aria-expanded:bg-muted/50"
      >
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0"
          colspan="3"
        >
          المجموع
        </td>
        <td
          data-slot="table-cell"
          class="p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pe-0 text-right"
        >
          $2,500.00
        </td>
      </tr>
    </tfoot>
  </table>
</div>
```
{% </codegroup> %}

{% </demo> %}
