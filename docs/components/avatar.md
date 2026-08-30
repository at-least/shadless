---
title: "Avatar"
description: "An image element with a fallback for representing the user."
---

# Avatar

An image element with a fallback for representing the user.

<p class="page-links">[doc](https://www.radix-ui.com/primitives/docs/components/avatar) · [api](https://www.radix-ui.com/primitives/docs/components/avatar#api-reference)</p>

<iframe class="demo" src="/demos/avatar-demo.html" title="avatar-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/avatar.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/avatar.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/avatar.html` | component markup — copy your page's structure from here |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/avatar.js` | this component's behavior — registers with the base |


**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/avatar.js"></script>
```

**Copy the markup from                               into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup.


No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/avatar.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
avatar
├── avatar-image
├── avatar-fallback
└── avatar-badge
```

## Basic

A basic avatar component with an image and a fallback.

<iframe class="demo" src="/demos/avatar-basic.html" title="avatar-basic" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Badge

Use the `AvatarBadge` component to add a badge to the avatar. The badge is positioned at the bottom right of the avatar.

<iframe class="demo" src="/demos/avatar-badge.html" title="avatar-badge" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

Use the `className` prop to add custom styles to the badge such as custom colors, sizes, etc.

```tsx showLineNumbers
<Avatar>
  <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
  <AvatarFallback>CN</AvatarFallback>
  <AvatarBadge className="bg-green-600 dark:bg-green-800" />
</Avatar>
```

## Badge with Icon

You can also use an icon inside `<AvatarBadge>`.

<iframe class="demo" src="/demos/avatar-badge-icon.html" title="avatar-badge-icon" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Avatar Group

Use the `AvatarGroup` component to add a group of avatars.

<iframe class="demo" src="/demos/avatar-group.html" title="avatar-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Avatar Group Count

Use `<AvatarGroupCount>` to add a count to the group.

<iframe class="demo" src="/demos/avatar-group-count.html" title="avatar-group-count" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Avatar Group with Icon

You can also use an icon inside `<AvatarGroupCount>`.

<iframe class="demo" src="/demos/avatar-group-count-icon.html" title="avatar-group-count-icon" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Sizes

Use the `size` prop to change the size of the avatar.

<iframe class="demo" src="/demos/avatar-size.html" title="avatar-size" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Dropdown

You can use the `Avatar` component as a trigger for a dropdown menu.

<iframe class="demo" src="/demos/avatar-dropdown.html" title="avatar-dropdown" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/avatar-rtl.html" title="avatar-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="avatar"` |
| `data-slot="avatar-image"` |
| `data-slot="avatar-fallback"` |
| `data-slot="avatar-badge"` |
| `data-slot="avatar-group"` |
| `data-slot="avatar-group-count"` |

**Runtime:** `avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

### Avatar

The `Avatar` component is the root component that wraps the avatar image and fallback.

| Prop        | Type                        | Default     |
| ----------- | --------------------------- | ----------- |
| `size`      | `"default" \| "sm" \| "lg"` | `"default"` |
| `className` | `string`                    | -           |

### AvatarImage

The `AvatarImage` component displays the avatar image. It is a plain `<img data-slot="avatar-image">` — the shadless runtime switches to the fallback from its load state.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `src`       | `string` | -       |
| `alt`       | `string` | -       |
| `className` | `string` | -       |

### AvatarFallback

The `AvatarFallback` component displays a fallback when the image fails to load. It is a plain `<span data-slot="avatar-fallback">` shown by the shadless runtime while the image is loading or failed.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### AvatarBadge

The `AvatarBadge` component displays a badge indicator on the avatar, typically positioned at the bottom right.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### AvatarGroup

The `AvatarGroup` component displays a group of avatars with overlapping styling.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### AvatarGroupCount

The `AvatarGroupCount` component displays a count indicator in an avatar group, typically showing the number of additional avatars.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

For more information about Radix UI Avatar props, see the [Radix UI documentation](https://www.radix-ui.com/primitives/docs/components/avatar#api-reference).
