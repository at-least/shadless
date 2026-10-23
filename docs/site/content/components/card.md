---
title: "Card"
description: "Displays a card with header, content, and footer."
weight: 12
---

# Card

Displays a card with header, content, and footer.

{% <demo name="card-demo" status="authored"> %}
<iframe class="demo" src="/demos/card-demo.html" title="card-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-demo.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=card-demo.html
<div
  data-slot="card"
  data-size="default"
  class="*:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card w-full max-w-sm"
>
  <div data-slot="card-header" class="group/card-header">
    <div data-slot="card-title" class="cn-font-heading">Login to your account</div>
    <div data-slot="card-description">Enter your email below to login to your account</div>
    <div data-slot="card-action">
      <button
        data-slot="button"
        data-variant="link"
        data-size="default"
        class="rounded-lg group/button"
      >
        Sign Up
      </button>
    </div>
  </div>
  <div data-slot="card-content">
    <form>
      <div class="flex flex-col gap-6">
        <div class="grid gap-2">
          <label data-slot="label" for="email">Email</label
          ><input
            data-slot="input"
            class="rounded-lg"
            id="email"
            placeholder="m@example.com"
            required=""
            type="email"
          />
        </div>
        <div class="grid gap-2">
          <div class="flex items-center">
            <label data-slot="label" for="password">Password</label
            ><a href="#" class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
              >Forgot your password?</a
            >
          </div>
          <input data-slot="input" class="rounded-lg" id="password" required="" type="password" />
        </div>
      </div>
    </form>
  </div>
  <div data-slot="card-footer" class="flex-col gap-2">
    <button
      data-slot="button"
      data-variant="default"
      data-size="default"
      class="rounded-lg group/button w-full"
      type="submit"
    >
      Login</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg group/button border-border dark:border-input w-full"
    >
      Login with Google
    </button>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Installation

**Add shadless and this component to your Tailwind v4 (>= 4.3) entry:**

