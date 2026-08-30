---
title: "Alert"
description: "Displays a callout for user attention."
---

# Alert

Displays a callout for user attention.

::::demo alert-demo
<iframe class="demo" src="/demos/alert-demo.html" title="alert-demo" data-status="existing-dist" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/alert-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [alert-demo.html]
<div class="grid w-full max-w-md items-start gap-4">
  <div
    data-slot="alert"
    role="alert"
    class="grid gap-0.5 rounded-lg border px-2.5 py-2 text-left text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pr-18 has-[&gt;svg]:grid-cols-[auto_1fr] has-[&gt;svg]:gap-x-2 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg]:text-current *:[svg:not([class*='size-'])]:size-4 group/alert relative w-full bg-card text-card-foreground"
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
      class="lucide lucide-circle-check"
    >
      <circle cx="12" cy="12" r="10"></circle>
      <path d="m9 12 2 2 4-4"></path>
    </svg>
    <div
      data-slot="alert-title"
      class="font-medium group-has-[&gt;svg]/alert:col-start-2 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
    >
      Payment successful
    </div>
    <div
      data-slot="alert-description"
      class="text-muted-foreground text-sm text-balance md:text-pretty [&amp;_p:not(:last-child)]:mb-4 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
    >
      Your payment of $29.99 has been processed. A receipt has been sent to your email address.
    </div>
  </div>
  <div
    data-slot="alert"
    role="alert"
    class="grid gap-0.5 rounded-lg border px-2.5 py-2 text-left text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pr-18 has-[&gt;svg]:grid-cols-[auto_1fr] has-[&gt;svg]:gap-x-2 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg]:text-current *:[svg:not([class*='size-'])]:size-4 group/alert relative w-full bg-card text-card-foreground"
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
      class="lucide lucide-info"
    >
      <circle cx="12" cy="12" r="10"></circle>
      <path d="M12 16v-4"></path>
      <path d="M12 8h.01"></path>
    </svg>
    <div
      data-slot="alert-title"
      class="font-medium group-has-[&gt;svg]/alert:col-start-2 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
    >
      New feature available
    </div>
    <div
      data-slot="alert-description"
      class="text-muted-foreground text-sm text-balance md:text-pretty [&amp;_p:not(:last-child)]:mb-4 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
    >
      We've added dark mode support. You can enable it in your account settings.
    </div>
  </div>
</div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/alert.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/alert.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/alert.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/alert.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
alert
├── Icon
├── alert-title
├── alert-description
└── alert-action
```

## Basic

A basic alert with an icon, title and description.

::::demo alert-basic
<iframe class="demo" src="/demos/alert-basic.html" title="alert-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/alert-basic.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [alert-basic.html]
<div
  data-slot="alert"
  role="alert"
  class="grid gap-0.5 rounded-lg border px-2.5 py-2 text-left text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pr-18 has-[&gt;svg]:grid-cols-[auto_1fr] has-[&gt;svg]:gap-x-2 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg]:text-current *:[svg:not([class*='size-'])]:size-4 group/alert relative w-full bg-card text-card-foreground max-w-md"
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
    class="lucide lucide-circle-check"
  >
    <circle cx="12" cy="12" r="10"></circle>
    <path d="m9 12 2 2 4-4"></path>
  </svg>
  <div
    data-slot="alert-title"
    class="font-medium group-has-[&gt;svg]/alert:col-start-2 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
  >
    Account updated successfully
  </div>
  <div
    data-slot="alert-description"
    class="text-muted-foreground text-sm text-balance md:text-pretty [&amp;_p:not(:last-child)]:mb-4 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
  >
    Your profile information has been saved. Changes will be reflected immediately.
  </div>
</div>
```
:::

::::


## Destructive

Use `variant="destructive"` to create a destructive alert.

::::demo alert-destructive
<iframe class="demo" src="/demos/alert-destructive.html" title="alert-destructive" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/alert-destructive.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [alert-destructive.html]
<div
  data-slot="alert"
  role="alert"
  class="grid gap-0.5 rounded-lg border px-2.5 py-2 text-left text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pr-18 has-[&gt;svg]:grid-cols-[auto_1fr] has-[&gt;svg]:gap-x-2 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg:not([class*='size-'])]:size-4 group/alert relative w-full text-destructive bg-card *:data-[slot=alert-description]:text-destructive/90 *:[svg]:text-current max-w-md"
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
    class="lucide lucide-circle-alert"
  >
    <circle cx="12" cy="12" r="10"></circle>
    <line x1="12" x2="12" y1="8" y2="12"></line>
    <line x1="12" x2="12.01" y1="16" y2="16"></line>
  </svg>
  <div
    data-slot="alert-title"
    class="font-medium group-has-[&gt;svg]/alert:col-start-2 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
  >
    Payment failed
  </div>
  <div
    data-slot="alert-description"
    class="text-muted-foreground text-sm text-balance md:text-pretty [&amp;_p:not(:last-child)]:mb-4 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
  >
    Your payment could not be processed. Please check your payment method and try again.
  </div>
