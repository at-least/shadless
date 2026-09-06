---
title: "Avatar"
description: "An image element with a fallback for representing the user."
weight: 6
---

# Avatar

An image element with a fallback for representing the user.

{% <demo name="avatar-demo" status="authored"> %}
<iframe class="demo" src="/demos/avatar-demo.html" title="avatar-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-demo.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-demo.html
<div class="flex flex-row flex-wrap items-center gap-6 md:gap-12">
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover grayscale"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png" /><span
      data-slot="avatar-badge"
      class="text-primary-foreground ring-background absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&amp;&gt;svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&amp;&gt;svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&amp;&gt;svg]:size-2 bg-green-600 dark:bg-green-800"
    ></span
  ></span>
  <div
    data-slot="avatar-group"
    class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
  >
    <span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@shadcn"
        src="https://github.com/shadcn.png" /></span
    ><span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@maxleiter"
        src="https://github.com/maxleiter.png" /></span
    ><span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@evilrabbit"
        src="https://github.com/evilrabbit.png"
    /></span>
    <div
      data-slot="avatar-group-count"
      class="bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&amp;&gt;svg]:size-4 group-has-data-[size=lg]/avatar-group:[&amp;&gt;svg]:size-5 group-has-data-[size=sm]/avatar-group:[&amp;&gt;svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background"
    >
      +3
    </div>
  </div>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

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
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/avatar.js` | this component's behavior — registers with the base |

**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/avatar.js"></script>
```

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

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

{% <demo name="avatar-basic" status="authored"> %}
<iframe class="demo" src="/demos/avatar-basic.html" title="avatar-basic" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-basic.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-basic.html
<span
  data-slot="avatar"
  data-size="default"
  class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
  ><img
    data-slot="avatar-image"
    class="rounded-full aspect-square size-full object-cover grayscale"
    alt="@shadcn"
    src="https://github.com/shadcn.png"
/></span>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Badge

Use the `AvatarBadge` component to add a badge to the avatar. The badge is positioned at the bottom right of the avatar.

{% <demo name="avatar-badge" status="authored"> %}
<iframe class="demo" src="/demos/avatar-badge.html" title="avatar-badge" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-badge.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-badge.html
<span
  data-slot="avatar"
  data-size="default"
  class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
  ><img
    data-slot="avatar-image"
    class="rounded-full aspect-square size-full object-cover"
    alt="@shadcn"
    src="https://github.com/shadcn.png" /><span
    data-slot="avatar-badge"
    class="text-primary-foreground ring-background absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&amp;&gt;svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&amp;&gt;svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&amp;&gt;svg]:size-2 bg-green-600 dark:bg-green-800"
  ></span
></span>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

Use the `className` prop to add custom styles to the badge such as custom colors, sizes, etc.

```html
<span data-slot="avatar">
  <img data-slot="avatar-image" src="https://github.com/shadcn.png" alt="@shadcn" />
  <span data-slot="avatar-fallback">CN</span>
  <span data-slot="avatar-badge" class="bg-green-600 dark:bg-green-800" />
</span>
```

## Badge with Icon

You can also use an icon inside `data-slot="avatar-badge"`.

{% <demo name="avatar-badge-icon" status="authored"> %}
<iframe class="demo" src="/demos/avatar-badge-icon.html" title="avatar-badge-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-badge-icon.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-badge-icon.html
<span
  data-slot="avatar"
  data-size="default"
  class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten grayscale"
  ><img
    data-slot="avatar-image"
    class="rounded-full aspect-square size-full object-cover"
    alt="@pranathip"
    src="https://github.com/pranathip.png" /><span
    data-slot="avatar-badge"
    class="bg-primary text-primary-foreground ring-background absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&amp;&gt;svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&amp;&gt;svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&amp;&gt;svg]:size-2"
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
      class="lucide lucide-plus"
    >
      <path d="M5 12h14"></path>
      <path d="M12 5v14"></path></svg></span
