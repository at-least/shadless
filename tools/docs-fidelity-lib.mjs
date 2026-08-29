// docs-fidelity-lib.mjs — Wave I gate B pure helpers: mdx↔html page
// fidelity comparison. Extracted so tools/docs-fidelity.mjs stays a thin
// driver and the parsing/comparison logic is unit-testable.
//
// Two normalization sides, deliberately asymmetric (2026-08-26 audit
// lesson): mdx-side text NEVER strips tags (a fence line like
// `useChat<ChatMessage>({` must keep its generics — stripping "tags"
// there produced phantom missing-fence reports), while html-side text
// strips markup first and decodes entities second.
import { parseFrontmatter } from "../src/docs/frontmatter.mjs"
import { fenceShadow, locateCodeTabsSpans, locateInstallSection, locateRtlMigrateSpan, locateUsageSpan, locateCompositionSpan, applyTextAdjustments, dropReactImportFences, stripImportsFromMixedFences, TEXT_ADJUSTMENTS } from "../src/docs/transforms.mjs"

// re-exported for the unit suite (fence shadow now lives in transforms.mjs)
export { fenceShadow as blankFences }

export const decodeEntities = (s) => s
  .replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&quot;/g, '"')
  .replace(/&#39;/g, "'").replace(/&#x27;/g, "'").replace(/&amp;/g, "&")

// html side: drop tags, decode entities, collapse whitespace
export const htmlText = (s) => decodeEntities(String(s).replace(/<[^>]*>/g, "")).replace(/\s+/g, " ").trim()

// mdx side: collapse whitespace only — NO tag stripping (generics!)
export const mdText = (s) => String(s).replace(/\s+/g, " ").trim()

// heading text: mdx inline code backticks are not rendered
export const stripInlineCode = (s) => s.replace(/`([^`]+)`/g, "$1")

// ---- fence scanning (line-based; handles 3+ backtick fences and info
// strings like ```tsx showLineNumbers — a regex pair-scan mis-opens on
// the CLOSING fence of an info-string fence and produced 400 phantom
// issues in the first audit) -----------------------------------------------
export function scanFences(src) {
  const fences = []
  let open = null, lang = "", buf = []
  for (const line of src.split("\n")) {
    if (open === null) {
      const m = /^(`{3,})(\w*)/.exec(line)
      if (m) { open = m[1].length; lang = m[2]; buf = []; continue }
    } else if ((line.match(/^`{3,}/) || [""])[0].length >= open) {
      fences.push({ lang, content: buf.join("\n") })
      open = null; continue
    } else buf.push(line)
  }
  // unclosed fence: MDX renders code-to-EOF — emit the buffer so the
  // survival check still applies (and malformed sources aren't silently ok)
  if (open !== null && buf.length) fences.push({ lang, content: buf.join("\n") })
  return fences
}

// ---- transform drops (span location comes from src/docs/transforms.mjs
// — the SAME locate* the builder replaces with, so the gate cannot
// drift from the build; see that module's header) ----
export function withoutCodeTabs(raw) {
  const shadow = fenceShadow(raw)
  let out = raw
  // right-to-left so earlier offsets stay valid while slicing. The
  // builder replaces the whole install <CodeTabs> block (CLI + manual
  // tabs) with the shadless copy-files steps; the replacement adds no
  // headings and its fences are extra html (one-directional fence check)
  for (const { start, end } of locateCodeTabsSpans(shadow).reverse()) {
    out = out.slice(0, start) + "\n" + out.slice(end)
  }
  return out
}

export function withoutCompositionSection(raw) {
  const span = locateCompositionSpan(fenceShadow(raw))
  if (!span) return raw
  // builder keeps the heading + a slot-tree text fence; mdx facts keep
  // the heading (fence survival is one-directional, extras tolerated)
  return raw.slice(0, span.start) + "## Composition\n" + raw.slice(span.end)
}

export function withoutUsageSection(raw) {
  const span = locateUsageSpan(fenceShadow(raw))
  if (!span) return raw
  // guideTransform-analog: the builder replaces the span with usageMdx,
  // which opens with the same `## Usage` heading and adds NO fences —
  // keeping the heading keeps the heading comparison aligned
  return raw.slice(0, span.start) + "## Usage\n" + raw.slice(span.end)
}

export function withoutRtlMigrate(raw) {
  const span = locateRtlMigrateSpan(fenceShadow(raw))
  if (!span) return raw
  // guideTransform replaces the shadcn-CLI migrate section with headingless
  // prose — dropping the span mirrors that on the mdx-facts side
  return raw.slice(0, span.start) + "\n" + raw.slice(span.end)
}

