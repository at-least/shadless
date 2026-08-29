#!/usr/bin/env node
// docs-consistency.mjs — Wave I gate A: demo-tree parity.
//
// docs/site/components/* must be the injectSiteSkin (pre-paint + Geist
// fonts.css link) image of its source tree. Three invariants, all byte-exact:
//   1. for every dist/components/<f>.html  → site copy == injectSiteSkin(dist)
//   2. for every authored demo docs/demos/<name>.html (catalog status
//      "authored") → site copy == injectSiteSkin(demos file)
//   3. shared assets (out.css, shadless.js, js/) are
//      byte-identical dist↔site
//
// Why this gate exists (2026-08-26 audit): build-rtl wrote dist RTL
// variants without the pre-paint script while every other emitter
// injected it; a later mirror step then copied the un-injected
// dist content over the already-injected site copy —
// docs/site/components/alert-rtl-fa.html shipped without pre-paint and
// no existing gate noticed (render/console/links were all green).
//
// Also detects: stale site after dist rebuild (docs/site is a committed
// vendor output), a patch applied to dist but not mirrored, a demo file
// added to one tree but not the other.
//
// Run AFTER the full docs chain (docs-build). Exits 1
// on the first kind of mismatch with per-file detail (capped).
import { readFileSync, existsSync, readdirSync } from "node:fs"
import { join } from "node:path"
import { injectSiteSkin } from "../src/docs/theme-prepaint.mjs"

const SITE = "docs/site"
const problems = []
const problem = (kind, file, detail) => problems.push({ kind, file, detail })

if (!existsSync(join(SITE, "components"))) {
  console.error("FAIL  docs-consistency: docs/site/components/ not found — run the docs chain first (make docs / npm run docs)")
  process.exit(1)
}

// ---- 1. dist tree → site image ------------------------------------------------
const distFiles = readdirSync("dist/components").filter((f) => f.endsWith(".html")).sort()
let distChecked = 0
for (const f of distFiles) {
  const distHtml = readFileSync(join("dist/components", f), "utf8")
  const sitePath = join(SITE, "components", f)
  if (!existsSync(sitePath)) { problem("site-missing", f, "dist demo has no docs/site copy"); continue }
  const want = injectSiteSkin(distHtml)
  const got = readFileSync(sitePath, "utf8")
  distChecked++
  if (want !== got) problem("site-drift", f, `site copy ≠ injectSiteSkin(dist) (dist len=${distHtml.length}, want=${want.length}, site=${got.length})`)
}

// ---- 2. authored demos → site image --------------------------------------------
const catalog = JSON.parse(readFileSync("docs/catalog.json", "utf8"))
let authoredChecked = 0
for (const p of catalog.previews) {
  if (p.status !== "authored") continue
  const demosPath = `docs/demos/${p.name}.html`
  if (!existsSync(demosPath)) { problem("demos-missing", p.name, `catalog says authored but ${demosPath} is gone`); continue }
  const sitePath = join(SITE, "components", `${p.name}.html`)
  if (!existsSync(sitePath)) { problem("site-missing", p.name, "authored demo has no docs/site copy"); continue }
  const want = injectSiteSkin(readFileSync(demosPath, "utf8"))
  const got = readFileSync(sitePath, "utf8")
  authoredChecked++
  if (want !== got) problem("site-drift", p.name, `site copy ≠ injectSiteSkin(docs/demos) (want=${want.length}, site=${got.length})`)
}

// ---- 3. shared assets byte-equal ----------------------------------------------
let assetsChecked = 0
for (const asset of ["out.css", "shadless.js"]) {
  const d = join("dist", asset), s = join(SITE, asset)
  if (!existsSync(d)) continue
  assetsChecked++
  if (!existsSync(s)) { problem("asset-missing", asset, "asset in dist/ but not docs/site/"); continue }
  if (readFileSync(d, "utf8") !== readFileSync(s, "utf8")) problem("asset-drift", asset, "docs/site copy differs from dist/")
}
let glueChecked = 0
for (const f of readdirSync("dist/js")) {
  glueChecked++
  const s = join(SITE, "js", f)
  if (!existsSync(s)) { problem("asset-missing", `js/${f}`, "behavior file in dist/ but not docs/site/"); continue }
  if (readFileSync(join("dist/js", f), "utf8") !== readFileSync(s, "utf8")) problem("asset-drift", `js/${f}`, "docs/site copy differs from dist/js/")
}

