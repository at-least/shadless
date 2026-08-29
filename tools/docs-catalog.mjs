#!/usr/bin/env node
// docs-catalog.mjs — FT1: enumerate every ComponentPreview + ComponentSource
// name referenced by the docs mirror set and write docs/catalog.json.
// Extraction is tag-scoped (fence-stripped, multiline attrs, any attribute
// order) — line-based tooling over-counts (gate v2 lesson: sed on rg -U output
// emits one artifact line per physical line of a multiline match).
//
// Status rules (recorded in PLAN §Wave F):
//   preview "X-demo" + dist/components/X.html exists -> existing-dist (primary
//   demos reuse dist/, per scope decision v2); every other name -> to-author
//   (docs/demos/<name>.html, authored in FT7 waves).
//   ComponentSource name=X -> dist/components/X.html (existing-dist | no-dist).
//   Previews whose component is tombstoned (no implementation in shadless)
//   get status="tombstoned" instead of "to-author" — they can never be
//   authored. Mirror of GREY_COMPONENTS in tools/docs-build.mjs; lists must
//   stay in sync (cross-checked at every build).
import { readFileSync, writeFileSync, readdirSync, existsSync, mkdirSync, rmSync } from "node:fs"

const RADIX_DIR = ".upstream/shadcn-ui/apps/v4/content/docs/components/radix"
const DIST_DIR = "dist/components"
const OUT = "docs/catalog.json"
const pin = JSON.parse(readFileSync("src/registry/pin.json", "utf8"))

// fence-stripped copy: code fences are prose, not rendered tags (replace with
// spaces to keep offsets/line numbers stable)
const stripFences = (text) => text.replace(/```[\s\S]*?```/g, (m) => m.replace(/[^\n]/g, " "))

const scanSet = (key, dir) => {
  const previews = [] // per-tag records
  const sources = []
  const flags = []
  let previewTags = 0
  let multilinePreviewTags = 0
  const files = readdirSync(dir).filter((f) => f.endsWith(".mdx"))
  for (const f of files) {
    const component = f.replace(/\.mdx$/, "")
    const text = stripFences(readFileSync(`${dir}/${f}`, "utf8"))
    for (const m of text.matchAll(/<(ComponentPreview|ComponentSource)\b([^>]*)>/g)) {
      const tag = m[1]
      const attrs = m[2]
      // (^|\s) anchor: a bare \b would let data-name= match name=
      const attr = (name) => (attrs.match(new RegExp(`(?:^|\\s)${name}="([^"]*)"`)) || [])[1]
      const line = text.slice(0, m.index).split("\n").length
      if (tag === "ComponentPreview") {
        previewTags++
        if (attrs.includes("\n")) multilinePreviewTags++
        const name = attr("name")
        if (!name) { flags.push({ file: f, kind: tag, reason: "no name= attr" }); continue }
        previews.push({ name, component, line, styleName: attr("styleName") || null,
          description: attr("description") || null })
      } else {
        const name = attr("name")
        if (!name) { flags.push({ file: f, kind: tag, reason: "no name= attr",
          src: attr("src") || null }); continue }
        sources.push({ name, component, line })
      }
    }
  }
  return { key, dir, files: files.length, previewTags, multilinePreviewTags, previews, sources, flags }
}

const distFiles = new Set(readdirSync(DIST_DIR).map((f) => f.replace(/\.html$/, "")))
const distPath = (name) => (distFiles.has(name) ? `dist/components/${name}.html` : null)
// FT8: guide previews (mode-toggle, card-rtl when mirrored to guides) may
// have a direct dist match without the "-demo" suffix — accept that too.
// primaryMatch still records the radix-style host page for the "-demo" case.
const primaryMatch = (name) => /^([a-z0-9-]+)-demo$/.exec(name)

