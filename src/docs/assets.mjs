// FT2 docs: static site-local assets written under docs/site/ by the build.
// No React at runtime; site.js only drives CodeTabs tab switching (recorded
// as allowed in PLAN Wave F scope decisions).
export const SITE_CSS = `
/* Site chrome tokens mirror the product's shadcn neutral theme
   (dist/out.css :root/.dark --background/--card/--muted/--border …)
   so the docs shell and the iframe demos paint the same background —
   a separate palette made the whole site (and the preview-frame seam)
   visibly off-tone against shadcn/ui. */
:root { color-scheme: light; --fg: oklch(0.145 0 0); --muted: oklch(0.556 0 0); --muted-strong: oklch(0.371 0 0); --border: oklch(0.922 0 0); --bg: oklch(1 0 0); --bg-subtle: oklch(0.97 0 0); --link: oklch(0.371 0 0); }
:root.dark { color-scheme: dark; --fg: oklch(0.985 0 0); --muted: oklch(0.708 0 0); --muted-strong: oklch(0.871 0 0); --border: oklch(1 0 0 / 10%); --bg: oklch(0.145 0 0); --bg-subtle: oklch(0.205 0 0); --link: oklch(0.871 0 0); }
* { box-sizing: border-box; }
/* Geist-first chrome to match ui.shadcn.com (fonts.css supplies the face;
   the system stack stays as the no-webfont fallback). */
body { margin: 0; font: 15px/1.65 "Geist", ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif; color: var(--fg); background: var(--bg); }
a { color: var(--link); text-decoration: none; }
a:hover { text-decoration: underline; }
.layout { display: grid; grid-template-columns: 240px minmax(0, 1fr); min-height: 100vh; }
.sidebar { border-right: 1px solid var(--border); padding: 24px 16px; position: sticky; top: 0; height: 100vh; overflow-y: auto; }
.sidebar .brand-row { display: flex; align-items: center; justify-content: space-between; gap: 8px; margin-bottom: 12px; }
.sidebar .brand, .mobile-topbar .brand { font-weight: 700; color: var(--fg); }
/* Mobile sidebar drawer: below lg (1024px) the fixed sidebar column is
   replaced by a hamburger drawer — mirrors ui.shadcn.com's
   collapsed-docs-sidebar, and frees the full viewport width for the
   article column (preview iframes included). Closed state is
   visibility:hidden so the drawer's links stay out of tab order and
   the a11y tree; visibility transitions discretely and stays "visible"
   for the duration of the slide, so the close animation still plays.
   .sidebar-toggle mirrors .theme-toggle's affordance. */
.mobile-topbar { display: none; }
.sidebar-backdrop { display: none; }
.sidebar-toggle { background: none; border: 1px solid var(--border); border-radius: 6px; padding: 4px; color: var(--muted); cursor: pointer; display: inline-flex; align-items: center; justify-content: center; width: 28px; height: 28px; }
.sidebar-toggle:hover { color: var(--fg); background: var(--bg-subtle); }
.sidebar-toggle svg { width: 16px; height: 16px; display: block; }
body.sidebar-open { overflow: hidden; }
@media (max-width: 1023.98px) {
  .layout { grid-template-columns: minmax(0, 1fr); }
  .mobile-topbar {
    display: flex;
    align-items: center;
    gap: 10px;
    position: sticky;
    top: 0;
    /* under the backdrop: while the drawer is open the topbar is scrim-
       dimmed like the page behind it (the drawer itself covers the
       hamburger corner regardless — closes are Escape / backdrop / link) */
    z-index: 40;
    padding: 10px 16px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
  }
  .sidebar {
    position: fixed;
    inset-block: 0;
    inset-inline-start: 0;
    width: min(18rem, 85vw);
    z-index: 60;
    background: var(--bg);
    /* dvh tracks mobile dynamic toolbars (iOS Safari); older engines
       ignore it and keep the base 100vh */
    height: 100dvh;
    transform: translateX(-100%);
    visibility: hidden;
    transition: transform 0.2s ease, visibility 0.2s;
    box-shadow: 0 0 40px oklch(0 0 0 / 12%);
  }
  .sidebar[data-open="true"] { transform: none; visibility: visible; }
  .sidebar-backdrop {
    display: block;
    position: fixed;
    inset: 0;
    z-index: 50;
    background: oklch(0 0 0 / 45%);
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.2s ease;
  }
  .sidebar-backdrop[data-open="true"] { opacity: 1; pointer-events: auto; }
}
.sidebar ul { list-style: none; margin: 0; padding: 0; }
.sidebar li a { display: block; padding: 3px 10px; border-radius: 6px; color: var(--muted); }
.sidebar li a:hover { background: var(--bg-subtle); color: var(--fg); text-decoration: none; }
.sidebar li a.active { background: var(--bg-subtle); color: var(--fg); font-weight: 600; }
.sidebar li.sidebar-group { padding: 14px 10px 4px; font-size: 12px; font-weight: 600; letter-spacing: 0.06em; text-transform: uppercase; color: var(--muted); }
.main { padding: 40px 48px 96px; max-width: 1100px; }
/* Narrower desktop windows: halve the gutters so the article column
   (and with it the preview iframes) stays past the demos' 40rem (sm:)
   breakpoint without horizontal scrolling for as long as possible. */
@media (max-width: 1279.98px) { .main { padding-inline: 24px; } }
.breadcrumbs { font-size: 13px; color: var(--muted); margin-bottom: 8px; }
.breadcrumbs a { color: var(--muted); }
.breadcrumbs a:hover { color: var(--fg); }
.breadcrumbs .sep { margin: 0 6px; }
.theme-toggle { background: none; border: 1px solid var(--border); border-radius: 6px; padding: 4px; color: var(--muted); cursor: pointer; display: inline-flex; align-items: center; justify-content: center; position: relative; width: 28px; height: 28px; }
.theme-toggle:hover { color: var(--fg); background: var(--bg-subtle); }
.theme-toggle svg { width: 16px; height: 16px; display: block; }
/* FT8: swap icon based on current theme. Sun visible by default (light
   mode), moon visible in dark mode. */
.theme-toggle .icon-moon { display: none; }
:root.dark .theme-toggle .icon-sun { display: none; }
:root.dark .theme-toggle .icon-moon { display: block; }
.unavailable-link { color: var(--muted); text-decoration: underline dotted; cursor: help; }
.page-header .lead { color: var(--muted); font-size: 17px; max-width: 60ch; }
.page-header .links { display: flex; gap: 12px; margin-top: 8px; font-size: 13px; }
.columns { display: grid; grid-template-columns: minmax(0, 1fr) 200px; gap: 48px; }
/* Upstream parity: the TOC rail shows only at xl (≥1280px).
   Hiding it earlier also reclaims the rail's 248px for the article
   column, keeping preview iframes past the demos' own 40rem (sm:)
   breakpoints on narrower desktop windows — without this, a ~1024px
   window stacks the alert-dialog footer buttons (mobile layout). */
@media (max-width: 1279.98px) {
  .columns { grid-template-columns: minmax(0, 1fr); }
  .toc { display: none; }
}
/* Prose is capped at 76ch, but component previews opt out and span the
   full column — mirrors upstream typeset.css, where [data-not-typeset]
   (the preview wrapper) escapes the prose measure. The wider iframe
   clears the demos' own 40rem (sm:) breakpoints at desktop widths, so
   they render their desktop layout (row-aligned footer buttons, etc.)
   exactly as they do on ui.shadcn.com; at the old 76ch cap the iframe
   landed at ~631px and every dialog demo showed its mobile layout. */
article > *:not(.component-preview):not(.component-preview-ph) { max-width: 76ch; }
article > :first-child { margin-top: 0; }
.toc { position: sticky; top: 40px; align-self: start; font-size: 13px; color: var(--muted); max-height: 85vh; overflow-y: auto; }
.toc ul { list-style: none; margin: 0; padding: 0; }
.toc li.toc-3 { padding-left: 14px; }
pre { background: var(--bg-subtle); border: 1px solid var(--border); border-radius: 8px; padding: 14px 16px; overflow-x: auto; font: 13px/1.6 "Geist Mono", ui-monospace, SFMono-Regular, Menlo, monospace; }
code { font: 13px/1.6 "Geist Mono", ui-monospace, SFMono-Regular, Menlo, monospace; }
:not(pre) > code { background: var(--bg-subtle); border: 1px solid var(--border); border-radius: 4px; padding: 1px 5px; font-size: 12.5px; }
/* Upstream parity: line-number gutter + dual-theme shiki tokens.
   Every code block's lines are <span data-line> inside
   code[data-line-numbers][data-line-numbers-max-digits=N]; the gutter is
   a CSS counter ::before (faded, right-aligned, tabular). Token spans
   carry --shiki-light/--shiki-dark inline vars; site chrome switches on
   :root.dark — same mechanism upstream uses for its vesper dark theme. */
article pre code[data-line-numbers] { display: block; counter-reset: ln; --ln-gutter: 1.5rem; }
article pre code[data-line-numbers-max-digits="2"] { --ln-gutter: 2rem; }
article pre code[data-line-numbers-max-digits="3"] { --ln-gutter: 2.5rem; }
article pre code[data-line-numbers] > span[data-line] { display: block; position: relative; padding-left: calc(var(--ln-gutter) + 1.25rem); }
article pre code[data-line-numbers] > span[data-line]::before {
  counter-increment: ln;
  content: counter(ln);
  position: absolute;
  inset-inline-start: 0;
  width: var(--ln-gutter);
  text-align: right;
  color: var(--muted);
  opacity: 0.6;
  user-select: none;
  font-variant-numeric: tabular-nums;
}
article pre span[data-line] span { color: var(--shiki-light, inherit); }
:root.dark article pre span[data-line] span { color: var(--shiki-dark, inherit); }
table { border-collapse: collapse; width: 100%; font-size: 14px; }
th, td { border: 1px solid var(--border); padding: 6px 12px; text-align: left; }
th { background: var(--bg-subtle); }
hr { border: 0; border-top: 1px solid var(--border); margin: 32px 0; }
blockquote { margin: 0; padding: 4px 16px; border-left: 4px solid var(--border); color: var(--muted); }
.component-preview-ph { border: 1px dashed var(--border); border-radius: 8px; padding: 28px 16px; margin: 20px 0; display: flex; gap: 12px; align-items: center; justify-content: center; color: var(--muted); background: var(--bg-subtle); }
.component-preview-ph .ph-note { font-size: 13px; font-style: italic; }
.component-preview-ph .ph-cross-link { font-size: 12px; }
.component-source-ph { border-style: dashed; }
.component-preview { border: 1px solid var(--border); border-radius: 8px; margin: 20px 0; overflow: hidden; }
/* Upstream-style stacked layout: preview iframe on top, source code below.
   The source starts collapsed to a capped sliver behind a gradient veil
   with a centered "View Code" button (mirrors ui.shadcn.com's
   ComponentPreview). No tab switching. */
.preview-frame {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px 16px;
  background: var(--bg);
  min-height: 18rem;
  position: relative;
}
.preview-frame iframe {
  display: block;
  width: 100%;
  min-height: 288px;
  height: 320px;
  border: 0;
  background: var(--bg);
}
/* Below the TOC cutoff the column alone can't always keep demos past
   their 40rem (sm:) breakpoint (240px sidebar + padding), so at tablet
   widths and up the iframe is floored at 41rem (16px over the breakpoint
   to survive an inner scrollbar) and the frame scrolls horizontally
   instead of letting desktop demos collapse to their mobile layout —
   e.g. alert-dialog's footer buttons stacking full-width. Mobile-width
   windows keep unfloored iframes: there the demos are SUPPOSED to show
   their responsive layout. Centering switches from justify-content to
   margin-inline:auto so a floored-too-wide iframe overflows only to the
   right (scroll-reachable), never both sides. */
@media (min-width: 768px) {
  .preview-frame { overflow-x: auto; justify-content: flex-start; }
  .preview-frame iframe { min-width: 41rem; margin-inline: auto; }
}
.preview-quality {
  position: absolute;
  top: 8px;
  right: 8px;
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 4px;
  background: var(--bg-subtle);
  color: var(--muted);
  font-style: italic;
  border: 1px dashed var(--border);
  pointer-events: none;
}
.preview-source {
  position: relative;
  border-top: 1px solid var(--border);
  background: var(--bg-subtle);
}
/* Collapsed: source body capped to a sliver, gradient veil + centered
   "View Code" button over it. data-open="true" (set by site.js on click)
   uncaps the body and hides the veil. */
.preview-source:not([data-open="true"]) .source-body {
  max-height: 12rem;
  overflow: hidden;
}
.preview-source[data-open="true"] .source-veil { display: none; }
.source-veil {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  padding-bottom: 1rem;
  background: linear-gradient(to bottom, transparent 0%, var(--bg-subtle) 78%);
}
.source-block + .source-block {
  border-top: 1px solid var(--border);
}
.source-label {
  padding: 6px 14px;
  background: var(--bg);
  color: var(--muted);
  font: 11px "Geist Mono", ui-monospace, SFMono-Regular, Menlo, monospace;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  border-bottom: 1px solid var(--border);
}
.preview-code { margin: 0; border: 0; border-radius: 0; max-height: 18rem; overflow: auto; }
/* Mobile: tighter preview padding; the veil's View Code button is the
   collapse affordance at every width (no <details> summary needed). */
@media (max-width: 768px) {
  .main { padding-inline: 16px; }
  .preview-frame { min-height: 0; padding: 12px; }
}
.callout { display: flex; gap: 12px; border: 1px solid var(--border); border-left-width: 4px; border-radius: 8px; padding: 12px 16px; margin: 20px 0; background: var(--bg-subtle); }
.callout svg { width: 18px; height: 18px; flex: none; margin-top: 3px; }
.callout[data-variant="info"] { border-left-color: var(--link); }
.callout[data-variant="warn"] { border-left-color: #9a6700; }
.callout[data-variant="error"] { border-left-color: #cf222e; }
.callout-title { margin: 0 0 4px; font-weight: 600; }
.callout-body > :last-child { margin-bottom: 0; }
.code-tabs { margin: 16px 0; }
.tabs-list { display: flex; gap: 4px; border-bottom: 1px solid var(--border); margin-bottom: -1px; }
.tab-trigger { border: 1px solid transparent; border-bottom: none; background: none; padding: 6px 14px; font: inherit; font-size: 13px; color: var(--muted); cursor: pointer; border-radius: 6px 6px 0 0; }
.tab-trigger[aria-selected="true"] { background: var(--bg-subtle); border-color: var(--border); color: var(--fg); font-weight: 600; }
.tab-content { margin: 0; }
.tab-content[hidden] { display: none; }
.steps { padding-left: 20px; }
[data-slot="accordion"] { border-radius: 8px; overflow: hidden; }
[data-slot="accordion-item"] { border-bottom: 1px solid var(--border); }
[data-slot="accordion-trigger"] { width: 100%; background: none; border: 0; padding: 14px 16px; text-align: left; font: inherit; font-size: 15px; cursor: pointer; color: var(--fg); display: flex; align-items: center; justify-content: space-between; gap: 12px; }
[data-slot="accordion-trigger"]:hover { background: var(--bg-subtle); }
[data-slot="accordion-content"] { padding: 0 16px 14px; color: var(--fg); }
[data-slot="accordion-item"][data-state="closed"] [data-slot="accordion-content"] { display: none; }
kbd { border: 1px solid var(--border); border-bottom-width: 2px; border-radius: 4px; padding: 1px 6px; font-size: 12px; background: var(--bg-subtle); }
.index-list { list-style: none; padding: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: 6px; }
.index-list a { display: block; padding: 6px 10px; border: 1px solid var(--border); border-radius: 6px; }
.index-list li.index-unavailable { display: flex; gap: 8px; align-items: baseline; justify-content: space-between; padding: 6px 10px; border: 1px dashed var(--border); border-radius: 6px; background: var(--bg-subtle); color: var(--muted-strong); cursor: default; word-break: keep-all; }
.index-list .index-group { grid-column: 1 / -1; padding: 14px 10px 4px; font-size: 12px; font-weight: 600; letter-spacing: 0.06em; text-transform: uppercase; color: var(--muted); }
/* FT5/FT8: greyed tombstone/FT6 entries — muted, dashed, no link.
   word-break:keep-all avoids breaking "navigation-menu" mid-hyphen (was
   wrapping to 3 lines in the grid at narrow card widths). */
.index-list li.index-unavailable .idx-note { font-size: 11.5px; font-style: italic; }
.sidebar li.sidebar-unavailable { padding: 3px 10px; font-size: 13px; color: var(--muted-strong); cursor: default; display: flex; align-items: center; justify-content: space-between; gap: 6px; }
.sidebar li.sidebar-unavailable::after { content: " — n/a"; font-size: 11px; font-style: italic; }
.sidebar .greyed-guide-link { font-size: 11px; color: var(--link); padding: 0 4px; }
.linked-card { display: flex; flex-direction: column; gap: 4px; border: 1px solid var(--border); border-radius: 10px; padding: 18px; color: var(--fg); }
.linked-card:hover { border-color: var(--link); text-decoration: none; }
.linked-card svg { width: 40px; height: 40px; }
.linked-card p { margin: 0; }
span.linked-card.unavailable-link { display: flex; border-style: dashed; color: var(--muted); }

/* FT13: heading permalink anchor — appended by site.js as <a class="anchor"
   href="#id">#</a> on every [id] heading. Hidden until the heading (or
   anchor) is hovered. Upstream affordance: small faded #
   floats to the left of the heading on hover. */
.typeset :is(h1, h2, h3, h4, h5, h6)[id] { position: relative; }
.typeset :is(h1, h2, h3, h4, h5, h6)[id] > a.anchor {
  position: absolute;
  inset-inline-start: -1.25em;
  font-weight: 400;
  color: var(--muted);
  text-decoration: none;
  opacity: 0;
  transition: opacity 0.15s ease;
}
.typeset :is(h1, h2, h3, h4, h5, h6)[id]:hover > a.anchor,
.typeset :is(h1, h2, h3, h4, h5, h6)[id] > a.anchor:focus-visible {
  opacity: 1;
}
.typeset :is(h1, h2, h3, h4, h5, h6)[id] > a.anchor:hover {
  color: var(--link);
}

/* FT8/Step 2 (reworked for upstream parity): Previous/Next nav mirrors
   ui.shadcn.com — small icon-only buttons at the top-right of the page
   header (.docs-nav), plus pill links at the BOTTOM of the article:
   prev on the left, next pulled right. Labels show the sibling page
   names ("Accordion" / "Alert Dialog" upstream). Placeholder spans keep
   the DOM stable when only one of prev/next exists. */
.header-row { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
.docs-nav { display: flex; gap: 8px; }
.docs-nav a {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--muted);
  background: var(--bg-subtle);
}
.docs-nav a:hover { color: var(--fg); border-color: var(--fg); text-decoration: none; }
.docs-nav svg { width: 15px; height: 15px; display: block; }
.page-prev-next {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 32px;
  max-width: 76ch;
}
.page-prev-next > span { display: none; }
.page-prev-next > a {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-subtle);
  font-size: 13px;
  color: var(--fg);
  transition: border-color 0.15s ease, background 0.15s ease;
}
.page-prev-next > a:hover {
  border-color: var(--fg);
  background: var(--bg);
  text-decoration: none;
}
.page-prev-next .pn-next { margin-left: auto; }
.page-prev-next svg { width: 14px; height: 14px; }

/* FT8/Step 7: icon-only copy button on code blocks (shadcn/ui uses a
   pure SVG icon in this affordance; we match). The View Code CTA is a
   TEXT button — upstream renders "View Code" with a label — styled
   below as the outline variant. */
article pre { position: relative; padding-right: 56px; }
button.code-copy {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 5px;
  color: var(--muted);
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: color 0.15s ease, border-color 0.15s ease;
}
button.code-copy:hover {
  color: var(--fg);
  border-color: var(--fg);
}
button.code-copy.copied {
  color: var(--link);
  border-color: var(--link);
}
button.code-copy svg {
  width: 14px;
  height: 14px;
  display: block;
}
button.code-copy {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 2;
}
/* FT8/Step 4 (reworked): centered "View Code" CTA inside the source
   veil — upstream's outline button over the gradient. Click reveals the
   full source body (see .preview-source rules above). */
button.view-code-cta {
  position: relative;
  z-index: 1;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 6px 14px;
  font: inherit;
  font-size: 13px;
  color: var(--fg);
  cursor: pointer;
}
button.view-code-cta:hover {
  border-color: var(--fg);
}
/* (superseded button.preview-rtl-toggle / button.preview-lang-select rules
   removed — the language affordance is .preview-rtl-langs; those selectors
   never matched anything in any built page) */

/* FT8/Step 9: RTL language selector. A small group of language
   buttons (AR / HE / EN / FA) in the top-left of the RTL preview
   frame. Each button swaps the iframe src to the matching
   NAME-rtl-LANG.html file when clicked; the active button shows the
   link-color highlight. Mirrors ui.shadcn.com's multi-language
   picker affordance (not a single dir toggle). */
.preview-rtl-langs {
  position: absolute;
  top: 8px;
  left: 8px;
  z-index: 3;
  display: inline-flex;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 1px;
  gap: 1px;
}
.preview-rtl-langs > button {
  background: transparent;
  border: 0;
  padding: 3px 8px;
  font: 11px "Geist Mono", ui-monospace, SFMono-Regular, Menlo, monospace;
  color: var(--muted);
  cursor: pointer;
  border-radius: 3px;
  letter-spacing: 0.04em;
  transition: color 0.15s ease, background 0.15s ease;
}
.preview-rtl-langs > button:hover {
  color: var(--fg);
  background: var(--bg-subtle);
}
.preview-rtl-langs > button[data-active-lang] {
  color: var(--link);
  background: var(--bg-subtle);
}
`

