#!/usr/bin/env node
// example-golden — hop 1 of the 1:1 gate.
//
//   hop 1 (THIS): local oracle render  == upstream live-site snapshot
//   hop 2 (existing example-oracle --check): shipped iframe page == oracle
//
// Both green ⇒ every docs example is 1:1 with ui.shadcn.com, by machine.
//
// Comparison is DOM-STRUCTURAL (canonOf in tools/oracle-lib.mjs — the only
// copy): both sides are parsed in a real browser and canonicalized to
// (tag, sorted attr map, text) trees. The upstream snapshot comes from
// tools/upstream-snapshot.mjs (SSR payload slice, committed artifact;
// refresh on re-pin).
//
// Exemptions (probes/out/upstream-payload/exemptions.json) are explicit,
// each with a reason, and the gate fails if an exemption no longer applies
// (stale exemption = drift in the exemption list itself).
//
// Debug modes (replaces the retired dev-golden-* scripts — same comparator,
// no drift):
//   --classify        bucket every failure by first-divergence signature
//   --diff <name> <page>   deep-dive one demo (page = snapshot file stem)
//
// Kernel -demo previews: upstream's SSR initial state is the closed
// trigger; our shipped page is the runtime fixture (template + glue).
// The oracle static render CAN still be compared (closed state), so they
// stay IN scope. Only demos whose oracle build fails need exemption.
import { chromium } from "playwright"
import { readFileSync, writeFileSync, existsSync, readdirSync, mkdirSync } from "node:fs"
import { resolve, join } from "node:path"
import { buildOracle, awaitOracle, norm, canonOf } from "./oracle-lib.mjs"

const EXAMPLES = ".upstream/shadcn-ui/apps/v4/examples/radix"
const SNAPSHOT_DIR = "probes/out/upstream-payload"
const TMP = "probes/out/example-golden"
const EXEMPTIONS = `${SNAPSHOT_DIR}/exemptions.json`

const argv = process.argv.slice(2)
const MODE = argv.includes("--classify") ? "classify"
  : argv.includes("--diff") ? "diff"
  : "gate"

const loadExemptions = () => existsSync(EXEMPTIONS)
  ? JSON.parse(readFileSync(EXEMPTIONS, "utf8"))
  : { examples: {} }

async function rootHtml(page) {
  return norm(await page.evaluate(() => document.querySelector("#root").innerHTML))
}

// first-divergence signature (classify mode): bucket failures by the shape
// of the difference so fix-vs-exempt decisions are made per bucket, not
// per demo
const sig = (a, b) => {
  let i = 0
  while (i < Math.min(a.length, b.length) && a[i] === b[i]) i++
  const ctx = a.slice(Math.max(0, i - 60), i + 60) + " ||| " + b.slice(Math.max(0, i - 60), i + 60)
  return ctx.replace(/radix-<auto>/g, "#").slice(0, 200)
}