// tombstoned components: preview names with these component prefixes cannot
// be authored (no shadless implementation exists). Match by longest prefix
// first so 'data-table' beats 'data', 'date-picker' beats 'date', etc.
const TOMBSTONE_PREFIXES = [
  'calendar', 'chart', 'combobox', 'command', 'data-table', 'date-picker',
  'drawer', 'form', 'input-otp', 'menubar', 'navigation-menu', 'questionnaire',
  'resizable', 'sidebar', 'sonner', 'toast', 'typography',
]
const isTombstoneName = (name) => {
  for (const p of TOMBSTONE_PREFIXES) {
    if (name === p || name.startsWith(p + '-')) return true
  }
  return false
}

// name -> merged record (a name may be referenced from several pages; FT7
// authors it once). hostPages keeps every referencing component. Used both
// per-set (honest per-set counts) and across sets (guide preview that also
// appears in radix collapses to one entry).
const dedupe = (records) => {
  const byName = new Map()
  for (const r of records) {
    if (!byName.has(r.name)) byName.set(r.name, { ...r, hostPages: [] })
    const e = byName.get(r.name)
    if (!e.hostPages.includes(r.component)) e.hostPages.push(r.component)
    // keep the defining page's metadata (first host wins)
  }
  return [...byName.values()]
}

// FT8: scan guide sources too — mode-toggle, card-rtl, etc. live on guide
// pages, not in radix subtree. Same tag-scope, same dedupe, same dist-match.
// Guide set uses the guide slug as the "component" (hostPages entry).
const GUIDE_SOURCES = [
  { slug: 'installation', path: 'docs/content/installation.mdx' },
  { slug: 'dark-mode',    path: 'docs/content/dark-mode.mdx' },
  { slug: 'rtl',          path: '.upstream/shadcn-ui/apps/v4/content/docs/rtl/index.mdx' },
  { slug: 'shimmer',      path: '.upstream/shadcn-ui/apps/v4/content/docs/utils/shimmer.mdx' },
  { slug: 'scroll-fade',  path: '.upstream/shadcn-ui/apps/v4/content/docs/utils/scroll-fade.mdx' },
  { slug: 'ai-sdk',       path: '.upstream/shadcn-ui/apps/v4/content/docs/helpers/ai-sdk.mdx' },
  { slug: 'tanstack-ai',  path: '.upstream/shadcn-ui/apps/v4/content/docs/helpers/tanstack-ai.mdx' },
]
const scanGuides = () => {
  const previews = [] // per-tag records (uses guide slug as hostPages key)
  const flags = []
  let previewTags = 0
  let multilinePreviewTags = 0
  let files = 0
  for (const g of GUIDE_SOURCES) {
    if (!existsSync(g.path)) continue
    files++
    const text = stripFences(readFileSync(g.path, 'utf8'))
    for (const m of text.matchAll(/<ComponentPreview\b([^>]*)>/g)) {
      const attrs = m[1]
      previewTags++
      if (attrs.includes('\n')) multilinePreviewTags++
      // (^|\s) anchor: a bare \b would let data-name="x" match name= (same
      // hardening scanSet/scanGuidePreviews already carry)
      const attr = (n) => (attrs.match(new RegExp(`(?:^|\\s)${n}="([^"]*)"`)) || [])[1]
      const name = attr('name')
      if (!name) { flags.push({ file: g.slug, kind: 'ComponentPreview', reason: 'no name= attr' }); continue }
      previews.push({
        name, component: g.slug, line: text.slice(0, m.index).split('\n').length,
        styleName: attr('styleName'),
        description: attr('description'),
      })
    }
  }
  return { key: 'guides', dir: 'docs/content/ + UP/...', files, previewTags, multilinePreviewTags, previews, sources: [], flags }
}

const sets = [scanSet('components/radix', RADIX_DIR), scanGuides()]
const catalog = {
  version: 1,
  generatedFrom: { repo: pin.shadcn_ui.repo, tag: pin.shadcn_ui.tag, commit: pin.shadcn_ui.commit },
  sets: {},
  previews: [],
  sources: [],
  flags: [],
}

