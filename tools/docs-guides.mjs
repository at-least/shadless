#!/usr/bin/env node
// docs-guides.mjs — FT4: guides keep-list (decision record), content-map
// generator, and the shared /docs/ route resolver used by BOTH the build
// (tools/docs-build.mjs rewrites/greys links) and the link checker
// (tools/docs-links.mjs) — one table, no drift between the two.
//
// Keep-list decision (2026-08-22, recorded in docs/content-map.json):
//   adapted : installation (upstream index is CLI/create-only → authored
//             vanilla rewrite docs/content/installation.mdx), dark-mode
//             (upstream index is framework cards → authored CSS-variables
//             rewrite docs/content/dark-mode.mdx), utils/shimmer +
//             utils/scroll-fade (mirrored; Installation section replaced —
//             the utilities ship precompiled inside dist/out.css)
//   mirror  : rtl (load-bearing: 56 radix pages link /docs/rtl), helpers/
//             ai-sdk + helpers/tanstack-ai (@shadcn/helpers reference; kept
//             per keep-list, previews are base-style → unavailable)
//   pruned  : forms / react / registry (React-only), changelog, (root), and
//             every framework sub-page (installation/*, dark-mode/*, rtl/*) —
//             links to pruned targets render as unavailable-link spans
//
// Guide ComponentPreview names are enumerated here with the same
// fence-stripped tag-scoped scan as docs-catalog.mjs (that tool stays
// radix-only by design; FT4 guide enumeration lives here, per PLAN FT4).
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs'
import { basename, join } from 'node:path'

const UP = '.upstream/shadcn-ui/apps/v4/content/docs'

export const GUIDES = [
  {
    slug: 'introduction',
    route: '/docs/introduction',
    title: 'Introduction',
    source: 'docs/content/introduction.mdx',
    disposition: 'adapted',
    notes: 'FT8: vanilla rewrite of upstream (root)/index.mdx; pure prose + 1 Accordion FAQ — was wrongly pruned as "framework content" but contains no React/CLI specifics',
    installSection: false,
    pinned: true, // top of sidebar, above the components list
  },
  {
    slug: 'installation',
    route: '/docs/installation',
    title: 'Installation',
    source: 'docs/content/installation.mdx',
    disposition: 'adapted',
    notes: 'authored vanilla rewrite; replaces upstream installation/index.mdx (CLI/create/framework cards — React toolchain) and installation/manual.mdx (Tailwind+aliases+components.json setup); copy-files story over dist/ artifacts',
    installSection: false,
  },
  {
    slug: 'dark-mode',
    route: '/docs/dark-mode',
    title: 'Dark Mode',
    source: 'docs/content/dark-mode.mdx',
    disposition: 'adapted',
    notes: 'authored rewrite; upstream dark-mode/index.mdx is framework cards only (next/vite/astro/remix/tanstack-start pruned); .dark theme variables ship precompiled in dist/out.css; mode-toggle preview to-author (FT7)',
    installSection: false,
  },
  {
    slug: 'rtl',
    route: '/docs/rtl',
    title: 'RTL',
    source: `${UP}/rtl/index.mdx`,
    disposition: 'mirror',
    notes: 'load-bearing (56 radix pages link /docs/rtl); framework sub-pages (rtl/next|vite|start) pruned → their LinkedCard links greyed; card-rtl preview to-author (FT7); shadcn-CLI migrate section rewritten (rtlMigrate)',
    installSection: false,
    rtlMigrate: true,
  },
  {
    slug: 'shimmer',
    route: '/docs/utils/shimmer',
    title: 'shimmer',
    source: `${UP}/utils/shimmer.mdx`,
    disposition: 'adapted',
    notes: 'mirrored; Installation section replaced (utilities ship precompiled in dist/out.css, no npm install); all 9 previews base-style (base-rhea) → unavailable',
    installSection: true,
    util: 'shimmer',
  },
  {
    slug: 'scroll-fade',
    route: '/docs/utils/scroll-fade',
    title: 'scroll-fade',
    source: `${UP}/utils/scroll-fade.mdx`,
    disposition: 'adapted',
    notes: 'mirrored; Installation section replaced (utilities ship precompiled in dist/out.css); all 7 previews base-style (6 base-rhea + 1 base-nova) → unavailable',
    installSection: true,
    util: 'scroll-fade',
  },
  {
    slug: 'ai-sdk',
    route: '/docs/helpers/ai-sdk',
    title: 'AI SDK',
    source: `${UP}/helpers/ai-sdk.mdx`,
    disposition: 'mirror',
    notes: 'kept per keep-list; @shadcn/helpers/ai-sdk is a React useChat package — mirrored as reference (fences stay verbatim, same policy as radix pages); ai-sdk-helper-demo base-style → unavailable',
    installSection: false,
  },
  {
    slug: 'tanstack-ai',
    route: '/docs/helpers/tanstack-ai',
    title: 'TanStack AI',
    source: `${UP}/helpers/tanstack-ai.mdx`,
    disposition: 'mirror',
    notes: 'kept per keep-list; @shadcn/helpers/tanstack-ai is a React package — mirrored as reference; tanstack-ai-helper-demo base-style → unavailable',
    installSection: false,
  },
  {
    slug: 'typography',
    route: '/docs/typography',
    title: 'Typography',
    source: 'docs/content/typography.mdx',
    disposition: 'adapted',
    notes: 'FT8: vanilla rewrite — upstream typography.mdx demos a <Typography> component shadless does not (and should not) ship; this guide maps the same typographic roles to plain Tailwind utilities already in dist/out.css',
    installSection: false,
  },
]

