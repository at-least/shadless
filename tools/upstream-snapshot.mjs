#!/usr/bin/env node
// upstream-snapshot — golden-master snapshot of ui.shadcn.com example DOM.
//
// Hop 1 of the 1:1 gate: the LOCAL oracle render must equal what the live
// upstream site actually serves, else "oracle parity" only proves we match
// ourselves. This tool crawls /docs/components/radix/<page> (~50 pages),
// slices each <div data-slot="component-preview">'s demo DOM out of the
// SSR payload (document order == mdx ComponentPreview order, names come
// from the pinned mdx), normalizes runtime ids, and stores everything in
// src/registry/upstream-snapshot/ as a COMMITTED artifact — CI compares
// offline (deterministic, no network flake); refresh on re-pin.
//
//   node tools/upstream-snapshot.mjs            # all radix pages
//   node tools/upstream-snapshot.mjs --page alert
//
// Normalization (same family as example-oracle's norm): radix auto ids.
// Attribute-order differences between SSR and CSR are irrelevant — the
// GATE compares parsed DOM structure, not strings.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, existsSync } from "node:fs"
import { join } from "node:path"
import { fenceShadow } from "../src/docs/transforms.mjs"

// Which of upstream's three parallel registries this repo targets is recorded
// ONCE, in src/registry/pin.json's `registry` path. It used to be spelled out
// here twice more — a docs directory and a crawl URL, both with "radix" baked
// in — with nothing comparing the three. Change one and the golden hop
// compares radix-generated pages against another base's LIVE pages, where
// every diff looks like a real regression rather than a mismatched comparison.
const PIN = JSON.parse(readFileSync("src/registry/pin.json", "utf8"))
const PINNED_BASE = (/registry\/bases\/([^/]+)\//.exec(PIN.shadcn_ui?.registry ?? "") ?? [])[1]
if (!PINNED_BASE) {
  console.error(`FAIL  upstream-snapshot: src/registry/pin.json has no \`shadcn_ui.registry\` of the form apps/v4/registry/bases/<base>/ui — cannot tell which base to crawl`)
  process.exit(1)
}
const DOCS_DIR = `.upstream/shadcn-ui/apps/v4/content/docs/components/${PINNED_BASE}`
const OUT_DIR = "src/registry/upstream-snapshot"
const BASE = `https://ui.shadcn.com/docs/components/${PINNED_BASE}`

const norm = (html) => html
  .replace(/radix-:r[a-z0-9]*:?/g, "radix-<auto>")
  .replace(/radix-_r_[a-z0-9-]*/g, "radix-<auto>")
  // SSR react-useId ids (radix-_R_2k3aa…_) normalize to the same bucket
  // as CSR ones (radix-:r1:) — runtime-generated, not part of the contract
  .replace(/radix-_R_[a-z0-9-]*/gi, "radix-<auto>")

// preview names in mdx document order (fences shadowed so fenced
// ComponentPreview text can't reorder the mapping)
export function previewNames(mdx) {
  const shadow = fenceShadow(mdx)
  return [...shadow.matchAll(/<ComponentPreview\b([^>]*)>/g)]
    .map((m) => /name="([^"]*)"/.exec(m[1])?.[1] ?? null)
    .filter(Boolean)
}

// Slice the demo DOM of every component-preview block out of SSR HTML.
// The demo container is the data-align div INSIDE data-slot="preview";
// its children are the demo markup. Returns array of innerHTML strings
// in document order.
export function slicePreviews(html) {
  const out = []
  // preview wrapper (dir varies, RTL demos differ) then the demo container
  const open = /<div data-slot="preview"[^>]*><div data-align="[^"]*" data-chromeless="false" class="preview[^"]*">/g
  for (const m of html.matchAll(open)) {
    const start = m.index + m[0].length
    // stack-balance to the closing </div> of the data-align container
    let depth = 1, i = start
    while (depth > 0 && i < html.length) {
      const nextOpen = html.indexOf("<div", i)
      const nextClose = html.indexOf("</div>", i)
      if (nextClose === -1) break
      if (nextOpen !== -1 && nextOpen < nextClose) { depth++; i = nextOpen + 4 }
      else { depth--; i = nextClose + 6 }
    }
    out.push(html.slice(start, html.lastIndexOf("</div>", i - 6)))
  }
  return out
}

const pageArgIdx = process.argv.indexOf("--page")
const only = pageArgIdx >= 0 ? process.argv[pageArgIdx + 1] : null

// pages with no component-preview on upstream (verified: sidebar and
// typography render through different chrome — and both sit in our grey
// list anyway: sidebar tombstone, typography FT6 guide)
const SKIP = new Set(["sidebar", "typography"])

mkdirSync(OUT_DIR, { recursive: true })
const pages = readdirSync(DOCS_DIR).filter((f) => f.endsWith(".mdx")).map((f) => f.replace(/\.mdx$/, "")).sort()
  .filter((p) => !SKIP.has(p))
  .filter((p) => !only || p === only)

let total = 0, failed = 0
for (const page of pages) {
  const mdx = readFileSync(join(DOCS_DIR, `${page}.mdx`), "utf8")
  const names = previewNames(mdx)
  const res = await fetch(`${BASE}/${page}`)
  if (!res.ok) { console.error(`FAIL ${page}: HTTP ${res.status}`); failed++; continue }
  const html = await res.text()
  const slices = slicePreviews(html)
  if (slices.length !== names.length) {
    console.error(`FAIL ${page}: ${names.length} mdx previews != ${slices.length} SSR slices`)
    failed++
    continue
  }
  const doc = { page, previews: {} }
  names.forEach((n, i) => { doc.previews[n] = norm(slices[i].trim()) })
  writeFileSync(join(OUT_DIR, `${page}.json`), JSON.stringify(doc, null, 1) + "\n")
  total += names.length
  console.log(`${page}: ${names.length} previews`)
}
if (failed) { console.error(`FAIL  upstream-snapshot (${failed} pages)`); process.exit(1) }
console.log(`upstream-snapshot: ${total} previews across ${pages.length} pages -> ${OUT_DIR}`)
