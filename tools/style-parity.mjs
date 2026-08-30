#!/usr/bin/env node
// style-parity — COMPUTED STYLE parity between the React oracle and the
// shadless fixture, per contract component. The missing coverage layer:
// every existing gate is DOM/structure-level; "same DOM + same out.css ⇒
// same styles" was an INFERENCE, never tested (the user hit exactly this
// with the alert-dialog layout drift — markup right, style outcome wrong).
//
// Method: for each contract def, load oracle.html and shadless.html, apply
// def.open / def.openShadless, then walk both DOMs pairing elements by
// data-slot sequence and compare getComputedStyle over a property list
// covering color / spacing / layout / typography / borders / shadows /
// transforms. Deterministic (no screenshots) — computed values round to
// 2 decimals; radix runtime vars and transition-timing noise are dropped.
//
// Known-difference allowances live in the def as styleIgnore (property
// names), defaulting to the recorded runtime-style family.
import { chromium } from "playwright"
import { readFileSync, readdirSync, existsSync } from "node:fs"
import { resolve } from "node:path"
import { cellMap, loadBaseline, writeBaseline, diffBaseline, showCell, showChange } from "../gates/parity-baseline.mjs"

const DIR = "tools/contracts/out"
// every differing (component, slot#occurrence, property) triple
const cells = []
const harnessErrors = []

const PROPS = [
  "color", "background-color", "border-color", "border-top-width", "border-radius",
  "display", "position", "flex-direction", "align-items", "justify-content",
  "gap", "padding-top", "padding-right", "padding-bottom", "padding-left",
  "margin-top", "margin-right", "margin-bottom", "margin-left",
  "width", "height", "min-width", "max-width", "font-size", "font-weight",
  "line-height", "text-align", "opacity", "z-index", "overflow", "visibility",
  "box-shadow", "transform", "inset", "flex-wrap", "grid-template-columns",
]
// runtime-measured / animation noise both sides carry differently
const SKIP_PROPS = new Set(["transform", "transition", "transition-duration", "animation"])

// Pair by SLOT SEQUENCE (slot name + occurrence index), not tree order —
// harness wrappers (#root) and fixture chrome offset raw walks. Elements
// without a slot pair under the null-slot sequence.
const collect = () => page.evaluate((props) => {
  const out = []
  const walk = (el) => {
    if (el.tagName === "SCRIPT" || el.tagName === "TEMPLATE") return
    if (el.hasAttribute("data-slot")) {
      const cs = getComputedStyle(el)
      const style = {}
      for (const p of props) style[p] = cs.getPropertyValue(p)
      out.push({ slot: el.getAttribute("data-slot"), tag: el.tagName, style })
    }
    for (const c of el.children) walk(c)
  }
  for (const root of document.body.children) walk(root)
  return out
}, PROPS)

// Measurements were being taken MID-ANIMATION. getComputedStyle during a
// transition returns interpolated values, and chromium serializes an
// interpolated colour as oklab() while a static one stays oklch() — so the
// gate reported oracle=41.97px/shadless=42px, font-weight 499.75/500,
// padding 2.37e-08px/0px and oklab/oklch pairs that are the SAME style caught
// at different instants. The old presence-only baseline hid this by marking
// whole components permanently dirty. Freeze both sides before collecting.
const FREEZE = `*, *::before, *::after {
  transition: none !important; animation: none !important;
  animation-duration: 0s !important; transition-duration: 0s !important;
}`
// Harness shell, identical on both sides. Neither page is a real consumer
// page: the oracle renders into a bare <body>, the fixture loads our demo
// globals (which give body a padding and the foreground color the docs
// previews want). Those are harness conventions, not component styling,
// and they showed up as 64px width / rgb-vs-oklch colour deltas on every
// root slot. Pin the shell on both sides so only the component differs.
const SHELL = "padding:0;margin:0;color:var(--foreground);background:var(--background)"
async function freeze(page) {
  await page.evaluate((shell) => { document.body.style.cssText += ";" + shell }, SHELL)
  await page.addStyleTag({ content: FREEZE })
  await page.evaluate(() => document.getAnimations?.().forEach((a) => a.finish()))
  await page.waitForTimeout(120)
}