// ---- 4. skin residue: shipped HTML must carry zero non-allowlist cn-* ---------
// Source-resolve (tools/resolve-skins.mjs) expands cn-* at the source;
// the only survivors upstream keeps are the transform ALLOWLIST (marker
// classes resolved at CLI-install time; the live site CSS defines only
// .cn-font-heading among them). Any other cn-* in shipped HTML is a
// hand-authored leak outside the resolver.
{
  const { SKIN_ALLOWLIST } = await import("../src/emitter/skin.mjs")
  const CN = /\bcn-[a-z0-9-]+/g
  const trees = ["dist/components", "docs/demos"]
  for (const tree of trees) {
    for (const f of readdirSync(tree).filter((x) => x.endsWith(".html"))) {
      const bad = [...new Set(readFileSync(join(tree, f), "utf8").match(CN) ?? [])]
        .filter((c) => !SKIN_ALLOWLIST.has(c))
      if (bad.length) problem("cn-residue", `${tree}/${f}`, bad.join(" "))
    }
  }
}

// ---- install-import reality (2026-08-27: the no-stylesheet bug) ----------------
// Every `@import "shadless…"` the site teaches must resolve to a REAL file
// through the package's exports semantics (".": core, "./*": dist/css/*,
// exact keys). Born from: aspect-ratio/collapsible/scroll-fade/shimmer have
// no dist/css file, yet their install steps told users to import one.
{
  const pkg = JSON.parse(readFileSync("package.json", "utf8"))
  const resolveImport = (spec) => {
    const sub = spec.replace(/^shadless/, "")
    if (sub === "" || sub === "/") return pkg.exports["."]
    const exact = pkg.exports[sub]
    if (exact) return exact
    const m = sub.match(/^\/(.*)$/)
    if (m) return `dist/css/${m[1]}`
    return null
  }
  const seen = new Set()
  for (const f of readdirSync(SITE).filter((x) => x.endsWith(".html"))) {
    const html = readFileSync(join(SITE, f), "utf8")
    // the pages carry HTML-ENTITY quotes inside the highlighted fences —
    // match both raw and &quot; forms (a plain-quote-only regex matched
    // nothing and passed vacuously; caught by the negative test)
    for (const spec of [...new Set(html.match(/@import\s+(?:"|&quot;)shadless[^"&]*/g) ?? [])]) {
      const key = spec.replace(/@import\s+|"|&quot;/g, "")
      if (seen.has(key)) continue
      seen.add(key)
      const target = resolveImport(key)
      if (!target) { problem("install-import", f, `${key}: no exports rule resolves it`); continue }
      if (!existsSync(target)) problem("install-import", f, `${key} → ${target} does not exist on disk`)
    }
  }
}

// ---- React-import retirement detector ------------------------------------------
// No built page may teach React imports from @/components/ui — the usage
// sections were replaced and the import fences dropped (2026-08-27). A new
// upstream mdx shape the predicate misses lands here, loudly.
{
  for (const f of readdirSync(SITE).filter((x) => x.endsWith(".html"))) {
    if (readFileSync(join(SITE, f), "utf8").includes("@/components/ui"))
      problem("react-import", f, "built page still teaches a React import")
  }
}

// ---- report --------------------------------------------------------------------
const byKind = {}
for (const p of problems) (byKind[p.kind] ??= []).push(p)
for (const [kind, list] of Object.entries(byKind)) {
  console.error(`FAIL  ${kind} (${list.length}):`)
  for (const p of list.slice(0, 10)) console.error(`  - ${p.file}: ${p.detail}`)
  if (list.length > 10) console.error(`  … +${list.length - 10} more`)
}
console.log(`docs consistency: ${distChecked} dist demos, ${authoredChecked} authored demos, ${assetsChecked} assets + ${glueChecked} behavior files compared — mismatches: ${problems.length}`)
if (problems.length) {
  console.error("FAIL  docs consistency (site tree is not the injectSiteSkin image of its sources)")
  process.exit(1)
}
console.log("PASS  docs consistency (dist/authored/asset trees mirror byte-exactly into docs/site)")
