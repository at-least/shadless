// T4 converter: shadcn registry .tsx → versioned IR (src/registry/ir/*.json).
// Drift gates (exit 1 on any): babel string count == raw-text regex count;
// every IR class string appears verbatim in source; tier dist == tiers.json;
// every non-native IR tag resolves via tagHints (no silent <button> coercion).
//
// Pure helpers are exported for unit tests (tools/unit/converter.mjs); the
// pipeline + gates run under the main guard only.
import { parse } from "@babel/parser"
import { readFileSync, writeFileSync, mkdirSync, readdirSync } from "node:fs"
import { join } from "node:path"
import { pathToFileURL } from "node:url"
import { NAT, externalMemberTag, normalizeTag } from "../tags.mjs"

// Resolved tree (tools/resolve-skins.mjs): bases/radix/ui with cn-*
// expanded to nova-skin utilities — upstream generation parity. All
// downstream consumers (IR, contracts oracle, example oracle) read the
// RESOLVED tree so shipped markup never carries cn-* names.
const UI = "build/resolved-ui/ui"
const OUT = "src/registry/ir"
const PIN_FILE = "src/registry/pin.json"
const TIERS_FILE = "src/registry/tiers.json"

// tier decision tables [measured] from probes/h3,h5 (src/registry/tiers.json)
export const KERNEL = new Set(["alert-dialog", "context-menu", "dialog", "dropdown-menu",
  "hover-card", "popover", "select", "slider", "scroll-area", "sheet", "tabs", "tooltip"])
export const TRIVIAL = new Set(["accordion", "aspect-ratio", "avatar", "checkbox",
  "collapsible", "label", "progress", "radio-group", "separator", "switch",
  "toggle", "toggle-group"])
export const MEDIUM = new Set(["menubar", "navigation-menu"])
export const LOGIC = new Set(["combobox", "field", "sidebar"])
// bases/radix addition: questionnaire is a foreign-runtime wrapper
// (@shadcn/react/questionnaire) — external like react-day-picker, but the
// @shadcn/ prefix is also used for build plumbing so the filter can't key
// on it alone.
export const EXPLICIT_EXTERNAL = new Set(["questionnaire"])

export const tierOf = (name, imports) => {
  if (EXPLICIT_EXTERNAL.has(name)) return "external"
  if (LOGIC.has(name)) return "logic"
  if (MEDIUM.has(name)) return "medium"
  if (KERNEL.has(name)) return "kernel"
  if (TRIVIAL.has(name)) return "trivial-js"
  const ext = imports.filter((i) =>
    !i.startsWith("radix-ui") && !i.startsWith("@radix-ui") &&
    i !== "react" && i !== "react-dom" && i !== "lucide-react" &&
    i !== "class-variance-authority" && i !== "clsx" && i !== "tailwind-merge" &&
    !i.startsWith("@shadcn/") && !i.startsWith("@/") && !i.startsWith(".") && !i.startsWith("next/"))
  if (ext.length) return "external"
  return "static"
}

export const parseTs = (src) => parse(src, { sourceType: "module", plugins: ["typescript", "jsx"] })
export const str = (n) => n && n.type === "StringLiteral" ? n.value : null
// JSX element names: identifier or member expression (X.Y). Conditional/
// Logical names are grammatically impossible in JSX opening elements — the
// old branches for them were dead code.
export const jsxName = (n) =>
  n.type === "JSXIdentifier" ? n.name :
  n.type === "JSXMemberExpression" ? `${jsxName(n.object)}.${n.property.name}` : null

// exported names of a module (specifiers + declarations + default)
export function collectExportedNames(ast) {
  const out = new Set()
  for (const n of ast.program.body) {
    if (n.type === "ExportNamedDeclaration") {
      if (n.declaration) {
        if (n.declaration.type === "FunctionDeclaration" && n.declaration.id) out.add(n.declaration.id.name)
        if (n.declaration.type === "VariableDeclaration")
          for (const d of n.declaration.declarations) if (d.id?.name) out.add(d.id.name)
      }
      for (const s of n.specifiers || []) out.add(s.exported?.name ?? s.local.name)
    }
    if (n.type === "ExportDefaultDeclaration") {
      const d = n.declaration
      if (d?.type === "FunctionDeclaration" && d.id) out.add(d.id.name)
      else if (d?.type === "Identifier") out.add(d.name)
      else out.add("default")
    }
  }
  return out
}

