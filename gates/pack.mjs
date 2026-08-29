#!/usr/bin/env node
// gates/pack.mjs — the npm surface, machine-checked.
//
// Everything a consumer touches before any CSS or JS runs lives in
// package.json and README.md, and until this gate nothing asked whether
// those two agreed with each other or with the tarball:
//
//   - `./runtime.min` was a bare string export, so `import` resolved to the
//     IIFE (no export statement) and yielded undefined;
//   - README documented `@import "shadless (bare)"` — not a specifier;
//   - React + the whole conversion toolchain sat in `dependencies`, so the
//     "React-free" package installed React 19 transitively;
//   - the tarball shipped 3.9 MB of demo pages and dead glue.
//
// Checks (no network — `npm pack --dry-run --json` lists the tarball):
//   1. dependencies is empty (a React-free library must not install React);
//   2. every export target — each condition of each entry, patterns
//      expanded against dist/ — exists on disk AND is in the tarball;
//   3. every `shadless…` specifier the README's export table and code
//      fences document resolves through the exports map;
//   4. the tarball carries nothing outside the product surface (no demo
//      pages, no oracle stylesheets).
import { execFileSync } from "node:child_process"
import { readFileSync, existsSync, readdirSync } from "node:fs"

const pkg = JSON.parse(readFileSync("package.json", "utf8"))
const fail = []

// 1. dependencies
const deps = Object.keys(pkg.dependencies ?? {})
if (deps.length) fail.push(`dependencies must be empty (React-free means installing nothing): ${deps.join(", ")}`)

// 2. exports → files
const targets = new Map() // specifier-ish → [file paths]
const addTarget = (key, cond, value) => {
  if (typeof value === "string") {
    if (value.includes("*")) {
      const [pre, post] = value.split("*")
      const dir = pre.replace(/^\.\//, "").replace(/\/[^/]*$/, "")
      const stem = pre.slice(pre.lastIndexOf("/") + 1)
      const files = existsSync(dir) ? readdirSync(dir).filter((f) => f.startsWith(stem) && f.endsWith(post)) : []
      if (!files.length) fail.push(`export ${key} [${cond}] → ${value}: pattern matches nothing`)
      targets.set(`${key} [${cond}]`, files.map((f) => `${dir}/${f}`))
    } else targets.set(`${key} [${cond}]`, [value.replace(/^\.\//, "")])
  } else if (value && typeof value === "object") {
    for (const [c, v] of Object.entries(value)) addTarget(key, c, v)
  }
}
for (const [k, v] of Object.entries(pkg.exports ?? {})) addTarget(k, "default", v)
if (typeof pkg.style === "string") targets.set("style", [pkg.style.replace(/^\.\//, "")])
// the `import` condition must resolve to a real ES module
for (const [k, v] of Object.entries(pkg.exports ?? {})) {
  if (v && typeof v === "object" && typeof v.import === "string" && !v.import.includes("*")) {
    const src = existsSync(v.import) ? readFileSync(v.import, "utf8") : ""
    if (!/\bexport[\s{]/.test(src)) fail.push(`export ${k} [import] → ${v.import} has no export statement (an IIFE under the import condition yields undefined)`)
  }
}

const packed = new Set(JSON.parse(execFileSync("npm", ["pack", "--dry-run", "--json", "--ignore-scripts"], { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }))[0].files.map((f) => f.path))
for (const [key, files] of targets) for (const f of files) {
  if (!existsSync(f)) fail.push(`export ${key} → ${f}: file does not exist`)
  else if (!packed.has(f)) fail.push(`export ${key} → ${f}: not in the tarball (package.json "files")`)
}

// 3. README specifiers resolve
const readme = readFileSync("README.md", "utf8")
const specs = new Set()
for (const m of readme.matchAll(/`(shadless(?:\/[^`\s]*)?)`/g)) specs.add(m[1])
for (const m of readme.matchAll(/(?:@import|from|import)\s+"(shadless(?:\/[^"]*)?)"/g)) specs.add(m[1])
const resolves = (spec) => {
  const sub = spec === "shadless" ? "." : "." + spec.slice("shadless".length)
  if (pkg.exports[sub]) return true
  for (const k of Object.keys(pkg.exports)) {
    if (!k.includes("*")) continue
    const [pre, post] = k.split("*")
    if (sub.startsWith(pre) && sub.endsWith(post) && sub.length > pre.length + post.length) return true
  }
  return false
}
for (const s of specs) {
  const probe = s.replace("<name>", "button") // the README's placeholder
  if (!resolves(probe)) fail.push(`README documents "${s}" but package.json exports do not resolve it`)
}

// 4. nothing outside the product surface
const allowed = /^(package\.json|README\.md|CHANGELOG\.md|LICENSE|dist\/(css|js|esm)\/[^/]+|dist\/shadless(-core|\.full(\.min)?)?\.(css|js)|dist\/shadless\.min\.js)$/
for (const f of packed) if (!allowed.test(f)) fail.push(`tarball carries ${f} — outside the product surface`)

if (fail.length) {
  console.error(`FAIL  pack (${fail.length} problems)\n  ` + fail.join("\n  "))
  process.exit(1)
}
console.log(`PASS  pack (${targets.size} export targets in a ${packed.size}-file tarball; ${specs.size} README specifiers resolve; dependencies empty)`)
