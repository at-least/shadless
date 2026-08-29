#!/usr/bin/env node
// product-css (J2) — derive the npm-consumable product surface from the
// pipeline's demo-oriented artifacts:
//
//   dist/shadless-core.css          theme vars + @theme + custom variants +
//                                @utility helpers + keyframes (NO docs-site
//                                chrome, NO demo @source, NO demo body pad)
//   dist/css/<name>.css          per-component @layer block (@apply source,
//                                written by tools/demo.mjs — READ here)
//   dist/shadless.product.css    tokens + fixes + all parts (tailwind input)
//   dist/shadless.full[.min].css compiled by the demo chain (zero-build use)
//
// Why extraction instead of shipping globals.css wholesale: probes/h4/
// globals.css was captured to render the shadcn ORACLE site correctly, so it
// carries site chrome (rehype-pretty-code prose, steps, typeset, dialog-ring,
// style-* packs, a:active dimming, overscroll resets) that must NOT leak
// into consumer pages. The keep-list below is deliberately narrow; --verify
// proves the extracted set still compiles every component rule.
//
// Usage:
//   node tools/product-css.mjs            # write tokens + product entry
//   node tools/product-css.mjs --verify   # compiled full.css ⊇ component rules
import { readFileSync, writeFileSync, mkdirSync, readdirSync, existsSync } from "node:fs"
import { fileURLToPath } from "node:url"
import path from "node:path"

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..")

// Pull a balanced block (at-rule or selector) starting at the index of its
// opening brace. Returns [blockText, nextIndex].
function takeBlock(text, openIdx) {
  let depth = 0
  for (let i = openIdx; i < text.length; i++) {
    if (text[i] === "{") depth++
    else if (text[i] === "}") {
      depth--
      if (depth === 0) return [text.slice(openIdx, i + 1), i + 1]
    }
  }
  throw new Error("unbalanced block")
}

// Header text (at-rule/selector) from line start up to the opening brace.
function headerOf(text, braceIdx) {
  return text.slice(text.lastIndexOf("\n", braceIdx) + 1, braceIdx).trim()
}

