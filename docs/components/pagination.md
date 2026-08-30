---
title: "Pagination"
description: "Pagination with page navigation, next and previous links."
---

# Pagination

Pagination with page navigation, next and previous links.

<iframe class="demo" src="/demos/pagination-demo.html" title="pagination-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/pagination.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/pagination.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/pagination.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                   into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/pagination.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
pagination
└── pagination-content
    ├── pagination-item
    │   └── PaginationPrevious
    ├── pagination-item
    │   └── pagination-link
    ├── pagination-item
    │   └── pagination-ellipsis
    └── pagination-item
        └── PaginationNext
```

## Simple

A simple pagination with only page numbers.

<iframe class="demo" src="/demos/pagination-simple.html" title="pagination-simple" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Icons Only

Use just the previous and next buttons without page numbers. This is useful for data tables with a rows per page selector.

<iframe class="demo" src="/demos/pagination-icons-only.html" title="pagination-icons-only" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Next.js

By default the `<PaginationLink />` component will render an `<a />` tag.

To use the Next.js `<Link />` component, make the following updates to `pagination.tsx`.

```diff showLineNumbers /typeof Link/ {1}
+ import Link from "next/link"

- type PaginationLinkProps = ... & React.ComponentProps<"a">
+ type PaginationLinkProps = ... & React.ComponentProps<typeof Link>

const PaginationLink = ({...props }: ) => (
  <PaginationItem>
-   <a>
+   <Link>
      // ...
-   </a>
+   </Link>
  </PaginationItem>
)

```

::: tip
**Note:** We are making updates to the cli to automatically do this for you.
:::

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/pagination-rtl.html" title="pagination-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## Changelog

### RTL Support

If you're upgrading from a previous version of the `Pagination` component, you'll need to apply the following updates to add the `text` prop:



**Update                     .**

```diff
  function PaginationPrevious({
    className,
+   text = "Previous",
    ...props
- }: React.ComponentProps<typeof PaginationLink>) {
+ }: React.ComponentProps<typeof PaginationLink> & { text?: string }) {
    return (
      <PaginationLink
        aria-label="Go to previous page"
        size="default"
        className={cn("cn-pagination-previous", className)}
        {...props}
      >
        <ChevronLeftIcon />
        <span className="cn-pagination-previous-text hidden sm:block">
-         Previous
+         {text}
        </span>
      </PaginationLink>
    )
  }
```

**Update                 .**

```diff
  function PaginationNext({
    className,
+   text = "Next",
    ...props
- }: React.ComponentProps<typeof PaginationLink>) {
+ }: React.ComponentProps<typeof PaginationLink> & { text?: string }) {
    return (
      <PaginationLink
        aria-label="Go to next page"
        size="default"
        className={cn("cn-pagination-next", className)}
        {...props}
      >
-       <span className="cn-pagination-next-text hidden sm:block">Next</span>
+       <span className="cn-pagination-next-text hidden sm:block">{text}</span>
        <ChevronRightIcon />
      </PaginationLink>
    )
  }
```
