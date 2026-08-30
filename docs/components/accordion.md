---
title: "Accordion"
description: "A vertically stacked set of interactive headings that each reveal a section of content."
---

# Accordion

A vertically stacked set of interactive headings that each reveal a section of content.

<p class="page-links"><a href="https://www.radix-ui.com/primitives/docs/components/accordion" rel="noopener">doc</a> · <a href="https://www.radix-ui.com/primitives/docs/components/accordion#api-reference" rel="noopener">api</a></p>

::::demo accordion-demo
<iframe class="demo" src="/demos/accordion-demo.html" title="accordion-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/accordion-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [accordion-demo.html]
<div data-slot="accordion" class="flex w-full flex-col max-w-lg" data-orientation="vertical">
  <div
    data-state="open"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="open" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="true"
        data-state="open"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        What are your shipping options?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="open"
      id="radix-<auto>_"
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
        transition-duration: 0s;
        animation-name: none;
        --radix-collapsible-content-height: 18px;
        --radix-collapsible-content-width: 1264px;
      "
    >
      <div
        class="pt-0 pb-2.5 h-(--radix-accordion-content-height) [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground [&amp;_p:not(:last-child)]:mb-4"
      >
        We offer standard (5-7 days), express (2-3 days), and overnight shipping. Free shipping on
        international orders.
      </div>
    </div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        What is your return policy?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        How can I contact customer support?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
</div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/accordion.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/accordion.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/accordion.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/accordion.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/accordion.js"></script>
```

**Copy the markup from                                  into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. root `data-type="single|multiple"`; each `accordion-trigger` carries `aria-expanded` + `data-state="open|closed"` and `aria-controls` → its `accordion-content`, which is `hidden` while closed (kept in the DOM — radix unmounts it). Keys: arrows / Home / End move focus between triggers (rtl-aware); Enter / Space / click toggles.

Each trigger dispatches `shadless:open` / `shadless:close` (a sibling closed by single mode gets its own `close`), bubbling, after the state change, whichever path caused it.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/accordion.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
accordion
├── accordion-item
│   ├── accordion-trigger
│   └── accordion-content
└── accordion-item
    ├── accordion-trigger
    └── accordion-content
```

## Basic

A basic accordion that shows one item at a time. The first item is open by default.

::::demo accordion-basic
<iframe class="demo" src="/demos/accordion-basic.html" title="accordion-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/accordion-basic.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [accordion-basic.html]
<div data-slot="accordion" class="flex w-full flex-col max-w-lg" data-orientation="vertical">
  <div
    data-state="open"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="open" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="true"
        data-state="open"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        How do I reset my password?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="open"
      id="radix-<auto>_"
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
        transition-duration: 0s;
        animation-name: none;
        --radix-collapsible-content-height: 18px;
        --radix-collapsible-content-width: 1264px;
      "
    >
      <div
        class="pt-0 pb-2.5 h-(--radix-accordion-content-height) [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground [&amp;_p:not(:last-child)]:mb-4"
      >
        Click on 'Forgot Password' on the login page, enter your email address, and we'll send you a
        link to reset your password. The link will expire in 24 hours.
      </div>
    </div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        Can I change my subscription plan?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        What payment methods do you accept?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
</div>
```
:::

::::


## Multiple

Use `type="multiple"` to allow multiple items to be open at the same time.

::::demo accordion-multiple
<iframe class="demo" src="/demos/accordion-multiple.html" title="accordion-multiple" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/accordion-multiple.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [accordion-multiple.html]
<div data-slot="accordion" class="flex w-full flex-col max-w-lg" data-orientation="vertical">
  <div
    data-state="open"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="open" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="true"
        data-state="open"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        Notification Settings<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="open"
      id="radix-<auto>_"
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
        transition-duration: 0s;
        animation-name: none;
        --radix-collapsible-content-height: 18px;
        --radix-collapsible-content-width: 1264px;
      "
    >
      <div
        class="pt-0 pb-2.5 h-(--radix-accordion-content-height) [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground [&amp;_p:not(:last-child)]:mb-4"
      >
        Manage how you receive notifications. You can enable email alerts for updates or push
        notifications for mobile devices.
      </div>
    </div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        Privacy &amp; Security<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        Billing &amp; Subscription<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
</div>
```
:::

::::


## Disabled

Use the `disabled` prop on `AccordionItem` to disable individual items.

