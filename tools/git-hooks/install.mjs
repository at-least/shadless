#!/usr/bin/env node
// Installs (or removes) the shadless git hooks into .git/hooks/.
//   node tools/git-hooks/install.mjs               install pre-commit + pre-push
//   node tools/git-hooks/install.mjs --uninstall   remove them
// Refuses to overwrite a hook that is not ours (no marker) unless --force.
import { copyFileSync, chmodSync, unlinkSync, existsSync, readFileSync } from "node:fs"
import { join } from "node:path"

const HOOKS = ["pre-commit", "pre-push"]
const MARKER = "shadless"
const argv = process.argv.slice(2)
const uninstall = argv.includes("--uninstall")
const force = argv.includes("--force")

for (const name of HOOKS) {
  const src = join("tools", "git-hooks", name)
  const dst = join(".git", "hooks", name)
  if (uninstall) {
    if (!existsSync(dst)) continue
    if (!readFileSync(dst, "utf8").includes(MARKER) && !force) { console.error(`hooks: ${dst} is not a shadless hook — pass --force`); process.exit(1) }
    unlinkSync(dst); console.log(`hooks: removed ${dst}`); continue
  }
  if (existsSync(dst) && !readFileSync(dst, "utf8").includes(MARKER) && !force) {
    console.error(`hooks: ${dst} exists and is not a shadless hook — inspect it, then --force`); process.exit(1)
  }
  copyFileSync(src, dst); chmodSync(dst, 0o755)
  console.log(`hooks: installed ${dst}`)
}
if (!uninstall) console.log(`\n  pre-commit → gates fast tier (<1s)   pre-push → medium tier (~10s)\n  hand-edits to generated files are caught by 'make reproducible' (CI), not here`)
