// Upstream skin ingestion (bases/radix migration).
//
// Upstream split the registry into structure (bases/radix/ui/*.tsx, named
// cn-* classes) + skins (registry/styles/style-<name>.css defining each
// cn-* via @apply under a `.style-<name>` scope). shadless ships the nova
// skin, and resolves cn-* at the SOURCE (pipeline/resolve_skins.go —
// upstream generation parity): IR, fixture markup and demo DOM all carry
// plain utility classes. This module is the map that resolver consumes;
// it emits no CSS itself (the old @utility injection + anti-tree-shake
// sentinel were the downstream patches this replaces).
//
// The parse is strictly mechanical and validated: every block in the
// source skin must be a flat pure-@apply body (no nested selectors, no
// bare declarations) — anything else fails the build loudly.
import { readFileSync } from "node:fs"

export const SKIN_PATH = ".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css"

/** cn-X → its @apply body (the same map upstream's generator builds via
 * packages/shadcn/src/styles/create-style-map.ts).
 * @type {Record<string, string>} */
export const SKIN_MAP = {}

// Upstream transform-style-map.ts ALLOWLIST: cn-* names that stay
// unresolved (CSS-selector hooks / install-time handling). Verified
// against the live site: only .cn-font-heading has a CSS rule there;
// the rest are inert markers resolved when the CLI installs into a
// user project.
export const SKIN_ALLOWLIST = new Set([
  "cn-menu-target",
  "cn-menu-translucent",
  "cn-rtl-flip",
  "cn-font-heading",
])

/** @param {string} css @returns {void} */
export function parseSkinMap(css) {
  const start = css.indexOf("{")
  const end = css.lastIndexOf("}")
  if (start === -1 || end === -1 || !/^\s*\.style-nova\s*\{/.test(css))
    throw new Error("skin: expected a single top-level .style-nova block")
  const body = css.slice(start + 1, end)
  const blocks = body.match(/\.([\w-]+)\s*\{[^{}]*\}/g) ?? []
  for (const b of blocks) {
    const m = /^\.([\w-]+)\s*\{([^{}]*)\}$/.exec(b)
    if (!m) throw new Error(`skin: unparsable block: ${b.slice(0, 60)}`)
    const [, name, inner] = m
    const decls = inner.trim()
    if (!/^@apply [^;]+;$/.test(decls))
      throw new Error(`skin: cn-${name} is not a flat pure-@apply block: ${decls.slice(0, 60)}`)
    SKIN_MAP[name] = decls.replace(/^@apply /, "").replace(/;$/, "")
  }
  if (!Object.keys(SKIN_MAP).length) throw new Error("skin: no cn-* blocks found")
}

parseSkinMap(readFileSync(SKIN_PATH, "utf8"))
