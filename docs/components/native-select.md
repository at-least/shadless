---
title: "Native Select"
description: "A styled native HTML select element with consistent design system integration."
---

# Native Select

A styled native HTML select element with consistent design system integration.

::: tip
}>
For a styled select component, see the [Select](/components/select)
component.
:::

::::demo native-select-demo
<iframe class="demo" src="/demos/native-select-demo.html" title="native-select-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/native-select-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [native-select-demo.html]
<div
  class="group/native-select relative w-fit has-[select:disabled]:opacity-50"
  data-slot="native-select-wrapper"
  data-size="default"
>
  <select
    data-slot="native-select"
    data-size="default"
    class="border-input placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 w-full min-w-0 appearance-none rounded-lg border bg-transparent py-1 pr-8 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] data-[size=sm]:py-0.5 outline-none disabled:pointer-events-none disabled:cursor-not-allowed"
  >
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="">
      Select status
    </option>
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="todo">
      Todo
    </option>
    <option
      data-slot="native-select-option"
      class="bg-[Canvas] text-[CanvasText]"
      value="in-progress"
    >
      In Progress
    </option>
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="done">
      Done
    </option>
    <option
      data-slot="native-select-option"
      class="bg-[Canvas] text-[CanvasText]"
      value="cancelled"
    >
      Cancelled
    </option></select
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
    class="lucide lucide-chevron-down text-muted-foreground top-1/2 right-2.5 size-4 -translate-y-1/2 pointer-events-none absolute select-none"
    aria-hidden="true"
    data-slot="native-select-icon"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</div>
```
:::

::::

## Installation

**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/native-select.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/native-select.css` | this component's slot styles (`@apply` source — your build compiles it) |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
native-select-wrapper
├── native-select-option
├── native-select-option
├── native-select-option
└── native-select-option
```

## Groups

Use `NativeSelectOptGroup` to organize options into categories.

::::demo native-select-groups
<iframe class="demo" src="/demos/native-select-groups.html" title="native-select-groups" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/native-select-groups.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [native-select-groups.html]
<div
  class="group/native-select relative w-fit has-[select:disabled]:opacity-50"
  data-slot="native-select-wrapper"
  data-size="default"
>
  <select
    data-slot="native-select"
    data-size="default"
    class="border-input placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 w-full min-w-0 appearance-none rounded-lg border bg-transparent py-1 pr-8 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] data-[size=sm]:py-0.5 outline-none disabled:pointer-events-none disabled:cursor-not-allowed"
  >
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="">
      Select department
    </option>
    <optgroup
      data-slot="native-select-optgroup"
      class="bg-[Canvas] text-[CanvasText]"
      label="Engineering"
    >
      <option
        data-slot="native-select-option"
        class="bg-[Canvas] text-[CanvasText]"
        value="frontend"
      >
        Frontend
      </option>
      <option
        data-slot="native-select-option"
        class="bg-[Canvas] text-[CanvasText]"
        value="backend"
      >
        Backend
      </option>
      <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="devops">
        DevOps
      </option>
    </optgroup>
    <optgroup
      data-slot="native-select-optgroup"
      class="bg-[Canvas] text-[CanvasText]"
      label="Sales"
    >
      <option
        data-slot="native-select-option"
        class="bg-[Canvas] text-[CanvasText]"
        value="sales-rep"
      >
        Sales Rep
      </option>
      <option
        data-slot="native-select-option"
        class="bg-[Canvas] text-[CanvasText]"
        value="account-manager"
      >
        Account Manager
      </option>
      <option
        data-slot="native-select-option"
        class="bg-[Canvas] text-[CanvasText]"
        value="sales-director"
      >
        Sales Director
      </option>
    </optgroup>
    <optgroup
      data-slot="native-select-optgroup"
      class="bg-[Canvas] text-[CanvasText]"
      label="Operations"
    >
      <option
        data-slot="native-select-option"
        class="bg-[Canvas] text-[CanvasText]"
        value="support"
      >
        Customer Support
      </option>
      <option
        data-slot="native-select-option"
        class="bg-[Canvas] text-[CanvasText]"
        value="product-manager"
      >
        Product Manager
      </option>
      <option
        data-slot="native-select-option"
        class="bg-[Canvas] text-[CanvasText]"
        value="ops-manager"
      >
        Operations Manager
      </option>
    </optgroup></select
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
    class="lucide lucide-chevron-down text-muted-foreground top-1/2 right-2.5 size-4 -translate-y-1/2 pointer-events-none absolute select-none"
    aria-hidden="true"
    data-slot="native-select-icon"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</div>
```
:::

::::

## Disabled

Add the `disabled` prop to the `NativeSelect` component to disable the select.

