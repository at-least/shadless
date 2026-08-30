#!/usr/bin/env node
// docs-build — upstream mdx → VitePress markdown.
//
// This tool used to BE the site generator: MDX `evaluate()` over the upstream
// source, rendered through a hand-written vanilla JSX shim (src/docs/jsx.mjs)
// into HTML strings, wrapped in a hand-written 456-line stylesheet with its own
// sidebar, drawer, pager, TOC and code-block chrome. ~1,900 lines of us
// re-implementing a static site generator to document a component library.
// VitePress does that part; this tool now does only the part VitePress cannot:
// turn upstream's React documentation into documentation for a product that
// ships static HTML and a vanilla runtime.
//
// That job is a TEXT transform, start to finish — nothing is rendered here:
//
//   1. the section replacements (src/docs/transforms.mjs locates the spans,
//      shared byte-for-byte with the docs-fidelity gate):
//        ## Installation  the upstream <CodeTabs> is `npx shadcn add` + a
//                         React manual tab; replaced with copy-files steps
//        ## Usage         React import + JSX; replaced with the markup story
//                         and the cva-axis → data-attribute table (from the IR)
//        ## Composition   keep only the slot tree, PascalCase → data-slot
//        ## API Reference upstream's prop tables stay; the data-slot surface
//                         and the runtime contract are appended
//      plus the two declared prose adjustments and the React import fences.
//
//   2. the MDX component vocabulary → VitePress markdown. Measured over the
//      65 upstream pages AFTER step 1, only four shapes survive: 460
//      ComponentPreview, 18 Steps/30 Step, 16 Callout, 3 ComponentSource.
//      CodeTabs/Tabs* go to zero — all 61 were the install block step 1
//      replaces. The long tail (31 occurrences on 12 published pages) is
//      prose already inside inline code, except one real <Kbd>.
//
// Anything else that reaches the output as JSX is a shape this mapping has
// never seen, and it stops the build (assertNoJsx) rather than reaching a page
// as a broken Vue component.
import { readFileSync, writeFileSync, mkdirSync, rmSync, existsSync, readdirSync, copyFileSync } from 'node:fs'
import { basename, join } from 'node:path'
import { parseFrontmatter, stripImports } from '../src/docs/frontmatter.mjs'
import { GUIDES, resolveDocsRoute, writeContentMap } from './docs-guides.mjs'
import { protocolMdx, trivialMdx, apiReferenceMdx } from './fixture-families.mjs'
import { extractDemoScripts } from '../src/docs/demo-scripts.mjs'
import { fenceShadow, locateCodeTabsSpans, locateInstallSection, locateRtlMigrateSpan, locateUsageSpan, locateCompositionSpan, applyTextAdjustments, dropReactImportFences, stripImportsFromMixedFences } from '../src/docs/transforms.mjs'

const RADIX_DIR = '.upstream/shadcn-ui/apps/v4/content/docs/components/radix'
const ROOT = 'docs'                      // the VitePress project root
const COMPONENT_DIR = join(ROOT, 'components')
const GUIDE_DIR = join(ROOT, 'guides')
const PUBLIC_DIR = join(ROOT, 'public')  // served at / — demos and their assets

const catalog = JSON.parse(readFileSync('docs/catalog.json', 'utf8'))
const meta = JSON.parse(readFileSync(join(RADIX_DIR, 'meta.json'), 'utf8'))
const previews = new Map(catalog.previews.map((p) => [p.name, p]))
const rtlLangs = existsSync('build/rtl-langs.json')
  ? JSON.parse(readFileSync('build/rtl-langs.json', 'utf8'))
  : {}

// ---- mirror set: catalog sources with status existing-dist --------------------
// Upstream mirrors every component under three URLs (radix/base/aria — one per
// React primitive library). Only radix is targeted here; see the note in
// src/registry/pin.json.
const componentPages = catalog.sources
  .filter((s) => s.status === 'existing-dist')
  .filter((s) => existsSync(join(RADIX_DIR, `${s.name}.mdx`)))
  .map((s) => ({ name: s.name, source: join(RADIX_DIR, `${s.name}.mdx`) }))
  .sort((a, b) => a.name.localeCompare(b.name))
