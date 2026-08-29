#!/usr/bin/env node
// five-components.mjs — FT6: disposition report for the 5 unaccounted
// components (data-table / date-picker / questionnaire / toast / typography).
// For each: locate its source in the pinned upstream — the registry the
// pipeline consumes (apps/v4/registry/bases/radix/ui) first, then any other
// registry dir (bases product lines), blocks, examples, and the radix docs
// page's ComponentSource src= targets (app example code). When a single-file
// ui source exists, run the tier classification adapted from
// probes/h5-tiers.mjs (AST hook scan + external-dep check; the external list
// is extended with the bases product-line React runtimes, which the probe's
// new-york-v4-calibrated list does not know).
//
// Disposition bar (PLAN design decisions): implement = single-file registry
// primitive in the pipeline registry, mechanically convertible (static /
// kernel / trivial-js tier). grey = everything else (foreign runtime deps
// without vanilla ports, React-heavy compositions, app-level examples,
// docs-only components). Precedent: the 12 Wave D/E tombstones.
//
// Deterministic: reads the pin only, no network. Exit 0. Report only —
// dispositions land in the PLAN ledger via the orchestrator.
import { parse } from "@babel/parser"
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs"
import { join } from "node:path"

const pin = JSON.parse(readFileSync("src/registry/pin.json", "utf8"))
const PIN = ".upstream/shadcn-ui"
const APP = `${PIN}/apps/v4`
const UI = `${APP}/registry/bases/radix/ui` // the registry the converter consumes
const TOMBSTONES = new Set(["menubar", "navigation-menu", "combobox", "sidebar",
  "calendar", "chart", "command", "drawer", "form", "input-otp", "resizable", "sonner"])

const FIVE = ["data-table", "date-picker", "questionnaire", "toast", "typography"]

// Foreign runtime deps without a vanilla port: probes/h5-tiers.mjs EXTERNAL
// list + the bases product-line React runtimes and composition deps.
const EXTERNAL = new Set([
  "cmdk", "sonner", "vaul", "embla-carousel-react", "react-day-picker",
  "recharts", "input-otp", "react-hook-form", "next-themes",
  "react-resizable-panels", "react-photo-view", "expo-image",
  "@shadcn/react", "@base-ui/react", "@tanstack/react-table",
  "@dnd-kit/core", "@dnd-kit/modifiers", "@dnd-kit/sortable", "@dnd-kit/utilities",
])
// radix primitive family -> behavior source (same sets as the probe)
const KERNEL = new Set(["Dialog", "AlertDialog", "Sheet", "ContextMenu",
  "DropdownMenu", "Popover", "Tooltip", "HoverCard", "Select", "Tabs",
  "Slider", "ScrollArea"])
const TRIVIAL = new Set(["Accordion", "Collapsible", "Checkbox", "RadioGroup",
  "Switch", "Toggle", "ToggleGroup", "Progress", "Avatar", "AspectRatio",
  "Label", "Separator"])
const MEDIUM = new Set(["Menubar", "NavigationMenu", "Breadcrumb"])

const walk = (dir, out = []) => {
  if (!existsSync(dir)) return out
  for (const e of readdirSync(dir).sort()) {
    const p = join(dir, e)
    if (statSync(p).isDirectory()) walk(p, out)
    else out.push(p)
  }
  return out
}
const pkg = (d) => (d.startsWith("@") ? d.split("/").slice(0, 2).join("/") : d)

