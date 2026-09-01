#!/usr/bin/env node
// browser-shell — the chromium thin shell for the Go pipeline's browser
// nodes (Wave 3). One long-lived process: the Go driver writes JSON request
// lines to stdin, this shell writes one JSON response line per request to
// stdout. All state (assertions, comparisons, reports) lives in Go; this
// file only speaks browser primitives:
//
//   {"op":"launch"}
//   {"op":"newPage"}                                            → {"pageId":N}
//   {"op":"goto","pageId":N,"url":U}                            → {"ok":true}
//   {"op":"evaluate","pageId":N,"expr":E,"arg":any}             → {"value":any}
//   {"op":"waitForFunction","pageId":N,"expr":E,"timeout":ms}   → {"ok":true}
//   {"op":"close","pageId":N}
//   {"op":"close"}                                              (browser)
//
// Event capture per page (console errors / pageerror / requestfailed) is
// part of the primitive set because Go cannot see them:
//   {"op":"newPage","capture":true} arms the collectors;
//   {"op":"events","pageId":N} drains them → {"errors":[…]}
//
// The set is deliberately minimal: anything expressible as evaluate() runs
// through evaluate (the oracle canon lives in a page-level function on the
// Go side and is injected as an expression). This mirrors the decision that
// Go owns every judgement call and node only owns chromium.
import readline from "node:readline"
import { chromium } from "playwright"

const pages = new Map()
let nextId = 1
let browser = null

const send = (obj) => process.stdout.write(JSON.stringify(obj) + "\n")

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
        // resource-load console messages are redundant with requestfailed
        // (which carries the URL); requestfailed decides.
        page.on("console", (m) => {
          if (m.type() === "error" && !/^Failed to load resource/.test(m.text()))
            errors.push(m.text())
        })
        page.on("requestfailed", (r) => {
          const url = r.url()
          if (!/nonexistent/.test(url))
            errors.push(`requestfailed: ${url} ${r.failure()?.errorText || ""}`)
        })
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
    case "waitForFunction": {
      const { page } = pages.get(req.pageId)
      await page.waitForFunction(req.expr, null, { timeout: req.timeout ?? 5000 })
      return { ok: true }
    }
    case "events": {
      const { errors } = pages.get(req.pageId)
      return { errors }
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
