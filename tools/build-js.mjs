// tools/build-js.mjs — the JS product surface, one base + one file per
// component, mirroring the CSS surface (core + shadless/<name>.css).
//
//   dist/shadless.js       vendored radix kernel + src/runtime/core.js
//                          (delegation engine, registry, theme, auto-init)
//   dist/shadless.min.js   the same, minified
//   dist/js/<name>.js      src/runtime/components/<name>.js — registers the
//                          component's behavior with the base; carousel
//                          bundles the vendored embla engine in front
//
//   dist/esm/shadless.min.mjs  the same, minified (the `import` condition of
//                          shadless/runtime.min)
//   dist/esm/shadless.mjs  the base as an ES module: the same IIFE body
//                          (window.shadless is still set — the component
//                          files address it by that global) followed by
//                          `export default shadless` + named exports
//   dist/esm/<name>.mjs    `import "./shadless.mjs"` + the component file,
//                          so a bundler evaluates the base first no matter
//                          how a consumer orders its imports
//   dist/esm/*.d.ts        src/runtime/shadless.d.ts beside the base; each
//                          component module is typed as a side-effect module
//
// Static components have no file; the docs say so per component.
import { readFileSync, writeFileSync, mkdirSync, rmSync, readdirSync } from "node:fs"
import { transformSync } from "esbuild"

// the members of window.shadless re-exported by name (kept in step with
// core.js's `global.shadless = {…}` — the ESM unit test asserts the set)
export const NAMED_EXPORTS = ["init", "initAll", "destroy", "refresh", "start", "stop", "register", "get", "instances", "h", "theme"]

// `;` between concatenated IIFEs: `})(window)` followed by `(function () {` would otherwise be a CALL
export const iifeBase = (kernel, core) => `${kernel}\n;\n${core}`
export const esmBase = (kernel, core) =>
  `${iifeBase(kernel, core)}\n;\nconst shadless = globalThis.shadless\nexport default shadless\nexport const { ${NAMED_EXPORTS.join(", ")} } = shadless\n`
export const esmComponent = (src) => `import "./shadless.mjs"\n;\n${src}`

export function buildJs(DIST = "dist") {
  const kernel = readFileSync("vendor/radix-kernel.iife.js", "utf8")
  const core = readFileSync("src/runtime/core.js", "utf8")
  const base = iifeBase(kernel, core)
  writeFileSync(`${DIST}/shadless.js`, base)
  writeFileSync(`${DIST}/shadless.min.js`, transformSync(base, { minify: true, target: "es2017" }).code)
  rmSync(`${DIST}/js`, { recursive: true, force: true })
  mkdirSync(`${DIST}/js`, { recursive: true })
  rmSync(`${DIST}/esm`, { recursive: true, force: true })
  mkdirSync(`${DIST}/esm`, { recursive: true })
  writeFileSync(`${DIST}/esm/shadless.mjs`, esmBase(kernel, core))
  // the `import` condition of shadless/runtime.min — the IIFE min has no
  // export statement, so an ESM consumer of the min entry got undefined
  writeFileSync(`${DIST}/esm/shadless.min.mjs`, transformSync(esmBase(kernel, core), { minify: true, target: "es2017", format: "esm" }).code)
  writeFileSync(`${DIST}/esm/shadless.d.ts`, readFileSync("src/runtime/shadless.d.ts", "utf8"))
  const names = []
  for (const f of readdirSync("src/runtime/components").filter((x) => x.endsWith(".js")).sort()) {
    const name = f.slice(0, -3)
    let src = readFileSync(`src/runtime/components/${f}`, "utf8")
    if (name === "carousel") src = readFileSync("vendor/embla-carousel.iife.js", "utf8") + "\n;\n" + src
    writeFileSync(`${DIST}/js/${f}`, src)
    writeFileSync(`${DIST}/esm/${name}.mjs`, esmComponent(src))
    writeFileSync(`${DIST}/esm/${name}.d.ts`, `// registers the ${name} behavior with the base (side-effect module)\nimport "./shadless.mjs"\nexport {}\n`)
    names.push(name)
  }
  return names
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].split("/").pop())) {
  const names = buildJs()
  console.log(`build-js: dist/shadless.js (base) + ${names.length} component files in dist/js/ (+ dist/esm/ mirrors)`)
}
