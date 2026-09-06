// Emit pipeline/default_content.go from src/emitter/index.mjs's DEFAULT_CONTENT.
// Shape per entry: string | null | { html } | { attrs } | { children } | mix.
// Go: type contentEntry struct { Inner string; Attrs map[string]string; Children map[string]string; Set bool }
// Set distinguishes "present and null" from "absent" (JS: undefined key).
import { fileURLToPath, pathToFileURL } from "node:url"
import { join } from "node:path"

// src/emitter/index.mjs reads several paths relative to cwd (e.g.
// readFileSync("src/registry/tiers.json", ...)), so this chdir is
// load-bearing — derive the repo root from this script's own location
// instead of hardcoding one checkout's absolute path.
const ROOT = fileURLToPath(new URL("..", import.meta.url))
process.chdir(ROOT)
const { DEFAULT_CONTENT } = await import(pathToFileURL(join(ROOT, "src/emitter/index.mjs")).href)

const q = (s) => JSON.stringify(s)
let out = `// Code generated from src/emitter/index.mjs DEFAULT_CONTENT. DO NOT EDIT;
// regenerate with tools/default-content-dump.mjs.
package main

// defaultContentEntry mirrors one entry of DEFAULT_CONTENT. Set marks the key
// present-and-null (the Go equivalent of JS undefined-vs-null).
type defaultContentEntry struct {
	Inner    string
	Attrs    map[string]string
	Children map[string]string
	Set      bool
}

var DEFAULT_CONTENT = map[string]map[string]defaultContentEntry{
`
for (const [comp, fns] of Object.entries(DEFAULT_CONTENT)) {
  out += `\t${q(comp)}: {\n`
  for (const [fn, entry] of Object.entries(fns)) {
    if (entry === null) { out += `\t\t${q(fn)}: {Set: true},\n`; continue }
    if (typeof entry === "string") { out += `\t\t${q(fn)}: {Set: true, Inner: escHtml(${q(entry)})},\n`; continue }
    out += `\t\t${q(fn)}: {Set: true`
    if (entry.html) out += `, Inner: ${q(entry.html)}`
    if (entry.attrs) { out += `, Attrs: []attrPair{`
      out += Object.entries(entry.attrs).map(([k,v])=>`{${q(k)}, ${q(v)}}`).join(", ") + `}`
    }
    if (entry.children) { out += `, Children: map[string]string{`
      out += Object.entries(entry.children).map(([k,v])=>`${q(k)}: ${q(v)}`).join(", ") + `}`
    }
    out += "},\n"
  }
  out += "\t},\n"
}
out += "}\n"
import { writeFileSync } from "node:fs"
writeFileSync("/tmp/default_content.go", out)
console.log("written", out.length, "bytes")
