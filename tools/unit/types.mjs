// types — the ES-module entry's declarations type-check, and a consumer
// written against them compiles under --strict. dist/esm/*.d.ts were
// string-concatenated from src/runtime/shadless.d.ts and tsc never ran;
// the file could have been syntactically wrong (or describe a runtime that
// no longer existed) and every gate would stay green.
//
// The consumer sample exercises what a TS user actually does: narrow a
// handle on `component`, listen to the typed events, type the helpers, and
// set the auto-init opt-out.
import { spawnSync } from "node:child_process"
import { mkdirSync, writeFileSync, rmSync, readFileSync, existsSync, mkdtempSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"

const CONSUMER = `
import shadless, { get, init, h, theme, type Handle, type SelectHandle } from "shadless/runtime"
import "shadless/js/dialog"

const handle: Handle | null = get("#d1-trigger")
if (handle) {
  switch (handle.component) {
    case "select": {
      const v: string | null = handle.value()
      const l: string | null = handle.label()
      handle.select("#opt")
      void v; void l
      break
    }
    case "tabs": handle.activate(1, true); const i: number = handle.active(); void i; break
    case "slider": handle.setValue(3, 0, { commit: true }); const vs: number[] = handle.values(); void vs; break
    case "carousel": handle.scrollNext(); break
    default: handle.open(); handle.close(false); const o: boolean = handle.isOpen(); void o
  }
}
const sel = shadless.get("#s1-trigger") as SelectHandle | null
if (sel && sel.component === "select") sel.selected()

document.addEventListener("shadless:change", (e) => {
  const c: string = e.detail.component
  const values: number[] | undefined = e.detail.values
  const label: string | null | undefined = e.detail.label
  void c; void values; void label
})
document.addEventListener("shadless:commit", (e) => { const v: number[] = e.detail.values; void v })
document.addEventListener("shadless:themechange", (e) => { const m: "light" | "dark" = e.detail.mode; void m })

init(document.body, { force: true })
const idx: number = h.nextIndex(new KeyboardEvent("keydown"), [])
const rec = h.wire(document.body)
if (rec) document.body.addEventListener("click", () => {}, { signal: rec.signal })
h.formMirror(document.body, { read: () => true, write: (v) => { void v } })
theme.set("dark")
window.shadlessNoAutoInit = true
void idx
`

export function run(t) {
  // a scratch package with the REAL exports map (types conditions included)
  // and the SOURCE declarations in the positions build-js copies them to —
  // the consumer resolves "shadless/runtime" and "shadless/js/dialog" the
  // way a TS project does, so the types condition is exercised, not a path
  // OUTSIDE the repo: inside it, TypeScript's package self-reference (this
  // package.json is named "shadless") would resolve to the committed dist
  // instead of the scratch copy of the source declarations
  const OUT = mkdtempSync(join(tmpdir(), "shadless-types-"))
  mkdirSync(`${OUT}/node_modules/shadless/dist/esm`, { recursive: true })
  const pkg = JSON.parse(readFileSync("package.json", "utf8"))
  writeFileSync(`${OUT}/node_modules/shadless/package.json`, JSON.stringify({ name: "shadless", exports: pkg.exports }))
  writeFileSync(`${OUT}/node_modules/shadless/dist/esm/shadless.d.ts`, readFileSync("src/runtime/shadless.d.ts", "utf8"))
  writeFileSync(`${OUT}/node_modules/shadless/dist/esm/shadless.mjs`, "export default {}\n")
  writeFileSync(`${OUT}/node_modules/shadless/dist/esm/dialog.d.ts`, `import "./shadless.mjs"\nexport {}\n`)
  writeFileSync(`${OUT}/node_modules/shadless/dist/esm/dialog.mjs`, "")
  writeFileSync(`${OUT}/consumer.ts`, CONSUMER)
  const tsc = existsSync("node_modules/.bin/tsc") ? "node_modules/.bin/tsc" : null
  t.ok("types: typescript is installed (devDependency)", !!tsc)
  if (!tsc) return
  const r = spawnSync(tsc, ["--noEmit", "--strict", "--target", "es2020", "--module", "esnext", "--moduleResolution", "bundler",
    "--skipLibCheck", "--lib", "es2020,dom", `${OUT}/consumer.ts`], { encoding: "utf8" })
  const out = (r.stdout + r.stderr).trim().replaceAll(OUT + "/", "")
  rmSync(OUT, { recursive: true, force: true })
  t.ok(`types: shadless.d.ts through the exports map + a strict consumer compile${out ? `\n${out}` : ""}`, r.status === 0)
}