let pExisting = 0, pAuthor = 0, pToAuthor = 0, pTomb = 0, unavail = 0
// Authored status derives from the FILE SYSTEM, not the previous catalog
// (Wave H D5): docs/demos/<name>.html existing ⇒ authored. The old
// prev-catalog-only preservation silently reset all 301 authored entries
// to to-author after a test-clean baseline regen — 149 pages rendered
// placeholders and the RTL selector vanished, with every gate green.
const prevAuthored = new Map()
if (existsSync(OUT)) {
  try {
    const prev = JSON.parse(readFileSync(OUT, "utf8"))
    for (const p of prev.previews ?? []) {
      if (p.status === "authored") prevAuthored.set(p.name, true)
    }
  } catch {}
}
// Per-set dedupe (so each set's `uniquePreviewNames` count stays honest)
// and a global dedupe across sets (so card-rtl on radix + guides collapses
// to one catalog entry with both hostPages).
const radixUnique = dedupe(sets[0].previews)
const guidesUnique = dedupe(sets[1].previews)
const uniquePreviews = dedupe(sets.flatMap((s) => s.previews))
for (const p of uniquePreviews) {
  const m = primaryMatch(p.name)
  const d = m ? distPath(m[1]) : null
  const direct = distPath(p.name) // FT8: direct dist match (e.g. mode-toggle, alert-demo)
  // FT8: guide-only preview with a base-* styleName → unavailable (base-line
  // demo; shadless implements radix only). catalog mirrors the disposition
  // that scanGuidePreviews wrote into content-map.json so the two stay in
  // sync — build uses catalog first, content-map only as fallback.
  const isGuideOnly = p.hostPages.every((h) => !existsSync(`${RADIX_DIR}/${h}.mdx`))
  const isBaseStyle = p.styleName && p.styleName.startsWith('base-')
  // Kernel-tier -demo previews keep the dist fixture (template + glue)
  // path: oracle's static DOM has no Portal mount target, so opening
  // popups would fail. The fixture's <template id="d1-portal"> is the
  // mount target the radix kernel runtime needs.
  const tiers = JSON.parse(readFileSync("probes/out/tiers.json", "utf8"))
  // A -demo is kernel iff the underlying component is — strip "-demo"
  // suffix to look up tier (tiers.json keys are components).
  const isKernel = (name) => {
    const m = /^(.+)-demo$/.exec(name); const key = m ? m[1] : name
    return tiers[key]?.tier === "kernel"
  }
  const authoredFile = `docs/demos/${p.name}.html`
  const hasAuthoredFile = existsSync(authoredFile)
  if (hasAuthoredFile && !isKernel(p.name)) {
    p.status = "authored"; p.demoPath = authoredFile; pAuthor++
  } else if (hasAuthoredFile && isKernel(p.name)) {
    // kernel-tier -demo: authored file exists (oracle ran), but we must
    // point the iframe at the dist fixture, not the oracle static DOM
    // (which can't open popups). Strip the oracle artifact so next oracle
    // run sees the dist fixture as canonical.
    if (process.env.DEBUG_CATALOG) console.error(`[catalog] kernel-rm ${p.name} key=${(/^(.+)-demo$/.exec(p.name)||[])[1]}`)
    rmSync(authoredFile)
    const direct = distPath(p.name)
    const d = primaryMatch(p.name) ? distPath(primaryMatch(p.name)[1]) : null
    if (direct) { p.status = "existing-dist"; p.demoPath = direct; pExisting++ }
    else if (d) { p.status = "existing-dist"; p.demoPath = d; pExisting++ }
    else { p.status = "to-author"; p.demoPath = authoredFile; pToAuthor++ }
  } else if (prevAuthored.has(p.name) && !hasAuthoredFile) {
    // inconsistency: previously authored but the file is gone — loud, not
    // silent (a deleted demo must be a decision, not an accident)
    console.error(`FAIL catalog: ${p.name} was authored but ${authoredFile} is missing`)
    process.exit(1)
  } else if (direct) { p.status = "existing-dist"; p.demoPath = direct; pExisting++ }
  else if (d) { p.status = "existing-dist"; p.demoPath = d; pExisting++ }
  else if (isTombstoneName(p.name)) { p.status = "tombstoned"; p.demoPath = null; pTomb++ }
  else if (isGuideOnly && isBaseStyle) { p.status = "unavailable"; p.demoPath = null; unavail++ }
  else { p.status = "to-author"; p.demoPath = authoredFile; pToAuthor++ }
  delete p.component; delete p.line
  catalog.previews.push(p)
}
for (const s of sets) {
  let sExisting = 0, sNo = 0
  for (const src of dedupe(s.sources)) {
    const d = distPath(src.name)
    if (d) { src.status = "existing-dist"; src.demoPath = d; sExisting++ }
    else { src.status = "no-dist"; src.demoPath = null; sNo++ }
    delete src.component; delete src.line
    catalog.sources.push(src)
  }
  catalog.flags.push(...s.flags)
  catalog.sets[s.key] = {
    dir: s.dir, mdxFiles: s.files, previewTags: s.previewTags,
    multilinePreviewTags: s.multilinePreviewTags,
    uniquePreviewNames: s.key === "components/radix" ? radixUnique.length : guidesUnique.length,
    sourceTags: s.sources.length + s.flags.filter((x) => x.kind === "ComponentSource").length,
    namedSourceTags: s.sources.length,
  }
  console.log(`${s.key}: ${s.files} mdx, ${s.previewTags} preview tags (${s.multilinePreviewTags} multiline), ` +
    `${s.key === "components/radix" ? radixUnique.length : guidesUnique.length} unique preview names, ` +
    `${s.sources.length + s.flags.length - s.flags.filter((x) => x.kind === "ComponentPreview").length} source tags ` +
    `(${s.sources.length} named, ${s.flags.filter((x) => x.kind === "ComponentSource").length} flagged)`)
}
console.log(`total previews: ${pExisting} existing-dist, ${pAuthor} authored, ${pToAuthor} to-author, ${pTomb} tombstoned` +
  (unavail > 0 ? ` + ${unavail} unavailable (base-style)` : ``))

