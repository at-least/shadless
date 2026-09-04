---
title: "Toggle"
description: "A two-state button that can be either on or off."
---

# Toggle

A two-state button that can be either on or off.

::::demo toggle-demo
<iframe class="demo" src="/demos/toggle-demo.html" title="toggle-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/toggle-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [toggle-demo.html]
<button
  type="button"
  aria-pressed="false"
  data-state="off"
  data-slot="toggle"
  class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 font-medium transition-all group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
  aria-label="Toggle bookmark"
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
    class="lucide lucide-bookmark group-data-[state=on]/toggle:fill-foreground"
  >
    <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"></path></svg
  >Bookmark
</button>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/toggle.js
// shadless toggle behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle", { slots: {
    toggle: {
      onClick: function (root) {
        var on = root.getAttribute("aria-pressed") !== "true"
        root.setAttribute("aria-pressed", String(on))
        root.setAttribute("data-state", on ? "on" : "off")
        h.emit(root, "change", "toggle", { pressed: on })
      },
    },
    // radio-group: click checks exclusively; checked item cannot be unchecked,
  } })
})()
```
:::

::::

## Installation

**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/toggle.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/toggle.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |
| `dist/js/toggle.js` | this component's behavior — registers with the base |

**Load the behavior files in your page:**

```html
<script src="shadless.js"></script>
<script src="js/toggle.js"></script>
```

**Copy the markup from any example on this page (the code tab under its preview) into your page and adapt it — the inline utilities are picked up by your build's content scan.**

**Behavior**

Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. `aria-pressed` + `data-state="on|off"` on the root. Keys: Space / click toggles.

The root dispatches `shadless:change` (`detail: { pressed }`), bubbling, after the state change, whichever path caused it.

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Outline

Use `variant="outline"` for an outline style.

::::demo toggle-outline
<iframe class="demo" src="/demos/toggle-outline.html" title="toggle-outline" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/toggle-outline.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [toggle-outline.html]
<div class="flex flex-wrap items-center gap-2">
  <button
    type="button"
    aria-pressed="false"
    data-state="off"
    data-slot="toggle"
    class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 rounded-lg text-sm font-medium transition-all [&amp;_svg:not([class*='size-'])]:size-4 group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-8 min-w-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    aria-label="Toggle italic"
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
      class="lucide lucide-italic"
    >
      <line x1="19" x2="10" y1="4" y2="4"></line>
      <line x1="14" x2="5" y1="20" y2="20"></line>
      <line x1="15" x2="9" y1="4" y2="20"></line></svg
    >Italic</button
  ><button
    type="button"
    aria-pressed="false"
    data-state="off"
    data-slot="toggle"
    class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 rounded-lg text-sm font-medium transition-all [&amp;_svg:not([class*='size-'])]:size-4 group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-8 min-w-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    aria-label="Toggle bold"
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
      class="lucide lucide-bold"
    >
      <path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8"></path></svg
    >Bold
  </button>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/toggle.js
// shadless toggle behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle", { slots: {
    toggle: {
      onClick: function (root) {
        var on = root.getAttribute("aria-pressed") !== "true"
        root.setAttribute("aria-pressed", String(on))
        root.setAttribute("data-state", on ? "on" : "off")
        h.emit(root, "change", "toggle", { pressed: on })
      },
    },
    // radio-group: click checks exclusively; checked item cannot be unchecked,
  } })
})()
```
:::

::::

## With Text

::::demo toggle-text
<iframe class="demo" src="/demos/toggle-text.html" title="toggle-text" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/toggle-text.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [toggle-text.html]
<button
  type="button"
  aria-pressed="false"
  data-state="off"
  data-slot="toggle"
  class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 rounded-lg text-sm font-medium transition-all [&amp;_svg:not([class*='size-'])]:size-4 group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none hover:bg-muted focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-transparent h-8 min-w-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
  aria-label="Toggle italic"
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
    class="lucide lucide-italic"
  >
    <line x1="19" x2="10" y1="4" y2="4"></line>
    <line x1="14" x2="5" y1="20" y2="20"></line>
    <line x1="15" x2="9" y1="4" y2="20"></line></svg
  >Italic
