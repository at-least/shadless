// Emit pipeline/src/emit/default_content.rs from src/emitter/index.mjs's
// DEFAULT_CONTENT. Shape per entry: string | null | { html } | { attrs } |
// { children } | mix. Entry.set marks the key present-and-null (the Rust
// equivalent of JS undefined-vs-null). The file is the committed artifact;
// re-run this script after editing DEFAULT_CONTENT and commit the diff.
import { fileURLToPath, pathToFileURL } from "node:url"
import { join } from "node:path"
import { writeFileSync } from "node:fs"

// src/emitter/index.mjs reads several paths relative to cwd (e.g.
// readFileSync("src/registry/tiers.json", ...)), so this chdir is
// load-bearing — derive the repo root from this script's own location
// instead of hardcoding one checkout's absolute path.
const ROOT = fileURLToPath(new URL("..", import.meta.url))
process.chdir(ROOT)
const { DEFAULT_CONTENT } = await import(pathToFileURL(join(ROOT, "src/emitter/index.mjs")).href)

const snake = (s) => s.replace(/([A-Z])/g, "_$1").toLowerCase().replace(/^_/, "").replace(/-/g, "_")
const raw = (s) => {
  // pick a raw-string hash count the content cannot terminate
  const hashes = (() => {
    for (let n = 2; ; n++) if (!s.includes('"' + "#".repeat(n))) return n
  })()
  return `r${"#".repeat(hashes)}"${s}"${"#".repeat(hashes)}`
}
const vec = (pairs) =>
  `vec![${pairs.map(([k, v]) => `(${JSON.stringify(k)}.to_string(), ${JSON.stringify(v)}.to_string())`).join(", ")}]`

let out = `//! Code generated from src/emitter/index.mjs DEFAULT_CONTENT. DO NOT EDIT;
//! regenerate with tools/default-content-dump.mjs. Entry.set marks the key
//! present-and-null (the Rust equivalent of JS undefined-vs-null).

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Default, Clone)]
pub struct Entry {
    pub inner: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<(String, String)>,
    pub set: bool,
}

pub fn default_content() -> &'static HashMap<&'static str, HashMap<&'static str, Entry>> {
    static M: OnceLock<HashMap<&'static str, HashMap<&'static str, Entry>>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m: HashMap<&'static str, HashMap<&'static str, Entry>> = HashMap::new();
`
for (const [comp, fns] of Object.entries(DEFAULT_CONTENT)) {
  const cv = snake(comp)
  out += `        let mut ${cv}: HashMap<&'static str, Entry> = HashMap::new();\n`
  for (const [fn, entry] of Object.entries(fns)) {
    const fs = snake(fn)
    const ev = cv + (fs === cv ? "" : fs.startsWith(cv + "_") ? "_" + fs.slice(cv.length + 1) : "_" + fs) + "_e"
    const e = entry === null ? {} : typeof entry === "string" ? { html: entry } : entry
    const attrs = Object.entries(e.attrs ?? {})
    const children = Object.entries(e.children ?? {})
    out += `        let ${ev} = Entry {\n`
    out += `            inner: ${raw(e.html ?? "")}.to_string(),\n`
    out += `            attrs: ${attrs.length ? vec(attrs) : "vec![]"},\n`
    out += `            children: ${children.length ? vec(children) : "vec![]"},\n`
    out += `            set: true,\n`
    out += `        };\n`
    out += `        ${cv}.insert(${raw(fn)}, ${ev});\n`
  }
  out += `        m.insert(${raw(comp)}, ${cv});\n`
}
out += `        m
    })
}
`
writeFileSync(join(ROOT, "pipeline/src/emit/default_content.rs"), out)
console.log("written", out.length, "bytes")
