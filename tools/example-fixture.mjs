#!/usr/bin/env node
// example-fixture — INTERACTIVE pages for dialog-family kernel examples.
//
// Problem it fixes (user report 2026-08-27): every kernel-tier example
// beyond the bare -demo shipped as an oracle-rendered STATIC snapshot —
// zero scripts, dead buttons. The oracle render gives the closed state
// only; dialog content is unmounted in React, so the static page has
// nothing to open.
//
// Generation per example: oracle-render the CLOSED state (page shell +
// trigger), then CLICK the trigger and harvest the MOUNTED overlay +
// content nodes radix appended to <body> — assemble a fixture page:
// closed markup + <template id="d1-portal"> + vendored kernel + the
// component's glue. Same wireDialog protocol the t6 fixtures use
// (#d1-trigger, #d1-portal, data-slot lookups), so the existing glue
// drives it untouched.
//
// Self-verifying: write to the final path, click through (open → content
// present → dismiss → closed); on any failure the page is DELETED and the
// example reported — a dead page can never land. --check regenerates and
// byte-compares against the committed page.
//
// Families: the dialog trio (single-instance #d1-* protocol), popover /
// tooltip / hover-card (portal, one template per trigger, multi-instance
// glue), tabs (inline panels harvested per trigger), slider / scroll-area
// (scripts only). Menus, select and navigation-menu still need their
// protocol mapped.
import { chromium } from "playwright"
import { readFileSync, writeFileSync, rmSync, readdirSync } from "node:fs"
import { resolve } from "node:path"
import { buildOracle, awaitOracle } from "./oracle-lib.mjs"
import { buildContractOracle } from "./contracts/oracle-build.mjs"
import { THEME_PREPAINT_SCRIPT } from "../src/docs/theme-prepaint.mjs"
// trivial components present on the page (by content, from example-oracle)
// load their own behavior files next to the kernel families'
const trivialScripts = (target, jsdir) => (target.trivial ?? []).map((c) => `<script src="${jsdir}${c}.js"></script>`).join("\n")

const EXAMPLES = ".upstream/shadcn-ui/apps/v4/examples/radix"
const TMP = "build/example-fixture"
const CHECK = process.argv.includes("--check")
// glue file + how an instance is opened + where its content appears.
//   portal   content mounts as new <body> children (harvested into a
//            <template id="<k>-portal"> per trigger; multi-instance glue
//            resolves it from the trigger id "<k>-trigger")
//   inline   content lives inside the root but only the active panel is
//            mounted (tabs): every panel is harvested by activating each
//            trigger and reassembled into the root
//   none     the closed markup already carries everything; the glue only
//            needs the scripts (slider, scroll-area)
// The dialog family keeps its original single-instance protocol (#d1-*).
// FAMILY — the kernel protocol table, shared with the docs (tools/fixture-families.mjs)
import { FAMILY } from "./fixture-families.mjs"
const componentOf = (name) =>
  Object.keys(FAMILY).sort((a, b) => b.length - a.length).find((c) => name === c || name.startsWith(c + "-"))

// targets: every example page whose rendered DOM carries a kernel family —
// decided by tools/example-oracle.mjs by CONTENT (docs/example-fixture-
// targets.json), so a dropdown inside an avatar example lands here too.
// Families the page carries that this tool cannot wire yet are reported.
// --contracts: the contract defs are the targets. Their React usage trees
// (tools/contracts/components/*.mjs) render through the same oracle bundle
// the contracts runner uses, and the fixture is written to def.shadlessPage
// (src/kernel/<name>.html) with the ids the usage tree already carries
// (#d1-trigger → d1-portal / d1-tpl). Controlled-open trees (dialog family)
// are closed first with window.__setOpen(false). Assets are linked from the
// fixture's own directory (../../dist, ../../vendor, same-dir glue) — the
// contracts runner rewrites them to absolute paths, demo.mjs to dist-relative.
const CONTRACTS = process.argv.includes("--contracts")
const contractTargets = async () => {
  const out = []
  for (const f of readdirSync("tools/contracts/components").filter((x) => x.endsWith(".mjs")).sort()) {
    const def = (await import(`./contracts/components/${f}`)).default
    const comp = f.slice(0, -4).replace(/-multiple$/, "")
    if (!FAMILY[comp] || !def.shadlessPage.startsWith("src/kernel/")) continue
    out.push({ name: f.slice(0, -4), families: [comp], unsupported: [], trivial: [], def })
  }
  return out
}
const targets = CONTRACTS ? await contractTargets() : JSON.parse(readFileSync("docs/example-fixture-targets.json", "utf8"))
  .map((t) => ({ name: t.name, trivial: t.trivial ?? [],
    // dialog kinds annotate a closed-markup COPY (regex) and must run last so
    // the copy already carries the other families' live-DOM annotations
    families: t.families.filter((f) => FAMILY[f]).sort((a, b) => (FAMILY[a].kind === "dialog") - (FAMILY[b].kind === "dialog")),
    unsupported: t.families.filter((f) => !FAMILY[f]) }))

