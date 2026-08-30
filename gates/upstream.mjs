#!/usr/bin/env node
// gates/upstream.mjs — the re-pin drill. One command from "a new shadcn
// release exists" to "green, or a classified report with task packets".
//
//   node gates/upstream.mjs --to=shadcn@4.20.0            full drill
//   node gates/upstream.mjs --to=shadcn@4.20.0 --fetch    fetch tags first (network)
//   node gates/upstream.mjs --to=shadcn@4.19.0            same tag: must be green (self-test)
//   node gates/upstream.mjs --report-only                 re-classify the last run
//
// Steps, each recorded in build/gates/upstream-report.md:
//   1. checkout the tag in .upstream, re-record src/registry/pin.json
//   2. gates/ledger.mjs --dissolve  — every auto-dissolve exemption is deleted;
//      the rebuild has to re-earn each one with evidence
//   3. overlays/upstream/*.patch applied with git apply --3way
//   4. gates/run.mjs --tier=full --keep-going — the WHOLE picture, not the first red
//   5. IR semantic diff (gates/ir-diff.mjs) old pin -> new pin
//   6. gates/overlay.mjs --report + --tasks — stale/orphaned manual work, as packets
//   7. classify every failed gate: EXPECTED (its components changed upstream)
//      or UNEXPECTED (nothing upstream moved — our pipeline regressed)
//
// Nothing here needs a human to remember a checklist. What it cannot do
// (re-author a stale glue file, decide whether a new upstream slot needs a
// runtime) it hands over as a task packet with the exact diff and the gates
// that must be green afterwards.
import { execFileSync, spawnSync } from "node:child_process"
import { readFileSync, writeFileSync, existsSync, readdirSync, mkdirSync, cpSync, rmSync } from "node:fs"
import { diffIr, loadIrFromDir, renderIrDiff } from "./ir-diff.mjs"

const UP = ".upstream/shadcn-ui"
const OUT = "build/gates"
const argv = process.argv.slice(2)
const flag = (n, d = null) => { const eq = argv.find((a) => a.startsWith(`--${n}=`)); if (eq) return eq.slice(n.length + 3); return argv.includes(`--${n}`) ? true : d }
const to = flag("to", null)
const reportOnly = flag("report-only", false)
const noBuild = flag("no-build", false)

const git = (args, opts = {}) => execFileSync("git", ["-C", UP, ...args], { encoding: "utf8", ...opts }).trim()
const run = (cmd, args) => spawnSync(cmd, args, { stdio: "inherit" }).status === 0
const pin = () => JSON.parse(readFileSync("src/registry/pin.json", "utf8"))
mkdirSync(OUT, { recursive: true })

const step = (title) => console.log(`\n##### upstream: ${title}`)
const report = []
const H = (s) => report.push(`\n## ${s}\n`)
const P = (s) => report.push(s)

// ------------------------------------------------------------------ 0. state
const from = pin().shadcn_ui
const REGISTRY = from.registry
if (!to && !reportOnly) { console.error("usage: gates/upstream.mjs --to=shadcn@X.Y.Z [--fetch] [--no-build]"); process.exit(2) }

if (!reportOnly) {
  // ------------------------------------------------------------- 1. checkout
  step(`checkout ${to}`)
  if (flag("fetch", false)) { try { git(["fetch", "--tags", "--quiet"]) } catch (e) { console.error(`fetch failed (offline?): ${e.message.split("\n")[0]}`) } }
  const dirty = git(["status", "--porcelain"])
  if (dirty) {
    // an applied overlay series leaves the tree dirty by design; anything
    // else is a hand-edit that would be lost
    const series = existsSync("overlays/upstream") ? readdirSync("overlays/upstream").filter((f) => f.endsWith(".patch")) : []
    if (!series.length) { console.error(`.upstream has uncommitted changes and no overlay series explains them:\n${dirty}\n  reset it or turn the change into overlays/upstream/*.patch`); process.exit(1) }
    git(["checkout", "--", "."])
  }
  try { git(["checkout", "--quiet", to]) } catch (e) { console.error(`cannot checkout ${to}: ${e.message.split("\n").slice(-1)[0]}  (try --fetch)`); process.exit(1) }
  cpSync("src/registry/ir", `${OUT}/ir-before`, { recursive: true, force: true })
  if (!run("./build/pipeline", ["pin", "--force"])) process.exit(1)
  const toPin = pin().shadcn_ui
  H(`Re-pin ${from.tag} → ${toPin.tag}`)
  P(`- from: \`${from.tag}\` (${from.commit.slice(0, 10)})\n- to:   \`${toPin.tag}\` (${toPin.commit.slice(0, 10)})`)
  const commits = git(["log", "--oneline", `${from.commit}..${toPin.commit}`]).split("\n").filter(Boolean)
  P(`- upstream commits in range: ${commits.length}`)

  // ------------------------------------------------------------- 2. dissolve
  step("dissolve auto-dissolve exemptions")
  run("node", ["gates/ledger.mjs", "--dissolve"])

  // ------------------------------------------------------- 3. source patches
  step("apply overlays/upstream")
  const conflicts = []
  const series = existsSync("overlays/upstream") ? readdirSync("overlays/upstream").filter((f) => f.endsWith(".patch")).sort() : []
  for (const f of series) {
    const r = spawnSync("git", ["-C", UP, "apply", "--3way", `${process.cwd()}/overlays/upstream/${f}`], { encoding: "utf8" })
    if (r.status !== 0) conflicts.push({ f, out: (r.stdout + r.stderr).trim() })
  }
  console.log(`  ${series.length - conflicts.length}/${series.length} patches applied${conflicts.length ? `, ${conflicts.length} CONFLICT` : ""}`)
  writeFileSync(`${OUT}/upstream-conflicts.json`, JSON.stringify(conflicts, null, 2))

  // ------------------------------------------------------------------ 4. run
  if (!noBuild) {
    step("full tier, keep going")
    run("node", ["gates/run.mjs", "--tier=full", "--keep-going"])
  }
}