```css
@import "shadless";
@import "shadless/card.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build (>= 4.3) |
| `dist/css/card.css` | this component's slot styles (`@apply` source — your build compiles it) |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

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

{% <demo name="card-small" status="authored"> %}
<iframe class="demo" src="/demos/card-small.html" title="card-small" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-small.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=card-small.html
<div
  data-slot="card"
  data-size="sm"
  class="*:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card mx-auto w-full max-w-sm"
>
  <div data-slot="card-header" class="group/card-header">
    <div data-slot="card-title" class="cn-font-heading">Small Card</div>
    <div data-slot="card-description">This card uses the small size variant.</div>
  </div>
  <div data-slot="card-content">
    <p>
      The card component supports a size prop that can be set to "sm" for a more compact appearance.
    </p>
  </div>
  <div data-slot="card-footer">
    <button
      data-slot="button"
      data-variant="outline"
      data-size="sm"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 group/button border-border dark:border-input rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg w-full"
    >
      Action
    </button>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Spacing

In addition to the `size` prop, you can use the `--card-spacing` CSS variable to control the spacing between sections and the inset of card parts.

{% <demo name="card-spacing" status="authored"> %}
<iframe class="demo" src="/demos/card-spacing.html" title="card-spacing" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-spacing.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=card-spacing.html
<div class="mx-auto grid w-full max-w-sm gap-4">
  <div
    role="group"
    dir="ltr"
    data-slot="toggle-group"
    data-variant="outline"
    data-size="sm"
    data-spacing="2"
    data-orientation="horizontal"
    class="group/toggle-group justify-center"
    tabindex="0"
    style="outline: none; --gap: 2"
  >
    <button
      type="button"
      data-state="on"
      role="radio"
      aria-checked="true"
      data-slot="toggle-group-item"
      data-variant="outline"
      data-size="sm"
      data-spacing="2"
      class="group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-lg group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-lg group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-lg group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-lg group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t aria-pressed:bg-muted focus-visible:border-ring aria-invalid:border-destructive data-[state=on]:bg-muted group/toggle border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5"
      tabindex="-1"
      data-radix-collection-item=""
    >
      16px</button
    ><button
      type="button"
      data-state="off"
      role="radio"
      aria-checked="false"
      data-slot="toggle-group-item"
      data-variant="outline"
      data-size="sm"
      data-spacing="2"
      class="group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-lg group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-lg group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-lg group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-lg group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t aria-pressed:bg-muted focus-visible:border-ring aria-invalid:border-destructive data-[state=on]:bg-muted group/toggle border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5"
      tabindex="-1"
      data-radix-collection-item=""
    >
      20px</button
    ><button
      type="button"
      data-state="off"
      role="radio"
      aria-checked="false"
      data-slot="toggle-group-item"
      data-variant="outline"
      data-size="sm"
      data-spacing="2"
      class="group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-lg group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-lg group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-lg group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-lg group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t aria-pressed:bg-muted focus-visible:border-ring aria-invalid:border-destructive data-[state=on]:bg-muted group/toggle border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5"
      tabindex="-1"
      data-radix-collection-item=""
    >
      24px</button
    ><button
      type="button"
      data-state="off"
      role="radio"
      aria-checked="false"
      data-slot="toggle-group-item"
      data-variant="outline"
      data-size="sm"
      data-spacing="2"
      class="group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-lg group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-lg group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-lg group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-lg group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t aria-pressed:bg-muted focus-visible:border-ring aria-invalid:border-destructive data-[state=on]:bg-muted group/toggle border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5"
      tabindex="-1"
      data-radix-collection-item=""
    >
      32px
    </button>
  </div>
  <div
    data-slot="card"
    data-size="default"
    class="*:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card"
  >
    <div data-slot="card-header" class="group/card-header">
      <div data-slot="card-title" class="cn-font-heading">Login to your account</div>
      <div data-slot="card-description">Enter your email below to login to your account</div>
      <div data-slot="card-action">
        <button
          data-slot="button"
          data-variant="link"
          data-size="default"
          class="rounded-lg group/button"
        >
          Sign Up
        </button>
      </div>
    </div>
    <div data-slot="card-content">
      <form>
        <div class="flex flex-col gap-6">
          <div class="grid gap-2">
            <label data-slot="label" for="email-spacing">Email</label
            ><input
              data-slot="input"
              class="rounded-lg"
              id="email-spacing"
              placeholder="m@example.com"
              required=""
              type="email"
            />
          </div>
          <div class="grid gap-2">
            <div class="flex items-center">
              <label data-slot="label" for="password-spacing">Password</label
              ><a href="#" class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                >Forgot your password?</a
              >
            </div>
            <input
              data-slot="input"
              class="rounded-lg"
              id="password-spacing"
              required=""
              type="password"
            />
          </div>
        </div>
      </form>
    </div>
    <div data-slot="card-footer" class="flex-col gap-2">
      <button
        data-slot="button"
        data-variant="default"
        data-size="default"
        class="rounded-lg group/button w-full"
        type="submit"
      >
        Login</button
      ><button
        data-slot="button"
        data-variant="outline"
        data-size="default"
        class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg group/button border-border dark:border-input w-full"
      >
        Login with Google
      </button>
    </div>
  </div>
</div>
```

