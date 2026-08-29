#!/usr/bin/env node
// gates/meta.mjs — the gate that tests the gates.
//
// The most expensive recurring bug class in this repo's history is not a
// broken component, it is a gate that PASSES WHEN IT SHOULD FAIL:
//
//   ed1bef4  install-import check matched only raw quotes; highlighted fences
//            carry &quot; — the gate passed vacuously over every page
//   900555a  regex LITERALS don't interpolate ${comp} — shipped id-less dead
//            pages, caught only because that tool self-tests
//   e3c2b12  sync driftScan walked the wrong path — dead code, always
//            reported "no drift"
//   244e20c  --scope=x was silently ignored, so scoped runs checked nothing
//
// Each was found by a HAND-RUN negative test, mentioned in a commit message
// and then lost. This file makes those permanent: every gate in the registry
// must declare at least one mutation, and every mutation must actually make
// its gate exit non-zero.
//
// Mutations edit real files, so the harness snapshots each declared file and
// restores it in a finally — including on crash or Ctrl-C.
//
//   node gates/meta.mjs                 every mutation (needs a fresh build)
//   node gates/meta.mjs --tier=fast     only mutations for fast-tier gates
//   node gates/meta.mjs --only=<id>     one mutation
//   node gates/meta.mjs --coverage      no execution: who is unproven
import { spawnSync } from "node:child_process"
import { readFileSync, writeFileSync, existsSync, unlinkSync, readdirSync } from "node:fs"
import { NODES, BY_ID, effectiveTier, tierRank, ungatedBuilds } from "./registry.mjs"

const argv = process.argv.slice(2)
const flag = (n, d = null) => {
  const eq = argv.find((a) => a.startsWith(`--${n}=`))
  if (eq) return eq.slice(n.length + 3)
  return argv.includes(`--${n}`) ? true : d
}
const tier = flag("tier", null)
const only = flag("only", null)
const coverage = flag("coverage", false)

const files = readdirSync("gates/mutations").filter((f) => f.endsWith(".mjs") && !f.startsWith("_"))
const mutations = []
for (const f of files) {
  const m = (await import(`./mutations/${f}`)).default
  m.file = `gates/mutations/${f}`
  mutations.push(m)
}
const byId = new Map(mutations.map((m) => [m.id, m]))

// ------------------------------------------------------------- coverage
const gates = NODES.filter((n) => n.kind === "gate")
const problems = []
for (const g of gates) {
  const declared = g.mutations ?? []
  if (!declared.length) problems.push(`gate ${g.id}: declares no mutation — unproven`)
  for (const id of declared) if (!byId.has(id)) problems.push(`gate ${g.id}: declares mutation "${id}" but gates/mutations/${id}.mjs does not exist`)
  if (!g.why) problems.push(`gate ${g.id}: no "why" — a gate nobody can explain cannot be reviewed`)
}
for (const m of mutations) {
  if (!BY_ID.has(m.gate)) problems.push(`${m.file}: targets unknown gate "${m.gate}"`)
  else if (!(BY_ID.get(m.gate).mutations ?? []).includes(m.id))
    problems.push(`${m.file}: gate "${m.gate}" does not list this mutation in the registry`)
  if (!m.why) problems.push(`${m.file}: no "why" — say which real bug class this proves`)
}
const ungated = ungatedBuilds()

if (coverage) {
  console.log(`gates: ${gates.length}, mutations: ${mutations.length}`)
  for (const g of gates) console.log(`  ${g.id.padEnd(22)} ${(g.mutations ?? []).join(", ") || "— UNPROVEN"}`)
  if (ungated.length) console.log(`\nbuild artifacts with no gate downstream: ${ungated.join(", ")}`)
  if (problems.length) { console.error(`\n${problems.length} problems:\n  ` + problems.join("\n  ")); process.exit(1) }
  process.exit(0)
}
if (problems.length) {
  console.error(`FAIL  meta (registry/mutation wiring)\n  ` + problems.join("\n  "))
  process.exit(1)
}

// ------------------------------------------------------------- selection
let selected = mutations
if (only) selected = mutations.filter((m) => String(only).split(",").includes(m.id))
else if (tier) {
  const max = tierRank(tier)
  selected = mutations.filter((m) => tierRank(effectiveTier(m.gate)) <= max)
}
if (!selected.length) { console.error(`no mutations selected`); process.exit(2) }

// ----------------------------------------------------------- the harness
const snapshot = (paths) => paths.map((p) => [p, existsSync(p) ? readFileSync(p) : null])
const restore = (snap) => {
  for (const [p, buf] of snap) {
    if (buf === null) { if (existsSync(p)) unlinkSync(p) }
    else writeFileSync(p, buf)
  }
}

let live = null
const cleanup = () => { if (live) { restore(live); live = null } }
process.on("SIGINT", () => { cleanup(); process.exit(130) })
process.on("SIGTERM", () => { cleanup(); process.exit(143) })
process.on("uncaughtException", (e) => { cleanup(); console.error(e); process.exit(1) })

console.log(`meta: ${selected.length} mutations${tier ? ` (tier=${tier})` : ""}\n`)
const failures = []
for (const m of selected) {
  const gate = BY_ID.get(m.gate)
  process.stdout.write(`  ${m.id.padEnd(34)} -> ${m.gate.padEnd(20)} `)
  let caught = false, note = ""
  try {
    // `files` may be a function: some mutations locate their target inside a
    // build artifact, which only exists after a build. Resolved here so
    // --coverage still works on an unbuilt tree.
    live = snapshot(typeof m.files === "function" ? m.files() : m.files)
    m.apply()
    let status = 0
    for (const cmd of gate.run) {
      const r = spawnSync(cmd[0], cmd.slice(1), { stdio: "pipe", encoding: "utf8" })
      if (r.status !== 0) { status = r.status ?? 1; break }
    }
    caught = status !== 0
  } catch (e) {
    note = ` (mutation itself errored: ${e.message.split("\n")[0]})`
  } finally {
    if (live) restore(live)
    live = null
  }
  if (caught) console.log("CAUGHT")
  else { console.log(`NOT CAUGHT${note}`); failures.push(`${m.id}: ${m.gate} stayed green under "${m.why}"${note}`) }
}

if (failures.length) {
  console.error(`\nFAIL  meta (${failures.length}/${selected.length} mutations not caught)\n  ` +
    failures.join("\n  ") +
    `\n\n  A gate that cannot fail is not a gate. Fix the gate, not the mutation.`)
  process.exit(1)
}
console.log(`\nPASS  meta (${selected.length} mutations, every one caught by its gate` +
  (ungated.length ? `; note: ungated build artifacts: ${ungated.join(", ")}` : "") + `)`)
