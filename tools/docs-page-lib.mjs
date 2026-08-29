// Pure page-planning helpers extracted from tools/docs-build.mjs (Wave H)
// so they're unit-testable — prevNextFor previously consulted the output
// directory mid-wipe (every component page lost its "Next" link) and
// addHeadingIds put component-embedded h3s into the TOC/clipboard handler.
export const decode = (s) => s
  .replace(/&amp;/g, '&').replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"')

export function addHeadingIds(body) {
  const used = new Map()
  const toc = []
  // tolerate headings that already carry attributes (e.g. id= from mdx
  // plugins). Headings embedded in component markup (AccordionTrigger's
  // <h3>, whose inner markup carries data-slot attrs) are SKIPPED: giving
  // them ids put FAQ questions into the page TOC and made site.js's
  // heading handler copy the URL to the clipboard on accordion toggles.
  const out = body.replace(/<h([23])(\s[^>]*)?>([\s\S]*?)<\/h\1>/g, (m, level, attrs, inner) => {
    if (/data-slot=/.test(inner)) return m
    const text = decode(inner.replace(/<[^>]+>/g, ''))
    let slug = text.toLowerCase().replace(/[^a-z0-9\s-]/g, '').trim().replace(/\s+/g, '-')
    if (!slug) slug = 'section'
    const n = used.get(slug) ?? 0
    used.set(slug, n + 1)
    if (n > 0) slug = `${slug}-${n}`
    toc.push({ depth: Number(level), text, id: slug })
    return `<h${level} id="${slug}"${attrs ?? ''}>${inner}</h${level}>`
  })
  return { body: out, toc }
}

// prev/next must consult the BUILD PLAN, not the output directory mid-wipe
// (OUT_DIR is rmSync'd then written page-by-page, so a disk existsSync saw
// only already-written pages and "next" links never existed).
export function prevNextFor(name, { sidebarOrder, mirrorSet, plannedPages, guides }) {
  if (name === 'index') return null
  if (mirrorSet.includes(name)) {
    const i = sidebarOrder.indexOf(name)
    if (i === -1) return null
    const exists = (n) => plannedPages.has(n)
    let prev = null
    for (let k = i - 1; k >= 0; k--) { if (exists(sidebarOrder[k])) { prev = sidebarOrder[k]; break } }
    let next = null
    for (let k = i + 1; k < sidebarOrder.length; k++) { if (exists(sidebarOrder[k])) { next = sidebarOrder[k]; break } }
    return { prev, next }
  }
  const guideSlugs = guides.map((g) => g.slug ?? g)
  const i = guideSlugs.indexOf(name)
  if (i === -1) return null
  return {
    prev: i > 0 ? guideSlugs[i - 1] : null,
    next: i < guideSlugs.length - 1 ? guideSlugs[i + 1] : null,
  }
}