```js,name=behavior
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/toggle-group.js
// shadless toggle-group behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle-group", { slots: {
    // toggle-group: single (items role=radio) selects exclusively with click-
    // again deselecting; multiple (items as plain buttons) toggles items independently.
    // Root is role=group in BOTH modes (radix current semantics — radiogroup
    // was the old single-mode root role).
    // Roving tabindex tracks the active item in both modes.,
    "toggle-group-item": {
      onClick: function (item) {
        var group = item.closest("[data-slot=toggle-group]")
        if (!group) return
        var wasOn = item.getAttribute("data-state") === "on"
        var items = group.querySelectorAll("[data-slot=toggle-group-item]")
        // mode from the ITEM's own role (root is role=group in both modes
        // under radix current semantics): single items are role=radio with
        // aria-checked; multiple items are plain buttons with aria-pressed
        if (item.getAttribute("role") === "radio") {
          items.forEach(function (it) { h.setGroupItem(it, it === item && !wasOn) })
        } else {
          item.setAttribute("aria-pressed", String(!wasOn))
          item.setAttribute("data-state", !wasOn ? "on" : "off")
          items.forEach(function (it) { if (it !== item) it.setAttribute("tabindex", "-1") })
        }
        item.setAttribute("tabindex", "0")
        item.focus()
        // single: the on item's value or null; multiple: every on item's value
        var on = Array.prototype.filter.call(items, function (it) { return it.getAttribute("data-state") === "on" }).map(h.itemValue)
        h.emit(group, "change", "toggle-group", { value: item.getAttribute("role") === "radio" ? (on[0] || null) : on, item: item })
      },
    },
    // arrows/home/end move focus only (radix roving focus); selection happens
    // via Space/Enter (= click). From non-item focus, arrows land on the LAST
    // item (measured radix quirk), Home on the first.,
    "toggle-group": {
      onKeydown: function (group, ctx, ev) {
        if (h.NAV_KEYS.indexOf(ev.key) < 0) return
        ev.preventDefault()
        var items = [...group.querySelectorAll("[data-slot=toggle-group-item]")]
        if (!items.length) return
        var next = items[h.nextIndex(ev, items)]
        if (!next) return
        items.forEach(function (it) { it.setAttribute("tabindex", it === next ? "0" : "-1") })
        next.focus()
      },
    },
  } })
})()
```
{% </codegroup> %}

{% </demo> %}

Use negative margins with `-mx-(--card-spacing)` to make content go edge to edge while keeping it aligned with the card inset. When the edge-to-edge content sits above a footer, use `-mb-(--card-spacing)` on `CardContent` to remove the section gap.

{% <demo name="card-edge-to-edge" status="authored"> %}
<iframe class="demo" src="/demos/card-edge-to-edge.html" title="card-edge-to-edge" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-edge-to-edge.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=card-edge-to-edge.html
<div
  data-slot="card"
  data-size="default"
  class="*:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card mx-auto w-full max-w-sm"
>
  <div data-slot="card-header" class="group/card-header">
    <div data-slot="card-title" class="cn-font-heading">Terms of Service</div>
    <div data-slot="card-description">Review the terms before accepting the agreement.</div>
  </div>
  <div data-slot="card-content" class="-mb-(--card-spacing)">
    <div
      class="-mx-(--card-spacing) max-h-48 space-y-4 overflow-y-scroll border-t bg-muted/50 px-(--card-spacing) py-4 text-sm leading-relaxed"
    >
      <p>
        These terms govern your use of the workspace, including access to shared documents, project
        files, and collaboration tools.
      </p>
      <p>
        You are responsible for the content you upload and for ensuring that your team has the
        appropriate permissions to view or edit it.
      </p>
      <p>
        We may update features or limits as the service evolves. When those changes materially
        affect your workflow, we will notify your workspace administrators.
      </p>
      <p>
        By continuing, you agree to keep your account credentials secure and to follow your
        organization's acceptable use policies.
      </p>
    </div>
  </div>
  <div data-slot="card-footer" class="justify-end gap-2">
    <button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg group/button border-border dark:border-input"
    >
      Decline</button
    ><button
      data-slot="button"
      data-variant="default"
      data-size="default"
      class="rounded-lg group/button"
    >
      Accept
    </button>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## Image

Add an image before the card header to create a card with an image.

{% <demo name="card-image" status="authored"> %}
<iframe class="demo" src="/demos/card-image.html" title="card-image" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-image.html">Open the demo page</a></p>