export const PRUNED = {
  forms: { source: `${UP}/forms/`, reason: 'React-only (react-hook-form / tanstack-form / formisch guides); field.mdx links ×4 → greyed spans' },
  react: { source: `${UP}/react/`, reason: 'React-only component recipes (message-scroller, questionnaire); radix links ×6 → greyed spans' },
  registry: { source: `${UP}/registry/`, reason: 'shadcn CLI registry system (json schema, MCP, namespaces) — no vanilla equivalent' },
  changelog: { source: `${UP}/changelog/`, reason: 'shadcn release notes — not shadless content' },
  '(root)': { source: `${UP}/(root)/`, reason: 'shadcn-site root pages (theming, cli, components.json…) — React/CLI specific' },
  'framework sub-pages': { source: `${UP}/{installation,dark-mode,rtl}/* (non-index)`, reason: 'per-React-framework setup guides (next/vite/astro/remix/tanstack/laravel/gatsby…) — installation/dark-mode/rtl index pages kept instead' },
}

// ---- shared /docs/ route resolver ------------------------------------------------
// members: Set of component page slugs. Returns {file, frag} for routable
// targets, {grey:true} for pruned/unknown. Upstream variant routes
// (/docs/components/{base,aria}/<name>) fall through to grey: the
// base/aria mirror is retired (2026-08-26) and those pages don't exist.
export function resolveDocsRoute(href, members) {
  if (!href.startsWith('/') || href.startsWith('//')) return null
  const [path, frag] = href.split('#')
  const comp = /^\/docs\/components\/(?:radix\/)?([a-z0-9-]+)$/.exec(path)
  if (comp) return members.has(comp[1]) ? { file: `${comp[1]}.html`, frag } : { grey: true }
  const guide = GUIDES.find((g) => g.route === path)
  if (guide) return { file: `${guide.slug}.html`, frag }
  return { grey: true } // pruned guides, /create, anything else
}

// ---- guide ComponentPreview enumeration (same scan discipline as the catalog) ----
const stripFences = (text) => text.replace(/```[\s\S]*?```/g, (m) => m.replace(/[^\n]/g, ' '))

