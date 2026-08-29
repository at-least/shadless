#!/usr/bin/env node
// gates/overlay.mjs — every manual intervention on top of the mechanical
// conversion, audited against the upstream it was written for.
//
// The conversion is mechanical, but it is not complete: some content, some
// conversion rules and some whole files are written by a person or an LLM.
// On every upstream re-pin each of those must re-prove three things —
//
//   applies    the thing it attaches to still exists  (requires)
//   needed     removing it would change the output    (dissolve check)
//   effective  it actually did what it claims          (asserts / gates)
//
// — and whatever cannot prove itself is listed, with enough input for a
// person or an LLM to do it right in one pass. The old patches/ overlay was
// find/replace on generated HTML: any unrelated upstream change broke it
// and the failure mode was a silent no-apply. Nothing here is anchored to
// output text.
//
// Three kinds of unit, discovered from where they already live (no
// second copy of any rule table):
//
//   rule      a table entry in src/ (DEFAULT_CONTENT, TEXT_ADJUSTMENTS,
//             DEAD_UTILITIES, SKIN_ALLOWLIST, KNOWN_ICONS, tier sets, the
//             Persian dictionary, contract ignoreAttrs). Precondition is a
//             structural predicate on the IR / upstream tree.
//   authored  a whole hand-written file (kernel fixtures + glue, contract
//             defs, the runtime, hand-authored demos). Anchored to the
//             sha256 of the upstream inputs it was written against,
//             recorded in overlays/manifest.json. Input changed ⇒ stale.
//   source    a git patch on the upstream tree itself, under
//             overlays/upstream/*.patch, applied with `git apply --3way`
//             before conversion. Conflict ⇒ conflict bucket.
//
//   node gates/overlay.mjs --audit     the gate: exit 1 on any orphaned /
//                                      stale / conflict / unclassified
//   node gates/overlay.mjs --record    re-anchor every authored unit to the
//                                      current upstream (after review)
//   node gates/overlay.mjs --tasks     write one task packet per stale or
//                                      orphaned unit to build/gates/tasks/
//   node gates/overlay.mjs --report    audit without exiting non-zero
import { readFileSync, writeFileSync, existsSync, readdirSync, mkdirSync, rmSync } from "node:fs"
import { createHash } from "node:crypto"
import { execFileSync } from "node:child_process"
import { parseTs } from "../src/converter/index.mjs"
import { extractTranslations } from "../tools/rtl-lib.mjs"

const UP = ".upstream/shadcn-ui"
const REG = "apps/v4/registry/bases/radix/ui"
const EXAMPLES = "apps/v4/examples/radix"
const EXAMPLES_ARIA = "apps/v4/examples/aria"
const MDX = "apps/v4/content/docs/components/radix"
const MANIFEST = "overlays/manifest.json"
const PATCHES = "overlays/upstream"
const TASKS = "build/gates/tasks"

const argv = process.argv.slice(2)
const has = (f) => argv.includes(`--${f}`)
const pin = JSON.parse(readFileSync("src/registry/pin.json", "utf8"))
const sha = (buf) => createHash("sha256").update(buf).digest("hex")
const up = (rel) => `${UP}/${rel}`
const upExists = (rel) => existsSync(up(rel))
const upRead = (rel) => readFileSync(up(rel), "utf8")
const ir = (name) => existsSync(`src/registry/ir/${name}.json`)
  ? JSON.parse(readFileSync(`src/registry/ir/${name}.json`, "utf8")) : null
const registryNames = () => readdirSync(up(REG)).filter((f) => f.endsWith(".tsx")).map((f) => f.slice(0, -4))

