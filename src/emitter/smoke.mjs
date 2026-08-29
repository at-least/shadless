// T5 smoke: every generated static page renders in chromium with zero console
// errors and at least one [data-slot]; also verify out.css compiled with all
// slot-keyed AND anchor-class rules from shadless.css.
import { chromium } from "playwright"
import { existsSync, readdirSync, readFileSync } from "node:fs"

// Safe IR loader: returns null if the IR file doesn't exist (e.g. mode-toggle
// is an authored demo, not an emitter-generated component). The previous
// version threw ENOENT on these files, breaking smoke after every emitter run.
function loadIr(name) {
  const path = `src/registry/ir/${name}.json`
  return existsSync(path) ? JSON.parse(readFileSync(path, "utf8")) : null
}

const TIERS = JSON.parse(readFileSync("probes/out/tiers.json", "utf8"))
const EXPECT_STATIC = Object.values(TIERS).filter((t) => t.tier === "static").length

const pages = readdirSync("dist/components").filter((f) => f.endsWith(".html"))
// emit phase only writes the static pages; kernel/trivial demo pages are
// produced later by the demo builder (tools/demo.mjs). Tolerate the extra
// demo pages so `npm run emit` stays green once the demo has been built.
// Skip pages that have no IR (authored demos like mode-toggle).
const staticPages = pages.filter((f) => loadIr(f.replace(".html", ""))?.tier === "static")
if (staticPages.length !== EXPECT_STATIC) {
  console.error(`FAIL smoke: ${staticPages.length} static pages (expected ${EXPECT_STATIC} from tiers.json)`)
  process.exit(1)
}

const css = readFileSync("dist/out.css", "utf8")
const shadlessCss = readFileSync("dist/shadless.css", "utf8")
// Every slot rule the emitter generated must survive the tailwind compile
// into out.css — catches any component's CSS vanishing.
const emittedSlots = [...new Set([...shadlessCss.matchAll(/\[data-slot="([\w-]+)"\]/g)].map((m) => m[1]))]
const missingSlots = emittedSlots.filter((s) => !css.includes(`[data-slot="${s}"]`))
// Same for anchor-class rules (slotless elements: .button-group-text etc.)
const emittedAnchors = [...new Set([...shadlessCss.matchAll(/^  \.([\w-]+) \{ @apply/gm)].map((m) => m[1]))]
const missingAnchors = emittedAnchors.filter((a) => !new RegExp(`\\.${a}[\\s,{:]`).test(css))
if (missingSlots.length || missingAnchors.length) {
  console.error(`FAIL smoke: out.css missing rules — slots: ${missingSlots.join(", ") || "none"}; anchors: ${missingAnchors.join(", ") || "none"}`)
  process.exit(1)
}
if (!emittedSlots.length && !emittedAnchors.length) {
  console.error("FAIL smoke: shadless.css has no slot-keyed or anchor rules"); process.exit(1)
}

const browser = await chromium.launch()
let fail = false
for (const f of staticPages) {
  const page = await browser.newPage()
  const errors = []
  page.on("pageerror", (e) => errors.push(e.message))
  page.on("console", (m) => { if (m.type() === "error") errors.push(m.text()) })
  await page.goto(`file://${process.cwd()}/dist/components/${f}`)
  const slots = await page.evaluate(`document.querySelectorAll("[data-slot]").length`)
  await page.close()
  const ir = loadIr(f.replace(".html", ""))
  const irSlots = ir.components.flatMap((c) => c.elements.filter((e) => e.slot)).length
  if ((slots === 0 && irSlots > 0) || errors.length) {
    console.error(`FAIL smoke [${f}]: slots=${slots} errors=${JSON.stringify(errors)}`)
    fail = true
  }
}
await browser.close()
if (fail) { console.log("FAIL  emit smoke"); process.exit(1) }
console.log(`PASS  emit smoke (${staticPages.length} static pages render, 0 console errors, out.css slot+anchor rules)`)
