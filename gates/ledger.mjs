#!/usr/bin/env node
// gates/ledger.mjs — the recorded-difference ledger, as data.
//
// Replaces tools/exemption-ledger.mjs + the prose EXEMPTIONS.md as the
// MACHINE surface. EXEMPTIONS.md is now GENERATED from gates/ledger.json
// (`--render`), so the human-readable file cannot drift from the checked one.
//
// Why this shape:
//
//   class      every exemption declares how it ENDS, not just why it exists.
//              "permanent"     a real engine/by-design difference
//              "auto-dissolve" expected to vanish at the next re-pin — the
//                              upstream pipeline DELETES these wholesale and
//                              lets whatever still fails come back with
//                              evidence. No human "walks the Automation
//                              column" any more; that was the review step
//                              most likely to be skipped.
//              "debt"          tracked work; carries a budget
//
//   budgets    the ratchet, done right. tools/style-parity.mjs recorded a
//              COUNT per component but compared only PRESENCE (:138-146), so
//              23 of 29 components could accumulate unlimited new drift while
//              the gate stayed green. Here a budget fails when the number
//              GROWS *and* when it SHRINKS without being re-recorded — slack
//              cannot silently accumulate.
//
//   recorded_at_pin  which upstream release the difference was observed
//              against. An auto-dissolve entry older than the current pin is
//              stale by definition and is reported.
import { readFileSync, writeFileSync, readdirSync, existsSync } from "node:fs"

const LEDGER = "gates/ledger.json"
const RENDERED = "EXEMPTIONS.md"
const CLASSES = new Set(["permanent", "auto-dissolve", "debt"])

const argv = process.argv.slice(2)
const has = (f) => argv.includes(`--${f}`)

// ---------------------------------------------------------------- sources
// Every id the codebase currently claims as an exemption. Same derivation as
// the tool this replaces — the ids are the contract with the sources.
async function collectSourceIds() {
  const ids = new Map() // id -> source name
  for (const f of readdirSync("tools/contracts/components").filter((x) => x.endsWith(".mjs"))) {
    const name = f.replace(".mjs", "")
    const def = (await import(`../tools/contracts/components/${f}`)).default
    if (def.mountedCheck === false) ids.set(`mounted-check:${name}`, "contracts")
    if (def.mountedClasses === false) ids.set(`mounted-classes:${name}`, "contracts")
    for (const [slot, attrs] of Object.entries(def.ignoreAttrs ?? {}))
      for (const a of attrs) ids.set(`ignore-attrs:${name}:${slot}:${a}`, "contracts")
  }
  const golden = JSON.parse(readFileSync("src/registry/upstream-snapshot/exemptions.json", "utf8"))
  for (const rec of Object.values(golden.examples)) ids.set(`golden:${rec.reason}`, "golden")
  const css = (await import("../src/emitter/css.mjs")).DEAD_UTILITIES
  for (const t of css) ids.set(`dead-utility:${t}`, "emitter")
  const skin = (await import("../src/emitter/skin.mjs")).SKIN_ALLOWLIST
  for (const t of skin) ids.set(`skin-allowlist:${t}`, "emitter")
  return ids
}

