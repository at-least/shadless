---
title: "Empty"
description: "Use the Empty component to display an empty state."
---

# Empty

Use the Empty component to display an empty state.

<iframe class="demo" src="/demos/empty-demo.html" title="empty-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/empty.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/empty.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/empty.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/empty.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `variant="outline"` (JSX prop) | `data-variant="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
empty
├── empty-header
│   ├── empty-icon
│   ├── empty-title
│   └── empty-description
└── empty-content
```

## Outline

Use the `border` utility class to create an outline empty state.

<iframe class="demo" src="/demos/empty-outline.html" title="empty-outline" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Background

Use the `bg-*` and `bg-gradient-*` utilities to add a background to the empty state.

<iframe class="demo" src="/demos/empty-background.html" title="empty-background" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Avatar

Use the `EmptyMedia` component to display an avatar in the empty state.

<iframe class="demo" src="/demos/empty-avatar.html" title="empty-avatar" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Avatar Group

Use the `EmptyMedia` component to display an avatar group in the empty state.

<iframe class="demo" src="/demos/empty-avatar-group.html" title="empty-avatar-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## InputGroup

You can add an `InputGroup` component to the `EmptyContent` component.

<iframe class="demo" src="/demos/empty-input-group.html" title="empty-input-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/empty-rtl.html" title="empty-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="empty"` |
| `data-slot="empty-header"` |
| `data-slot="empty-icon"` |
| `data-slot="empty-title"` |
| `data-slot="empty-description"` |
| `data-slot="empty-content"` |

### Empty

The main component of the empty state. Wraps the `EmptyHeader` and `EmptyContent` components.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<Empty>
  <EmptyHeader />
  <EmptyContent />
</Empty>
```

### EmptyHeader

The `EmptyHeader` component wraps the empty media, title, and description.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<EmptyHeader>
  <EmptyMedia />
  <EmptyTitle />
  <EmptyDescription />
</EmptyHeader>
```

### EmptyMedia

Use the `EmptyMedia` component to display the media of the empty state such as an icon or an image. You can also use it to display other components such as an avatar.

| Prop        | Type                  | Default   |
| ----------- | --------------------- | --------- |
| `variant`   | `"default" \| "icon"` | `default` |
| `className` | `string`              |           |

```tsx
<EmptyMedia variant="icon">
  <Icon />
</EmptyMedia>
```

```tsx
<EmptyMedia>
  <Avatar>
    <AvatarImage src="..." />
    <AvatarFallback>CN</AvatarFallback>
  </Avatar>
</EmptyMedia>
```

### EmptyTitle

Use the `EmptyTitle` component to display the title of the empty state.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<EmptyTitle>No data</EmptyTitle>
```

### EmptyDescription

Use the `EmptyDescription` component to display the description of the empty state.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<EmptyDescription>You do not have any notifications.</EmptyDescription>
```

### EmptyContent

Use the `EmptyContent` component to display the content of the empty state such as a button, input or a link.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` |         |

```tsx
<EmptyContent>
  <Button>Add Project</Button>
</EmptyContent>
```
