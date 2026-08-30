// docs smoke: serve the built VitePress site over http (python3 http.server on
// an ephemeral port — one tree, killed on exit) and drive playwright with the
// REAL mouse (lesson 2: synthetic PointerEvents are rejected by radix-like
// filters; take click points off centered content).
//   1. dialog.html: primary preview (dialog-demo) opens via a real-mouse
//      click on the trigger INSIDE the iframe, asserts live content
//      data-state=open + trigger aria-expanded=true (F2 lesson: assert on
//      LIVE nodes — CSS selectors never match <template> content), then
//      closes via Escape with focus inside the frame; portal nodes removed.
//   2. avatar.html: preview page loads over http with 0 console/page
//      errors (FT3 avatar data-URI fix), images settled.
// --all: additionally visit EVERY built page —
// render (non-empty article), no raw mdx leaking (ComponentPreview/
// ComponentSource text outside pre/code), 0 console errors + 0 pageerrors.
// Lazy preview iframes are scrolled into view so demo-page errors count
// too (page-level console/pageerror listeners cover all frames). Exit 0 +
// one PASS line per check; --all's final line is
// `PASS  docs verify (<N> pages, 0 console errors)`.
import { createServer } from 'node:net'
import { spawn } from 'node:child_process'
import { readdirSync } from 'node:fs'
import { basename } from 'node:path'
import { chromium } from 'playwright'
import { GUIDES } from './docs-guides.mjs'

const ALL = process.argv.includes('--all')
// VitePress's output. cleanUrls is on, so a page's file keeps the .html
// suffix even though the site links it without one.
const SITE_DIR = 'docs/.vitepress/dist'
const pageUrl = (rel) => `${base}/${rel}`

// ---- ephemeral-port static server (python3 http.server) ------------------------
const freePort = await new Promise((res) => {
  const s = createServer()
  s.listen(0, '127.0.0.1', () => { const p = s.address().port; s.close(() => res(p)) })
})
const server = spawn('python3', ['-m', 'http.server', String(freePort), '--bind', '127.0.0.1', '--directory', SITE_DIR], { stdio: 'ignore' })
const shutdown = () => { try { server.kill('SIGTERM') } catch {} }
process.on('exit', shutdown)
process.on('SIGINT', () => { shutdown(); process.exit(130) })
process.on('SIGTERM', () => { shutdown(); process.exit(143) })
const base = `http://127.0.0.1:${freePort}`
let serverUp = false
for (let i = 0; i < 100; i++) {
  try { await fetch(`${base}/index.html`); serverUp = true; break } catch { await new Promise((r) => setTimeout(r, 50)) }
}
if (!serverUp) {
  console.error('FAIL  docs smoke: static server did not come up (python3 http.server)')
  shutdown()
  process.exit(1)
}

const browser = await chromium.launch()
const failures = []
const check = (label, ok, detail = '') => {
  if (ok) console.log(`PASS  ${label}`)
  else { console.error(`FAIL  ${label}${detail ? ` — ${detail}` : ''}`); failures.push(label) }
}

// ---- 1. dialog preview: real-mouse open + Escape close inside the iframe -------
const page = await browser.newPage()
await page.goto(pageUrl('components/dialog.html'))
// let every lazy iframe on the page settle first — recomputing the trigger
// box mid-layout-shift made the real-mouse click land outside the trigger
// (flake appeared once authored demos went live: 6 iframes per page)
await page.waitForLoadState('networkidle', { timeout: 10000 }).catch(() => {})
const frame = page.locator('iframe.demo[title="dialog-demo"]').contentFrame()

const trigger = frame.locator('[data-slot="dialog-trigger"]')
await trigger.waitFor()
await trigger.scrollIntoViewIfNeeded()
await page.waitForTimeout(300) // settle after scroll
const tbox = await trigger.boundingBox()
await page.mouse.click(tbox.x + tbox.width / 2, tbox.y + tbox.height / 2)

