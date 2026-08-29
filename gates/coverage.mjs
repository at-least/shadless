#!/usr/bin/env node
// gates/coverage.mjs — the product surface as a matrix, and which gate
// covers each cell.
//
// Every bug in this repo's history sat in a cell no gate had been written
// for: the dead-button pages (state=open, gate=none), the invisible ghost
// buttons (path=css-import × theme=light, gate=none), the font parity
// (chrome), the opaque menus (skin rules, oracle circular). Gates were
// added one bug at a time. This file enumerates the cells up front so
// "what is still unverified" is a number that can only go down, instead of
// something the next downstream consumer discovers.
//
//   component × path × theme × dir × state
//     path   demo-inline | css-import | full-css
//     theme  light | dark
//     dir    ltr | rtl
//     state  closed | open       (open only for components with behavior)
//
// A cell is COVERED when some gate makes a computed-style or behavioral
// assertion about it; presence-only checks (a rule exists, a macro parses)
// are recorded as "shallow", not covered. The count of UNCOVERED cells is
// budgeted in gates/ledger.json.
//
//   node gates/coverage.mjs            summary + gates/out/coverage.json
//   node gates/coverage.mjs --cells    every uncovered cell
//   node gates/coverage.mjs --check    the gate: UNCOVERED must not exceed the budget in
//                                      gates/ledger.json, and a shrink must be recorded
//   node gates/coverage.mjs --record   record the current count as the budget
import { readdirSync, readFileSync, existsSync, writeFileSync, mkdirSync } from "node:fs"

const tiers = JSON.parse(readFileSync("probes/out/tiers.json", "utf8"))
const ir = (n) => JSON.parse(readFileSync(`src/registry/ir/${n}.json`, "utf8"))
const components = Object.entries(tiers).filter(([, t]) => t.tier !== "external" && t.tier !== "logic").map(([n]) => n)
  .filter((n) => existsSync(`src/registry/ir/${n}.json`)).sort()