// The theme x direction matrix, collected from the SAME open page: after
// the state is reached and frozen, toggle .dark on <html> and dir on
// <html>, re-collect. Four variants per element, one page load. Dark and
// RTL were the two largest uncovered slices of the product matrix
// (pipeline/gate_coverage.go) and cost nothing extra here.
const MATRIX = [
  { id: "light@ltr", dark: false, dir: "ltr" },
  { id: "dark@ltr", dark: true, dir: "ltr" },
  { id: "light@rtl", dark: false, dir: "rtl" },
  { id: "dark@rtl", dark: true, dir: "rtl" },
]
async function collectMatrix() {
  const out = {}
  for (const v of MATRIX) {
    await page.evaluate((v) => {
      document.documentElement.classList.toggle("dark", v.dark)
      document.documentElement.setAttribute("dir", v.dir)
    }, v)
    await page.waitForTimeout(60)
    out[v.id] = await collect()
  }
  await page.evaluate(() => { document.documentElement.classList.remove("dark"); document.documentElement.setAttribute("dir", "ltr") })
  return out
}

const normVal = (v) => {
  if (!v) return v
  return v
    // every number, including scientific notation (2.37075e-08px) which the
    // old float-only pattern skipped entirely
    .replace(/-?\d*\.?\d+(?:e[-+]?\d+)?/gi, (n) => {
      const r = Math.round(parseFloat(n) * 100) / 100
      return String(Object.is(r, -0) ? 0 : r)
    })
    // an interpolated colour serializes as oklab(); the same colour at rest
    // serializes as oklch(). After rounding, a chroma/hue-free oklab IS the
    // oklch of the same lightness.
    .replace(/\boklab\((-?[\d.]+) 0 0\)/g, "oklch($1 0 0)")
    .replace(/calc\([^)]*\)/g, "calc(…)")
}

