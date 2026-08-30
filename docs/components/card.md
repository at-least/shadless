---
title: "Card"
description: "Displays a card with header, content, and footer."
---

# Card

Displays a card with header, content, and footer.

::::demo card-demo
<iframe class="demo" src="/demos/card-demo.html" title="card-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-demo.html">Open the demo page</a></p>

::: code-group
```text [card-demo.html]
<div data-slot="card" data-size="default" class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col w-full max-w-sm"><div data-slot="card-header" class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"><div data-slot="card-title" class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading">Login to your account</div><div data-slot="card-description" class="text-muted-foreground text-sm">Enter your email below to login to your account</div><div data-slot="card-action" class="col-start-2 row-span-2 row-start-1 self-start justify-self-end"><button data-slot="button" data-variant="link" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 text-primary underline-offset-4 hover:underline h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2">Sign Up</button></div></div><div data-slot="card-content" class="px-(--card-spacing)"><form><div class="flex flex-col gap-6"><div class="grid gap-2"><label data-slot="label" class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed" for="email">Email</label><input data-slot="input" class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50" id="email" placeholder="m@example.com" required="" type="email"></div><div class="grid gap-2"><div class="flex items-center"><label data-slot="label" class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed" for="password">Password</label><a href="#" class="ml-auto inline-block text-sm underline-offset-4 hover:underline">Forgot your password?</a></div><input data-slot="input" class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50" id="password" required="" type="password"></div></div></form></div><div data-slot="card-footer" class="bg-muted/50 rounded-b-xl border-t p-(--card-spacing) flex items-center flex-col gap-2"><button data-slot="button" data-variant="default" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-full" type="submit">Login</button><button data-slot="button" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-full">Login with Google</button></div></div>
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/card.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/card.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/card.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                             into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/card.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
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

::::demo card-small
<iframe class="demo" src="/demos/card-small.html" title="card-small" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-small.html">Open the demo page</a></p>

::: code-group
```text [card-small.html]
<div data-slot="card" data-size="sm" class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col mx-auto w-full max-w-sm"><div data-slot="card-header" class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"><div data-slot="card-title" class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading">Small Card</div><div data-slot="card-description" class="text-muted-foreground text-sm">This card uses the small size variant.</div></div><div data-slot="card-content" class="px-(--card-spacing)"><p>The card component supports a size prop that can be set to "sm" for a more compact appearance.</p></div><div data-slot="card-footer" class="bg-muted/50 rounded-b-xl border-t p-(--card-spacing) flex items-center"><button data-slot="button" data-variant="outline" data-size="sm" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 border bg-clip-padding font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5 w-full">Action</button></div></div>
```
:::

::::


## Spacing

In addition to the `size` prop, you can use the `--card-spacing` CSS variable to control the spacing between sections and the inset of card parts.

::::demo card-spacing
<iframe class="demo" src="/demos/card-spacing.html" title="card-spacing" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-spacing.html">Open the demo page</a></p>

::: code-group
```text [card-spacing.html]
<div class="mx-auto grid w-full max-w-sm gap-4"><div role="group" dir="ltr" data-slot="toggle-group" data-variant="outline" data-size="sm" data-spacing="2" data-orientation="horizontal" class="rounded-lg data-[size=sm]:rounded-[min(var(--radius-md),10px)] group/toggle-group flex w-fit flex-row items-center gap-[--spacing(var(--gap))] data-vertical:flex-col data-vertical:items-stretch justify-center" tabindex="0" style="outline: none; --gap: 2;"><button type="button" data-state="on" role="radio" aria-checked="true" data-slot="toggle-group-item" data-variant="outline" data-size="sm" data-spacing="2" class="group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-lg group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-lg group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-lg group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-lg shrink-0 focus:z-10 focus-visible:z-10 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 font-medium transition-all group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5" tabindex="-1" data-radix-collection-item="">16px</button><button type="button" data-state="off" role="radio" aria-checked="false" data-slot="toggle-group-item" data-variant="outline" data-size="sm" data-spacing="2" class="group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-lg group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-lg group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-lg group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-lg shrink-0 focus:z-10 focus-visible:z-10 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 font-medium transition-all group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5" tabindex="-1" data-radix-collection-item="">20px</button><button type="button" data-state="off" role="radio" aria-checked="false" data-slot="toggle-group-item" data-variant="outline" data-size="sm" data-spacing="2" class="group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-lg group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-lg group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-lg group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-lg shrink-0 focus:z-10 focus-visible:z-10 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 font-medium transition-all group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5" tabindex="-1" data-radix-collection-item="">24px</button><button type="button" data-state="off" role="radio" aria-checked="false" data-slot="toggle-group-item" data-variant="outline" data-size="sm" data-spacing="2" class="group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:px-2 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-lg group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-lg group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-lg group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-lg shrink-0 focus:z-10 focus-visible:z-10 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:border-l-0 group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:border-t-0 group-data-horizontal/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-l group-data-vertical/toggle-group:data-[spacing=0]:data-[variant=outline]:first:border-t hover:text-foreground aria-pressed:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive data-[state=on]:bg-muted gap-1 font-medium transition-all group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-input hover:bg-muted border bg-transparent h-7 min-w-7 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;_svg:not([class*='size-'])]:size-3.5" tabindex="-1" data-radix-collection-item="">32px</button></div><div data-slot="card" data-size="default" class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col [--card-spacing:--spacing(4)]"><div data-slot="card-header" class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"><div data-slot="card-title" class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading">Login to your account</div><div data-slot="card-description" class="text-muted-foreground text-sm">Enter your email below to login to your account</div><div data-slot="card-action" class="col-start-2 row-span-2 row-start-1 self-start justify-self-end"><button data-slot="button" data-variant="link" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 text-primary underline-offset-4 hover:underline h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2">Sign Up</button></div></div><div data-slot="card-content" class="px-(--card-spacing)"><form><div class="flex flex-col gap-6"><div class="grid gap-2"><label data-slot="label" class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed" for="email-spacing">Email</label><input data-slot="input" class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50" id="email-spacing" placeholder="m@example.com" required="" type="email"></div><div class="grid gap-2"><div class="flex items-center"><label data-slot="label" class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed" for="password-spacing">Password</label><a href="#" class="ml-auto inline-block text-sm underline-offset-4 hover:underline">Forgot your password?</a></div><input data-slot="input" class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50" id="password-spacing" required="" type="password"></div></div></form></div><div data-slot="card-footer" class="bg-muted/50 rounded-b-xl border-t p-(--card-spacing) flex items-center flex-col gap-2"><button data-slot="button" data-variant="default" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-full" type="submit">Login</button><button data-slot="button" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-full">Login with Google</button></div></div></div>
```
:::

::::


Use negative margins with `-mx-(--card-spacing)` to make content go edge to edge while keeping it aligned with the card inset. When the edge-to-edge content sits above a footer, use `-mb-(--card-spacing)` on `CardContent` to remove the section gap.

::::demo card-edge-to-edge
<iframe class="demo" src="/demos/card-edge-to-edge.html" title="card-edge-to-edge" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-edge-to-edge.html">Open the demo page</a></p>

::: code-group
```text [card-edge-to-edge.html]
<div data-slot="card" data-size="default" class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col mx-auto w-full max-w-sm"><div data-slot="card-header" class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"><div data-slot="card-title" class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading">Terms of Service</div><div data-slot="card-description" class="text-muted-foreground text-sm">Review the terms before accepting the agreement.</div></div><div data-slot="card-content" class="px-(--card-spacing) -mb-(--card-spacing)"><div class="-mx-(--card-spacing) max-h-48 space-y-4 overflow-y-scroll border-t bg-muted/50 px-(--card-spacing) py-4 text-sm leading-relaxed"><p>These terms govern your use of the workspace, including access to shared documents, project files, and collaboration tools.</p><p>You are responsible for the content you upload and for ensuring that your team has the appropriate permissions to view or edit it.</p><p>We may update features or limits as the service evolves. When those changes materially affect your workflow, we will notify your workspace administrators.</p><p>By continuing, you agree to keep your account credentials secure and to follow your organization's acceptable use policies.</p></div></div><div data-slot="card-footer" class="bg-muted/50 rounded-b-xl border-t p-(--card-spacing) flex items-center justify-end gap-2"><button data-slot="button" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2">Decline</button><button data-slot="button" data-variant="default" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2">Accept</button></div></div>
```
:::

::::


## Image

Add an image before the card header to create a card with an image.

::::demo card-image
<iframe class="demo" src="/demos/card-image.html" title="card-image" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-image.html">Open the demo page</a></p>

::: code-group
```text [card-image.html]
<div data-slot="card" data-size="default" class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col relative mx-auto w-full max-w-sm pt-0"><div class="absolute inset-0 z-30 aspect-video bg-black/35"></div><img alt="Event cover" class="relative z-20 aspect-video w-full object-cover brightness-60 grayscale dark:brightness-40" src="https://avatar.vercel.sh/shadcn1"><div data-slot="card-header" class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"><div data-slot="card-action" class="col-start-2 row-span-2 row-start-1 self-start justify-self-end"><span data-slot="badge" data-variant="secondary" class="h-5 gap-1 rounded-4xl border border-transparent px-2 py-0.5 text-xs font-medium transition-all has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&amp;&gt;svg]:size-3! group/badge inline-flex w-fit shrink-0 items-center justify-center overflow-hidden whitespace-nowrap focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&amp;&gt;svg]:pointer-events-none bg-secondary text-secondary-foreground [a]:hover:bg-secondary/80">Featured</span></div><div data-slot="card-title" class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading">Design systems meetup</div><div data-slot="card-description" class="text-muted-foreground text-sm">A practical talk on component APIs, accessibility, and shipping faster.</div></div><div data-slot="card-footer" class="bg-muted/50 rounded-b-xl border-t p-(--card-spacing) flex items-center"><button data-slot="button" data-variant="default" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 w-full">View Event</button></div></div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo card-rtl
<iframe class="demo" src="/demos/card-rtl.html" title="card-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/card-rtl.html">Open the demo page</a> · <a href="/demos/card-rtl-he.html">HE</a> · <a href="/demos/card-rtl-en.html">EN</a></p>

