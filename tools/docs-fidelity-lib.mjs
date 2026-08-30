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

// The BUILT page is markdown now (tools/docs-build.mjs writes docs/components
// and docs/guides; VitePress renders them). The facts that used to be read out
// of our own HTML template — the TOC list, the prev/next pager, the breadcrumb —
// are VitePress's to generate and are gone from this comparison. What remains
// is what the CONTENT transform is responsible for.
export function mdPageFacts(md) {
  const front = parseFrontmatter(md)
  const body = md.replace(/^---\n[\s\S]*?\n---\n/, "")
  const h1 = (/^# (.+)$/m.exec(body) || [])[1] ?? ""
  const afterH1 = body.slice(body.indexOf(`# ${h1}`) + h1.length + 3)
  // markdown headings AND raw <hN> tags, in document order — the same two
  // views mdxPageFacts takes of the source, because a text transform copies
  // both through (the typography guide demonstrates heading styles with raw
  // tags and labels them with markdown ones)
  const shadow = fenceShadow(afterH1)
  const noInlineCode = shadow.replace(/`[^`\n]+`/g, (m) => " ".repeat(m.length))
  const headings = [
    ...[...shadow.matchAll(/^(#{2,4})[ \t]+(.+)$/gm)]
      .map((m) => ({ at: m.index, depth: m[1].length, text: stripInlineCode(mdText(m[2])) })),
    ...[...noInlineCode.matchAll(/<h([234])\b[^>]*>([\s\S]*?)<\/h\1>/g)]
      .map((m) => ({ at: m.index, depth: +m[1], text: htmlText(m[2]) })),
  ].sort((a, b) => a.at - b.at).map(({ depth, text }) => ({ depth, text }))
  const previews = [
    ...afterH1.matchAll(/<iframe class="demo" src="([^"]*)" title="([^"]*)" data-status="([^"]*)"/g),
  ].map((m) => ({ name: m[2], status: m[3], src: m[1] }))
  const missing = [...afterH1.matchAll(/<div class="demo-missing" data-demo="([^"]*)" data-status="([^"]*)"/g)]
    .map((m) => ({ name: m[1], status: m[2], src: null }))
  const inOrder = [...afterH1.matchAll(/<iframe class="demo"[^>]*title="([^"]*)"|<div class="demo-missing" data-demo="([^"]*)"/g)]
    .map((m) => m[1] ?? m[2])
  const byName = new Map([...previews, ...missing].map((p) => [p.name, p]))
  return {
    text: body,
    h1: mdText(h1),
    lead: mdText(front.description ?? ""),
    headings,
    previews: inOrder.map((n) => byName.get(n)),
    fences: scanFences(afterH1),
    chips: [...(/<p class="page-links">(.*)<\/p>/.exec(afterH1)?.[1] ?? "").matchAll(/<a href="([^"]*)"[^>]*>([^<]*)<\/a>/g)]
      .map((m) => ({ label: m[2], href: m[1] })),
    iframes: previews.map((p) => p.src),
    docsHrefs: [...fenceShadow(afterH1).matchAll(/\]\((\/docs\/[^)]*)\)/g)].map((m) => m[1]),
    allHrefs: [...fenceShadow(afterH1).matchAll(/\]\(([^)]*)\)/g)].map((m) => m[1]),
  }
}

// ---- comparison (content-level; disk-existence checks stay in the driver) ------
export const KNOWN_STATUSES = new Set(["existing-dist", "authored", "unavailable", "to-author", "tombstoned", "unknown"])

export function comparePage(m, h, { pageName, isComponentPage, expectedManualRef }) {
  const issues = []
  const issue = (kind, detail) => issues.push(`${kind}: ${detail}`)

  // 1. h1 + lead vs frontmatter
  const wantTitle = m.frontmatter.title ?? pageName
  if (h.h1 !== mdText(wantTitle)) issue("h1", `built="${h.h1}" mdx="${mdText(wantTitle)}"`)
  if (h.lead !== mdText(m.frontmatter.description ?? "")) issue("lead", `built="${h.lead.slice(0, 70)}" mdx="${mdText(m.frontmatter.description ?? "").slice(0, 70)}"`)

  // 2. headings in order (shim headings excluded on the html side)
  const wantHeads = JSON.stringify(m.headings)
  const gotHeads = JSON.stringify(h.headings)
  if (wantHeads !== gotHeads) issue("headings", `mdx ${wantHeads.slice(0, 140)} != built ${gotHeads.slice(0, 140)}`)

  // 3. (the TOC was ours to build and is VitePress's now — nothing to compare)

  // 4. preview names in order + known statuses (unknown is tolerable
  // nowhere now — every page comes from the radix catalog)
  const wantPrev = JSON.stringify(m.previews.map((p) => p.name))
  const gotPrev = JSON.stringify(h.previews.map((p) => p.name))
  if (wantPrev !== gotPrev) issue("previews", `mdx ${wantPrev.slice(0, 120)} != built ${gotPrev.slice(0, 120)}`)
  for (const p of h.previews) {
    if (!KNOWN_STATUSES.has(p.status)) issue("preview-status", `${p.name}: status "${p.status}" not emitted by the catalog`)
    if (p.status === "unknown" && isComponentPage) issue("preview-status", `${p.name}: unknown status on a component page (radix catalog must be complete)`)
  }

  // 5. every mdx fence survives into a fence on the built page. Weaker than it
  //    was against rendered HTML — a text transform copies fences through
  //    verbatim — but it still catches a fence eaten by a span replacement
  //    reaching past its section.
  for (const f of m.fences) {
    const want = mdText(f.content)
    if (!want) continue
    if (!h.fences.some((p) => mdText(p.content).includes(want)))
      issue("fence", `[${f.lang}] "${want.slice(0, 80)}" has no matching fence in the built page`)
  }

  // 6. link chips == frontmatter.links (order-insensitive, key-order-safe)
  const pair = (c) => `${c.label}→${c.href}`
  const wantChips = Object.entries(m.frontmatter.links ?? {}).map(([label, href]) => pair({ label, href })).sort()
  const gotChips = h.chips.map(pair).sort()
  if (JSON.stringify(wantChips) !== JSON.stringify(gotChips))
    issue("chips", `mdx ${wantChips.join(" | ")} != html ${gotChips.join(" | ")}`)

  // 7. rewritten manual tab references the real dist demo (component pages)
  if (expectedManualRef && !h.text.includes(expectedManualRef))
    issue("manual-tab", `rewritten manual tab never mentions ${expectedManualRef}`)

  return issues
}
