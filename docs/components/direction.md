---
title: "Direction"
description: "A provider component that sets the text direction for your application."
---

# Direction

A provider component that sets the text direction for your application.

The `DirectionProvider` component is used to set the text direction (`ltr` or `rtl`) for your application. This is essential for supporting right-to-left languages like Arabic, Hebrew, and Persian.

Here's a preview of the component in RTL mode. Use the language selector to switch the language. To see more examples, look for the RTL section on components pages.

::::demo card-rtl
<iframe class="demo" src="/demos/card-rtl.html" title="card-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-rtl.html">Open the demo page</a> · <a href="/demos/card-rtl-he.html">HE</a> · <a href="/demos/card-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [card-rtl.html]
<div
  data-slot="card"
  data-size="default"
  class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col w-full max-w-sm"
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
        class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 text-primary underline-offset-4 hover:underline h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
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
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-full"
      type="submit"
    >
      تسجيل الدخول</button
    ><button
      data-slot="button"
      data-variant="outline"
      data-size="default"
      class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-full"
    >
      تسجيل الدخول باستخدام Google
    </button>
  </div>
</div>
```
:::

::::

## Installation

**Add shadless to your Tailwind v4 entry:**

```css
@import "shadless";
```

This component has no stylesheet of its own — its styling rides the core theme and utilities in `shadless`.

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/components/direction.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |

**Copy the markup from `dist/components/direction.html` into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.

## Reading the direction

There is no `useDirection` hook and no `dist/js/direction.js` — direction is the DOM's own `dir` attribute. Read it the way the runtime itself does (`isRtl` in `dist/shadless.js`): the nearest `[dir]` ancestor, falling back to `<html dir>`.

```js showLineNumbers
function directionOf(el) {
  var d = el.closest("[dir]")
  return (d ? d.getAttribute("dir") : document.documentElement.getAttribute("dir")) || "ltr"
}
```