// top-level cva tables of a module: { tableName: {base, variants, defaults} }
export function cvaTablesOf(ast) {
  const tables = {}
  const take = (decl) => {
    for (const d of decl.declarations || []) {
      const init = d.init
      if (!d.id?.name || !init || init.type !== "CallExpression" || !init.callee ||
          init.callee.name !== "cva") continue
      const base = str(init.arguments[0]) ?? ""
      const variants = {}, defaults = {}
      const cfg = init.arguments[1]
      // variant/default values: "lit" | ["a","b"] (join) | "" (no classes, key exists)
      const valOf = (n) => n?.type === "ArrayExpression"
        ? n.elements.map((e) => str(e) ?? "").filter(Boolean).join(" ")
        : str(n) ?? ""
      if (cfg) for (const p of cfg.properties) {
        if (p.key?.name === "variants")
          for (const vk of p.value.properties || []) {
            variants[vk.key.name ?? vk.key.value] = {}
            for (const vv of vk.value.properties || [])
              // variant value keys can be quoted ("icon-xs") — key.value
              variants[vk.key.name ?? vk.key.value][vv.key.name ?? vv.key.value] = valOf(vv.value)
          }
        if (p.key?.name === "defaultVariants")
          for (const dv of p.value.properties || [])
            defaults[dv.key.name ?? dv.key.value] = str(dv.value) ?? dv.value?.value
      }
      tables[d.id.name] = { base, variants, defaults }
    }
  }
  for (const n of ast.program.body) {
    if (n.type === "VariableDeclaration") take(n)
    if (n.type === "ExportNamedDeclaration" && n.declaration?.type === "VariableDeclaration") take(n.declaration)
  }
  return tables
}

// Resolve one cva-table call `Table({axis: value, ...})` into class strings on
// acc. Handles the bounded value shapes measured in the pinned tree:
//   "literal" | x ?? "lit" | x || "lit" | <param with literal default>
//   ident ? "then" : "else"  (attr-driven: default=else, then via data-* attr)
// Unknown literal variant values (e.g. size "icon-xs" against a table without
// that key) FAIL loudly — the old `continue` silently dropped all classes.
export function resolveCvaArgs(ctx, el, table, args, acc, ref, cross) {
  if (table.base) acc.push(table.base)
  const dyn = [], dynAxes = [], dynDefaults = {}
  const ownAttr = (name) => !!el?.openingElement?.attributes?.some((a) => a.type === "JSXAttribute" && a.name?.name === name)
  for (const [axis, vals] of Object.entries(table.variants || {})) {
    const v = args[axis]
    let val = null
    if (v) {
      if (v.type === "StringLiteral") val = v.value
      else if (v.type === "LogicalExpression") {
        if (str(v.right)) val = str(v.right)
        else if (v.right.type === "Identifier") val = ctx.paramDefaults?.[v.right.name] ?? null
        // `context.size || size` with data-size={…} on the element: the value
        // arrives through React CONTEXT at runtime and the element exposes it
        // as its own attribute. Statically this collapsed to the param default,
        // so toggle-group items had no size rules at all on the css-import
        // path. Treat it as a dynamic axis keyed on data-<axis>, with the
        // param default as the axis default (twin blocks in css.mjs).
        if (v.left?.type !== "StringLiteral" && ownAttr(`data-${axis}`) && val != null) {
          dynAxes.push(axis); dynDefaults[axis] = val
          continue
        }
      }
      else if (v.type === "Identifier") val = ctx.paramDefaults?.[v.name] ?? null
    }
    const def = table.defaults?.[axis]
    if (val != null) {
      // key must exist; "" is a legitimate no-classes variant value
      if (Object.hasOwn(vals, val)) {
        if (vals[val]) acc.push(vals[val])
        // A literal PARAM default (`size = "default"`) that feeds the axis is
        // the axis's default in every React render — attachment and marker
        // declare no cva defaultVariants and rely on this. Record it on the
        // table so the css emitter's twin :not([data-<axis>]) blocks exist:
        // without it slot-only markup got no size/orientation classes at
        // all (gates/path-parity.mjs, 12 properties on the attachment root).
        // Only the element's own root call binds the default; a cross-file
        // reference resolved elsewhere must not redefine the source table.
        const fromParam = v.type === "Identifier" || (v.type === "LogicalExpression" && v.right?.type === "Identifier")
        if (fromParam && def === undefined && !cross) (table.defaults ??= {})[axis] = val
        continue
      }
      throw new Error(`cva unknown variant value: ${axis}=${JSON.stringify(val)} (ref ${ref})`)
    }
    if (v?.type === "ConditionalExpression" && str(v.consequent) && str(v.alternate)) {
      if (vals[str(v.alternate)]) acc.push(vals[str(v.alternate)]) // falsy state = base
      const test = v.test.type === "Identifier" ? v.test.name : null
      let attr = null
      // attr scan: the element itself, then (asChild pattern) the whole fn —
      // bases/radix pagination binds data-active={isActive} on the <a> child
      // while the cva call sits on the wrapping <Button>
      const scan = []
      if (el) scan.push(el.openingElement.attributes)
      if (ctx.fnNode) {
        const seen = new Set()
        const collect = (n) => {
          if (!n || typeof n !== "object" || seen.has(n)) return
          seen.add(n)
          if (n.type === "JSXElement" && n.openingElement?.attributes) scan.push(n.openingElement.attributes)
          for (const k of Object.keys(n)) {
            if (k === "loc" || k === "start" || k === "end") continue
            const v2 = n[k]
            if (Array.isArray(v2)) v2.forEach((c) => c && typeof c.type === "string" && collect(c))
            else if (v2 && typeof v2.type === "string") collect(v2)
          }
        }
        collect(ctx.fnNode.body)
      }
      if (test)
        outer: for (const attrs of scan)
          for (const a of attrs)
            if (a.type === "JSXAttribute" && a.name?.name?.startsWith("data-") &&
                a.value?.type === "JSXExpressionContainer" &&
                a.value.expression.type === "Identifier" &&
                a.value.expression.name === test) { attr = a.name.name; break outer }
      // attr-driven consequent: without the matching data-* attr the "then"
      // classes would be silently lost (old behavior) — fail loudly instead
      if (!attr)
        throw new Error(`cva ident-ternary without data-* attr binding: ${test} (ref ${ref})`)
      if (vals[str(v.consequent)])
        dyn.push({ attr, when: "true", classes: vals[str(v.consequent)] })
      continue
    }
    // dynamic axis: merge default into base (same-rule ordering keeps any
    // override classes pushed later in el.classes winning), emit all values
    if (def && vals[def]) acc.push(vals[def])
    dynAxes.push(axis)
  }
  if (el && (dyn.length || dynAxes.length))
    ctx.recordCvaRef(el, ref, table, dyn, dynAxes, cross, dynDefaults)
}