// ================================================================= rules
// Each adapter yields { id, kind:"rule", home, requires(): string|null,
// dissolved(): string|null }. requires() returns a reason when the
// precondition FAILS; dissolved() returns a reason when the rule is no
// longer needed. Both null ⇒ applied.
async function ruleUnits() {
  const units = []
  const emitter = await import("../src/emitter/index.mjs")
  const converter = await import("../src/converter/index.mjs")
  const css = await import("../src/emitter/css.mjs")
  const skin = await import("../src/emitter/skin.mjs")
  const transforms = await import("../src/docs/transforms.mjs")

  // DEFAULT_CONTENT — (component, fn) keyed example content for static pages
  for (const [comp, fns] of Object.entries(emitter.DEFAULT_CONTENT)) {
    for (const fn of Object.keys(fns)) {
      units.push({
        id: `default-content:${comp}.${fn}`, kind: "rule", home: "src/emitter/index.mjs DEFAULT_CONTENT",
        requires: () => {
          const i = ir(comp)
          if (!i) return `component ${comp} has no IR (gone upstream?)`
          if (!i.components.some((c) => c.export && c.fn === fn)) return `${comp} no longer exports ${fn}`
          return null
        },
        dissolved: () => null,
      })
    }
  }

  // TEXT_ADJUSTMENTS — prose rewrites anchored to upstream mdx sentences
  for (const adj of transforms.TEXT_ADJUSTMENTS) {
    for (const [i, op] of adj.ops.entries()) {
      units.push({
        id: `text-adjustment:${adj.id}#${i}`, kind: "rule", home: "src/docs/transforms.mjs TEXT_ADJUSTMENTS",
        requires: () => {
          for (const f of adj.files) {
            if (!upExists(`${MDX}/${f}`)) return `${f} missing upstream`
            if (!transforms.fenceShadow(upRead(`${MDX}/${f}`)).includes(op.find)) return `find string no longer in ${f}: "${op.find.slice(0, 50)}…"`
          }
          return null
        },
        dissolved: () => null,
      })
    }
  }

  // DEAD_UTILITIES — classes the registry uses but no stylesheet defines
  const registryText = () => registryNames().map((n) => upRead(`${REG}/${n}.tsx`)).join("\n")
  // the converter also runs over the examples (build-demo / oracle bundles)
  const sourceText = () => registryText() + "\n" + [EXAMPLES, EXAMPLES_ARIA].flatMap((d) =>
    readdirSync(up(d)).filter((f) => f.endsWith(".tsx")).map((f) => upRead(`${d}/${f}`))).join("\n")
  // the ONE skin this build ingests — other skins defining a class is irrelevant
  const upstreamStyles = () => upRead(skin.SKIN_PATH.replace(`${UP}/`, ""))
  for (const tok of css.DEAD_UTILITIES) {
    units.push({
      id: `dead-utility:${tok}`, kind: "rule", home: "src/emitter/css.mjs DEAD_UTILITIES",
      requires: () => registryText().includes(tok) ? null : `${tok} no longer referenced by the registry`,
      dissolved: () => new RegExp(`@utility\\s+${tok}\\b|\\.${tok}\\s*[{,]`).test(upstreamStyles())
        ? `upstream now defines ${tok} — it is not dead any more` : null,
    })
  }
  for (const tok of skin.SKIN_ALLOWLIST) {
    units.push({
      id: `skin-allowlist:${tok}`, kind: "rule", home: "src/emitter/skin.mjs SKIN_ALLOWLIST",
      requires: () => (registryText().includes(tok) || upstreamStyles().includes(tok)) ? null : `${tok} appears nowhere upstream`,
      dissolved: () => new RegExp(`@utility\\s+${tok}\\b|\\.${tok}\\s*\\{`).test(upstreamStyles())
        ? `upstream now emits rules for ${tok} — the allowlist entry would hide them` : null,
    })
  }

  // KNOWN_ICONS — icon component names the converter treats as <svg>
  for (const icon of converter.KNOWN_ICONS) {
    units.push({
      id: `known-icon:${icon}`, kind: "rule", home: "src/converter/index.mjs KNOWN_ICONS",
      requires: () => null,
      dissolved: () => new RegExp(`\\b${icon}\\b`).test(sourceText()) ? null : `${icon} is used by no registry or example file`,
    })
  }

  // tier classification — every registry component must be classified, and
  // every classified name must exist. A NEW upstream component with no tier
  // is exactly the re-pin case that silently shipped nothing before.
  const tiers = { kernel: converter.KERNEL, trivial: converter.TRIVIAL, medium: converter.MEDIUM,
    logic: converter.LOGIC, external: converter.EXPLICIT_EXTERNAL }
  const classified = new Set()
  for (const [tier, set] of Object.entries(tiers)) {
    for (const name of set) {
      classified.add(name)
      units.push({
        id: `tier:${tier}:${name}`, kind: "rule", home: "src/converter/index.mjs tier sets",
        requires: () => upExists(`${REG}/${name}.tsx`) ? null : `${name}.tsx is gone from the registry`,
        dissolved: () => null,
      })
    }
  }
  units.push({
    id: "tier:coverage", kind: "rule", home: "src/converter/index.mjs tier sets",
    requires: () => {
      // static tier is the default (no set); the converter decides static vs
      // needs-runtime from the source. What must not happen: a component that
      // imports a radix primitive and sits in no set.
      // Slot / Direction / VisuallyHidden are structural helpers, not behavior
      const INERT = new Set(["Slot", "Direction", "VisuallyHidden", "Primitive"])
      const behaviorImport = (src) => [...src.matchAll(/import\s*\{([^}]*)\}\s*from\s*"(?:radix-ui|@radix-ui\/[\w-]+)"/g)]
        .flatMap((m) => m[1].split(",").map((x) => x.trim().split(/\s+as\s+/)[0]).filter(Boolean))
        .some((name) => !INERT.has(name))
      const unclassified = registryNames().filter((n) => !classified.has(n) && behaviorImport(upRead(`${REG}/${n}.tsx`)))
      return unclassified.length ? `radix-backed components with no tier: ${unclassified.join(", ")}` : null
    },
    dissolved: () => null,
  })

  // Persian dictionary — keys must exist in upstream's Arabic dictionary
  {
    const src = readFileSync("tools/build-rtl.mjs", "utf8")
    const keys = [...src.matchAll(/^\s+(\w+):\s*"[^"]*",?$/gm)].map((m) => m[1]).filter((k) => k !== "dir")
    units.push({
      id: "rtl:persian-dictionary", kind: "rule", home: "tools/build-rtl.mjs PERSIAN",
      requires: () => {
        const p = `${EXAMPLES_ARIA}/alert-rtl.tsx`
        if (!upExists(p)) return "alert-rtl.tsx gone from examples/aria"
        const t = extractTranslations(parseTs(upRead(p)))
        if (!t?.ar) return "alert-rtl.tsx has no Arabic dictionary"
        const missing = keys.filter((k) => !(k in t.ar.values))
        return missing.length ? `Persian keys with no Arabic counterpart upstream: ${missing.join(", ")}` : null
      },
      dissolved: () => {
        const t = extractTranslations(parseTs(upRead(`${EXAMPLES_ARIA}/alert-rtl.tsx`)))
        return t?.fa ? "upstream now ships a Persian dictionary — use it instead" : null
      },
    })
  }

  // contract ignoreAttrs — the slot they exempt must still exist
  for (const f of readdirSync("tools/contracts/components").filter((x) => x.endsWith(".mjs"))) {
    const name = f.slice(0, -4)
    const base = name.replace(/-multiple$/, "")
    const def = (await import(`../tools/contracts/components/${f}`)).default
    for (const slot of Object.keys(def.ignoreAttrs ?? {})) {
      units.push({
        id: `ignore-attrs:${name}:${slot}`, kind: "rule", home: `tools/contracts/components/${f}`,
        requires: () => {
          const i = ir(base)
          if (!i) return `${base} has no IR`
          const slots = new Set(i.components.flatMap((c) => c.elements.map((e) => e.slot)).filter(Boolean))
          return slots.has(slot) ? null : `slot ${slot} no longer emitted by ${base}`
        },
        dissolved: () => null,
      })
    }
  }
  return units
}

