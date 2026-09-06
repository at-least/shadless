// FT9 root-cause: shared constants for the theme pre-paint script and the
// shadless CSS fix layer (closes upstream-shadcn CSS gaps in dist/out.css).
// src/emitter/index.mjs (static tier) reads THEME_PREPAINT_SCRIPT directly.
// Every other production consumer — docs-build, demo, RTL/example-oracle —
// went through this module's injectPrePaint()/injectSiteSkin() until the
// VitePress/Go pipeline port; those callers (tools/demo.mjs,
// tools/docs-build.mjs) no longer exist, and the port's single source of
// truth for injection is now pipeline/prepaint.go, which mirrors
// THEME_PREPAINT_SCRIPT verbatim (SHADLESS_CSS_FIXES the same way, via
// pipeline/product_css.go). This file carries no injection logic any more.

// FT9 root-cause: shared pre-paint + live-toggle script. Two responsibilities:
//   1. FOUC avoidance: before first paint, read shadless-docs-theme from
//      localStorage and toggle .dark on <html>.
//   2. Live sync: if a different window/iframe (e.g. the docs host page)
//      changes localStorage, this document follows via the browser's
//      'storage' event. No parent reach-in needed — every demo flips on
//      its own when storage changes.
// Single inline <script>; idempotent (string-detect on the IIFE prefix).
export const THEME_PREPAINT_SCRIPT = `<script>(function(){try{var k="shadless-docs-theme";var apply=function(d){document.documentElement.classList.toggle("dark",!!d)};var s=localStorage.getItem(k);var d=s?s==="dark":matchMedia("(prefers-color-scheme: dark)").matches;apply(d);addEventListener("storage",function(e){if(e.key===k)apply(e.newValue==="dark")});}catch(e){}})();</script>`

// FT10 history: this layer once carried `color: var(--foreground)` rules
// for button outline/ghost — added 2026-08-25 and misdiagnosed as "upstream
// shadcn CSS gaps". The 2026-08-28 root cause (FINDING-css-variant-cascade.md)
// showed upstream has no such gap: React/cva composes one value per axis, so
// ghost/outline never receive the default variant's colors. The gap was ours
// — the css emitter merged default-variant utilities into the bare slot
// rule, and CSS has no un-apply. Fixed in src/emitter/css.mjs (twin
// :not([data-<axis>]) / [data-<axis>="<default>"] blocks modeling
// defaultVariants on the attribute API; regression-gated by
// gates/path-parity.mjs). Variant-less resting colors now inherit from
// the page — upstream's own model; the demo globals' body rule provides
// --foreground. The export stays (tools/demo.mjs and pipeline/product_css.go
// consume it) for gaps that are genuinely upstream's.
export const SHADLESS_CSS_FIXES = ``
