---
title: "Carousel"
description: "A carousel with motion and swipe built using Embla."
---

# Carousel

A carousel with motion and swipe built using Embla.

<p class="page-links">[doc](https://www.embla-carousel.com/get-started/react) · [api](https://www.embla-carousel.com/api)</p>

<iframe class="demo" src="/demos/carousel-demo.html" title="carousel-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## About

The carousel component is built using the [Embla Carousel](https://www.embla-carousel.com/) library.

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/carousel.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/carousel.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/carousel.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/carousel.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/carousel.js"></script>
```

**Copy the markup from                                 into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
| `<… data-slot="carousel">` | no ids, no templates: the glue wires every root it finds |

Content that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.

**From code:** `shadless.get(rootEl)` → the embla api (`scrollNext()`, `scrollTo(i)`, `on("select", …)`). `shadless.get` accepts an element or a selector and walks up from any element inside the instance.

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/carousel.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
carousel
├── carousel-content
│   ├── carousel-item
│   └── carousel-item
├── carousel-previous
└── carousel-next
```

## Sizes

To set the size of the items, you can use the `basis` utility class on the `<CarouselItem />`.

<iframe class="demo" src="/demos/carousel-size.html" title="carousel-size" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

```tsx showLineNumbers {4-6}
// 33% of the carousel width.
<Carousel>
  <CarouselContent>
    <CarouselItem className="basis-1/3">...</CarouselItem>
    <CarouselItem className="basis-1/3">...</CarouselItem>
    <CarouselItem className="basis-1/3">...</CarouselItem>
  </CarouselContent>
</Carousel>
```

```tsx showLineNumbers {4-6}
// 50% on small screens and 33% on larger screens.
<Carousel>
  <CarouselContent>
    <CarouselItem className="md:basis-1/2 lg:basis-1/3">...</CarouselItem>
    <CarouselItem className="md:basis-1/2 lg:basis-1/3">...</CarouselItem>
    <CarouselItem className="md:basis-1/2 lg:basis-1/3">...</CarouselItem>
  </CarouselContent>
</Carousel>
```

## Spacing

To set the spacing between the items, we use a `pl-[VALUE]` utility on the `<CarouselItem />` and a negative `-ml-[VALUE]` on the `<CarouselContent />`.

<iframe class="demo" src="/demos/carousel-spacing.html" title="carousel-spacing" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

```tsx showLineNumbers /-ml-4/ /pl-4/
<Carousel>
  <CarouselContent className="-ml-4">
    <CarouselItem className="pl-4">...</CarouselItem>
    <CarouselItem className="pl-4">...</CarouselItem>
    <CarouselItem className="pl-4">...</CarouselItem>
  </CarouselContent>
</Carousel>
```

```tsx showLineNumbers /-ml-2/ /pl-2/ /md:-ml-4/ /md:pl-4/
<Carousel>
  <CarouselContent className="-ml-2 md:-ml-4">
    <CarouselItem className="pl-2 md:pl-4">...</CarouselItem>
    <CarouselItem className="pl-2 md:pl-4">...</CarouselItem>
    <CarouselItem className="pl-2 md:pl-4">...</CarouselItem>
  </CarouselContent>
</Carousel>
```

## Orientation

Use the `orientation` prop to set the orientation of the carousel.

<iframe class="demo" src="/demos/carousel-orientation.html" title="carousel-orientation" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

```tsx showLineNumbers /vertical | horizontal/
<Carousel orientation="vertical | horizontal">
  <CarouselContent>
    <CarouselItem>...</CarouselItem>
    <CarouselItem>...</CarouselItem>
    <CarouselItem>...</CarouselItem>
  </CarouselContent>
</Carousel>
```

## Options

You can pass options to the carousel using the `opts` prop. See the [Embla Carousel docs](https://www.embla-carousel.com/api/options/) for more information.

```tsx showLineNumbers {2-5}
<Carousel
  opts={{
    align: "start",
    loop: true,
  }}
>
  <CarouselContent>
    <CarouselItem>...</CarouselItem>
    <CarouselItem>...</CarouselItem>
    <CarouselItem>...</CarouselItem>
  </CarouselContent>
</Carousel>
```

## API

Use a state and the `setApi` props to get an instance of the carousel API.

<iframe class="demo" src="/demos/carousel-api.html" title="carousel-api" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>



## Events

You can listen to events using the api instance from `setApi`.



See the [Embla Carousel docs](https://www.embla-carousel.com/api/events/) for more information on using events.

## Plugins

You can use the `plugins` prop to add plugins to the carousel.

```ts showLineNumbers {1,6-10}
import Autoplay from "embla-carousel-autoplay"

export function Example() {
  return (
    <Carousel
      plugins={[
        Autoplay({
          delay: 2000,
        }),
      ]}
    >
      // ...
    </Carousel>
  )
}
```

<iframe class="demo" src="/demos/carousel-plugin.html" title="carousel-plugin" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/carousel-rtl.html" title="carousel-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

When localizing the carousel for RTL languages, you need to set the `direction` option in the `opts` prop to match the text direction. This ensures the carousel scrolls in the correct direction.

```tsx showLineNumbers {2-5}
<Carousel
  dir={dir}
  opts={{
    direction: dir,
  }}
>
  <CarouselContent>
    <CarouselItem>...</CarouselItem>
    <CarouselItem>...</CarouselItem>
    <CarouselItem>...</CarouselItem>
  </CarouselContent>
  <CarouselPrevious className="rtl:rotate-180" />
  <CarouselNext className="rtl:rotate-180" />
</Carousel>
```

The `direction` option accepts `"ltr"` or `"rtl"` and should match the `dir` prop value. You may also want to rotate the navigation buttons using the `rtl:rotate-180` class to ensure they point in the correct direction.

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="carousel"` |
| `data-slot="carousel-content"` |
| `data-slot="carousel-item"` |
| `data-slot="carousel-previous"` |
| `data-slot="carousel-next"` |

**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → the embla api (`scrollNext()`, `scrollTo(i)`, `on("select", …)`). Markup protocol: see Installation → Behavior protocol.

See the [Embla Carousel docs](https://www.embla-carousel.com/api/) for more information on props and plugins.