export function extractTokens(globals) {
  const keep = []
  // 1. @import lines (tailwindcss + tw-animate-css)
  for (const m of globals.matchAll(/^@import .+;$/gm)) keep.push(m[0])
  // 2. the inlined shadcn library tailwind.css (marker-bounded)
  const begin = globals.indexOf("/* === begin inlined shadcn/tailwind.css === */")
  const end = globals.indexOf("/* === end inlined shadcn/tailwind.css === */")
  if (begin < 0 || end < 0) throw new Error("inlined shadcn/tailwind.css markers missing")
  keep.push(globals.slice(begin, end + "/* === end inlined shadcn/tailwind.css === */".length))
  // 3. dark-mode custom variant (line-scoped exact grab)
  const dark = globals.match(/^@custom-variant dark [^\n]+$/m)
  if (dark) keep.push(dark[0])
  // 4. walk top-level-ish at-rule/selector blocks with balanced braces;
  //    keep only the product-relevant ones
  for (const m of globals.matchAll(/@theme[^{]*\{|(^|\n)[ \t]*(:root|\.dark)\s*\{/g)) {
    const braceIdx = m.index + m[0].length - 1
    const header = headerOf(globals, braceIdx)
    const body = takeBlock(globals, braceIdx)[0]
    if (header.startsWith("@theme") && body.includes("--color-background:")) keep.push(header + body)
    else if ((header === ":root" || header === ".dark") && body.includes("--background:")) keep.push(header + " " + body)
  }
  // 5. the base border/outline reset rule (indented inside @layer base, so
  //    the line-anchored walk above would miss it; flat [^}] body is safe)
  const star = globals.match(/\n[ \t]*\*[ \t]*\{[^}]*@apply border-border[^}]*\}/)
  if (star) keep.push(`@layer base {${star[0].trim()}\n}`)
  return keep.join("\n\n") + "\n"
}

export function buildProductEntry(tokensCss, fixesCss, partsCss) {
  return `${tokensCss}\n${fixesCss}\n${partsCss}\n`
}

// --verify: every [data-slot="…"] selector in the per-component PARTS must
// survive compilation in BOTH chains (a missing custom variant / @utility /
// token silently drops rules). out.css legitimately carries extra docs-site
// slots (layout/copy-button/docs chrome) — those must NOT appear in the
// product build, which is what the chrome check enforces. The stray-class
// check enforces hermeticity: every standalone class rule in the product
// build must trace back to the product source text (unescaped), else
// content scanning leaked into the build (see tools/tw.mjs).
export function verifyProduct(fullCss, outCss, partsCss, productSource) {
  const slotSet = (css) => new Set([...css.matchAll(/\[data-slot="([^"]+)"\]/g)].map((m) => m[1]))
  const expected = slotSet(partsCss)
  const fullSlots = slotSet(fullCss)
  const outSlots = slotSet(outCss)
  const missing = [...expected].filter((s) => !fullSlots.has(s))
  const demoDropped = [...expected].filter((s) => !outSlots.has(s))
  const chrome = ["rehype", "typeset", "dialog-ring", "style-vega", "data-wrapper", '[data-slot="docs"]', '[data-slot="layout"]', '[data-slot="copy-button"]']
    .filter((needle) => fullCss.includes(needle))
  // runtime vars the compiled output must carry. NOTE: --color-* aliases
  // are compile-time only under `@theme inline` (inlined at use sites), so
  // they never appear literally in a correct build — do not "fix" this by
  // adding them back.
  const tokens = ["--background:", "--radius:"]
    .filter((tok) => !fullCss.includes(tok))
  // compiled @apply of VARIANT-qualified utilities (data-[position=popper]:h-…)
  // emits the variant into the selector (.anchor[data-position=popper]) — a
  // legitimate compile artifact of a product-source rule, so compare the
  // base class too before calling it stray
  const stray = [...fullCss.matchAll(/^  \.(\S+) \{$/gm)].map((m) => m[1].replace(/\\(.)/g, "$1"))
    .filter((cls) => {
      const base = cls.replace(/\[data-[^\]]*\]/g, "")
      return !productSource.includes(cls) && !productSource.includes(base)
    })
  return { missing, demoDropped, chrome, tokens, stray }
}

function main() {
  // dist/globals.css (composed by the demo chain: probe base + fixes +
  // nova skin utilities + slot rules) when present; the probe capture
  // alone predates the skin and cannot supply the @utility cn-* defs the
  // product slot rules @apply.
  const globalsPath = existsSync(path.join(ROOT, "dist/globals.css"))
    ? path.join(ROOT, "dist/globals.css")
    : path.join(ROOT, "probes/h4/globals.css")
  // the demo entry turns tailwind's automatic content detection off
  // (source(none) + explicit @source, see tools/demo.mjs); the CONSUMER's
  // build must keep detection on — their pasted markup is what it scans
  const globals = readFileSync(globalsPath, "utf8")
    .replace('@import "tailwindcss" source(none);', '@import "tailwindcss";')

  if (process.argv.includes("--verify")) {
    const full = readFileSync(path.join(ROOT, "dist/shadless.full.css"), "utf8")
    const out = readFileSync(path.join(ROOT, "dist/out.css"), "utf8")
    const parts = readdirSync(path.join(ROOT, "dist/css")).filter((f) => f.endsWith(".css") && f !== "shadless.css").sort()
      .map((f) => readFileSync(path.join(ROOT, "dist/css", f), "utf8")).join("\n")
    const productSource = readFileSync(path.join(ROOT, "dist/shadless.product.css"), "utf8")
    const { missing, demoDropped, chrome, tokens: missingTokens, stray } = verifyProduct(full, out, parts, productSource)
    const problems = []
    if (missing.length) problems.push(`slot rules missing from product build: ${missing.join(", ")}`)
    if (demoDropped.length) problems.push(`slot rules missing from DEMO build (both chains disagree): ${demoDropped.join(", ")}`)
    if (chrome.length) problems.push(`docs chrome leaked into product build: ${chrome.join(", ")}`)
    if (missingTokens.length) problems.push(`tokens missing from product build: ${missingTokens.join(", ")}`)
    if (stray.length) problems.push(`standalone classes with no origin in product source (content-scan leak?): ${stray.join(", ")}`)
    if (problems.length) {
      console.error(`FAIL  product-css --verify\n  ${problems.join("\n  ")}`)
      process.exit(1)
    }
    console.log(`PASS  product-css --verify (${new Set([...parts.matchAll(/\[data-slot="([^"]+)"\]/g)].map((m) => m[1])).size} slot rules in both chains, no docs chrome, no stray classes)`)
    return
  }

  mkdirSync(path.join(ROOT, "dist/css"), { recursive: true })
  // tw-animate-css is INLINED into the product surface: the consumer story
  // is "two @imports + your tailwind build" — an extra npm package (or a
  // resolvable node_modules) for the animate layer would break that. The
  // demo globals keep the real @import (it resolves inside this repo).
  const animate = readFileSync(path.join(ROOT, "node_modules/tw-animate-css/dist/tw-animate.css"), "utf8").trim()
  const tokens = extractTokens(globals).replace(
    '@import "tw-animate-css";',
    `/* === begin inlined tw-animate-css (self-contained product surface) === */\n${animate}\n/* === end inlined tw-animate-css === */`
  )
  if (tokens.includes('@import "tw-animate-css";')) throw new Error("tw-animate-css import not replaced")
  const strayImports = [...tokens.matchAll(/^@import ("[^"]+"|url\([^)]*\));?$/gm)].map((m) => m[1]).filter((s) => s !== '"tailwindcss"')
  if (strayImports.length) throw new Error(`shadless.css not self-contained — unresolved @import(s): ${strayImports.join(", ")}`)
  writeFileSync(path.join(ROOT, "dist/shadless-core.css"),
    `/* shadless theme — extracted from probes/h4/globals.css by tools/product-css.mjs.\n` +
    `   Product surface ONLY: theme vars, @theme, custom variants, @utility helpers,\n` +
    `   keyframes. Deliberately excluded: docs-site chrome (prose/steps/packs),\n` +
    `   demo @source and demo body padding. The only @import left is "tailwindcss"\n` +
    `   itself — the animate layer is inlined so consumers need nothing else. */\n` + tokens)

  // product entry = tokens + fixes + per-component parts (written by demo.mjs)
  const fixes = readFileSync(path.join(ROOT, "src/docs/theme-prepaint.mjs"), "utf8")
    .match(/export const SHADLESS_CSS_FIXES = `([^`]*)`/)[1]
  const parts = readdirSync(path.join(ROOT, "dist/css")).filter((f) => f.endsWith(".css") && f !== "shadless.css").sort()
    .map((f) => readFileSync(path.join(ROOT, "dist/css", f), "utf8").trim())
  if (!parts.length) throw new Error("dist/css has no per-component files — run the demo chain first")
  writeFileSync(path.join(ROOT, "dist/shadless.product.css"),
    buildProductEntry(readFileSync(path.join(ROOT, "dist/shadless-core.css"), "utf8"), fixes, parts.join("\n\n")))
  console.log(`product-css: shadless-core.css + shadless.product.css (${parts.length} component parts)`)
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main()
