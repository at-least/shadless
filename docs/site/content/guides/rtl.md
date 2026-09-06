---
title: "RTL"
description: "Right-to-left support for shadcn/ui components."
weight: 4
---

# RTL

Right-to-left support for shadcn/ui components.

shadcn/ui components have first-class support for right-to-left (RTL) layouts. Text alignment, positioning, and directional styles automatically adapt for languages like Arabic, Hebrew, and Persian.

{% <demo name="card-rtl" status="authored"> %}
<iframe class="demo" src="/demos/card-rtl.html" title="card-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-rtl.html">Open the demo page</a> · <a href="/demos/card-rtl-he.html">HE</a> · <a href="/demos/card-rtl-en.html">EN</a></p>

{% <codegroup> %}
```text,name=card-rtl.html
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
{% </codegroup> %}

{% </demo> %}

shadless has no install step that rewrites anything: the classes you see are the pinned registry's own. What makes the difference is which example you start from — the pages below ship an RTL variant beside the LTR one wherever upstream authored it.

## How RTL works here

There is no CLI to opt into and no `components.json`. shadless emits a mechanical conversion of the pinned shadcn registry, so where upstream uses a physical utility (`pl-*`, `right-*`, `rounded-l-*`) the shipped stylesheet carries it too — converting them would be a divergence from the oracle the whole port is checked against. The `css-direction` gate holds that inventory to a committed baseline, so a re-pin that moves the RTL story is a visible decision instead of a silent regression.

What you get instead: upstream authors a separate RTL example for the components that need one, and shadless ships those as their own pages — `<name>-rtl` beside `<name>-demo`, plus the `-he` / `-fa` / `-en` language variants `build-rtl` emits. Compare `button-group-demo` (`rounded-l-none`) with `button-group-rtl` (`rounded-s-none`) to see the difference an RTL page actually makes.

So: set `dir="rtl"` on the page, start from this page's RTL examples rather than the LTR ones, and check the components you use — the ones whose stylesheet is already logical need nothing more. To flip an individual icon, give it the `rtl:rotate-180` utility class.

## Font Recommendations

Use a font with proper support for your target language. [Noto](https://fonts.google.com/noto) is a good family for this and pairs well with Inter and Geist. shadless ships no fonts and no font configuration — load them the way your own build already loads fonts.

## Animations

Animation utilities are emitted as the registry writes them, physical names included — nothing rewrites `slide-in-from-right` into `slide-in-from-end` for you. Where upstream's own RTL example uses the logical name, the shipped `-rtl` page carries it; elsewhere, set the direction on the element yourself.

**A note on tw-animate-css:**

There is a [known issue](https://github.com/Wombosvideo/tw-animate-css/issues/67) with the `tw-animate-css` library where the logical slide utilities are not working as expected. For now, set the `dir` attribute on the portal content element itself.

```html showLineNumbers /dir="rtl"/
<button data-slot="popover-trigger" id="<k>-trigger">Open</button>
<template id="<k>-portal">
  <div data-slot="popover-content" dir="rtl">
    <div>Content</div>
  </div>
</template>
```
```html showLineNumbers /dir="rtl"/
<button data-slot="tooltip-trigger" id="<k>-trigger">Open</button>
<template id="<k>-portal">
  <div data-slot="tooltip-content" dir="rtl">
    <div>Content</div>
  </div>
</template>
```
shadless components ship the pinned registry's classes as-is, and that cuts both ways: many slots are already logical (start/end-aware) and need nothing but `dir="rtl"` on the page, while others still carry the physical utilities upstream wrote (`pl-*`, `right-*`, `rounded-l-*`) — `css-direction` keeps a committed inventory of exactly which. There is no migration command to run; check the components you actually use, and prefer this page's `-rtl` examples over the LTR ones where upstream authored a pair. To flip an individual icon, give it the `rtl:rotate-180` utility class.