::::demo accordion-disabled
<iframe class="demo" src="/demos/accordion-disabled.html" title="accordion-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/accordion-disabled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [accordion-disabled.html]
<div data-slot="accordion" class="flex flex-col w-full" data-orientation="vertical">
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        Can I access my account history?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
  <div
    data-state="closed"
    data-disabled=""
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" data-disabled="" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-disabled=""
        disabled=""
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        Premium feature information<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      data-disabled=""
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        How do I update my email address?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
</div>
```
:::

::::


## Borders

Add `border` to the `Accordion` and `border-b last:border-b-0` to the `AccordionItem` to add borders to the items.

::::demo accordion-borders
<iframe class="demo" src="/demos/accordion-borders.html" title="accordion-borders" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/accordion-borders.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [accordion-borders.html]
<div
  data-slot="accordion"
  class="flex w-full flex-col max-w-lg rounded-lg border"
  data-orientation="vertical"
>
  <div
    data-state="open"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b border-b px-4 last:border-b-0"
  >
    <h3 data-orientation="vertical" data-state="open" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="true"
        data-state="open"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        How does billing work?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="open"
      id="radix-<auto>_"
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
        transition-duration: 0s;
        animation-name: none;
        --radix-collapsible-content-height: 36px;
        --radix-collapsible-content-width: 1264px;
      "
    >
      <div
        class="pt-0 pb-2.5 h-(--radix-accordion-content-height) [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground [&amp;_p:not(:last-child)]:mb-4"
      >
        We offer monthly and annual subscription plans. Billing is charged at the beginning of each
        cycle, and you can cancel anytime. All plans include automatic backups, 24/7 support, and
        unlimited team members.
      </div>
    </div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b border-b px-4 last:border-b-0"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        Is my data secure?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b border-b px-4 last:border-b-0"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        What integrations do you support?<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
</div>
```
:::

::::


## Card

Wrap the `Accordion` in a `Card` component.

::::demo accordion-card
<iframe class="demo" src="/demos/accordion-card.html" title="accordion-card" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/accordion-card.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [accordion-card.html]
<div
  data-slot="card"
  data-size="default"
  class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col w-full max-w-sm"
>
  <div
    data-slot="card-header"
    class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"
  >
    <div
      data-slot="card-title"
      class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading"
    >
      Subscription &amp; Billing
    </div>
    <div data-slot="card-description" class="text-muted-foreground text-sm">
      Common questions about your account, plans, payments and cancellations.
    </div>
  </div>
  <div data-slot="card-content" class="px-(--card-spacing)">
    <div data-slot="accordion" class="flex w-full flex-col" data-orientation="vertical">
      <div
        data-state="open"
        data-orientation="vertical"
        data-slot="accordion-item"
        class="not-last:border-b"
      >
        <h3 data-orientation="vertical" data-state="open" class="flex">
          <button
            type="button"
            aria-controls="radix-<auto>_"
            aria-expanded="true"
            data-state="open"
            data-orientation="vertical"
            id="radix-<auto>_"
            data-slot="accordion-trigger"
            class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
            data-radix-collection-item=""
          >
            What subscription plans do you offer?<svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
              data-slot="accordion-trigger-icon"
            >
              <path d="m6 9 6 6 6-6"></path></svg
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
              class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
              data-slot="accordion-trigger-icon"
            >
              <path d="m18 15-6-6-6 6"></path>
            </svg>
          </button>
        </h3>
        <div
          data-state="open"
          id="radix-<auto>_"
          role="region"
          aria-labelledby="radix-<auto>_"
          data-orientation="vertical"
          data-slot="accordion-content"
          class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
          style="
            --radix-accordion-content-height: var(--radix-collapsible-content-height);
            --radix-accordion-content-width: var(--radix-collapsible-content-width);
            transition-duration: 0s;
            animation-name: none;
            --radix-collapsible-content-height: 36px;
            --radix-collapsible-content-width: 1264px;
          "
        >
          <div
            class="pt-0 pb-2.5 h-(--radix-accordion-content-height) [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground [&amp;_p:not(:last-child)]:mb-4"
          >
            We offer three subscription tiers: Starter ($9/month), Professional ($29/month), and
            Enterprise ($99/month). Each plan includes increasing storage limits, API access,
            priority support, and team collaboration features.
          </div>
        </div>
      </div>
      <div
        data-state="closed"
        data-orientation="vertical"
        data-slot="accordion-item"
        class="not-last:border-b"
      >
        <h3 data-orientation="vertical" data-state="closed" class="flex">
          <button
            type="button"
            aria-controls="radix-<auto>_"
            aria-expanded="false"
            data-state="closed"
            data-orientation="vertical"
            id="radix-<auto>_"
            data-slot="accordion-trigger"
            class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
            data-radix-collection-item=""
          >
            How does billing work?<svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
              data-slot="accordion-trigger-icon"
            >
              <path d="m6 9 6 6 6-6"></path></svg
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
              class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
              data-slot="accordion-trigger-icon"
            >
              <path d="m18 15-6-6-6 6"></path>
            </svg>
          </button>
        </h3>
        <div
          data-state="closed"
          id="radix-<auto>_"
          hidden=""
          role="region"
          aria-labelledby="radix-<auto>_"
          data-orientation="vertical"
          data-slot="accordion-content"
          class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
          style="
            --radix-accordion-content-height: var(--radix-collapsible-content-height);
            --radix-accordion-content-width: var(--radix-collapsible-content-width);
          "
        ></div>
      </div>
      <div
        data-state="closed"
        data-orientation="vertical"
        data-slot="accordion-item"
        class="not-last:border-b"
      >
        <h3 data-orientation="vertical" data-state="closed" class="flex">
          <button
            type="button"
            aria-controls="radix-<auto>_"
            aria-expanded="false"
            data-state="closed"
            data-orientation="vertical"
            id="radix-<auto>_"
            data-slot="accordion-trigger"
            class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
            data-radix-collection-item=""
          >
            How do I cancel my subscription?<svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
              data-slot="accordion-trigger-icon"
            >
              <path d="m6 9 6 6 6-6"></path></svg
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
              class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
              data-slot="accordion-trigger-icon"
            >
              <path d="m18 15-6-6-6 6"></path>
            </svg>
          </button>
        </h3>
        <div
          data-state="closed"
          id="radix-<auto>_"
          hidden=""
          role="region"
          aria-labelledby="radix-<auto>_"
          data-orientation="vertical"
          data-slot="accordion-content"
          class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
          style="
            --radix-accordion-content-height: var(--radix-collapsible-content-height);
            --radix-accordion-content-width: var(--radix-collapsible-content-width);
          "
        ></div>
      </div>
    </div>
  </div>
