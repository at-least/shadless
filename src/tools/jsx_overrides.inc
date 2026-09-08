vec![
    ("attachment", r#"```tsx showLineNumbers
<Dialog>
  <Attachment>
    {/* media, content, actions */}
    <DialogTrigger asChild>
      <AttachmentTrigger aria-label="Preview research-summary.pdf" />
    </DialogTrigger>
  </Attachment>
  <DialogContent>{/* ... */}</DialogContent>
</Dialog>
```
"#, r#"```html showLineNumbers
<div data-slot="attachment">
  <!-- media, content, actions -->
  <button data-slot="dialog-trigger" id="<k>-trigger" aria-label="Preview research-summary.pdf"></button>
</div>
<template id="<k>-portal">
  <div data-slot="dialog-content"><!-- ... --></div>
</template>
```"#),
    ("attachment", r#"```tsx showLineNumbers
<AttachmentTrigger asChild>
  <a
    href={url}
    target="_blank"
    rel="noreferrer"
    aria-label="Open workspace.png"
  />
</AttachmentTrigger>
```
"#, r#"```html showLineNumbers
<a data-slot="attachment-trigger" href="/workspace.png" target="_blank" rel="noreferrer" aria-label="Open workspace.png"></a>
```"#),
    ("breadcrumb", r#"You can compose `<BreadcrumbItem />` with a `<DropdownMenu />` to create a dropdown in the breadcrumb."#, r#"You can compose a `data-slot="breadcrumb-item"` with a dropdown-menu trigger (see the [Dropdown Menu](/docs/components/dropdown-menu) page) to create a dropdown in the breadcrumb."#),
    ("bubble", r#"```tsx showLineNumbers
<Bubble variant="muted" align="end">
  <BubbleContent asChild>
    <button type="button" onClick={onReply}>
      I forgot my password
    </button>
  </BubbleContent>
</Bubble>
```
"#, r#"```html showLineNumbers
<div data-slot="bubble" data-variant="muted" data-align="end">
  <button type="button" data-slot="bubble-content">
    I forgot my password
  </button>
</div>
```

There is no `asChild`: put `data-slot="bubble-content"` on the real `<button>` (or `<a>`) itself, and wire your click handler to it like any other element."#),
    ("hover-card", r#"Use `openDelay` and `closeDelay` on the `HoverCard` to control when the card opens and
closes.

```tsx showLineNumbers
<HoverCard openDelay={100} closeDelay={200}>
  <HoverCardTrigger>Hover</HoverCardTrigger>
  <HoverCardContent>Content</HoverCardContent>
</HoverCard>
```
"#, r#"shadless's hover-card glue hardcodes `openDelay: 700, closeDelay: 300` (Radix's own defaults) in `dist/js/hover-card.js` — there is no markup prop to change them; edit that file directly if you need different timing."#),
    ("hover-card", r#"## Positioning

Use the `side` and `align` props on `HoverCardContent` to control placement.

```tsx showLineNumbers
<HoverCard>
  <HoverCardTrigger>Hover</HoverCardTrigger>
  <HoverCardContent side="top" align="start">
    Content
  </HoverCardContent>
</HoverCard>
```
"#, r#"## Positioning

shadless's hover-card glue doesn't expose `side`/`align` through markup either — `popperOptions` in `dist/js/hover-card.js` only sets `sideOffset: 4`, so placement is whatever the kernel's popper default is (auto-flipping on collision); edit that file to pass a preferred placement."#),
    ("carousel", r#"## Options

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
"#, r#"## Options

shadless's carousel glue (`dist/js/carousel.js`) calls Embla with a fixed options object (`axis`/`direction`, derived from the markup) and nothing else — `align`/`loop`/every other [Embla option](https://www.embla-carousel.com/docs/api/options) is not exposed through markup. To change them, edit the `EmblaCarousel(viewport, { … })` call in that file directly; there is no declarative `opts` surface."#),
    ("carousel", r#"## API

Use a state and the `setApi` props to get an instance of the carousel API.

<ComponentPreview
  styleName="radix-nova"
  name="carousel-api"
  previewClassName="sm:h-[32rem]"
/>

```tsx showLineNumbers {1,4,22}
import { type CarouselApi } from "@/components/ui/carousel"

export function Example() {
  const [api, setApi] = React.useState<CarouselApi>()
  const [current, setCurrent] = React.useState(0)
  const [count, setCount] = React.useState(0)

  React.useEffect(() => {
    if (!api) {
      return
    }

    setCount(api.scrollSnapList().length)
    setCurrent(api.selectedScrollSnap() + 1)

    api.on("select", () => {
      setCurrent(api.selectedScrollSnap() + 1)
    })
  }, [api])

  return (
    <Carousel setApi={setApi}>
      <CarouselContent>
        <CarouselItem>...</CarouselItem>
        <CarouselItem>...</CarouselItem>
        <CarouselItem>...</CarouselItem>
      </CarouselContent>
    </Carousel>
  )
}
```

## Events

You can listen to events using the api instance from `setApi`.

```tsx showLineNumbers {1,4-14,16}
import { type CarouselApi } from "@/components/ui/carousel"

export function Example() {
  const [api, setApi] = React.useState<CarouselApi>()

  React.useEffect(() => {
    if (!api) {
      return
    }

    api.on("select", () => {
      // Do something on select.
    })
  }, [api])

  return (
    <Carousel setApi={setApi}>
      <CarouselContent>
        <CarouselItem>...</CarouselItem>
        <CarouselItem>...</CarouselItem>
        <CarouselItem>...</CarouselItem>
      </CarouselContent>
    </Carousel>
  )
}
```
"#, r#"## API

Use a state and the `setApi` props to get an instance of the carousel API.

<ComponentPreview
  styleName="radix-nova"
  name="carousel-api"
  previewClassName="sm:h-[32rem]"
/>

```js showLineNumbers
const api = shadless.get(document.querySelector('[data-slot="carousel"]'))
let current = api.selectedScrollSnap() + 1
const count = api.scrollSnapList().length

api.on("select", () => {
  current = api.selectedScrollSnap() + 1
})
```

## Events

You can listen to events on the instance `shadless.get(rootEl)` returns — it's the real Embla api, so its own `.on()` works exactly as [Embla's docs](https://www.embla-carousel.com/docs/api/events) describe.

```js showLineNumbers
const api = shadless.get(document.querySelector('[data-slot="carousel"]'))

api.on("select", () => {
  // Do something on select.
})
```

"#),
    ("carousel", r#"## Plugins

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
"#, r#"## Plugins

Plugins are an Embla concept (`EmblaCarousel(viewport, options, plugins)`) — shadless's glue (`dist/js/carousel.js`) doesn't pass a plugins array, so `embla-carousel-autoplay` and friends aren't available without editing that file yourself to add one."#),
    ("navigation-menu", r#"## Link Component

Use the `asChild` prop to compose a custom link component such as Next.js `Link`.

```tsx showLineNumbers
import Link from "next/link"

import {
  NavigationMenuItem,
  NavigationMenuLink,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu"

export function NavigationMenuDemo() {
  return (
    <NavigationMenuItem>
      <NavigationMenuLink asChild className={navigationMenuTriggerStyle()}>
        <Link href="/docs">Documentation</Link>
      </NavigationMenuLink>
    </NavigationMenuItem>
  )
}
```
"#, r#"## Link Component

There's no `asChild`/framework-Link composition in shadless — `navigation-menu-link` is a plain `<a>`, so just point `href` at whatever route your app uses:

```html showLineNumbers
<li data-slot="navigation-menu-item">
  <a data-slot="navigation-menu-link" href="/docs">Documentation</a>
</li>
```

"#),
    ("pagination", r#"## Next.js

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

<Callout className="mt-6">

**Note:** We are making updates to the cli to automatically do this for you.

</Callout>

"#, r#"shadless has no CLI or framework Link component — `PaginationLink` always renders a real `<a>`; point `href` at whatever your router expects.

"#),
    ("spinner", r#"## Customization

You can replace the default spinner icon with any other icon by editing the `Spinner` component.

<ComponentPreview styleName="radix-nova" name="spinner-custom" />

```tsx showLineNumbers title="components/ui/spinner.tsx"
import { LoaderIcon } from "lucide-react"

import { cn } from "@/lib/utils"

function Spinner({ className, ...props }: React.ComponentProps<"svg">) {
  return (
    <LoaderIcon
      role="status"
      aria-label="Loading"
      className={cn("size-4 animate-spin", className)}
      {...props}
    />
  )
}

export { Spinner }
```
"#, r#"## Customization

You can replace the default spinner icon with any other icon — swap the inner `<svg>` in the spinner markup for a different one.

<ComponentPreview styleName="radix-nova" name="spinner-custom" />

```html showLineNumbers
<svg data-slot="spinner">
  <!-- swap this inner <svg> for a different icon's markup -->
  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="..."/></svg>
</svg>
```

The `size-4 animate-spin` styling ships as a `[data-slot="spinner"]` CSS rule (`dist/css/spinner.css`), not an inline class — override it the same way, by targeting that selector.

"#),
    ("rtl", r#"For now, make sure you pass in the `dir` prop to portal elements."#, r#"For now, set the `dir` attribute on the portal content element itself."#),
    ("rtl", r#"When you install components, the CLI automatically transforms physical positioning classes to logical equivalents, so your components work seamlessly in both LTR and RTL contexts."#, r#"shadless has no install step that rewrites anything: the classes you see are the pinned registry's own. What makes the difference is which example you start from — the pages below ship an RTL variant beside the LTR one wherever upstream authored it."#),
    ("rtl", r#"The CLI also handles animation classes, automatically transforming physical directional animations to their logical equivalents. For example, `slide-in-from-right` becomes `slide-in-from-end`.

This ensures animations like dropdowns, popovers, and tooltips animate in the correct direction based on the document's text direction."#, r#"Animation utilities are emitted as the registry writes them, physical names included — nothing rewrites `slide-in-from-right` into `slide-in-from-end` for you. Where upstream's own RTL example uses the logical name, the shipped `-rtl` page carries it; elsewhere, set the direction on the element yourself."#),
    ("message-scroller", r#"```tsx
<MessageScrollerContent aria-busy={status === "streaming"}>
  {/* messages */}
</MessageScrollerContent>
```

Pass `aria-busy` while a turn streams if announcements should wait for the
completed message row.
"#, r#"```html
<div data-slot="message-scroller-content" aria-busy="true">
  <!-- messages -->
</div>
```

Toggle `aria-busy` yourself while a turn streams, so announcements wait for the completed message row — there is no built-in status tracking.
"#),
    ("message-scroller", r#"It uses `role="region"`, `aria-label="Messages"`, and `tabIndex={0}`,"#, r#"It uses `role="region"`, `aria-label="Messages"`, and `tabindex="0"`,"#),
    ("message-scroller", r#"`MessageScrollerButton` renders a real button. When there is nothing to scroll
toward, it sets `inert`, uses `tabIndex={-1}`, and exposes `data-active="false"`
so inactive scroll controls do not create extra focus stops."#, r#"The scroll button is a real `<button data-slot="message-scroller-button">`, and when there is nothing to scroll toward it exposes `data-active="false"`. Nothing else follows from that here: no shipped stylesheet reads `data-active` on this slot (only `dist/css/tabs.css` and `dist/css/navigation-menu.css` use that attribute at all), and the button sets neither `inert` nor `tabindex="-1"`. Style the attribute and take the inactive control out of the focus order yourself."#),
    ("message-scroller", r#"## Unstyled

The behavior in `MessageScroller` comes from the `@shadcn/react` package. To use
it directly with your own markup and styles, see
[Message Scroller](/docs/react/message-scroller) under @shadcn/react.
"#, r#"## Unstyled

There is no behavior package to reach past: the shipped markup IS the whole component. Copy any example on this page and restyle it — the `data-slot` names are the only contract.
"#),
    ("carousel", r#"When localizing the carousel for RTL languages, you need to set the `direction` option in the `opts` prop to match the text direction. This ensures the carousel scrolls in the correct direction.

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
"#, r#"When localizing the carousel for RTL languages, set `dir="rtl"` on the carousel root — the runtime reads that attribute directly (falling back to `<html dir>`). There's no `opts` to configure; `opts` isn't read at all (see [Options](#options)).

```html showLineNumbers
<div data-slot="carousel" dir="rtl">
  <div data-slot="carousel-content">
    <div data-slot="carousel-item">...</div>
    <div data-slot="carousel-item">...</div>
    <div data-slot="carousel-item">...</div>
  </div>
  <button data-slot="carousel-previous" class="rtl:rotate-180"></button>
  <button data-slot="carousel-next" class="rtl:rotate-180"></button>
</div>
```
"#),
    ("table", r#"A table showing actions for each row using a `<DropdownMenu />` component."#, r#"A table showing actions for each row using a dropdown-menu trigger (see the [Dropdown Menu](/docs/components/dropdown-menu) page)."#),
    ("rtl", r#"```tsx showLineNumbers /dir="rtl"/
<Popover>
  <PopoverTrigger>Open</PopoverTrigger>
  <PopoverContent dir="rtl">
    <div>Content</div>
  </PopoverContent>
</Popover>
```
"#, r#"```html showLineNumbers /dir="rtl"/
<button data-slot="popover-trigger" id="<k>-trigger">Open</button>
<template id="<k>-portal">
  <div data-slot="popover-content" dir="rtl">
    <div>Content</div>
  </div>
</template>
```"#),
    ("rtl", r#"```tsx showLineNumbers /dir="rtl"/
<Tooltip>
  <TooltipTrigger>Open</TooltipTrigger>
  <TooltipContent dir="rtl">
    <div>Content</div>
  </TooltipContent>
</Tooltip>
```
"#, r#"```html showLineNumbers /dir="rtl"/
<button data-slot="tooltip-trigger" id="<k>-trigger">Open</button>
<template id="<k>-portal">
  <div data-slot="tooltip-content" dir="rtl">
    <div>Content</div>
  </div>
</template>
```"#),
    ("carousel", r#"```tsx showLineNumbers /vertical | horizontal/
<Carousel orientation="vertical | horizontal">
  <CarouselContent>
    <CarouselItem>...</CarouselItem>
    <CarouselItem>...</CarouselItem>
    <CarouselItem>...</CarouselItem>
  </CarouselContent>
</Carousel>
```
"#, r#"```html showLineNumbers /vertical/
<div data-slot="carousel" data-orientation="vertical">
  <div data-slot="carousel-content">
    <div data-slot="carousel-item">...</div>
    <div data-slot="carousel-item">...</div>
    <div data-slot="carousel-item">...</div>
  </div>
</div>
```

`dist/js/carousel.js` reads `data-orientation="vertical"` on the root; with no such attribute it falls back to whether the content element carries `flex-col`, and otherwise scrolls horizontally.
"#),
    ("item", r#"Use the `asChild` prop to render the item as a link. The hover and focus states will be applied to the anchor element.

<ComponentPreview styleName="radix-nova" name="item-link" />

```tsx showLineNumbers
<Item asChild>
  <a href="/dashboard">
    <ItemMedia variant="icon">
      <HomeIcon />
    </ItemMedia>
    <ItemContent>
      <ItemTitle>Dashboard</ItemTitle>
      <ItemDescription>Overview of your account and activity.</ItemDescription>
    </ItemContent>
  </a>
</Item>
```
"#, r#"shadless has no `asChild` — put `data-slot="item"` on the anchor itself. The hover and focus states apply to the anchor.

<ComponentPreview styleName="radix-nova" name="item-link" />

```html showLineNumbers
<a data-slot="item" href="/dashboard">
  <div data-slot="item-media" data-variant="icon">
    <!-- lucide "home" icon -->
  </div>
  <div data-slot="item-content">
    <div data-slot="item-title">Dashboard</div>
    <p data-slot="item-description">Overview of your account and activity.</p>
  </div>
</a>
```
"#),
    ("marker", r##"Turn a marker into a link or button with the `asChild` prop on `Marker`.

<ComponentPreview
  styleName="radix-rhea"
  name="marker-link-button"
  previewClassName="h-auto theme-blue"
/>

```tsx showLineNumbers
import { Marker, MarkerContent } from "@/components/ui/marker"

export function MarkerLinkDemo() {
  return (
    <Marker asChild>
      <a href="#">
        <MarkerContent>View the pull request</MarkerContent>
      </a>
    </Marker>
  )
}
```
"##, r##"Turn a marker into a link or button by putting `data-slot="marker"` on the `<a>` or `<button>` itself — shadless has no `asChild`.

<ComponentPreview
  styleName="radix-rhea"
  name="marker-link-button"
  previewClassName="h-auto theme-blue"
/>

```html showLineNumbers
<a data-slot="marker" href="#">
  <span data-slot="marker-content">View the pull request</span>
</a>
```
"##),
    ("marker", r#"When a marker links or triggers an action, render it as a real `<button>` or `<a>` with the `asChild` prop so it is focusable and exposes the correct role. The accessible name comes from the marker text.

```tsx showLineNumbers
<Marker asChild>
  <a href="/files">
    <MarkerIcon>
      <FileTextIcon />
    </MarkerIcon>
    <MarkerContent>Explored 4 files</MarkerContent>
  </a>
</Marker>
```
"#, r#"When a marker links or triggers an action, put `data-slot="marker"` on a real `<button>` or `<a>` so it is focusable and exposes the correct role. The accessible name comes from the marker text.

```html showLineNumbers
<a data-slot="marker" href="/files">
  <span data-slot="marker-icon"><!-- lucide "file-text" icon --></span>
  <span data-slot="marker-content">Explored 4 files</span>
</a>
```
"#),
    ("direction", r#"## useDirection

The `useDirection` hook is used to get the current direction of the application.

```tsx showLineNumbers
import { useDirection } from "@/components/ui/direction"

function MyComponent() {
  const direction = useDirection()
  return <div>Current direction: {direction}</div>
}
```
"#, r#"## Reading the direction

There is no `useDirection` hook and no `dist/js/direction.js` — direction is the DOM's own `dir` attribute. Read it the way the runtime itself does (`isRtl` in `dist/shadless.js`): the nearest `[dir]` ancestor, falling back to `<html dir>`.

```js showLineNumbers
function directionOf(el) {
  var d = el.closest("[dir]")
  return (d ? d.getAttribute("dir") : document.documentElement.getAttribute("dir")) || "ltr"
}
```
"#),
]