const browser = await chromium.launch()
const page = await browser.newPage({ reducedMotion: "reduce" })
await page.route(/^(https?:)?\/\//, (r) => r.abort())

const defs = readdirSync("tools/contracts/components").filter((f) => f.endsWith(".mjs"))
let compared = 0, components = 0
for (const f of defs) {
  const name = f.replace(".mjs", "")
  const dir = `${DIR}/${name}`
  if (!existsSync(`${dir}/oracle.html`) || !existsSync(`${dir}/shadless.html`)) continue
  const def = (await import(`./contracts/components/${f}`)).default
  const styleIgnore = new Set([...(def.styleIgnore ?? []), ...SKIP_PROPS])
  const props = PROPS.filter((p) => !styleIgnore.has(p))
  try {
    // oracle side (inject out.css so the classes actually style)
    await page.goto(`file://${process.cwd()}/${dir}/oracle.html`)
    await page.waitForTimeout(400)
    if (def.open) { await eval(`(async (page) => { ${def.open} })(page)`); await page.waitForTimeout(400) }
    // The oracle is styled by build/gates/oracle.css — built by
    // pipeline/oracle_css.go from upstream's own globals/tailwind.css/skin and
    // the resolved registry source, reading NOTHING under src/. Styling the
    // oracle with our own dist/out.css (the previous shape) made this gate
    // blind to every css-emitter bug: same stylesheet on both sides, same
    // computed values, green. Upstream applies the skin under a root class.
    await page.addStyleTag({ path: "build/gates/oracle.css" })
    await page.evaluate(() => document.documentElement.classList.add("style-nova"))
    await freeze(page)
    const oracleSides = await collectMatrix()
    // shadless side (loads its own out.css via relative link)
    await page.goto(`file://${process.cwd()}/${dir}/shadless.html`)
    await page.addStyleTag({ path: "dist/out.css" })
    await page.waitForTimeout(400)
    if (def.openShadless ?? def.open) { await eval(`(async (page) => { ${def.openShadless ?? def.open} })(page)`); await page.waitForTimeout(400) }
    await freeze(page)
    const shadlessSides = await collectMatrix()
    const key = (e, seen) => {
      const k = e.slot + "#" + (seen[e.slot] = (seen[e.slot] ?? 0) + 1)
      return k
    }
    for (const variant of MATRIX) {
      const diffs = []
      const seenA = {}, seenB = {}
      const mapA = new Map(oracleSides[variant.id].map((e) => [key(e, seenA), e]))
      const mapB = new Map(shadlessSides[variant.id].map((e) => [key(e, seenB), e]))
      for (const [k, a] of mapA) {
        const b = mapB.get(k)
        if (!b) { diffs.push({ key: k, prop: "<presence>", oracle: "present", shadless: "missing" }); continue }
        for (const p of props) {
          const va = normVal(a.style[p]), vb = normVal(b.style[p])
          if (va !== vb) diffs.push({ key: k, prop: p, oracle: String(va), shadless: String(vb) })
        }
      }
      for (const k of mapB.keys()) if (!mapA.has(k)) diffs.push({ key: k, prop: "<presence>", oracle: "missing", shadless: "present" })
      // light/ltr cells keep their historical ids; the other variants suffix
      for (const d of diffs) cells.push({ component: name, ...d, key: variant.id === "light@ltr" ? d.key : `${d.key}@${variant.id}` })
      compared += mapA.size
    }
    components++
  } catch (e) {
    harnessErrors.push(`${name}: harness error — ${e.message.split("\n")[0]}`)
  }
}
await browser.close()

// ---------------------------------------------------------------- ratchet
//
// The old baseline recorded a COUNT per component but compared only
// PRESENCE (`if (!baseline[name])`), and every component contributed exactly
// one entry, so the counts were never read. 23 of 29 components were listed
// — meaning any NEW computed-style regression in dialog, select, tabs,
// tooltip, carousel … kept the gate green. The comment claimed "FAILS when
// drift GROWS"; the code did not implement it.
//
// The unit of the ratchet is now the CELL: component / slot#occurrence /
// property. 23 components became ~200 cells, so a fixed cell that is
// replaced by a different broken one no longer hides. Both the cell's
// IDENTITY and its VALUES are pinned (gates/parity-baseline.mjs): recording
// the id alone left every recorded cell free to drift by any amount forever,
// which is the opposite of what "we looked at this difference" means.
//
// `flaky` lists components whose fixtures race their own runtime init; their
// cells are excluded from the ratchet entirely and counted separately, so a
// flake can neither fail the build nor masquerade as cleanliness.
const STRICT = process.argv.includes("--strict")
const RECORD = process.argv.includes("--record")
const BASELINE_PATH = "gates/style-parity-baseline.json"

const cellId = (c) => `${c.component}/${c.key}/${c.prop}`

if (harnessErrors.length) {
  console.error(`FAIL  style-parity (harness)\n  ${harnessErrors.join("\n  ")}`)
  process.exit(1)
}

// `flaky` is read from the raw file: --record must work on a baseline in the
// pre-value format too, and loadBaseline refuses that one (correctly — the
// COMPARE path cannot silently treat a value-less baseline as matching).
const flaky = new Set((existsSync(BASELINE_PATH)
  ? JSON.parse(readFileSync(BASELINE_PATH, "utf8")).flaky : null) ?? [])

const ratcheted = cells.filter((c) => !flaky.has(c.component))
const flakyCells = cells.length - ratcheted.length
const actual = cellMap(ratcheted.map((c) => ({ id: cellId(c), oracle: c.oracle, shadless: c.shadless })))

if (RECORD || !existsSync(BASELINE_PATH)) {
  writeBaseline(BASELINE_PATH, {
    note: "Cells where the shadless fixture's computed style differs from the React oracle, " +
      "with the two values as recorded. This list may only shrink and the values are pinned; " +
      "see gates/ledger.mjs budget style-parity.dirty-cells.",
    flaky, cells: actual,
  })
  console.log(`style-parity: baseline recorded (${actual.size} cells across ` +
    `${new Set(ratcheted.map((c) => c.component)).size} components, ${flaky.size} flaky components excluded)`)
  process.exit(0)
}

if (STRICT && cells.length) {
  console.error(`FAIL  style-parity --strict (${cells.length} differing cells)\n  ` +
    cells.slice(0, 10).map((c) => `${cellId(c)}: ${showCell(c)}`).join("\n  "))
  process.exit(1)
}

const { appeared, fixed, changed } = diffBaseline(loadBaseline(BASELINE_PATH).cells, actual)

if (appeared.length) {
  console.error(`FAIL  style-parity (${appeared.length} NEW differing cells vs the React oracle)\n  ` +
    appeared.slice(0, 40).map((id) => `${id}: ${showCell(actual.get(id))}`).join("\n  ") +
    (appeared.length > 40 ? `\n  … +${appeared.length - 40} more` : ""))
  process.exit(1)
}
if (changed.length) {
  console.error(`FAIL  style-parity (${changed.length} recorded cells still differ from the oracle, but by a ` +
    `DIFFERENT amount than what was recorded — look at them again, then re-record)\n  ` +
    changed.slice(0, 20).map(showChange).join("\n  ") +
    (changed.length > 20 ? `\n  … +${changed.length - 20} more` : "") +
    `\n\n  node tools/style-parity.mjs --record`)
  process.exit(1)
}
if (fixed.length) {
  console.error(`FAIL  style-parity (${fixed.length} recorded cells no longer differ — record the win so ` +
    `the slack cannot be silently re-spent)\n  ` + fixed.slice(0, 12).join("\n  ") +
    (fixed.length > 12 ? `\n  … +${fixed.length - 12} more` : "") +
    `\n\n  node tools/style-parity.mjs --record && node gates/ledger.mjs --record`)
  process.exit(1)
}
console.log(`PASS  style-parity (${components} components, ${compared} elements compared, ` +
  `${actual.size} cells at the recorded baseline incl. their values, ${flakyCells} cells in ${flaky.size} flaky components excluded; ` +
  `--strict is the end state)`)