// collect class strings from a className value node (string | cn(...) args).
// ctx is required to resolve cva calls (imported or local); a null ctx only
// collects literal strings (used for child sketches).
export function classStrings(v, acc, ctx, el) {
  if (!v) return
  if (v.type === "JSXExpressionContainer") return classStrings(v.expression, acc, ctx, el)
  if (v.type === "StringLiteral") { acc.push(v.value); return }
  if (v.type === "TemplateLiteral" && v.quasis.length === 1)
    { acc.push(v.quasis[0].value.cooked); return }
  if (v.type === "CallExpression" && v.callee.type === "Identifier") {
    if (v.callee.name === "cn") {
      for (const a of v.arguments) classStrings(a, acc, ctx, el)
      return
    }
    if (ctx) {
      const hit = ctx.lookupCva(v.callee.name)
      if (hit) {
        // Babel object properties are "ObjectProperty" (not ESTree "Property")
        const args = {}
        const a0 = v.arguments[0]
        if (a0?.type === "ObjectExpression")
          for (const p of a0.properties || [])
            if ((p.type === "ObjectProperty" || p.type === "Property") && p.key?.name)
              args[p.key.name] = p.value
        resolveCvaArgs(ctx, el, hit.table, args, acc, v.callee.name, hit.cross)
        return
      }
    }
  }
  // class-cond pattern (H1b): ternary/logical inside cn() — both branches
  // are collected as unconditional classes; the conditional record documents
  // the axis (recorded in extractFn's class-cond walk)
  if (v.type === "ConditionalExpression") {
    classStrings(v.consequent, acc, ctx, el); classStrings(v.alternate, acc, ctx, el)
  }
  if (v.type === "LogicalExpression") classStrings(v.right, acc, ctx, el)
}

export function extractAttrs(el, ctx) {
  let slot = null, classes = [], spread = false
  for (const a of el.openingElement.attributes) {
    if (a.type === "JSXSpreadAttribute") { spread = true; continue }
    if (a.name.name === "data-slot") slot = str(a.value)
    if (a.name.name === "className") classStrings(a.value, classes, ctx, el)
  }
  // component-wrap: an imported component (e.g. Button) renders with the
  // wrapped table's classes — resolve them onto this element (bounded: axis
  // props must be literal / ??lit / param-default). Works for slot overrides
  // (`<Button data-slot=…>`, ref rules recorded) AND slotless wraps
  // (`<PaginationLink size="default">` — base classes inline onto the
  // element and ship via the css.mjs anchor path).
  if (ctx && el.openingElement.name.type === "JSXIdentifier") {
    const imp = ctx.importMap.get(el.openingElement.name.name)
    const comp = imp ? ctx.compCva.get(imp) : null
    if (comp) {
      const args = {}
      for (const a of el.openingElement.attributes)
        if (a.type === "JSXAttribute" && comp.table.variants[a.name?.name])
          args[a.name.name] = a.value?.type === "JSXExpressionContainer"
            ? a.value.expression : a.value
      // ORDER is React's: the wrapped component composes cn(itsVariants(...),
      // className) — its own classes come FIRST, the wrapper's className
      // after. Appending them (the old shape) reversed the cascade once the
      // emitter started twMerge-ing lists: rounded-lg (button) beat the
      // carousel arrow's rounded-full.
      const wrapped = []
      resolveCvaArgs(ctx, el, comp.table, args, wrapped, `${comp.file}:${comp.cvaName}`, true)
      classes.unshift(...wrapped)
    }
  }
  return { slot, classes: classes.filter((c) => c !== null && c !== ""), spread }
}