::: code-group
```text [card-rtl.html]
<div data-slot="card" data-size="default" class="ring-foreground/10 bg-card text-card-foreground gap-(--card-spacing) overflow-hidden rounded-xl py-(--card-spacing) text-sm ring-1 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[&gt;img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col w-full max-w-sm" dir="rtl"><div data-slot="card-header" class="gap-1 rounded-t-xl px-(--card-spacing) [.border-b]:pb-(--card-spacing) group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]"><div data-slot="card-title" class="text-base leading-snug font-medium group-data-[size=sm]/card:text-sm cn-font-heading">تسجيل الدخول إلى حسابك</div><div data-slot="card-description" class="text-muted-foreground text-sm">أدخل بريدك الإلكتروني أدناه لتسجيل الدخول إلى حسابك</div><div data-slot="card-action" class="col-start-2 row-span-2 row-start-1 self-start justify-self-end"><button data-slot="button" data-variant="link" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 text-primary underline-offset-4 hover:underline h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2">إنشاء حساب</button></div></div><div data-slot="card-content" class="px-(--card-spacing)"><form><div class="flex flex-col gap-6"><div class="grid gap-2"><label data-slot="label" class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed" for="email-rtl">البريد الإلكتروني</label><input data-slot="input" class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50" id="email-rtl" placeholder="m@example.com" required="" type="email"></div><div class="grid gap-2"><div class="flex items-center"><label data-slot="label" class="gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed" for="password-rtl">كلمة المرور</label><a href="#" class="ms-auto inline-block text-sm underline-offset-4 hover:underline">نسيت كلمة المرور؟</a></div><input data-slot="input" class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50" id="password-rtl" required="" type="password"></div></div></form></div><div data-slot="card-footer" class="bg-muted/50 rounded-b-xl border-t p-(--card-spacing) flex items-center flex-col gap-2"><button data-slot="button" data-variant="default" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-full" type="submit">تسجيل الدخول</button><button data-slot="button" data-variant="outline" data-size="default" class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2 w-full">تسجيل الدخول باستخدام Google</button></div></div>
```
:::

::::


## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.

| Slot |
| --- |
| `data-slot="card"` |
| `data-slot="card-header"` |
| `data-slot="card-title"` |
| `data-slot="card-description"` |
| `data-slot="card-action"` |
| `data-slot="card-content"` |
| `data-slot="card-footer"` |

### Card

The `Card` component is the root container for card content.

| Prop        | Type                | Default     |
| ----------- | ------------------- | ----------- |
| `size`      | `"default" \| "sm"` | `"default"` |
| `className` | `string`            | -           |

### CardHeader

The `CardHeader` component is used for a title, description, and optional action.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardTitle

The `CardTitle` component is used for the card title.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardDescription

The `CardDescription` component is used for helper text under the title.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardAction

The `CardAction` component places content in the top-right of the header (for example, a button or a badge).

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardContent

The `CardContent` component is used for the main card body.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

### CardFooter

The `CardFooter` component is used for actions and secondary content at the bottom of the card.

| Prop        | Type     | Default |
| ----------- | -------- | ------- |
| `className` | `string` | -       |

## Changelog

### Spacing Variable

If you're upgrading from a previous version of the `Card` component, you'll need to apply the following updates to use the `--card-spacing` variable:



**Update the Card root spacing classes.**

Replace the hard-coded gap and vertical padding with `--card-spacing`, and set the default and small size values on the root:

```diff
  className={cn(
-   "group/card flex flex-col gap-4 overflow-hidden rounded-xl bg-card py-4 text-sm text-card-foreground ring-1 ring-foreground/10 has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:gap-3 data-[size=sm]:py-3 data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl",
+   "group/card flex flex-col gap-(--card-spacing) overflow-hidden rounded-xl bg-card py-(--card-spacing) text-sm text-card-foreground ring-1 ring-foreground/10 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl",
    className
  )}
```

**Update CardHeader spacing classes.**

Replace the horizontal padding and border spacing with the shared variable:

```diff
  className={cn(
-   "group/card-header @container/card-header grid auto-rows-min items-start gap-1 rounded-t-xl px-4 group-data-[size=sm]/card:px-3 has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto] [.border-b]:pb-4 group-data-[size=sm]/card:[.border-b]:pb-3",
+   "group/card-header @container/card-header grid auto-rows-min items-start gap-1 rounded-t-xl px-(--card-spacing) has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto] [.border-b]:pb-(--card-spacing)",
    className
  )}
```

**Update CardContent and CardFooter spacing classes.**

Use `--card-spacing` for the content inset and footer padding:

```diff
  function CardContent({ className, ...props }: React.ComponentProps<"div">) {
    return (
      <div
        data-slot="card-content"
-       className={cn("px-4 group-data-[size=sm]/card:px-3", className)}
+       className={cn("px-(--card-spacing)", className)}
        {...props}
      />
    )
  }
```

```diff
  className={cn(
-   "flex items-center rounded-b-xl border-t bg-muted/50 p-4 group-data-[size=sm]/card:p-3",
+   "flex items-center rounded-b-xl border-t bg-muted/50 p-(--card-spacing)",
    className
  )}
```



After applying these changes, you can customize card spacing by setting `--card-spacing` on the `Card` with an arbitrary property class:

```tsx
function Example() {
  return <Card className="[--card-spacing:--spacing(6)]">...</Card>
}
```
