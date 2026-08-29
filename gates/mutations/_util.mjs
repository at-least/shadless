// Helpers for mutation definitions. Every helper ASSERTS it changed
// something: a mutation that silently no-ops would make its gate look
// vacuously green in the meta-gate report — the exact failure mode the
// meta-gate exists to catch, reproduced one level up.
import { readFileSync, writeFileSync, existsSync, readdirSync } from "node:fs"

export function edit(path, fn) {
  if (!existsSync(path)) throw new Error(`mutation target missing: ${path} (build first)`)
  const before = readFileSync(path, "utf8")
  const after = fn(before)
  if (after === before) throw new Error(`mutation no-op on ${path} — the anchor text is gone; fix the mutation`)
  writeFileSync(path, after)
}

// Replace an exact anchor once, asserting it was present.
export function replaceOnce(path, find, replace) {
  edit(path, (s) => {
    if (!s.includes(find)) throw new Error(`anchor not found in ${path}: ${find.slice(0, 60)}`)
    return s.replace(find, replace)
  })
}

// First file under `dir` (recursively) whose content matches `pred`.
export function findFile(dir, pred, exts = [".html"]) {
  const out = []
  const walk = (d) => {
    for (const e of readdirSync(d, { withFileTypes: true })) {
      const p = `${d}/${e.name}`
      if (e.isDirectory()) walk(p)
      else if (exts.some((x) => e.name.endsWith(x))) out.push(p)
    }
  }
  walk(dir)
  for (const p of out.sort()) {
    const s = readFileSync(p, "utf8")
    if (pred(s, p)) return p
  }
  throw new Error(`no file under ${dir} matched the mutation's predicate`)
}
