// FT2 docs: icon inlining for the ESM imports stripped from radix mdx (gate
// hit 4). Lucide icons reuse the repo-pinned lucide-react package: the SVG
// node data lives in plain source arrays (dist/esm/icons/<name>.mjs), parsed
// without importing React. @tabler/icons-react is NOT a repo dependency and
// the CDN was unreachable at build time — its single used icon
// (IconInfoCircle) is hand-inlined below with tabler's standard outline
// geometry (24x24, stroke 2, round caps — MIT, tabler/tabler-icons).
import { existsSync, readFileSync } from 'node:fs'
import { join } from 'node:path'
import { el } from './jsx.mjs'

const LUCIDE_DIR = join('node_modules', 'lucide-react', 'dist', 'esm', 'icons')

const TABLER_ICONS = {
  IconInfoCircle: [
    ['path', { d: 'M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0' }],
    ['path', { d: 'M12 8l.01 0' }],
    ['path', { d: 'M11 12h1v4h1' }],
  ],
}

const kebab = (s) => s.replace(/([a-z0-9])([A-Z])/g, '$1-$2').toLowerCase()

const lucideCache = new Map()
function lucideNode(name) {
  if (lucideCache.has(name)) return lucideCache.get(name)
  // lucide-react exports both `Info` and `InfoIcon` aliases; files are
  // kebab. Extension varies by version (0.474 ships .js, newer ship .mjs).
  const candidates = [kebab(name), kebab(name.replace(/Icon$/, ''))]
  for (const cand of candidates) {
    for (const ext of ['.mjs', '.js']) {
      const p = join(LUCIDE_DIR, cand + ext)
      if (!existsSync(p)) continue
      const m = /const __iconNode = (\[[\s\S]*?\]);/.exec(readFileSync(p, 'utf8'))
      if (!m) continue
      // Try raw JSON first; on failure quote only object-position keys. The
      // old blanket `\b(\w+):` rewrite corrupted hyphenated keys (fill-rule)
      // and colons inside path-data strings.
      let node
      try { node = JSON.parse(m[1]) }
      catch { node = JSON.parse(m[1].replace(/([{,]\s*)([A-Za-z][\w-]*)(\s*:)/g, '$1"$2"$3')) }
      lucideCache.set(name, node)
      return node
    }
  }
  return null
}

// build an icon component (MDX element factory): called by the serializer
// when an <XIcon /> survives import-stripping and resolves through
// useMDXComponents
export function iconComponent(name) {
  const node = TABLER_ICONS[name] ?? lucideNode(name)
  if (!node) throw new Error(`icon not found: ${name}`)
  const children = node.map(([tag, attrs]) =>
    el(tag, Object.fromEntries(Object.entries(attrs).filter(([k]) => k !== 'key'))))
  return (props = {}) => el('svg', {
    xmlns: 'http://www.w3.org/2000/svg',
    viewBox: '0 0 24 24',
    fill: 'none',
    stroke: 'currentColor',
    'stroke-width': '2',
    'stroke-linecap': 'round',
    'stroke-linejoin': 'round',
    // callers may override with their own aria-hidden value
    'aria-hidden': props['aria-hidden'] ?? 'true',
    ...(props.className ? { class: props.className } : {}),
  }, children)
}
