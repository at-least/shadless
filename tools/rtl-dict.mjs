#!/usr/bin/env node
// rtl-dict — lift the RTL translation dictionaries out of upstream and into a
// file this repo owns.
//
// The dictionaries live in `examples/aria/<name>-rtl.tsx`, which is upstream's
// React Aria registry. This repo targets the RADIX registry only (see
// src/registry/pin.json), so aria is a tree we deliberately do not build from
// — and one node reaching into it for string data was the only thing keeping
// that dependency alive. That data is not aria-specific in any way: it is
// `{ en, ar, he } → { dir, values }`, the same strings whichever primitive
// library renders them.
//
// So this step reads aria and NOTHING ELSE DOES. build-rtl.mjs consumes the
// JSON. The dependency is one declared edge in the graph instead of a read
// buried in a page emitter.
//
// It is a snapshot, but not a hand-maintained one: this is a graph node whose
// inputs are those .tsx files, so an upstream move re-runs it, and
// `reproducible` fails if the committed JSON is not what a fresh run produces.
//
// All-or-nothing, for the reason ddc03fb gave when example-oracle had the same
// shape of bug: a partial write here shrinks the dictionary silently, and
// every page that would have been translated just quietly stops existing.

import { readFileSync, writeFileSync, readdirSync } from "node:fs"
import { extractTranslations, parseTs } from "./rtl-lib.mjs"

const EXAMPLES = ".upstream/shadcn-ui/apps/v4/examples/aria"
const OUT = "src/registry/rtl-translations.json"

// Which absences are legitimate is a property of the component, and
// src/registry/tiers.json already records it — no second list. Upstream ships
// -rtl examples for things we do not build (calendar, drawer, sidebar) and for
// things that are not our components at all (data-table, shimmer, typography),
// and some of those carry no `translations` because they have no prose to
// translate. A missing dictionary is a SKIP for those and a FAILURE for
// anything we ship.
const TIERS = JSON.parse(readFileSync("src/registry/tiers.json", "utf8"))
const WHOLESALE = new Set(["static", "kernel", "trivial-js"])
const shipped = (name) => {
  const t = TIERS[name.replace(/-rtl$/, "")]
  return !!t && (WHOLESALE.has(t.tier) || t.emit === true)
}

const files = readdirSync(EXAMPLES).filter((f) => f.endsWith("-rtl.tsx")).sort()
if (files.length === 0) {
  console.error(`FAIL  rtl-dict: no *-rtl.tsx under ${EXAMPLES} — the pinned upstream moved or the checkout is incomplete`)
  process.exit(1)
}

const dict = {}
const failures = []
const skipped = []
// the Arabic entry is load-bearing: build-rtl substitutes OUT of `ar` into
// every other language, so a dictionary without it can produce nothing
const problem = (t) => !t ? "no `translations` object literal"
  : !t.ar ? "no Arabic dictionary" : null
for (const file of files) {
  const name = file.replace(".tsx", "")
  let translations
  try {
    translations = extractTranslations(parseTs(readFileSync(`${EXAMPLES}/${file}`, "utf8")))
  } catch (e) {
    failures.push(`${name}: ${e.message.split("\n")[0]}`)
    continue
  }
  const bad = problem(translations)
  if (bad) {
    if (shipped(name)) failures.push(`${name}: ${bad} in ${file}`)
    else skipped.push(name)
    continue
  }
  dict[name] = translations
}

if (failures.length) {
  for (const f of failures) console.error(`FAIL [${f.split(":")[0]}]: ${f.slice(f.indexOf(":") + 2)}`)
  console.error(`FAIL  rtl-dict (${failures.length}/${files.length} dictionaries unreadable) — nothing written; ${OUT} keeps its previous contents`)
  process.exit(1)
}

// insertion order is the sorted file list, so the bytes are stable
writeFileSync(OUT, JSON.stringify(dict, null, 1) + "\n")
console.log(`rtl-dict: ${Object.keys(dict).length} dictionaries lifted from ${EXAMPLES} to ${OUT}` +
  (skipped.length ? ` (${skipped.length} upstream -rtl examples carry no dictionary and are for components we do not ship: ${skipped.join(", ")})` : ""))
