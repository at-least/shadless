// T8 demo build: unified slot-keyed CSS for all 49 emitted components + a
// browsable demo page per component in dist/components/. Static pages come
// from the emitter (T5); kernel pages reuse the verified src/kernel fixtures;
// trivial-js pages reuse probes/t7. Paths are rewritten to dist-relative assets
// and a single dist/out.css styles every page.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, copyFileSync, existsSync, rmSync } from "node:fs"
import { join } from "node:path"
import { transformSync as esbuildMinify } from "esbuild"
import { componentCss, wrapComponentCss } from "../src/emitter/css.mjs"
import { buildJs } from "./build-js.mjs"
import { injectPrePaint, SHADLESS_CSS_FIXES } from "../src/docs/theme-prepaint.mjs"

import { rewritePaths, ensureLink } from "./demo-lib.mjs"

const IRDIR = "src/registry/ir"
const DIST = "dist"
mkdirSync(`${DIST}/components`, { recursive: true })
// the JS surface (base + per-component files) is built by tools/build-js.mjs

// ---- tiers + names ----------------------------------------------------------
// `field` is presentational (markup + cva, no JS) — emitted alongside
// the static tier. The other gap components (menubar, navigation-menu,
// combobox, sidebar) are tombstoned (recorded in probes/out/tiers.json).
const TIERS = ["static", "kernel", "trivial-js"]
const LOGIC_PRESENTATIONAL = new Set(["field"])
// carousel is external-tier but has a clean vanilla embla port —
// emitted alongside the other tiers. The remaining external components are
// tombstoned (no vanilla upstream — see probes/out/tiers.json).
const EXTERNAL_EMIT = new Set(["carousel"])
// medium-tier (wireMenu/glue + custom glue) components now emitted —
// both joined once their glue passed contract parity
const MEDIUM_EMIT = new Set(["menubar", "navigation-menu"])
const irAll = readdirSync(IRDIR).filter((f) => f.endsWith(".json"))
  .map((f) => JSON.parse(readFileSync(join(IRDIR, f), "utf8")))
  .filter((ir) => TIERS.includes(ir.tier) || LOGIC_PRESENTATIONAL.has(ir.name) || EXTERNAL_EMIT.has(ir.name) || MEDIUM_EMIT.has(ir.name))
const byName = new Map(irAll.map((ir) => [ir.name, ir]))
const names = irAll.map((ir) => ir.name).sort()
// expected count derives from tiers.json (single source): static+kernel+
// trivial-js tiers + the two explicitly-emitted extras (field, carousel)
{
  const TIERS_COUNTED = JSON.parse(readFileSync("probes/out/tiers.json", "utf8"))
  const expected = Object.values(TIERS_COUNTED)
    .filter((x) => TIERS.includes(x.tier)).length + LOGIC_PRESENTATIONAL.size + EXTERNAL_EMIT.size + MEDIUM_EMIT.size
  if (irAll.length !== expected)
    throw new Error(`expected ${expected} emitted components (${TIERS.join("/")} + field + carousel), got ${irAll.length}`)
}

// ---- 1. unified globals.css (base + all-47 slot rules + @source per page) ---
const base = readFileSync("probes/h4/globals.css", "utf8")
  .replace('@source "./demo.html";\n', "")
const cssParts = []
for (const ir of irAll) {
  const css = componentCss(ir)
  if (!css.rules.length) continue
  cssParts.push(wrapComponentCss(ir.name, css))
}
// NOTE on the repo-wide auto-scan (out.css compiles from the repo root):
// it is LOAD-BEARING for the docs site — authored docs/demos/*.html pages
// load out.css in iframes and their utilities are only picked up by that
// scan. `@source not` cannot carve tool/source dirs out of it: any
// `@source not` disables automatic detection entirely (empirically verified
// 2026-08-26 — the -4041-line out.css regression). The residual coupling —
// utility-like STRING literals in tracked files surface as unused rules —
// is why tools/unit fixtures concatenate their tokens at runtime.
const sources = names.map((n) => `@source "./components/${n}.html";`).join("\n")
writeFileSync(`${DIST}/globals.css`,
  base + "\n" + sources + "\n\n" + SHADLESS_CSS_FIXES + "\n\n" + cssParts.join("\n\n") +
  "\n@layer base { body { @apply bg-background text-foreground p-8; } }\n")

// ---- 2. copy shared assets into dist/ --------------------------------------
buildJs(DIST)

// ---- 3. per-component demo pages -------------------------------------------
// path rewrites live in tools/demo-lib.mjs (shared, unit-tested)

const KERNEL_T6 = ["alert-dialog", "context-menu", "dropdown-menu", "hover-card",
  "popover", "scroll-area", "select", "sheet", "slider", "tabs", "tooltip"]