const contractDefs = new Set(readdirSync("tools/contracts/components").filter((f) => f.endsWith(".mjs")).map((f) => f.slice(0, -4).replace(/-multiple$/, "")))
const cvaComps = new Set(components.filter((n) => Object.keys(ir(n).cva ?? {}).length))
const oracleOwned = new Set(JSON.parse(readFileSync("docs/example-oracle.json", "utf8")).map((t) => t.name.replace(/-demo$/, "").replace(/-.*$/, "")))
const oracleDemoOf = (n) => JSON.parse(readFileSync("docs/example-oracle.json", "utf8")).some((t) => t.name === `${n}-demo` || t.name.startsWith(`${n}-`))
const rtlDemoOf = (n) => existsSync(`docs/demos/${n}-rtl.html`)
const hasBehavior = (n) => tiers[n].tier !== "static"
// components whose class lists style an attribute-driven state (see
// gates/path-parity.mjs stateConfigs — same token shapes)
const stateTokens = new Set(components.filter((n) => {
  const j = ir(n)
  const all = [...j.components.flatMap((c) => c.elements.flatMap((e) => e.classes)),
    ...Object.values(j.cva ?? {}).flatMap((t) => [t.base, ...Object.values(t.variants ?? {}).flatMap((v) => Object.values(v))])].join(" ")
  return /(^|\s|:)(data-(open|closed|checked|unchecked|active|selected|disabled|horizontal|vertical|inset|highlighted|empty|pressed)|data-\[(?!slot=|variant=|size=)[\w-]+(=[\w-]+)?\]|aria-(expanded|invalid|checked|disabled|pressed|selected|current)|aria-\[[\w-]+=[\w-]+\]):/.test(all)
}))
const sweepSrc = readFileSync("tools/interactivity-sweep.mjs", "utf8")
const knownDead = new Set((sweepSrc.match(/const KNOWN_DEAD = new Set\(\[([\s\S]*?)\]\)/)?.[1] ?? "").split(",").map((s) => s.trim().replace(/^["']|["']$/g, "")).filter(Boolean))

const PATHS = ["demo-inline", "css-import", "full-css"]
const THEMES = ["light", "dark"], DIRS = ["ltr", "rtl"]

const cells = []
for (const c of components) for (const path of PATHS) for (const theme of THEMES) for (const dir of DIRS)
  for (const state of hasBehavior(c) ? ["closed", "open"] : ["closed"]) {
    const by = [], shallow = []
    // demo-inline: the shipped demo pages with inline utilities
    if (path === "demo-inline") {
      if (theme === "light" && dir === "ltr") {
        if (oracleDemoOf(c)) by.push("example-gate", "golden-gate")         // DOM parity, live + local oracle
        if (contractDefs.has(c)) by.push("contracts")                        // behavior, both states
        if (state === "open" && hasBehavior(c) && !knownDead.has(c) && oracleDemoOf(c)) by.push("interactivity-sweep")
        if (state === "closed") shallow.push("demo-smoke")
      }
      if (contractDefs.has(c)) by.push("style-parity")                       // computed style, theme x dir matrix, both states
      if (state === "closed" && oracleDemoOf(c)) by.push("demo-parity")       // shipped demo DOM vs upstream css, theme x dir
    }
    if (path === "demo-inline" && theme === "light" && dir === "rtl" && state === "closed" && rtlDemoOf(c)) shallow.push("docs-smoke", "css-direction")
    // css-import: slot-only markup + per-component css
    // path-parity renders every slot at rest AND in every attribute-driven
    // state its class list styles (data-state=open, aria-expanded, …) — the
    // open state of a component whose classes carry no state variant is
    // purely behavioral and stays uncovered here
    if (path === "css-import" || path === "full-css") {
      if (state === "closed" || stateTokens.has(c)) by.push("path-parity")
    }
    // template macros
    cells.push({ component: c, path, theme, dir, state, covered_by: by, shallow })
  }

const covered = cells.filter((x) => x.covered_by.length)
const shallowOnly = cells.filter((x) => !x.covered_by.length && x.shallow.length)
const uncovered = cells.filter((x) => !x.covered_by.length && !x.shallow.length)
const byDim = (key) => {
  const m = {}
  for (const x of uncovered) m[x[key]] = (m[x[key]] ?? 0) + 1
  return Object.entries(m).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k}=${v}`).join("  ")
}
mkdirSync("gates/out", { recursive: true })
writeFileSync("gates/out/coverage.json", JSON.stringify({ total: cells.length, covered: covered.length, shallow: shallowOnly.length, uncovered: uncovered.length, cells }, null, 1))

if (process.argv.includes("--cells")) for (const x of uncovered) console.log(`${x.component} ${x.path} ${x.theme} ${x.dir} ${x.state}`)
console.log(`coverage: ${cells.length} cells over ${components.length} components — ` +
  `${covered.length} covered (computed/behavioral), ${shallowOnly.length} shallow (presence only), ${uncovered.length} UNCOVERED`)
console.log(`  uncovered by path:  ${byDim("path")}`)
console.log(`  uncovered by theme: ${byDim("theme")}`)
console.log(`  uncovered by dir:   ${byDim("dir")}`)
console.log(`  uncovered by state: ${byDim("state")}`)
console.log(`  detail: gates/out/coverage.json  (--cells lists them)`)

const LEDGER = "gates/ledger.json"
const KEY = "coverage.uncovered-cells"
const ledger = JSON.parse(readFileSync(LEDGER, "utf8"))
if (process.argv.includes("--record")) {
  ledger.budgets[KEY] = { max: uncovered.length, target: 0, class: "debt",
    reason: "cells of the product matrix (component x path x theme x dir x state) no gate makes a computed-style or behavioral assertion about; see gates/coverage.mjs" }
  writeFileSync(LEDGER, JSON.stringify(ledger, null, 2) + "\n")
  console.log(`coverage: budget ${KEY} recorded = ${uncovered.length}`)
} else if (process.argv.includes("--check")) {
  const b = ledger.budgets[KEY]
  if (!b) { console.error(`FAIL  coverage: no budget ${KEY} in ${LEDGER} — run node gates/coverage.mjs --record`); process.exit(1) }
  if (uncovered.length > b.max) { console.error(`FAIL  coverage: ${uncovered.length} uncovered cells > budget ${b.max} — a gate or a contract def was lost, or a new component landed unverified`); process.exit(1) }
  if (uncovered.length < b.max) { console.error(`FAIL  coverage: ${uncovered.length} uncovered cells < budget ${b.max} — coverage improved; record it: node gates/coverage.mjs --record`); process.exit(1) }
  console.log(`PASS  coverage (${uncovered.length} uncovered cells, at budget; ${covered.length} covered)`)
}