</button>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/toggle.js
// shadless toggle behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle", { slots: {
    toggle: {
      onClick: function (root) {
        var on = root.getAttribute("aria-pressed") !== "true"
        root.setAttribute("aria-pressed", String(on))
        root.setAttribute("data-state", on ? "on" : "off")
        h.emit(root, "change", "toggle", { pressed: on })
      },
    },
    // radio-group: click checks exclusively; checked item cannot be unchecked,
  } })
})()
```
:::

::::

## Size

Use the `size` prop to change the size of the toggle.

::::demo toggle-sizes
<iframe class="demo" src="/demos/toggle-sizes.html" title="toggle-sizes" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/toggle-sizes.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [toggle-sizes.html]
<div class="flex flex-wrap items-center gap-2">
  <button
    type="button"
    aria-pressed="false"
    data-state="off"
    data-slot="toggle"
    class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 font-medium transition-all group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
    aria-label="Toggle small"
  >
    Small</button
  ><button
    type="button"
    aria-pressed="false"
    data-state="off"
    data-slot="toggle"
    class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 rounded-lg text-sm font-medium transition-all [&amp;_svg:not([class*='size-'])]:size-4 group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-8 min-w-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    aria-label="Toggle default"
  >
    Default</button
  ><button
    type="button"
    aria-pressed="false"
    data-state="off"
    data-slot="toggle"
    class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 rounded-lg text-sm font-medium transition-all [&amp;_svg:not([class*='size-'])]:size-4 group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-9 min-w-9 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    aria-label="Toggle large"
  >
    Large
  </button>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/toggle.js
// shadless toggle behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle", { slots: {
    toggle: {
      onClick: function (root) {
        var on = root.getAttribute("aria-pressed") !== "true"
        root.setAttribute("aria-pressed", String(on))
        root.setAttribute("data-state", on ? "on" : "off")
        h.emit(root, "change", "toggle", { pressed: on })
      },
    },
    // radio-group: click checks exclusively; checked item cannot be unchecked,
  } })
})()
```
:::

::::

## Disabled

::::demo toggle-disabled
<iframe class="demo" src="/demos/toggle-disabled.html" title="toggle-disabled" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/toggle-disabled.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [toggle-disabled.html]
<div class="flex flex-wrap items-center gap-2">
  <button
    type="button"
    aria-pressed="false"
    data-state="off"
    data-disabled=""
    data-slot="toggle"
    class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 rounded-lg text-sm font-medium transition-all [&amp;_svg:not([class*='size-'])]:size-4 group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none hover:bg-muted focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-transparent h-8 min-w-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    aria-label="Toggle disabled"
    disabled=""
  >
    Disabled</button
  ><button
    type="button"
    aria-pressed="false"
    data-state="off"
    data-disabled=""
    data-slot="toggle"
    class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 rounded-lg text-sm font-medium transition-all [&amp;_svg:not([class*='size-'])]:size-4 group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-8 min-w-8 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
    aria-label="Toggle disabled outline"
    disabled=""
  >
    Disabled
  </button>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/toggle.js
// shadless toggle behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle", { slots: {
    toggle: {
      onClick: function (root) {
        var on = root.getAttribute("aria-pressed") !== "true"
        root.setAttribute("aria-pressed", String(on))
        root.setAttribute("data-state", on ? "on" : "off")
        h.emit(root, "change", "toggle", { pressed: on })
      },
    },
    // radio-group: click checks exclusively; checked item cannot be unchecked,
  } })
})()
```
:::

::::

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo toggle-rtl
<iframe class="demo" src="/demos/toggle-rtl.html" title="toggle-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/toggle-rtl.html">Open the demo page</a> · <a href="/demos/toggle-rtl-he.html">HE</a> · <a href="/demos/toggle-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [toggle-rtl.html]
<button
  type="button"
  aria-pressed="false"
  data-state="off"
  data-slot="toggle"
  class="hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 font-medium transition-all group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pe-1.5 has-data-[icon=inline-start]:ps-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5"
  aria-label="Toggle bookmark"
  dir="rtl"
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
    class="lucide lucide-bookmark group-aria-pressed/toggle:fill-foreground"
  >
    <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"></path></svg
  >إشارة مرجعية
</button>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/toggle.js
// shadless toggle behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle", { slots: {
    toggle: {
      onClick: function (root) {
        var on = root.getAttribute("aria-pressed") !== "true"
        root.setAttribute("aria-pressed", String(on))
        root.setAttribute("data-state", on ? "on" : "off")
        h.emit(root, "change", "toggle", { pressed: on })
      },
    },
    // radio-group: click checks exclusively; checked item cannot be unchecked,
  } })
})()
```
:::

::::

## API Reference

**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="toggle"` |

**Runtime:** `aria-pressed` + `data-state="on|off"` on the root. Keys: Space / click toggles. The root dispatches `shadless:change` (`detail: { pressed }`). No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.

Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — check `dist/css/toggle.css` for any other `data-*` selector on these slots.

| Slot | Attribute | Values | Default |
| --- | --- | --- | --- |
| `toggle` | `data-variant` | `default`, `outline` | `default` |
| `toggle` | `data-size` | `default`, `sm`, `lg` | `default` |

See the [Radix Toggle](https://www.radix-ui.com/primitives/docs/components/toggle#api-reference) documentation.
