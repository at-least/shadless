// FT2 docs: vanilla JSX shim + HTML serializer (productionized F1 spike —
// zero React). MDX passes every element through jsx()/jsxs() and expects the
// renderer to CALL function-typed elements (React semantics); fragments are
// transparent; className is renamed to class at serialization time.
export const Fragment = Symbol.for('shadless.fragment')

export const jsx = (type, props, key) => ({
  __el: true, type, props: key === undefined ? props : { ...props, key },
})
export const jsxs = jsx

const esc = (s) =>
  String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;')
    .replace(/>/g, '&gt;').replace(/"/g, '&quot;')

const VOID = new Set(['br', 'hr', 'img', 'input', 'meta', 'link', 'area', 'base', 'col', 'embed', 'source', 'track', 'wbr'])

// MDX (hast-util-to-style-object) delivers the style prop as an object
const styleStr = (o) =>
  Object.entries(o).map(([k, v]) => `${k.replace(/[A-Z]/g, (m) => '-' + m.toLowerCase())}:${v}`).join(';')

export const el = (tag, props = {}, children = []) => jsx(tag, { ...props, children }, undefined)

export function serialize(node) {
  if (node === null || node === undefined || node === false || node === true) return ''
  if (typeof node === 'string' || typeof node === 'number') return esc(node)
  if (Array.isArray(node)) return node.map(serialize).join('')
  if (typeof node === 'object' && node.__el) {
    const { type, props = {} } = node
    if (typeof type === 'function') return serialize(type(props))
    const kids = serialize(props.children)
    if (typeof type === 'symbol') return kids
    const attrs = Object.entries(props)
      .filter(([k, v]) => k !== 'children' && k !== 'key' && v !== undefined && v !== null && v !== false)
      .map(([k, v]) => {
        if (k === 'className') k = 'class'
        // boolean-true props render as bare attributes (open, hidden, …) —
        // dropping them silently changed <details open> into a closed one
        if (v === true) return ` ${k}`
        if (v === '') return ` ${k}`
        if (k === 'style' && v && typeof v === 'object') v = styleStr(v)
        return ` ${k}="${esc(v)}"`
      })
      .join('')
    if (VOID.has(type)) return `<${type}${attrs}>`
    return `<${type}${attrs}>${kids}</${type}>`
  }
  throw new Error('unserializable node: ' + JSON.stringify(node))
}