const mirrorSet = [...new Set(componentPages.map((p) => p.name))].sort()
const mirrorTotal = componentPages.length + GUIDES.length
console.log(`mirror set: ${mirrorSet.length} components = ${componentPages.length} pages + ${GUIDES.length} guides`)

// FT5: canonical grey list — the radix meta.json entries with NO shadless
// implementation. Cross-checked against docs/catalog.json at every build:
// sources no-dist ⊆ grey, and built ∪ grey == meta.json pages exactly.
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

// sidebar order follows meta.json pages[] restricted to the mirror set
const sidebarOrder = meta.pages.filter((p) => mirrorSet.includes(p))
if (sidebarOrder.length !== mirrorSet.length) {
  console.error(`FAIL mirror members missing from meta.json: ${mirrorSet.filter((n) => !meta.pages.includes(n)).join(', ')}`)
  process.exit(1)
}

// ---- routes ---------------------------------------------------------------------
const componentRoute = (name) => `/components/${name}`
const guideRoute = (slug) => `/guides/${slug}`
const siteMembers = new Set(mirrorSet)

// ---- fence-aware helpers ---------------------------------------------------------
const stripImportsOutsideFences = (src) =>
  src.split(/(```[\s\S]*?```)/).map((seg, i) => (i % 2 ? seg : stripImports(seg))).join('')

// Inline code is not markup: `<AvatarBadge>` in prose is a code span, and the
// 31-occurrence long tail is almost entirely that. Shadow it the same way
// fenceShadow shadows blocks — offsets preserved — so the scanners below see
// only real markup.
const inlineCodeShadow = (text) =>
  text.replace(/`+[^`\n]*`+/g, (m) => ' '.repeat(m.length))
const markupShadow = (text) => inlineCodeShadow(fenceShadow(text))

// Run `re` against the markup shadow and splice the replacement into the REAL
// text at the same offsets, right to left so earlier offsets stay valid.
function replaceMarkup(text, re, fn) {
  const shadow = markupShadow(text)
  const hits = [...shadow.matchAll(re)]
  let out = text
  for (const m of hits.reverse()) {
    const whole = text.slice(m.index, m.index + m[0].length)
    out = out.slice(0, m.index) + fn(whole, m) + out.slice(m.index + m[0].length)
  }
  return out
}

// JSX attributes: name="value" or name={expression}. Expressions are dropped —
// every one of them upstream is a React icon element.
function parseAttrs(tag) {
  const attrs = {}
  for (const m of tag.matchAll(/([A-Za-z][\w-]*)=(?:"([^"]*)"|\{((?:[^{}]|\{[^}]*\})*)\})/g)) {
    if (m[2] !== undefined) attrs[m[1]] = m[2]
  }
  return attrs
}

// ---- the four MDX shapes ---------------------------------------------------------

// <ComponentPreview name="alert-colors" /> — the ONLY attribute that carries
// information for us is `name` (which example), plus `direction` (which of the
// generated RTL variants). styleName is upstream's registry selector (always
// radix here); previewClassName/align/className size upstream's preview frame.
function previewMarkdown(attrs, page) {
  const name = attrs.name
  if (!name) throw new Error(`${page}: <ComponentPreview> without a name`)
  const p = previews.get(name)
  const status = p?.status ?? 'to-author'
  if (status !== 'existing-dist' && status !== 'authored') {
    const note = status === 'unavailable' ? 'demo not available in shadless (base-style demo)'
      : status === 'tombstoned' ? 'demo not available in shadless (component greyed)'
      : 'demo not yet available'
    return `<div class="demo-missing" data-demo="${name}" data-status="${status}">${note} — <code>${name}</code></div>`
  }
  const file = basename(status === 'authored' ? `docs/demos/${name}.html` : p.demoPath)
  // RTL previews are one file per language (tools/build-rtl.mjs). Arabic is the
  // BASE file and the others carry a language suffix — the same rule the old
  // site's language buttons used (assets.mjs: `lang === 'ar' ? baseSrc : …`).
  // A picker would need script; markdown links reach the same files.
  const others = (attrs.direction === 'rtl' ? (rtlLangs[name] ?? []) : [])
    .filter((l) => l !== 'ar')
    .map((l) => `[${l.toUpperCase()}](/demos/${name}-${l}.html)`)
    .join(' · ')
  return [
    `<iframe class="demo" src="/demos/${file}" title="${name}" data-status="${status}" loading="lazy"></iframe>`,
    `\n<p class="demo-langs">[Open the demo page](/demos/${file})${others ? ` · ${others}` : ''}</p>\n`,
    demoSource(name, file),
  ].join('')
}

// The source under the preview. Upstream's ComponentPreview shows the .tsx;
// what a shadless consumer copies is the MARKUP, so that is what this shows —
// the demo page's <body>, without the page scaffold (doctype, the ../out.css
// link, the theme pre-paint boilerplate) that the old site's HTML tab dumped
// along with it. The scaffold is not content: the Installation section above
// already names every file the page loads.
//
// The behavior file is the component's own dist/js/<name>.js. The shared
// runtime (dist/shadless.js, 163 KB, identical on every page) is REFERENCED,
// never inlined — inlining it 121× was 90% of the old site's bytes.
function demoSource(name, file) {
  const path = join(PUBLIC_DIR, 'demos', file)
  if (!existsSync(path)) return ''
  const raw = readFileSync(path, 'utf8')
  const markup = (/<body[^>]*>([\s\S]*?)<\/body>/.exec(raw)?.[1] ?? raw)
    .replace(/<script[\s\S]*?<\/script>/g, '')
    .replace(/\n{3,}/g, '\n\n')
    .trim()
  const { srcScripts, inlineScripts } = extractDemoScripts(raw)
  const js = []
  if (srcScripts.includes('shadless.js')) js.push('// <script src="shadless.js"></script>  — the shared runtime (see Installation)')
  for (const s of srcScripts) {
    if (s === 'shadless.js') continue
    js.push(`// ${s}\n${readFileSync(join('dist', s), 'utf8').trim()}`)
  }
  js.push(...inlineScripts)
  const jsText = js.join('\n\n').trim()
  // `text`, not `html`, for the markup: these blocks are one long line of
  // emitted markup, and tokenising 460 of them at build time inflated the site
  // from 11 MB to 49 MB — the same blow-up that moved the old site's
  // highlighting into the browser (28386e6). The behavior file is short and
  // worth colouring.
  if (!jsText) return `\n::: details Source\n\`\`\`text\n${markup}\n\`\`\`\n:::\n`
  return `\n:::: details Source\n\n::: code-group\n\`\`\`text [${file}]\n${markup}\n\`\`\`\n\n\`\`\`js [behavior]\n${jsText}\n\`\`\`\n:::\n\n::::\n`
}

// <Callout variant="info" title="…"> … </Callout> → a VitePress container.
// Upstream's variants are info/warning; the icon attribute is a React element.
const CALLOUT_KIND = { info: 'tip', warning: 'warning', danger: 'danger' }
function convertCallouts(text, page) {
  let out = text
  for (;;) {
    const shadow = markupShadow(out)
    const open = /<Callout\b[^>]*>/.exec(shadow)
    if (!open) break
    const close = shadow.indexOf('</Callout>', open.index)
    if (close === -1) throw new Error(`${page}: <Callout> without a closing tag`)
    const attrs = parseAttrs(out.slice(open.index, open.index + open[0].length))
    const kind = CALLOUT_KIND[attrs.variant ?? 'info'] ?? 'tip'
    const body = out.slice(open.index + open[0].length, close)
      .split('\n').map((l) => l.replace(/^ {1,3}(?=\S)/, '')).join('\n').trim()
    const block = `::: ${kind}${attrs.title ? ` ${attrs.title}` : ''}\n${body}\n:::`
    out = out.slice(0, open.index) + block + out.slice(close + '</Callout>'.length)
  }
  return out
}

// <Steps> wraps <Step>…</Step> with markdown blocks (tables, fences) between
// them, which is why upstream needs a component: an <ol> may not contain a
// <table>. In markdown the wrapper carries nothing, and a step is a lead-in
// line for the block that follows it.
function convertSteps(text) {
  let out = replaceMarkup(text, /<\/?Steps\b[^>]*>/g, () => '')
  out = replaceMarkup(out, /<Step>([\s\S]*?)<\/Step>/g, (_w, m) => `**${m[1].trim()}**`)
  return out
}

// Upstream renders the registry .tsx of a component here. shadless has no
// .tsx to show — the shipped artifact is the demo page's markup, which the
// preview above already links. The old builder emitted a "source not yet
// available" placeholder; drop it instead of shipping a promise.
const dropComponentSource = (text) =>
  replaceMarkup(text, /<ComponentSource\b[\s\S]*?\/>/g, () => '')

// <Kbd>Tab</Kbd> is the one long-tail component that is real markup rather
// than a code span — and it is a plain HTML element.
const convertKbd = (text) =>
  replaceMarkup(text, /<\/?Kbd>/g, (w) => (w[1] === '/' ? '</kbd>' : '<kbd>'))

// <LinkedCard href="/docs/rtl/next"> — the framework-card grid at the top of
// the rtl guide. Every href points at a per-framework setup page that is
// PRUNED here (docs-guides.mjs, 'framework sub-pages'): shadless has no
// framework setup step. The declared disposition for those links is GREYED,
// not removed, so the cards keep their contents and lose the link.
const convertLinkedCards = (text) =>
  replaceMarkup(text, /<LinkedCard\b[^>]*>|<\/LinkedCard>/g,
    (w) => (w[1] === '/' ? '</div>' : '<div class="linked-card">'))

// Raw HTML blocks in upstream mdx are JSX: className is React's spelling.
const convertClassName = (text) => replaceMarkup(text, /\bclassName=/g, () => 'class=')

// Internal /docs/ links: rewrite to this site's routes, and render links to
// pages that do not exist here as plain text (upstream links base/aria
// variants, pruned guides and /create).
function rewriteLinks(text) {
  return replaceMarkup(text, /\[([^\]]*)\]\((\/[^)\s]*)\)/g, (whole, m) => {
    const route = resolveDocsRoute(m[2], siteMembers)
    if (!route) return whole
    if (route.grey) return m[1]
    const slug = route.file.replace(/\.html$/, '')
    const target = siteMembers.has(slug) ? componentRoute(slug) : guideRoute(slug)
    return `[${m[1]}](${target}${route.frag ? `#${route.frag}` : ''})`
  })
}

// The guard that replaces the old builder's silent skips: any JSX that reaches
// here is a shape this mapping has never seen. Rendering it would hand VitePress
// an unresolved Vue component; the build stops instead.
function assertNoJsx(page, text) {
  const shadow = markupShadow(text)
  const left = [...new Set([...shadow.matchAll(/<([A-Z]\w*)/g)].map((m) => m[1]))]
  if (left.length) throw new Error(`${page}: unmapped JSX components: ${left.join(', ')}`)
}

// ---- section transforms (unchanged; spans from src/docs/transforms.mjs) -----------
const replaceSpan = (raw, start, end, replacement) => raw.slice(0, start) + replacement + raw.slice(end)

// Which of upstream's optional sections this page carried. Recorded per page in
// the content map: upstream renaming `## Usage` used to drop the whole
// replacement silently and leave the React original in place, with every gate
// green. The section set is committed, so a rename now shows up as a diff.
const sectionsSeen = new Map()

function installStepsMdx(name) {
  const demo = readFileSync(join('dist/components', `${name}.html`), 'utf8')
  const initAll = /shadless\.initAll\(\)/.test(demo)
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
  const cssImports = hasOwnCss ? `\n@import "shadless/${name}.css";` : ''
  const noCssNote = hasOwnCss ? '' : `\nThis component has no stylesheet of its own — its styling rides the core theme and utilities in \`shadless\`.\n`
  return `<Steps>

<Step>Add shadless${hasOwnCss ? ' and this component' : ''} to your Tailwind v4 entry:</Step>

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

function usageMdx(name) {
  let axes = []
  try {
    const ir = JSON.parse(readFileSync(join('src/registry/ir', `${name}.json`), 'utf8'))
    for (const table of Object.values(ir.cva ?? {}))
      axes = [...new Set([...axes, ...Object.keys(table.variants ?? {})])]
  } catch { /* no IR (e.g. direction) — no axes table */ }
  const rows = axes.map((a) => `| \`${a}="outline"\` (JSX prop) | \`data-${a}="outline"\` (markup) |`).join('\n')
  return `## Usage

Copy the markup from \`dist/components/${name}.html\` and adapt it — every slot
is a \`data-slot\` attribute, and open/close state is a \`data-state\` the
runtime drives.${rows ? ` The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
${rows}` : ''}
`
}

function compositionTransform(name, raw, seen) {
  const span = locateCompositionSpan(fenceShadow(raw))
  if (!span) return raw
  seen.push('composition')
  const section = raw.slice(span.start, span.end)
  const tree = /```text\n([\s\S]*?)```/.exec(section)?.[1]?.trimEnd()
  let mapped = null
  if (tree) {
    const nameToSlot = {}
    const ir = JSON.parse(readFileSync(join('src/registry/ir', `${name}.json`), 'utf8'))
    for (const c of ir.components) {
      const slot = c.elements.find((e) => e.slot)?.slot
      if (slot) nameToSlot[c.fn] = slot
    }
    mapped = tree.replace(/[A-Z][A-Za-z0-9]+/g, (w) => nameToSlot[w] ?? w)
  }
  const body = mapped
    ? `The slot tree — every node is a \`data-slot\` attribute in the shipped markup:\n\n\`\`\`text\n${mapped}\n\`\`\`\n`
    : `See the demos for real compositions — every slot is a \`data-slot\` attribute in the shipped markup.\n`
  return replaceSpan(raw, span.start, span.end, `## Composition\n\n${body}\n`)
}

// upstream's ## API Reference is the React prop tables. They stay — they
// document the behavior contract the kernel mirrors — and the surface a
// shadless consumer actually authors against is appended: the data-slot
// vocabulary from the IR and the runtime contract from the fixture families.
//
// The insert point is the heading itself. It used to be "after the section's
// first paragraph (the Radix link line)", but 19 of the 52 pages open the
// section with a `### <Component>` subheading instead — on those the surface
// landed INSIDE the first subcomponent, reading as documentation of that one
// slot rather than of the component.
function apiReferenceTransform(name, raw, seen) {
  const m = /^## API Reference[ \t]*\n/m.exec(raw)
  if (!m) return raw
  seen.push('api-reference')
  let slots = []
  const irPath = join('src/registry/ir', `${name}.json`)
  if (existsSync(irPath)) {
    const ir = JSON.parse(readFileSync(irPath, 'utf8'))
    slots = [...new Set(ir.components.flatMap((c) => c.elements.map((e) => e.slot).filter(Boolean)))]
  }
  const extra = apiReferenceMdx(name, slots)
  if (!extra) return raw
  const at = m.index + m[0].length
  return raw.slice(0, at) + '\n' + extra + raw.slice(at)
}

function componentTransform(name, raw) {
  const seen = []
  raw = stripImportsFromMixedFences(dropReactImportFences(raw))
  const spans = locateCodeTabsSpans(fenceShadow(raw))
  if (spans.length !== 1) throw new Error(`install code-tabs: fence-shadowed count ${spans.length}, expected 1`)
  seen.push('installation')
  let out = replaceSpan(raw, spans[0].start, spans[0].end, installStepsMdx(name))
  const usage = locateUsageSpan(fenceShadow(out))
  if (usage) {
    seen.push('usage')
    out = replaceSpan(out, usage.start, usage.end, usageMdx(name))
  }
  out = compositionTransform(name, out, seen)
  out = apiReferenceTransform(name, out, seen)
  sectionsSeen.set(name, seen.sort())
  return applyTextAdjustments(`${name}.mdx`, out)
}

function utilsInstallMdx(util) {
  return `## Installation

In shadless, the \`${util}\` utilities ship precompiled inside \`dist/out.css\` —
no npm install, Tailwind setup, or CSS import is required. Load \`out.css\` and
use the classes directly (see the [Installation](/docs/installation) guide).`
}