// The self-test needs the page's relative assets (../out.css, ../shadless.js,
// ../js/*) to resolve, and docs/demos carries none — so it renders from a
// scratch tree under build/ assembled from what this build just produced.
//
// It used to render from docs/site instead, which made a COMMITTED build
// artifact a prerequisite of the build: docs-build writes that tree and runs
// AFTER this step, so the stylesheet the self-test loaded was whatever the
// last build had left in git. out.css still comes from the previous build
// (demo-css runs later too) but now from dist/, the tree that is committed on
// purpose.
import { mkdirSync, copyFileSync, existsSync } from "node:fs"
const SELFTEST = "build/fixture"
mkdirSync(`${SELFTEST}/pages`, { recursive: true })
mkdirSync(`${SELFTEST}/js`, { recursive: true })
for (const f of readdirSync("dist/js")) copyFileSync(`dist/js/${f}`, `${SELFTEST}/js/${f}`)
copyFileSync("dist/shadless.js", `${SELFTEST}/shadless.js`)
// absent only in a tree that has never been built — the page then renders
// unstyled, exactly as it did when docs/site/out.css was missing
if (existsSync("dist/out.css")) copyFileSync("dist/out.css", `${SELFTEST}/out.css`)

const browser = await chromium.launch()
const page = await browser.newPage()
await page.route(/^(https?:)?\/\//, (r) => r.abort())

let emitted = 0
const failures = []
const unsupported = []
for (const target of targets) {
  const { name } = target
  if (!target.families.length) { unsupported.push(`${name}: ${target.unsupported.join(", ")}`); continue }
  if (target.unsupported.length) unsupported.push(`${name}: ${target.unsupported.join(", ")} (page wired for ${target.families.join(", ")} only)`)
  const outPath = CONTRACTS ? target.def.shadlessPage : `docs/demos/${name}.html`
  // the JS surface: one base + one file per component (dist/shadless.js,
  // dist/js/<name>.js); contract fixtures link dist from src/kernel/
  const assetBase = CONTRACTS ? { css: "../../dist/out.css", base: "../../dist/shadless.js", js: "../../dist/js/" } : { css: "../out.css", base: "../shadless.js", js: "../js/" }
  try {
    if (CONTRACTS) {
      const OUT = `${TMP}/contracts/${name}`
      await buildContractOracle(target.def, OUT)
      await page.goto(`file://${resolve(OUT)}/oracle.html`); await page.waitForTimeout(600)
      // controlled-open trees mount their content at first render — close
      await page.evaluate(() => { if (typeof window.__setOpen === "function" && window.__open) window.__setOpen(false) })
      await page.waitForTimeout(400)
    } else {
      const { htmlFile } = await buildOracle(name, { tmp: TMP })
      await awaitOracle(page, htmlFile)
    }
    const dir = await page.evaluate(() => document.documentElement.getAttribute("dir") || "ltr")
    // several families on one page: each contributes templates and its glue;
    // the first family's self-test proves the page
    const jsFiles = [...new Set(target.families.map((f) => FAMILY[f].js).filter(Boolean))]
    const allTemplates = [], selfTests = []
    let bodyHtml
    const shell = (inner) => `<!doctype html>
<html${dir === "rtl" ? ' dir="rtl"' : ""}><head><meta charset="utf-8"><title>shadless ${name}</title>
<link rel="stylesheet" href="${assetBase.css}">${THEME_PREPAINT_SCRIPT}</head>
<body class="p-8">
${inner}
<script src="${assetBase.base}"></script>
${jsFiles.map((g) => `<script src="${assetBase.js}${g}.js"></script>`).join("\n")}
${trivialScripts(target, assetBase.js)}
</body></html>
`
    for (const comp of target.families) {
    const fam = FAMILY[comp]
    let templates = "", selfTest
    // radix auto ids go; so do the aria-hidden marks radix's modal layer
    // leaves on the outside content (a controlled-open tree closed via
    // window.__setOpen restores them asynchronously — never part of the fixture)
    const stripRadixIds = (h) => h.replace(/\s(?:id|aria-controls|aria-labelledby|aria-describedby)="radix-[^"]*"/g, "")
      .replace(/\s(?:aria-hidden|data-aria-hidden)="true"/g, "")
    // Radix's auto ids become STABLE fixture ids by mapping, not deletion:
    // every reference (aria-controls on the trigger, aria-labelledby on the
    // content, …) then exists exactly where the oracle has it. Unmapped
    // radix ids are stripped afterwards. `idMap` accumulates per page.
    const idMap = new Map()
    const remap = (h) => h.replace(/radix-[\w:-]*/g, (id) => idMap.get(id) ?? id)
    const learn = (html, slotToStable) => {
      let base = null
      for (const [slot, stable] of Object.entries(slotToStable)) {
        // attribute order is radix's, not ours: find the tag, then its id
        const tag = new RegExp('<[^>]*data-slot="' + slot + '"[^>]*>').exec(html)?.[0]
        const m = tag && /\sid="(radix-[^"]*)"/.exec(tag)
        if (m) idMap.set(m[1], stable)
        base ??= stable
      }
      // every other radix id inside the layer (select-item text ids, menu
      // labels, the tooltip's sr-only duplicate) gets a stable derived id so
      // the references between them survive
      let n = 0
      for (const m of html.matchAll(/\sid="(radix-[^"]*)"/g)) if (!idMap.has(m[1])) idMap.set(m[1], `${base ?? "x"}-e${n++}`)
      // a reference to an id that exists nowhere in the layer is radix's
      // internal id for the TRIGGER (its DOM id was overridden by the usage
      // tree's id prop): point it at the stable trigger id
      const triggerStable = base ? `${base.replace(/s\d+$/, "")}-trigger` : null
      for (const m of html.matchAll(/aria-labelledby="(radix-[^"]*)"/g)) if (!idMap.has(m[1]) && triggerStable) idMap.set(m[1], triggerStable)
    }
    const harvestAdded = async (before) => page.evaluate((n) => {
      const added = [...document.body.children].slice(n)
        .filter((el) => el.tagName !== "SCRIPT" && !el.hasAttribute("data-radix-focus-guard"))
      return added.length ? added.map((el) => el.outerHTML).join("\n") : null
    }, before)

    if (fam.kind === "dialog") {
      const closedHtml = await page.evaluate(() => document.querySelector("#root").innerHTML)
      const before = await page.evaluate(() => document.body.children.length)
      await page.click('[data-slot$="-trigger"]')
      await page.waitForTimeout(600)
      const portalHtml = await harvestAdded(before)
      if (!portalHtml) throw new Error("no mounted overlay/content after trigger click")
      // protocol rewrite: stable fixture ids, drop radix auto ids.
      // NB: the slot patterns must be built with new RegExp — a regex LITERAL
      // treats ${comp} as literal text and never matches (the exact bug that
      // shipped id-less dead pages on the first run)
      const re = (slot) => new RegExp('(<[^>]*data-slot="' + comp + '-' + slot + '"[^>]*?)>')
      learn(portalHtml, { [`${comp}-content`]: "d1", [`${comp}-title`]: "d1-title", [`${comp}-description`]: "d1-desc" })
      const trigOrig = await page.evaluate((sel) => document.querySelector("#root " + sel)?.id, `[data-slot="${comp}-trigger"]`)
      if (trigOrig && trigOrig.startsWith("radix-")) idMap.set(trigOrig, "d1-trigger")
      const fixed = stripRadixIds(remap(portalHtml))
      bodyHtml = stripRadixIds(remap(closedHtml)).replace(re("trigger"), (m, open) => /\sid="/.test(open) ? m : `${open} id="d1-trigger">`)
      templates = `<template id="d1-portal">\n${fixed}\n</template>`
      selfTest = async () => {
        await page.click("#d1-trigger"); await page.waitForTimeout(500)
        // this family's content precisely — a static *-content slot on the
        // page (attachment-content) must not read as "still open"
        const mine = `[data-slot="${comp}-content"]`
        if (!await page.evaluate((sel) => !!document.querySelector(sel), mine)) throw new Error("did not open")
        // dismissal mirrors what the glue wires: content's direct-child X,
        // then action/cancel, then Escape (no-close-button examples have neither)
        await page.evaluate(() => {
          const btn = document.querySelector('[data-slot$="-content"] > button')
            ?? document.querySelector('[data-slot$="-action"], [data-slot$="-cancel"]')
          if (btn) btn.click()
          else document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }))
        })
        await page.waitForTimeout(500)
        if (!await page.evaluate((sel) => !document.querySelector(sel), mine)) throw new Error("did not close")
      }
    } else if (fam.kind === "portal") {
      // one template per trigger instance: open each in the live React page,
      // harvest what radix appended to <body>, close it again
      const triggerSel = `[data-slot="${comp}-trigger"]`
      const count = await page.evaluate((sel) => document.querySelectorAll("#root " + sel).length, triggerSel)
      if (!count) throw new Error("no trigger in the oracle render")
      const parts = []
      const contentSel = `[data-slot="${comp}-content"]`
      // find the newly mounted content by slot + harvested marker, not by
      // body-child count: a TooltipProvider swaps one tooltip for the next
      // (unmount + mount, count unchanged) and sub layers land before the
      // trailing focus guard
      const mounted = async () => page.evaluate((sel) => {
        const c = document.querySelector(sel + ":not([data-ef-harvested])")
        if (!c) return null
        const top = c.closest("body > *") || c
        const html = top.outerHTML
        c.setAttribute("data-ef-harvested", "")
        return html
      }, contentSel)
      const origIds = await page.evaluate((sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id), triggerSel)
      const prefixOf = (i) => /^(\w+)-trigger$/.exec(origIds[i])?.[1] ?? `k${i}`
      const refs = []
      for (let i = 0; i < count; i++) {
        const el = (await page.$$("#root " + triggerSel))[i]
        const act = async () => {
          if (fam.open === "hover") {
            // approach from BELOW the row: a horizontal path from the page
            // corner crosses sibling triggers (kbd-tooltip: two triggers on
            // one line) and radix's provider then keeps the first one
            await page.mouse.move(2, 2); await page.waitForTimeout(400)
            const box = await el.boundingBox()
            await page.mouse.move(box.x + box.width / 2, box.y + box.height + 60); await page.waitForTimeout(300)
            await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2, { steps: 6 })
            await page.waitForTimeout(1100) // radix TooltipProvider / HoverCard open delays (700ms)
          } else { await el.click(); await page.waitForTimeout(500) }
        }
        await act()
        let portalHtml = await mounted()
        if (!portalHtml) { await act(); portalHtml = await mounted() }
        if (!portalHtml) throw new Error(`instance ${i}: nothing mounted after ${fam.open}`)
        learn(portalHtml, { [`${comp}-content`]: prefixOf(i) })
        if (origIds[i].startsWith("radix-")) idMap.set(origIds[i], `${prefixOf(i)}-trigger`)
        // does the open trigger point at its content (aria-describedby on a
        // tooltip trigger, aria-controls on a popover's)? Then the glue will
        // publish content.id and the content root needs one; a hover-card's
        // does not, and an id the oracle lacks is a diff
        refs[i] = await el.evaluate((e) => e.hasAttribute("aria-describedby") || e.hasAttribute("aria-controls"))
        parts.push(portalHtml.replace(/\sdata-ef-harvested=""/g, ""))
        await page.keyboard.press("Escape")
        await page.mouse.move(0, 0); await page.waitForTimeout(700)
      }
      // ids by instance order: k<i>-trigger ↔ k<i>-portal
      await page.evaluate(({ sel, ids }) => document.querySelectorAll("#root " + sel).forEach((t, i) => { t.id = ids[i] }), { sel: triggerSel, ids: origIds.map((_, i) => `${prefixOf(i)}-trigger`) })
      const prefixes = origIds.map((_, i) => prefixOf(i))
      bodyHtml = stripRadixIds(remap(await page.evaluate(() => document.querySelector("#root").innerHTML)))
      const ensureContentId = (h, id) => h.replace(new RegExp('(<[^>]*data-slot="' + comp + '-content"(?:(?!\\sid=)[^>])*?)>'), `$1 id="${id}">`)
      templates = parts.map((p, k) => `<template id="${prefixes[k]}-portal">\n${refs[k] ? ensureContentId(stripRadixIds(remap(p)), prefixes[k]) : stripRadixIds(remap(p))}\n</template>`).join("\n")
      selfTest = async () => {
        const t = await page.$(`#${prefixes[0]}-trigger`)
        if (fam.open === "hover") { const b = await t.boundingBox(); await page.mouse.move(b.x + b.width / 2, b.y + b.height + 60); await page.waitForTimeout(300); await page.mouse.move(b.x + b.width / 2, b.y + b.height / 2, { steps: 6 }); await page.waitForTimeout(1100) }
        else { await t.click(); await page.waitForTimeout(500) }
        if (!await page.evaluate((c) => !!document.querySelector(`[data-slot="${c}-content"]`), comp)) throw new Error("did not open")
        await page.keyboard.press("Escape"); await page.mouse.move(0, 0); await page.waitForTimeout(700)
        if (!await page.evaluate((c) => !document.querySelector(`[data-slot="${c}-content"]`), comp)) throw new Error("did not close")
      }
    } else if (fam.kind === "menu" || fam.kind === "select") {
      const isSelect = fam.kind === "select"
      const triggerSel = `[data-slot="${comp}-trigger"]`
      const contentSel = `[data-slot="${comp}-content"]`
      const subTriggerSel = `[data-slot="${comp}-sub-trigger"]`
      // disabled triggers (select-disabled) open nothing: skip them; a page
      // with no openable trigger ships its closed markup + scripts only
      const enabled = await page.evaluate((sel) => [...document.querySelectorAll("#root " + sel)].map((t) => !t.disabled && t.getAttribute("aria-disabled") !== "true" && !t.hasAttribute("data-disabled")), triggerSel)
      const count = enabled.length
      if (!count) throw new Error("no trigger in the oracle render")
      const templates_ = []
      // radix mounts a layer as <div data-radix-popper-content-wrapper> in
      // <body> — NOT necessarily as the last child: sub layers land before
      // the trailing focus-guard span, so "body children added since N" misses
      // them (the first version harvested a focus guard). Layers are found
      // by slot and marked once harvested.
      const mountedContent = async (sel) => page.evaluate((sel) => !!document.querySelector(sel + ":not([data-ef-harvested])"), sel)
      const harvestLayer = async (layerId, sel) => {
        if (!await mountedContent(sel)) throw new Error(`layer ${layerId}: nothing mounted`)
        const { html, subCount } = await page.evaluate(({ sel, sub, layerId }) => {
          const content = document.querySelector(sel + ":not([data-ef-harvested])")
          const subs = [...content.querySelectorAll(sub)]
          subs.forEach((t, j) => { if (t.id && t.id.startsWith("radix-")) t.setAttribute("data-ef-orig", t.id); t.setAttribute("data-radixuigo-menu-subtrigger", `${layerId}s${j}`); t.id = `${layerId}s${j}-trigger` })
          const html = content.outerHTML
          content.setAttribute("data-ef-harvested", "")
          return { html, subCount: subs.length }
        }, { sel, sub: subTriggerSel, layerId })
        learn(html, { [`${comp}-content`]: layerId, [`${comp}-sub-content`]: layerId })
        templates_.push({ layerId, html: html.replace(/\sdata-ef-harvested=""/g, "") })
        for (let j = 0; j < subCount; j++) {
          const st = await page.$(`#${layerId}s${j}-trigger`)
          const box = await st.boundingBox()
          // radix opens a sub menu on pointer movement over its trigger, or on
          // ArrowRight from the focused trigger; pointer first
          await page.mouse.move(box.x + 4, box.y + box.height / 2, { steps: 3 })
          await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2, { steps: 6 })
          await page.waitForTimeout(600)
          const subSel = `[data-slot="${comp}-sub-content"]`
          if (!await mountedContent(subSel)) { await st.focus(); await page.keyboard.press("ArrowRight"); await page.waitForTimeout(500) }
          await harvestLayer(`${layerId}s${j}`, subSel)
          // back to the parent layer: point away from the sub-trigger row
          await page.mouse.move(box.x + box.width / 2, box.y - 40, { steps: 4 }); await page.waitForTimeout(300)
        }
      }
      const existing = await page.evaluate((sel) => [...document.querySelectorAll("#root " + sel)].map((t) => /^(\w+)-trigger$/.exec(t.id)?.[1] ?? null), triggerSel)
      const menuOrig = await page.evaluate((sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id), triggerSel)
      const idOf = (i) => existing[i] ?? (isSelect ? `s${i}` : `m${i}`)
      menuOrig.forEach((o, i) => { if (o.startsWith("radix-")) idMap.set(o, `${idOf(i)}-trigger`) })
      for (let i = 0; i < count; i++) {
        if (!enabled[i]) continue
        const id = idOf(i)
        const el = (await page.$$("#root " + triggerSel))[i]
        if (fam.open === "contextmenu") { const b = await el.boundingBox(); await page.mouse.click(b.x + b.width / 2, b.y + b.height / 2, { button: "right" }) }
        else await el.click()
        await page.waitForTimeout(500)
        await harvestLayer(id, contentSel)
        // close everything (Escape per open layer)
        for (let k = 0; k < 4; k++) { await page.keyboard.press("Escape"); await page.waitForTimeout(150) }
        await page.waitForTimeout(400)
      }
      await page.evaluate(({ sel, isSelect, attr, ids }) => document.querySelectorAll("#root " + sel).forEach((t, i) => {
        const id = ids[i]
        t.id = `${id}-trigger`; if (!isSelect) t.setAttribute(attr, id)
      }), { sel: triggerSel, isSelect, attr: fam.attr, ids: existing.map((e, i) => idOf(i)) })
      bodyHtml = stripRadixIds(remap(await page.evaluate(() => document.querySelector("#root").innerHTML)))
      // sub-trigger original ids (recorded in-page) map to their stable ids
      for (const t of templates_) for (const m of t.html.matchAll(/data-ef-orig="(radix-[^"]*)"[^>]*\sid="([^"]+)"/g)) idMap.set(m[1], m[2])
      templates = templates_.map((t) => `<template id="${t.layerId}-tpl">\n${stripRadixIds(remap(t.html)).replace(/\sdata-ef-orig="[^"]*"/g, "")}\n</template>`).join("\n")
      const firstEnabled = enabled.indexOf(true)
      selfTest = async () => {
        if (firstEnabled < 0) return // nothing openable by design
        const t = await page.$(`#${idOf(firstEnabled)}-trigger`)
        if (fam.open === "contextmenu") { const b = await t.boundingBox(); await page.mouse.click(b.x + b.width / 2, b.y + b.height / 2, { button: "right" }) }
        else await t.click()
        await page.waitForTimeout(500)
        if (!await page.evaluate((sel) => !!document.querySelector(sel + "[data-state=open]"), contentSel)) throw new Error("did not open")
        // the first sub menu must open from its trigger too (kernel hover path)
        const sub = await page.$(`[data-slot="${comp}-sub-trigger"]`)
        if (sub) {
          const b = await sub.boundingBox()
          await page.mouse.move(b.x + 4, b.y + b.height / 2, { steps: 3 }); await page.mouse.move(b.x + b.width / 2, b.y + b.height / 2, { steps: 6 })
          await page.waitForTimeout(600)
          if (!await page.evaluate((c) => !!document.querySelector(`[data-slot="${c}-sub-content"]`), comp)) throw new Error("sub menu did not open")
          await page.keyboard.press("Escape"); await page.waitForTimeout(300)
        }
        await page.keyboard.press("Escape"); await page.waitForTimeout(500)
        if (await page.evaluate((sel) => !!document.querySelector(sel + "[data-state=open]"), contentSel)) throw new Error("did not close")
      }
    } else if (fam.kind === "nav") {
      const triggerSel = '[data-slot="navigation-menu-trigger"]'
      const contentSel = '[data-slot="navigation-menu-content"]'
      const count = await page.evaluate((sel) => document.querySelectorAll("#root " + sel).length, triggerSel)
      if (!count) throw new Error("no trigger in the oracle render")
      const parts = []
      const navOrig = await page.evaluate((sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id), triggerSel)
      const navIdOf = (i) => /^(\w+)-trigger$/.exec(navOrig[i])?.[1] ?? `n${i}`
      for (let i = 0; i < count; i++) {
        const el = (await page.$$("#root " + triggerSel))[i]
        await el.click(); await page.waitForTimeout(500)
        const html = await page.evaluate((sel) => {
          const c = document.querySelector(sel + ":not([data-ef-harvested])")
          if (!c) return null
          const html = c.outerHTML; c.setAttribute("data-ef-harvested", ""); return html
        }, contentSel)
        if (!html) throw new Error(`instance ${i}: no content mounted after click`)
        learn(html, { "navigation-menu-content": `${navIdOf(i)}-content` })
        if (navOrig[i].startsWith("radix-")) idMap.set(navOrig[i], `${navIdOf(i)}-trigger`)
        parts.push(html.replace(/\sdata-ef-harvested=""/g, ""))
        await page.keyboard.press("Escape"); await page.waitForTimeout(400)
      }
      await page.evaluate((sel) => {
        document.querySelectorAll("#root " + sel).forEach((t, i) => { const id = /^(\w+)-trigger$/.exec(t.id)?.[1] ?? `n${i}`; t.id = `${id}-trigger`; t.setAttribute("data-radixuigo-nav-trigger", id) })
        // the viewport React left behind (if any) is the glue's to create
        document.querySelectorAll('#root [data-slot="navigation-menu-viewport"]').forEach((v) => v.remove())
      }, triggerSel)
      bodyHtml = stripRadixIds(remap(await page.evaluate(() => document.querySelector("#root").innerHTML)))
      const navIds = await page.evaluate((sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id.replace(/-trigger$/, "")), triggerSel)
      templates = parts.map((p, k) => `<template id="${navIds[k]}-content-tpl">\n${stripRadixIds(remap(p))}\n</template>`).join("\n")
      selfTest = async () => {
        await page.click(`#${navIds[0]}-trigger`); await page.waitForTimeout(500)
        if (!await page.evaluate((sel) => !!document.querySelector(sel), contentSel)) throw new Error("did not open")
        await page.keyboard.press("Escape"); await page.waitForTimeout(500)
        if (await page.evaluate((sel) => !!document.querySelector(sel + "[data-state=open]"), contentSel)) throw new Error("did not close")
      }
    } else if (fam.kind === "inline") {
      // tabs: radix mounts only the active panel; activate each trigger and
      // collect its panel, then rebuild every root with all panels present
      // (inactive ones hidden) and stable ids
      bodyHtml = await page.evaluate(async () => {
        const sleep = (ms) => new Promise((r) => setTimeout(r, ms))
        const roots = [...document.querySelectorAll("#root [data-slot=tabs]")]
        let r = 0
        for (const root of roots) {
          const triggers = [...root.querySelectorAll("[data-slot=tabs-trigger]")]
          const activeIdx = triggers.findIndex((t) => t.getAttribute("data-state") === "active")
          const panels = []
          for (let i = 0; i < triggers.length; i++) {
            triggers[i].click(); await sleep(80)
            const panel = root.querySelector("[data-slot=tabs-content]:not([hidden])") ?? root.querySelector("[data-slot=tabs-content]")
            panels.push(panel ? panel.outerHTML : null)
          }
          if (activeIdx >= 0) { triggers[activeIdx].click(); await sleep(80) }
          root.querySelectorAll("[data-slot=tabs-content]").forEach((p) => p.remove())
          // ids the usage tree already gave (t1/t2 → d1/d2 in the contract def) stay
          const tid = (t, i) => (t.id && !t.id.startsWith("radix-")) ? t.id : `tab${r}-${i}`
          const pid = (t, i) => { const c = t.getAttribute("aria-controls"); return (c && !c.startsWith("radix-")) ? c : `panel${r}-${i}` }
          const pids = triggers.map((t, i) => pid(t, i))
          triggers.forEach((t, i) => {
            t.id = tid(t, i); t.setAttribute("aria-controls", pids[i])
            const st = i === Math.max(activeIdx, 0) ? "active" : "inactive"
            t.setAttribute("data-state", st); t.setAttribute("aria-selected", st === "active" ? "true" : "false")
          })
          panels.forEach((html, i) => {
            if (!html) return
            const tpl = document.createElement("template"); tpl.innerHTML = html
            const p = tpl.content.firstElementChild
            p.id = pids[i]; p.setAttribute("aria-labelledby", triggers[i].id)
            // radix mounts a panel with animation-duration: 0s (Presence, no
            // enter animation on first mount); the oracle records it on the
            // active panel — every panel here starts the same way
            if (!p.getAttribute("style")) p.setAttribute("style", "animation-duration: 0s")
            const st = i === Math.max(activeIdx, 0) ? "active" : "inactive"
            p.setAttribute("data-state", st); if (st === "inactive") p.setAttribute("hidden", ""); else p.removeAttribute("hidden")
            root.appendChild(p)
          })
          r++
        }
        return document.querySelector("#root").innerHTML
      })
      selfTest = async () => {
        const idx = await page.evaluate(() => [...document.querySelectorAll("[data-slot=tabs-trigger]")]
          .findIndex((t) => !t.disabled && t.getAttribute("data-state") !== "active"))
        if (idx < 0) return
        await (await page.$$("[data-slot=tabs-trigger]"))[idx].click(); await page.waitForTimeout(300)
        const ok = await page.evaluate((i) => {
          const t = document.querySelectorAll("[data-slot=tabs-trigger]")[i]
          const p = document.getElementById(t.getAttribute("aria-controls"))
          // panel-less lists (tabs-icons, tabs-line): the trigger state is the whole story
          return t.getAttribute("data-state") === "active" && (!p || !p.hasAttribute("hidden"))
        }, idx)
        if (!ok) throw new Error(`tab ${idx} did not activate`)
      }
    } else {
      if (CONTRACTS && target.def.open) { await eval(`(async (page) => { ${target.def.open} })(page)`); await page.waitForTimeout(500) }
      bodyHtml = stripRadixIds(await page.evaluate(() => document.querySelector("#root").innerHTML))
      selfTest = async () => {
        if (comp === "slider") {
          const thumb = await page.$("[data-slot=slider-thumb]")
          if (!thumb) return
          const before = await thumb.getAttribute("aria-valuenow")
          await thumb.focus(); await page.keyboard.press("ArrowRight"); await page.waitForTimeout(200)
          const after = await thumb.getAttribute("aria-valuenow")
          if (before === after && !(await thumb.evaluate((t) => t.getAttribute("aria-disabled") === "true" || t.closest("[data-disabled]")))) throw new Error("slider thumb did not move on ArrowRight")
        } else if (comp === "carousel") {
          const next = await page.$("[data-slot=carousel-next]:not([disabled])")
          if (!next) return
          const before = await page.evaluate(() => [...document.querySelectorAll("[data-slot=carousel-previous]")].map((b) => b.disabled).join())
          await next.click(); await page.waitForTimeout(500)
          const after = await page.evaluate(() => [...document.querySelectorAll("[data-slot=carousel-previous]")].map((b) => b.disabled).join())
          if (before === after) throw new Error("carousel did not scroll on next (previous button state unchanged)")
        } else if (comp === "scroll-area") {
          const wired = await page.evaluate(() => [...document.querySelectorAll("[data-slot=scroll-area-scrollbar]")].every((b) => b.hasAttribute("data-state") || b.style.length > 0 || b.querySelector("[data-slot=scroll-area-thumb]")?.style.length > 0))
          if (!wired) throw new Error("scroll-area scrollbars not wired")
        }
      }
    }

    if (templates) allTemplates.push(templates)
    if (selfTest) selfTests.push(selfTest)
    }
    if (CONTRACTS) bodyHtml = `<div>\n${bodyHtml}\n</div>`
    const html = shell(allTemplates.length ? `${bodyHtml}\n${allTemplates.join("\n")}` : bodyHtml)

    // write to the final path, then prove the page interactive; delete on
    // any failure so a dead page can never land. The fixture's relative
    // asset paths (../out.css, ../shadless.js, ../js/*) resolve in the
    // build/fixture scratch tree — docs/demos has no vendored assets — so
    // the self-test loads a copy from there
    writeFileSync(outPath, html)
    const errors = []
    const onErr = (e) => errors.push((process.env.EF_KEEP ? String(e.stack || e) : String(e)).slice(0, 400))
    page.on("pageerror", onErr)
    const testPath = CONTRACTS ? outPath : `${SELFTEST}/pages/${name}.html`
    if (!CONTRACTS) writeFileSync(testPath, html)
    await page.goto(`file://${resolve(testPath)}`)
    await page.waitForTimeout(400)
    try { for (const st of selfTests) await st() } catch (e) { throw new Error(`self-test: ${e.message} (${errors.join(" | ") || "no page errors"})`) }
    // the programmatic handles: every openable instance must open and close
    // through shadless.get(trigger) too — the API is part of the contract
    // — and announce it: shadless:open / shadless:close bubble from the trigger
    const apiFail = await page.evaluate(async (fams) => {
      const sleep = (ms) => new Promise((r) => setTimeout(r, ms))
      const seen = []
      const log = (e) => seen.push(e.type.slice(9) + ":" + e.detail.component + ":" + (e.target === trig ? "trigger" : e.target.id))
      document.addEventListener("shadless:open", log)
      document.addEventListener("shadless:close", log)
      var trig = null
      for (const comp of fams) {
        trig = document.querySelector(`[data-slot="${comp}-trigger"][id$="-trigger"]:not([disabled])`)
        if (!trig) continue
        const a = shadless.get(trig)
        if (!a || typeof a.open !== "function") return `${comp}: shadless.get(trigger) has no open()`
        seen.length = 0
        a.open(); await sleep(800)
        if (!a.isOpen() || !document.querySelector(`[data-slot="${comp}-content"]`)) return `${comp}: open() did not open`
        if (seen.join(",") !== `open:${comp}:trigger`) return `${comp}: open() events = [${seen}] (want open:${comp}:trigger)`
        a.close(); await sleep(600)
        if (a.isOpen()) return `${comp}: close() did not close`
        if (seen.join(",") !== `open:${comp}:trigger,close:${comp}:trigger`) return `${comp}: close() events = [${seen}] (want open,close on the trigger)`
      }
      return null
    }, target.families.filter((f) => ["dialog", "portal", "menu", "select", "nav"].includes(FAMILY[f].kind)))
    if (apiFail) throw new Error(`self-test (api): ${apiFail}`)
    page.off("pageerror", onErr)
    if (errors.length) throw new Error(`self-test: page errors — ${errors.join(" | ")}`)
    if (CHECK && readFileSync(outPath, "utf8") !== html) failures.push(`${name}: committed page drifted from regeneration`)
    emitted++
  } catch (e) {
    if (!process.env.EF_KEEP && !CONTRACTS) {
      rmSync(outPath, { force: true })
      rmSync(`${SELFTEST}/pages/${name}.html`, { force: true })
    }
    failures.push(`${name}: ${e.message.split("\n")[0]}`)
  }
}
await browser.close()
if (failures.length) {
  console.error(`FAIL  example-fixture\n  ${failures.join("\n  ")}`)
  process.exit(1)
}
if (unsupported.length) console.log(`example-fixture: ${unsupported.length} pages carry families without a protocol yet:\n  ${unsupported.join("\n  ")}`)
console.log(`PASS  example-fixture (${emitted} interactive pages${CHECK ? " == committed" : " emitted"}, open/close self-verified)`)