</div>
```
:::

::::


## Action

Use `AlertAction` to add a button or other action element to the alert.

::::demo alert-action
<iframe class="demo" src="/demos/alert-action.html" title="alert-action" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/alert-action.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [alert-action.html]
<div
  data-slot="alert"
  role="alert"
  class="grid gap-0.5 rounded-lg border px-2.5 py-2 text-left text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pr-18 has-[&gt;svg]:grid-cols-[auto_1fr] has-[&gt;svg]:gap-x-2 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg]:text-current *:[svg:not([class*='size-'])]:size-4 group/alert relative w-full bg-card text-card-foreground max-w-md"
>
  <div
    data-slot="alert-title"
    class="font-medium group-has-[&gt;svg]/alert:col-start-2 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
  >
    Dark mode is now available
  </div>
  <div
    data-slot="alert-description"
    class="text-muted-foreground text-sm text-balance md:text-pretty [&amp;_p:not(:last-child)]:mb-4 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
  >
    Enable it under your profile settings to get started.
  </div>
  <div data-slot="alert-action" class="absolute top-2 right-2">
    <button
      data-slot="button"
      data-variant="default"
      data-size="xs"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-6 gap-1 rounded-[min(var(--radius-md),10px)] px-2 text-xs in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3"
    >
      Enable
    </button>
  </div>
</div>
```
:::

::::


## Custom Colors

You can customize the alert colors by adding custom classes such as `bg-amber-50 dark:bg-amber-950` to the `Alert` component.

::::demo alert-colors
<iframe class="demo" src="/demos/alert-colors.html" title="alert-colors" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/alert-colors.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [alert-colors.html]
<div
  data-slot="alert"
  role="alert"
  class="grid gap-0.5 rounded-lg border px-2.5 py-2 text-left text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pr-18 has-[&gt;svg]:grid-cols-[auto_1fr] has-[&gt;svg]:gap-x-2 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg]:text-current *:[svg:not([class*='size-'])]:size-4 group/alert relative w-full max-w-md border-amber-200 bg-amber-50 text-amber-900 dark:border-amber-900 dark:bg-amber-950 dark:text-amber-50"
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
    class="lucide lucide-triangle-alert"
  >
    <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"></path>
    <path d="M12 9v4"></path>
    <path d="M12 17h.01"></path>
  </svg>
  <div
    data-slot="alert-title"
    class="font-medium group-has-[&gt;svg]/alert:col-start-2 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
  >
    Your subscription will expire in 3 days.
  </div>
  <div
    data-slot="alert-description"
    class="text-muted-foreground text-sm text-balance md:text-pretty [&amp;_p:not(:last-child)]:mb-4 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
  >
    Renew now to avoid service interruption or upgrade to a paid plan to continue using the service.
  </div>
</div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo alert-rtl
<iframe class="demo" src="/demos/alert-rtl.html" title="alert-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/alert-rtl.html">Open the demo page</a> · <a href="/demos/alert-rtl-he.html">HE</a> · <a href="/demos/alert-rtl-en.html">EN</a> · <a href="/demos/alert-rtl-fa.html">FA</a></p>

::: code-group
```text:line-numbers [alert-rtl.html]
<div class="grid w-full max-w-md items-start gap-4" dir="rtl">
  <div
    data-slot="alert"
    role="alert"
    class="grid gap-0.5 rounded-lg border px-2.5 py-2 text-start text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pe-18 has-[&gt;svg]:grid-cols-[auto_1fr] has-[&gt;svg]:gap-x-2 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg]:text-current *:[svg:not([class*='size-'])]:size-4 group/alert relative w-full bg-card text-card-foreground"
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
      class="lucide lucide-circle-check"
    >
      <circle cx="12" cy="12" r="10"></circle>
      <path d="m9 12 2 2 4-4"></path>
    </svg>
    <div
      data-slot="alert-title"
      class="font-medium group-has-[&gt;svg]/alert:col-start-2 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
    >
      تم الدفع بنجاح
    </div>
    <div
      data-slot="alert-description"
      class="text-muted-foreground text-sm text-balance md:text-pretty [&amp;_p:not(:last-child)]:mb-4 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
    >
      تمت معالجة دفعتك البالغة 29.99 دولارًا. تم إرسال إيصال إلى عنوان بريدك الإلكتروني.
    </div>
  </div>
  <div
    data-slot="alert"
    role="alert"
    class="grid gap-0.5 rounded-lg border px-2.5 py-2 text-start text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pe-18 has-[&gt;svg]:grid-cols-[auto_1fr] has-[&gt;svg]:gap-x-2 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg]:text-current *:[svg:not([class*='size-'])]:size-4 group/alert relative w-full bg-card text-card-foreground"
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
      class="lucide lucide-info"
    >
      <circle cx="12" cy="12" r="10"></circle>
      <path d="M12 16v-4"></path>
      <path d="M12 8h.01"></path>
    </svg>
    <div
      data-slot="alert-title"
      class="font-medium group-has-[&gt;svg]/alert:col-start-2 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
    >
      ميزة جديدة متاحة
    </div>
    <div
      data-slot="alert-description"
      class="text-muted-foreground text-sm text-balance md:text-pretty [&amp;_p:not(:last-child)]:mb-4 [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground"
    >
      لقد أضفنا دعم الوضع الداكن. يمكنك تفعيله في إعدادات حسابك.
    </div>
  </div>
</div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="alert"` |
| `data-slot="alert-title"` |
| `data-slot="alert-description"` |
| `data-slot="alert-action"` |

### Alert

The `Alert` component displays a callout for user attention.

| Prop      | Type                         | Default     |
| --------- | ---------------------------- | ----------- |
| `variant` | `"default" \| "destructive"` | `"default"` |

### AlertTitle

The `AlertTitle` component displays the title of the alert.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### AlertDescription

The `AlertDescription` component displays the description or content of the alert.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### AlertAction

The `AlertAction` component displays an action element (like a button) positioned absolutely in the top-right corner of the alert.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |
