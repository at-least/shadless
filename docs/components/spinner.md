---
title: "Spinner"
description: "An indicator that can be used to show a loading state."
---

# Spinner

An indicator that can be used to show a loading state.

<iframe class="demo" src="/demos/spinner-demo.html" title="spinner-demo" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/spinner.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/spinner.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/spinner.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                                into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/spinner.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives.
## Customization

You can replace the default spinner icon with any other icon by editing the `Spinner` component.

<iframe class="demo" src="/demos/spinner-custom.html" title="spinner-custom" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

```tsx showLineNumbers title="components/ui/spinner.tsx"

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

## Size

Use the `size-*` utility class to change the size of the spinner.

<iframe class="demo" src="/demos/spinner-size.html" title="spinner-size" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Button

Add a spinner to a button to indicate a loading state. Place the `<Spinner />` before the label with `data-icon="inline-start"` for a start position, or after the label with `data-icon="inline-end"` for an end position.

<iframe class="demo" src="/demos/spinner-button.html" title="spinner-button" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Badge

Add a spinner to a badge to indicate a loading state. Place the `<Spinner />` before the label with `data-icon="inline-start"` for a start position, or after the label with `data-icon="inline-end"` for an end position.

<iframe class="demo" src="/demos/spinner-badge.html" title="spinner-badge" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Input Group

<iframe class="demo" src="/demos/spinner-input-group.html" title="spinner-input-group" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## Empty

<iframe class="demo" src="/demos/spinner-empty.html" title="spinner-empty" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page</p>

## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

<iframe class="demo" src="/demos/spinner-rtl.html" title="spinner-rtl" data-status="authored" loading="lazy"></iframe>
<p class="demo-langs">Open the demo page · HE · EN</p>