// FT8: mark demos whose upstream mdx file uses React hooks as
// "informational" — they were authored from React example-registry code in
// FT7 batches, so their interactive semantics aren't validated against the
// radix oracle the way primary demos are. PLAN FT4 evidence: 73/456 radix
// demos. Heuristic: every preview in a file whose mdx uses React hooks is
// marked informational (cheaper and more conservative than per-preview
// scanning). Reproduces the documented 73 count within ~1.
{
  const hookRe = /\b(useState|useEffect|useRef|useContext|useMemo|useCallback|useReducer|useLayoutEffect|useImperativeHandle|useId|useTransition|useDeferredValue|useSyncExternalStore|useInsertionEffect)\b/
  const fileHasHooks = new Set()
  for (const f of readdirSync(".upstream/shadcn-ui/apps/v4/content/docs/components/radix").filter((x) => x.endsWith(".mdx"))) {
    const text = stripFences(readFileSync(`.upstream/shadcn-ui/apps/v4/content/docs/components/radix/${f}`, "utf8"))
    if (hookRe.test(text)) fileHasHooks.add(f.replace(/\.mdx$/, ""))
  }
  // Map every preview to its host pages. If any host page is in fileHasHooks,
  // mark it informational.
  let marked = 0
  for (const p of catalog.previews) {
    if ((p.hostPages || []).some((h) => fileHasHooks.has(h))) {
      p.quality = "informational"
      marked++
    }
  }
  console.log(`quality: ${marked} informational (host radix page uses React hooks; not contract-tested)`)
}
// single write at the end (catalog + quality pass both complete by now)
mkdirSync("docs", { recursive: true })
writeFileSync(OUT, JSON.stringify(catalog, null, 2) + "\n")

console.log(`radix unique preview names: ${catalog.sets["components/radix"].uniquePreviewNames}`)
console.log(`catalog: ${OUT} (${catalog.previews.length} previews, ${catalog.sources.length} sources, ${catalog.flags.length} flags)`)