::::demo native-select-disabled
<iframe class="demo" src="/demos/native-select-disabled.html" title="native-select-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/native-select-disabled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [native-select-disabled.html]
<div
  class="group/native-select relative w-fit has-[select:disabled]:opacity-50"
  data-slot="native-select-wrapper"
  data-size="default"
>
  <select
    data-slot="native-select"
    data-size="default"
    class="border-input placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 w-full min-w-0 appearance-none rounded-lg border bg-transparent py-1 pr-8 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] data-[size=sm]:py-0.5 outline-none disabled:pointer-events-none disabled:cursor-not-allowed"
    disabled=""
  >
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="">
      Disabled
    </option>
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="apple">
      Apple
    </option>
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="banana">
      Banana
    </option>
    <option
      data-slot="native-select-option"
      class="bg-[Canvas] text-[CanvasText]"
      value="blueberry"
    >
      Blueberry
    </option></select
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
    class="lucide lucide-chevron-down text-muted-foreground top-1/2 right-2.5 size-4 -translate-y-1/2 pointer-events-none absolute select-none"
    aria-hidden="true"
    data-slot="native-select-icon"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</div>
```
:::

::::

## Invalid

Use `aria-invalid` to show validation errors and the `data-invalid` attribute to the `Field` component for styling.

::::demo native-select-invalid
<iframe class="demo" src="/demos/native-select-invalid.html" title="native-select-invalid" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/native-select-invalid.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [native-select-invalid.html]
<div
  class="group/native-select relative w-fit has-[select:disabled]:opacity-50"
  data-slot="native-select-wrapper"
  data-size="default"
>
  <select
    data-slot="native-select"
    data-size="default"
    class="border-input placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 w-full min-w-0 appearance-none rounded-lg border bg-transparent py-1 pr-8 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] data-[size=sm]:py-0.5 outline-none disabled:pointer-events-none disabled:cursor-not-allowed"
    aria-invalid="true"
  >
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="">
      Error state
    </option>
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="apple">
      Apple
    </option>
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="banana">
      Banana
    </option>
    <option
      data-slot="native-select-option"
      class="bg-[Canvas] text-[CanvasText]"
      value="blueberry"
    >
      Blueberry
    </option></select
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
    class="lucide lucide-chevron-down text-muted-foreground top-1/2 right-2.5 size-4 -translate-y-1/2 pointer-events-none absolute select-none"
    aria-hidden="true"
    data-slot="native-select-icon"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</div>
```
:::

::::

## Native Select vs Select

- Use `NativeSelect` for native browser behavior, better performance, or mobile-optimized dropdowns.
- Use `Select` for custom styling, animations, or complex interactions.

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo native-select-rtl
<iframe class="demo" src="/demos/native-select-rtl.html" title="native-select-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/native-select-rtl.html">Open the demo page</a> · <a href="/demos/native-select-rtl-he.html">HE</a> · <a href="/demos/native-select-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [native-select-rtl.html]
<div
  class="group/native-select relative w-fit has-[select:disabled]:opacity-50"
  data-slot="native-select-wrapper"
  data-size="default"
>
  <select
    data-slot="native-select"
    data-size="default"
    class="border-input placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-8 w-full min-w-0 appearance-none rounded-lg border bg-transparent py-1 pe-8 ps-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] data-[size=sm]:py-0.5 outline-none disabled:pointer-events-none disabled:cursor-not-allowed"
    dir="rtl"
  >
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="">
      اختر الحالة
    </option>
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="todo">
      مهام
    </option>
    <option
      data-slot="native-select-option"
      class="bg-[Canvas] text-[CanvasText]"
      value="in-progress"
    >
      قيد التنفيذ
    </option>
    <option data-slot="native-select-option" class="bg-[Canvas] text-[CanvasText]" value="done">
      منجز
    </option>
    <option
      data-slot="native-select-option"
      class="bg-[Canvas] text-[CanvasText]"
      value="cancelled"
    >
      ملغي
    </option></select
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
    class="lucide lucide-chevron-down text-muted-foreground top-1/2 end-2.5 size-4 -translate-y-1/2 pointer-events-none absolute select-none"
    aria-hidden="true"
    data-slot="native-select-icon"
  >
    <path d="m6 9 6 6 6-6"></path>
  </svg>
</div>
```
:::

::::

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="native-select-wrapper"` |
| `data-slot="native-select"` |
| `data-slot="native-select-icon"` |
| `data-slot="native-select-option"` |
| `data-slot="native-select-optgroup"` |

**Runtime:** no JavaScript — this is markup + CSS. No `cva`-declared variants. Check `dist/css/native-select.css` for any `data-*` attribute this slot's styling depends on.