// ============================================================== authored
// { id, kind:"authored", file, inputs:[upstream-relative paths], extra:[...] }
// hash = sha256 over the concatenated inputs (+ extras such as the kernel
// sha) at record time. Input content changed ⇒ stale.
function authoredUnits() {
  const units = []
  const regFile = (name) => `${REG}/${name.replace(/-multiple$/, "")}.tsx`

  // src/kernel/*.html are GENERATED by tools/example-fixture.mjs --contracts
  // from the contract defs' React usage trees (the defs are the authored units)
  // per-component behavior files: written against the component's tsx (and
  // the vendored kernel, whose sha is part of the anchor)
  for (const f of readdirSync("src/runtime/components").filter((x) => x.endsWith(".js"))) {
    const name = f.slice(0, -3)
    units.push({ id: `behavior:${name}`, kind: "authored", file: `src/runtime/components/${f}`,
      inputs: [regFile(name)], extra: [`kernel:${pin.kernel.sha256}`] })
  }
  // the trivial-tier runtime is written against every trivial component
  {
    const { TRIVIAL } = JSON.parse(readFileSync("build/gates/.trivial.json", "utf8"))
    units.push({ id: "runtime:core", kind: "authored", file: "src/runtime/core.js",
      inputs: TRIVIAL.map(regFile), extra: [] })
  }
  // hand-authored demos: docs/demos pages the oracle does not own
  {
    const owned = new Set(JSON.parse(readFileSync("docs/example-oracle.json", "utf8")).map((t) => t.name))
    // pages tools/example-fixture.mjs generates (self-verified, not hand-written)
    const fixtureOwned = new Set(existsSync("docs/example-fixture-targets.json")
      ? JSON.parse(readFileSync("docs/example-fixture-targets.json", "utf8")).map((t) => t.name) : [])
    for (const f of readdirSync("docs/demos").filter((x) => x.endsWith(".html") && !/-rtl-(he|en|fa)\.html$/.test(x))) {
      const name = f.slice(0, -5)
      if (owned.has(name)) continue
      if (fixtureOwned.has(name)) continue // tools/example-fixture.mjs owns these
      const ex = `${EXAMPLES}/${name}.tsx`
      units.push({ id: `demo:${name}`, kind: "authored", file: `docs/demos/${f}`,
        inputs: upExists(ex) ? [ex] : [], extra: upExists(ex) ? [] : ["no-upstream-input"] })
    }
  }
  return units
}

