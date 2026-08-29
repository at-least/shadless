// Upstream-fidelity gate: asserts the built docs site matches the visual
// contract of ui.shadcn.com's docs (the reference page captured for this
// work: /docs/components/aria/alert). Unlike the self-consistency gates
// (fidelity/consistency/links — mdx↔html, dist↔site), this one compares
// against UPSTREAM-derived facts so chrome/theme drift is caught by a
// machine, not by a human staring at the page.
//
// Baselines (each justified by the captured upstream HTML / shiki themes
// upstream renders with):
//   - site chrome background = shadcn neutral tokens: light oklch(1 0 0),
//     dark oklch(0.145 0 0) — and must be SEAMLESS with the iframe demos
//     (same computed background) and the preview-frame padding.
//   - code tokens: dual-theme shiki vars. Every styled span carries BOTH
//     --shiki-light and --shiki-dark; the light values ∈ github-light-
//     default's palette and dark values ∈ vesper's palette (palettes are
//     recomputed from shiki at gate runtime — no stale snapshot).
//   - line numbers: code[data-line-numbers] + data-line-numbers-max-digits,
//     counter ::before on each line span, tight line height (no blank-line
//     doubling from the trailing-newline scheme).
//   - layout regression guards: no leftover patch banners, no <details>
//     preview-source, prev/next pager AFTER the article + icon pager in
//     the header.
import { createServer } from 'node:net'
import { spawn } from 'node:child_process'
import { chromium } from 'playwright'
import { createHighlighter } from 'shiki'

const LIGHT_THEME = 'github-light-default'
const DARK_THEME = 'vesper'

// authoritative palettes straight from the themes the build tokenizes with
const highlighter = await createHighlighter({ themes: [LIGHT_THEME, DARK_THEME], langs: ['tsx'] })
const paletteOf = (name) => {
  const set = new Set([highlighter.getTheme(name).fg.toLowerCase()])
  for (const s of highlighter.getTheme(name).settings) {
    for (const c of [s.settings?.foreground, s.settings?.background]) {
      if (typeof c === 'string' && c.startsWith('#')) set.add(c.toLowerCase())
    }
  }
  return set
}
const LIGHT_PALETTE = paletteOf(LIGHT_THEME)
const DARK_PALETTE = paletteOf(DARK_THEME)

// ---- ephemeral-port static server (same pattern as docs-smoke) ----------------
const freePort = await new Promise((res) => {
  const s = createServer()
  s.listen(0, '127.0.0.1', () => { const p = s.address().port; s.close(() => res(p)) })
})
const server = spawn('python3', ['-m', 'http.server', String(freePort), '--bind', '127.0.0.1', '--directory', 'docs/site'], { stdio: 'ignore' })
const shutdown = () => { try { server.kill('SIGTERM') } catch {} }
process.on('exit', shutdown)
process.on('SIGINT', () => { shutdown(); process.exit(130) })
process.on('SIGTERM', () => { shutdown(); process.exit(143) })
const base = `http://127.0.0.1:${freePort}`
for (let i = 0; i < 100; i++) {
  try { await fetch(`${base}/index.html`); break } catch { await new Promise((r) => setTimeout(r, 50)) }
}

const browser = await chromium.launch()
const failures = []
const check = (label, ok, detail = '') => {
  if (ok) console.log(`PASS  ${label}`)
  else { console.error(`FAIL  ${label}${detail ? ` — ${detail}` : ''}`); failures.push(label) }
}

