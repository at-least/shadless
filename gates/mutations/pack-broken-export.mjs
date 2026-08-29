// The incident: `./runtime.min` was a bare string export, so the `import`
// condition resolved to an IIFE with no export statement. Reproduce the
// generic shape — an export target that points at nothing.
import { replaceOnce } from "./_util.mjs"
const F = "package.json"
export default {
  id: "pack-broken-export", gate: "pack", files: [F],
  why: "package.json exports point at a file the tarball does not carry (a README-documented specifier that cannot resolve)",
  apply() { replaceOnce(F, '"./dist/shadless.full.min.css"', '"./dist/shadless.full.MUTATED.css"') },
}
