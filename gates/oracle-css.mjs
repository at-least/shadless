#!/usr/bin/env node
// gates/oracle-css.mjs — a stylesheet for the React oracle that owes nothing
// to src/emitter.
//
// Until now the React oracle in style-parity was styled with dist/out.css —
// OUR compiled output. That made "computed styles match the oracle" a
// tautology for every bug in the CSS emitter: the cva default-variant
// cascade (0dd7391, 30 broken cells) and the skin marker rules
// (cn-menu-translucent, five opaque menus) were both invisible to it.
//
// This builds gates/out/oracle.css from upstream's own inputs only:
//   apps/v4/app/globals.css          custom variants, @theme, :root/.dark tokens
//   packages/shadcn/src/tailwind.css what @import "shadcn/tailwind.css" resolves to
//   apps/v4/registry/styles/style-nova.css   the skin (.style-nova .cn-* rules)
//   @source probes/out/resolved-ui   the resolved registry the oracle bundle
//                                    renders (cn-* expanded at the source,
//                                    exactly what the oracle DOM carries)
// compiled with the same tailwindcss the product uses. Nothing under src/
// is read.
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs"
import { execFileSync } from "node:child_process"
import { resolve } from "node:path"

const UP = ".upstream/shadcn-ui"
const OUT_DIR = "gates/out"
const OUT = `${OUT_DIR}/oracle.css`
mkdirSync(OUT_DIR, { recursive: true })

const app = readFileSync(`${UP}/apps/v4/app/globals.css`, "utf8")
const shadcnTw = readFileSync(`${UP}/packages/shadcn/src/tailwind.css`, "utf8")
const skin = readFileSync(`${UP}/apps/v4/registry/styles/style-nova.css`, "utf8")
const legacy = `${UP}/apps/v4/app/legacy-themes.css`

const lines = []
for (const line of app.split("\n")) {
  if (line.includes('"shadcn/tailwind.css"')) { lines.push("/* shadcn/tailwind.css (inlined from packages/shadcn/src) */", shadcnTw); continue }
  if (line.includes('"./legacy-themes.css"')) { if (existsSync(legacy)) lines.push(`@import "${resolve(legacy)}";`); continue }
  if (line.startsWith("@source ")) continue // the app's own style dirs; replaced below
  lines.push(line)
}
lines.push(`@source "${resolve("probes/out/resolved-ui")}";`)
lines.push(`@source "${resolve("tools/contracts/components")}";`) // usage trees carry example classes
// the examples' own utilities (max-w-lg on an accordion demo, …): the demo
// pages carry them inline, so the oracle stylesheet must define them
lines.push(`@source "${resolve(`${UP}/apps/v4/examples`)}";`)
lines.push("/* === style-nova.css (the pinned skin, verbatim) === */", skin)
const entry = `${OUT_DIR}/oracle.entry.css`
writeFileSync(entry, lines.join("\n"))

const cli = resolve("node_modules/.bin/tailwindcss")
execFileSync(cli, ["-i", resolve(entry), "-o", resolve(OUT)], { cwd: OUT_DIR, stdio: "inherit" })
const css = readFileSync(OUT, "utf8")
const skinRules = (css.match(/\.style-nova\s+\.cn-[\w-]+/g) ?? []).length
console.log(`oracle-css: ${OUT} (${(css.length / 1024).toFixed(0)}KB, ${skinRules} skin rules, zero bytes from src/)`)