for (const mode of ['light', 'dark']) {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, colorScheme: mode })
  await page.goto(`${base}/alert.html`, { waitUntil: 'load' })
  await page.waitForTimeout(600)
  const r = await page.evaluate(() => {
    const out = {}
    out.siteBg = getComputedStyle(document.body).backgroundColor
    const fr = document.querySelector('[data-component-preview="alert-demo"] iframe')
    out.iframeBg = getComputedStyle(fr.contentDocument.body).backgroundColor
    out.frameBg = getComputedStyle(fr.closest('.preview-frame')).backgroundColor
    out.patchBanners = document.querySelectorAll('[data-patch-banner]').length
    out.detailsSource = document.querySelectorAll('details.preview-source').length
    const art = document.querySelector('article')
    const pn = document.querySelector('.page-prev-next')
    out.pagerAfterArticle = Boolean(pn) && (art.compareDocumentPosition(pn) & Node.DOCUMENT_POSITION_FOLLOWING) !== 0
    out.docsNav = document.querySelectorAll('.docs-nav a').length
    out.codeFacts = [...document.querySelectorAll('article pre code')].map((c) => ({
      numbered: c.hasAttribute('data-line-numbers'),
      digits: c.getAttribute('data-line-numbers-max-digits'),
      lines: c.querySelectorAll(':scope > span[data-line]').length,
      lightVars: [...c.querySelectorAll('span[style]')].map((s) => s.style.getPropertyValue('--shiki-light')).filter(Boolean),
      darkVars: [...c.querySelectorAll('span[style]')].map((s) => s.style.getPropertyValue('--shiki-dark')).filter(Boolean),
    }))
    const code = document.querySelector('article pre code[data-line-numbers]')
    const line = code.querySelector(':scope > span[data-line]:nth-child(2)')
    const before = getComputedStyle(line, '::before')
    out.before = { content: before.content, incr: before.counterIncrement }
    out.lineH = Math.round(line.getBoundingClientRect().height)
    return out
  })

  // 1. chrome background = shadcn neutral, seamless with demos
  const wantBg = mode === 'light' ? 'oklch(1 0 0)' : 'oklch(0.145 0 0)'
  check(`[${mode}] chrome background is shadcn neutral (${wantBg})`, r.siteBg === wantBg, `got ${r.siteBg}`)
  check(`[${mode}] preview seam: site == frame == iframe backgrounds`,
    r.siteBg === r.frameBg && r.siteBg === r.iframeBg, `site=${r.siteBg} frame=${r.frameBg} iframe=${r.iframeBg}`)

  // 2. code tokens: dual vars, palettes match the upstream themes
  const lights = new Set(r.codeFacts.flatMap((c) => c.lightVars.map((v) => v.toLowerCase())))
  const darks = new Set(r.codeFacts.flatMap((c) => c.darkVars.map((v) => v.toLowerCase())))
  check(`[${mode}] every styled token carries BOTH theme vars`,
    r.codeFacts.every((c) => c.lightVars.length === c.darkVars.length) && lights.size > 0,
    `light=${lights.size} dark=${darks.size}`)
  const badLight = [...lights].filter((c) => !LIGHT_PALETTE.has(c))
  const badDark = [...darks].filter((c) => !DARK_PALETTE.has(c))
  check(`[${mode}] light token colors ⊆ ${LIGHT_THEME}`, badLight.length === 0, `foreign: ${badLight.join(',')}`)
  check(`[${mode}] dark token colors ⊆ ${DARK_THEME}`, badDark.length === 0, `foreign: ${badDark.join(',')}`)

  // 3. line numbers
  check(`[${mode}] all code blocks line-numbered with digits attr`,
    r.codeFacts.length > 0 && r.codeFacts.every((c) => c.numbered && c.digits && +c.digits >= 1),
    `blocks=${r.codeFacts.length}`)
  check(`[${mode}] gutter counter renders`,
    r.before.content === 'counter(ln)' && r.before.incr.includes('ln'),
    `content=${r.before.content} incr=${r.before.incr}`)
  check(`[${mode}] line height tight (no newline doubling)`, r.lineH <= 30, `lineH=${r.lineH}px`)

  // 4. layout regression guards
  check(`[${mode}] no leftover patch banners`, r.patchBanners === 0, `count=${r.patchBanners}`)
  check(`[${mode}] no <details> preview-source`, r.detailsSource === 0)
  check(`[${mode}] prev/next pager after article + icon pager in header`,
    r.pagerAfterArticle && r.docsNav === 2, `pagerAfter=${r.pagerAfterArticle} docsNav=${r.docsNav}`)
  await page.close()
}

