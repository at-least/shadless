// FT2 docs: MDX component map (the pipeline's equivalent of an mdx-components file).
// Enumerated from the 49 built radix mdx files (fence- and inline-code
// stripped): ComponentPreview, ComponentSource, CodeTabs, TabsList,
// TabsTrigger, TabsContent, Steps, Step, Callout, Kbd + the two imported
// icon identifiers (InfoIcon, IconInfoCircle).
// FT3: ComponentPreview existing-dist → iframe (site-relative demo copy in
// docs/site/components/, laid out like dist so ../out.css etc. resolve) +
// Preview/HTML/JS tabs reading the REAL dist demo files from the registry
// (docs/catalog.json name → demoPath). JS tab = local scripts the demo
// uses (glue/*.js, shadless.js, inline init); pinned vendor IIFEs
// (radix-kernel, embla) and the unified out.css are skipped — recorded in
// PLAN FT3 report. to-author/unknown → explicit "demo not yet available"
// note (FT7 flips them). 'authored' (FT7 batches: docs/demos/<name>.html,
// same dist-style relative refs) gets the full preview treatment — FT7
// never touches the build. ComponentSource stays a placeholder (FT5/FT7).
import { readFileSync, existsSync } from 'node:fs'
import { basename } from 'node:path'
import { el } from './jsx.mjs'
import { iconComponent } from './icons.mjs'
import { extractDemoScripts } from './demo-scripts.mjs'
import { tokenizeLines, lineNumbersAttrs } from './highlight.mjs'

const catalog = JSON.parse(readFileSync('docs/catalog.json', 'utf8'))
const previews = new Map(catalog.previews.map((p) => [p.name, p]))
const previewStatus = new Map(catalog.previews.map((p) => [p.name, p.status]))
// FT4: guide preview dispositions from docs/content-map.json (kept guides
// carry previews the radix catalog never enumerated). 'unavailable' =
// base-line demo outside shadless scope (renders an explicit note, never an
// iframe); 'to-author' = same interim note as radix to-author names.
// Read lazily inside buildComponentMap(): the build writes the content map
// before compiling pages (ESM imports evaluate first, so a module-level
// read would race the generator).
let guidePreviewStatus = new Map()
// FT8/Step 9: which languages have a generated file per RTL preview.
// Written by tools/build-rtl.mjs. Used to render only the language
// buttons whose target file exists, so the iframe never 404s on click.
const rtlLangs = existsSync('docs/site/rtl-langs.json')
  ? JSON.parse(readFileSync('docs/site/rtl-langs.json', 'utf8'))
  : {}
// status → on-disk demo file (authored demos live in docs/demos/)
const demoFile = (name, status) => {
  if (status === 'authored') return `docs/demos/${name}.html`
  const p = previews.get(name)
  if (!p?.demoPath) throw new Error(`no demo path for "${name}" (status=${status}) — run tools/docs-catalog.mjs`)
  return p.demoPath
}

const classes = (...xs) => xs.filter(Boolean).join(' ')
const kids = (c) => (c === undefined || c === null ? [] : Array.isArray(c) ? c : [c])

// local (non-vendor) scripts a demo page references → real file bodies.
// Inline scripts go through extractDemoScripts so the FT9 theme pre-paint
// boilerplate never leaks into the docs JS tab (Wave H D3).
function demoSources(file) {
  const html = readFileSync(file, 'utf8')
  const { srcScripts, inlineScripts } = extractDemoScripts(html)
  const parts = srcScripts.map((s) => readFileSync(`dist/${s}`, 'utf8').trim())
  parts.push(...inlineScripts)
  return { html, js: parts.join('\n\n') }
}

