#!/usr/bin/env node
// gates/demo-parity.mjs — every shipped demo page, styled by our CSS, computes
// what the SAME DOM computes under upstream's own stylesheet.
//
// The oracle-rendered demo pages carry React's exact DOM (golden hop 2 proves
// it) with inline utilities plus our slot rules. Load each page twice — once
// as shipped (../out.css), once with build/gates/oracle.css and the .style-nova
// root — and compare getComputedStyle over every [data-slot] element in
// light/dark x ltr/rtl. Same DOM on both sides, so every difference is the
// emitted CSS: skin markers, slot rules leaking into inline-styled pages,
// theme tokens. Covers the demo-inline cells of every component that has an
// oracle page, contract def or not.
//
// Cells: page / slot#n / property @ theme @ dir, ratcheted in
// gates/demo-parity-baseline.json (identity-pinned, may only shrink).
//
//   node gates/demo-parity.mjs            the gate
//   node gates/demo-parity.mjs --record   re-record
//   node gates/demo-parity.mjs --details  print light/ltr cells with values
import { chromium } from "playwright"
import { readFileSync, existsSync } from "node:fs"
import { resolve } from "node:path"
import { cellMap, loadBaseline, writeBaseline, diffBaseline, showCell, showChange } from "./parity-baseline.mjs"

const BASELINE = "gates/demo-parity-baseline.json"
const RECORD = process.argv.includes("--record")
const PROPS = [
  "color", "background-color", "border-color", "border-top-width", "border-radius", "padding-top",
  "padding-right", "padding-bottom", "padding-left", "margin-top", "margin-left", "width", "height",
  "min-width", "max-width", "font-size", "font-weight", "line-height", "display", "flex-direction",
  "align-items", "justify-content", "gap", "position", "opacity", "box-shadow", "text-align", "overflow",
]
const MATRIX = [["light", "ltr"], ["dark", "ltr"], ["light", "rtl"], ["dark", "rtl"]]
const owned = JSON.parse(readFileSync("docs/example-oracle.json", "utf8")).filter((t) => t.out.startsWith("docs/demos/"))
const oracleCss = readFileSync("build/gates/oracle.css", "utf8")
const outCss = readFileSync("dist/out.css", "utf8")
// harness shell pinned on both sides: our demo globals give <body> a
// padding and the foreground colour (docs preview conventions); upstream's
// layout does that with classes on its own body. Neither is the component.
const FREEZE = "*,*::before,*::after{transition:none!important;animation:none!important} body{padding:0!important;margin:0;color:var(--foreground);background:var(--background)}"

const browser = await chromium.launch()
const page = await browser.newPage({ reducedMotion: "reduce" })
await page.route(/^(https?:)?\/\//, (r) => r.abort())
const collect = () => page.evaluate((props) => {
  const out = {}
  for (const [theme, dir] of [["light", "ltr"], ["dark", "ltr"], ["light", "rtl"], ["dark", "rtl"]]) {
    document.documentElement.classList.toggle("dark", theme === "dark"); document.documentElement.setAttribute("dir", dir)
    const seen = {}
    for (const el of document.querySelectorAll("body [data-slot]")) {
      const slot = el.getAttribute("data-slot"); const key = `${slot}#${seen[slot] = (seen[slot] ?? 0) + 1}`
      const cs = getComputedStyle(el); const style = {}
      for (const p of props) style[p] = cs.getPropertyValue(p)
      out[`${key}@${theme}@${dir}`] = style
    }
  }
  return out
}, PROPS)
const norm = (v) => v.replace(/-?\d*\.?\d+(?:e[-+]?\d+)?/gi, (n) => String(Math.round(parseFloat(n) * 100) / 100 || 0)).replace(/\boklab\((-?[\d.]+) 0 0\)/g, "oklch($1 0 0)")

const cells = []
let pages = 0, compared = 0
for (const t of owned) {
  const html = readFileSync(t.out, "utf8")
  // scripts (runtime, pre-paint) stripped: this is a stylesheet comparison of
  // the closed DOM, and the runtime would mutate one side
  const body = /<body([^>]*)>([\s\S]*)<\/body>/.exec(html)
  if (!body) continue
  const bare = body[2].replace(/<script[\s\S]*?<\/script>/g, "")
  const doc = (css, root) => `<!doctype html><html class="${root}"><head><style>${css}</style><style>${FREEZE}</style></head><body${body[1]}>${bare}</body></html>`
  await page.setContent(doc(outCss, "")); await page.waitForTimeout(30)
  const ours = await collect()
  await page.setContent(doc(oracleCss, "style-nova")); await page.waitForTimeout(30)
  const theirs = await collect()
  pages++
  for (const [k, ref] of Object.entries(theirs)) {
    const got = ours[k]; if (!got) continue
    compared++
    const [slotKey, theme, dir] = k.split("@")
    for (const p of PROPS) {
      const a = norm(ref[p]), b = norm(got[p])
      if (a !== b) cells.push({ id: `${t.name}/${slotKey}/${p}@${theme}@${dir}`, oracle: a, shadless: b })
    }
  }
}
await browser.close()

const actual = cellMap(cells)
if (process.argv.includes("--details")) for (const [id, v] of actual) if (/@light@ltr$/.test(id)) console.log(`${id}: ${showCell(v)}`)
if (RECORD || !existsSync(BASELINE)) {
  writeBaseline(BASELINE, { note: "shipped demo DOM under our css vs the same DOM under upstream css; may only shrink, and a recorded cell's VALUES are pinned too", cells: actual })
  console.log(`demo-parity: baseline recorded (${actual.size} cells over ${pages} pages, ${compared} element×theme×dir comparisons)`)
  process.exit(0)
}
const { appeared, fixed, changed } = diffBaseline(loadBaseline(BASELINE).cells, actual)
if (appeared.length) { console.error(`FAIL  demo-parity (${appeared.length} NEW cells where a shipped demo ≠ upstream css)\n  ` + appeared.slice(0, 40).map((id) => `${id}: ${showCell(actual.get(id))}`).join("\n  ")); process.exit(1) }
if (changed.length) { console.error(`FAIL  demo-parity (${changed.length} recorded cells still differ, but by a DIFFERENT amount — re-look, then re-record: node gates/demo-parity.mjs --record)\n  ` + changed.slice(0, 20).map(showChange).join("\n  ")); process.exit(1) }
if (fixed.length) { console.error(`FAIL  demo-parity (${fixed.length} recorded cells no longer differ — record the win: node gates/demo-parity.mjs --record && node gates/ledger.mjs --record)\n  ` + fixed.slice(0, 20).join("\n  ")); process.exit(1) }
console.log(`PASS  demo-parity (${pages} pages, ${compared} comparisons, ${actual.size} cells at the recorded baseline incl. their values; --strict is the end state)`)
