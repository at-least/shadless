#!/usr/bin/env node
// Regenerate pipeline/internal/twmerge/config.json and snapshot.json after a
// deliberate tailwind-merge version bump:
//
//   node tools/twmerge-dump.mjs
//
// config.json — the real getDefaultConfig() with every function replaced by
// a token ({$fn: bundleIdentifier} / {$theme: key}) plus classGroupOrder,
// because Go maps do not preserve the JS Object.keys iteration order that
// determines validator precedence at shared trie prefixes.
//
// snapshot.json — twMerge(input) for every class list the repo actually
// feeds it (upstream ui/*.tsx string literals + dist/components html class
// attributes). pipeline/internal/twmerge/conformance_test.go must agree
// 100% with this file after a bump.
import { getDefaultConfig, validators, twMerge } from "tailwind-merge"
import { writeFileSync, readFileSync, readdirSync } from "node:fs"
import { join } from "node:path"
import { parse } from "@babel/parser"

// ---- config dump -----------------------------------------------------------
const fnNames = new Map(Object.entries(validators).map(([k, v]) => [v, k]))
const cfg = getDefaultConfig()

function enc(v) {
  if (typeof v === "function") {
    if ("isThemeGetter" in v) {
      const probe = new Proxy({}, { get: (_t, p) => [`§${String(p)}`] })
      const key = v(probe)[0].slice(1)
      if (!(key in cfg.theme)) throw new Error("theme getter for unknown key " + key)
      return { $theme: key }
    }
    const name = fnNames.get(v)
    if (!name) throw new Error("unnamed validator: " + v.toString())
    return { $fn: name }
  }
  if (Array.isArray(v)) return v.map(enc)
  if (v && typeof v === "object")
    return Object.fromEntries(Object.entries(v).map(([k, x]) => [k, enc(x)]))
  return v
}

writeFileSync("pipeline/internal/twmerge/config.json", JSON.stringify({
  theme: enc(cfg.theme),
  classGroups: enc(cfg.classGroups),
  classGroupOrder: Object.keys(cfg.classGroups),
  conflictingClassGroups: cfg.conflictingClassGroups,
  conflictingClassGroupModifiers: cfg.conflictingClassGroupModifiers,
  postfixLookupClassGroups: cfg.postfixLookupClassGroups ?? [],
  orderSensitiveModifiers: cfg.orderSensitiveModifiers,
}))

// ---- snapshot ---------------------------------------------------------------
const inputs = new Set()
const DIR = ".upstream/shadcn-ui/apps/v4/registry/bases/radix/ui"
for (const f of readdirSync(DIR).filter((x) => x.endsWith(".tsx"))) {
  const src = readFileSync(join(DIR, f), "utf8")
  const ast = parse(src, { sourceType: "module", plugins: ["typescript", "jsx"] })
  const walk = (n) => {
    if (!n || typeof n.type !== "string") return
    if (n.type === "StringLiteral" && n.value.includes(" ") &&
        /(^|\s)([a-z][a-z0-9-]*-|\[|data-)/.test(n.value)) inputs.add(n.value)
    for (const k of Object.keys(n)) {
      if (k === "loc" || k === "start" || k === "end") continue
      const v = n[k]
      if (Array.isArray(v)) v.forEach(walk)
      else if (v && typeof v.type === "string") walk(v)
    }
  }
  walk(ast.program)
}
for (const f of readdirSync("dist/components").filter((x) => x.endsWith(".html"))) {
  const html = readFileSync(`dist/components/${f}`, "utf8")
  for (const m of html.matchAll(/class="([^"]+)"/g)) inputs.add(m[1])
}
const clean = [...inputs].filter((s) => !s.includes("&") && !s.includes("<") && s.length < 2000)
writeFileSync("pipeline/internal/twmerge/snapshot.json",
  JSON.stringify(clean.map((s) => [s, twMerge(s)])))
console.log(`twmerge-dump: config + ${clean.length} snapshot cases`)
