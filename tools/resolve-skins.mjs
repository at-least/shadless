#!/usr/bin/env node
// resolve-skins — upstream generation parity at the SOURCE level.
//
// Upstream's generator (packages/shadcn/src/styles/transform-style-map.ts)
// resolves the cn-* semantic classes in bases/*/ui/*.tsx into the skin's
// inline Tailwind utilities BEFORE anything renders: their docs site and
// their generated styles/<flavor>-<skin>/ui both ship plain utility
// classes. shadless previously let cn-* names ride the whole pipeline and
// compensated downstream (skin @utility injection + anti-tree-shake
// sentinel + post-hoc DOM class expansion) — three patches, all deleted
// in favor of this one resolve step.
//
// Semantics (verified against the live ui.shadcn.com payload):
//   - cn-X in SKIN_MAP (style-nova.css @apply body) → expanded in place
//   - allowlisted names (transform-style-map.ts ALLOWLIST) → kept as-is;
//     the live site CSS defines only .cn-font-heading among them — the
//     rest are inert marker classes resolved at CLI-install time
//   - marker-only names styled by no skin → DROPPED (payload carries 0
//     cn-field-orientation-* though the structural tsx has them)
//   - each class-string literal passes twMerge (cn() = clsx+twMerge, so
//     the DOM-side merge is replicated at source)
//
// Output: probes/out/resolved-ui/ — a structural copy of bases/radix
// (ui/ transformed, lib+hooks verbatim) that the converter, contracts
// oracle and example oracle consume instead of bases/radix directly.
// Import specifiers are left untouched (@/registry/bases/radix/…), so
// consumers alias that prefix here.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, copyFileSync, rmSync } from "node:fs"
import { join, dirname } from "node:path"
import { parse } from "@babel/parser"
import { twMerge } from "tailwind-merge"
import { SKIN_MAP, SKIN_ALLOWLIST } from "../src/emitter/skin.mjs"

const SRC = ".upstream/shadcn-ui/apps/v4/registry/bases/radix"
const OUT = "probes/out/resolved-ui"

// ---- class-string expansion -------------------------------------------------
const CN_TOKEN = /(^|\s)(cn-[a-z0-9-]+)/g

export function expandClassString(str) {
  const toks = str.split(/\s+/).filter(Boolean)
  if (!toks.some((t) => t.startsWith("cn-"))) return str
  const expanded = toks.flatMap((t) => {
    if (!t.startsWith("cn-")) return [t]
    if (SKIN_ALLOWLIST.has(t)) return [t]
    const body = SKIN_MAP[t]
    return body ? body.split(/\s+/).filter(Boolean) : []
  })
  return twMerge(expanded.join(" "))
}

// ---- RTL transform (upstream packages/shadcn transform-rtl.ts parity) -------
// The GENERATED ui-rtl trees run this AFTER skin resolution: physical
// classes → logical equivalents, translate-x gains rtl: counterparts,
// cn-rtl-flip → rtl:rotate-180. Verified against the live payload
// (accordion-rtl trigger: ml-auto → ms-auto).
const RTL_MAPPINGS = [
  ["-ml-", "-ms-"], ["-mr-", "-me-"], ["ml-", "ms-"], ["mr-", "me-"],
  ["pl-", "ps-"], ["pr-", "pe-"], ["-left-", "-start-"], ["-right-", "-end-"],
  ["left-", "start-"], ["right-", "end-"], ["inset-l-", "inset-inline-start-"],
  ["inset-r-", "inset-inline-end-"], ["rounded-tl-", "rounded-ss-"],
  ["rounded-tr-", "rounded-se-"], ["rounded-bl-", "rounded-es-"],
  ["rounded-br-", "rounded-ee-"], ["rounded-l-", "rounded-s-"],
  ["rounded-r-", "rounded-e-"], ["border-l-", "border-s-"], ["border-r-", "border-e-"],
  ["border-l", "border-s"], ["border-r", "border-e"], ["text-left", "text-start"],
  ["text-right", "text-end"], ["scroll-ml-", "scroll-ms-"], ["scroll-mr-", "scroll-me-"],
  ["scroll-pl-", "scroll-ps-"], ["scroll-pr-", "scroll-pe-"],
  ["float-left", "float-start"], ["float-right", "float-end"],
  ["clear-left", "clear-start"], ["clear-right", "clear-end"],
  ["origin-top-left", "origin-top-start"], ["origin-top-right", "origin-top-end"],
  ["origin-bottom-left", "origin-bottom-start"], ["origin-bottom-right", "origin-bottom-end"],
  ["origin-left", "origin-start"], ["origin-right", "origin-end"],
]
const RTL_TRANSLATE_X = [["-translate-x-", "translate-x-"], ["translate-x-", "-translate-x-"]]
const RTL_REVERSE = [["space-x-", "space-x-reverse"], ["divide-x-", "divide-x-reverse"]]
const RTL_SWAP = [["cursor-w-resize", "cursor-e-resize"], ["cursor-e-resize", "cursor-w-resize"]]
const RTL_LOGICAL_SLIDE = [
  ["data-[side=inline-start]", "slide-in-from-right", "slide-in-from-end"],
  ["data-[side=inline-start]", "slide-out-to-right", "slide-out-to-end"],
  ["data-[side=inline-end]", "slide-in-from-left", "slide-in-from-start"],
  ["data-[side=inline-end]", "slide-out-to-left", "slide-out-to-start"],
]
const POSITIONING_PREFIXES = ["-left-", "-right-", "left-", "right-"]

