#!/usr/bin/env node
// gates/dist-complete.mjs — the tracked no-build stylesheet must carry every
// component's slot rules.
//
// Why (2026-08-29): dist/out.css had been committed from a state where only
// the emitter's 23 static pages existed — verify:medium's emit step wipes
// dist/components and rewrites out.css, and the interactive pages come back
// only from the full `npm run demo`. The file looked plausible (thousands of
// lines) yet 123 slot selectors from 25 components were gone, so anyone on
// the no-build path had unstyled dialogs, menus, selects, … Nothing caught
// it: product-verify checks shadless.full.css, docs-consistency compares
// copies of out.css with each other, neither asks whether out.css is whole.
//
// Oracle: dist/css/<name>.css is the per-component @apply source (tracked,
// one per component); every `[data-slot="…"]` selector it declares must
// appear in dist/out.css. Fast tier, no build: it judges the committed files.
import { readFileSync, readdirSync, existsSync } from "node:fs"

const OUT = "dist/out.css"
if (!existsSync(OUT)) { console.error(`FAIL  dist-complete: ${OUT} missing`); process.exit(1) }
const out = readFileSync(OUT, "utf8")
const missing = []
let selectors = 0, files = 0
for (const f of readdirSync("dist/css").filter((x) => x.endsWith(".css")).sort()) {
  files++
  const src = readFileSync(`dist/css/${f}`, "utf8")
  for (const sel of new Set(src.match(/\[data-slot="[^"]+"\]/g) ?? [])) {
    selectors++
    if (!out.includes(sel)) missing.push(`${f.slice(0, -4)}: ${sel}`)
  }
}
if (missing.length) {
  const comps = new Set(missing.map((m) => m.split(":")[0]))
  console.error(`FAIL  dist-complete: ${OUT} lacks ${missing.length} slot selectors from ${comps.size} components ` +
    `(${[...comps].slice(0, 6).join(", ")}${comps.size > 6 ? ", …" : ""})\n` +
    `  ${missing.slice(0, 8).join("\n  ")}${missing.length > 8 ? "\n  …" : ""}\n` +
    `  out.css was built from a partial dist/components — run the full \`npm run demo\` and commit its out.css`)
  process.exit(1)
}
console.log(`PASS  dist-complete (${selectors} slot selectors from ${files} component sources all present in ${OUT})`)
