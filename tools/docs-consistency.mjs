#!/usr/bin/env node
// docs-consistency.mjs — the checks on the built site that the step writing it
// cannot answer for itself.
//
// It used to open with three byte-exactness invariants: docs/site/components/*
// must be the injectSiteSkin image of dist/components/*, the same for the
// authored demos, and the shared assets (out.css, shadless.js, js/*) must be
// byte-identical dist↔site. Those were written when docs/site was a COMMITTED
// tree that could go stale against a dist rebuild. It is a build output now
// (gitignored, written by docs-build earlier in this same run), so all three
// re-read what injectSiteSkin produced seconds earlier and cannot fail.
// Deleted 2026-08-31 along with the mutation that proved §1
// (docs-consistency-site-drift).
//
// What remains is not tautological — each reads a tree against a rule that
// lives somewhere else:
//   - skin residue: shipped HTML carries zero non-allowlist cn-* classes
//     (the rule is src/emitter/skin.mjs's ALLOWLIST)
//   - install-import reality: every `@import "shadless…"` the site TEACHES
//     must resolve through package.json's exports to a file on disk
//   - React-import retirement: no built page may teach `@/components/ui`
import { readFileSync, existsSync, readdirSync } from "node:fs"
import { join } from "node:path"

// the built pages are markdown now (VitePress renders them); the two checks
// below read what the pages TEACH, which is the same text either way
const PAGE_DIRS = ["docs/components", "docs/guides"]
const problems = []
const problem = (kind, file, detail) => problems.push({ kind, file, detail })

if (!PAGE_DIRS.every(existsSync)) {
  console.error("FAIL  docs-consistency: the markdown pages are not built — run the docs chain first (make docs)")
  process.exit(1)
}
const sitePages = PAGE_DIRS.flatMap((d) => readdirSync(d).filter((f) => f.endsWith(".md")).map((f) => join(d, f)))

// ---- 1. skin residue: shipped HTML must carry zero non-allowlist cn-* ---------
// Source-resolve (tools/resolve-skins.mjs) expands cn-* at the source;
// the only survivors upstream keeps are the transform ALLOWLIST (marker
// classes resolved at CLI-install time; the live site CSS defines only
// .cn-font-heading among them). Any other cn-* in shipped HTML is a
// hand-authored leak outside the resolver.
let skinScanned = 0
{
  const { SKIN_ALLOWLIST } = await import("../src/emitter/skin.mjs")
  const CN = /\bcn-[a-z0-9-]+/g
  const trees = ["dist/components", "docs/demos"]
  for (const tree of trees) {
    for (const f of readdirSync(tree).filter((x) => x.endsWith(".html"))) {
      skinScanned++
      const bad = [...new Set(readFileSync(join(tree, f), "utf8").match(CN) ?? [])]
        .filter((c) => !SKIN_ALLOWLIST.has(c))
      if (bad.length) problem("cn-residue", `${tree}/${f}`, bad.join(" "))
    }
  }
}

// ---- 2. install-import reality (2026-08-27: the no-stylesheet bug) ------------
// Every `@import "shadless…"` the site teaches must resolve to a REAL file
// through the package's exports semantics (".": core, "./*": dist/css/*,
// exact keys). Born from: aspect-ratio/collapsible/scroll-fade/shimmer have
// no dist/css file, yet their install steps told users to import one.
let importsChecked = 0
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
  for (const f of sitePages) {
    const html = readFileSync(f, "utf8")
    // markdown fences carry plain quotes; the entity form is kept because the
    // check outlived one rendering pipeline already and a plain-quote-only
    // regex once passed vacuously against entity-escaped pages
    for (const spec of [...new Set(html.match(/@import\s+(?:"|&quot;)shadless[^"&]*/g) ?? [])]) {
      const key = spec.replace(/@import\s+|"|&quot;/g, "")
      if (seen.has(key)) continue
      seen.add(key)
      importsChecked++
      const target = resolveImport(key)
      if (!target) { problem("install-import", f, `${key}: no exports rule resolves it`); continue }
      if (!existsSync(target)) problem("install-import", f, `${key} → ${target} does not exist on disk`)
    }
  }
}

// ---- 3. React-import retirement detector --------------------------------------
// No built page may teach React imports from @/components/ui — the usage
// sections were replaced and the import fences dropped (2026-08-27). A new
// upstream mdx shape the predicate misses lands here, loudly.
{
  for (const f of sitePages) {
    if (readFileSync(f, "utf8").includes("@/components/ui"))
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
console.log(`docs consistency: ${skinScanned} shipped pages scanned for skin residue, ${importsChecked} taught @imports resolved, ${sitePages.length} built pages checked for React imports — problems: ${problems.length}`)
if (problems.length) {
  console.error("FAIL  docs consistency")
  process.exit(1)
}
console.log("PASS  docs consistency (no skin residue, every taught @import resolves, no page teaches a React import)")