function rtlMigrateMdx() {
  return `shadless components ship the pinned registry's classes as-is — the current registry already uses logical (start/end-aware) utilities, so there is no migration step: every component is RTL-ready the moment the page carries \`dir="rtl"\`. To flip an individual icon, give it the \`rtl:rotate-180\` utility class.`
}

function guideTransform(guide, raw) {
  if (guide.rtlMigrate) {
    const span = locateRtlMigrateSpan(fenceShadow(raw))
    if (!span) throw new Error(`rtl migrate section: not found in ${guide.source}`)
    raw = replaceSpan(raw, span.start, span.end, rtlMigrateMdx())
    const LINK = 'For other styles, see the [Migration Guide](#migrating-existing-components).'
    if (!raw.includes(LINK)) throw new Error('rtl guide: migrate anchor link not found — re-anchor')
    raw = raw.replace(LINK, 'For other styles, the shipped utilities are already logical (start/end-aware) — `dir="rtl"` is all it takes.')
  }
  if (!guide.installSection) return raw
  const span = locateInstallSection(fenceShadow(raw))
  if (!span) throw new Error('utils Installation section: not found (or no following ## Usage)')
  return replaceSpan(raw, span.start, span.end, utilsInstallMdx(guide.util) + '\n\n')
}

// ---- page assembly ----------------------------------------------------------------
const yaml = (s) => JSON.stringify(String(s)) // JSON is valid YAML for scalars

function buildPage(page) {
  const raw = readFileSync(page.source, 'utf8')
  const fm = parseFrontmatter(raw)
  let body = page.transform(raw)
  body = body.replace(/^---\n[\s\S]*?\n---\n/, '') // frontmatter is re-emitted
  body = stripImportsOutsideFences(body)
  body = dropComponentSource(body)
  body = convertCallouts(body, page.name)
  body = convertSteps(body)
  body = convertKbd(body)
  body = convertLinkedCards(body)
  body = convertClassName(body)
  body = replaceMarkup(body, /<ComponentPreview\b[\s\S]*?\/>/g,
    (whole) => previewMarkdown(parseAttrs(whole), page.name))
  body = rewriteLinks(body)
  assertNoJsx(page.name, body)

  const title = fm.title ?? page.name
  // frontmatter.links are upstream's "Docs / API Reference" chips. Marked so
  // docs-fidelity can find them without guessing which paragraph they are.
  // frontmatter.links are upstream's "doc / api" chips. Emitted as an html
  // block (markdown inside one is NOT parsed, so these are anchors, not
  // markdown links) and marked so docs-fidelity finds them without having to
  // guess which paragraph they are.
  const links = fm.links && Object.keys(fm.links).length
    ? `<p class="page-links">${Object.entries(fm.links).map(([k, v]) => `<a href="${v}" rel="noopener">${k}</a>`).join(' · ')}</p>\n`
    : ''
  const front = [
    '---',
    `title: ${yaml(title)}`,
    ...(fm.description ? [`description: ${yaml(fm.description)}`] : []),
    '---',
  ].join('\n')
  return `${front}\n\n# ${title}\n\n${fm.description ? `${fm.description}\n\n` : ''}${links}${links ? '\n' : ''}${body.trim()}\n`
}

// ---- write ------------------------------------------------------------------------
rmSync(COMPONENT_DIR, { recursive: true, force: true })
rmSync(GUIDE_DIR, { recursive: true, force: true })
mkdirSync(COMPONENT_DIR, { recursive: true })
mkdirSync(GUIDE_DIR, { recursive: true })

writeContentMap(componentPages.map((p) => ({ name: p.name, source: p.source })))

const allPages = [
  ...componentPages.map((c) => ({
    name: c.name, source: c.source, dir: COMPONENT_DIR,
    transform: (src) => componentTransform(c.name, src),
  })),
  ...GUIDES.map((g) => ({
    name: g.slug, source: g.source, dir: GUIDE_DIR,
    transform: (src) => guideTransform(g, src),
  })),
]
let built = 0
const errors = []
for (const page of allPages) {
  try {
    writeFileSync(join(page.dir, `${page.name}.md`), buildPage(page))
    built++
  } catch (err) {
    errors.push(`${page.name}: ${err.message}`)
  }
}

