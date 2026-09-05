#!/usr/bin/env node
// browser-shell — the chromium thin shell for the Go pipeline's browser
// nodes (Wave 3). One long-lived process: the Go driver writes JSON request
// lines to stdin, this shell writes one JSON response line per request to
// stdout. All state (assertions, comparisons, reports) lives in Go; this
// file only speaks browser primitives.
//
// Pages carry optional event capture (console errors / pageerror /
// requestfailed). Locators are addressed by selector with an optional
// `frame` (an iframe selector whose contentFrame the locator runs in) —
// the real-mouse interactions (mouseClick at coordinates, keyboard) stay
// page-level because that is how playwright delivers them.
//
// Ops:
//   launch / newPage {capture} / goto {waitUntil} / close {pageId?}
//   evaluate {expr, arg}              — arg becomes the expression's param
//   waitForFunction {expr, timeout}
//   events                            — drain captured errors
//   waitForLoadState {state, timeout} — "load"|"networkidle"|…
//   waitForTimeout {ms}
//   mouseClick {x, y} / keyPress {key} / wheel {dx, dy}
//   focus      {selector, timeout?}      — page.focus
//   addScriptTag {content}
//   frameExists {frame}               — iframe selector present (bool)
//   locCount   {frame?, selector}
//   locWait    {frame?, selector, state?, timeout?}
//   locScroll  {frame?, selector, index?} — scrollIntoViewIfNeeded
//   locAttr    {frame?, selector, attr, index?}
//   locBox     {frame?, selector, index?} → {x,y,width,height}
//   locEval    {frame?, selector, expr, index?} — expr(el) on the nth match
//   locEvalAll {frame?, selector, expr}   — expr(el) per match → [values]
import readline from "node:readline"
import { chromium } from "playwright"

const pages = new Map()
let nextId = 1
let browser = null

const send = (obj) => process.stdout.write(JSON.stringify(obj) + "\n")

async function frameOf(page, frameSel) {
  if (!frameSel) return null
  const handle = await page.locator(frameSel).first().contentFrame({ timeout: 10_000 })
  if (!handle) throw new Error(`iframe not found: ${frameSel}`)
  return handle
}

async function locatorIn(page, req) {
  const loc = await frameOf(page, req.frame)
  return (loc ?? page).locator(req.selector)
}