></span>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Avatar Group

Use the `AvatarGroup` component to add a group of avatars.

{% <demo name="avatar-group" status="authored"> %}
<iframe class="demo" src="/demos/avatar-group.html" title="avatar-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-group.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-group.html
<div
  data-slot="avatar-group"
  class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
>
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@maxleiter"
      src="https://github.com/maxleiter.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png"
  /></span>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Avatar Group Count

Use `data-slot="avatar-group-count"` to add a count to the group.

{% <demo name="avatar-group-count" status="authored"> %}
<iframe class="demo" src="/demos/avatar-group-count.html" title="avatar-group-count" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-group-count.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-group-count.html
<div
  data-slot="avatar-group"
  class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
>
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@maxleiter"
      src="https://github.com/maxleiter.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png"
  /></span>
  <div
    data-slot="avatar-group-count"
    class="bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&amp;&gt;svg]:size-4 group-has-data-[size=lg]/avatar-group:[&amp;&gt;svg]:size-5 group-has-data-[size=sm]/avatar-group:[&amp;&gt;svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background"
  >
    +3
  </div>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Avatar Group with Icon

You can also use an icon inside `data-slot="avatar-group-count"`.

{% <demo name="avatar-group-count-icon" status="authored"> %}
<iframe class="demo" src="/demos/avatar-group-count-icon.html" title="avatar-group-count-icon" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-group-count-icon.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-group-count-icon.html
<div
  data-slot="avatar-group"
  class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
>
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@maxleiter"
      src="https://github.com/maxleiter.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png"
  /></span>
  <div
    data-slot="avatar-group-count"
    class="bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&amp;&gt;svg]:size-4 group-has-data-[size=lg]/avatar-group:[&amp;&gt;svg]:size-5 group-has-data-[size=sm]/avatar-group:[&amp;&gt;svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background"
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
      class="lucide lucide-plus"
    >
      <path d="M5 12h14"></path>
      <path d="M12 5v14"></path>
    </svg>
  </div>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Sizes

Use the `size` prop to change the size of the avatar.

{% <demo name="avatar-size" status="authored"> %}
<iframe class="demo" src="/demos/avatar-size.html" title="avatar-size" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-size.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-size.html
<div class="flex flex-wrap items-center gap-2 grayscale">
  <span
    data-slot="avatar"
    data-size="sm"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="lg"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@shadcn"
      src="https://github.com/shadcn.png"
  /></span>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## Dropdown

You can use the `Avatar` component as a trigger for a dropdown menu.

{% <demo name="avatar-dropdown" status="authored"> %}
<iframe class="demo" src="/demos/avatar-dropdown.html" title="avatar-dropdown" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-dropdown.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=avatar-dropdown.html
<button
  data-slot="dropdown-menu-trigger"
  data-variant="ghost"
  data-size="icon"
  class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground size-8 rounded-full"
  type="button"
  id="m0-trigger"
  aria-haspopup="menu"
  aria-expanded="false"
  data-state="closed"
  data-radixuigo-menu-trigger="m0"
>
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><span
      data-slot="avatar-fallback"
      class="bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs"
      >CN</span
    ></span
  >
</button>
<template id="m0-tpl">
  <div
    data-side="bottom"
    data-align="start"
    role="menu"
    aria-orientation="vertical"
    data-state="open"
    data-radix-menu-content=""
    dir="ltr"
    id="m0"
    aria-labelledby="m0-trigger"
    data-slot="dropdown-menu-content"
    class="data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-32 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-dropdown-menu-content-available-height) origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden overflow-y-auto data-[state=closed]:overflow-hidden w-32"
    tabindex="-1"
    data-orientation="vertical"
    style="
      outline: none;
      --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin);
      --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);
      --radix-dropdown-menu-content-available-height: var(--radix-popper-available-height);
      --radix-dropdown-menu-trigger-width: var(--radix-popper-anchor-width);
      --radix-dropdown-menu-trigger-height: var(--radix-popper-anchor-height);
      pointer-events: auto;
    "
  >
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Profile
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Billing
      </div>
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="default"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Settings
      </div>
    </div>
    <div
      role="separator"
      aria-orientation="horizontal"
      data-slot="dropdown-menu-separator"
      class="bg-border -mx-1 my-1 h-px"
    ></div>
    <div role="group" data-slot="dropdown-menu-group">
      <div
        role="menuitem"
        data-slot="dropdown-menu-item"
        data-variant="destructive"
        class="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&amp;_svg:not([class*='size-'])]:size-4 group/dropdown-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
        tabindex="-1"
        data-orientation="vertical"
        data-radix-collection-item=""
      >
        Log out
      </div>
    </div>
  </div>
