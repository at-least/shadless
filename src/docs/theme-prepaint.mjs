// FT9 root-cause: shared utilities for the theme pre-paint script (HTML
// injection) and the shadless CSS fix layer (closes upstream-shadcn CSS
// gaps in dist/out.css). Used by the three emitters:
//   - src/emitter/index.mjs (static tier)
//   - tools/demo.mjs (kernel + trivial-js + carousel + field; writes final
//     dist/globals.css that Tailwind compiles into dist/out.css)
//   - tools/docs-build.mjs (post-process for hand-authored docs/demos/*
//     and as a defense-in-depth safety net)
//
// Idempotent everywhere: HTML injection detects the script by string;
// CSS fix layer is layer-scoped so re-running can't double up.

// FT9 root-cause: shared pre-paint + live-toggle script. Two responsibilities:
//   1. FOUC avoidance: before first paint, read shadless-docs-theme from
//      localStorage and toggle .dark on <html>.
//   2. Live sync: if a different window/iframe (e.g. the docs host page)
//      changes localStorage, this document follows via the browser's
//      'storage' event. No parent reach-in needed — every demo flips on
//      its own when storage changes.
// Single inline <script>; idempotent (string-detect on the IIFE prefix).
export const THEME_PREPAINT_SCRIPT = `<script>(function(){try{var k="shadless-docs-theme";var apply=function(d){document.documentElement.classList.toggle("dark",!!d)};var s=localStorage.getItem(k);var d=s?s==="dark":matchMedia("(prefers-color-scheme: dark)").matches;apply(d);addEventListener("storage",function(e){if(e.key===k)apply(e.newValue==="dark")});}catch(e){}})();</script>`

export function injectPrePaint(html) {
  // Idempotency: check for the actual script-tag shape, not just the
  // localStorage key (some demos already use the key in their own JS).
  // FT11: SIG updated to match THEME_PREPAINT_SCRIPT's IIFE prefix
  // (var k;var apply). The old SIG (var s) silently broke idempotency
  // after FT10, causing every docs/site/components/*.html to ship with
  // two pre-paint scripts in <head>.
  const SIG = '<script>(function(){try{var k="shadless-docs-theme"'
  if (html.includes(SIG)) return html
  if (html.includes('</head>')) return html.replace('</head>', THEME_PREPAINT_SCRIPT + '</head>')
  if (/<head[^>]*>/i.test(html)) return html.replace(/(<head[^>]*>)/i, `$1${THEME_PREPAINT_SCRIPT}`)
  return THEME_PREPAINT_SCRIPT + html
}

// Docs-site font skin, composed on top of injectPrePaint. ui.shadcn.com
// renders every component in Geist (next/font, see apps/v4/lib/fonts.tsx);
// without it the demos fall back to the OS system font and every text
// metric drifts from upstream (wider buttons, extra description lines —
// the "alert-dialog buttons look different" report). The product tree
// (dist/) stays font-agnostic: a consumer's own --font-sans wins, and
// the example/golden gates byte-match dist demos, so the link is added
// ONLY to the docs/site/components/* copies. docs-build and both
// consistency gate (docs-consistency.mjs)
// derive their image from THIS function so the three can't drift —
// the FT11 lesson, applied.
export const SITE_FONTS_LINK = '<link rel="stylesheet" href="../fonts.css">'
export function injectSiteSkin(html) {
  const out = injectPrePaint(html)
  if (out.includes(SITE_FONTS_LINK)) return out
  if (out.includes('</head>')) return out.replace('</head>', SITE_FONTS_LINK + '</head>')
  if (/<head[^>]*>/i.test(out)) return out.replace(/(<head[^>]*>)/i, `$1${SITE_FONTS_LINK}`)
  return SITE_FONTS_LINK + out
}

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
