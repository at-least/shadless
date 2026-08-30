---
title: "Dark Mode"
description: "Dark mode with the .dark class and CSS variables — no framework."
---

# Dark Mode

Dark mode with the .dark class and CSS variables — no framework.

shadless ships dark mode as CSS: `dist/out.css` defines every theme token as
a CSS variable on `:root`, with a `.dark` override block. Dark mode is a
class toggle — no provider, no framework.

::::demo mode-toggle
<iframe class="demo" src="/demos/mode-toggle.html" title="mode-toggle" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/mode-toggle.html">Open the demo page</a></p>

::: code-group
```text [mode-toggle.html]
<span data-slot="button-group">
    <button data-slot="button" data-variant="outline" data-size="icon" id="mt-sun" aria-label="Use light theme">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/></svg>
    </button>
    <button data-slot="button" data-variant="outline" data-size="icon" id="mt-moon" aria-label="Use dark theme">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/></svg>
    </button>
  </span>
```

```js [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)
```
:::

::::


## How it works

```css
/* inside dist/out.css */
:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  /* … */
}

.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  /* … */
}
```

Every component consumes the variables through utility classes
(`bg-background`, `text-foreground`, …), so flipping the class re-themes the
whole page.

## Toggle without a framework

Apply `.dark` to `<html>` and persist the choice:

```html
<script>
  const stored = localStorage.getItem("theme")
  const dark = stored ? stored === "dark"
    : matchMedia("(prefers-color-scheme: dark)").matches
  document.documentElement.classList.toggle("dark", dark)
</script>
```

```js
// a toggle button — shadless components work as the trigger
document.querySelector("#theme-toggle").addEventListener("click", () => {
  const dark = document.documentElement.classList.toggle("dark")
  localStorage.setItem("theme", dark ? "dark" : "light")
})
```

Put the inline script in `<head>` (before paint) to avoid a flash of the
wrong theme.

## System preference

Follow the OS setting instead of persisting a choice:

```js
matchMedia("(prefers-color-scheme: dark)").addEventListener("change", (e) => {
  document.documentElement.classList.toggle("dark", e.matches)
})
```