const ComponentPreview = (p) => {
  const name = p.name ?? '(unnamed)'
  const status = previewStatus.get(name) ?? guidePreviewStatus.get(name) ?? 'unknown'
  const catalogEntry = previews.get(name)
  const quality = catalogEntry?.quality ?? null
  const attrs = {
    'data-component-preview': name,
    ...(p.styleName ? { 'data-style-name': p.styleName } : {}),
    ...(p.description ? { 'data-description': p.description } : {}),
    ...(p.direction ? { 'data-direction': p.direction } : {}),
    'data-status': status,
    ...(quality ? { 'data-quality': quality } : {}),
  }
  if (status !== 'existing-dist' && status !== 'authored') {
    const note = status === 'unavailable'
      ? 'demo not available in shadless (base-style demo)'
      : status === 'tombstoned'
      ? 'demo not available in shadless (component greyed)'
      : 'demo not yet available'
    const entry = previews.get(name)
    const host = entry?.hostPages?.find((h) => h !== name)
    const crossLink = host ? { href: `${host}.html`, label: status === 'unavailable' ? 'see guide' : 'see host page' } : null
    return el('div', { ...attrs, class: 'component-preview-ph' }, [
      el('span', { class: 'ph-note' }, [note]),
      el('code', { class: 'ph-name' }, [name]),
      crossLink ? el('span', { class: 'ph-cross-link' }, [
        ' — ',
        el('a', { href: crossLink.href }, [crossLink.label]),
      ]) : null,
    ].filter(Boolean))
  }
  const file = demoFile(name, status)
  const { html, js } = demoSources(file)
  // Stacked layout mirroring upstream shadcn/ui: preview iframe on top,
  // source code blocks below (HTML + JS stacked). The source starts
  // COLLAPSED to a capped sliver behind a gradient veil with a centered
  // "View Code" button (upstream ComponentPreview); site.js flips
  // data-open="true" on click, which uncaps the body and hides the veil.
  // Upstream parity: source blocks get the SAME treatment as mdx fences —
  // dual-theme shiki tokens (--shiki-light/--shiki-dark vars) + line-number
  // gutter via data-line spans + data-line-numbers attrs (site.css draws
  // the counter ::before). HTML/JS tabs both highlight.
  const tokenSpanStyle = (tok) => [
    tok.light ? `--shiki-light:${tok.light}` : '',
    tok.dark ? `--shiki-dark:${tok.dark}` : '',
    tok.italic ? 'font-style:italic' : '',
    tok.bold ? 'font-weight:bold' : '',
  ].filter(Boolean).join(';')
  const highlightedCode = (code, lang) => {
    const lines = tokenizeLines(code, lang)
    const lineSpans = lines.map((tokens) => {
      const children = tokens.map((tok) =>
        tok.light || tok.italic || tok.bold
          ? el('span', { style: tokenSpanStyle(tok) }, [tok.content])
          : tok.content)
      const last = children[children.length - 1]
      if (typeof last === 'string') children[children.length - 1] = last + '\n'
      else children.push('\n')
      return el('span', { 'data-line': '' }, children)
    })
    return el('code', lineNumbersAttrs(lines.length), lineSpans)
  }
  const sourceBlock = (label, code, lang) => el('div', { class: 'source-block' }, [
    el('div', { class: 'source-label' }, [label]),
    el('pre', { class: 'preview-code' }, [highlightedCode(code, lang)]),
  ])
  const sourceBody = el('div', { class: 'source-body' }, [
    sourceBlock('HTML', html, 'html'),
    ...(js ? [sourceBlock('JS', js, 'js')] : []),
  ])
  const previewFrame = el('div', { class: 'preview-frame' }, [
    el('iframe', {
      src: `components/${basename(file)}`,
      title: `${name} demo`,
      loading: 'lazy',
      'data-preview-frame': '',
    }),
    // FT8/Step 9: RTL language selector. Only emitted when the preview
    // is RTL. Replaces the Step 6 single-toggle button with a row of
    // language buttons (Arabic / Hebrew / English / Persian, when
    // available for that preview). Click swaps the iframe src to the
    // matching `<name>-rtl-<lang>.html` file. Buttons for languages
    // that weren't generated (no upstream translation for that
    // component) are skipped at compile time, so the iframe never
    // 404s on click.
    ...(p.direction === 'rtl'
      ? [el('div', { class: 'preview-rtl-langs' }, (
          rtlLangs[name] || ['ar']
        ).map((lang, i) => el('button', {
          type: 'button',
          'data-rtl-lang': lang,
          ...(i === 0 ? { 'data-active-lang': '' } : {}),
        }, [lang.toUpperCase()])))]
      : []),
    // FT8/Step 9: RTL language selector — sits in the preview frame as
    // before (see above).
    ...(quality === 'informational'
      ? [el('span', {
          class: 'preview-quality',
          title: 'Authored from a React-hook example; interactive semantics are not contract-tested',
        }, ['informational'])]
      : []),
  ])
  return el('div', { ...attrs, class: 'component-preview' }, [
    previewFrame,
    // FT8/Step 4 (reworked for upstream parity): source below the
    // preview, collapsed behind the gradient veil. The veil carries the
    // centered "View Code" button; site.js sets data-open="true" on
    // click, uncapping .source-body and hiding the veil via CSS.
    el('div', { class: 'preview-source' }, [
      sourceBody,
      el('div', { class: 'source-veil' }, [
        el('button', {
          type: 'button',
          class: 'view-code-cta',
          'data-view-code': '',
          'aria-expanded': 'false',
          // aria-controls is set by site.js once the source-body has an id
        }, ['View Code']),
      ]),
    ]),
  ])
}

const ComponentSource = (p) => el('pre', {
  'data-component-source': p.name ?? '',
  ...(p.title ? { 'data-title': p.title } : {}),
  ...(p.styleName ? { 'data-style-name': p.styleName } : {}),
  class: 'component-source-ph',
}, [
  el('code', {}, [`source not yet available — ${p.name ?? '(unnamed)'}`]),
])

const Callout = (p) => el('div', {
  'data-callout': '',
  'data-variant': p.variant ?? 'info',
  class: classes('callout', p.variant && `callout-${p.variant}`, p.className),
}, [
  // icon is always provided by buildComponentMap's wrapper (default = lucide
  // Info); rendering a fallback custom element here would be invalid HTML
  ...(p.icon ? [el('span', { class: 'callout-icon' }, [p.icon])] : []),
  el('div', { class: 'callout-body' }, [
    ...(p.title ? [el('p', { class: 'callout-title' }, [p.title])] : []),
    ...kids(p.children),
  ]),
])

