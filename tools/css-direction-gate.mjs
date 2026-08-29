#!/usr/bin/env node
// css-direction-gate (J5) — inventory physical reading-direction utilities
// in the emitted CSS and hold them to a committed baseline.
//
// Why a baseline instead of "fail on any physical utility": dist/shadless.css
// is a MECHANICAL conversion of the pinned upstream registry. Where upstream
// shadcn uses physical utilities (ml-/pr-/rounded-r-/left-…), fidelity to the
// oracle REQUIRES emitting them — the gate's job is not to forbid them but to
// make drift loud: a new or disappeared physical utility means the upstream
// pin moved the RTL story, and that must be a visible decision at sync time,
// not a silent regression in the -rtl-* variant pages.
//
// Scan target: dist/shadless.css (the library's emitted source; hand-authored
// demo markup is out of scope). Runs after the demo css build so the
// scanned state is the shipped one (committed dist carries the overlay).
//
// Usage:
//   node tools/css-direction-gate.mjs           # compare against baseline
//   node tools/css-direction-gate.mjs --update  # re-record after review
import { readFileSync } from "node:fs"
import { fileURLToPath } from "node:url"
import path from "node:path"

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..")
const CSS_PATH = path.join(ROOT, "dist/shadless.css")

// Physical reading-direction utility SHAPES (logical twins: ms/me/ps/pe/
// start/end/rounded-s/rounded-e/inset-s/inset-e are fine). Matched against
// the utility segment AFTER stripping variant prefixes (responsive, has-*,
// named group, arbitrary-variant, pseudo-element).
// NOTE: comments in this file must NOT contain literal utility syntax —
// tracked files are content-scanned by the tailwind CLI (see tools/tw.mjs)
// and literals here would surface as phantom rules in dist/out.css.
const PHYSICAL_PATTERNS = [
  /^m[lr]-/,            // physical margin start/end
  /^p[lr]-/,            // physical padding start/end
  /^s[lr]-/,            // physical scroll margin/padding (sr-only sanctioned below)
  /^(left|right)-/,     // absolute positioning on the reading axis — review class
  /^text-(left|right)$/, // text alignment on the reading axis
  /^border-[lr]-\d/,    // physical horizontal border width (color lookalikes safe)
  /^rounded-[lr]-/,     // physical start/end corner radius (symmetric radii safe)
  /^(space|divide)-x/,  // physical sibling spacing
]

// Utilities that LOOK physical but are sanctioned (a11y semantics).
const SANCTIONED = new Set(["sr-only"])

// Strip variant prefixes: keep only the segment after the LAST ':' (covers
// sm:, has-[>kbd]:, group-data-[…]/attachment:, [&>…]:, before:…). Arbitrary
// values may contain ':' inside brackets — those never hold our utilities.
export function utilitySegment(token) {
  return token.slice(token.lastIndexOf(":") + 1)
}

export function isPhysicalUtility(token) {
  const u = utilitySegment(token)
  if (SANCTIONED.has(u)) return false
  return PHYSICAL_PATTERNS.some((re) => re.test(u))
}

// Extract every @apply token list from the CSS source.
export function extractApplyTokens(css) {
  const tokens = []
  for (const m of css.matchAll(/@apply([^;]+);/g)) {
    for (const tok of m[1].split(/\s+/)) if (tok) tokens.push(tok)
  }
  return tokens
}

// token -> count for the physical subset, sorted for stable baselines.
export function scanDirections(css) {
  const counts = new Map()
  for (const tok of extractApplyTokens(css)) {
    if (!isPhysicalUtility(tok)) continue
    const u = utilitySegment(tok)
    counts.set(u, (counts.get(u) || 0) + 1)
  }
  return [...counts.entries()].sort((a, b) => a[0].localeCompare(b[0]))
}

// Baseline: the recorded physical-utility inventory of the current pin.
// Tokens are stored as SPLIT PAIRS (joined at load) because this file is
// tracked and content-scanned by the tailwind CLI's repo-wide auto-scan
// (see tools/tw.mjs) — literal utility strings here would surface as
// phantom rules in dist/out.css. When re-recording, keep the pair shape.
const BASELINE = [
  [["border", "-l-0"], 1],
  [["left", "-3"], 1],
  [["ml", "-[-0.15rem]"], 1],
  [["ml", "-[-0.3rem]"], 1],
  [["ml", "-1"], 1],
  [["mr", "-[-0.15rem]"], 1],
  [["mr", "-[-0.3rem]"], 1],
  [["mr", "-1"], 1],
  [["pl", "-1.5"], 4],
  [["pl", "-1.5!"], 1],
  [["pl", "-2"], 4],
  [["pl", "-2.5"], 1],
  [["pr", "-0"], 2],
  [["pr", "-1.5"], 4],
  [["pr", "-1.5!"], 1],
  [["pr", "-18"], 1],
  [["pr", "-2"], 4],
  [["pr", "-8"], 1],
  [["right", "-2"], 1],
  [["right", "-2.5"], 1],
  [["right", "-3"], 2],
  [["rounded", "-l-none"], 1],
  [["rounded", "-r-lg"], 1],
  [["rounded", "-r-lg!"], 1],
  [["rounded", "-r-none"], 1],
  [["text", "-left"], 5],
].map(([pair, n]) => [pair.join(""), n])

// --update prints the fresh inventory in the same split-pair shape so it
// can be pasted back into BASELINE.
function printBaselineEntries(entries) {
  return entries.map(([tok, n]) => {
    const m = tok.match(/^([a-z]+)(.*)$/)
    return `  [["${m[1]}", "${m[2]}"], ${n}],`
  }).join("\n")
}

function formatEntries(entries) {
  return entries.map(([tok, n]) => `  ${tok} ×${n}`).join("\n")
}

function main() {
  const css = readFileSync(CSS_PATH, "utf8")
  const entries = scanDirections(css)

  if (process.argv.includes("--update")) {
    console.log(`# fresh inventory (${entries.length} entries) — paste into BASELINE in tools/css-direction-gate.mjs after review:\n${printBaselineEntries(entries)}`)
    return
  }

  const baseline = BASELINE

  const bmap = new Map(baseline)
  const cmap = new Map(entries)
  const diffs = []
  for (const [tok, n] of entries)
    if (!bmap.has(tok)) diffs.push(`  + ${tok} ×${n} (new)`)
    else if (bmap.get(tok) !== n) diffs.push(`  ~ ${tok}: ×${n} was ×${bmap.get(tok)}`)
  for (const [tok, n] of baseline)
    if (!cmap.has(tok)) diffs.push(`  - ${tok} ×${n} (gone)`)

  if (diffs.length) {
    console.error(`FAIL  css-direction-gate: emitted physical utilities drifted from baseline\n${diffs.join("\n")}`)
    console.error(`\nIf intended (upstream re-pin / reviewed change): re-record with node tools/css-direction-gate.mjs --update`)
    process.exit(1)
  }
  console.log(`PASS  css-direction-gate (${entries.length} physical utilities match baseline)`)
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main()