export function sketchChildren(el) {
  const out = []
  for (const c of el.children) {
    if (c.type === "JSXText") { if (c.value.trim()) out.push("text"); continue }
    if (c.type === "JSXExpressionContainer") {
      const e = c.expression
      if (e.type === "Identifier") out.push(`{${e.name}}`)
      else if (e.type === "LogicalExpression" || e.type === "ConditionalExpression")
        out.push("OPT?")
      else out.push("expr")
      continue
    }
    if (c.type === "JSXElement") {
      const n = jsxName(c.openingElement.name)
      const a = extractAttrs(c, null)
      out.push(`<${n}${a.slot ? ` slot=${a.slot}` : ""}${a.classes.length ? ` class=[${a.classes.length}]` : ""}>`)
    }
  }
  return out
}

// walk every node
export function walk(node, fn) {
  fn(node)
  for (const k of Object.keys(node)) {
    const v = node[k]
    if (Array.isArray(v)) v.forEach((c) => c && typeof c.type === "string" && walk(c, fn))
    else if (v && typeof v.type === "string") walk(v, fn)
  }
}

// fn-local tag variables: `const Comp = asChild ? Slot.Root : "div"` and
// `const Tag = "span"` — the native string is what the no-React emitter
// renders (the Slot branch is React-only).
export function tagVarsOf(fnNode) {
  const out = {}
  walk(fnNode, (n) => {
    if (n.type !== "VariableDeclaration" || !n.declarations) return
    for (const d of n.declarations) {
      if (!d.id?.name || !d.init) continue
      let t = null
      if (d.init.type === "StringLiteral") t = d.init.value
      else if (d.init.type === "ConditionalExpression" &&
               d.init.alternate.type === "StringLiteral") t = d.init.alternate.value
      if (t) {
        if (out[d.id.name] && out[d.id.name] !== t)
          throw new Error(`conflicting tag var ${d.id.name}: ${out[d.id.name]} vs ${t}`)
        out[d.id.name] = t
      }
    }
  })
  return out
}

// lucide icon component names render as <svg> in the no-React emit
// (ChevronLeft/ChevronUp/MoreVertical/Circle/Loader were listed but used by
// no registry or example file at shadcn@4.19.0 — gates/overlay.mjs reports a
// rule as dissolved when nothing upstream needs it; names ending in Icon
// resolve via the /Icon$/ fallback regardless)
export const KNOWN_ICONS = new Set(["ChevronRight", "ChevronDown",
  "MoreHorizontal", "Check", "X", "Plus", "Minus", "Search"])
const isIconName = (t, icons) => icons.includes(t) || KNOWN_ICONS.has(t) || /Icon$/.test(t)

