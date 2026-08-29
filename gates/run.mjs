#!/usr/bin/env node
// gates/run.mjs — the only way the pipeline is executed.
//
// Replaces tools/verify.mjs (a hand-ordered 23-entry array) and the
// hand-ordered Makefile recipes. Order comes from `needs` in the registry,
// so adding a step can no longer land it in the wrong place, and a targeted
// run builds exactly its transitive closure instead of everything.
//
//   node gates/run.mjs --tier=fast            gates that need no browser (~s)
//   node gates/run.mjs --tier=medium          + compiles, still no browser
//   node gates/run.mjs --tier=full            everything (playwright)
//   node gates/run.mjs --all                  every node, builds included
//   node gates/run.mjs --only=path-parity     one gate + exactly what it needs
//   node gates/run.mjs --gates-only --tier=full   assume artifacts are fresh
//   node gates/run.mjs --builds-only --tier=full  artifacts only (meta runs gates itself)
//   node gates/run.mjs --keep-going --tier=full   don't stop at the first red; write build/gates/run-report.json
//   node gates/run.mjs --list                 the graph, no execution
import { spawnSync } from "node:child_process"
import { mkdirSync, writeFileSync } from "node:fs"
import { NODES, BY_ID, plan, planTier, effectiveTier } from "./registry.mjs"

const argv = process.argv.slice(2)
const flag = (n, d = null) => {
  const eq = argv.find((a) => a.startsWith(`--${n}=`))
  if (eq) return eq.slice(n.length + 3)
  return argv.includes(`--${n}`) ? true : d
}

const tier = flag("tier", null)
const only = flag("only", null)
const all = flag("all", false)
const gatesOnly = flag("gates-only", false)
const buildsOnly = flag("builds-only", false)
const list = flag("list", false)
const dryRun = flag("dry-run", false)
// --keep-going: run every node even after a failure (the re-pin drill wants
// the whole picture, not the first red). Failed nodes are skipped as
// dependencies: a gate whose build failed is reported as blocked, not run.
const keepGoing = flag("keep-going", false)

let selected
if (only) {
  const ids = String(only).split(",").map((s) => s.trim()).filter(Boolean)
  for (const id of ids) if (!BY_ID.has(id)) {
    console.error(`unknown node: ${id}\nknown: ${NODES.map((n) => n.id).join(", ")}`)
    process.exit(2)
  }
  selected = plan(ids)
} else if (all) {
  selected = plan(NODES.map((n) => n.id))
} else if (tier) {
  if (!["fast", "medium", "full"].includes(tier)) {
    console.error(`unknown tier: ${tier} (fast|medium|full)`)
    process.exit(2)
  }
  // --tier=full is the release chain: every artifact, every gate. Lower tiers
  // pull in only what their gates depend on.
  selected = tier === "full" ? plan(NODES.map((n) => n.id)) : planTier(tier)
} else {
  console.error("usage: gates/run.mjs --tier=fast|medium|full | --only=<id>[,<id>] | --all | --list")
  process.exit(2)
}

if (gatesOnly) selected = selected.filter((n) => n.kind === "gate")
if (buildsOnly) selected = selected.filter((n) => n.kind === "build")

if (list) {
  const width = Math.max(...selected.map((n) => n.id.length))
  for (const n of selected) {
    const eff = effectiveTier(n.id)
    console.log(`${n.kind === "gate" ? "GATE " : "build"} ${n.id.padEnd(width)}  [${eff}` +
      `${eff === n.tier ? "" : ` (self ${n.tier})`}]` +
      (n.needs.length ? `  needs: ${n.needs.join(", ")}` : ""))
  }
  console.log(`\n${selected.length} nodes (${selected.filter((n) => n.kind === "gate").length} gates)`)
  process.exit(0)
}

const label = only ? `only=${only}` : all ? "all" : `tier=${tier}`
console.log(`gates: ${selected.length} nodes (${label})\n`)

const t0 = Date.now()
const timings = []
const failed = new Map() // id -> { cmd, tail }
const blocked = []
for (const n of selected) {
  const started = Date.now()
  const dead = n.needs.find((d) => failed.has(d) || blocked.includes(d))
  if (dead) { blocked.push(n.id); console.log(`=== ${n.id} === BLOCKED (needs ${dead})`); continue }
  console.log(`=== ${n.kind === "gate" ? "GATE" : "build"} ${n.id} ===`)
  if (dryRun) { console.log(`  (dry-run) ${n.run.map((c) => c.join(" ")).join(" && ")}`); continue }
  for (const cmd of n.run) {
    const r = spawnSync(cmd[0], cmd.slice(1), keepGoing ? { stdio: ["inherit", "pipe", "pipe"], encoding: "utf8" } : { stdio: "inherit" })
    if (keepGoing) { if (r.stdout) process.stdout.write(r.stdout); if (r.stderr) process.stderr.write(r.stderr) }
    if (r.status !== 0) {
      const secs = ((Date.now() - t0) / 1000).toFixed(1)
      console.error(`\nFAIL  ${n.id}  (after ${secs}s)`)
      console.error(`\n  why this gate exists:\n    ${n.why}`)
      console.error(`\n  reproduce just this node (rebuilds only what it needs):`)
      console.error(`    node gates/run.mjs --only=${n.id}`)
      console.error(`  run the failing command alone:`)
      console.error(`    ${cmd.join(" ")}`)
      if (!keepGoing) process.exit(1)
      failed.set(n.id, { cmd: cmd.join(" "), tail: ((r.stdout ?? "") + (r.stderr ?? "")).split("\n").filter(Boolean).slice(-25).join("\n") })
      break
    }
  }
  timings.push([n.id, Date.now() - started])
}
if (keepGoing) {
  const report = { failed: Object.fromEntries(failed), blocked, passed: timings.map(([id]) => id).filter((id) => !failed.has(id)) }
  mkdirSync("build/gates", { recursive: true })
  writeFileSync("build/gates/run-report.json", JSON.stringify(report, null, 2) + "\n")
  if (failed.size) {
    console.error(`\nFAIL  gates (${failed.size} failed: ${[...failed.keys()].join(", ")}` +
      (blocked.length ? `; ${blocked.length} blocked: ${blocked.join(", ")}` : "") + `)  — report: build/gates/run-report.json`)
    process.exit(1)
  }
}

const total = ((Date.now() - t0) / 1000).toFixed(1)
const slow = timings.filter(([, ms]) => ms > 5000).sort((a, b) => b[1] - a[1]).slice(0, 8)
console.log(`\nPASS  gates (${selected.length} nodes, ${label}, ${total}s)`)
if (slow.length) console.log(`  slowest: ${slow.map(([id, ms]) => `${id} ${(ms / 1000).toFixed(1)}s`).join("  ")}`)
