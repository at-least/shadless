#!/usr/bin/env node
// gates/wireit.mjs — derive the incremental runner's config from the registry.
//
// gates/run.mjs executes the graph but re-runs every node every time: the
// registry knew each node's `needs` and `produces`, not its INPUTS, so
// nothing could say "unchanged since the last green run". Wireit
// (github.com/google/wireit) is that missing half — per-script `files`
// (inputs) + `output` + `dependencies`, content-hash freshness, output
// caching (local + GitHub Actions), and independent scripts run in parallel.
//
// This file keeps gates/registry.mjs the single source of truth: every node
// becomes an npm script `w:<id>` whose wireit entry is generated from the
// node's run / needs / inputs / produces, plus four aggregators:
//
//   npm run w:fast | w:medium | w:full   the tiers (== make fast/medium/build)
//   npm run w:builds                      every artifact, no gates (meta's prelude)
//   npm run w:<id>                        one node + exactly what it needs
//
// A node with `inputs: null` (pin, reproducible) declares no `files`, so
// wireit never considers it fresh — it judges state outside the tree.
//
//   node gates/wireit.mjs --write    regenerate package.json
//   node gates/wireit.mjs --check    fail if package.json drifted (the wireit-sync gate)
//
// Gotchas, recorded so nobody rediscovers them:
//   - gates/meta.mjs invokes gate commands DIRECTLY (node.run), never through
//     wireit, so a mutation can't be skipped as "fresh". Keep it that way.
//   - `clean: false` everywhere: the tools own their output dirs (emit wipes
//     dist/components itself; several nodes write into the same dir), and
//     wireit's default pre-run delete would fight them.
//   - WIREIT_PARALLEL defaults to 2×cpus; playwright nodes each own a
//     chromium, so the Makefile and CI pin it to 4.
import { readFileSync, writeFileSync } from "node:fs"
import { NODES, planTier } from "./registry.mjs"

const argv = process.argv.slice(2)
const write = argv.includes("--write")
const check = argv.includes("--check")
if (write === check) { console.error("usage: node gates/wireit.mjs --write | --check"); process.exit(2) }

const script = (id) => `w:${id}`
// produces entries name files, globs, or directories; wireit wants globs
const outGlob = (p) => (/[*.]/.test(p.split("/").pop()) ? p : `${p}/**`)

const config = {}
for (const n of NODES) {
  const entry = {
    command: n.run.map((c) => c.join(" ")).join(" && "),
    dependencies: n.needs.map(script),
    output: (n.produces ?? []).map(outGlob),
    clean: false,
  }
  if (n.inputs) entry.files = n.inputs
  if (!entry.dependencies.length) delete entry.dependencies
  config[script(n.id)] = entry
}
const ids = (nodes) => nodes.map((n) => script(n.id))
config["w:fast"] = { dependencies: ids(planTier("fast")) }
config["w:medium"] = { dependencies: ids(planTier("medium")) }
config["w:full"] = { dependencies: ids(NODES) }
config["w:builds"] = { dependencies: ids(NODES.filter((n) => n.kind === "build")) }

const pkgPath = "package.json"
const pkg = JSON.parse(readFileSync(pkgPath, "utf8"))
const scripts = Object.fromEntries(Object.entries(pkg.scripts ?? {}).filter(([k]) => !k.startsWith("w:")))
for (const k of Object.keys(config)) scripts[k] = "wireit"
const next = { ...pkg, scripts, wireit: config }
const text = JSON.stringify(next, null, 2) + "\n"

if (write) {
  writeFileSync(pkgPath, text)
  console.log(`wireit: ${Object.keys(config).length} scripts written to package.json (${NODES.length} nodes + 4 aggregators)`)
} else {
  const current = readFileSync(pkgPath, "utf8")
  if (current !== text) {
    console.error("FAIL  wireit-sync (package.json's wireit config does not match gates/registry.mjs)\n" +
      "  regenerate: node gates/wireit.mjs --write")
    process.exit(1)
  }
  console.log(`PASS  wireit-sync (package.json wireit config == registry, ${NODES.length} nodes)`)
}
