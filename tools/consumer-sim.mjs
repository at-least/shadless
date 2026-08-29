#!/usr/bin/env node
// consumer-sim — machine-proof of the PRIMARY distribution path.
//
// The story we document: a consumer with only tailwindcss installed
// @imports shadless tokens + the per-component css files they use, pastes
// markup carrying inline utilities, and their own build emits exactly
// that component's styles. This gate simulates that consumer in a scratch
// directory and fails when any part of the story breaks:
//
//   - shadless.css must be self-contained (no @import beyond "tailwindcss"
//     itself — the animate layer is inlined at product-css time);
//   - the scratch build must compile clean;
//   - slot rules for the IMPORTED components must be present;
//   - ZERO rules for components that were not imported (file-granularity
//     tree shaking);
//   - inline utilities from the consumer's page must be emitted (their
//     content scan);
//   - theme variables must survive (dark mode flips on them);
//   - size sanity bound — catches "everything leaked into the build".
//
// The scratch dir lives inside the repo (node_modules resolvable for the
// tailwindcss CLI) and is the compile's --cwd, so its scan sees ONLY the
// consumer page — same shape as tools/tw.mjs hermetic product builds.
import { execFileSync } from "node:child_process"
import { readFileSync, writeFileSync, mkdirSync, rmSync, symlinkSync, readdirSync } from "node:fs"
import { resolve } from "node:path"

const ROOT = resolve(".")
const SIM = "probes/out/consumer-sim"
const IMPORTED = ["button", "alert"]
const NOT_IMPORTED = ["dialog", "accordion", "select", "tooltip", "carousel"]

rmSync(SIM, { recursive: true, force: true })
mkdirSync(SIM, { recursive: true },)
// install the package the way a consumer would have it: node_modules/
// shadless → this repo. The entry then imports through the REAL
// package.json exports map — the specifiers the docs document are the
// ones machine-checked here, not just file copies
mkdirSync(`${SIM}/node_modules`, { recursive: true })
symlinkSync(ROOT, `${SIM}/node_modules/shadless`, "dir")

writeFileSync(`${SIM}/entry.css`, `@import "shadless";
${IMPORTED.map((n) => `@import "shadless/${n}.css";`).join("\n")}
`)
// the consumer's page: shadless markup (data-slots + inline utilities)
writeFileSync(`${SIM}/page.html`, `<!doctype html>
<html><head><meta charset="utf-8"><title>consumer</title></head><body>
<button data-slot="button" data-variant="outline" class="font-medium">Continue</button>
<div data-slot="alert" data-variant="default">Account created — you are signed in.</div>
</body></html>
`)

// 1. core self-containment
const core = readFileSync("dist/shadless-core.css", "utf8")
const strayImports = [...core.matchAll(/^@import\s+("[^"]+"|url\([^)]*\));?$/gm)]
  .map((m) => m[1]).filter((s) => s !== '"tailwindcss"')
if (strayImports.length) {
  console.error(`FAIL  consumer-sim: shadless-core.css is not self-contained — @import ${strayImports.join(", ")} would need an extra package`)
  process.exit(1)
}

// 2. the consumer's own build
execFileSync(process.execPath, ["tools/tw.mjs", `${SIM}/entry.css`, `${SIM}/out.css`, "--cwd", SIM], { stdio: "inherit" })
const out = readFileSync(`${SIM}/out.css`, "utf8")

const problems = []
// 3. imported slot rules present
for (const n of IMPORTED) {
  if (!out.includes(`[data-slot="${n}"]`)) problems.push(`imported component ${n}: no slot rule in the consumer build`)
}
// 4. nothing from non-imported components
for (const n of NOT_IMPORTED) {
  if (out.includes(`[data-slot="${n}"]`)) problems.push(`tree-shaking broken: ${n} rules leaked into a build that never imported it`)
}
// 5. inline utilities from the consumer page
if (!/\.font-medium\b/.test(out)) problems.push("inline utility from the consumer page (font-medium) not emitted")
// 6. theme variables
if (!out.includes("--background:") || !out.includes(".dark")) problems.push("theme variables / .dark override missing from the consumer build")
// 7. size sanity — a full leak lands in the hundreds of KB
const KB = Math.round(out.length / 1024)
if (out.length > 80 * 1024) problems.push(`consumer build is ${KB}KB — the whole library leaked in (expected a couple dozen KB)`)

// 8. EVERY per-component stylesheet compiles individually with the core
// (2026-08-27 survey became a gate: a component that only compiles as part
// of the full product entry — e.g. referencing something a sibling part
// defines — would ship a file consumers cannot import alone)
let individualOk = 0
const individualFail = []
for (const f of readdirSync("dist/css").filter((x) => x.endsWith(".css"))) {
  const n = f.replace(".css", "")
  writeFileSync(`${SIM}/entry.css`, `@import "shadless";\n@import "shadless/${n}.css";\n`)
  try {
    execFileSync(process.execPath, ["tools/tw.mjs", `${SIM}/entry.css`, `${SIM}/out.css`, "--cwd", SIM], { stdio: "pipe" })
    individualOk++
  } catch { individualFail.push(n) }
}
if (individualFail.length) problems.push(`components that do NOT compile individually with the core: ${individualFail.join(", ")}`)

rmSync(SIM, { recursive: true, force: true })
if (problems.length) {
  console.error(`FAIL  consumer-sim\n  ${problems.join("\n  ")}`)
  process.exit(1)
}
console.log(`PASS  consumer-sim (${IMPORTED.length} components imported, tree-shaking intact, ${KB}KB build, core self-contained, ${individualOk}/${individualOk + individualFail.length} components compile individually)`)
