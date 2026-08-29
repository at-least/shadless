#!/usr/bin/env node
// probes/j1/classify.mjs — Wave J1 probe: can the emitted slot CSS be
// mechanically partitioned into structure vs style layers (a two-layer
// components/ vs styles/ split) under our fidelity constraint?
//
// Constraint: the layered build (base + style) must compile to CSS equal to
// the current monolith. That only holds if every RULE lands wholly in one
// layer with order preserved — a rule mixing structure and style utilities
// cannot be split without changing rule shape (two rules instead of one ⇒
// different cascade AND different bytes).
//
// This probe measures the mixing rate under a prefix table with three
// buckets: unambiguous structure, unambiguous style, ambiguous (counted as
// assignable to either side — they don't force a mix).
import { readFileSync, readdirSync } from "node:fs"
import { join } from "node:path"

const CSS_DIR = "dist/css"

// unambiguous STRUCTURE: layout / positioning / sizing / spacing / flow
const STRUCTURE = [
  /^(flex|grid|block|inline-flex|inline-block|inline-grid|inline|hidden|contents|flow-root|table|isolate|visible|invisible|collapse)$/,
  /^(relative|absolute|fixed|sticky|static)$/,
  /^(top|bottom|inset|start|end|inset-x|inset-y|inset-s|inset-e)-/,
  /^(h|w|size|min-h|min-w|max-h|max-w|max-w)-/,
  /^(p|px|py|ps|pe|pt|pb|p[sb][te]?|m|mx|my|ms|me|mt|mb|m[sb][te]?)-/,
  /^-?(p|m)/, // negative paddings/margins
  /^gap(-x|-y)?-/, /^(row|col)-/, /^cols-/, /^grid-/, /^flex-/,
  /^(items|justify|self|place|content|align)-/, /^justify-/,
  /^(overflow|overflow-x|overflow-y|overscroll)-/, /^z-/, /^order-/,
  /^(whitespace|truncate|text-ellipsis|text-clip|break-words|wrap-break-word|wrap-anywhere)$/,
  /^sr-only$/, /^not-sr-only$/, /^(pointer-events|select|appearance|resize)-/,
  /^aspect-/, /^object-/, /^(shrink|grow)-/, /^basis-/, /^list-/,
  /^(border-collapse|border-spacing)$/, /^table-/, /^(left|right)-/, // axis positioning (also RTL-flagged)
]

// unambiguous STYLE: color / radius / shadow / typography visuals / effects
const STYLE = [
  /^bg-/, /^(text|border|ring|outline|divide|accent|caret|fill|stroke|decoration|shadow|inset-ring)-/,
  /^text-(xs|sm|base|lg|xl|\d.?\d*x?l?|left|right|center)$/, // typography size/align
  /^(font|tracking|leading|antialiased|italic|not-italic|underline|no-underline|line-through|overline|uppercase|lowercase|capitalize|normal-case)$/,
  /^rounded(-[trbl]|-[s,e]|-[a-z]+)?/, 
  /^border(-[trblsyxe]+)?(-\d)?$/, /^border-[trbl]-\d/, /^border-[a-z]/, // width/color/side
  /^(opacity|translate|scale|rotate|skew|blur|brightness|grayscale|invert|sepia|saturate|contrast|drop-shadow)-/,
  /^(transition|duration|ease|animate|animation)-/, /^transition$/, /^animate-/,
  /^(backdrop|filter)-/, /^mix-blend/, /^bg-clip/, /^bg-(cover|contain|repeat|no-repeat|center)$/,
  /^outline(-\d|-none|hidden|solid|dashed)?/, /^ring(-\d|-inset|offset)?/, /^ring$/, /^shadow$/,
  /^(transition-\[.*)$/, /^caret-/, /^accent-/, /^snap-/, /^scroll-/, /^touch-/,
]

// ambiguous by omission: anything unmatched (counted per bucket, never
// forces a mix on its own)

const seg = (tok) => tok.slice(tok.lastIndexOf(":") + 1)

function classify(util) {
  const u = seg(util)
  if (STRUCTURE.some((re) => re.test(u))) return "structure"
  if (STYLE.some((re) => re.test(u))) return "style"
  return "ambiguous"
}

const rules = []
for (const f of readdirSync(CSS_DIR).filter((x) => x.endsWith(".css") && x !== "tokens.css").sort()) {
  const css = readFileSync(join(CSS_DIR, f), "utf8")
  for (const m of css.matchAll(/^  ([^{]+)\{\s*@apply ([^;]+);/gm)) {
    rules.push({ file: f.replace(/\.css$/, ""), sel: m[1].trim(), utils: m[2].split(/\s+/) })
  }
}

let pureS = 0, pureY = 0, mixed = 0, onlyAmb = 0
const mixedExamples = []
const ambUtils = new Map()
for (const r of rules) {
  const kinds = r.utils.map(classify)
  const hasS = kinds.includes("structure"), hasY = kinds.includes("style")
  const amb = r.utils.filter((_, i) => kinds[i] === "ambiguous")
  amb.forEach((u) => ambUtils.set(u, (ambUtils.get(u) || 0) + 1))
  if (hasS && hasY) { mixed++; if (mixedExamples.length < 8) mixedExamples.push(r) }
  else if (hasS) pureS++
  else if (hasY) pureY++
  else onlyAmb++
}

console.log(`rules total: ${rules.length}`)
console.log(`pure-structure(+amb): ${pureS}  pure-style(+amb): ${pureY}  ambiguous-only: ${onlyAmb}`)
console.log(`HARD-MIXED (needs rule split): ${mixed} (${(mixed / rules.length * 100).toFixed(1)}%)`)
console.log(`\ntop ambiguous utilities:`)
for (const [u, n] of [...ambUtils].sort((a, b) => b[1] - a[1]).slice(0, 15)) console.log(`  ${u} ×${n}`)
console.log(`\nmixed examples:`)
for (const r of mixedExamples)
  console.log(`  [${r.file}] ${r.sel.slice(0, 60)}\n    structure: ${r.utils.filter((u) => classify(u) === "structure").join(" ").slice(0, 100)}\n    style:     ${r.utils.filter((u) => classify(u) === "style").join(" ").slice(0, 100)}`)