const TRIVIAL_T7 = ["accordion", "aspect-ratio", "avatar", "checkbox", "collapsible",
  "label", "progress", "radio-group", "separator", "switch", "toggle", "toggle-group"]

let emitted = 0
for (const name of names) {
  const ir = byName.get(name)
  let html
  if (ir.tier === "static") {
    // already produced by the emitter — leave as-is
    if (!existsSync(`${DIST}/components/${name}.html`))
      throw new Error(`static page missing: ${name} (run npm run emit first)`)
    emitted++
    continue
  } else if (ir.tier === "kernel") {
    if (name === "dialog" || KERNEL_T6.includes(name)) {
      html = rewritePaths(readFileSync(`src/kernel/${name}.html`, "utf8"))
    } else throw new Error(`no kernel fixture for ${name}`)
  } else if (MEDIUM_EMIT.has(name)) {
    if (name === "menubar" || name === "navigation-menu") html = ensureLink(rewritePaths(readFileSync(`src/kernel/${name}.html`, "utf8")))
    else throw new Error(`no medium fixture for ${name}`)
  } else if (ir.tier === "trivial-js") {
    if (!TRIVIAL_T7.includes(name)) throw new Error(`no trivial fixture for ${name}`)
    html = ensureLink(rewritePaths(readFileSync(`probes/t7/${name}.html`, "utf8")))
  } else if (LOGIC_PRESENTATIONAL.has(name)) {
    if (name === "field") html = fieldDemoHtml()
    else throw new Error(`no presentational fixture for ${name}`)
  } else if (EXTERNAL_EMIT.has(name)) {
    if (name === "carousel") html = ensureLink(rewritePaths(readFileSync("probes/t8/carousel.html", "utf8")))
    else throw new Error(`no external fixture for ${name}`)
  } else throw new Error(`unhandled tier ${ir.tier}`)
  // FT9: every demo HTML ships with the theme pre-paint script so it
  // flips dark mode independently of any host page. Idempotent.
  writeFileSync(`${DIST}/components/${name}.html`, injectPrePaint(html))
  emitted++
}
if (emitted !== 51) throw new Error(`emitted ${emitted}, expected 51`)

// ---- 4. demo index ----------------------------------------------------------
const groups = [
  ["static", "Static (markup + CSS)"],
  ["kernel", "Kernel (base + per-component behavior)"],
  ["trivial-js", "Trivial (shadless runtime)"],
  ["logic", "Presentational logic (markup + CSS)"],
  ["external", "External (vanilla port)"],
]
const indexHtml = `<!doctype html><html><head><meta charset="utf-8"><title>shadless demo</title>
<link rel="stylesheet" href="out.css"></head>
<body>
<h1>shadless demo</h1>
${groups.map(([tier, label]) => {
  const ns = irAll.filter((ir) => ir.tier === tier || (tier === "kernel" && MEDIUM_EMIT.has(ir.name))).map((ir) => ir.name).sort()
  return `<h2>${label} <small>(${ns.length})</small></h2><ul>${
    ns.map((n) => `<li><a href="components/${n}.html">${n}</a></li>`).join("")}</ul>`
}).join("\n")}
</body></html>`
writeFileSync(`${DIST}/demo-index.html`, indexHtml)

// ---- field fixture (presentational; markup + cva, no JS) -------------------
function fieldDemoHtml() {
  return `<!doctype html>
<html><head><meta charset="utf-8"><title>shadless field</title>
<link rel="stylesheet" href="../out.css"></head>
<body>
  <fieldset data-slot="field-set">
    <legend data-slot="field-legend" data-variant="legend">Login</legend>

    <div data-slot="field-group">
      <div data-slot="field" class="group/field" data-orientation="vertical">
        <div data-slot="field-content" class="group/field-content">
          <label data-slot="field-label" class="group/field-label peer/field-label" for="email">Email</label>
          <input data-slot="input" id="email" type="email" placeholder="m@example.com">
          <p data-slot="field-description">We'll never share your email.</p>
        </div>
      </div>

      <div data-slot="field" class="group/field" data-orientation="vertical" data-invalid="true">
        <div data-slot="field-content" class="group/field-content">
          <label data-slot="field-label" class="group/field-label peer/field-label" for="pw">Password</label>
          <input data-slot="input" id="pw" type="password" aria-invalid="true">
          <div data-slot="field-error">Password must be at least 8 characters.</div>
        </div>
      </div>

      <div data-slot="field-separator" class="group/field-group">
        <div data-slot="separator" class="absolute inset-0 top-1/2"></div>
        <span data-slot="field-separator-content">or</span>
      </div>
    </div>
  </fieldset>
</body></html>`
}

// ---- dialog fixture (no t6/dialog — mirror sheet with dialog slots) --------

console.log(`demo: ${emitted} pages, globals.css + assets written`)