// [variant, name, alpha] with bracket-aware last-colon split (upstream
// splitClassName semantics)
export function splitClassName(cls) {
  let lastColon = -1, depth = 0
  for (let i = cls.length - 1; i >= 0; i--) {
    const c = cls[i]
    if (c === "]") depth++
    else if (c === "[") depth--
    else if (c === ":" && depth === 0) { lastColon = i; break }
  }
  const variant = lastColon === -1 ? null : cls.slice(0, lastColon)
  const rest = lastColon === -1 ? cls : cls.slice(lastColon + 1)
  // alpha modifier: first "/" outside brackets
  let slash = -1; depth = 0
  for (let i = 0; i < rest.length; i++) {
    const c = rest[i]
    if (c === "[") depth++
    else if (c === "]") depth--
    else if (c === "/" && depth === 0) { slash = i; break }
  }
  const value = slash === -1 ? rest : rest.slice(0, slash)
  const alpha = slash === -1 ? null : rest.slice(slash + 1)
  return [variant, value, alpha]
}

export function applyRtlMapping(input) {
  return input.split(/\s+/).filter(Boolean).flatMap((cls) => {
    if (cls.startsWith("rtl:") || cls.startsWith("ltr:")) return [cls]
    if (cls === "cn-rtl-flip") return ["rtl:rotate-180"]
    const [variant, value, alpha] = splitClassName(cls)
    if (!value) return [cls]
    const m = (v) => alpha ? `${v}/${alpha}` : v
    for (const [phys, rtlPhys] of RTL_TRANSLATE_X)
      if (value.startsWith(phys)) {
        const rtlValue = value.replace(phys, rtlPhys)
        return [cls, variant ? `rtl:${variant}:${m(rtlValue)}` : `rtl:${m(rtlValue)}`]
      }
    for (const [prefix, reverse] of RTL_REVERSE)
      if (value.startsWith(prefix)) {
        const r = variant ? `rtl:${variant}:${reverse}` : `rtl:${reverse}`
        return [cls, r]
      }
    for (const [phys, swapped] of RTL_SWAP)
      if (value === phys) {
        const r = variant ? `rtl:${variant}:${swapped}` : `rtl:${swapped}`
        return [cls, r]
      }
    for (const [vp, phys, logical] of RTL_LOGICAL_SLIDE)
      if (variant?.includes(vp) && value.startsWith(phys)) {
        const mapped = value.replace(phys, logical)
        return [variant ? `${variant}:${m(mapped)}` : m(mapped)]
      }
    const isPhysSide = variant?.includes("data-[side=left]") || variant?.includes("data-[side=right]")
    let mapped = value
    for (const [phys, logical] of RTL_MAPPINGS) {
      if (isPhysSide && POSITIONING_PREFIXES.some((p) => phys.startsWith(p))) continue
      if (value.startsWith(phys)) {
        if (!phys.endsWith("-") && value !== phys) continue
        mapped = value.replace(phys, logical)
        break
      }
    }
    const result = variant ? `${variant}:${mapped}` : mapped
    return [m(result)]
  }).join(" ")
}