// ------------------------------------------------------------- 5. classify
step("classify")
const toPin = pin().shadcn_ui
const changedFiles = (() => {
  try {
    return git(["diff", "--name-status", "--no-renames", `${from.commit}..${toPin.commit}`, "--",
      REGISTRY, "apps/v4/examples", "apps/v4/registry/styles/style-nova.css", "apps/v4/content/docs/components/radix",
      "apps/v4/app/globals.css"]).split("\n").filter(Boolean).map((l) => { const [s, p] = l.split("\t"); return { status: s, path: p } })
  } catch { return [] }
})()
const compOf = (p) => {
  const base = p.split("/").pop().replace(/\.(tsx|mdx|css)$/, "")
  if (p.startsWith(REGISTRY)) return base
  if (p.includes("/examples/")) return base.replace(/-(demo|basic|rtl|.*)$/, (m) => m) // keep full, map below
  return base
}
const registryNames = readdirSync(`${UP}/${REGISTRY}`).filter((f) => f.endsWith(".tsx")).map((f) => f.slice(0, -4))
const componentOfExample = (name) => registryNames.filter((r) => name === r || name.startsWith(r + "-")).sort((a, b) => b.length - a.length)[0] ?? name
const changedComponents = new Set(changedFiles.map((f) => componentOfExample(compOf(f.path))))

const irBefore = loadIrFromDir(`${OUT}/ir-before`)
const irAfter = loadIrFromDir("src/registry/ir")
const irDiff = diffIr(irBefore, irAfter)
for (const n of Object.keys(irDiff.components)) changedComponents.add(n)

H("Upstream changes in scope")
P(changedFiles.length ? changedFiles.map((f) => `- ${f.status} \`${f.path}\``).join("\n") : "- none")
H("IR semantic diff")
P("```\n" + renderIrDiff(irDiff) + "\n```")

const runReport = existsSync(`${OUT}/run-report.json`) ? JSON.parse(readFileSync(`${OUT}/run-report.json`, "utf8")) : { failed: {}, blocked: [], passed: [] }
H("Gates")
P(`- passed: ${runReport.passed.length}\n- failed: ${Object.keys(runReport.failed).length}\n- blocked: ${runReport.blocked.length}`)
const expected = [], unexpected = []
for (const [id, f] of Object.entries(runReport.failed)) {
  const mentioned = registryNames.filter((n) => new RegExp(`\\b${n}\\b`).test(f.tail))
  const hits = mentioned.filter((n) => changedComponents.has(n))
  const entry = `### ${id}\n\n${hits.length ? `EXPECTED — upstream changed: ${hits.join(", ")}` : mentioned.length ? `UNEXPECTED — mentions ${mentioned.slice(0, 6).join(", ")}, none changed upstream` : "UNEXPECTED — no component attribution; read the tail"}\n\n\`\`\`\n${f.tail}\n\`\`\`\nrepro: \`node gates/run.mjs --only=${id}\``
  ;(hits.length ? expected : unexpected).push(entry)
}
if (unexpected.length) { H(`UNEXPECTED failures (${unexpected.length}) — our pipeline, not upstream`); P(unexpected.join("\n\n")) }
if (expected.length) { H(`EXPECTED failures (${expected.length}) — consequences of upstream changes`); P(expected.join("\n\n")) }
if (runReport.blocked.length) P(`\nblocked (a dependency failed): ${runReport.blocked.join(", ")}`)

// --------------------------------------------------------------- 6. overlay
step("overlay audit + task packets")
spawnSync("node", ["gates/overlay.mjs", "--tasks"], { stdio: "inherit" })
const tasks = existsSync(`${OUT}/tasks`) ? readdirSync(`${OUT}/tasks`) : []
const conflicts = existsSync(`${OUT}/upstream-conflicts.json`) ? JSON.parse(readFileSync(`${OUT}/upstream-conflicts.json`, "utf8")) : []
H("Manual work")
P(tasks.length ? tasks.map((t) => `- \`${OUT}/tasks/${t}\``).join("\n") : "- none: every manual intervention still applies")
if (conflicts.length) P(conflicts.map((c) => `- CONFLICT \`overlays/upstream/${c.f}\`\n\`\`\`\n${c.out}\n\`\`\``).join("\n"))

H("Next")
P([
  "1. read UNEXPECTED failures first — those are ours",
  "2. work the task packets (each names the gates that must be green)",
  "3. `node gates/ledger.mjs --record` for exemptions that legitimately survive; `node gates/overlay.mjs --record` after re-authoring",
  "4. `make upstream-snapshot` (network) to refresh the ui.shadcn.com golden snapshot for the new release",
  "5. `make` must be green; then commit source + regenerated output together (the dist/ diff IS the review)",
].map((s) => `- ${s}`).join("\n"))

const md = `# Upstream drill report\n` + report.join("\n") + "\n"
writeFileSync(`${OUT}/upstream-report.md`, md)
const green = !Object.keys(runReport.failed).length && !tasks.length && !conflicts.length
console.log(`\n${green ? "PASS " : "REPORT"} upstream ${from.tag} → ${toPin.tag}: ${runReport.passed.length} passed, ${Object.keys(runReport.failed).length} failed (${unexpected.length} unexpected), ${tasks.length} task packets, ${conflicts.length} conflicts\n  ${OUT}/upstream-report.md`)
process.exit(green ? 0 : 1)
