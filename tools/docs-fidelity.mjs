#!/usr/bin/env node
// docs-fidelity.mjs — Wave I gate B: mdx↔html page fidelity.
//
// For EVERY page recorded in docs/content-map.json (components +
// guides), compare the upstream/authored mdx source against the built
// docs/site/<name>.html: h1/lead vs frontmatter, heading order+text,
// TOC, ComponentPreview names/statuses, code-fence survival
// (full-content), link chips, manual-tab rewrite marker, prev/next
// targets, iframe targets, unrewritten /docs/ hrefs. Catches silent
// content loss that render/console/link gates can't see (a dropped
// section still "renders", a swallowed fence still has 0 console
// errors).
//
// The transforms applied before fact extraction (manual-tab drop,
// install-section drop, text adjustments) come from
// src/docs/transforms.mjs — the SAME module the builder uses, so the
// gate cannot drift from the build.
//
// Reverse assertions (2026-08-26 variant retirement + prose rules):
// the retired variant strip/pages/hrefs and declared TEXT_ADJUSTMENTS
// finds must never survive into a built page.
import { readFileSync, existsSync, readdirSync } from "node:fs"
import { join, basename } from "node:path"
import { GUIDES } from "./docs-guides.mjs"
import { mdxPageFacts, htmlPageFacts, comparePage, htmlText } from "./docs-fidelity-lib.mjs"
import { applyTextAdjustments, TEXT_ADJUSTMENTS } from "../src/docs/transforms.mjs"

const SITE = "docs/site"
if (!existsSync(join(SITE, "site.js"))) {
  console.error("FAIL  docs-fidelity: docs/site/ not built — run the docs chain first (make docs / npm run docs)")
  process.exit(1)
}

const contentMap = JSON.parse(readFileSync("docs/content-map.json", "utf8"))
const diskNames = new Set(readdirSync(SITE).filter((f) => f.endsWith(".html")).map((f) => f.replace(/\.html$/, "")))
const guideBySlug = new Map(GUIDES.map((g) => [g.slug, g]))

const issues = [] // {page, kind, detail}
let pages = 0

for (const [name, meta] of Object.entries(contentMap.pages)) {
  if (name === "index") continue
  const htmlPath = join(SITE, `${name}.html`)
  if (!existsSync(htmlPath)) { issues.push({ page: name, kind: "missing-page", detail: "content-map page has no built html" }); continue }
  pages++
  const isComponent = /\/components\/radix\//.test(meta.source ?? "")
  const guide = guideBySlug.get(name)

  const rawPath = meta.source
  if (!rawPath || !existsSync(rawPath)) { issues.push({ page: name, kind: "source-missing", detail: `source ${rawPath} unreadable` }); continue }
  let raw = readFileSync(rawPath, "utf8")
  // simulate the builder's text adjustments so mdx facts mirror the build
  try {
    raw = applyTextAdjustments(basename(rawPath), raw)
  } catch (e) {
    issues.push({ page: name, kind: "text-adjustment", detail: e.message })
    continue
  }
  const M = mdxPageFacts(raw, {
    dropCodeTabs: isComponent,
    dropInstallSection: Boolean(guide?.installSection),
    dropRtlMigrate: Boolean(guide?.rtlMigrate),
    dropUsageSection: isComponent,
    dropCompositionSection: isComponent,
  })
  const H = htmlPageFacts(readFileSync(htmlPath, "utf8"))

  for (const d of comparePage(M, H, {
    pageName: name,
    isComponentPage: isComponent,
    expectedManualRef: isComponent ? `dist/components/${name}.html` : null,
  })) {
    const [kind, ...rest] = d.split(": ")
    issues.push({ page: name, kind, detail: rest.join(": ") })
  }

  // ---- disk-existence checks (need the site tree) ----
  for (const [label, target] of [["prev", H.pnPrev], ["next", H.pnNext]]) {
    if (target && !diskNames.has(target)) issues.push({ page: name, kind: `pn-${label}`, detail: `target ${target} has no built page` })
  }
  for (const src of H.iframes) {
    if (!existsSync(join(SITE, src))) issues.push({ page: name, kind: "iframe-404", detail: src })
  }
  if (H.docsHrefs.length) issues.push({ page: name, kind: "docs-href", detail: `unrewritten: ${[...new Set(H.docsHrefs)].join(", ")}` })

  // ---- variant retirement (2026-08-26): the base/aria mirror is GONE.
  // Reverse assertion — catches the strip or variant hrefs growing back
  // after an upstream upgrade re-introduces them into the build path.
  if (H.variantTabs.length) issues.push({ page: name, kind: "variant-tabs", detail: `${H.variantTabs.length} variant tab strip(s) present — base/aria mirror is retired` })
  const variantHrefs = H.allHrefs.filter((h) => /-(base|aria)\.html$/.test(h))
  if (variantHrefs.length) issues.push({ page: name, kind: "variant-href", detail: `links to retired variant pages: ${[...new Set(variantHrefs)].join(", ")}` })

  // ---- retired prose (TEXT_ADJUSTMENTS): declared product decisions
  // must be applied — a surviving find string means the build skipped
  // the substitution (or upstream re-introduced it)
  const adj = TEXT_ADJUSTMENTS.find((a) => a.files.includes(basename(rawPath)))
  if (adj) {
    const text = htmlText(H.article)
    for (const op of adj.ops) {
      if (text.includes(op.find)) issues.push({ page: name, kind: "retired-prose", detail: `${adj.id}: "${op.find.slice(0, 60)}" survives in the built page` })
    }
  }
}

// retired variant pages must not exist on disk at all
for (const f of readdirSync(SITE)) {
  if (/\.(base|aria)\.html$/.test(f)) issues.push({ page: f, kind: "variant-page", detail: "retired base/aria page exists on disk" })
}

// ---- report ----
const byKind = {}
for (const i of issues) (byKind[i.kind] ??= []).push(i)
for (const [kind, list] of Object.entries(byKind)) {
  console.error(`FAIL  ${kind} (${list.length}):`)
  for (const i of list.slice(0, 8)) console.error(`  - [${i.page}] ${i.detail}`)
  if (list.length > 8) console.error(`  … +${list.length - 8} more`)
}
console.log(`docs fidelity: ${pages} pages compared against mdx sources — issues: ${issues.length}`)
if (issues.length) {
  console.error("FAIL  docs fidelity (built pages drift from their mdx sources)")
  process.exit(1)
}
console.log("PASS  docs fidelity (every page matches its mdx source: headings/TOC/previews/fences/links)")