export function scanGuidePreviews(catalog) {
  const catalogPreviews = new Map(catalog.previews.map((p) => [p.name, p.status]))
  const out = {}
  for (const g of GUIDES) {
    const text = stripFences(readFileSync(g.source, 'utf8'))
    for (const m of text.matchAll(/<ComponentPreview\b([^>]*)>/g)) {
      const attrs = m[1]
      // (^|\s) anchor: a bare \b would let data-name= match name=
      const attr = (name) => (attrs.match(new RegExp(`(?:^|\\s)${name}="([^"]*)"`)) || [])[1]
      const name = attr('name')
      if (!name) continue
      if (out[name]) { out[name].hostPages.push(g.slug); continue }
      const styleName = attr('styleName') || null
      let disposition, reason
      if (catalogPreviews.has(name)) {
        disposition = catalogPreviews.get(name) // e.g. card-rtl → to-author (FT7)
        reason = 'already cataloged in the radix set'
      } else if (styleName && styleName.startsWith('base-')) {
        disposition = 'unavailable'
        reason = `base-line demo (${styleName}) — shadless implements the radix line only`
      } else {
        disposition = 'to-author'
        reason = 'authored in an FT7 wave'
      }
      out[name] = { hostPages: [g.slug], styleName, disposition, reason }
    }
  }
  return out
}

// ---- content-map ------------------------------------------------------------------
// docs/content-map.json covers EVERY mirrored page (49 components + 9 guides
// + generated index) with a disposition, plus the pruned record and the guide
// preview table.
export function writeContentMap(componentPages) {
  const catalog = JSON.parse(readFileSync('docs/catalog.json', 'utf8'))
  const pages = {}
  for (const c of componentPages) {
    pages[c.name] = {
      source: c.source,
      disposition: 'adapted',
      notes: 'radix mirror; installation Manual tab rewritten to the vanilla copy-files path (build-time transform over dist/ artifacts)',
    }
  }
  for (const g of GUIDES) pages[g.slug] = { source: g.source, disposition: g.disposition, notes: g.notes }
  pages.index = { source: '(generated)', disposition: 'generated', notes: 'components + guides index page' }

  const contentMap = {
    version: 1,
    generatedBy: 'tools/docs-guides.mjs (FT4)',
    pages,
    pruned: PRUNED,
    guidePreviews: scanGuidePreviews(catalog),
  }
  mkdirSync('docs', { recursive: true })
  writeFileSync('docs/content-map.json', JSON.stringify(contentMap, null, 2) + '\n')
  return contentMap
}

// ---- CLI ---------------------------------------------------------------------------
if (process.argv[1] && basename(process.argv[1]) === 'docs-guides.mjs') {
  const catalog = JSON.parse(readFileSync('docs/catalog.json', 'utf8'))
  // Same radix-only page list the build uses (docs-build.mjs) — the CLI
  // previously rebuilt it radix-only and rewrote docs/content-map.json
  // with 98 fewer pages, silently degrading the docs-build cross-checks
  // until the next full build
  const componentPages = catalog.sources
    .filter((s) => s.status === 'existing-dist')
    .filter((s) => existsSync(join(`${UP}/components/radix`, `${s.name}.mdx`)))
    .map((s) => ({
      name: s.name,
      source: join(`${UP}/components/radix`, `${s.name}.mdx`),
    }))
    .sort((a, b) => a.name.localeCompare(b.name))
  const map = writeContentMap(componentPages)
  const by = (d) => Object.values(map.pages).filter((p) => p.disposition === d).length
  const guidesBy = (d) => GUIDES.filter((g) => g.disposition === d).length
  const nUnavail = Object.values(map.guidePreviews).filter((p) => p.disposition === 'unavailable').length
  const nToAuthor = Object.values(map.guidePreviews).filter((p) => p.disposition === 'to-author').length
  console.log(`guides keep-list: ${GUIDES.length} kept (${guidesBy('mirror')} mirror, ${guidesBy('adapted')} adapted), ${Object.keys(PRUNED).length} pruned groups`)
  console.log(`guide previews: ${Object.keys(map.guidePreviews).length} names (${nUnavail} unavailable base-style, ${nToAuthor} to-author)`)
  console.log(`content-map: docs/content-map.json (${Object.keys(map.pages).length} pages = ${componentPages.length} adapted component variants + ${guidesBy('adapted')} adapted guides + ${guidesBy('mirror')} mirrored guides + ${by('generated')} generated index)`)
}