// Live values for every budgeted number, read from the same place the gate
// that reports it reads. Keep this the ONLY definition of each metric.
async function collectBudgetValues() {
  const v = {}
  const golden = JSON.parse(readFileSync("src/registry/upstream-snapshot/exemptions.json", "utf8"))
  v["golden.exempt-demos"] = Object.keys(golden.examples).length
  const sweep = readFileSync("tools/interactivity-sweep.mjs", "utf8")
  const block = sweep.match(/const KNOWN_DEAD = new Set\(\[([\s\S]*?)\]\)/)
  v["interactivity.dead-families"] = block
    ? block[1].split(",").map((s) => s.trim()).filter((s) => /^["']/.test(s)).length : -1
  const dp = "gates/demo-parity-baseline.json"
  v["demo-parity.dirty-cells"] = existsSync(dp) ? JSON.parse(readFileSync(dp, "utf8")).cells.length : -1
  const pp = "gates/path-parity-baseline.json"
  v["path-parity.dirty-cells"] = existsSync(pp) ? JSON.parse(readFileSync(pp, "utf8")).cells.length : -1
  const bl = "gates/style-parity-baseline.json"
  v["style-parity.dirty-cells"] = existsSync(bl)
    ? JSON.parse(readFileSync(bl, "utf8")).cells.length : -1
  // coverage.* budgets are enforced by `go test -run TestCoverage` (the count
  // needs the IR, and a stale build/gates/coverage.json from a mutation run
  // must never feed the ledger); the ledger only validates the entry's shape
  return v
}

const load = () => JSON.parse(readFileSync(LEDGER, "utf8"))
const save = (l) => writeFileSync(LEDGER, JSON.stringify(l, null, 2) + "\n")
const currentPin = () => JSON.parse(readFileSync("src/registry/pin.json", "utf8")).shadcn_ui.tag

// Golden reasons carry their own end-condition in the text; trust it so a
// re-pin dissolves them without anyone re-classifying 147 rows by hand.
const classOfGoldenReason = (r) =>
  /re-check on re-pin|deploy lag|frame lag/i.test(r) ? "auto-dissolve" : "permanent"

// ---------------------------------------------------------------- verify
async function verify() {
  const ledger = load()
  const sourceIds = await collectSourceIds()
  const problems = []

  for (const [id, e] of Object.entries(ledger.entries)) {
    if (!CLASSES.has(e.class)) problems.push(`${id}: unknown class "${e.class}"`)
    if (!e.reason || e.reason.length < 8) problems.push(`${id}: missing or trivial reason`)
    if (!e.recorded_at_pin) problems.push(`${id}: no recorded_at_pin`)
  }
  const undocumented = [...sourceIds.keys()].filter((id) => !ledger.entries[id]).sort()
  const stale = Object.keys(ledger.entries).filter((id) => !sourceIds.has(id)).sort()
  if (undocumented.length)
    problems.push(`exemptions in the sources with no ledger entry (add them, with a class):\n    ` +
      undocumented.join("\n    "))
  if (stale.length)
    problems.push(`ledger entries whose source flag is gone (delete them):\n    ` + stale.join("\n    "))

  const values = await collectBudgetValues()
  for (const [name, b] of Object.entries(ledger.budgets)) {
    const actual = values[name]
    if (actual === undefined && name.startsWith("coverage.")) continue // checked by `go test -run TestCoverage`
    if (actual === undefined) { problems.push(`budget ${name}: no live value known`); continue }
    if (actual < 0) { problems.push(`budget ${name}: could not read the live value`); continue }
    if (actual > b.max)
      problems.push(`budget ${name}: ${actual} > recorded max ${b.max} — this number may only shrink`)
    else if (actual < b.max)
      problems.push(`budget ${name}: ${actual} < recorded max ${b.max} — it improved; ` +
        `re-record so the slack cannot be silently re-spent:  node gates/ledger.mjs --record`)
  }

  const pin = currentPin()
  const staleAuto = Object.entries(ledger.entries)
    .filter(([, e]) => e.class === "auto-dissolve" && e.recorded_at_pin !== pin)
    .map(([id]) => id)

  if (problems.length) {
    console.error(`FAIL  ledger\n  ` + problems.join("\n  "))
    process.exit(1)
  }
  const byClass = {}
  for (const e of Object.values(ledger.entries)) byClass[e.class] = (byClass[e.class] ?? 0) + 1
  console.log(`PASS  ledger (${Object.keys(ledger.entries).length} exemptions: ` +
    Object.entries(byClass).map(([k, v]) => `${v} ${k}`).join(", ") +
    `; ${Object.keys(ledger.budgets).length} budgets at their recorded max)` +
    (staleAuto.length ? `\n  note: ${staleAuto.length} auto-dissolve entries predate pin ${pin} ` +
      `— they should have dissolved at the last re-pin` : ""))
}

// ---------------------------------------------------------------- record
// Reconcile the ledger with the sources: add missing ids, drop stale ones,
// re-record budgets. Reasons for new ids are harvested from the id itself
// where it carries one (golden), otherwise flagged TODO so the human writes
// the reason rather than the tool inventing it.
async function record() {
  const ledger = existsSync(LEDGER) ? load() : { pin: currentPin(), entries: {}, budgets: {}, notes: [] }
  const pin = currentPin()
  ledger.pin = pin
  const sourceIds = await collectSourceIds()
  let added = 0, dropped = 0
  for (const [id, source] of sourceIds) {
    if (ledger.entries[id]) continue
    const reason = id.startsWith("golden:") ? id.slice(7) : "TODO: state why this difference is accepted"
    ledger.entries[id] = {
      class: id.startsWith("golden:") ? classOfGoldenReason(reason) : "permanent",
      reason, source, recorded_at_pin: pin,
    }
    added++
  }
  for (const id of Object.keys(ledger.entries)) {
    if (!sourceIds.has(id)) { delete ledger.entries[id]; dropped++ }
  }
  const values = await collectBudgetValues()
  for (const [name, b] of Object.entries(ledger.budgets)) {
    if (values[name] >= 0 && values[name] !== b.max) {
      console.log(`  budget ${name}: ${b.max} -> ${values[name]}`)
      b.max = values[name]
    }
  }
  save(ledger)
  const todo = Object.entries(ledger.entries).filter(([, e]) => e.reason.startsWith("TODO")).length
  console.log(`ledger recorded: +${added} -${dropped}, ${Object.keys(ledger.entries).length} entries` +
    (todo ? `\n  ${todo} entries still need a real reason (search for TODO in ${LEDGER})` : ""))
}

// -------------------------------------------------------------- dissolve
// Called by the upstream drill right after a re-pin: delete every
// auto-dissolve entry recorded against a DIFFERENT pin, so the rebuild has
// to re-earn each one. Whatever the gates still flag comes back as a real,
// evidenced failure instead of hiding under an exemption recorded against
// an older upstream. Entries recorded at the current pin stay — a same-tag
// drill (the self-test) must be a no-op here.
//
// The golden exemptions are ALSO a source file (src/registry/upstream-snapshot/
// exemptions.json, one row per demo): the demos whose reason dissolved are
// pruned there too, otherwise the ledger and the source disagree and the
// verify step fails for the wrong reason.
function dissolve() {
  const ledger = load()
  const pin = currentPin()
  const gone = Object.entries(ledger.entries)
    .filter(([, e]) => e.class === "auto-dissolve" && e.recorded_at_pin !== pin).map(([id]) => id)
  for (const id of gone) delete ledger.entries[id]
  ledger.pin = pin
  save(ledger)
  const goldenPath = "src/registry/upstream-snapshot/exemptions.json"
  const golden = JSON.parse(readFileSync(goldenPath, "utf8"))
  const goneReasons = new Set(gone.filter((id) => id.startsWith("golden:")).map((id) => id.slice(7)))
  let pruned = 0
  for (const [demo, rec] of Object.entries(golden.examples))
    if (goneReasons.has(rec.reason)) { delete golden.examples[demo]; pruned++ }
  writeFileSync(goldenPath, JSON.stringify(golden, null, 1) + "\n")
  // pruning is a deliberate shrink of the golden budget — record it here,
  // otherwise the drill's own verify step reports the improvement as an
  // UNEXPECTED failure (seen on the first cross-tag drill: 147 -> 90)
  if (ledger.budgets["golden.exempt-demos"]) { ledger.budgets["golden.exempt-demos"].max = Object.keys(golden.examples).length; save(ledger) }
  console.log(`ledger dissolved: removed ${gone.length} auto-dissolve entries recorded before ${pin} ` +
    `(${pruned} golden demo exemptions pruned); re-run the gates and re-record whatever legitimately survives`)
  for (const id of gone.slice(0, 10)) console.log(`  - ${id}`)
  if (gone.length > 10) console.log(`  … +${gone.length - 10} more`)
}

// ---------------------------------------------------------------- render
function render() {
  const ledger = load()
  const groups = { permanent: [], "auto-dissolve": [], debt: [] }
  for (const [id, e] of Object.entries(ledger.entries)) groups[e.class].push([id, e])
  const section = (title, key, blurb) => {
    const rows = groups[key].sort(([a], [b]) => a.localeCompare(b))
    return [`## ${title} (${rows.length})`, "", blurb, "",
      "| Id | Reason | Recorded at |", "|---|---|---|",
      ...rows.map(([id, e]) => `| \`${id}\` | ${e.reason.replace(/\|/g, "\\|")} | ${e.recorded_at_pin} |`),
      ""].join("\n")
  }
  const md = [
    "# EXEMPTIONS — the recorded-difference ledger",
    "",
    "<!-- GENERATED from gates/ledger.json by `node gates/ledger.mjs --render`.",
    "     Do not edit by hand: `gates/ledger.mjs --verify` checks the JSON, not",
    "     this file, and the next render will overwrite whatever you wrote. -->",
    "",
    `Pin: \`${ledger.pin}\` · ${Object.keys(ledger.entries).length} exemptions · ` +
      `${Object.keys(ledger.budgets).length} budgets`,
    "",
    "Every \"known difference, accepted for a reason\" lives here, and every entry",
    "declares **how it ends**. `gates/ledger.mjs --verify` (the `ledger` gate)",
    "keeps this list in lockstep with the sources in both directions: a new",
    "exemption with no entry fails, an entry whose source flag vanished fails.",
    "",
    section("Permanent", "permanent",
      "Real engine or by-design differences. These do not dissolve; upstream would have to change."),
    section("Auto-dissolve on re-pin", "auto-dissolve",
      "Deploy lag, SSR-frame lag and other pin-relative differences. `make upstream` " +
      "DELETES every one of these after a re-pin and lets the gates re-earn them — " +
      "nobody reviews this section by hand."),
    section("Debt", "debt",
      "Accepted for now, tracked to zero. Governed by the budgets below."),
    "## Budgets",
    "",
    "A budget may only shrink. Growing fails the `ledger` gate; shrinking without",
    "re-recording also fails, so slack cannot be silently re-spent.",
    "",
    "| Metric | Max | Target | Reason |", "|---|---|---|---|",
    ...Object.entries(ledger.budgets).map(([n, b]) =>
      `| \`${n}\` | ${b.max} | ${b.target ?? 0} | ${b.reason.replace(/\|/g, "\\|")} |`),
    "",
    "## Work items",
    "",
    "Not cross-checked — these track work, not accepted differences.",
    "",
    ...ledger.notes.map((n) => `- [ ] ${n}`),
    "",
  ].join("\n")
  writeFileSync(RENDERED, md)
  console.log(`rendered ${RENDERED} from ${LEDGER}`)
}

if (has("record")) await record()
else if (has("dissolve")) dissolve()
else if (has("render")) render()
else await verify()