const hashUnit = (u) => sha([...u.inputs.map((p) => upExists(p) ? readFileSync(up(p)) : Buffer.from("<missing>")),
  ...u.extra.map((e) => Buffer.from(e))].map((b) => b.toString("base64")).join("|"))

// ================================================================ source
function sourceUnits() {
  if (!existsSync(PATCHES)) return []
  return readdirSync(PATCHES).filter((f) => f.endsWith(".patch")).sort().map((f) => ({
    id: `source:${f}`, kind: "source", file: `${PATCHES}/${f}`,
  }))
}
// A patch series is "applied" when the upstream working tree carries exactly
// it: every patch reverse-applies cleanly. It "conflicts" when it neither
// applies forward (tree clean, patch broken) nor reverse-applies.
function sourceState(u) {
  const check = (args) => { try { execFileSync("git", ["-C", UP, "apply", "--check", ...args, `${process.cwd()}/${u.file}`], { stdio: "pipe" }); return true } catch { return false } }
  if (check(["-R"])) return { state: "applied" }
  if (check([])) return { state: "not-applied", reason: "applies cleanly but is not applied — run gates/upstream.mjs --apply-patches" }
  return { state: "conflict", reason: "neither applied nor applicable to the pinned upstream — rebase the patch" }
}

// ================================================================= audit
async function audit({ strict }) {
  // tier sets are needed by authoredUnits (runtime inputs); stash them once
  const { TRIVIAL } = await import("../src/converter/index.mjs")
  mkdirSync("build/gates", { recursive: true })
  writeFileSync("build/gates/.trivial.json", JSON.stringify({ TRIVIAL: [...TRIVIAL] }))

  const manifest = existsSync(MANIFEST) ? JSON.parse(readFileSync(MANIFEST, "utf8")) : { pin: null, units: {} }
  const buckets = { applied: [], dissolved: [], orphaned: [], stale: [], conflict: [], unrecorded: [] }

  for (const u of await ruleUnits()) {
    const r = u.requires()
    if (r) { buckets.orphaned.push({ ...u, reason: r }); continue }
    const d = u.dissolved()
    if (d) { buckets.dissolved.push({ ...u, reason: d }); continue }
    buckets.applied.push(u)
  }
  for (const u of authoredUnits()) {
    const rec = manifest.units[u.id]
    const h = hashUnit(u)
    if (!rec) { buckets.unrecorded.push({ ...u, hash: h }); continue }
    if (rec.hash !== h) buckets.stale.push({ ...u, reason: `upstream inputs changed since ${rec.pin}: ${u.inputs.join(", ") || u.extra.join(", ")}`, recorded: rec })
    else buckets.applied.push(u)
  }
  for (const u of sourceUnits()) {
    const s = sourceState(u)
    if (s.state === "applied") buckets.applied.push(u)
    else buckets.conflict.push({ ...u, reason: s.reason })
  }
  // recorded units whose file is gone
  for (const id of Object.keys(manifest.units)) {
    if (!buckets.applied.some((u) => u.id === id) && !buckets.stale.some((u) => u.id === id))
      buckets.orphaned.push({ id, kind: "authored", reason: "recorded in the manifest but the unit no longer exists — delete the entry (--record)" })
  }

  const bad = ["orphaned", "stale", "conflict", "unrecorded"].flatMap((b) => buckets[b].map((u) => ({ bucket: b, ...u })))
  const line = (u) => `${u.bucket.padEnd(10)} ${u.id}\n             ${u.reason ?? "(new authored unit — record it: node gates/overlay.mjs --record)"}${u.file ? `\n             file: ${u.file}` : ""}`
  const counts = Object.entries(buckets).map(([k, v]) => `${v.length} ${k}`).join(", ")
  if (buckets.dissolved.length) {
    console.log(`overlay: ${buckets.dissolved.length} rules can be DELETED (upstream no longer needs them):`)
    for (const u of buckets.dissolved) console.log(`  ${u.id}  —  ${u.reason}\n    at ${u.home}`)
  }
  if (bad.length) {
    console.error(`${strict ? "FAIL " : "REPORT"} overlay (${counts})\n  ` + bad.map(line).join("\n  "))
    console.error(`\n  task packets: node gates/overlay.mjs --tasks`)
    if (strict) process.exit(1)
    return buckets
  }
  console.log(`PASS  overlay (${counts}; every manual intervention still applies to ${pin.shadcn_ui.tag})`)
  return buckets
}

