// FT2 docs build: radix-subtree mirror → static site under docs/site/.
// Mirror rule: a component gets an individual page iff docs/catalog.json
// sources[] has {name, status:"existing-dist"} (49 components); tombstones/
// greyed get no page in FT2 (grey index entries land in FT5). Pipeline per
// page: strip ESM imports (fence-aware, gate hit 4) → MDX evaluate with the
// vanilla jsx shim + component map + line-wrapping rehype plugin → serialize to HTML
// → heading ids + TOC → page template with meta.json sidebar. The tool
// prints the mirror-set size it uses; acceptance requires built == mirror.
import { readFileSync, writeFileSync, mkdirSync, rmSync, existsSync, readdirSync, copyFileSync } from 'node:fs'
import { join } from 'node:path'
import { evaluate } from '@mdx-js/mdx'
import { buildSync } from 'esbuild'
import remarkFrontmatter from 'remark-frontmatter'
import remarkGfm from 'remark-gfm'
import { Fragment, jsx, jsxs, serialize, el } from '../src/docs/jsx.mjs'
import { buildComponentMap } from '../src/docs/components.mjs'
import { parseFrontmatter, stripImports } from '../src/docs/frontmatter.mjs'
import { createHighlightPlugin } from '../src/docs/highlight.mjs'
import { SITE_CSS, SITE_JS, FONTS_CSS, themePrePaintInline } from '../src/docs/assets.mjs'
import { GUIDES, resolveDocsRoute, writeContentMap } from './docs-guides.mjs'
import { protocolMdx, trivialMdx, apiReferenceMdx } from "./fixture-families.mjs"
import { injectSiteSkin } from '../src/docs/theme-prepaint.mjs'
import { extractDemoScripts } from '../src/docs/demo-scripts.mjs'
import { addHeadingIds, prevNextFor } from './docs-page-lib.mjs'
import { fenceShadow, locateCodeTabsSpans, locateInstallSection, locateRtlMigrateSpan, locateUsageSpan, locateCompositionSpan, applyTextAdjustments, dropReactImportFences, stripImportsFromMixedFences } from '../src/docs/transforms.mjs'

const RADIX_DIR = '.upstream/shadcn-ui/apps/v4/content/docs/components/radix'
const OUT_DIR = 'docs/site'

// Upstream mirrors each component under three URLs (radix/base/aria
// subtrees, one per React primitive library). shadless RETIRED the
// base/aria mirror (2026-08-26): those pages advertise React library
// implementation differences that don't exist in the no-React product —
// our demos are vanilla regardless of which upstream subtree the mdx
// came from. Only the radix subtree is mirrored; docs-fidelity
// asserts the variant tab strip and -base/-aria hrefs stay gone.
const catalog = JSON.parse(readFileSync('docs/catalog.json', 'utf8'))
const meta = JSON.parse(readFileSync(join(RADIX_DIR, 'meta.json'), 'utf8'))

// ---- mirror set: catalog sources with status existing-dist --------------------
// One page per mirrored component, compiled from the radix mdx source.
// mirrorSet is the deduplicated list of component names.
const componentPages = catalog.sources
  .filter((s) => s.status === 'existing-dist')
  .filter((s) => existsSync(join(RADIX_DIR, `${s.name}.mdx`)))
  .map((s) => ({
    name: s.name,
    source: join(RADIX_DIR, `${s.name}.mdx`),
  }))
  .sort((a, b) => a.name.localeCompare(b.name))
const mirrorSet = [...new Set(componentPages.map((p) => p.name))].sort()
console.log(`mirror set: ${mirrorSet.length} components = ${componentPages.length} pages + ${GUIDES.length} guides`)
const mirrorTotal = componentPages.length + GUIDES.length

// FT5: canonical grey list — the 15 radix meta.json entries with NO shadless
// implementation. Source of truth = PLAN Wave D/E + FT6 final report:
//   12 tombstones (Wave D/E: menubar/navigation-menu/combobox/sidebar +
//   calendar/chart/command/drawer/input-otp/resizable/sonner + form —
//   form has no upstream mdx page) and 5 FT6 grey dispositions
//   (tools/five-components.mjs: data-table/date-picker/questionnaire/
//   toast/typography). Cross-checked against docs/catalog.json at every
// build: sources no-dist(10) ⊆ grey(15), and built(51) ∪ grey(15) ==
// meta.json pages(66) exactly — mismatches fail the build.
const GREY_COMPONENTS = [
  // 10 tombstones (Wave D/E externals; form has no mdx page; the medium
  // pair menubar/navigation-menu emitted since — contract-tested glue)
  'calendar', 'chart', 'combobox', 'command', 'drawer', 'form', 'input-otp',
  'resizable', 'sidebar', 'sonner',
  // 5 FT6 grey dispositions (0 implement / 5 grey)
  'data-table', 'date-picker', 'questionnaire', 'toast', 'typography',
]
const greySet = new Set(GREY_COMPONENTS)
{
  const noDist = catalog.sources.filter((s) => s.status === 'no-dist').map((s) => s.name)
  const notSub = noDist.filter((n) => !greySet.has(n))
  const overlap = mirrorSet.filter((n) => greySet.has(n))
  const accounted = new Set([...mirrorSet, ...GREY_COMPONENTS])
  const uncovered = meta.pages.filter((p) => !accounted.has(p))
  const orphan = [...accounted].filter((n) => !meta.pages.includes(n))
  if (notSub.length || overlap.length || uncovered.length || orphan.length || accounted.size !== meta.pages.length) {
    console.error(`FAIL grey-list cross-check: noDist-not-grey=[${notSub}] built∩grey=[${overlap}] meta-uncovered=[${uncovered}] grey-not-in-meta=[${orphan}]`)
    process.exit(1)
  }
}

// FT4: content map (decision record) — covers every mirrored page + index
writeContentMap(componentPages.map((p) => ({ name: p.name, source: p.source })))

// sidebar order follows meta.json pages[] restricted to the mirror set
const built = new Set(mirrorSet)
const sidebarOrder = meta.pages.filter((p) => built.has(p))
if (sidebarOrder.length !== mirrorSet.length) {
  console.error(`FAIL mirror members missing from meta.json: ${mirrorSet.filter((n) => !meta.pages.includes(n)).join(', ')}`)
  process.exit(1)
}

// ---- fence-aware import stripping ---------------------------------------------
// transform only outside ``` fences so mirrored code inside fences survives
function stripImportsOutsideFences(src) {
  return src.split(/(```[\s\S]*?```)/).map((seg, i) => (i % 2 ? seg : stripImports(seg))).join('')
}

// ---- FT4 content transforms -----------------------------------------------------
// Both transforms below operate on the RAW mdx via offset-exact spans
// located in a fence shadow (spaces keep offsets stable): the blocks
// they replace CONTAIN code fences, so the split-on-fences approach
// cannot see them whole. Span location comes from src/docs/transforms.mjs
// (shared with the docs-fidelity gate — one locate, no drift possible).
const replaceSpan = (raw, start, end, replacement) => raw.slice(0, start) + replacement + raw.slice(end)

// Rewrites the upstream install <CodeTabs> block (CLI command tab + manual
// npm/copy-the-tsx tab) into the shadless install steps — unwrapped, no
// tabs: shadless has no CLI, so a Command tab would be a lie.
//
// PRIMARY story: the consumer's own Tailwind v4 build — @import the
// self-contained tokens + this component's css; their content scan picks
// up the inline utilities from the copied markup; only the behavior JS is
// file copying. The no-build alternative (precompiled dist/out.css) is a
// footnote. Artifact list is derived from the REAL dist demo page (source
// of truth — script tags exactly as the demo loads them); the css-import
// path is machine-proven by pipeline/gate_consumer_sim.go.
function installStepsMdx(name) {
  const demo = readFileSync(join('dist/components', `${name}.html`), 'utf8')
  const initAll = /shadless\.initAll\(\)/.test(demo)
  // pre-paint boilerplate is filtered here (D3) — with it included, every
  // demo since FT9 looked like it carried an "inline init script" and the
  // install steps told users to copy one that doesn't exist. Same
  // extractor as the docs JS tab, so the two can no longer disagree.
  const { srcScripts: scripts, inlineScripts } = extractDemoScripts(demo)
  const inlineInit = inlineScripts.length > 0
  // a few components emit ZERO slot rules (aspect-ratio, collapsible,
  // scroll-fade, shimmer — all styling rides inline utilities + core
  // utilities); they have no dist/css/<name>.css, so no import for it
  const hasOwnCss = existsSync(join('dist/css', `${name}.css`))
  const rows = [
    `| \`dist/shadless-core.css\` | theme + animate layer — self-contained, needs only your tailwindcss build |`,
    ...(hasOwnCss
      ? [`| \`dist/css/${name}.css\` | this component's slot styles (\`@apply\` source — your build compiles it) |`]
      : []),
    `| \`dist/components/${name}.html\` | component markup — copy your page's structure from here |`,
  ]
  const loadLines = []
  // the JS surface mirrors the CSS one: one base, one file per component
  if (scripts.includes('shadless.js')) {
    rows.push(`| \`dist/shadless.js\` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (\`shadless.init(root)\` for content added later) |`)
    loadLines.push('<script src="shadless.js"></script>')
  } else if (!scripts.length) {
    rows.push(`| — | no JavaScript: this component is markup + CSS |`)
  }
  for (const s of scripts) {
    if (s === 'shadless.js') continue
    const label = s.startsWith('js/') ? `this component's behavior — registers with the base` : `vendored runtime (${s})`
    rows.push(`| \`dist/${s}\` | ${label} |`)
    loadLines.push(`<script src="${s}"></script>`)
  }
  const inlineNote = !initAll && scripts.length > 0 && inlineInit
    ? ' (including the inline init script at the bottom of the demo page)'
    : ''
  const jsStep = loadLines.length
    ? `

<Step>Load the behavior files in your page:</Step>

\`\`\`html
${loadLines.join('\n')}
\`\`\``
    : ''
  const cssImports = hasOwnCss ? `\n@import "shadless/${name}.css";` : ""
  const noCssNote = hasOwnCss ? "" : `\nThis component has no stylesheet of its own — its styling rides the core theme and utilities in \`shadless\`.\n`
  return `<Steps className="mb-0 pt-2">

<Step>Add shadless${hasOwnCss ? " and this component" : ""} to your Tailwind v4 entry:</Step>

\`\`\`css
@import "shadless";${cssImports}
\`\`\`
${noCssNote}
The files this component needs:

| File | Purpose |
| --- | --- |
${rows.join('\n')}
${jsStep}

<Step>Copy the markup${inlineNote ? ' and init' : ''} from \`dist/components/${name}.html\` into your page and adapt it${inlineInit && !initAll ? inlineNote : ''} — the inline utilities are picked up by your build's content scan.</Step>
${protocolMdx(name)}${trivialMdx(name)}
No Tailwind build? Use the precompiled \`dist/out.css\` (every component) as a single stylesheet instead of the imports above.

</Steps>`
}

// The upstream ## Usage section is React (import + JSX fences) — replaced
// with the shadless story: copy the markup, slots are data-slot attrs, and
// the component's cva axes map JSX props to data attributes (from the IR —
// the same source that drives the emitted pages, so it cannot drift).
function usageMdx(name) {
  let axes = []
  try {
    const ir = JSON.parse(readFileSync(join('src/registry/ir', `${name}.json`), 'utf8'))
    for (const table of Object.values(ir.cva ?? {}))
      axes = [...new Set([...axes, ...Object.keys(table.variants ?? {})])]
  } catch { /* no IR (e.g. direction) — no axes table */ }
  const rows = axes.map((a) => `| \`${a}="outline"\` (JSX prop) | \`data-${a}="outline"\` (markup) |`).join("\n")
  return `## Usage

Copy the markup from \`dist/components/${name}.html\` and adapt it — every slot
is a \`data-slot\` attribute, and open/close state is a \`data-state\` the
runtime drives.${rows ? ` The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
${rows}` : ""}
`
}

// ## Composition: keep ONLY the slot tree, with PascalCase component names
// translated to their data-slot equivalents via the IR (fn → root slot).
// The React composition examples in the section are dropped — the demos
// show real compositions as shipped markup.
function compositionTransform(name, raw) {
  const span = locateCompositionSpan(fenceShadow(raw))
  if (!span) return raw
  const section = raw.slice(span.start, span.end)
  const tree = /```text\n([\s\S]*?)```/.exec(section)?.[1]?.trimEnd()
  let mapped = null
  if (tree) {
    let nameToSlot = {}
    try {
      const ir = JSON.parse(readFileSync(join('src/registry/ir', `${name}.json`), 'utf8'))
      for (const c of ir.components) {
        const slot = c.elements.find((e) => e.slot)?.slot
        if (slot) nameToSlot[c.fn] = slot
      }
    } catch {}
    mapped = tree.replace(/[A-Z][A-Za-z0-9]+/g, (w) => nameToSlot[w] ?? w)
  }
  const body = mapped
    ? `The slot tree — every node is a \`data-slot\` attribute in the shipped markup:\n\n\`\`\`text\n${mapped}\n\`\`\`\n`
    : `See the demos for real compositions — every slot is a \`data-slot\` attribute in the shipped markup.\n`
  return replaceSpan(raw, span.start, span.end, `## Composition\n\n${body}\n`)
}
function componentTransform(name, raw) {
  raw = stripImportsFromMixedFences(dropReactImportFences(raw))
  const spans = locateCodeTabsSpans(fenceShadow(raw))
  if (spans.length !== 1) throw new Error(`install code-tabs: fence-shadowed count ${spans.length}, expected 1`)
  let out = replaceSpan(raw, spans[0].start, spans[0].end, installStepsMdx(name))
  const usage = locateUsageSpan(fenceShadow(out))
  if (usage) out = replaceSpan(out, usage.start, usage.end, usageMdx(name))
  out = compositionTransform(name, out)
  out = apiReferenceTransform(name, out)
  return applyTextAdjustments(`${name}.mdx`, out)
}

// ## API Reference upstream is one line linking the React prop tables. Keep
// the link (it documents the behavior contract the kernel mirrors) and add
// the surface a shadless consumer actually authors against: the data-slot
// vocabulary from the IR and the runtime contract from the same tables that
// drive the fixtures (tools/fixture-families.mjs). No heading, no fence —
// the fidelity gate compares those against upstream.
function apiReferenceTransform(name, raw) {
  const m = /^## API Reference[ \t]*\n/m.exec(raw)
  if (!m) return raw
  let slots = []
  try {
    const ir = JSON.parse(readFileSync(join('src/registry/ir', `${name}.json`), 'utf8'))
    slots = [...new Set(ir.components.flatMap((c) => c.elements.map((e) => e.slot).filter(Boolean)))]
  } catch {}
  const extra = apiReferenceMdx(name, slots)
  if (!extra) return raw
  // insert after the section's first paragraph (the Radix link line), or
  // right after the heading when the section is empty
  const start = m.index + m[0].length
  const rest = raw.slice(start)
  const para = /^\s*(?:[^\n]+\n)+?(?=\n|$)/.exec(rest)
  const at = para ? start + para[0].length : start
  return raw.slice(0, at) + "\n" + extra + raw.slice(at)
}

// ---- FT4: utils guides Installation section (vanilla truth) --------------------
function utilsInstallMdx(util) {
  return `## Installation

In shadless, the \`${util}\` utilities ship precompiled inside \`dist/out.css\` —
no npm install, Tailwind setup, or CSS import is required. Load \`out.css\` and
use the classes directly (see the [Installation](/docs/installation) guide).`
}

// rtl guide: the ## Migrating existing components section is shadcn-CLI
// only (migrate command, DirectionProvider wiring). shadless emits the
// pinned registry as-is — logical utilities included — so RTL needs no
// migration step. No headings in the replacement (headings compare
// two-directionally in the fidelity gate).
function rtlMigrateMdx() {
  return `shadless components ship the pinned registry's classes as-is — the current registry already uses logical (start/end-aware) utilities, so there is no migration step: every component is RTL-ready the moment the page carries \`dir="rtl"\`. To flip an individual icon, give it the \`rtl:rotate-180\` utility class.`
}

function guideTransform(guide, raw) {
  if (guide.rtlMigrate) {
    const span = locateRtlMigrateSpan(fenceShadow(raw))
    if (!span) throw new Error(`rtl migrate section: not found in ${guide.source}`)
    raw = replaceSpan(raw, span.start, span.end, rtlMigrateMdx())
    // the Supported Styles section links the removed anchor — rewrite the
    // link line too (loud failure if upstream rewords it)
    const LINK = "For other styles, see the [Migration Guide](#migrating-existing-components)."
    if (!raw.includes(LINK)) throw new Error("rtl guide: migrate anchor link not found — re-anchor")
    raw = raw.replace(LINK, "For other styles, the shipped utilities are already logical (start/end-aware) — \`dir=\"rtl\"\` is all it takes.")
  }
  if (!guide.installSection) return raw
  const span = locateInstallSection(fenceShadow(raw))
  if (!span) throw new Error('utils Installation section: not found (or no following ## Usage)')
  // end at the '#' of "## Usage" — end+1 would eat the first '#' and
  // glue the heading into the replacement text ("…guide).# Usage")
  return replaceSpan(raw, span.start, span.end, utilsInstallMdx(guide.util) + '\n\n')
}

// ---- compile one mdx page -------------------------------------------------------
const highlightPlugin = createHighlightPlugin()
const components = buildComponentMap()

async function compilePage(page) {
  const raw = readFileSync(page.source, 'utf8')
  const frontmatter = parseFrontmatter(raw)
  // FT4 transforms run on the raw source (they locate spans in a
  // fence-stripped shadow — see below), then fence-aware import stripping
  const source = stripImportsOutsideFences(page.transform ? page.transform(raw) : raw)
  const { default: MDXContent } = await evaluate(source, {
    Fragment, jsx, jsxs,
    useMDXComponents: (incoming = {}) => ({ ...incoming, ...components }),
    remarkPlugins: [remarkFrontmatter, remarkGfm],
    rehypePlugins: [highlightPlugin],
  })
  return { frontmatter, body: serialize(MDXContent({})) }
}

// ---- page template ----------------------------------------------------------------
const esc = (s) =>
  String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')

const sidebarHtml = [
  // FT8: pinned guides (introduction) appear at the very top, above the
  // components list — they're the front door, not buried under 49 links.
  ...GUIDES.filter((g) => g.pinned).map((g) => `      <li><a href="${g.slug}.html">${g.title}</a></li>`),
  ...sidebarOrder.map((n) => `      <li><a href="${n}.html">${n}</a></li>`),
  // FT5: greyed tombstone/FT6 entries — muted, no link (no page exists).
  // FT8: components with a matching guide page get a richer tooltip
  // pointing to the guide (typography → /docs/typography).
  '      <li class="sidebar-group">Not available</li>',
  ...meta.pages.filter((p) => greySet.has(p))
    .map((n) => {
      const guide = GUIDES.find((g) => g.slug === n)
      const title = guide ? `No component in shadless — see ${guide.title} guide` : 'Not available in shadless'
      return `      <li class="sidebar-unavailable" title="${title}">${n}${guide ? ` <a href="${guide.slug}.html" class="greyed-guide-link" title="Open ${guide.title} guide">→</a>` : ''}</li>`
    }),
].join('\n')
const guidesSidebar = [
  '      <li class="sidebar-group">Guides</li>',
  ...GUIDES.filter((g) => !g.pinned).map((g) => `      <li><a href="${g.slug}.html">${g.title}</a></li>`),
].join('\n')

// FT8: theme pre-paint inline script (FOUC avoidance) — emitted by the page
// template into <head>. Imported from assets.mjs (which derives it from
// THEME_PREPAINT_SCRIPT — single source of truth).

// FT8: theme toggle button — ships BOTH sun and moon SVGs; CSS swaps which
// is visible based on the .dark class on <html>.
const themeToggle = `<button type="button" class="theme-toggle" data-theme-toggle aria-label="Toggle theme">
    <svg class="icon-sun" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <circle cx="12" cy="12" r="4"/>
      <path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>
    </svg>
    <svg class="icon-moon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>
    </svg>
  </button>`

// Mobile sidebar drawer: below lg (1024px) the fixed sidebar column is
// hidden and this sticky topbar takes over — hamburger button opens the
// sidebar as an overlay (site.js toggles data-open on #sidebar + backdrop).
// Mirrors ui.shadcn.com's collapsed-docs-sidebar affordance, and
// frees the whole viewport width for the article + preview iframes.
const mobileTopbar = `<header class="mobile-topbar">
    <button type="button" class="sidebar-toggle" data-sidebar-toggle aria-controls="sidebar" aria-expanded="false" aria-label="Open navigation">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 6h16"/><path d="M4 12h16"/><path d="M4 18h16"/></svg>
    </button>
    <a class="brand" href="index.html">shadless docs</a>
  </header>`
const sidebarBackdrop = `<div class="sidebar-backdrop" data-sidebar-backdrop aria-hidden="true"></div>`

function breadcrumbFor(name) {
  // FT8: derive breadcrumb group + label from page identity. Pinned guides
  // get "Getting started"; radix components get "Components"; unpinned
  // guides get "Guides"; index gets nothing. Each crumb is either a link
  // (href set) or the terminal label (href omitted).
  const leaf = (label) => ({ label })
  const link = (href, label) => ({ href, label })
  if (name === 'index') return null
  const pinned = GUIDES.find((g) => g.slug === name && g.pinned)
  if (pinned) return [leaf('Getting started'), leaf(pinned.title)]
  if (GUIDES.some((g) => g.slug === name)) return [link('index.html', 'Home'), leaf(GUIDES.find((g) => g.slug === name).title)]
  if (mirrorSet.includes(name)) return [link('index.html', 'Components'), leaf(name)]
  return null
}

// planned page set — prev/next consults the BUILD PLAN via docs-page-lib
// (disk existsSync against the mid-wipe output dir never saw later pages)
const plannedPages = new Set(componentPages.map((p) => p.name))
const prevNext = (name) => prevNextFor(name, { sidebarOrder, mirrorSet, plannedPages, guides: GUIDES })

function pageHtml({ name, title, description, links, featured, body, toc }) {
  const linkChips = links && Object.keys(links).length
    ? `<p class="links">${Object.entries(links).map(([k, v]) => `<a href="${esc(v)}" rel="noopener">${esc(k)}</a>`).join('')}</p>` : ''
  const tocHtml = toc.length
    ? `      <nav class="toc" aria-label="On this page"><strong>On this page</strong>
      <ul>${toc.map((t) => `<li class="toc-${t.depth}"><a href="#${t.id}">${esc(t.text)}</a></li>`).join('')}</ul></nav>`
    : '      <nav class="toc" aria-label="On this page"></nav>'
  const crumbs = breadcrumbFor(name)
  const breadcrumbHtml = crumbs
    ? `<nav class="breadcrumbs" aria-label="Breadcrumb">${crumbs.map((c, i) => i < crumbs.length - 1
        ? (c.href ? `<a href="${c.href}">${esc(c.label)}</a>` : `<span>${esc(c.label)}</span>`) + `<span class="sep" aria-hidden="true">›</span>`
        : `<span>${esc(c.label)}</span>`).join('')}</nav>`
    : ''
  const pn = prevNext(name)
  // FT8 (reworked for upstream parity): pager mirrors ui.shadcn.com —
  // icon-only prev/next at the header's top-right + pill links labeled
  // with the sibling page names at the BOTTOM of the article. Label is
  // the slug in title case ("alert-dialog" → "Alert Dialog").
  const pnLabel = (slug) => slug.replace(/-/g, ' ').replace(/\b[a-z]/g, (c) => c.toUpperCase())
  const ARROW_LEFT = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>'
  const ARROW_RIGHT = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m12 5 7 7-7 7"/><path d="M5 12h14"/></svg>'
  const docsNavHtml = pn && (pn.prev || pn.next)
    ? `<div class="docs-nav">${
        pn.prev ? `<a href="${esc(pn.prev)}.html" aria-label="Previous page" title="Previous">${ARROW_LEFT}</a>` : ''
      }${
        pn.next ? `<a href="${esc(pn.next)}.html" aria-label="Next page" title="Next">${ARROW_RIGHT}</a>` : ''
      }</div>`
    : ''
  const prevNextHtml = pn && (pn.prev || pn.next)
    ? `<nav class="page-prev-next" aria-label="Page navigation">${
        pn.prev ? `<a class="pn-prev" href="${esc(pn.prev)}.html">${ARROW_LEFT}<span>${esc(pnLabel(pn.prev))}</span></a>` : '<span></span>'
      }${
        pn.next ? `<a class="pn-next" href="${esc(pn.next)}.html"><span>${esc(pnLabel(pn.next))}</span>${ARROW_RIGHT}</a>` : '<span></span>'
      }</nav>`
    : ''
  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>${esc(title)} — shadless docs</title>
<link rel="stylesheet" href="site.css">
<link rel="stylesheet" href="fonts.css">
<script>${themePrePaintInline}</script>
</head>
<body>
${mobileTopbar}
<div class="layout">
  <aside class="sidebar" id="sidebar">
    <div class="brand-row">
      <a class="brand" href="index.html">shadless docs</a>
      ${themeToggle}
    </div>
    <nav aria-label="Components and guides">
      <ul>
${sidebarHtml}
${guidesSidebar}
      </ul>
    </nav>
  </aside>
  <div class="main">
    ${breadcrumbHtml}
    <header class="page-header">
      <div class="header-row">
        <h1>${esc(title)}</h1>
        ${docsNavHtml}
      </div>
${description ? `      <p class="lead">${esc(description)}</p>\n` : ''}${linkChips}
    </header>
    <div class="columns">
      <article data-page="${esc(name)}" class="typeset"${featured ? ' data-featured=""' : ''}>
${body}
      </article>
${tocHtml}
    </div>
${prevNextHtml}
  </div>
</div>
${sidebarBackdrop}
<script src="site.js"></script>
<script src="highlight.js" defer></script>
</body>
</html>
`
}

// FT5 components index: ALL 66 radix meta.json entries in meta order —
// available (mirror set) as links, the 15 grey as muted non-link entries
// carrying data-component + an explicit "not available" marker.
const greyIndexEntry = (n) => `          <li class="index-unavailable" data-component="${n}"><span class="idx-name">${n}</span><span class="idx-note">not available</span></li>`
const indexComponents = meta.pages
  .map((n) => greySet.has(n) ? greyIndexEntry(n) : `          <li><a href="${n}.html">${n}</a></li>`)
  .join('\n')

const indexHtml = `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Components — shadless docs</title>
<link rel="stylesheet" href="site.css">
<link rel="stylesheet" href="fonts.css">
<script>${themePrePaintInline}</script>
</head>
<body>
${mobileTopbar}
<div class="layout">
  <aside class="sidebar" id="sidebar">
    <div class="brand-row">
      <a class="brand" href="index.html">shadless docs</a>
      ${themeToggle}
    </div>
    <nav aria-label="Components and guides">
      <ul>
${sidebarHtml}
${guidesSidebar}
      </ul>
    </nav>
  </aside>
  <div class="main">
    <header class="page-header">
      <h1>Components</h1>
      <p class="lead">${mirrorSet.length} radix components available (linked below) · ${GREY_COMPONENTS.length} not available (greyed — upstream tombstones or out of pipeline scope) · ${GUIDES.length} guides.</p>
    </header>
    <p><strong>New here?</strong> Read the <a href="introduction.html">Introduction</a> to learn what shadless is and why it exists.</p>
    <div class="columns">
      <article data-page="index">
        <ul class="index-list" data-grey-count="${GREY_COMPONENTS.length}">
${indexComponents}
          <li class="index-group">Guides</li>
${GUIDES.map((g) => `          <li><a href="${g.slug}.html">${g.title}</a></li>`).join('\n')}
        </ul>
      </article>
    </div>
  </div>
</div>
${sidebarBackdrop}
<script src="site.js"></script>
<script src="highlight.js" defer></script>
</body>
</html>
`

// ---- build ---------------------------------------------------------------------
// build-rtl's language manifest lives in build/rtl-langs.json (outside
// this output tree), so wiping docs/site never has to preserve it
rmSync(OUT_DIR, { recursive: true, force: true })
mkdirSync(OUT_DIR, { recursive: true })
writeFileSync(join(OUT_DIR, 'site.css'), SITE_CSS)
writeFileSync(join(OUT_DIR, 'site.js'), SITE_JS)
// Client-side code highlighting: one self-contained IIFE (shiki core + the
// two upstream themes + the fence grammars, JS regex engine — no wasm, no
// network). Highlighting is presentation; baking it into the pages was
// 137 MB of token spans. See src/docs/highlight-client.mjs.
buildSync({
  entryPoints: ['src/docs/highlight-client.mjs'],
  bundle: true, format: 'iife', minify: true, target: 'es2020',
  outfile: join(OUT_DIR, 'highlight.js'), logLevel: 'error',
})
// Geist skin (ui.shadcn.com parity — see FONTS_CSS in assets.mjs).
// Vendored binaries live at docs/fonts/ (OFL); serve one copy from
// assets/fonts/ — fonts.css's url()s resolve against its own path, so
// root pages (fonts.css) and demo copies (../fonts.css) share it.
writeFileSync(join(OUT_DIR, 'fonts.css'), FONTS_CSS)
mkdirSync(join(OUT_DIR, 'assets', 'fonts'), { recursive: true })
for (const woff2 of ['Geist-Variable.woff2', 'GeistMono-Variable.woff2'])
  copyFileSync(join('docs', 'fonts', woff2), join(OUT_DIR, 'assets', 'fonts', woff2))
writeFileSync(join(OUT_DIR, 'index.html'), indexHtml)

// FT3: copy the dist demo tree into the site so ComponentPreview iframes
// resolve over http from a SINGLE served tree (docs/site/). Relative layout
// preserved (components/<c>.html refs ../out.css, ../shadless.js, ../js/…).
mkdirSync(join(OUT_DIR, 'components'), { recursive: true })
// FT9 root-cause: every copied demo HTML gets the same theme pre-paint
// script the host docs page uses, so iframes flip theme independently
// instead of relying on the parent to reach into their document. dist/*
// demos already include the script (emitter + demo.mjs inject it); docs/
// demos/* are hand-authored and need it injected at copy time. injectSiteSkin
// is idempotent (string detection), so calling it again on already-injected
// files is a no-op.
let copiedDemos = 0
for (const f of readdirSync('dist/components')) {
  if (!f.endsWith('.html')) continue
  const src = readFileSync(join('dist/components', f), 'utf8')
  writeFileSync(join(OUT_DIR, 'components', f), injectSiteSkin(src))
  copiedDemos++
}
// FT7 authored demos (catalog status "authored", files in docs/demos/) ride
// the same layout — dist-style relative refs keep working once copied.
// Inject pre-paint here too (these are hand-authored, no emitter pass).
let copiedAuthored = 0
for (const p of catalog.previews) {
  if (p.status !== 'authored') continue
  const src = readFileSync(`docs/demos/${p.name}.html`, 'utf8')
  writeFileSync(join(OUT_DIR, 'components', `${p.name}.html`), injectSiteSkin(src))
  copiedAuthored++
}
for (const asset of ['out.css', 'shadless.js'])
  copyFileSync(join('dist', asset), join(OUT_DIR, asset))
mkdirSync(join(OUT_DIR, 'js'), { recursive: true })
let copiedGlue = 0
for (const f of readdirSync('dist/js')) {
  copyFileSync(join('dist/js', f), join(OUT_DIR, 'js', f))
  copiedGlue++
}
console.log(`demo assets copied: ${copiedDemos} component pages, ${copiedAuthored} authored demos, ${copiedGlue} glue files + out.css/shadless.js/vendor IIFEs`)

let compileErrors = 0
let builtCount = 0
// FT4: internal /docs/ link resolution — kept targets (components + guides)
// are rewritten to site-relative .html; everything else (pruned guides,
// /create, …) renders as a marked non-link so the site-wide
// 0-dangling-/docs/ gate holds (tools/docs-links.mjs enforces it).
const siteMembers = new Set(componentPages.map((p) => p.name))
const rewriteInternalLinks = (body) => body.replace(
  /<a href="([^"]*)"([^>]*)>([\s\S]*?)<\/a>/g,
  (m, href, rest, inner) => {
    if (!href.startsWith('/')) return m
    const route = resolveDocsRoute(href, siteMembers)
    if (!route) return m
    if (route.grey) {
      const cls = /class="([^"]*)"/.exec(rest)?.[1]
      const restNoClass = rest.replace(/\s*class="[^"]*"/, '')
      return `<span${restNoClass} class="unavailable-link${cls ? ` ${cls}` : ''}" title="Not available in the shadless docs mirror">${inner}</span>`
    }
    return `<a href="${route.file}${route.frag ? `#${route.frag}` : ''}"${rest}>${inner}</a>`
  }
)

const allPages = [
  // component transform keyed by the component name so the manual-tab
  // rewrite inside componentTransform reads the right
  // dist/components/<name>.html.
  ...componentPages.map((c) => ({
    name: c.name,
    source: c.source,
    transform: (src) => componentTransform(c.name, src),
  })),
  ...GUIDES.map((g) => ({ name: g.slug, source: g.source, transform: (src) => guideTransform(g, src) })),
]
for (const page of allPages) {
  try {
    const { frontmatter, body } = await compilePage(page)
    const { body: withIds, toc } = addHeadingIds(body)
    const html = pageHtml({
      name: page.name,
      title: frontmatter.title ?? page.name,
      description: frontmatter.description ?? '',
      links: frontmatter.links,
      featured: frontmatter.featured,
      body: rewriteInternalLinks(withIds),
      toc,
    })
    writeFileSync(join(OUT_DIR, `${page.name}.html`), html)
    builtCount++
  } catch (err) {
    compileErrors++
    console.error(`COMPILE ERROR [${page.name}]: ${err.message}`)
  }
}

console.log(`built pages: ${builtCount}`)
console.log(`compile errors: ${compileErrors}`)
console.log(`components index: ${meta.pages.length} radix meta entries (${mirrorSet.length} linked, ${GREY_COMPONENTS.length} greyed) + ${GUIDES.length} guides`)
if (compileErrors !== 0 || builtCount !== mirrorTotal) {
  console.error('FAIL  docs build (errors or count mismatch)')
  process.exit(1)
}
console.log(`PASS  docs build (${builtCount}/${mirrorTotal} pages: ${mirrorSet.length} components + ${GUIDES.length} guides → ${OUT_DIR}/)`)