// Geist skin for the docs site (linked as docs/site/fonts.css by every
// docs page and injected into every docs/site/components/* demo copy —
// see injectSiteSkin in theme-prepaint.mjs). ui.shadcn.com renders its
// demos in Geist via next/font; out.css's @theme maps --font-sans /
// --font-mono / --font-heading to same-named custom properties (the
// next/font contract: the consumer defines them on <html>). Unlayered
// author CSS beats the layered @theme self-references, so these :root
// definitions win for the docs tree only — dist/ stays font-agnostic.
// Font binaries are vendored at docs/fonts/ (OFL — LICENSE.txt there)

// and copied to docs/site/assets/fonts/; url()
// resolves against this stylesheet's own location, so one copy serves
// root pages and the components/ demos that link it as ../fonts.css.
export const FONTS_CSS = `
@font-face {
  font-family: "Geist";
  font-style: normal;
  font-weight: 100 900;
  font-display: swap;
  src: url("assets/fonts/Geist-Variable.woff2") format("woff2");
}
@font-face {
  font-family: "Geist Mono";
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url("assets/fonts/GeistMono-Variable.woff2") format("woff2");
}
:root {
  --font-sans: "Geist", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --font-heading: "Geist", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --font-mono: "Geist Mono", ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}
`

// FT9: same pre-paint + storage sync used in theme-prepaint.mjs — derived
// from THEME_PREPAINT_SCRIPT (single source of truth) so the two copies
// cannot drift (FT11 recorded one such drift incident).
import { THEME_PREPAINT_SCRIPT } from './theme-prepaint.mjs'
export const themePrePaintInline = THEME_PREPAINT_SCRIPT
  .replace(/^<script>/, '').replace(/<\/script>$/, '')

