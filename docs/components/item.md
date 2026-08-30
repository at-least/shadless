---
title: "Item"
description: "A versatile component for displaying content with media, title, description, and actions."
---

# Item

A versatile component for displaying content with media, title, description, and actions.

<iframe class="demo" src="/demos/item-demo.html" title="item-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

The `Item` component is a straightforward flex container that can house nearly any type of content. Use it to display a title, description, and actions. Group it with the `ItemGroup` component to create a list of items.

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/item.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/item.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/item.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                             into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/item.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
| `size="outline"` (JSX prop) | `data-size="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
item-group
└── item
    ├── item-header
    ├── item-media
    ├── item-content
    │   ├── item-title
    │   └── item-description
    ├── item-actions
    └── item-footer
```

## Item vs Field

Use `Field` if you need to display a form input such as a checkbox, input, radio, or select.

If you only need to display content such as a title, description, and actions, use `Item`.

## Variant

Use the `variant` prop to change the visual style of the item.

<iframe class="demo" src="/demos/item-variant.html" title="item-variant" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Size

Use the `size` prop to change the size of the item. Available sizes are `default`, `sm`, and `xs`.

<iframe class="demo" src="/demos/item-size.html" title="item-size" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Icon

Use `ItemMedia` with `variant="icon"` to display an icon.

<iframe class="demo" src="/demos/item-icon.html" title="item-icon" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Avatar

You can use `ItemMedia` with `variant="avatar"` to display an avatar.

<iframe class="demo" src="/demos/item-avatar.html" title="item-avatar" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Image

Use `ItemMedia` with `variant="image"` to display an image.

<iframe class="demo" src="/demos/item-image.html" title="item-image" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Group

Use `ItemGroup` to group related items together.

<iframe class="demo" src="/demos/item-group.html" title="item-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Header

Use `ItemHeader` to add a header above the item content.

<iframe class="demo" src="/demos/item-header.html" title="item-header" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Link

Use the `asChild` prop to render the item as a link. The hover and focus states will be applied to the anchor element.

<iframe class="demo" src="/demos/item-link.html" title="item-link" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

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

## Dropdown

<iframe class="demo" src="/demos/item-dropdown.html" title="item-dropdown" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/item-rtl.html" title="item-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="item-group"` |
| `data-slot="item-separator"` |
| `data-slot="item"` |
| `data-slot="item-media"` |
| `data-slot="item-content"` |
| `data-slot="item-title"` |
| `data-slot="item-description"` |
| `data-slot="item-actions"` |
| `data-slot="item-header"` |
| `data-slot="item-footer"` |

### Item

The main component for displaying content with media, title, description, and actions.

| Prop      | Type                                | Default     |
| --------- | ----------------------------------- | ----------- |
| `variant` | `"default" \| "outline" \| "muted"` | `"default"` |
| `size`    | `"default" \| "sm" \| "xs"`         | `"default"` |
| `asChild` | `boolean`                           | `false`     |

### ItemGroup

A container that groups related items together with consistent styling.

```tsx
<ItemGroup>
  <Item />
  <Item />
</ItemGroup>
```

### ItemSeparator

A separator between items in a group.

```tsx
<ItemGroup>
  <Item />
  <ItemSeparator />
  <Item />
</ItemGroup>
```

### ItemMedia

Use `ItemMedia` to display media content such as icons, images, or avatars.

| Prop      | Type                             | Default     |
| --------- | -------------------------------- | ----------- |
| `variant` | `"default" \| "icon" \| "image"` | `"default"` |

```tsx
<ItemMedia variant="icon">
  <Icon />
</ItemMedia>
```

```tsx
<ItemMedia variant="image">
  <img src="..." alt="..." />
</ItemMedia>
```

### ItemContent

Wraps the title and description of the item.

```tsx
<ItemContent>
  <ItemTitle>Title</ItemTitle>
  <ItemDescription>Description</ItemDescription>
</ItemContent>
```

### ItemTitle

Displays the title of the item.

```tsx
<ItemTitle>Item Title</ItemTitle>
```

### ItemDescription

Displays the description of the item.

```tsx
<ItemDescription>Item description</ItemDescription>
```

### ItemActions

Container for action buttons or other interactive elements.

```tsx
<ItemActions>
  <Button>Action</Button>
</ItemActions>
```

### ItemHeader

Displays a header above the item content.

```tsx
<Item>
  <ItemHeader>Header</ItemHeader>
  <ItemContent>...</ItemContent>
</Item>
```

### ItemFooter

Displays a footer below the item content.

```tsx
<Item>
  <ItemContent>...</ItemContent>
  <ItemFooter>Footer</ItemFooter>
</Item>
```
