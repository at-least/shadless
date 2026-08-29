#!/usr/bin/env node
// Generate pipeline/graph.json from gates/registry.mjs.
//
// gates/registry.mjs stays the single source of truth for WHAT the pipeline
// does. This file is where the Go runner's view of it may be FINER: a registry
// node whose work is really N independent jobs becomes N nodes, each with its
// own key.
//
// Why that matters, with the one that pays for the exercise: `contracts` is a
// single node that runs 29 components — 46% of the full tier — and it already
// spawns a child process per component, serially. As one node it re-runs all
// 29 whenever any contract def changes, and can never use more than one core.
// Fanned out, editing tools/contracts/components/dialog.mjs re-runs dialog.
//
// A fan-out is only legal when the jobs are genuinely independent: separate
// outputs, no shared mutable state, and each exits non-zero on its own
// failure. Both are true here (run.mjs already isolates each def in its own
// process precisely because a broken def used to take the others down).
import { NODES } from "../gates/registry.mjs"
import { readdirSync, writeFileSync } from "node:fs"

// id -> expander. Returns the replacement nodes; the original is dropped and
// anything that `needs` it is rewired to need all of them.
const FANOUT = {
  contracts(n) {
    const names = readdirSync("tools/contracts/components")
      .filter((f) => f.endsWith(".mjs")).map((f) => f.slice(0, -4)).sort()
    // the broad patterns cover every def, which is exactly what we are
    // narrowing; the harness files they also covered are named explicitly
    const shared = n.inputs.filter((p) =>
      p !== "tools/contracts/**/*.mjs" && p !== "tools/contracts/components/**")
    return names.map((name) => ({
      ...n,
      id: `contracts:${name}`,
      run: [["node", "tools/contracts/run.mjs", name]],
      inputs: ["tools/contracts/run.mjs", "tools/contracts/oracle-build.mjs",
               `tools/contracts/components/${name}.mjs`, ...shared],
      produces: [`tools/contracts/out/${name}`],
    }))
  },
}

const out = []
const replaced = new Map() // original id -> [replacement ids]
for (const n of NODES) {
  const fan = FANOUT[n.id]
  if (!fan) { out.push(n); continue }
  const parts = fan(n)
  replaced.set(n.id, parts.map((p) => p.id))
  out.push(...parts)
}

const rewire = (needs) => needs.flatMap((d) => replaced.get(d) ?? [d])

writeFileSync("pipeline/graph.json", JSON.stringify(out.map((n) => ({
  id: n.id, kind: n.kind, tier: n.tier,
  needs: rewire(n.needs ?? []),
  run: n.run ?? [],
  inputs: n.inputs === null ? null : (n.inputs ?? []),
  produces: n.produces ?? [],
})), null, 1) + "\n")

const fanned = [...replaced].map(([k, v]) => `${k} -> ${v.length}`).join(", ")
console.log(`graph.json: ${out.length} nodes from ${NODES.length} registry nodes` +
  (fanned ? ` (fan-out: ${fanned})` : ""))