</div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo accordion-rtl
<iframe class="demo" src="/demos/accordion-rtl.html" title="accordion-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/accordion-rtl.html">Open the demo page</a> · <a href="/demos/accordion-rtl-he.html">HE</a> · <a href="/demos/accordion-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [accordion-rtl.html]
<div data-slot="accordion" class="flex w-full flex-col max-w-md" data-orientation="vertical">
  <div
    data-state="open"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="open" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="true"
        data-state="open"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-start text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ms-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        كيف يمكنني إعادة تعيين كلمة المرور؟<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="open"
      id="radix-<auto>_"
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
        transition-duration: 0s;
        animation-name: none;
        --radix-collapsible-content-height: 19px;
        --radix-collapsible-content-width: 1264px;
      "
    >
      <div
        class="pt-0 pb-2.5 h-(--radix-accordion-content-height) [&amp;_a]:underline [&amp;_a]:underline-offset-3 [&amp;_a]:hover:text-foreground [&amp;_p:not(:last-child)]:mb-4"
      >
        انقر على 'نسيت كلمة المرور' في صفحة تسجيل الدخول، أدخل عنوان بريدك الإلكتروني، وسنرسل لك
        رابطًا لإعادة تعيين كلمة المرور. سينتهي صلاحية الرابط خلال 24 ساعة.
      </div>
    </div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-start text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ms-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        هل يمكنني تغيير خطة الاشتراك الخاصة بي؟<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
  <div
    data-state="closed"
    data-orientation="vertical"
    data-slot="accordion-item"
    class="not-last:border-b"
  >
    <h3 data-orientation="vertical" data-state="closed" class="flex">
      <button
        type="button"
        aria-controls="radix-<auto>_"
        aria-expanded="false"
        data-state="closed"
        data-orientation="vertical"
        id="radix-<auto>_"
        data-slot="accordion-trigger"
        class="focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-start text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ms-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
      >
        ما هي طرق الدفع التي تقبلونها؟<svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-chevron-down pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
          data-slot="accordion-trigger-icon"
        >
          <path d="m6 9 6 6 6-6"></path></svg
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
          class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
          data-slot="accordion-trigger-icon"
        >
          <path d="m18 15-6-6-6 6"></path>
        </svg>
      </button>
    </h3>
    <div
      data-state="closed"
      id="radix-<auto>_"
      hidden=""
      role="region"
      aria-labelledby="radix-<auto>_"
      data-orientation="vertical"
      data-slot="accordion-content"
      class="data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden"
      style="
        --radix-accordion-content-height: var(--radix-collapsible-content-height);
        --radix-accordion-content-width: var(--radix-collapsible-content-width);
      "
    ></div>
  </div>
</div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="accordion"` |
| `data-slot="accordion-item"` |
| `data-slot="accordion-trigger"` |
| `data-slot="accordion-trigger-icon"` |
| `data-slot="accordion-content"` |

**Runtime:** root `data-type="single|multiple"`; each `accordion-trigger` carries `aria-expanded` + `data-state="open|closed"` and `aria-controls` → its `accordion-content`, which is `hidden` while closed (kept in the DOM — radix unmounts it). Keys: arrows / Home / End move focus between triggers (rtl-aware); Enter / Space / click toggles. Each trigger dispatches `shadless:open` / `shadless:close` (a sibling closed by single mode gets its own `close`). No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

See the [Radix UI](https://www.radix-ui.com/primitives/docs/components/accordion#api-reference) documentation for more information.