export function convertFile(name, src, REG) {
  const ast = parseTs(src)

  // imports + local→imported specifier map (for cross-file cva resolution)
  const imports = []
  const importMap = new Map()
  const moduleOf = new Map() // local name → module path (tags + cva)
  let icons = []
  for (const n of ast.program.body) {
    if (n.type !== "ImportDeclaration") continue
    const from = n.source.value
    imports.push(from)
    if (from === "lucide-react")
      icons = n.specifiers.map((s) => s.imported ? s.imported.name : s.local.name)
    for (const s of n.specifiers || [])
      if (s.type === "ImportSpecifier" && s.imported?.name) {
        importMap.set(s.local.name, s.imported.name)
        moduleOf.set(s.local.name, from)
      }
  }

  // cva tables (top-level, incl. exported consts)
  const cva = cvaTablesOf(ast)

  // components: exported names collected honestly (export { X } specifiers,
  // export declarations, export default).
  const exported = collectExportedNames(ast)

  // per-file cva-call resolution context
  const cvaRefs = []
  const refKeys = new Set()
  const fileCtx = {
    name,
    importMap,
    compCva: REG.compCva,
    cvaRefs,
    lookupCva: (localName) => {
      // imported cross-file table (e.g. toggle-group using toggle's) wins;
      // fall back to this file's own tables (call-in-className inlining)
      const imported = importMap.get(localName)
      if (imported && REG.cvaByExport.has(imported))
        return { table: REG.cvaByExport.get(imported).table, cross: true }
      return cva[localName] ? { table: cva[localName], cross: false } : null
    },
    // Ref rules are recorded only for cross-file refs — a file's own tables
    // already emit attribute rules via css.mjs's cvaSlot path, so inlining
    // their call-site classes into el.classes is enough (CSS-inert: those
    // slots sit inside cvaSlots and the plain-class path skips them).
    recordCvaRef: (el, ref, table, dyn, dynAxes, cross, defaults = {}) => {
      if (!cross) return
      const slot = el.openingElement.attributes
        .filter((a) => a.type === "JSXAttribute" && a.name?.name === "data-slot")
        .map((a) => str(a.value))[0] ?? null
      if (!slot) return
      const key = `${slot}|${ref}|${dyn.map((d) => d.attr + d.when).join(",")}|${dynAxes.join(",")}`
      if (refKeys.has(key)) return
      refKeys.add(key)
      cvaRefs.push({ slot, ref, table, dyn, dynAxes, defaults })
    },
  }

  // collect: top-level function declarations (any export state) + exported
  // const arrows.
  const components = []
  const conditionals = []
  const tagVars = {} // merged per file (conflicts throw in tagVarsOf)
  const pushFn = (id, fnNode) => {
    if (!id || !fnNode?.body) return
    Object.assign(tagVars, tagVarsOf(fnNode))
    components.push(extractFn(id, fnNode, src, conditionals, exported.has(id), fileCtx))
  }
  for (const n of ast.program.body) {
    if (n.type === "FunctionDeclaration") { pushFn(n.id?.name, n); continue }
    if (n.type === "ExportDefaultDeclaration") {
      const d = n.declaration
      if (d?.type === "FunctionDeclaration") pushFn(d.id?.name ?? "default", d)
      else if (d?.type === "ArrowFunctionExpression" || d?.type === "FunctionExpression")
        pushFn("default", d)
      continue
    }
    if (n.type === "ExportNamedDeclaration") {
      const d = n.declaration
      if (d?.type === "FunctionDeclaration") { pushFn(d.id?.name, d); continue }
      if (d?.type === "VariableDeclaration") {
        for (const dd of d.declarations) {
          if (!dd.id?.name) continue
          const init = dd.init
          if (init?.type === "ArrowFunctionExpression" || init?.type === "FunctionExpression")
            pushFn(dd.id.name, init)
        }
      }
    }
  }

  const tier = tierOf(name, imports)
  // same-file slotless wrap: fn root renders another same-file fn (e.g.
  // PaginationPrevious → PaginationLink) and sets no data-slot — the wrapped
  // fn's root classes (cva-resolved) must ride along, or the emitted anchor
  // rule ships without the base styles. Cross-file wraps were already
  // resolved in extractAttrs via compCva; slotted roots keep that path.
  {
    const byFn = new Map(components.map((c) => [c.fn, c]))
    const effRoot = (fnName, stack = new Set()) => {
      const c = byFn.get(fnName)
      const root = c?.elements?.[0]
      if (!root || stack.has(fnName)) return []
      if (root.slot) return root.classes // slotted base case: classes live here
      stack.add(fnName)
      const wrapped = root.tag && byFn.has(root.tag) ? effRoot(root.tag, stack) : []
      return [...wrapped, ...root.classes]
    }
    for (const c of components) {
      const root = c.elements[0]
      if (root && !root.slot && root.tag && byFn.has(root.tag) && root.tag !== c.fn)
        root.classes = [...new Set(effRoot(c.fn))].filter(Boolean)
    }
  }
  const ir = { schema: 2, source: { commit: REG.pinCommit ?? null }, name, tier,
    imports, icons, cva, components, conditionals, cvaRefs, tagHints: {} }
  ir.__meta = { moduleOf, tagVars, importMap } // stripped before serialization
  return ir
}