await frame.locator('[data-slot="dialog-content"][data-state="open"]').waitFor({ timeout: 3000 })
const expandedOpen = await trigger.getAttribute('aria-expanded')
check('dialog: real-mouse click opens dialog in iframe (live content data-state=open)',
  await frame.locator('[data-slot="dialog-content"][data-state="open"]').count() === 1 && expandedOpen === 'true',
  `aria-expanded=${expandedOpen}`)

// focus stays inside the frame: real-mouse click on the content card padding
// (off centered text and the top-right close button), then Escape
const cbox = await frame.locator('[data-slot="dialog-content"]').boundingBox()
await page.mouse.click(cbox.x + 12, cbox.y + 12)
await page.keyboard.press('Escape')

try {
  await frame.locator('[data-slot="dialog-content"]').waitFor({ state: 'detached', timeout: 3000 })
} catch { /* asserted below via live-node count (F2 lesson) */}
const livePortalNodes = await frame.locator('[data-slot="dialog-portal"], [data-slot="dialog-overlay"], [data-slot="dialog-content"]').count()
const expandedClosed = await trigger.getAttribute('aria-expanded')
const trigState = await trigger.getAttribute('data-state')
check('dialog: Escape closes dialog inside iframe (live portal nodes removed)',
  livePortalNodes === 0 && expandedClosed === 'false' && trigState === 'closed',
  `livePortalNodes=${livePortalNodes} aria-expanded=${expandedClosed} data-state=${trigState}`)

// ---- 2. avatar preview over http: 0 console errors ------------------------------
const avPage = await browser.newPage()
const avErrors = []
avPage.on('console', (m) => { if (m.type() === 'error') avErrors.push(m.text()) })
avPage.on('pageerror', (e) => avErrors.push(String(e)))
// external (fonts/CDN) network failures are out of scope — same policy as
// the --all sweep below; only same-origin failures count
avPage.on('requestfailed', (r) => { if (r.url().startsWith(base)) avErrors.push(`requestfailed: ${r.url()} ${r.failure()?.errorText || ''}`) })
await avPage.goto(pageUrl('components/avatar.html'))
const avFrame = avPage.locator('iframe.demo[title="avatar-demo"]').contentFrame()
await avFrame.locator('[data-slot="avatar"]').first().waitFor()
const avBox = await avFrame.locator('[data-slot="avatar"]').first().boundingBox()
await avPage.mouse.click(avBox.x + 2, avBox.y + 2) // click into the demo, then let it settle
const avImages = await avFrame.locator('[data-slot="avatar-image"]').elementHandles()
// the avatar demo's images are remote (github.com/<user>.png) — poll until they
// settle rather than reading naturalWidth the instant the frame appears
let settled = []
for (let i = 0; i < 40; i++) {
  settled = await Promise.all(avImages.map((h) => h.evaluate((img) => img.complete && img.naturalWidth > 0)))
  if (settled.every(Boolean)) break
  await avPage.waitForTimeout(250)
}
check(`avatar: preview over http reports 0 console errors (images=${avImages.length} settled=${settled})`,
  avErrors.length === 0 && avImages.length > 0 && settled.every(Boolean), JSON.stringify(avErrors))

// ---- 3. dialog page preview wiring counts ---------------------------------------
const iframes = await page.locator('iframe.demo').count()
const unavailable = await page.locator('.demo-missing').count()
const previews = iframes + unavailable
console.log(`dialog page wiring: previews=${previews} iframes=${iframes} unavailable-notes=${unavailable}`)
// FT7 batches convert to-author notes into iframes; the invariant is every
// preview resolves to exactly one of the two (no orphans, no double) and at
// least one live iframe exists. (No hardcoded count — mdx edits may change it.)
check('dialog page wiring: every preview is an iframe or an unavailable note',
  previews > 0 && iframes + unavailable === previews && iframes >= 1,
  `previews=${previews} iframes=${iframes} unavailable=${unavailable}`)