async function handle(req) {
  switch (req.op) {
    case "launch": {
      browser = await chromium.launch()
      return { ok: true }
    }
    case "newPage": {
      const page = await browser.newPage()
      const id = nextId++
      const errors = []
      if (req.capture) {
        page.on("pageerror", (e) => errors.push(String(e)))
        // captureOnly "pageerror": tools that pin themselves to pageerror
        // alone (example-fixture) opt out of the console/requestfailed net
        if (req.captureOnly !== "pageerror") {
          // resource-load console messages are redundant with requestfailed
          // (which carries the URL); requestfailed decides.
          page.on("console", (m) => {
            if (m.type() === "error" && !/^Failed to load resource/.test(m.text()))
              errors.push(m.text())
          })
          page.on("requestfailed", (r) => {
            const url = r.url()
            // same-origin failures count; the nonexistent-URL filter is the
            // legacy tolerance from tools/demo-smoke.mjs
            const scoped = req.origin ? url.startsWith(req.origin) : true
            if (scoped && !/nonexistent/.test(url))
              errors.push(`requestfailed: ${url} ${r.failure()?.errorText || ""}`)
          })
        }
      }
      pages.set(id, { page, errors })
      return { pageId: id }
    }
    case "goto": {
      const { page } = pages.get(req.pageId)
      await page.goto(req.url, { waitUntil: req.waitUntil ?? "load" })
      return { ok: true }
    }
    case "evaluate": {
      const { page } = pages.get(req.pageId)
      const value = await page.evaluate(req.expr, req.arg ?? null)
      return { value }
    }
    case "evaluateFn": {
      // string FUNCTIONS ("() => {…}") evaluate to undefined under plain
      // evaluate (strings are EXPRESSIONS there) — parse into a function
      const { page } = pages.get(req.pageId)
      const fn = new Function("return (" + req.expr + ")")()
      const value = await page.evaluate(fn, req.arg ?? null)
      return { value }
    }
    case "setContent": {
      const { page } = pages.get(req.pageId)
      await page.setContent(req.html)
      return { ok: true }
    }
    case "addStyleTag": {
      const { page } = pages.get(req.pageId)
      if (req.path) await page.addStyleTag({ path: req.path })
      else await page.addStyleTag({ content: req.content })
      return { ok: true }
    }
    case "addScriptTag": {
      const { page } = pages.get(req.pageId)
      await page.addScriptTag({ content: req.content })
      return { ok: true }
    }
    case "focus": {
      const { page } = pages.get(req.pageId)
      await page.focus(req.selector, { timeout: req.timeout ?? 30_000 })
      return { ok: true }
    }
    case "wheel": {
      const { page } = pages.get(req.pageId)
      await page.mouse.wheel(req.dx ?? 0, req.dy ?? 0)
      return { ok: true }
    }
    case "driver": {
      // contract-def open/openShadless strings: JS against the LIVE page
      const { page } = pages.get(req.pageId)
      const fn = new Function("page", "return (async () => { " + req.code + " })()")
      await fn(page)
      return { ok: true }
    }
    case "loadContractDef": {
      const mod = await import(req.file)
      const d = mod.default
      // JSON-safe projection of the fields the Go gates consume
      return { def: {
        name: d.name ?? null, usage: d.usage ?? null, imports: d.imports ?? null,
        slots: d.slots ?? [], open: d.open ?? null, openShadless: d.openShadless ?? null,
        mountedClasses: d.mountedClasses ?? null, mountedCheck: d.mountedCheck ?? null,
        shadlessPage: d.shadlessPage ?? null, scenarios: d.scenarios ?? [],
        triggerSlot: d.triggerSlot ?? null, stateProbe: d.stateProbe ?? null,
        styleIgnore: d.styleIgnore ?? [], facts: d.facts ?? null, note: d.note ?? null,
        ignoreAttrs: d.ignoreAttrs ?? null, oracleCss: d.oracleCss ?? null,
        closeSelector: d.closeSelector ?? null, overlaySlot: d.overlaySlot ?? null,
        contentSlot: d.contentSlot ?? null,
      } }
    }
    case "routeAbortExternal": {
      // example-golden keeps avatar-style demos in their INITIAL render
      // state: a loaded remote image flips radix Avatar to the img branch
      // and the trees diverge on structure, not styling
      const { page } = pages.get(req.pageId)
      await page.route(/^(https?:)?\/\//, (route) => route.abort())
      return { ok: true }
    }
    case "waitForFunction": {
      const { page } = pages.get(req.pageId)
      await page.waitForFunction(req.expr, null, { timeout: req.timeout ?? 15_000 })
      return { ok: true }
    }
    case "events": {
      const { errors } = pages.get(req.pageId)
      return { errors }
    }
    case "waitForLoadState": {
      const { page } = pages.get(req.pageId)
      await page.waitForLoadState(req.state ?? "load", { timeout: req.timeout ?? 30_000 })
      return { ok: true }
    }
    case "waitForTimeout": {
      const { page } = pages.get(req.pageId)
      await page.waitForTimeout(req.ms ?? 0)
      return { ok: true }
    }
    case "mouseClick": {
      const { page } = pages.get(req.pageId)
      await page.mouse.click(req.x, req.y, { button: req.button ?? "left" })
      return { ok: true }
    }
    case "keyPress": {
      const { page } = pages.get(req.pageId)
      await page.keyboard.press(req.key)
      return { ok: true }
    }
    case "frameExists": {
      const { page } = pages.get(req.pageId)
      const n = await page.locator(req.frame).count()
      return { value: n > 0 }
    }
    case "locCount": {
      const { page } = pages.get(req.pageId)
      const loc = await locatorIn(page, req)
      return { value: await loc.count() }
    }
    case "locWait": {
      const { page } = pages.get(req.pageId)
      const loc = await locatorIn(page, req)
      await loc.first().waitFor({ state: req.state ?? "visible", timeout: req.timeout ?? 15_000 })
      return { ok: true }
    }
    case "locScroll": {
      const { page } = pages.get(req.pageId)
      const loc = await locatorIn(page, req)
      const nth = loc.nth(req.index ?? 0)
      await nth.scrollIntoViewIfNeeded({ timeout: req.timeout ?? 15_000 }).catch(() => {})
      return { ok: true }
    }
    case "locAttr": {
      const { page } = pages.get(req.pageId)
      const loc = await locatorIn(page, req)
      const nth = loc.nth(req.index ?? 0)
      return { value: await nth.getAttribute(req.attr) }
    }
    case "locBox": {
      const { page } = pages.get(req.pageId)
      const loc = await locatorIn(page, req)
      const nth = loc.nth(req.index ?? 0)
      const box = await nth.boundingBox()
      return { value: box }
    }
    case "locEval": {
      // playwright's elementHandle.evaluate treats a string as an EXPRESSION
      // — "el => …" evaluates to a function value and returns undefined.
      const { page } = pages.get(req.pageId)
      const loc = await locatorIn(page, req)
      const nth = loc.nth(req.index ?? 0)
      const fn = new Function("return (" + req.expr + ")")()
      return { value: await nth.evaluate(fn, req.arg ?? undefined) }
    }
    case "locEvalAll": {
      const { page } = pages.get(req.pageId)
      const loc = await locatorIn(page, req)
      // elementHandle.evaluate treats a string as an EXPRESSION —
      // "el => …" evaluates to a function value and returns undefined.
      // Parse it into a real function first.
      const fn = new Function("return (" + req.expr + ")")()
      const handles = await loc.elementHandles()
      const values = []
      for (const h of handles) values.push(await h.evaluate(fn, req.arg ?? undefined))
      return { value: values }
    }
    // 15 s, not 5, on every interaction/wait default: example-fixture
    // --contracts timed out twice on a dialog trigger click under 16-way
    // browser load and passed five forced runs alone. Playwright's own
    // default is 30 s; 5 s only ever fit a quiet machine.
    case "locClick": {
      const { page } = pages.get(req.pageId)
      const loc = await locatorIn(page, req)
      const nth = loc.nth(req.index ?? 0)
      await nth.click({ timeout: req.timeout ?? 15_000, button: req.button ?? "left" })
      return { ok: true }
    }
    case "mouseMove": {
      const { page } = pages.get(req.pageId)
      await page.mouse.move(req.x, req.y, { steps: req.steps ?? 1 })
      return { ok: true }
    }
    case "close": {
      if (req.pageId !== undefined) {
        const { page } = pages.get(req.pageId)
        await page.close()
        pages.delete(req.pageId)
        return { ok: true }
      }
      if (browser) await browser.close()
      return { ok: true }
    }
    default:
      return { error: `unknown op ${req.op}` }
  }
}

const rl = readline.createInterface({ input: process.stdin })
rl.on("line", (line) => {
  if (!line.trim()) return
  let req
  try {
    req = JSON.parse(line)
  } catch (e) {
    send({ error: `bad request: ${e.message}` })
    return
  }
  handle(req)
    .then((res) => send(res))
    .catch((e) => send({ error: String(e?.message ?? e) }))
})
rl.on("close", async () => {
  if (browser) await browser.close().catch(() => {})
  process.exit(0)
})