export function withoutInstallSection(raw) {
  const span = locateInstallSection(fenceShadow(raw))
  if (!span) return raw
  // guideTransform replaces this span with utilsInstallMdx, which itself
  // opens with a `## Installation` heading — keep that heading so the
  // heading comparison matches the built page
  return raw.slice(0, span.start) + "## Installation\n\n" + raw.slice(span.end)
}

// ---- fact extraction -----------------------------------------------------------
const attr = (attrs, name) => (attrs.match(new RegExp(`(?:^|\\s)${name}="([^"]*)"`)) || [])[1]

export function mdxPageFacts(raw, { dropCodeTabs = false, dropInstallSection = false, dropRtlMigrate = false, dropUsageSection = false, dropCompositionSection = false } = {}) {
  let src = raw
  if (dropCodeTabs) src = withoutCodeTabs(src)
  if (dropInstallSection) src = withoutInstallSection(src)
  if (dropRtlMigrate) src = withoutRtlMigrate(src)
  if (dropUsageSection) src = withoutUsageSection(src)
  if (dropCodeTabs) src = stripImportsFromMixedFences(dropReactImportFences(src))
  if (dropCompositionSection) src = withoutCompositionSection(src)
  const body = fenceShadow(src)
  // two length-stable views of the same text: markdown headings read the
  // un-blanked view (inline-code backticks are part of the text there),
  // raw <hN> tags read the inline-code-blanked view (a table cell saying
  // `<h3>` must not open a phantom raw-heading match)
  const noInlineCode = body.replace(/`[^`\n]+`/g, (m) => " ".repeat(m.length))
  const mdHeadings = [...body.matchAll(/^(#{2,4})[ \t]+(.+)$/gm)]
    .map((m) => ({ at: m.index, depth: m[1].length, text: stripInlineCode(mdText(m[2])) }))
  const rawHeadings = [...noInlineCode.matchAll(/<h([234])\b[^>]*>([\s\S]*?)<\/h\1>/g)]
    .map((m) => ({ at: m.index, depth: +m[1], text: htmlText(m[2]) }))
  const headings = [...mdHeadings, ...rawHeadings].sort((a, b) => a.at - b.at)
    .map(({ depth, text }) => ({ depth, text }))
  const previews = [...body.matchAll(/<ComponentPreview\b([^>]*)>/g)]
    .map((m) => ({ name: attr(m[1], "name"), styleName: attr(m[1], "styleName"), direction: attr(m[1], "direction") }))
  const sources = [...body.matchAll(/<ComponentSource\b([^>]*)>/g)]
    .map((m) => ({ name: attr(m[1], "name"), src: attr(m[1], "src") }))
  return { frontmatter: parseFrontmatter(raw), headings, previews, sources, fences: scanFences(src) }
}

export function htmlPageFacts(html) {
  const article = (html.match(/<article[\s\S]*?<\/article>/) || [""])[0]
  const headings = [...article.matchAll(/<h([234])\b([^>]*)>([\s\S]*?)<\/h\1>/g)].map((m) => ({
    depth: +m[1],
    id: (/id="([^"]*)"/.exec(m[2]) || [])[1] ?? null,
    shim: m[3].includes("data-slot="), // component-embedded heading (Accordion shim) — excluded from TOC
    text: htmlText(m[3]),
  }))
  const toc = [...html.matchAll(/<li class="toc-(\d)"><a href="#([^"]*)">([\s\S]*?)<\/a>/g)]
    .map((m) => ({ depth: +m[1], id: m[2], text: htmlText(m[3]) }))
  const previews = [...article.matchAll(/<div data-component-preview="([^"]*)"[^>]*data-status="([^"]*)"/g)]
    .map((m) => ({ name: m[1], status: m[2] }))
  const pres = [...article.matchAll(/<pre\b[^>]*>([\s\S]*?)<\/pre>/g)].map((m) => htmlText(m[1]))
  const iframes = [...article.matchAll(/<iframe src="([^"]*)"/g)].map((m) => m[1])
  const chips = [...html.matchAll(/<p class="links">([\s\S]*?)<\/p>/g)].flatMap((m) =>
    [...m[1].matchAll(/<a href="([^"]*)" rel="noopener">([^<]*)<\/a>/g)].map((a) => ({ href: a[1], label: htmlText(a[2]) })))
  const pnPrev = (html.match(/class="pn-prev" href="([^"]*)\.html"/) || [])[1] ?? null
  const pnNext = (html.match(/class="pn-next" href="([^"]*)\.html"/) || [])[1] ?? null
  // variant-tabs extraction stays ONLY as a retirement detector — the
  // base/aria mirror is gone; the gate FAILs if any strip reappears
  const variantTabs = [...html.matchAll(/<nav class="variant-tabs"[\s\S]*?<\/nav>/g)].map((m) => m[0])
  return {
    article,
    h1: htmlText((html.match(/<h1[^>]*>([\s\S]*?)<\/h1>/) || [])[1] ?? ""),
    lead: htmlText((html.match(/<p class="lead">([\s\S]*?)<\/p>/) || [])[1] ?? ""),
    headings, toc, previews, pres, iframes, chips, pnPrev, pnNext, variantTabs,
    allHrefs: [...html.matchAll(/href="([^"]*)"/g)].map((m) => m[1]),
    docsHrefs: [...html.matchAll(/href="(\/docs\/[^"]*)"/g)].map((m) => m[1]),
  }
}

// ---- comparison (content-level; disk-existence checks stay in the driver) ------
export const KNOWN_STATUSES = new Set(["existing-dist", "authored", "unavailable", "to-author", "tombstoned", "unknown"])

export function comparePage(m, h, { pageName, isComponentPage, expectedManualRef }) {
  const issues = []
  const issue = (kind, detail) => issues.push(`${kind}: ${detail}`)

  // 1. h1 + lead vs frontmatter
  const wantTitle = m.frontmatter.title ?? pageName
  if (h.h1 !== mdText(wantTitle)) issue("h1", `html="${h.h1}" mdx="${mdText(wantTitle)}"`)
  if (h.lead !== mdText(m.frontmatter.description ?? "")) issue("lead", `html="${h.lead.slice(0, 70)}" mdx="${mdText(m.frontmatter.description ?? "").slice(0, 70)}"`)

  // 2. headings in order (shim headings excluded on the html side)
  const wantHeads = JSON.stringify(m.headings)
  const gotHeads = JSON.stringify(h.headings.filter((x) => !x.shim).map((x) => ({ depth: x.depth, text: x.text })))
  if (wantHeads !== gotHeads) issue("headings", `mdx ${wantHeads.slice(0, 140)} != html ${gotHeads.slice(0, 140)}`)

  // 3. TOC mirrors the id'd h2/h3 exactly
  const tocable = h.headings.filter((x) => !x.shim && x.id && x.depth <= 3)
    .map((x) => ({ depth: x.depth, id: x.id, text: x.text }))
  if (JSON.stringify(h.toc) !== JSON.stringify(tocable))
    issue("toc", `toc=${h.toc.length} id-headings=${tocable.length} ${h.toc.map((t) => t.id).slice(0, 6).join(",")}|${tocable.map((t) => t.id).slice(0, 6).join(",")}`)

  // 4. preview names in order + known statuses (unknown is tolerable
  // nowhere now — every page comes from the radix catalog)
  const wantPrev = JSON.stringify(m.previews.map((p) => p.name))
  const gotPrev = JSON.stringify(h.previews.map((p) => p.name))
  if (wantPrev !== gotPrev) issue("previews", `mdx ${wantPrev.slice(0, 120)} != html ${gotPrev.slice(0, 120)}`)
  for (const p of h.previews) {
    if (!KNOWN_STATUSES.has(p.status)) issue("preview-status", `${p.name}: status "${p.status}" not emitted by the catalog`)
    if (p.status === "unknown" && isComponentPage) issue("preview-status", `${p.name}: unknown status on a component page (radix catalog must be complete)`)
  }

  // 5. every mdx fence content survives into some html <pre> (full-text,
  //    normalized — first-line matching missed nothing but false-alarmed
  //    on generics)
  for (const f of m.fences) {
    const want = mdText(f.content)
    if (!want) continue
    if (!h.pres.some((p) => p === want || p.includes(want)))
      issue("fence", `[${f.lang}] "${want.slice(0, 80)}" has no matching <pre> in the built page`)
  }

  // 6. link chips == frontmatter.links (order-insensitive, key-order-safe)
  const pair = (c) => `${c.label}→${c.href}`
  const wantChips = Object.entries(m.frontmatter.links ?? {}).map(([label, href]) => pair({ label, href })).sort()
  const gotChips = h.chips.map(pair).sort()
  if (JSON.stringify(wantChips) !== JSON.stringify(gotChips))
    issue("chips", `mdx ${wantChips.join(" | ")} != html ${gotChips.join(" | ")}`)

  // 7. rewritten manual tab references the real dist demo (component pages)
  if (expectedManualRef && !h.article.includes(expectedManualRef))
    issue("manual-tab", `rewritten manual tab never mentions ${expectedManualRef}`)

  return issues
}
