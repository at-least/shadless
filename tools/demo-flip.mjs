#!/usr/bin/env node
// demo-flip.mjs — FT7 batch integration: flip catalog preview entries
// to-author → authored. Refuses to flip unless the demo file exists
// (docs/demos/<name>.html) and the current status is to-author.
// Tombstoned entries need --allow-tombstone (flipping a deliberate
// "won't build" decision back on should be explicit, not incidental).
// Usage: node tools/demo-flip.mjs [--allow-tombstone] <name> [name...]
import { readFileSync, writeFileSync, existsSync } from "node:fs"

const file = "docs/catalog.json"
const argv = process.argv.slice(2)
const allowTombstone = argv.includes("--allow-tombstone")
const names = argv.filter((a) => !a.startsWith("--"))
if (!names.length) {
  console.error("usage: node tools/demo-flip.mjs [--allow-tombstone] <name> [name...]")
  process.exit(1)
}
const catalog = JSON.parse(readFileSync(file, "utf8"))
let flipped = 0
for (const name of names) {
  const p = catalog.previews.find((x) => x.name === name)
  if (!p) { console.error(`FLIP FAIL: ${name} not in catalog`); process.exit(1) }
  const tombstone = p.status === "tombstoned"
  if (p.status !== "to-author" && !(tombstone && allowTombstone)) {
    const hint = tombstone && !allowTombstone ? " — pass --allow-tombstone to flip a tombstone" : ""
    console.error(`FLIP FAIL: ${name} status=${p.status} (expected to-author${allowTombstone ? " or tombstoned" : ""})${hint}`)
    process.exit(1)
  }
  if (!existsSync(`docs/demos/${name}.html`)) { console.error(`FLIP FAIL: docs/demos/${name}.html missing`); process.exit(1) }
  p.status = "authored"
  flipped++
}
writeFileSync(file, JSON.stringify(catalog, null, 2) + "\n")
const counts = catalog.previews.reduce((a, p) => { a[p.status] = (a[p.status] || 0) + 1; return a }, {})
console.log(`flipped: ${flipped} | catalog: ${JSON.stringify(counts)}`)
