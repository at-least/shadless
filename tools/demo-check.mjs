#!/usr/bin/env node
// demo-check.mjs — FT7 per-batch smoke: serve docs/site once, then for
// every name given (or every authored demo when --all): load the built
// copy (components/<name>.html), assert non-empty body, ≥1 data-slot
// element, 0 console errors / pageerrors. Interactive correctness stays
// with the contract tests + authoring spot-checks; this is the batch gate.
// Usage: node tools/demo-check.mjs <name> [name...] | --all
import { createServer } from "node:http"
import { readFileSync, readdirSync } from "node:fs"
import { spawn } from "node:child_process"
import { chromium } from "playwright"

const args = process.argv.slice(2)
const all = args[0] === "--all"
const catalog = JSON.parse(readFileSync("docs/catalog.json", "utf8"))
const names = all ? catalog.previews.filter((p) => p.status === "authored").map((p) => p.name) : args
if (!names.length) { console.error("no names given / no authored demos"); process.exit(1) }

const port = await new Promise((resolve, reject) => {
  // pid-derived ports collide when two demo-check runs race; listen(0)
  // lets the OS pick a free port
  const MIME = { ".html": "text/html", ".css": "text/css", ".js": "text/javascript", ".svg": "image/svg+xml", ".png": "image/png" }
  const s = createServer((req, res) => {
    const raw = req.url.split("?")[0]
    const path = decodeURIComponent(raw) === "/" ? "/index.html" : decodeURIComponent(raw)
    // path guard: reject traversal outside docs/site
    const file = `docs/site${path}`
    if (path.includes("..") || !file.startsWith("docs/site/")) {
      res.writeHead(403); res.end("forbidden"); return
    }
    try {
      const body = readFileSync(file)
      const dot = file.lastIndexOf(".")
      const ext = dot >= 0 ? file.slice(dot) : ""
      res.writeHead(200, { "content-type": MIME[ext] ?? "application/octet-stream" })
      res.end(body)
    } catch { res.writeHead(404); res.end("not found") }
  })
  s.on("error", reject)
  s.listen(0, "127.0.0.1", () => resolve({ s, port: s.address().port }))
}).then(({ s, port }) => { serverHolder = s; return port })
var serverHolder

const browser = await chromium.launch()
const failures = []
let checked = 0
for (const name of names) {
  const page = await browser.newPage()
  const errors = []
  page.on("console", (m) => { if (m.type() === "error") errors.push(m.text()) })
  page.on("pageerror", (e) => errors.push(String(e)))
  try {
    await page.goto(`http://127.0.0.1:${port}/components/${name}.html`, { waitUntil: "networkidle" })
    const slots = await page.locator("[data-slot]").count()
    if (slots === 0) failures.push(`${name}: 0 data-slot elements`)
    if (errors.length) failures.push(`${name}: ${errors.length} console errors (${errors[0].slice(0, 120)})`)
    checked++
  } catch (e) {
    failures.push(`${name}: ${e.message.slice(0, 120)}`)
  }
  await page.close()
}
await browser.close()
serverHolder.close()
console.log(`demo-check: ${checked}/${names.length} demos loaded (${failures.length} failures)`)
if (failures.length) { failures.forEach((f) => console.error("FAIL  " + f)); process.exit(1) }
console.log(`PASS  demo-check (${checked} demos, 0 console errors)`)