function extractFn(fnName, fnNode, src, conditionals, isExport, fileCtx) {
  const elements = []
  // literal-default params feed cva-call resolution (e.g. size = "icon-xs").
  // Defaults live inside destructuring ObjectPatterns ({ size = "icon" }),
  // not only as top-level AssignmentPattern params.
  const paramDefaults = {}
  for (const p of fnNode.params || []) {
    const items = p?.type === "ObjectPattern" ? p.properties : [p]
    for (const q of items) {
      // { size = "icon" } shorthand: ObjectProperty{value: AssignmentPattern}
      const ap = q?.type === "AssignmentPattern" ? q
        : (q?.type === "ObjectProperty" && q.value?.type === "AssignmentPattern") ? q.value : null
      if (ap && ap.left?.type === "Identifier" && str(ap.right))
        paramDefaults[ap.left.name] = str(ap.right)
    }
  }
  const ctx = { ...fileCtx, fn: fnName, paramDefaults, fnNode }
  walk(fnNode.body, (node) => {
    if (node.type !== "JSXElement") return
    const tag = jsxName(node.openingElement.name)
    const { slot, classes, spread } = extractAttrs(node, ctx)
    elements.push({ tag, slot, classes, spread, children: sketchChildren(node) })
    // child-cond
    for (const c of node.children)
      if (c.type === "JSXExpressionContainer" &&
          (c.expression.type === "LogicalExpression" || c.expression.type === "ConditionalExpression"))
        conditionals.push({ kind: "child-cond", fn: fnName, parent: tag })
    // class-cond inside className cn() args
    for (const a of node.openingElement.attributes) {
      if (a.name?.name !== "className" || !a.value) continue
      const v = a.value.expression ? a.value.expression : a.value
      const detect = (e) => {
        if (!e) return
        if (e.type === "ConditionalExpression") {
          // the predicate, when it is `ident === "literal"` / `!==`: the css
          // emitter turns such branches into attribute-keyed twin rules.
          // `default` is the literal default of that ident anywhere in the
          // file (carousel: orientation = "horizontal" lives on the ROOT fn
          // and reaches the item via context) — absent attribute ⇒ default.
          let test
          const t = e.test
          if (t?.type === "BinaryExpression" && (t.operator === "===" || t.operator === "!==") &&
              t.left?.type === "Identifier" && str(t.right) !== null) {
            const m = new RegExp(`\\b${t.left.name}\\s*=\\s*"([^"]+)"`).exec(src)
            test = { name: t.left.name, op: t.operator, value: str(t.right), ...(m ? { default: m[1] } : {}) }
          }
          conditionals.push({ kind: "class-cond", fn: fnName, slot,
            then: str(e.consequent) ?? "", else: str(e.alternate) ?? "", ...(test ? { test } : {}) })
        }
        if (e.type === "LogicalExpression") detect(e.right)
        if (e.type === "CallExpression") e.arguments.forEach(detect)
      }
      detect(v)
    }
  })
  return { fn: fnName, export: !!isExport, elements }
}

// ---------- tagHints -----------------------------------------------------------
// Build ir.tagHints: rawTag → native tag, resolving fn-local tag vars, lucide
// icons, same-file component references, cross-file registry component roots
// (import module's last path segment names the component file), and the
// external member-suffix rule. Unresolved tags throw — the emitter used to
// silently coerce them to <button>.
export function buildTagHints(irs) {
  const byName = new Map(irs.map((ir) => [ir.name, ir]))
  const fnRoot = new Map() // "file:fn" → raw root tag
  for (const ir of irs) for (const c of ir.components)
    if (c.elements[0]) fnRoot.set(`${ir.name}:${c.fn}`, c.elements[0].tag)

  // fn root tag → native, following same-file/cross-file component refs
  const resolveRoot = (file, fn, seen = new Set()) => {
    const key = `${file}:${fn}`
    if (seen.has(key)) return null // cycle guard
    const raw = fnRoot.get(key)
    if (raw == null) return null
    if (NAT.has(raw)) return raw
    const m = /^<ternary:([^/]+)\/(.+)>$/.exec(raw)
    if (m) return NAT.has(m[2]) ? m[2] : null
    // root references another component → same-file fn, else imported
    const target = byName.get(file)
    if (!target) return null
    if (target.components.some((c) => c.fn === raw))
      return resolveRoot(file, raw, new Set([...seen, key]))
    return hintFor(target, raw, new Set([...seen, key]))
  }

  const hintFor = (ir, tag, seen = new Set()) => {
    if (NAT.has(tag)) return tag
    const m = /^<ternary:([^/]+)\/(.+)>$/.exec(tag)
    if (m) return NAT.has(m[2]) ? m[2] : null
    if (isIconName(tag, ir.icons || [])) return "svg"
    // app-side icon helper (@/app/... import, renders an svg placeholder)
    const modOf = ir.__meta?.moduleOf?.get(tag)
    if (modOf?.startsWith("@/app/")) return "svg"
    if (ir.__meta?.tagVars?.[tag]) return ir.__meta.tagVars[tag]
    // same-file component
    const localFn = ir.components.find((c) => c.fn === tag)
    if (localFn) return resolveRoot(ir.name, tag, seen)
    // imported component from another registry file (module stem = file name)
    const mod = ir.__meta?.moduleOf.get(tag)
    if (mod) {
      const stem = mod.split("/").filter(Boolean).pop().replace(/\.[tj]sx?$/, "")
      const dep = byName.get(stem)
      if (dep) {
        // registry files export the component under the imported name
        const importedName = ir.__meta.importMap.get(tag) ?? tag
        if (!dep.components.some((c) => c.fn === importedName)) return null
        return resolveRoot(stem, importedName, seen)
      }
      // external package member tag (MessageScrollerPrimitive.Viewport)
      if (tag.includes(".")) return externalMemberTag(tag)
      return null
    }
    if (tag.includes(".")) return externalMemberTag(tag)
    return null
  }

  const unresolved = []
  for (const ir of irs) {
    const hints = {}
    for (const c of ir.components)
      for (const el of c.elements) {
        const t = el.tag
        if (t == null || NAT.has(t) || /^<ternary:/.test(t)) continue
        if (!(t in hints)) {
          const h = hintFor(ir, t)
          if (h == null) {
            // external-tier files are tombstones (react-day-picker etc.) —
            // their framework tags are unresolvable by design, only flag
            // tiers the emitter actually renders
            if (ir.tier !== "external") unresolved.push(`${ir.name}:${c.fn}: ${t}`)
          }
          else hints[t] = h
        }
      }
    ir.tagHints = hints
  }
  if (unresolved.length)
    throw new Error(`tagHints unresolved (${unresolved.length}):\n  ${unresolved.join("\n  ")}`)
}