const browser = await chromium.launch()
const page = await browser.newPage()
// keep avatar-style examples in their INITIAL render state (image
// pending → fallback shown), matching what upstream's SSR snapshot
// captured; a loaded image flips radix Avatar to the img branch and the
// trees diverge on structure, not styling
await page.route(/^(https?:)?\/\//, (route) => route.abort())
await page.goto("about:blank")

if (MODE === "diff") {
  const name = argv[argv.indexOf("--diff") + 1]
  const snapFile = argv[argv.indexOf("--diff") + 2] ?? name.split("-")[0]
  const snap = JSON.parse(readFileSync(join(SNAPSHOT_DIR, `${snapFile}.json`), "utf8"))
  const upstreamHtml = snap.previews[name]
  if (!upstreamHtml) console.error(`no snapshot preview "${name}" in ${snapFile}.json`)
  else {
    const { htmlFile } = await buildOracle(name, { tmp: TMP })
    await awaitOracle(page, htmlFile)
    const a = JSON.stringify(await canonOf(page, await rootHtml(page)))
    const b = JSON.stringify(await canonOf(page, upstreamHtml))
    if (a === b) console.log("EQUAL")
    else {
      let i = 0
      while (i < Math.min(a.length, b.length) && a[i] === b[i]) i++
      console.log("ORACLE  :", a.slice(Math.max(0, i - 80), i + 120))
      console.log("UPSTREAM:", b.slice(Math.max(0, i - 80), i + 120))
      process.exitCode = 1
    }
  }
} else {
  // render one demo's oracle side (throws if the build/render failed)
  async function oracleCanon(name) {
    const { htmlFile } = await buildOracle(name, { tmp: TMP })
    await awaitOracle(page, htmlFile)
    return JSON.stringify(await canonOf(page, await rootHtml(page)))
  }

  const exemptions = loadExemptions()
  let pass = 0, fail = 0, exempt = 0
  const staleExemptions = []
  const buckets = new Map()
  const failures = []
  const bucketOf = (key, name) => {
    const k = key.replace(/"[^"]{20,}"/g, '"…"').slice(0, 110)
    if (!buckets.has(k)) buckets.set(k, [])
    buckets.get(k).push(name)
  }
  const pages = readdirSync(SNAPSHOT_DIR).filter((f) => f.endsWith(".json") && f !== "exemptions.json").sort()
  for (const pf of pages) {
    const snap = JSON.parse(readFileSync(join(SNAPSHOT_DIR, pf), "utf8"))
    for (const [name, upstreamHtml] of Object.entries(snap.previews)) {
      const ex = exemptions.examples[name]
      if (ex && !ex.stale) { exempt++; continue }
      const tsx = `${EXAMPLES}/${name}.tsx`
      if (!existsSync(tsx)) {
        // no upstream example file: the snapshot has a demo we can't render
        // (guide-own or generated). Exempt explicitly or fail.
        if (ex) { exempt++; continue }
        console.error(`FAIL [${name}]: snapshot demo has no example tsx and no exemption`)
        fail++
        continue
      }
      try {
        const sa = await oracleCanon(name)
        const sb = JSON.stringify(await canonOf(page, upstreamHtml))
        if (sa === sb) pass++
        else {
          if (MODE === "classify") { bucketOf(sig(sa, sb), name); failures.push({ name, page: pf.replace(/\.json$/, ""), kind: "diff", signature: sig(sa, sb) }) }
          else console.error(`FAIL [${name}]: oracle != upstream snapshot`)
          fail++
        }
      } catch (e) {
        if (ex) { exempt++; continue } // expected failure (exemption)
        if (MODE === "classify") {
          bucketOf(`RENDER-FAIL ${e.message.split("\n")[0]}`, name)
          failures.push({ name, page: pf.replace(/\.json$/, ""), kind: "render", error: e.message.split("\n")[0] })
        }
        else console.error(`FAIL [${name}]: oracle build/render failed — ${e.message.split("\n")[0]} (add an exemption with a reason if unfixable)`)
        fail++
        continue
      }
      if (ex) staleExemptions.push(name) // exemption exists but demo rendered fine
    }
  }
  if (staleExemptions.length) {
    console.error(`FAIL  example-golden: stale exemptions (rendered fine, remove them): ${staleExemptions.join(", ")}`)
    process.exitCode = 1
  }
  if (MODE === "classify") {
    // machine-readable failure list — input for exemptions.json triage
    writeFileSync(join(TMP, "failures.json"), JSON.stringify(failures, null, 1) + "\n")
    console.log(`classify: ${pass} pass, ${fail} fail, ${exempt} exempt (${failures.length} recorded to ${TMP}/failures.json)`)
    for (const [k, names] of [...buckets.entries()].sort((x, y) => y[1].length - x[1].length))
      console.log(`\n${String(names.length).padStart(4)}×  ${k}\n     ${names.join(", ")}`)
    if (fail) process.exitCode = 1
  }
  else if (fail) { console.error(`FAIL  example-golden (${fail} failed, ${pass} passed, ${exempt} exempt)`); process.exitCode = 1 }
  else console.log(`PASS  example-golden (${pass} == upstream snapshot, ${exempt} exempt)`)
}
// exitCode (not process.exit) — piped stdout must flush before exit
await browser.close()
