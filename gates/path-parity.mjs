#!/usr/bin/env node
// gates/path-parity.mjs — every consume path computes what React computes,
// for every slot, in both themes and both directions.
//
// variant-parity proved the css-import path for cva variants. This is the
// general form: for EVERY slotted element the registry emits, slot-only
// markup styled by (a) the consumer build (core + that component's css)
// and (b) the no-build shadless.full.css must compute the same resting
// styles as the element React renders — the resolved inline class list —
// styled by upstream's own stylesheet (build/gates/oracle.css). Neither
// reference side is touched by src/emitter, so an emitter bug cannot hide
// on both sides at once.
//
// Cells are (component / slot#n / property @ path @ theme @ dir). Recorded
// drift lives in gates/path-parity-baseline.json under the same ratchet as
// style-parity: identity-pinned, may only shrink, shrink must be recorded.
//
//   node gates/path-parity.mjs            the gate
//   node gates/path-parity.mjs --record   re-record the baseline
//   node gates/path-parity.mjs --details  print every light/ltr css-import cell with values
//   PP_KEEP=1                             keep the scratch build
import { chromium } from "playwright"
import { execFileSync } from "node:child_process"
import { readFileSync, writeFileSync, mkdirSync, rmSync, symlinkSync, readdirSync, existsSync } from "node:fs"
import { resolve } from "node:path"
import { cvaSlot, splitMarkers } from "../src/emitter/css.mjs"
import { normalizeTag } from "../src/tags.mjs"
import { twMerge } from "tailwind-merge"

const ROOT = resolve(".")
const SIM = "build/gates/path-parity"
const BASELINE = "gates/path-parity-baseline.json"
const RECORD = process.argv.includes("--record")

const PROPS = [
  "color", "background-color", "border-color", "border-top-width", "border-bottom-width",
  "border-left-width", "border-right-width", "border-radius", "padding-top", "padding-right",
  "padding-bottom", "padding-left", "margin-top", "margin-left", "margin-right", "width",
  "min-width", "max-width", "height", "min-height", "row-gap", "column-gap", "font-size",
  "font-weight", "line-height", "letter-spacing", "text-align", "display", "flex-direction",
  "align-items", "justify-content", "position", "top", "left", "right", "opacity", "box-shadow",
  "outline-width", "overflow", "white-space", "text-decoration-line", "transform", "translate", "scale", "visibility",
]
const VOID = new Set(["input", "img", "br", "hr"])
const MATRIX = [["light", "ltr"], ["dark", "ltr"], ["light", "rtl"], ["dark", "rtl"]]

// child slots referenced by has-/*:/data-[slot=…] variants — synthesized so
// the root's own has-* utilities activate identically on both sides
const childSlots = (cls) => [...new Set([...cls.matchAll(/(?<!in-)data-\[slot=([-\w]+)\]/g)].map((m) => m[1]))]
// React evaluates a conditional; the converter records both branches plus
// the predicate — keep the branch the default selects, like the runtime does
const inlineClasses = (ir, fn, el) => {
  const conds = (ir.conditionals ?? []).filter((c) => c.kind === "class-cond" && c.fn === fn && c.test &&
    el.classes.includes(c.then) && el.classes.includes(c.else))
  const drop = new Set()
  for (const c of conds) {
    const truthy = c.test.default === undefined ? true : (c.test.default === c.test.value) === (c.test.op === "===")
    drop.add(truthy ? c.else : c.then)
  }
  // React runs cn() = twMerge over the whole list: a wrapped Button's
  // rounded-[…] loses to the wrapper's later rounded-full. Without the merge
  // the oracle side let Tailwind's internal order pick (the arbitrary radius)
  return twMerge(el.classes.filter((c) => !drop.has(c)).join(" "))
}