// FT8: vanilla Accordion shim — renders the same `data-slot` markup as
// dist/components/accordion*.html, which the shadless.js runtime already
// drives (click + arrow key handling). The MDX form is purely for prose
// authoring; users get the same interactive component as the demo.
let accordionSeq = 0
const AccordionRoot = (p) => {
  const id = `acc-intro-${accordionSeq++}`
  const type = p.type ?? 'single'
  const wrap = el('div', {
    'data-slot': 'accordion',
    'data-type': type === 'multiple' ? 'multiple' : null,
    id,
    class: p.className,
  }, kids(p.children))
  return wrap
}
const AccordionItem = (p) => el('div', {
  'data-slot': 'accordion-item',
  'data-state': 'closed',
  class: p.className,
}, kids(p.children))
// Mirrors upstream AccordionPrimitive.Header: the h3 wraps ONLY the trigger
// button — content stays outside the heading (wrapping everything made the
// whole answer part of the h3 and produced invalid button>p nesting).
// Paragraph children from mdx prose are flattened to their text — <p> is
// invalid inside <button> and browsers re-parent it on parse.
const flattenParas = (c) => kids(c).flatMap((n) =>
  n && typeof n === 'object' && n.__el && n.type === 'p' ? kids(n.props?.children) : [n])
const AccordionTrigger = (p) => el('h3', { class: 'flex' }, [
  el('button', {
    'data-slot': 'accordion-trigger',
    type: 'button',
    'aria-expanded': 'false',
    'data-state': 'closed',
    class: p.className,
  }, flattenParas(p.children)),
])
const AccordionContent = (p) => el('div', {
  'data-slot': 'accordion-content',
  'data-state': 'closed',
  hidden: '',
  class: p.className,
}, kids(p.children))

const LinkedCard = (p) => el('a', {
  href: p.href ?? '#',
  class: classes('linked-card', p.className),
}, kids(p.children))

// MDX leaves markdown blocks authored between <Step> tags (tables, code
// fences) as direct children of <Steps>; <ol> may only contain <li>.
// Fold each stray block into the preceding step so the built HTML is
// valid and step content stays inside its <li> — matching upstream's
// <Steps> render (table under step 1, fence under step 2).
const appendChild = (node, child) => {
  node.props.children = [...kids(node.props.children), child]
}
// Hoisted so isStepNode can recognize pending <Step> elements.
const Step = (p) => el('li', { 'data-step': '', class: 'step' }, kids(p.children))
// <Step> nodes stay function-typed until serialize() calls them (vanilla
// JSX shim), so detect both shapes: a called Step (el 'li') and a pending
// one (element whose type IS the Step function).
const isStepNode = (c) => c && typeof c === 'object' && c.__el && (c.type === 'li' || c.type === Step)
const StepsList = (p) => {
  const items = []
  for (const c of kids(p.children)) {
    if (isStepNode(c)) items.push(c)
    else if (items.length) appendChild(items[items.length - 1], c)
    else items.push(el('li', { 'data-step': '', class: 'step' }, [c]))
  }
  return el('ol', { 'data-steps': '', class: classes('steps', p.className) }, items)
}

export function buildComponentMap() {
  if (existsSync('docs/content-map.json')) {
    guidePreviewStatus = new Map(
      Object.entries(JSON.parse(readFileSync('docs/content-map.json', 'utf8')).guidePreviews ?? {})
        .map(([name, p]) => [name, p.disposition])
    )
  }
  const defaultInfoIcon = iconComponent('Info')
  const components = {
    ComponentPreview,
    ComponentSource,
    Callout: (p) => Callout({ ...p, icon: p.icon ?? defaultInfoIcon() }),
    CodeTabs: (p) => el('div', { 'data-code-tabs': '', class: 'code-tabs' }, kids(p.children)),
    TabsList: (p) => el('div', { 'data-tabs-list': '', role: 'tablist', class: 'tabs-list' }, kids(p.children)),
    TabsTrigger: (p) => el('button', {
      'data-tab-trigger': p.value ?? '',
      type: 'button',
      role: 'tab',
      class: 'tab-trigger',
      ...(p.value === undefined ? {} : { 'aria-selected': 'false' }),
    }, kids(p.children)),
    TabsContent: (p) => el('div', {
      'data-tab-content': p.value ?? '',
      role: 'tabpanel',
      class: 'tab-content',
      hidden: '',
    }, kids(p.children)),
    Steps: StepsList,
    Step,
    Kbd: (p) => el('kbd', { class: p.className }, kids(p.children)),
    LinkedCard,
    Accordion: AccordionRoot,
    AccordionItem,
    AccordionTrigger,
    AccordionContent,
    InfoIcon: iconComponent('InfoIcon'),
    IconInfoCircle: iconComponent('IconInfoCircle'),
  }
  return components
}