export const SITE_JS = `
// FT8: propagate theme to all preview iframes (same-origin, so we can reach
// into iframe.contentDocument). Called on initial load AND after every
// iframe load event (the iframe's contentDocument is null until load).
// Avoids editing 300+ demo files individually.
function applyThemeToIframe(iframe, dark) {
  try {
    const doc = iframe.contentDocument
    if (doc && doc.documentElement) doc.documentElement.classList.toggle('dark', dark)
  } catch {}
}
function applyThemeToIframes(dark) {
  for (const iframe of document.querySelectorAll('iframe[data-preview-frame]')) {
    applyThemeToIframe(iframe, dark)
  }
}
const root = document.documentElement
function currentDark() { return root.classList.contains('dark') }
applyThemeToIframes(currentDark())

// FT8: auto-size preview iframes to their content. Same-origin so we read
// scrollHeight from contentDocument directly (no postMessage). Floor is
// 288px = upstream's h-72 preview height: small demos center in the same
// frame shadcn.com shows, and portal dialogs (alert-dialog/dialog/sheet)
// get room to open without the iframe clipping them. Caps at 900px so
// huge demos don't dominate the page. Runs on iframe load, theme change,
// viewport resize, and any DOM mutation (catches lazy iframes).
function resizeIframe(iframe) {
  try {
    const doc = iframe.contentDocument
    if (!doc) return
    const body = doc.body
    if (!body) return
    const h = Math.min(Math.max(body.scrollHeight, 288), 900)
    iframe.style.height = h + 'px'
  } catch {}
}
function resizeAllIframes() {
  for (const iframe of document.querySelectorAll('iframe[data-preview-frame]')) {
    resizeIframe(iframe)
  }
}
// Wire BOTH resize + theme to every iframe's load event (and to new
// iframes as they appear via MutationObserver). Without the load handler,
// contentDocument is null and both operations are no-ops.
function wireIframe(iframe) {
  iframe.addEventListener('load', () => {
    applyThemeToIframe(iframe, currentDark())
    setTimeout(resizeIframe.bind(null, iframe), 50)
  })
}
for (const iframe of document.querySelectorAll('iframe[data-preview-frame]')) {
  wireIframe(iframe)
}
new MutationObserver((muts) => {
  for (const m of muts) {
    for (const node of m.addedNodes) {
      if (node.tagName === 'IFRAME' && node.dataset?.previewFrame !== undefined) {
        wireIframe(node)
      }
      // also check subtree (lazy iframes added to existing wrappers)
      if (node.querySelectorAll) {
        for (const f of node.querySelectorAll('iframe[data-preview-frame]')) wireIframe(f)
      }
    }
  }
}).observe(document.body, { childList: true, subtree: true })
window.addEventListener('resize', resizeAllIframes)

// CodeTabs tab switching (site-local, vanilla)
for (const root of document.querySelectorAll('[data-code-tabs]')) {
  const triggers = [...root.querySelectorAll('[data-tab-trigger]')]
  const panels = [...root.querySelectorAll('[data-tab-content]')]
  if (!triggers.length || !panels.length) continue
  const activate = (value) => {
    for (const t of triggers) t.setAttribute('aria-selected', String(t.dataset.tabTrigger === value))
    for (const p of panels) p.hidden = p.dataset.tabContent !== value
  }
  for (const t of triggers) t.addEventListener('click', () => activate(t.dataset.tabTrigger))
  activate(triggers[0].dataset.tabTrigger)
}
// mark the active sidebar entry
const here = location.pathname.split('/').pop() || 'index.html'
for (const a of document.querySelectorAll('.sidebar a')) {
  if (a.getAttribute('href') === here) a.classList.add('active')
}
// Mobile sidebar drawer (below lg): the topbar hamburger toggles the
// sidebar as an overlay (data-open on #sidebar + backdrop drives the CSS
// slide/fade). Escape, backdrop click, and any nav link close it; body
// scroll locks while open; crossing back over lg resets the state so a
// later resize down doesn't resurrect an open drawer.
;(function () {
  const btn = document.querySelector('[data-sidebar-toggle]')
  const sidebar = document.querySelector('.sidebar')
  const backdrop = document.querySelector('[data-sidebar-backdrop]')
  if (!btn || !sidebar || !backdrop) return
  const isOpen = () => sidebar.getAttribute('data-open') === 'true'
  const setOpen = (open) => {
    sidebar.setAttribute('data-open', String(open))
    backdrop.setAttribute('data-open', String(open))
    btn.setAttribute('aria-expanded', String(open))
    btn.setAttribute('aria-label', open ? 'Close navigation' : 'Open navigation')
    document.body.classList.toggle('sidebar-open', open)
    // closing while focus sits inside the drawer would strand it on a
    // visibility:hidden element — hand it back to the toggle
    if (!open && sidebar.contains(document.activeElement)) btn.focus()
  }
  btn.addEventListener('click', () => setOpen(!isOpen()))
  backdrop.addEventListener('click', () => setOpen(false))
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && isOpen()) setOpen(false)
  })
  sidebar.addEventListener('click', (e) => {
    if (e.target.closest('a')) setOpen(false)
  })
  // same 1023.98px boundary as the CSS media query — no dead zone where
  // the shell is desktop but the scroll lock survives
  const mobile = window.matchMedia('(max-width: 1023.98px)')
  const onMql = (e) => { if (!e.matches) setOpen(false) }
  if (mobile.addEventListener) mobile.addEventListener('change', onMql)
  else mobile.addListener(onMql) // Safari < 14
})()
// FT8: theme toggle button — mirror inline-ComponentPreview behavior in
// dark-mode.mdx: toggle .dark on <html>, persist to localStorage, reflect
// state on the icon.
const themeBtn = document.querySelector('[data-theme-toggle]')
if (themeBtn) {
  const reflect = () => {
    const dark = document.documentElement.classList.contains('dark')
    themeBtn.setAttribute('aria-label', dark ? 'Use light theme' : 'Use dark theme')
    themeBtn.setAttribute('data-state', dark ? 'dark' : 'light')
  }
  themeBtn.addEventListener('click', () => {
    const dark = !document.documentElement.classList.contains('dark')
    document.documentElement.classList.toggle('dark', dark)
    try { localStorage.setItem('shadless-docs-theme', dark ? 'dark' : 'light') } catch (e) {}
    reflect()
    applyThemeToIframes(dark)
    // Re-resize iframes after theme toggle (some demos may change layout height)
    setTimeout(resizeAllIframes, 100)
  })
  reflect()
}
// FT8: inline Accordion shim for prose (introduction.mdx, any guide using
// <Accordion>). Click + Enter/Space on the trigger toggles its state via
// the same data-state attributes the runtime uses. Avoid double-init when
// the same root is also wired by shadless.js.
for (const trigger of document.querySelectorAll('[data-slot="accordion-trigger"]')) {
  if (trigger.dataset.shimInit) continue
  trigger.dataset.shimInit = '1'
  const item = trigger.closest('[data-slot="accordion-item"]')
  const root = trigger.closest('[data-slot="accordion"]')
  const content = item?.querySelector('[data-slot="accordion-content"]')
  if (!item || !content) continue
  const multiple = root?.getAttribute('data-type') === 'multiple'
  trigger.addEventListener('click', () => {
    const wasOpen = trigger.getAttribute('data-state') === 'open'
    const open = !wasOpen
    if (!multiple && open && root) {
      for (const t of root.querySelectorAll('[data-slot="accordion-trigger"]')) {
        if (t === trigger) continue
        t.setAttribute('data-state', 'closed')
        t.setAttribute('aria-expanded', 'false')
        const c = t.closest('[data-slot="accordion-item"]')?.querySelector('[data-slot="accordion-content"]')
        if (c) { c.setAttribute('data-state', 'closed'); c.hidden = true }
      }
    }
    trigger.setAttribute('data-state', open ? 'open' : 'closed')
    trigger.setAttribute('aria-expanded', String(open))
    content.setAttribute('data-state', open ? 'open' : 'closed')
    content.hidden = !open
    item.setAttribute('data-state', open ? 'open' : 'closed')
  })
}

// FT13: heading permalink anchor — append a # anchor to every heading with
// an id inside .typeset. Hidden until the heading is hovered. Click on the
// anchor updates the URL hash (native <a> behavior); the surrounding click
// handler additionally copies the absolute URL to clipboard.
(function () {
  const headings = document.querySelectorAll('.typeset :is(h1, h2, h3, h4, h5, h6)[id]')
  for (const h of headings) {
    if (h.querySelector('a.anchor')) continue
    const a = document.createElement('a')
    a.className = 'anchor'
    a.href = '#' + h.id
    a.setAttribute('aria-label', 'Permalink to ' + h.textContent.trim())
    a.textContent = '#'
    h.prepend(a)
  }
  document.addEventListener('click', (e) => {
    const h = e.target?.closest?.('.typeset :is(h1, h2, h3, h4, h5, h6)[id]')
    if (!h || e.target.closest('a')) return
    // FT14: heading click — update URL hash + copy full URL. Mirrors the
    // <a class="anchor"> click behavior (which updates hash natively) so
    // both interactions produce the same effect: hash reflects heading,
    // clipboard holds the absolute link. history.pushState avoids
    // re-scrolling (the heading is already in view since it was clicked).
    // location.origin is the string "null" under file:// — derive the URL
    // from href so local previews copy something usable.
    const url = location.href.split('#')[0] + '#' + h.id
    history.pushState(null, '', '#' + h.id)
    if (navigator.clipboard?.writeText) {
      navigator.clipboard.writeText(url).catch(() => {})
    }
  })
})()

// FT8/Step 7: SVG icon glyphs used inside the action buttons.
// Match the lucide outline style used elsewhere in shadless. Each
// icon is a minimal path; combined with the button's own border + bg
// they read as small affordance buttons, not noisy chrome.
const ICON_COPY = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>'
const ICON_CHECK = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>'

// FT8/Step 3: Copy button on every <pre> in the article area. Matches
// shadcn/ui's affordance — small button top-right of the code block,
// copies the inner code text to clipboard, briefly flips to check icon.
// Selector: article pre (covers body code blocks + ComponentPreview
// source <details>). Idempotent + MutationObserver for lazy content.
function wireCopyButton(pre) {
  if (pre.querySelector(':scope > button.code-copy')) return
  const btn = document.createElement('button')
  btn.type = 'button'
  btn.className = 'code-copy'
  btn.setAttribute('aria-label', 'Copy code to clipboard')
  btn.title = 'Copy code'
  btn.innerHTML = ICON_COPY
  const restoreCopy = () => { btn.innerHTML = ICON_COPY; btn.classList.remove('copied') }
  btn.addEventListener('click', async () => {
    const code = pre.querySelector('code')
    const text = code ? code.innerText : pre.innerText
    try {
      await navigator.clipboard.writeText(text)
      btn.innerHTML = ICON_CHECK
      btn.classList.add('copied')
      setTimeout(restoreCopy, 1200)
    } catch (e) {
      restoreCopy()
    }
  })
  pre.appendChild(btn)
}
function wireAllCopyButtons() {
  for (const pre of document.querySelectorAll('article pre')) wireCopyButton(pre)
}
wireAllCopyButtons()
new MutationObserver(wireAllCopyButtons).observe(document.body, { childList: true, subtree: true })

// FT8/Step 4 (reworked): "View Code" CTA inside each ComponentPreview's
// source veil. Clicking sets data-open="true" on the .preview-source
// wrapper — CSS uncaps .source-body and hides the veil. aria-expanded
// reflects state. Reveal-only (upstream has no collapse-back affordance).
function wireViewCodeButtons() {
  for (const btn of document.querySelectorAll('button[data-view-code]')) {
    if (btn.dataset.wired) continue
    btn.dataset.wired = '1'
    const source = btn.closest('.preview-source')
    if (!source) continue
    const body = source.querySelector('.source-body')
    if (body && !body.id) body.id = 'src-' + Math.random().toString(36).slice(2, 9)
    if (body) btn.setAttribute('aria-controls', body.id)
    btn.addEventListener('click', () => {
      source.setAttribute('data-open', 'true')
      btn.setAttribute('aria-expanded', 'true')
    })
  }
}
wireViewCodeButtons()
new MutationObserver(wireViewCodeButtons).observe(document.body, { childList: true, subtree: true })

// FT8/Step 9: RTL language selector. Each RTL preview frame has a
// preview-rtl-langs group with one button per available language
// (AR / HE / EN / FA). Clicking a button swaps the iframe src to the
// matching NAME-rtl-LANG.html file (built by tools/build-rtl.mjs).
// To keep the iframe load console-error-free (no spurious 404s for
// components that don't ship all 4 languages), the host page passes
// a list of available languages per preview via a sibling data-attr;
// buttons for missing languages are removed at build time, not
// hidden at runtime.
function wireRtlLangs() {
  for (const group of document.querySelectorAll('.preview-rtl-langs')) {
    if (group.dataset.wired) continue
    group.dataset.wired = '1'
    const frame = group.closest('.preview-frame')
    const iframe = frame?.querySelector('iframe[data-preview-frame]')
    if (!iframe) continue
    const baseSrc = iframe.getAttribute('src') // e.g. "components/alert-rtl.html"
    const dotIdx = baseSrc.lastIndexOf('.')
    const stem = dotIdx === -1 ? baseSrc : baseSrc.slice(0, dotIdx)
    const ext = dotIdx === -1 ? '' : baseSrc.slice(dotIdx)
    group.querySelectorAll('button[data-rtl-lang]').forEach((btn) => {
      const lang = btn.dataset.rtlLang
      const variantSrc = lang === 'ar' ? baseSrc : stem + "-" + lang + ext
      btn.addEventListener('click', () => {
        iframe.setAttribute('src', variantSrc)
        group.querySelectorAll('button[data-active-lang]').forEach((b) => b.removeAttribute('data-active-lang'))
        btn.setAttribute('data-active-lang', '')
        setTimeout(resizeAllIframes, 100)
      })
    })
  }
}
wireRtlLangs()
new MutationObserver(wireRtlLangs).observe(document.body, { childList: true, subtree: true })
`