// Transform one tsx source: every StringLiteral whose value carries a
// cn-* token is replaced (by source-offset splice, reverse order) with
// the expanded + merged literal. Import specifiers never contain cn-
// tokens, so no scope filtering is needed. rtl=true additionally runs
// applyRtlMapping over every class-bearing literal (upstream's ui-rtl
// generation order: skin resolution first, then RTL transform).
export function resolveSource(src, { rtl = false } = {}) {
  const ast = parse(src, { sourceType: "module", plugins: ["typescript", "jsx"] })
  const edits = []
  const walk = (n) => {
    if (!n || typeof n.type !== "string") return
    if (n.type === "StringLiteral" && CN_TOKEN.test(n.value)) {
      CN_TOKEN.lastIndex = 0
      let next = expandClassString(n.value)
      if (rtl) next = applyRtlMapping(next)
      if (next !== n.value) edits.push({ start: n.start, end: n.end, value: JSON.stringify(next) })
    } else if (rtl && n.type === "StringLiteral" && /(^|\s)(ml-|mr-|pl-|pr-|left-|right-|rounded-[tlbr]+-|border-[lr]\b|text-(left|right)|translate-x-|space-x-|divide-x-|float-|clear-|origin-|scroll-[mp][lr]-|inset-[lr]-|cursor-[we]-resize)/.test(n.value)) {
      // class-looking literal with RTL-mappable tokens but no cn-* — RTL
      // transform applies to it too (upstream transforms cva base/variants
      // and className literals regardless of skin classes)
      const next = applyRtlMapping(n.value)
      if (next !== n.value) edits.push({ start: n.start, end: n.end, value: JSON.stringify(next) })
    }
    for (const k of Object.keys(n)) {
      if (k === "loc" || k === "start" || k === "end") continue
      const v = n[k]
      if (Array.isArray(v)) v.forEach(walk)
      else if (v && typeof v.type === "string") walk(v)
    }
  }
  walk(ast.program)
  let out = src
  for (const e of edits.sort((a, b) => b.start - a.start)) {
    out = out.slice(0, e.start) + e.value + out.slice(e.end)
  }
  return { out, edits: edits.length }
}

// ---- fixture pass ------------------------------------------------------------
// Kernel fixtures (src/kernel/*.html, generated from the contract oracles) carry
// cn-* class tokens (Wave L leftovers). Expand them through the same map
// so shipped HTML carries zero non-allowlist cn-*. Idempotent: expanded
// strings contain no cn- tokens, so a second run is a no-op.
export function resolveFixtureHtml(html) {
  return html.replace(/\bclass="([^"]*)"/g, (_m, cls) => {
    if (!/(^|\s)cn-/.test(cls)) return `class="${cls}"`
    const next = expandClassString(cls)
    return next ? `class="${next}"` : `class=""`
  })
}

// ---- drive ------------------------------------------------------------------
const isMain = process.argv[1] && process.argv[1].endsWith("resolve-skins.mjs")
if (isMain) {
  rmSync(OUT, { recursive: true, force: true })
  mkdirSync(OUT, { recursive: true })
  let files = 0, edits = 0
  const copyTree = (dir, outDir, transform) => {
    mkdirSync(outDir, { recursive: true })
    for (const f of readdirSync(dir, { withFileTypes: true })) {
      const p = join(dir, f.name), o = join(outDir, f.name)
      if (f.isDirectory()) { copyTree(p, o, transform); continue }
      const src = readFileSync(p, "utf8")
      if (transform && f.name.endsWith(".tsx")) {
        const { out, edits: n } = transform(src)
        writeFileSync(o, out); files++; edits += n
      } else copyFileSync(p, o)
    }
  }
  // ui/ resolved (LTR + RTL trees); lib/, hooks/ verbatim (utils cn()
  // itself is used by the runtime, untouched). blocks/components/examples
  // are not consumed.
  copyTree(join(SRC, "ui"), join(OUT, "ui"), (s) => resolveSource(s))
  copyTree(join(SRC, "ui"), join(OUT, "ui-rtl"), (s) => resolveSource(s, { rtl: true }))
  copyTree(join(SRC, "lib"), join(OUT, "lib"), false)
  copyTree(join(SRC, "hooks"), join(OUT, "hooks"), false)
  // fixtures
  const fixtures = process.argv.includes("--fixtures")
  if (fixtures) {
    let fx = 0
    for (const f of readdirSync("src/kernel").filter((x) => x.endsWith(".html"))) {
      const p = `src/kernel/${f}`
      const html = readFileSync(p, "utf8")
      const next = resolveFixtureHtml(html)
      if (next !== html) { writeFileSync(p, next); fx++ }
    }
    console.log(`resolve-skins: fixtures rewritten: ${fx}`)
  }
  console.log(`resolve-skins: ${files} ui files resolved (${edits} class strings), tree at ${OUT}`)
}