// ---------- main (pipeline + drift gates) --------------------------------------
function main() {
  const PIN = JSON.parse(readFileSync(PIN_FILE, "utf8"))
  const TIERS = JSON.parse(readFileSync(TIERS_FILE, "utf8"))
  mkdirSync(OUT, { recursive: true })

  const files = readdirSync(UI).filter((f) => f.endsWith(".tsx")).sort()
  if (files.length !== 61) {
    console.error(`FAIL expected 61 files, found ${files.length}`); process.exit(1)
  }
  // global cva variant keys (cross-file: pagination uses button's variants) +
  // cross-file cva registry: exported cva tables and their convention-named
  // components (buttonVariants ↔ Button), so consumers in other files
  // (pagination.tsx, toggle-group.tsx, attachment.tsx) can be resolved.
  const GLOBAL_KEYS = new Set()
  const REG = { cvaByExport: new Map(), compCva: new Map(), pinCommit: PIN.shadcn_ui.commit }
  const ALLSRC = []
  for (const f of files) {
    const s2 = readFileSync(join(UI, f), "utf8")
    ALLSRC.push(s2)
    const a2 = parseTs(s2)
    walk(a2, (n) => {
      if (n.type !== "CallExpression" || n.callee?.name !== "cva") return
      const cfg = n.arguments[1]
      if (!cfg) return
      for (const p of cfg.properties || []) {
        if (p.key?.name !== "variants") continue
        for (const vk of p.value.properties || [])
          for (const vv of vk.value.properties || [])
            if (vv.key) GLOBAL_KEYS.add(vv.key.name || vv.key.value)
      }
    })
    const nm = f.replace(/\.tsx$/, "")
    const exported = collectExportedNames(a2)
    for (const [cvaName, table] of Object.entries(cvaTablesOf(a2))) {
      if (!exported.has(cvaName)) continue
      REG.cvaByExport.set(cvaName, { file: nm, table })
      const stem = cvaName.replace(/Variants$/, "")
      const compName = stem[0].toUpperCase() + stem.slice(1)
      if (exported.has(compName)) REG.compCva.set(compName, { cvaName, table, file: nm })
    }
  }
  const ALLSRC_JOINED = ALLSRC.join("\n")
  const dist = {}, summary = []
  let fail = false
  const irs = []
  for (const f of files) {
    const name = f.replace(/\.tsx$/, "")
    const src = readFileSync(join(UI, f), "utf8")

    // gate 1: babel StringLiteral count == raw-text double-quote regex count
    const ast = parseTs(src)
    let babelCount = 0
    walk(ast, (n) => { if (n.type === "StringLiteral" || n.type === "DirectiveLiteral") babelCount++ })
    const stripped = src
      .replace(/\/\*[\s\S]*?\*\//g, "")
      .replace(/(^|[^:])\/\/[^\n]*/g, "$1")
    const rawCount = (stripped.match(/"(?:[^"\\]|\\.)*"/g) || []).length
    if (babelCount !== rawCount) {
      console.error(`FAIL drift[${name}]: babel strings ${babelCount} != raw grep ${rawCount}`)
      fail = true
    }

    let ir
    try {
      ir = convertFile(name, src, REG)
      irs.push(ir)
    } catch (e) {
      console.error(`FAIL convert[${name}]: ${e.message}`)
      fail = true
      continue
    }

    // gate 2: every IR class string appears verbatim in source. cva variant
    // values may be arrays joined across source lines — for those, fall back
    // to verbatim-per-token (each atomic class token must appear; an invented
    // token still fails)
    const verbatim = (s) => src.includes(s) || ALLSRC_JOINED.includes(s)
    const allClasses = []
    for (const c of ir.components) for (const el of c.elements) allClasses.push(...el.classes)
    for (const t of Object.values(ir.cva)) {
      if (t.base) allClasses.push(t.base)
      for (const vv of Object.values(t.variants)) for (const s of Object.values(vv)) if (s) allClasses.push(s)
    }
    for (const c of allClasses) {
      const ok = verbatim(c) || c.split(/\s+/).every((tok) => tok && verbatim(tok))
      if (!ok) {
        console.error(`FAIL drift[${name}]: class string not in source: ${JSON.stringify(c)}`)
        fail = true
      }
    }

    // gate 2b (completeness, independent scanner): every quoted string inside a
    // cn(...) call in the raw source must be recorded in IR classes
    // (external-tier files are tombstones — their framework-specific cn calls
    // (e.g. react-day-picker classNames maps) are out of scope)
    if (ir.tier !== "external") {
      const irStrings = new Set(allClasses)
      // literals used in === / !== comparisons inside cn args are not classes
      const cmpLits = new Set()
      for (const m of stripped.matchAll(/[=!]==?\s*"((?:[^"\\]|\\.)*)"/g)) cmpLits.add(m[1])
      // cva variant keys referenced in ternaries inside cn() are not classes
      for (const t of Object.values(ir.cva)) {
        for (const vals of Object.values(t.variants)) Object.keys(vals).forEach((k) => cmpLits.add(k))
        Object.values(t.defaults).forEach((v) => cmpLits.add(v))
      }
      GLOBAL_KEYS.forEach((k) => cmpLits.add(k))
      for (const m of stripped.matchAll(/\bcn\s*\(/g)) {
        let i = m.index + m[0].length, depth = 1
        while (i < stripped.length && depth > 0) {
          if (stripped[i] === "(") depth++
          else if (stripped[i] === ")") depth--
          i++
        }
        const inner = stripped.slice(m.index + m[0].length, i - 1)
        for (const s of inner.matchAll(/"((?:[^"\\]|\\.)*)"/g)) {
          const val = s[1]
          if (val && !cmpLits.has(val) && !irStrings.has(val)) {
            console.error(`FAIL drift[${name}]: cn string not in IR: ${JSON.stringify(val)}`)
            fail = true
          }
        }
      }
    } // tier !== external

    // gate 3: tier matches tiers.json
    const want = TIERS[name]?.tier
    if (want !== ir.tier) {
      console.error(`FAIL tier[${name}]: ir=${ir.tier} want=${want}`)
      fail = true
    }
    dist[ir.tier] = (dist[ir.tier] || 0) + 1
    summary.push({ name, tier: ir.tier, classes: allClasses.length, cond: ir.conditionals.length })
  }

  // cross-file tag hints (needs all IRs; throws on unresolved → loud fail)
  try { buildTagHints(irs) }
  catch (e) { console.error(`FAIL tagHints: ${e.message}`); fail = true }

  if (!fail) for (const ir of irs) {
    const { __meta, ...out } = ir
    writeFileSync(join(OUT, `${ir.name}.json`), JSON.stringify(out, null, 1))
  }
  const wantDist = {}
  for (const v of Object.values(TIERS)) wantDist[v.tier] = (wantDist[v.tier] || 0) + 1
  if (JSON.stringify(Object.entries(dist).sort()) !== JSON.stringify(Object.entries(wantDist).sort())) {
    console.error(`FAIL tier distribution: ${JSON.stringify(dist)} != ${JSON.stringify(wantDist)}`)
    fail = true
  }
  console.log(`convert: ${files.length} IR files -> ${OUT}`)
  console.log(`tier dist: ${JSON.stringify(dist)}`)
  console.log(`conditionals total: ${summary.reduce((a, s) => a + s.cond, 0)}`)
  if (fail) { console.log("FAIL  convert drift gates"); process.exit(1) }
  console.log("PASS  convert (0 drift, tiers match, tagHints resolved)")
}

const isMain = import.meta.url === pathToFileURL(process.argv[1] ?? "x").href
if (isMain) main()
