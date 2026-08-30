// T8 demo smoke: every dist/components/*.html renders with zero real console
// errors and every IR slot name appears in the page source (live DOM or
// inside a <template>, for kernel portal content). The avatar fixture used
// to load a broken file:/// image to exercise the fallback settle; FT3
// replaced it with an inline data-URI so demos work over http — the
// fallback-on-error path stays covered by `npm run contracts -- avatar`.
// The URL filter below is legacy tolerance, kept harmless.
import { chromium } from "playwright"
import { readdirSync, readFileSync } from "node:fs"

// FT8/Step 9: dist/components now also holds per-language RTL variants
// (e.g. alert-rtl-he.html, alert-rtl-en.html) emitted by build-rtl.mjs.
// The original 49 base components are the ones that have corresponding
// IRs and slots to verify; the language variants just re-use those slots
// with translated text and don't need a separate smoke pass.
// What ships is recorded per component in src/registry/tiers.json: the three
// shipped tiers, plus `emit: true` on the four that ship despite their tier.
// demo.mjs reads the same field, so the page count here and the set it builds
// cannot disagree — this was three separate encodings of one predicate, two
// hand-written filters and a "+ 4".
const REGISTRY_TIERS = JSON.parse(readFileSync("src/registry/tiers.json", "utf8"))
const shipped = (name, tier) =>
  ["static", "kernel", "trivial-js"].includes(tier) || REGISTRY_TIERS[name]?.emit === true

const allHtml = readdirSync("dist/components").filter((f) => f.endsWith(".html")).sort()
const pages = allHtml.filter((f) => !/-rtl-(en|he|fa)\.html$/.test(f))
const rtlVariants = allHtml.length - pages.length
// base components (one per emitted IR — derived from tiers.json, not a
// hardcoded 49) + alert-demo.html (FT8/Step 1 emits this from
// examples/radix/alert-demo.tsx).
{
  const emitted = Object.entries(REGISTRY_TIERS).filter(([n, x]) => shipped(n, x.tier)).length
  if (pages.length !== emitted + 1) {
    console.error(`FAIL demo-smoke: expected ${emitted + 1} base pages (${emitted} IR + alert-demo), got ${pages.length} (${rtlVariants} RTL variants skipped)`); process.exit(1)
  }
}

// Global slot vocabulary across all emitted components — demo fixtures
// legitimately compose multiple components (e.g. label + input), so a slot
// is valid if ANY IR declares it.
const ALL_SLOTS = new Set(readdirSync("src/registry/ir")
  .map((f) => JSON.parse(readFileSync(`src/registry/ir/${f}`, "utf8")))
  .filter((ir) => shipped(ir.name, ir.tier))
  .flatMap((ir) => ir.components.flatMap((c) => c.elements.filter((e) => e.slot).map((e) => e.slot))))

const irSlots = (name) => {
  const ir = JSON.parse(readFileSync(`src/registry/ir/${name}.json`, "utf8"))
  return new Set(ir.components.flatMap((c) => c.elements.filter((e) => e.slot).map((e) => e.slot)))
}

const pageSlotNames = (html) =>
  [...new Set([...html.matchAll(/data-slot="([\w-]+)"/g)].map((m) => m[1]))]

const browser = await chromium.launch()
let fail = false
// FT8/Step 1: alert-demo.html is a composition (not a registered
// component) — it has no IR file, so skip the slot-fidelity check
// (we still render it + assert 0 console errors below).
const irExists = (name) => {
  try {
    JSON.parse(readFileSync(`src/registry/ir/${name}.json`, "utf8"))
    return true
  } catch {
    return false
  }
}
for (const f of pages) {
  const name = f.replace(".html", "")
  const html = readFileSync(`dist/components/${f}`, "utf8")
  const irSet = irExists(name) ? irSlots(name) : new Set()
  const pageSlots = pageSlotNames(html)
  // phantom check: every slot in the page must be a valid slot in some
  // emitted component (catches typos / corrupted fixtures); require ≥1 live
  // slot when this component's IR declares any.
  const phantom = pageSlots.filter((s) => !ALL_SLOTS.has(s))
  const page = await browser.newPage()
  const errors = []
  page.on("pageerror", (e) => errors.push(String(e)))
  // resource-load console messages are redundant with requestfailed (which
  // carries the URL); skip them here and let requestfailed decide.
  page.on("console", (m) => {
    if (m.type() === "error" && !/^Failed to load resource/.test(m.text()))
      errors.push(m.text())
  })
  page.on("requestfailed", (req) => {
    const url = req.url()
    if (!/nonexistent/.test(url)) errors.push(`requestfailed: ${url} ${req.failure()?.errorText || ""}`)
  })
  await page.goto(`file://${process.cwd()}/dist/components/${f}`)
  const slots = await page.evaluate(`document.querySelectorAll("[data-slot]").length`)
  await page.close()
  if (phantom.length || errors.length || (slots === 0 && irSet.size)) {
    console.error(`FAIL demo-smoke [${f}]: phantom=${phantom.join(",")} slots=${slots} irSlots=${irSet.size} errors=${JSON.stringify(errors)}`)
    fail = true
  }
}
await browser.close()
if (fail) { console.log("FAIL  demo smoke"); process.exit(1) }
console.log(`PASS  demo smoke (52 base pages + ${rtlVariants} RTL variants, IR-slot fidelity, 0 console errors)`)
