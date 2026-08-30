---
title: "Card"
description: "Displays a card with header, content, and footer."
---

# Card

Displays a card with header, content, and footer.

<iframe class="demo" src="/demos/card-demo.html" title="card-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/card.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/card.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/card.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                             into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/card.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
card
├── card-header
│   ├── card-title
│   ├── card-description
│   └── card-action
├── card-content
└── card-footer
```

## Size

Use the `size="sm"` prop to set the size of the card to small. The small size variant uses smaller spacing.

<iframe class="demo" src="/demos/card-small.html" title="card-small" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Spacing

In addition to the `size` prop, you can use the `--card-spacing` CSS variable to control the spacing between sections and the inset of card parts.

<iframe class="demo" src="/demos/card-spacing.html" title="card-spacing" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

Use negative margins with `-mx-(--card-spacing)` to make content go edge to edge while keeping it aligned with the card inset. When the edge-to-edge content sits above a footer, use `-mb-(--card-spacing)` on `CardContent` to remove the section gap.

<iframe class="demo" src="/demos/card-edge-to-edge.html" title="card-edge-to-edge" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Image

Add an image before the card header to create a card with an image.

<iframe class="demo" src="/demos/card-image.html" title="card-image" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/card-rtl.html" title="card-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="card"` |
| `data-slot="card-header"` |
| `data-slot="card-title"` |
| `data-slot="card-description"` |
| `data-slot="card-action"` |
| `data-slot="card-content"` |
| `data-slot="card-footer"` |

### Card

The `Card` component is the root container for card content.

| Prop        | Type                | Default     |
| ----------- | ------------------- | ----------- |
| `size`      | `"default" \| "sm"` | `"default"` |
| `className` | `string`            | -           |

### CardHeader

The `CardHeader` component is used for a title, description, and optional action.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardTitle

The `CardTitle` component is used for the card title.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardDescription

The `CardDescription` component is used for helper text under the title.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardAction

The `CardAction` component places content in the top-right of the header (for example, a button or a badge).

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardContent

The `CardContent` component is used for the main card body.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardFooter

The `CardFooter` component is used for actions and secondary content at the bottom of the card.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

## Changelog

### Spacing Variable

If you're upgrading from a previous version of the `Card` component, you'll need to apply the following updates to use the `--card-spacing` variable:



**Update the Card root spacing classes.**

Replace the hard-coded gap and vertical padding with `--card-spacing`, and set the default and small size values on the root:

```diff
  className={cn(
-   "group/card flex flex-col gap-4 overflow-hidden rounded-xl bg-card py-4 text-sm text-card-foreground ring-1 ring-foreground/10 has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:gap-3 data-[size=sm]:py-3 data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl",
+   "group/card flex flex-col gap-(--card-spacing) overflow-hidden rounded-xl bg-card py-(--card-spacing) text-sm text-card-foreground ring-1 ring-foreground/10 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl",
    className
  )}
```

**Update CardHeader spacing classes.**

Replace the horizontal padding and border spacing with the shared variable:

```diff
  className={cn(
-   "group/card-header @container/card-header grid auto-rows-min items-start gap-1 rounded-t-xl px-4 group-data-[size=sm]/card:px-3 has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto] [.border-b]:pb-4 group-data-[size=sm]/card:[.border-b]:pb-3",
+   "group/card-header @container/card-header grid auto-rows-min items-start gap-1 rounded-t-xl px-(--card-spacing) has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto] [.border-b]:pb-(--card-spacing)",
    className
  )}
```

**Update CardContent and CardFooter spacing classes.**

Use `--card-spacing` for the content inset and footer padding:

```diff
  function CardContent({ className, ...props }: React.ComponentProps<"div">) {
    return (
      <div
        data-slot="card-content"
-       className={cn("px-4 group-data-[size=sm]/card:px-3", className)}
+       className={cn("px-(--card-spacing)", className)}
        {...props}
      />
    )
  }
```

```diff
  className={cn(
-   "flex items-center rounded-b-xl border-t bg-muted/50 p-4 group-data-[size=sm]/card:p-3",
+   "flex items-center rounded-b-xl border-t bg-muted/50 p-(--card-spacing)",
    className
  )}
```



After applying these changes, you can customize card spacing by setting `--card-spacing` on the `Card` with an arbitrary property class:

```tsx
function Example() {
  return <Card className="[--card-spacing:--spacing(6)]">...</Card>
}
```