// ---- custom-color demos: dark-mode palette flip (upstream uses class+
//      dark:variant pairs, not hardcoded light hex) -------------------------
{
  // oklch lightness bounds: amber-50 ≈ .987, amber-900 ≈ .414,
  // amber-950 ≈ .279, blue-50 ≈ .970, blue-950 ≈ .282
  const L = (bg) => parseFloat(/^oklch\(([\d.]+) /.exec(bg)?.[1] ?? 'NaN')
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } })
  await page.goto(`${base}/alert.html`, { waitUntil: 'load' })
  await page.waitForTimeout(600)
  const facts = {}
  for (const [mode, scheme] of [['light', 'light'], ['dark', 'dark']]) {
    await page.emulateMedia({ colorScheme: scheme })
    await page.reload()
    await page.waitForTimeout(500)
    facts[mode] = await page.evaluate(() => {
      const fr = document.querySelector('[data-component-preview="alert-colors"] iframe')
      const a = getComputedStyle(fr.contentDocument.querySelector('[data-slot="alert"]'))
      return { bg: a.backgroundColor, color: a.color, maxW: a.maxWidth }
    })
  }
  check('alert-colors: light = amber-50 bg / amber-900 text',
    L(facts.light.bg) > 0.95 && L(facts.light.color) < 0.55,
    `bg=${facts.light.bg} text=${facts.light.color}`)
  check('alert-colors: dark = amber-950 bg / amber-50 text (flips)',
    L(facts.dark.bg) < 0.35 && L(facts.dark.color) > 0.9,
    `bg=${facts.dark.bg} text=${facts.dark.color}`)
  check('alert-colors: max-w-md preserved', facts.dark.maxW === '448px' || facts.dark.maxW === '28rem', facts.dark.maxW)

  await page.goto(`${base}/badge.html`, { waitUntil: 'load' })
  const bf = {}
  for (const [mode, scheme] of [['light', 'light'], ['dark', 'dark']]) {
    await page.emulateMedia({ colorScheme: scheme })
    await page.reload()
    await page.waitForTimeout(500)
    bf[mode] = await page.evaluate(() => {
      const fr = document.querySelector('[data-component-preview="badge-colors"] iframe')
      return getComputedStyle(fr.contentDocument.querySelector('[data-slot="badge"]')).backgroundColor
    })
  }
  check('badge-colors: light = blue-50, dark = blue-950 (flips)',
    L(bf.light) > 0.9 && L(bf.dark) < 0.35, `light=${bf.light} dark=${bf.dark}`)

  // field-demo: group/fieldset structure must match upstream's nova skin.
  // The root fieldset ([data-slot="field-set"]) has cn-field-set with
  // gap-4; the inner field-group ([data-slot="field-group"]) has gap-5.
  await page.goto(`${base}/field.html`, { waitUntil: "load" })
  const ffg = {}
  for (const [mode, scheme] of [['light', 'light'], ['dark', 'dark']]) {
    await page.emulateMedia({ colorScheme: scheme })
    await page.reload()
    await page.waitForTimeout(500)
    ffg[mode] = await page.evaluate(() => {
      const fr = document.querySelector('[data-component-preview="field-demo"] iframe')
      const set = fr.contentDocument.querySelector('[data-slot="field-set"]')
      const grp = fr.contentDocument.querySelector('[data-slot="field-group"]')
      return {
        setGap: getComputedStyle(set).gap,
        grpGap: getComputedStyle(grp).gap,
      }
    })
  }
  check('field-demo: field-set gap = 16px, field-group gap = 20px (nova skin)',
    ffg.light.setGap === "16px" && ffg.light.grpGap === "20px" && ffg.dark.setGap === "16px" && ffg.dark.grpGap === "20px",
    JSON.stringify(ffg))
  await page.close()
}

// ---- AlertAction geometry (upstream: absolute top-2 right-2, root pr-18) ----
{
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } })
  await page.goto(`${base}/alert.html`, { waitUntil: 'load' })
  await page.waitForTimeout(600)
  const g = await page.evaluate(() => {
    const fr = document.querySelector('[data-component-preview="alert-action"] iframe')
    const doc = fr.contentDocument
    const alert = doc.querySelector('[data-slot="alert"]')
    const action = doc.querySelector('[data-slot="alert-action"]')
    if (!alert || !action) return { missing: true }
    const a = alert.getBoundingClientRect()
    const b = action.getBoundingClientRect()
    const cs = getComputedStyle(action)
    const acs = getComputedStyle(alert)
    return {
      missing: false,
      position: cs.position,
      padRight: parseFloat(acs.paddingRight),
      // distance of the action's box from the alert's top-right corner
      dTop: Math.round(b.top - a.top),
      dRight: Math.round(a.right - b.right),
      inside: b.top >= a.top && b.right <= a.right && b.bottom <= a.bottom,
    }
  })
  check('alert-action: present in demo iframe', !g.missing)
  if (!g.missing) {
    // top-2/right-2 = 8px; pr-18 = 72px at --spacing 0.25rem
    check('alert-action: absolutely positioned (top-right corner)', g.position === 'absolute', `position=${g.position}`)
    check('alert-action: sits ~8px from top and right edges', g.dTop <= 10 && g.dRight <= 10, `dTop=${g.dTop}px dRight=${g.dRight}px`)
    check('alert-action: inside alert bounds', g.inside, `dTop=${g.dTop} dRight=${g.dRight}`)
    check('alert root reserves right padding (pr-18 ≈ 72px)', Math.abs(g.padRight - 72) <= 2, `padRight=${g.padRight}px`)
  }
  await page.close()
}

await browser.close()
shutdown()
if (failures.length) {
  console.error(`\nFAIL  docs upstream fidelity (${failures.length} checks)`)
  process.exit(1)
}
console.log('\nPASS  docs upstream fidelity (chrome/theme/line-numbers match the upstream contract)')