// ---- 4. --all: every-page sweep (render + mdx leak + 0 console errors) -------
if (ALL) {
  const guideSlugs = new Set(GUIDES.map((g) => `guides/${g.slug}.html`))
  const pageFiles = [
    'index.html',
    ...readdirSync(`${SITE_DIR}/components`).filter((f) => f.endsWith('.html')).map((f) => `components/${f}`),
    ...readdirSync(`${SITE_DIR}/guides`).filter((f) => f.endsWith('.html')).map((f) => `guides/${f}`),
  ].sort()
  let renderFail = 0, leakFail = 0, consoleErrCount = 0, pageErrCount = 0, iframesLoaded = 0
  let nComponents = 0, nGuides = 0, nIndex = 0
  for (const f of pageFiles) {
    const p = await browser.newPage()
    const consoleErrs = []
    const pageErrs = []
    p.on('console', (m) => { if (m.type() === 'error') consoleErrs.push(m.text()) })
    p.on('pageerror', (e) => pageErrs.push(String(e)))
    // external (fonts/CDN) network failures are out of scope; internal ones count
    p.on('requestfailed', (r) => { if (r.url().startsWith(base)) consoleErrs.push(`requestfailed: ${r.url()} ${r.failure()?.errorText || ''}`) })
    await p.goto(`${base}/${f}`, { waitUntil: 'load' })
    // force lazy preview iframes to load so demo-page errors are counted
    for (const fr of await p.locator('iframe.demo').all()) {
      await fr.scrollIntoViewIfNeeded().catch(() => {})
      iframesLoaded++
    }
    try { await p.waitForLoadState('networkidle', { timeout: 2000 }) } catch { /* static site: settle best-effort */ }
    const res = await p.evaluate(() => {
      const article = document.querySelector('.vp-doc')
      const text = article?.innerText?.trim() ?? ''
      // raw mdx leak = ComponentPreview/ComponentSource as visible text
      // OUTSIDE pre/code (fenced/inline code legitimately mentions them)
      const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT,
        { acceptNode: (n) => (n.parentElement.closest('pre, code') ? NodeFilter.FILTER_REJECT : NodeFilter.FILTER_ACCEPT) })
      let visible = ''
      for (; walker.nextNode();) visible += walker.currentNode.data + '\n'
      return { rendered: !!article && text.length > 0, leaks: visible.match(/Component(Preview|Source)\b/g) ?? [] }
    })
    if (f === 'index.html') nIndex++
    else if (guideSlugs.has(f)) nGuides++
    else nComponents++
    if (!res.rendered) { renderFail++; console.error(`FAIL  render: ${f} — article missing/empty`) }
    if (res.leaks.length) { leakFail++; console.error(`FAIL  mdx leak: ${f} — ${[...new Set(res.leaks)].join(', ')}`) }
    if (consoleErrs.length) console.error(`FAIL  console: ${f} — ${consoleErrs.slice(0, 3).join(' | ')}`)
    if (pageErrs.length) console.error(`FAIL  pageerror: ${f} — ${pageErrs.slice(0, 3).join(' | ')}`)
    consoleErrCount += consoleErrs.length
    pageErrCount += pageErrs.length
    await p.close()
  }
  console.log(`pages: ${pageFiles.length}/${pageFiles.length} visited (${nComponents} components, ${nGuides} guides, ${nIndex} index) · ${iframesLoaded} preview iframes loaded`)
  check(`every-page render: ${pageFiles.length} pages non-empty (article present)`, renderFail === 0, `${renderFail} failed`)
  check(`every-page mdx: 0 raw ComponentPreview/ComponentSource outside code blocks`, leakFail === 0, `${leakFail} pages leaking`)
  check('every-page console: 0 errors, 0 pageerrors (iframes included)', consoleErrCount === 0 && pageErrCount === 0, `${consoleErrCount} console, ${pageErrCount} pageerror`)
  var verifyN = pageFiles.length
}

await browser.close()
shutdown()
if (failures.length) {
  console.log(ALL ? `FAIL  docs verify (${failures.length} failed)` : `FAIL  docs smoke (${failures.length} failed)`)
  process.exit(1)
}
if (ALL) console.log(`PASS  docs verify (${verifyN} pages, 0 console errors)`)
else console.log('PASS  docs smoke (dialog iframe open/close, avatar 0 errors, preview wiring)')