// section presence, per page, in the committed decision record — see sectionsSeen
{
  const mapPath = 'docs/content-map.json'
  const map = JSON.parse(readFileSync(mapPath, 'utf8'))
  for (const [name, entry] of Object.entries(map.pages ?? {})) {
    const seen = sectionsSeen.get(name)
    if (seen) entry.sections = seen
  }
  writeFileSync(mapPath, JSON.stringify(map, null, 2) + '\n')
}

// ---- index + sidebar ---------------------------------------------------------------
const indexMd = `---
title: "Components"
---

# Components

${mirrorSet.length} radix components available · ${GREY_COMPONENTS.length} not available (upstream tombstones or out of pipeline scope) · ${GUIDES.length} guides.

New here? Read the [Introduction](${guideRoute('introduction')}) to learn what shadless is and why it exists.

${meta.pages.map((n) => greySet.has(n)
  ? `- ${n} <span class="unavailable">not available</span>`
  : `- [${n}](${componentRoute(n)})`).join('\n')}

## Guides

${GUIDES.map((g) => `- [${g.title}](${guideRoute(g.slug)})`).join('\n')}
`
writeFileSync(join(ROOT, 'index.md'), indexMd)

const sidebar = [
  {
    text: 'Getting started',
    items: GUIDES.filter((g) => g.pinned).map((g) => ({ text: g.title, link: guideRoute(g.slug) })),
  },
  {
    text: 'Components',
    items: [{ text: 'All components', link: '/' },
      ...sidebarOrder.map((n) => ({ text: n, link: componentRoute(n) }))],
  },
  {
    text: 'Guides',
    items: GUIDES.filter((g) => !g.pinned).map((g) => ({ text: g.title, link: guideRoute(g.slug) })),
  },
  {
    text: 'Not available',
    items: meta.pages.filter((p) => greySet.has(p)).map((n) => ({ text: n })),
  },
]
mkdirSync(join(ROOT, '.vitepress'), { recursive: true })
writeFileSync(join(ROOT, '.vitepress/sidebar.json'), JSON.stringify(sidebar, null, 2) + '\n')

// ---- demos + their assets into the served tree -------------------------------------
// The pages iframe /demos/<file>.html. A demo's relative asset refs
// (../out.css, ../shadless.js, ../js/*) resolve one level up, so the shared
// assets sit at the public root. This is a copy of two trees this repo already
// reviews — it is a build artifact, and docs/public is gitignored.
rmSync(PUBLIC_DIR, { recursive: true, force: true })
mkdirSync(join(PUBLIC_DIR, 'demos'), { recursive: true })
mkdirSync(join(PUBLIC_DIR, 'js'), { recursive: true })
let copied = 0
for (const tree of ['dist/components', 'docs/demos']) {
  for (const f of readdirSync(tree)) {
    if (!f.endsWith('.html')) continue
    copyFileSync(join(tree, f), join(PUBLIC_DIR, 'demos', f))
    copied++
  }
}
for (const asset of ['out.css', 'shadless.js']) copyFileSync(join('dist', asset), join(PUBLIC_DIR, asset))
let glue = 0
for (const f of readdirSync('dist/js')) { copyFileSync(join('dist/js', f), join(PUBLIC_DIR, 'js', f)); glue++ }
console.log(`demos copied: ${copied} pages, ${glue} behavior files + out.css/shadless.js`)

if (errors.length) {
  for (const e of errors) console.error(`  - ${e}`)
  console.error(`FAIL  docs build (${errors.length} pages failed)`)
  process.exit(1)
}
if (built !== mirrorTotal) {
  console.error(`FAIL  docs build (built ${built}, expected ${mirrorTotal})`)
  process.exit(1)
}
console.log(`PASS  docs build (${built}/${mirrorTotal} pages: ${mirrorSet.length} components + ${GUIDES.length} guides → markdown)`)
