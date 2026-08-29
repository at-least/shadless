#!/usr/bin/env node
// unit-check.mjs — seconds-level regression net between `node --check`
// (syntax only) and the full verify chain (minutes).
//
// Why this exists (2026-08-25 incident): a cleanup round deleted a
// regex replace in tools/demo.mjs's rewritePaths as "dead code" — it
// wasn't (`[^"]*-out\.css` cannot match bare `out.css`; the dash is
// mandatory). `node --check` passed; the failure only surfaced minutes
// later at the demo-smoke step of verify. The pure functions touched in
// cleanup rounds deserve a seconds-level gate with real assertions.
//
// Wave H (2026-08-26): assertions live in tools/unit/*.mjs modules by
// area; this file is just the runner. Zero framework, same convention as
// every other gate: inline asserts, exit 1 on any failure, honest PASS.
import { makeT } from "./unit/harness.mjs"
import * as frontmatter from "./unit/frontmatter.mjs"
import * as css from "./unit/css.mjs"
import * as prepaint from "./unit/prepaint.mjs"
import * as routes from "./unit/routes.mjs"
import * as converter from "./unit/converter.mjs"
import * as emitter from "./unit/emitter.mjs"
import * as runtime from "./unit/runtime.mjs"
import * as docsTools from "./unit/docs-tools.mjs"
import * as docsFidelity from "./unit/docs-fidelity.mjs"
import * as transforms from "./unit/transforms.mjs"
import * as rtl from "./unit/rtl.mjs"
import * as cssDirection from "./unit/css-direction.mjs"
import * as productCss from "./unit/product-css.mjs"
import * as types from "./unit/types.mjs"

const suites = [
  ["frontmatter", frontmatter],
  ["css", css],
  ["prepaint", prepaint],
  ["routes", routes],
  ["converter", converter],
  ["emitter", emitter],
  ["runtime", runtime],
  ["docs-tools", docsTools],
  ["docs-fidelity", docsFidelity],
  ["transforms", transforms],
  ["rtl", rtl],
  ["css-direction", cssDirection],
  ["product-css", productCss],
  ["types", types],
]

const failures = []
let n = 0
for (const [name, mod] of suites) {
  const t = makeT(name, failures)
  await mod.run(t) // suites may be async (runtime/observer tests)
  n += t.count
}

if (failures.length) {
  console.log(`\nFAIL  unit-check (${failures.length}/${n} assertions failed)`)
  process.exit(1)
}
console.log(`PASS  unit-check (${n} assertions across ${suites.map((s) => s[0]).join("/")})`)