</template>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/dropdown-menu.js
// shadless dropdown-menu behavior — registers with shadless.h.installMenuFamily,
// the shared wireMenu glue also used by context-menu.js (core.js has the full
// story: the two files' bodies were byte-identical but for this line, so the
// body now lives once and both files just point at it).
(function () {
  shadless.register("dropdown-menu", { init: shadless.h.installMenuFamily })
})()

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

{% <demo name="avatar-rtl" status="authored"> %}
<iframe class="demo" src="/demos/avatar-rtl.html" title="avatar-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/avatar-rtl.html">Open the demo page</a> · <a href="/demos/avatar-rtl-he.html">HE</a> · <a href="/demos/avatar-rtl-en.html">EN</a></p>

{% <codegroup> %}
```text,name=avatar-rtl.html
<div class="flex flex-row flex-wrap items-center gap-6 md:gap-12" dir="rtl">
  <span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover grayscale"
      alt="@shadcn"
      src="https://github.com/shadcn.png" /></span
  ><span
    data-slot="avatar"
    data-size="default"
    class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
    ><img
      data-slot="avatar-image"
      class="rounded-full aspect-square size-full object-cover"
      alt="@evilrabbit"
      src="https://github.com/evilrabbit.png" /><span
      data-slot="avatar-badge"
      class="text-primary-foreground ring-background absolute end-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&amp;&gt;svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&amp;&gt;svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&amp;&gt;svg]:size-2 bg-green-600 dark:bg-green-800"
    ></span
  ></span>
  <div
    data-slot="avatar-group"
    class="group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background grayscale"
  >
    <span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@shadcn"
        src="https://github.com/shadcn.png" /></span
    ><span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@maxleiter"
        src="https://github.com/maxleiter.png" /></span
    ><span
      data-slot="avatar"
      data-size="default"
      class="size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten"
      ><img
        data-slot="avatar-image"
        class="rounded-full aspect-square size-full object-cover"
        alt="@evilrabbit"
        src="https://github.com/evilrabbit.png"
    /></span>
    <div
      data-slot="avatar-group-count"
      class="bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&amp;&gt;svg]:size-4 group-has-data-[size=lg]/avatar-group:[&amp;&gt;svg]:size-5 group-has-data-[size=sm]/avatar-group:[&amp;&gt;svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background"
    >
      +٣
    </div>
  </div>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/avatar.js
// shadless avatar behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("avatar", { slots: {
    avatar: {
      init: function (av) {
        var img = av.querySelector("[data-slot=avatar-image]")
        var fb = av.querySelector("[data-slot=avatar-fallback]")
        var settle = function () {
          var ok = img.complete && img.naturalWidth > 0
          if (ok) { if (fb) fb.remove() }
          else if (img) img.remove()
        }
        if (img) {
          if (img.complete) settle()
          else { img.addEventListener("load", settle); img.addEventListener("error", settle) }
        }
      },
    },
    // collapsible trigger: toggle root/trigger/content states,
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="avatar"` |
| `data-slot="avatar-image"` |
| `data-slot="avatar-fallback"` |
| `data-slot="avatar-badge"` |
| `data-slot="avatar-group"` |
| `data-slot="avatar-group-count"` |

**Runtime:** `avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup. No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.
