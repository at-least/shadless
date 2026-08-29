#!/usr/bin/env node
// gates/ir-diff.mjs — slot-level semantic diff between two IR sets.
//
// The upstream tsx diff is the wrong review surface for a re-pin: it shows
// React refactors, prop plumbing and comment changes that never reach the
// conversion. The IR is what the pipeline actually consumes, so a diff at
// that level answers the questions a re-pin review has:
//
//   which components appeared / vanished / changed tier
//   which slots appeared / vanished, which class lists changed
//   which cva axes / values / defaults changed
//
// and it is the routing signal gates/upstream.mjs uses to decide which
// overlay units and which gate failures are EXPECTED consequences of an
// upstream change versus regressions in our own pipeline.
//
//   node gates/ir-diff.mjs <git-ref>       committed IR at <ref> vs the working tree
//   node gates/ir-diff.mjs <dirA> <dirB>   two IR directories
import { execFileSync } from "node:child_process"
import { readdirSync, readFileSync, existsSync } from "node:fs"

export function loadIrFromGit(ref) {
  const out = {}
  let files
  try { files = execFileSync("git", ["ls-tree", "--name-only", ref, "src/registry/ir/"], { encoding: "utf8" }).split("\n").filter((f) => f.endsWith(".json")) }
  catch { return out }
  for (const f of files) out[f.split("/").pop().slice(0, -5)] = JSON.parse(execFileSync("git", ["show", `${ref}:${f}`], { encoding: "utf8" }))
  return out
}
export function loadIrFromDir(dir) {
  const out = {}
  if (!existsSync(dir)) return out
  for (const f of readdirSync(dir).filter((f) => f.endsWith(".json"))) out[f.slice(0, -5)] = JSON.parse(readFileSync(`${dir}/${f}`, "utf8"))
  return out
}

const slotsOf = (ir) => {
  const m = new Map() // slot -> Set(classes)
  for (const c of ir.components ?? []) for (const e of c.elements ?? []) {
    if (!e.slot) continue
    const cls = new Set((e.classes ?? []).join(" ").split(/\s+/).filter(Boolean))
    if (!m.has(e.slot)) m.set(e.slot, new Set())
    for (const t of cls) m.get(e.slot).add(t)
  }
  return m
}
const setDiff = (a, b) => ({ added: [...b].filter((x) => !a.has(x)), removed: [...a].filter((x) => !b.has(x)) })

// Returns { components: { name: { kind, ...detail } } } — only entries with a change.
export function diffIr(before, after) {
  const names = new Set([...Object.keys(before), ...Object.keys(after)])
  const components = {}
  for (const name of [...names].sort()) {
    const a = before[name], b = after[name]
    if (!a) { components[name] = { kind: "added", tier: b.tier }; continue }
    if (!b) { components[name] = { kind: "removed", tier: a.tier }; continue }
    const d = { kind: "changed", changes: [] }
    if (a.tier !== b.tier) d.changes.push({ what: "tier", from: a.tier, to: b.tier })
    const sa = slotsOf(a), sb = slotsOf(b)
    for (const s of sb.keys()) if (!sa.has(s)) d.changes.push({ what: "slot-added", slot: s })
    for (const s of sa.keys()) if (!sb.has(s)) d.changes.push({ what: "slot-removed", slot: s })
    for (const s of sa.keys()) if (sb.has(s)) {
      const { added, removed } = setDiff(sa.get(s), sb.get(s))
      if (added.length || removed.length) d.changes.push({ what: "classes", slot: s, added, removed })
    }
    const ca = a.cva ?? {}, cb = b.cva ?? {}
    for (const t of Object.keys(cb)) if (!ca[t]) d.changes.push({ what: "cva-added", table: t })
    for (const t of Object.keys(ca)) if (!cb[t]) d.changes.push({ what: "cva-removed", table: t })
    for (const t of Object.keys(ca)) if (cb[t]) {
      const va = ca[t].variants ?? {}, vb = cb[t].variants ?? {}
      for (const ax of Object.keys(vb)) if (!va[ax]) d.changes.push({ what: "cva-axis-added", table: t, axis: ax, values: Object.keys(vb[ax]) })
      for (const ax of Object.keys(va)) if (!vb[ax]) d.changes.push({ what: "cva-axis-removed", table: t, axis: ax })
      for (const ax of Object.keys(va)) if (vb[ax]) {
        const { added, removed } = setDiff(new Set(Object.keys(va[ax])), new Set(Object.keys(vb[ax])))
        if (added.length || removed.length) d.changes.push({ what: "cva-values", table: t, axis: ax, added, removed })
        for (const v of Object.keys(va[ax])) if (vb[ax][v] !== undefined && String(va[ax][v]) !== String(vb[ax][v]))
          d.changes.push({ what: "cva-value-classes", table: t, axis: ax, value: v })
      }
      const da = ca[t].defaults ?? {}, db = cb[t].defaults ?? {}
      for (const ax of new Set([...Object.keys(da), ...Object.keys(db)]))
        if (da[ax] !== db[ax]) d.changes.push({ what: "cva-default", table: t, axis: ax, from: da[ax], to: db[ax] })
      if (String(ca[t].base) !== String(cb[t].base)) d.changes.push({ what: "cva-base", table: t })
    }
    if (d.changes.length) components[name] = d
  }
  return { components }
}

export function renderIrDiff(diff) {
  const lines = []
  const entries = Object.entries(diff.components)
  if (!entries.length) return "no semantic change in the IR"
  for (const [name, d] of entries) {
    if (d.kind !== "changed") { lines.push(`${name.padEnd(20)} ${d.kind.toUpperCase()} (tier ${d.tier})`); continue }
    lines.push(`${name}`)
    for (const c of d.changes) {
      const s = c.what === "classes" ? `classes[${c.slot}] +${c.added.length} -${c.removed.length}: ${[...c.added.map((x) => "+" + x), ...c.removed.map((x) => "-" + x)].slice(0, 8).join(" ")}`
        : c.what === "cva-values" ? `cva ${c.table}.${c.axis}: ${[...c.added.map((x) => "+" + x), ...c.removed.map((x) => "-" + x)].join(" ")}`
        : c.what === "cva-default" ? `cva ${c.table}.${c.axis} default ${c.from} -> ${c.to}`
        : c.what === "cva-value-classes" ? `cva ${c.table}.${c.axis}=${c.value} classes changed`
        : c.what === "tier" ? `tier ${c.from} -> ${c.to}`
        : `${c.what} ${c.slot ?? c.table ?? ""}${c.axis ? "." + c.axis : ""}`
      lines.push(`  ${s}`)
    }
  }
  return lines.join("\n")
}

const isMain = process.argv[1] && import.meta.url.endsWith(process.argv[1].split("/").pop())
if (isMain) {
  const [a, b] = process.argv.slice(2)
  if (!a) { console.error("usage: gates/ir-diff.mjs <git-ref> | <dirA> <dirB>"); process.exit(2) }
  const before = b ? loadIrFromDir(a) : loadIrFromGit(a)
  const after = b ? loadIrFromDir(b) : loadIrFromDir("src/registry/ir")
  console.log(renderIrDiff(diffIr(before, after)))
}