// ================================================================ record
async function record() {
  const manifest = existsSync(MANIFEST) ? JSON.parse(readFileSync(MANIFEST, "utf8")) : { units: {} }
  const { TRIVIAL } = await import("../src/converter/index.mjs")
  mkdirSync("build/gates", { recursive: true })
  writeFileSync("build/gates/.trivial.json", JSON.stringify({ TRIVIAL: [...TRIVIAL] }))
  const next = { pin: pin.shadcn_ui.tag, commit: pin.shadcn_ui.commit, units: {} }
  let changed = 0
  for (const u of authoredUnits()) {
    const h = hashUnit(u)
    if (manifest.units[u.id]?.hash !== h) changed++
    next.units[u.id] = { file: u.file, inputs: u.inputs, extra: u.extra.filter((e) => !e.startsWith("kernel:")), hash: h, pin: pin.shadcn_ui.tag }
  }
  writeFileSync(MANIFEST, JSON.stringify(next, null, 2) + "\n")
  console.log(`overlay recorded: ${Object.keys(next.units).length} authored units anchored to ${pin.shadcn_ui.tag} (${changed} re-anchored)`)
}

// ================================================================= tasks
// One markdown packet per unit that needs a human or an LLM: what changed
// upstream (the real diff between the recorded commit and the pin), the
// current file, and the gates that must be green afterwards.
async function tasks() {
  const buckets = await audit({ strict: false })
  rmSync(TASKS, { recursive: true, force: true }); mkdirSync(TASKS, { recursive: true })
  const manifest = existsSync(MANIFEST) ? JSON.parse(readFileSync(MANIFEST, "utf8")) : { commit: null }
  const gateFor = (u) => u.id.startsWith("glue:") || u.id.startsWith("kernel-fixture:") ? "contracts, style-parity, interactivity-sweep"
    : u.id.startsWith("contract:") ? "contracts, style-parity" : u.id.startsWith("demo:") ? "example-gate, docs-smoke, interactivity-sweep"
    : u.id.startsWith("runtime:") ? "contracts, demo-smoke" : "the full tier"
  let n = 0
  for (const bucket of ["stale", "orphaned", "conflict"]) for (const u of buckets[bucket]) {
    let diff = ""
    if (u.inputs?.length && manifest.commit) {
      for (const p of u.inputs) {
        try { diff += execFileSync("git", ["-C", UP, "diff", `${manifest.commit}..${pin.shadcn_ui.commit}`, "--", p], { encoding: "utf8" }) }
        catch (e) { diff += `(could not diff ${p}: ${e.message.split("\n")[0]})\n` }
      }
    }
    const md = [`# ${u.id}`, "", `**bucket**: ${bucket}`, `**reason**: ${u.reason}`, u.file ? `**file**: \`${u.file}\`` : "", u.home ? `**home**: ${u.home}` : "", "",
      "## What to do", "", bucket === "stale"
        ? `The upstream inputs this file was written against changed. Read the diff below, update \`${u.file}\` so it reflects the new upstream, then run the gates listed and \`node gates/overlay.mjs --record\`.`
        : bucket === "orphaned" ? `The thing this rule attaches to no longer exists upstream. Either delete the rule at its home, or re-anchor it. Then run the full tier.`
        : `Rebase the patch onto the pinned upstream (\`git -C ${UP} apply --3way\`), resolve, regenerate with \`git -C ${UP} diff > ${u.file}\`.`,
      "", `**gates to satisfy**: ${gateFor(u)}  —  \`node gates/run.mjs --only=<gate>\``, "",
      diff ? `## Upstream diff (${manifest.commit?.slice(0, 10)} → ${pin.shadcn_ui.commit.slice(0, 10)})\n\n\`\`\`diff\n${diff}\`\`\`` : "",
      u.file && existsSync(u.file) ? `## Current file\n\n\`\`\`\n${readFileSync(u.file, "utf8")}\n\`\`\`` : "",
    ].filter((s) => s !== undefined).join("\n")
    writeFileSync(`${TASKS}/${u.id.replace(/[^\w.-]+/g, "_")}.md`, md)
    n++
  }
  console.log(`overlay tasks: ${n} packets under ${TASKS}/`)
}

if (has("record")) await record()
else if (has("tasks")) await tasks()
else if (has("report")) await audit({ strict: false })
else await audit({ strict: true })