// STATE renders. The resting element is one cell; every attribute-driven
// state the class list styles is another: data-open: (= [data-state=open]
// per upstream's custom variants), data-[side=left]:, aria-expanded:, …
// Both sides receive the same attribute, so the oracle's inline variants
// and our slot-rule variants must fire identically. One attribute per
// render — combinations are the runtime's job (contracts).
const SHORTHAND = { // upstream @custom-variant bodies (packages/shadcn/src/tailwind.css)
  "data-open": ["data-state", "open"], "data-closed": ["data-state", "closed"],
  "data-checked": ["data-state", "checked"], "data-unchecked": ["data-state", "unchecked"],
  "data-active": ["data-state", "active"], "data-selected": ["data-selected", "true"],
  "data-disabled": ["data-disabled", "true"], "data-horizontal": ["data-orientation", "horizontal"],
  "data-vertical": ["data-orientation", "vertical"],
}
const CVA_AXES = new Set(["variant", "size"]) // variant-parity's territory
const stateConfigs = (cls) => {
  const out = new Map() // "attr=value" -> [attr, value]
  for (const tok of cls.split(/\s+/)) {
    const segs = tok.split(":").slice(0, -1)
    // a child combinator (*:, **:) or has-/group-/peer-/in- earlier in the
    // chain means the attribute lives on ANOTHER element — not a state of
    // this one; data-[slot=…] is never a state
    if (segs.some((v) => v === "*" || v === "**" || /^(group|peer|has|in)-/.test(v))) continue
    for (const v of segs) {
      const bare = v.replace(/^not-.*$/, "")
      if (!bare || /^data-\[slot=/.test(bare)) continue
      if (SHORTHAND[bare]) { const [a, val] = SHORTHAND[bare]; out.set(`${a}=${val}`, [a, val]); continue }
      let m = /^(data|aria)-\[([\w-]+)(?:=([\w-]+))?\]$/.exec(bare)
      if (m) { if (CVA_AXES.has(m[2])) continue; const a = `${m[1]}-${m[2]}`, val = m[3] ?? "true"; out.set(`${a}=${val}`, [a, val]); continue }
      m = /^aria-(expanded|invalid|checked|disabled|pressed|selected|current)$/.exec(bare)
      if (m) { out.set(`aria-${m[1]}=true`, [`aria-${m[1]}`, "true"]); continue }
      m = /^data-(inset|highlighted|empty|pressed|autoscrolling|popup-open)$/.exec(bare)
      if (m) out.set(`data-${m[1]}=`, [`data-${m[1]}`, ""])
    }
  }
  return [...out.values()]
}

rmSync(SIM, { recursive: true, force: true })
mkdirSync(`${SIM}/node_modules`, { recursive: true })
symlinkSync(ROOT, `${SIM}/node_modules/shadless`, "dir")
const oracleCss = readFileSync("build/gates/oracle.css", "utf8")
const fullCss = readFileSync("dist/shadless.full.css", "utf8")

const browser = await chromium.launch()
const cells = []
let compared = 0, components = 0, stateRenders = 0, variantRenders = 0

const page = async (html) => { const p = await browser.newPage({ reducedMotion: "reduce" }); await p.setContent(html); await p.waitForTimeout(50); return p }
const readAll = (p, ids) => p.evaluate(({ ids, props }) => {
  const out = {}
  for (const [dark, dir] of [["light", "ltr"], ["dark", "ltr"], ["light", "rtl"], ["dark", "rtl"]]) {
    document.documentElement.classList.toggle("dark", dark === "dark"); document.documentElement.setAttribute("dir", dir)
    for (const id of ids) {
      const el = document.getElementById(id); if (!el) continue
      const cs = getComputedStyle(el); const style = {}
      for (const p of props) style[p] = cs.getPropertyValue(p)
      out[`${id}@${dark}@${dir}`] = style
    }
  }
  return out
}, { ids, props: PROPS })
const norm = (v) => v.replace(/-?\d*\.?\d+(?:e[-+]?\d+)?/gi, (n) => String(Math.round(parseFloat(n) * 100) / 100 || 0))
  .replace(/\boklab\((-?[\d.]+) 0 0\)/g, "oklch($1 0 0)")

for (const f of readdirSync("src/registry/ir").filter((x) => x.endsWith(".json")).sort()) {
  const name = f.slice(0, -5)
  if (!existsSync(`dist/css/${name}.css`)) continue
  const ir = JSON.parse(readFileSync(`src/registry/ir/${f}`, "utf8"))
  const cva = cvaSlot(ir)
  const cvaSlots = new Map(Object.values(cva).filter((t) => t.slot).map((t) => [t.slot, t.table]))
  const items = [] // { id, label, tag, slotHtml, inlineHtml, kids:[{id,label}] }
  let n = 0
  const seen = new Set()
  const elOfSlot = (slot) => { for (const c of ir.components) for (const e of c.elements) if (e.slot === slot) return [c, e]; return [null, null] }
  const tagOf = (el) => normalizeTag(el.tag, ir.tagHints) ?? "div"
  // React's class list for an element: cva slots compose base + the
  // selected value per axis (defaults elsewhere), plain slots take the
  // conditional's default branch
  // cva slots: React renders cn(table({...sel}), className) = twMerge over
  // base + one value per axis + the extras. The converter resolved the call
  // with DEFAULT values into el.classes, so the extras are el.classes minus
  // the base and minus every value string of the table — appending the raw
  // list put the default's bg-primary next to destructive's bg and let
  // Tailwind's internal order pick the default (every badge/attachment
  // variant read as the default on the oracle side: harness, not product)
  const classesOf = (c, el, sel = {}) => {
    const table = cvaSlots.get(el.slot)
    if (!table) {
      // context-driven axes (cvaRefs with a default): at rest React composes
      // the axis DEFAULT into the class list (context.size || size)
      const ctxDefaults = (ir.cvaRefs ?? []).filter((r) => r.slot === el.slot).flatMap((r) =>
        (r.dynAxes ?? []).map((ax) => r.defaults?.[ax] !== undefined ? r.table.variants?.[ax]?.[sel[ax] ?? r.defaults[ax]] : null)).filter(Boolean)
      return ctxDefaults.length ? twMerge([inlineClasses(ir, c.fn, el), ...ctxDefaults].join(" ")) : inlineClasses(ir, c.fn, el)
    }
    const values = new Set(Object.values(table.variants ?? {}).flatMap((v) => Object.values(v)).map(String))
    const extras = el.classes.filter((x) => x !== table.base && !values.has(x))
    return twMerge([table.base, ...Object.entries(table.variants ?? {}).map(([ax, vals]) => vals[sel[ax] ?? table.defaults?.[ax]]).filter(Boolean),
      ...extras].filter(Boolean).join(" "))
  }
  const wrap = (h) => `<div style="position:relative;width:480px;height:160px;margin:8px 0">${h}</div>`
  const openTag = (tag, attrs, inner) => VOID.has(tag) ? `<${tag} ${attrs}>` : `<${tag} ${attrs}>${inner}</${tag}>`
  // Children referenced by the root's has-/*:/data-[slot=…] variants are
  // rendered on BOTH sides as real elements with their own classes — inline
  // on the oracle side, slot-only on ours — so a variant that styles its
  // children (*:data-[slot=field-label]:w-auto) is measured on the child.
  // Grandchildren the child itself references stay inert (activation only).
  const build = (c, el, sel, attrs, label, extra = {}) => {
    const tag = tagOf(el)
    const cls = classesOf(c, el, sel)
    const { markers } = splitMarkers(cls)
    const kids = []
    let slotKids = "", inlineKids = ""
    for (const k of childSlots(cls)) {
      const [kc, ke] = elOfSlot(k)
      if (!ke) continue
      const ktag = tagOf(ke)
      if (/^[A-Z]/.test(ktag) || ktag === "?") continue
      const kcls = classesOf(kc, ke)
      const km = splitMarkers(kcls).markers.join(" ")
      const inert = childSlots(kcls).map((g) => `<div data-slot="${g}" style="display:none">x</div>`).join("")
      const kinner = VOID.has(ktag) || ktag === "svg" ? "" : `x${inert}`
      const kid = kids.length
      kids.push({ id: `${n}-${kid}`, label: `${label}>${k}` })
      slotKids += openTag(ktag, `data-slot="${k}" id="so-${n}-${kid}" class="${km}"`, kinner)
      inlineKids += openTag(ktag, `data-slot="${k}" id="in-${n}-${kid}" class="${kcls}"`, kinner)
    }
    const inner = (t, k) => VOID.has(t) || t === "svg" ? "" : `x${k}`
    items.push({
      id: n, label, tag, kids, ...extra,
      slotHtml: wrap(openTag(tag, `data-slot="${el.slot}" ${attrs} id="so-${n}" class="${markers.join(" ")}"`, inner(tag, slotKids))),
      // React's DOM carries data-slot AND the classes; the oracle stylesheet
      // has no slot rules, but has-data-[slot=…] / *:data-[slot=…] variants
      // need the attribute on the target to fire on both sides
      inlineHtml: wrap(openTag(tag, `data-slot="${el.slot}" ${attrs} id="in-${n}" class="${cls}"`, inner(tag, inlineKids))),
    })
    n++
    return cls
  }
  for (const c of ir.components) c.elements.forEach((el, idx) => {
    if (!el.slot || seen.has(el.slot)) return
    const tag = tagOf(el)
    if (/^[A-Z]/.test(tag) || tag === "?") return
    seen.add(el.slot)
    const cls = classesOf(c, el)
    if (!cls.trim()) return
    // at rest
    build(c, el, {}, "", `${el.slot}#${idx}`)
    // every cva axis value (variant-parity's cells, now against the real oracle)
    const table = cvaSlots.get(el.slot)
    if (table) for (const [axis, vals] of Object.entries(table.variants ?? {})) for (const v of Object.keys(vals)) {
      build(c, el, { [axis]: v }, `data-${axis}="${v}"`, `${el.slot}#${idx}[${axis}=${v}]`, { variant: true })
    }
    // context-driven axes (cvaRefs with a default): React sets data-<axis>
    // on the element and composes the value into the class list
    for (const r of (ir.cvaRefs ?? []).filter((r) => r.slot === el.slot)) for (const axis of r.dynAxes ?? []) {
      if (r.defaults?.[axis] === undefined) continue
      for (const [v, cls] of Object.entries(r.table.variants?.[axis] ?? {})) {
        if (!cls) continue
        const inline = classesOf(c, el, { [axis]: v })
        const tag = tagOf(el), { markers } = splitMarkers(inline)
        const wrapIt = (h) => `<div style="position:relative;width:480px;height:160px;margin:8px 0">${h}</div>`
        items.push({ id: n, label: `${el.slot}#${idx}[${axis}=${v}]`, tag, kids: [], variant: true,
          slotHtml: wrapIt(openTag(tag, `data-slot="${el.slot}" data-${axis}="${v}" id="so-${n}" class="${markers.join(" ")}"`, "x")),
          inlineHtml: wrapIt(openTag(tag, `data-slot="${el.slot}" data-${axis}="${v}" id="in-${n}" class="${inline}"`, "x")) })
        n++
      }
    }
    // every attribute-driven state the class list styles
    for (const [attr, val] of stateConfigs(cls)) build(c, el, {}, `${attr}="${val}"`, `${el.slot}#${idx}[${attr}=${val}]`, { state: true })
  })
  if (!items.length) continue
  components++
  stateRenders += items.filter((i) => i.state).length
  variantRenders += items.filter((i) => i.variant).length

  // (a) consumer build: core + this component's css
  writeFileSync(`${SIM}/entry.css`, `@import "shadless";\n@import "shadless/${name}.css";\n`)
  try { execFileSync(process.execPath, ["tools/tw.mjs", `${SIM}/entry.css`, `${SIM}/out.css`, "--cwd", SIM], { stdio: "pipe" }) }
  catch (e) { console.error(`FAIL  path-parity: shadless/${name}.css does not compile alone\n${e.stderr}`); process.exit(1) }
  const consumerCss = readFileSync(`${SIM}/out.css`, "utf8")
  const shell = "body{margin:0;padding:0;color:var(--foreground);background:var(--background)} *{transition:none!important;animation:none!important}"
  const doc = (css, body, rootClass = "") => `<!doctype html><html class="${rootClass}"><head><style>${css}</style><style>${shell}</style></head><body>${body}</body></html>`
  const ids = items.flatMap((i) => [`so-${i.id}`, ...i.kids.map((k) => `so-${k.id}`)])
  const inIds = items.flatMap((i) => [`in-${i.id}`, ...i.kids.map((k) => `in-${k.id}`)])
  if (process.env.PP_KEEP) writeFileSync(`${SIM}/${name}.html`, doc(consumerCss, items.map((i) => i.slotHtml).join("\n")) + "\n<!-- ORACLE -->\n" + doc("", items.map((i) => i.inlineHtml).join("\n"), "style-nova"))
  const pA = await page(doc(consumerCss, items.map((i) => i.slotHtml).join("\n")))
  const pB = await page(doc(fullCss, items.map((i) => i.slotHtml).join("\n")))
  const pO = await page(doc(oracleCss, items.map((i) => i.inlineHtml).join("\n"), "style-nova"))
  const [a, b, o] = await Promise.all([readAll(pA, ids), readAll(pB, ids), readAll(pO, inIds)])
  await Promise.all([pA.close(), pB.close(), pO.close()])
  for (const it of items) for (const [theme, dir] of MATRIX) {
    for (const node of [{ id: it.id, label: it.label }, ...it.kids]) {
      const ref = o[`in-${node.id}@${theme}@${dir}`]
      for (const [path, side] of [["css-import", a], ["full-css", b]]) {
        const got = side[`so-${node.id}@${theme}@${dir}`]
        if (!ref || !got) continue
        compared++
        for (const p of PROPS) {
          const va = norm(ref[p]), vb = norm(got[p])
          if (va !== vb) cells.push({ id: `${name}/${node.label}/${p}@${path}@${theme}@${dir}`, detail: `oracle=${va.slice(0, 50)} shadless=${vb.slice(0, 50)}` })
        }
      }
    }
  }
}
await browser.close()
if (!process.env.PP_KEEP) rmSync(SIM, { recursive: true, force: true })

if (process.argv.includes("--details")) for (const c of cells) if (/@css-import@light@ltr$/.test(c.id)) console.log(`${c.id}: ${c.detail}`)
const actual = new Set(cells.map((c) => c.id))
if (RECORD || !existsSync(BASELINE)) {
  const pin = JSON.parse(readFileSync("src/registry/pin.json", "utf8")).shadcn_ui.tag
  writeFileSync(BASELINE, JSON.stringify({ pin, note: "slot-only markup via css-import / full-css vs React inline classes under upstream css; may only shrink", cells: [...actual].sort() }, null, 1) + "\n")
  console.log(`path-parity: baseline recorded (${actual.size} cells over ${components} components, ${compared} element×path×theme×dir comparisons incl. ${variantRenders} variant + ${stateRenders} state renders)`)
  process.exit(0)
}
const recorded = new Set(JSON.parse(readFileSync(BASELINE, "utf8")).cells)
const appeared = cells.filter((c) => !recorded.has(c.id))
const fixed = [...recorded].filter((id) => !actual.has(id))
if (appeared.length) {
  console.error(`FAIL  path-parity (${appeared.length} NEW cells where a consume path ≠ React under upstream css)\n  ` +
    appeared.slice(0, 40).map((c) => `${c.id}: ${c.detail}`).join("\n  ") + (appeared.length > 40 ? `\n  … +${appeared.length - 40} more` : ""))
  process.exit(1)
}
if (fixed.length) {
  console.error(`FAIL  path-parity (${fixed.length} recorded cells no longer differ — record the win: node gates/path-parity.mjs --record && node gates/ledger.mjs --record)\n  ` + fixed.slice(0, 20).join("\n  "))
  process.exit(1)
}
console.log(`PASS  path-parity (${components} components, ${compared} comparisons incl. ${stateRenders} state renders, ${actual.size} cells at the recorded baseline; --strict is the end state)`)
