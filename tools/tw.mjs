#!/usr/bin/env node
// tw.mjs (J2) — hermetic @tailwindcss/cli wrapper.
//
// Why this exists (2026-08-26, found via the +88-line out.css diff): the
// CLI's automatic source detection scans the CWD's tracked repo files. Two
// consequences we now control explicitly:
//
//   1. out.css (the demo+site stylesheet) NEEDS repo-wide scan — authored
//      docs/demos/*.html pages load it in iframes and their utilities are
//      only picked up that way (load-bearing, previously accidental). It
//      compiles with `--cwd .` plus `@source not` exclusions (see
//      tools/demo.mjs) so tool/source fixtures don't leak dead classes.
//   2. the PRODUCT build must emit ONLY @apply-driven rules: compile with
//      no --cwd → fresh empty scratch dir → zero content scanning.
//
//   node tools/tw.mjs <in> <out> [--minify] [--cwd DIR]
import { execFileSync } from "node:child_process"
import { mkdtempSync, rmSync } from "node:fs"
import { tmpdir } from "node:os"
import path from "node:path"
import { fileURLToPath } from "node:url"

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..")

function main() {
  const args = process.argv.slice(2)
  const minify = args.includes("--minify")
  const cwdIdx = args.indexOf("--cwd")
  const skip = new Set()
  if (cwdIdx >= 0) { skip.add(cwdIdx); skip.add(cwdIdx + 1) }
  const positional = args.filter((a, i) => !a.startsWith("--") && !skip.has(i))
  if (positional.length !== 2) {
    console.error("usage: node tools/tw.mjs <in> <out> [--minify] [--cwd DIR]")
    process.exit(1)
  }
  const [input, output] = positional.map((p) => path.resolve(ROOT, p))

  // resolve npx binary from the repo's node_modules, not the compile cwd
  const cli = path.join(ROOT, "node_modules", ".bin", "tailwindcss")
  const cmd = (dir) => [cli, "-i", input, "-o", output].concat(minify ? ["--minify"] : [])

  if (cwdIdx >= 0) {
    const dir = path.resolve(ROOT, args[cwdIdx + 1])
    execFileSync(cmd()[0], cmd().slice(1), { cwd: dir, stdio: "inherit" })
    return
  }
  const scratch = mkdtempSync(path.join(tmpdir(), "shadless-tw-"))
  try { execFileSync(cmd()[0], cmd().slice(1), { cwd: scratch, stdio: "inherit" }) }
  finally { rmSync(scratch, { recursive: true, force: true }) }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main()