{% <codegroup> %}
```text,name=card-image.html
<div
  data-slot="card"
  data-size="default"
  class="py-(--card-spacing) has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card relative mx-auto w-full max-w-sm pt-0"
>
  <div class="absolute inset-0 z-30 aspect-video bg-black/35"></div>
  <img
    alt="Event cover"
    class="relative z-20 aspect-video w-full object-cover brightness-60 grayscale dark:brightness-40"
    src="https://avatar.vercel.sh/shadcn1"
  />
  <div data-slot="card-header" class="group/card-header">
    <div data-slot="card-action">
      <span data-slot="badge" data-variant="secondary" class="rounded-4xl group/badge"
        >Featured</span
      >
    </div>
    <div data-slot="card-title" class="cn-font-heading">Design systems meetup</div>
    <div data-slot="card-description">
      A practical talk on component APIs, accessibility, and shipping faster.
    </div>
  </div>
  <div data-slot="card-footer">
    <button
      data-slot="button"
      data-variant="default"
      data-size="default"
      class="rounded-lg group/button w-full"
    >
      View Event
    </button>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

{% <demo name="card-rtl" status="authored"> %}
<iframe class="demo" src="/demos/card-rtl.html" title="card-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-rtl.html">Open the demo page</a> · <a href="/demos/card-rtl-he.html">HE</a> · <a href="/demos/card-rtl-en.html">EN</a></p>

{% <codegroup> %}
```text,name=card-rtl.html
<div
  data-slot="card"
  data-size="default"
  class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col w-full max-w-sm"
  dir="rtl"
>
  <div
    data-slot="card-header"
    class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"
  >
    <div
      data-slot="card-title"
      class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading"
    >
      تسجيل الدخول إلى حسابك
    </div>
    <div data-slot="card-description" class="text-muted-foreground text-sm">
      أدخل بريدك الإلكتروني أدناه لتسجيل الدخول إلى حسابك
    </div>
    <div
      data-slot="card-action"
      class="col-start-2 row-span-2 row-start-1 self-start justify-self-end"
    >
      <button
        data-slot="button"
        data-variant="link"
        data-size="default"
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 text-primary underline-offset-4 hover:underline h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
      >
        إنشاء حساب
      </button>
    </div>
  </div>
  <div data-slot="card-content" class="px-(--card-spacing)">
    <form>
      <div class="flex flex-col gap-6">
        <div class="grid gap-2">
          <label
            data-slot="label"
            class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
            for="email-rtl"
            >البريد الإلكتروني</label
          ><input
            data-slot="input"
            class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
            id="email-rtl"
            placeholder="m@example.com"
            required=""
            type="email"
          />
        </div>
        <div class="grid gap-2">
          <div class="flex items-center">
            <label
              data-slot="label"
              class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed"
              for="password-rtl"
              >كلمة المرور</label
            ><a href="#" class="ms-auto inline-block text-sm underline-offset-4 hover:underline"
              >نسيت كلمة المرور؟</a
            >
          </div>
          <input
            data-slot="input"
            class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
            id="password-rtl"
            required=""
            type="password"
          />
        </div>
      </div>
    </form>
  </div>
  <div
    data-slot="card-footer"
    class="bg-muted/50 rounded-b-xl border-t p-(--card-spacing) flex items-center flex-col gap-2"
  >
    <button
      data-slot="button"
      data-variant="default"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-full"
      type="submit"
    >
      تسجيل الدخول</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-full"
    >
      تسجيل الدخول باستخدام Google
    </button>
  </div>
</div>
```
{% </codegroup> %}

{% </demo> %}

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="card"` |
| `data-slot="card-header"` |
| `data-slot="card-title"` |
| `data-slot="card-description"` |
| `data-slot="card-action"` |
| `data-slot="card-content"` |
| `data-slot="card-footer"` |

**Runtime:** no JavaScript — this is markup + CSS. No `cva`-declared variants — the stylesheet embedded below is the whole default look; its `data-*` selectors are the attributes each slot's styling depends on.

**The stylesheet** — `shadless/card.css`, the exact bytes of `dist/css/card.css`, verbatim. Every rule keys on the `data-slot` / `data-*` attributes in the tables above, so this is the authoritative default look of each slot:

```css
/* card */
@layer components {
  [data-slot="card"] { @apply ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl flex flex-col; }
  [data-slot="card-header"] { @apply gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]; }
  [data-slot="card-title"] { @apply text-base leading-snug font-medium group-data-[size=sm]/card:text-sm; }
  [data-slot="card-description"] { @apply text-muted-foreground text-sm; }
  [data-slot="card-action"] { @apply col-start-2 row-span-2 row-start-1 self-start justify-self-end; }
  [data-slot="card-content"] { @apply px-(--card-spacing); }
  [data-slot="card-footer"] { @apply bg-muted/50 rounded-b-xl border-t p-(--card-spacing) flex items-center; }
}
```
