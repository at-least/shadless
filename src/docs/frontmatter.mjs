// FT2 docs: frontmatter (yaml-lite) parsing + ESM-import stripping for the
// mirrored radix mdx. Imports appear in exactly 3 files (native-select/
// sidebar/input-group — gate hit 4); after stripping, the imported
// identifiers fall out of module scope and MDX resolves them through
// useMDXComponents, where the icon components render as inline <svg>.
export function parseFrontmatter(src) {
  // tolerate CRLF — /^---\n/ silently dropped the whole frontmatter (and
  // the title fell back to the filename) on CRLF files
  const m = /^---\r?\n([\s\S]*?)\r?\n---/.exec(src)
  if (!m) return {}
  const coerce = (v) =>
    v === 'true' ? true : v === 'false' ? false : /^-?\d+$/.test(v) ? Number(v)
      // strip quotes only when they are a matched pair around the value
      // (`years'` used to lose its apostrophe)
      : (v.length >= 2 && v[0] === v[v.length - 1] && /^["']$/.test(v[0]))
        ? v.slice(1, -1) : v
  const out = {}
  let cur = null
  for (const line of m[1].split(/\r?\n/)) {
    if (!line.trim() || line.trim().startsWith('#')) continue
    const top = /^([A-Za-z][\w-]*):\s*(.*)$/.exec(line)
    if (top && !/^\s/.test(line)) {
      if (top[2] === '') { out[top[1]] = {}; cur = out[top[1]] }
      else { out[top[1]] = coerce(top[2]); cur = null }
      continue
    }
    const sub = /^\s+([A-Za-z][\w-]*):\s*(.*)$/.exec(line)
    if (sub && cur) cur[sub[1]] = coerce(sub[2])
  }
  return out
}

export function stripImports(src) {
  return src
    // `import { ... } from "pkg";` / `import x from` / `import * as x from`.
    // The old `import[\s\S]*?from` started matching at a BARE import and
    // lazily swallowed the whole body up to the next from-import — now the
    // import clause must be braces/identifier/namespace, never bare.
    .replace(/^[ \t]*import[ \t]*(?:\{[\s\S]*?\}[ \t]*|\*(?:[ \t]+as[ \t]+[\w$]+)[ \t]*|[\w$]+[ \t]*)from[ \t]*["'][^"']+["'];?[ \t]*$/gm, '')
    // bare `import "pkg";` (line-anchored — cannot swallow prose)
    .replace(/^[ \t]*import[ \t]*["'][^"']+["'];?[ \t]*$/gm, '')
}