// tier classification of one ui source file (adapted from probes/h5-tiers.mjs)
const classify = (file) => {
  const raw = readFileSync(file, "utf8")
  const deps = [...raw.matchAll(/from "([^"]+)"/g)].map((m) => m[1])
  const ext = [...new Set(deps.map(pkg).filter((d) => EXTERNAL.has(d)))]
  if (ext.length) return { tier: "external", ext }
  // directive can use either quote style; strip before babel (single quotes
  // used to throw an unhandled parse error)
  const ast = parse(raw.replace(/^(['"])use client\1/m, ""), { sourceType: "module", plugins: ["typescript", "jsx"] })
  const hooks = new Set()
  const walkAst = (n) => {
    if (!n || !n.type) return
    if (n.type === "CallExpression" && n.callee.type === "Identifier" &&
        /^use[A-Z]/.test(n.callee.name)) hooks.add(n.callee.name)
    for (const k of Object.keys(n)) {
      if (k === "loc") continue
      const v = n[k]
      if (Array.isArray(v)) v.forEach((c) => c && typeof c.type === "string" && walkAst(c))
      else if (v && typeof v.type === "string") walkAst(v)
    }
  }
  walkAst(ast.program)
  const reactHooks = [...hooks].filter((h) =>
    /^use(State|Effect|Ref|Memo|Callback|Context|Id|LayoutEffect|ImperativeHandle|Transition|DeferredValue|SyncExternalStore|Optimistic|Reducer|DebugValue|InsertionEffect|FormState|ActionState)$/.test(h) || h === "use")
  if (reactHooks.length) return { tier: "logic", hooks: reactHooks }
  const radixAlias = [...raw.matchAll(/import\s*\{[^}]*?\bas\s+(\w+)Primitive\b[^}]*\}\s*from\s*"radix-ui"/g)].map((m) => m[1])
  const k = radixAlias.filter((a) => KERNEL.has(a))
  const t = radixAlias.filter((a) => TRIVIAL.has(a))
  const m = radixAlias.filter((a) => MEDIUM.has(a))
  if (m.length) return { tier: "medium", radix: radixAlias }
  if (k.length) return { tier: "kernel", radix: radixAlias }
  if (t.length) return { tier: "trivial-js", radix: radixAlias }
  return { tier: "static", radix: radixAlias }
}

// locate every trace of <name> in the pin (deterministic: sorted walks)
const regFiles = walk(`${APP}/registry`)
const exampleFiles = [...walk(`${APP}/examples`), ...walk(`${APP}/registry/bases/radix/examples`)]

const locate = (name) => {
  const pipelineUi = `${UI}/${name}.tsx`
  const docsPage = `${APP}/content/docs/components/radix/${name}.mdx`
  let srcTargets = []
  let deprecation = null
  if (existsSync(docsPage)) {
    const text = readFileSync(docsPage, "utf8").replace(/```[\s\S]*?```/g, " ")
    for (const m of text.matchAll(/<ComponentSource\b([^>]*)>/g)) {
      const src = (m[1].match(/\bsrc="([^"]*)"/) || [])[1]
      if (src && existsSync(`${APP}${src}`)) srcTargets.push(`${APP}${src}`)
    }
    const dep = text.match(/been deprecated\. Use the \[([\w-]+)\]/)
    if (dep) deprecation = dep[1]
  }
  return {
    name,
    pipelineUiExists: existsSync(pipelineUi),
    uiSources: regFiles.filter((p) => p.endsWith(`/ui/${name}.tsx`)),
    blockSources: regFiles.filter((p) => p.includes("/blocks/") && p.endsWith(`/${name}.tsx`)),
    examples: exampleFiles.filter((p) => new RegExp(`/${name}(-[\\w-]+)?\\.tsx$`).test(p)),
    srcTargets: [...new Set(srcTargets)].sort(),
    docsPage: existsSync(docsPage) ? docsPage : null,
    deprecation,
  }
}

// foreign deps + tombstoned internal imports across composition sources
const foreignDeps = (files) => {
  const out = new Set()
  for (const f of files) {
    for (const m of readFileSync(f, "utf8").matchAll(/from "([^"]+)"/g)) {
      const p = pkg(m[1])
      if (EXTERNAL.has(p)) out.add(p)
    }
  }
  return [...out].sort()
}
const tombstonedImports = (files) => {
  const out = new Set()
  for (const f of files) {
    for (const m of readFileSync(f, "utf8").matchAll(/@\/registry\/[^\s"]*\/ui\/([a-z0-9-]+)/g)) {
      if (TOMBSTONES.has(m[1])) out.add(m[1])
    }
  }
  return [...out].sort()
}

const report = (loc) => {
  const tier = loc.pipelineUiExists ? classify(`${UI}/${loc.name}.tsx`)
    : loc.uiSources.length ? classify(loc.uiSources[0]) : null
  const implement = loc.pipelineUiExists &&
    ["static", "kernel", "trivial-js"].includes(tier.tier)
  const disposition = implement ? "implement" : "grey"

  const lines = [
    `pin: ${PIN} (tag ${pin.shadcn_ui.tag})`,
    `pipeline registry: ${UI} — ui/${loc.name}.tsx ${loc.pipelineUiExists ? "PRESENT" : "absent"}`,
  ]
  if (loc.uiSources.length) {
    for (const p of loc.uiSources) lines.push(`evidence: ${p} (ui source, other product line; tier=${classify(p).tier})`)
  }
  for (const p of loc.blockSources) lines.push(`evidence: ${p} (registry block composition)`)
  for (const p of loc.examples.slice(0, 4)) lines.push(`evidence: ${p} (example composition)`)
  if (loc.examples.length > 4) lines.push(`evidence: … +${loc.examples.length - 4} more ${loc.name}-*.tsx examples`)
  for (const p of loc.srcTargets) lines.push(`evidence: ${p} (radix docs ComponentSource target — app example code)`)
  if (!loc.uiSources.length && !loc.blockSources.length && !loc.examples.length && !loc.srcTargets) {
    lines.push("evidence: absent from pin")
  }
  if (loc.deprecation) lines.push(`evidence: ${loc.docsPage} — deprecated upstream, points at ${loc.deprecation} (already a tombstone)`)

  const why = []
  if (!loc.pipelineUiExists && !loc.uiSources.length) why.push(`no ui/${loc.name}.tsx in any registry dir (not a registry primitive)`)
  if (!loc.pipelineUiExists && loc.uiSources.length) why.push(`ui wrapper exists only outside the pipeline registry (${loc.uiSources.map((p) => p.replace(`${APP}/registry/`, "")).join(", ")})`)
  if (tier?.tier === "external") why.push(`foreign runtime dep ${tier.ext.join(", ")} — no vanilla port (tombstone precedent)`)
  const comps = [...loc.blockSources, ...loc.srcTargets, ...loc.examples]
  const kinds = [loc.blockSources.length && "block", loc.srcTargets.length && "app example", loc.examples.length && "example"].filter(Boolean)
  if (comps.length) {
    const fd = foreignDeps(comps).filter((d) => !(tier?.ext || []).includes(d))
    const tb = tombstonedImports(comps)
    if (fd.length) why.push(`upstream sources are ${kinds.join("/")} compositions over ${fd.join(", ")} — no vanilla port (tombstone precedent)`)
    else if (tb.length) why.push(`upstream sources are ${kinds.join("/")} compositions wrapping tombstoned ${tb.join(", ")}`)
    else why.push(`upstream sources are ${kinds.join("/")} compositions, not registry primitives (pipeline unit = ui file)`)
  }
  if (loc.deprecation) why.push(`radix line deprecated upstream in favor of ${loc.deprecation}`)
  const reason = implement
    ? `single-file registry primitive, tier=${tier.tier} — mechanically convertible via the pipeline`
    : why.join("; ")
  return { name: loc.name, disposition, lines, reason }
}

let implementCount = 0
let greyCount = 0
for (const name of FIVE) {
  const r = report(locate(name))
  if (r.disposition === "implement") implementCount++
  else greyCount++
  console.log(`${r.name}: ${r.disposition}`)
  for (const l of r.lines) console.log(`  ${l}`)
  console.log(`  reason: ${r.reason}`)
}
console.log(`dispositions: ${implementCount} implement, ${greyCount} grey`)
console.log(`grey-list projection: 12 tombstones + ${greyCount} = ${12 + greyCount} (ledger updated by orchestrator)`)
