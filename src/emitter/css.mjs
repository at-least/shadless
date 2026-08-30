// Shared slot-keyed CSS generation for the emitter and the demo builder.
// plain-class slots + cva tables → `[data-slot=…]{ @apply … }` rules; marker
// tokens (group/peer) stay in HTML and are returned for the emitter to inject.
//
// Slotless elements with classes (button-group-text, input-group-text, icon
// sizing on pagination chevrons) get class-anchor rules instead — the old
// plain path silently dropped them, shipping whole components without CSS.
// Anchors are returned as a Map ("fn#elementIndex" → class token); the
// emitter renders the token into the element's class= (gate-whitelisted).
//
// Same slot with DIFFERENT class sets from different fns (input-group-control:
// input vs textarea) is resolved with :is(tag) qualification when the tags
// disambiguate; when the same tag carries different sets (bases/radix
// accordion-trigger-icon down/up) the shared tokens ride the slot rule and
// the remainders get per-element anchors (upstream own split).
import { normalizeTag, kebab } from "../tags.mjs"
import { SKIN_ALLOWLIST, SKIN_MAP } from "./skin.mjs"
import { twMerge } from "tailwind-merge"

// twMerge residue. React renders cn(table({…})) — twMerge DROPS a base
// utility that a value utility conflicts with. Usually the value sets the
// same property and later-wins in our rules is equivalent. Not when the
// dropped utility set MORE properties than the winner: text-sm sets
// font-size AND line-height, size=sm's text-[0.8rem] only font-size — React
// ends up inheriting line-height, our size rule kept text-sm's. Found by
// gates/path-parity.mjs (button/toggle size=sm, 19.2px vs 18.29px). For a
// dropped base token whose extra properties are known, the value rule gets
// an explicit reset to the inherited value. Anything not listed here is a
// path-parity cell, not a silent difference.
// [dropped-token shape, [reset utility, shape of a value token that already sets it]]
// (twMerge itself cannot answer "does the value set line-height": it treats
// any font-size utility as conflicting with leading-*, because text-sm does)
const RESIDUE = [
  [/^text-(xs|sm|base|lg|xl|\dxl)$/, [["leading-[inherit]", /^(leading-|text-(xs|sm|base|lg|xl|\dxl)$)/]]],   // font-size + line-height
  [/^size-/, [["w-auto", /^(w-|size-)/], ["h-auto", /^(h-|size-)/]]],                                          // width + height
]
// Tailwind's class-name escaping for a utility used as a selector
export const cssEscape = (t) => t.replace(/[^A-Za-z0-9_-]/g, (ch) => "\\" + ch)
export function residueResets(base, value) {
  const merged = new Set(twMerge(`${base} ${value}`).split(/\s+/))
  const out = []
  const valueToks = value.split(/\s+/).filter(Boolean)
  for (const tok of base.split(/\s+/).filter(Boolean)) {
    if (merged.has(tok)) continue
    for (const [re, resets] of RESIDUE) {
      if (!re.test(tok)) continue
      // reset only the properties the value leaves untouched (text-xs sets
      // line-height itself; w-3 after size-4 leaves only height dangling)
      for (const [r, setsIt] of resets) if (!valueToks.some((v) => setsIt.test(v))) out.push(r)
    }
  }
  return [...new Set(out)]
}

export const MARKER = /^(group|peer)(\/[\w-]+)?$/

// Utilities the pinned registry references but that are defined NOWHERE in
// upstream's own CSS (origin-top-center — navigation-menu viewport; not a
// tailwind builtin). Upstream compiles from a content scan, where an
// unknown class silently produces no rule; our slot rules go through
// @apply, where the same class hard-fails. Keep them in markup as inert —
// byte-compatible with upstream's effective output (dead class, no rule).
export const DEAD_UTILITIES = new Set(["origin-top-center"])

// splitMarkers: partition a class string into @apply-able utilities vs
// markup-only tokens. Markup-only = group/peer markers PLUS the upstream
// skin ALLOWLIST (cn-rtl-flip, cn-menu-target, …): the live ui.shadcn.com
// CSS defines zero rules for those — they are inert markers resolved when
// the CLI installs into a user project, so they must never reach @apply
// (an unresolvable utility) and simply stay on the element's class=.

export function splitMarkers(str) {
  const toks = str.split(/\s+/).filter(Boolean)
  return {
    apply: toks.filter((t) => !MARKER.test(t) && !SKIN_ALLOWLIST.has(t) && !DEAD_UTILITIES.has(t)).join(" "),
    markers: toks.filter((t) => MARKER.test(t) || SKIN_ALLOWLIST.has(t) || DEAD_UTILITIES.has(t)),
  }
}

// cva table → slot mapping: naming convention (buttonVariants→Button) else single
export function cvaSlot(ir) {
  const entries = Object.entries(ir.cva)
  if (!entries.length) return {}
  const out = {}
  for (const [varName, table] of entries) {
    const stem = varName.replace(/Variants$/, "")
    const fn = ir.components.find((c) => c.fn.toLowerCase() === stem.toLowerCase())
      || ir.components.find((c) => c.fn.toLowerCase().startsWith(stem.toLowerCase()))
    // The old `|| ir.name` fallback mis-keyed tables whose fn renders no
    // slotted element (inputGroupButtonVariants → the input-group ROOT),
    // polluting the wrapper with button sizing. No slot → no rule; those
    // cases are covered by cross-file cvaRefs / inlined call classes.
    const slot = fn?.elements.find((e) => e.slot)?.slot || null
    out[varName] = { table, slot }
  }
  return out
}

const cleanTag = (t) => kebab(String(t).replace(/^<ternary:[^/]+\//, "").replace(/>$/, ""))

export function componentCss(ir) {
  const rules = []
  const cvaMap = cvaSlot(ir)
  const cvaSlots = new Set(Object.values(cvaMap).map((v) => v.slot).filter(Boolean))
  const markers = {} // slot -> marker tokens (group/peer + allowlist), stay in HTML
  const anchors = new Map() // "fn#elementIndex" -> class token
  const anchorMarkers = new Map() // "fn#elementIndex" -> allowlist tokens for slotless elements (stay in HTML, no rule)
  const lateAnchorRules = [] // same-slot conflicting-remainder anchor rules
  const usedTokens = new Set()
  const token = (t) => {
    let out = t, n = 1
    while (usedTokens.has(out)) out = `${t}-${++n}`
    usedTokens.add(out)
    return out
  }

  // pass 1: group elements — slotted (by slot) / slotless (anchors)
  const bySlot = new Map() // slot -> [{ el, key }]
  for (const c of ir.components)
    c.elements.forEach((el, idx) => {
      if (!el.classes.length) return
      const key = `${c.fn}#${idx}`
      if (el.slot && !cvaSlots.has(el.slot)) {
        if (!bySlot.has(el.slot)) bySlot.set(el.slot, [])
        bySlot.get(el.slot).push({ el, key })
      } else if (!el.slot) {
        const base = c.elements[0] === el ? kebab(c.fn) : `${kebab(c.fn)}-${cleanTag(el.tag)}`
        anchors.set(key, token(base))
      }
    })

  // Conditional class branches (`orientation === "horizontal" ? "pl-4" :
  // "pt-4"`): the converter records both branches AND leaves both entries in
  // el.classes, so the slot rule applied pl-4 and pt-4 together — every
  // horizontal carousel item was 16px too tall on the css-import path, and
  // the fixture too. Here the branch entries leave the merged list and come
  // back as twin rules keyed on the data-<name> attribute the component
  // exposes (on the element itself or on the component root), absent ⇒ the
  // fn's default. Only conditionals whose test the converter could read
  // (ident === "literal", ir.conditionals[].test) are split; the rest keep
  // the old merged shape and stay visible to style-parity.
  const condBranches = new Map() // "fn#idx" -> [{ then, else, test }]
  const rootSlot = ir.components.map((c) => c.elements.find((e) => e.slot)?.slot).find(Boolean)
  for (const cond of ir.conditionals ?? []) {
    if (cond.kind !== "class-cond" || !cond.test) continue
    const c = ir.components.find((cc) => cc.fn === cond.fn)
    c?.elements.forEach((el, idx) => {
      if (el.classes.includes(cond.then) && el.classes.includes(cond.else)) {
        const key = `${cond.fn}#${idx}`
        if (!condBranches.has(key)) condBranches.set(key, [])
        condBranches.get(key).push(cond)
      }
    })
  }
  const stripBranches = (el, key) => {
    const conds = condBranches.get(key) ?? []
    return el.classes.filter((c) => !conds.some((cd) => c === cd.then || c === cd.else)).join(" ")
  }
  const branchRules = (selector, key) => {
    const out = []
    for (const cond of condBranches.get(key) ?? []) {
      const attr = `data-${kebab(cond.test.name)}`, v = cond.test.value
      const explicitTrue = cond.test.op === "===" ? cond.then : cond.else
      const explicitFalse = cond.test.op === "===" ? cond.else : cond.then
      // absent ⇒ default value ⇒ whichever branch the default selects
      const absentIsTrue = cond.test.default === undefined ? true : (cond.test.default === v) === (cond.test.op === "===")
      const ctx = rootSlot ? `, [data-slot="${rootSlot}"][${attr}="${v}"] *` : ""
      const ctxOther = rootSlot ? `, [data-slot="${rootSlot}"][${attr}]:not([${attr}="${v}"]) *` : ""
      const isV = `:is([${attr}="${v}"]${ctx})`
      const isOther = `:is([${attr}]:not([${attr}="${v}"])${ctxOther})`
      // :where() keeps the branch rules at the base rule's rank (see the cva
      // block below for why specificity is semantics here)
      const t = splitMarkers(explicitTrue), f = splitMarkers(explicitFalse)
      // On the demo path React evaluated the conditional and the element
      // carries ONE branch inline; the root may carry no data-<name> at all
      // (upstream's carousel root has no data-orientation). The absent-⇒-
      // default rule then adds the default branch UNDER the other branch's
      // inline utilities (vertical items got pl-4 next to their pt-4). A
      // branch rule therefore also yields to the presence of the OTHER
      // branch's utilities as inline classes.
      // …including the logical twin of a physical utility (pl-4 ↔ ps-4): the
      // RTL examples carry the logical form inline
      const twins = (tok) => {
        const m = /^(-?)(p|m|inset|scroll-p|scroll-m)(l|r|s|e)-(.+)$/.exec(tok)
        if (!m) return [tok]
        const alt = { l: "s", r: "e", s: "l", e: "r" }[m[3]]
        return [tok, `${m[1]}${m[2]}${alt}-${m[4]}`]
      }
      // A branch rule yields whenever EITHER branch's utilities are already
      // inline (React put the evaluated branch there, possibly overridden by
      // a className like pt-1, possibly in logical form like ps-4 — which in
      // an RTL box is padding-right, so adding our physical pl-4 doubles
      // it). Spacing/inset utilities are matched by GROUP prefix so pt-1
      // shadows pt-4; everything else by exact token.
      const shadows = (tok) => {
        const m = /^(-?)(p|m|inset|top|right|bottom|left|start|end)([tblrsexy])?-/.exec(tok)
        if (!m) return [`:not(.${cssEscape(tok)})`]
        const prefix = m[0]
        return [`:not([class^="${prefix}"])`, `:not([class*=" ${prefix}"])`]
      }
      const notInline = () => [...new Set([...t.apply.split(/\s+/), ...f.apply.split(/\s+/)].filter(Boolean).flatMap(twins).flatMap(shadows))].join("")
      // isV / isOther are :is(...) lists; the :not() chain must qualify the
      // element itself, not the last alternative inside the list
      const trueSel = `${selector}:where(${absentIsTrue ? `:not(${isOther.slice(4, -1)})` : isV}${notInline()})`
      const falseSel = `${selector}:where(${absentIsTrue ? isOther : `:not(${isV.slice(4, -1)})`}${notInline()})`
      if (t.apply) out.push(`  ${trueSel} { @apply ${t.apply}; }`)
      if (f.apply) out.push(`  ${falseSel} { @apply ${f.apply}; }`)
    }
    return out
  }

  // anchor rules (slotless elements): `.token { @apply … }`
  // allowlist skin markers (cn-rtl-flip …) ride the element's class= via
  // anchorMarkers — upstream ships them as inert markup markers with no
  // CSS rule, so they must not appear in any @apply.
  for (const [key, t] of anchors) {
    const [fnName, idx] = [key.slice(0, key.lastIndexOf("#")), Number(key.slice(key.lastIndexOf("#") + 1))]
    const c = ir.components.find((cc) => cc.fn === fnName)
    const el = c?.elements[idx]
    if (!el) continue
    const s = splitMarkers(stripBranches(el, key))
    if (s.markers.length) anchorMarkers.set(key, s.markers)
    if (s.apply) rules.push(`  .${t} { @apply ${s.apply}; }`)
    rules.push(...branchRules(`.${t}`, key))
  }

  // plain-class slot rules (elements with classes, not covered by cva)
  for (const [slot, items] of bySlot) {
    const sigs = new Map() // apply string -> { tags:Set, markers:[] }
    for (const { el, key } of items) {
      const s = splitMarkers(stripBranches(el, key))
      if (!sigs.has(s.apply)) sigs.set(s.apply, { tags: new Set(), markers: [] })
      const info = sigs.get(s.apply)
      const tag = normalizeTag(el.tag, ir.tagHints)
      info.tags.add(tag ?? "?")
      info.markers.push(...s.markers)
    }
    const allMarkers = [...new Set(items.flatMap(({ el }) => splitMarkers(el.classes.join(" ")).markers))]
    if (allMarkers.length) markers[slot] = allMarkers
    if (sigs.size === 1) {
      const [apply] = [...sigs.keys()]
      if (apply) rules.push(`  [data-slot="${slot}"] { @apply ${apply}; }`)
      for (const { key } of items) rules.push(...branchRules(`[data-slot="${slot}"]`, key))
      continue
    }
    // conflicting class sets on one slot: tag-disambiguable → :is(tag)
    // rules. Same tag in more than one sig (bases/radix accordion: two
    // complementary accordion-trigger-icon svgs with different visibility
    // classes) → intersection into the slot rule + per-element remainder
    // anchors, so shared styling rides the slot and the differing tokens
    // stay per-element (exactly upstream's own split).
    const tagOverlap = [...sigs.values()].some((info, _, arr) =>
      [...info.tags].some((t) => t === "?" || arr.some((o) => o !== info && o.tags.has(t))))
    if (!tagOverlap) {
      for (const [apply, info] of sigs) {
        const tag = [...info.tags][0]
        if (apply) rules.push(`  [data-slot="${slot}"]:is(${tag}) { @apply ${apply}; }`)
      }
      continue
    }
    const tokenLists = [...sigs.keys()].map((a) => a.split(/\s+/).filter(Boolean))
    if (!tokenLists.every((l) => l.length)) continue // an empty set can't anchor
    const common = tokenLists[0].filter((t) => tokenLists.every((l) => l.includes(t)))
    if (common.length) rules.push(`  [data-slot="${slot}"] { @apply ${common.join(" ")}; }`)
    for (const { el, key } of items) {
      const own = splitMarkers(el.classes.join(" ")).apply.split(/\s+/).filter(Boolean)
      const rest = own.filter((t) => !common.includes(t))
      if (!rest.length) continue
      const t = token(slot)
      anchors.set(key, t)
      lateAnchorRules.push(`  .${t} { @apply ${rest.join(" ")}; }`)
    }
  }
  // cva: the bare slot rule carries the BASE only. Each axis's default
  // value gets twin blocks — :not([data-<axis>]) for unspecified axes and
  // [data-<axis>="<default>"] for explicit ones — and every other value
  // its own additive rule. This models cva's defaultVariants on the
  // attribute API: composing the default INTO the bare rule (the old
  // shape) leaked default-only utility groups into variants that don't
  // restate them — CSS has no un-apply, so ghost/outline/link inherited
  // the default look's colors and rendered invisible in one theme or the
  // other (2026-08-28 css-import cascade finding; regression-gated by
  // gates/path-parity.mjs, which subsumed the original variant-parity).
  rules.push(...lateAnchorRules)
  for (const { table, slot } of Object.values(cvaMap)) {
    if (!slot) continue
    const s = splitMarkers(table.base || "")
    if (s.markers.length) markers[slot] = s.markers
    if (s.apply) rules.push(`  [data-slot="${slot}"] { @apply ${s.apply}; }`)
    for (const [axis, vals] of Object.entries(table.variants)) {
      const def = table.defaults?.[axis]
      for (const [val, cls] of Object.entries(vals)) {
        if (!cls) continue
        const t = splitMarkers(cls)
        if (t.markers.length) markers[slot] = [...(markers[slot] || []), ...t.markers]
        if (!t.apply) continue
        const resets = residueResets(splitMarkers(table.base || "").apply, t.apply)
        if (resets.length) {
          t.apply += " " + resets.join(" ")
          // the same residue on the DEMO path: React's element carries the
          // value's utilities inline (twMerge dropped the base's text-sm),
          // while our bare slot rule still contributes text-sm's line-height.
          // Key the reset on the inline utility that caused the drop, so an
          // element styled by inline classes gets the reset too.
          const baseApply = splitMarkers(table.base || "").apply
          const causes = t.apply.split(/\s+/).filter((vt) => residueResets(baseApply, vt).length)
          if (causes.length) rules.push(`  [data-slot="${slot}"]:where(${causes.map((c) => "." + cssEscape(c)).join(", ")}) { @apply ${resets.join(" ")}; }`)
        }
        // Specificity is part of cva's semantics. In React every value class
        // is a plain utility (0,1,0): a variant's `bg-transparent` loses to
        // the base's `data-[state=on]:bg-muted` (0,2,0) whenever the state is
        // on. An attribute-qualified rule ([data-slot][data-variant=…]) is
        // (0,2,0) and, coming later, beat the base's state variants — toggle
        // rendered transparent while pressed on the css-import path
        // (gates/path-parity.mjs, state renders). :where() zeroes the
        // qualifier: every value rule is (0,1,0) like the bare slot rule,
        // later-wins resolves value-vs-base exactly like twMerge, and the
        // base's own state variants keep their rank.
        if (val === def)
          // one rule, three selectors — the default applies when the axis
          // is unspecified, explicitly set to it, or set to "" (cva treats
          // a falsy variant prop as unset → defaultVariants)
          rules.push(
            `  [data-slot="${slot}"]:where(:not([data-${axis}]), [data-${axis}="${val}"], [data-${axis}=""]) { @apply ${t.apply}; }`,
          )
        else rules.push(`  [data-slot="${slot}"]:where([data-${axis}="${val}"]) { @apply ${t.apply}; }`)
      }
    }
  }
  // cross-file cva refs (converter-recorded): a slot styled by another
  // file's table with an axis the element exposes as data-<axis> at runtime
  // (toggle-group items: size/variant through React context). Axes with a
  // recorded default get the twin-default shape; others the plain
  // attribute rule.
  for (const r of ir.cvaRefs || []) {
    for (const d of r.dyn || [])
      rules.push(`  [data-slot="${r.slot}"][${d.attr}="${d.when}"] { @apply ${d.classes}; }`)
    for (const axis of r.dynAxes || []) {
      const vals = r.table.variants?.[axis] || {}
      const def = r.defaults?.[axis]
      const baseApply = splitMarkers(r.table.base || "").apply
      for (const [val, cls] of Object.entries(vals)) {
        if (!cls) continue
        const t = splitMarkers(cls)
        if (!t.apply) continue
        if (def === undefined) { rules.push(`  [data-slot="${r.slot}"][data-${axis}="${val}"] { @apply ${t.apply}; }`); continue }
        // context-driven axis with a known default (toggle-group items): the
        // same twin-block / :where / residue shape as a local cva table
        const resets = residueResets(baseApply, t.apply)
        const apply = resets.length ? `${t.apply} ${resets.join(" ")}` : t.apply
        rules.push(val === def
          ? `  [data-slot="${r.slot}"]:where(:not([data-${axis}]), [data-${axis}="${val}"], [data-${axis}=""]) { @apply ${apply}; }`
          : `  [data-slot="${r.slot}"]:where([data-${axis}="${val}"]) { @apply ${apply}; }`)
        if (resets.length) {
          const causes = t.apply.split(/\s+/).filter((vt) => residueResets(baseApply, vt).length)
          if (causes.length) rules.push(`  [data-slot="${r.slot}"]:where(${causes.map((c) => "." + cssEscape(c)).join(", ")}) { @apply ${resets.join(" ")}; }`)
        }
      }
    }
  }
  // Allowlisted skin markers stay in the markup as literal classes (that is
  // upstream's install-time contract for them). Most have NO body in the
  // pinned skin and are inert. cn-menu-translucent is the exception: at
  // shadcn@4.19.0 style-nova.css defines it (bg-popover/70 + backdrop-blur,
  // applied unconditionally to every menu/select content) — keeping the
  // class without its rule shipped opaque menus on five components while
  // every gate stayed green (found by gates/overlay.mjs's dissolve check:
  // "upstream now emits rules for this allowlist entry"). Emit the skin's
  // own body as a class-anchored rule wherever the marker is used, so a
  // consumer importing just this component's css gets it too.
  //
  // Cascade position matters as much as the rule: upstream's skin rules are
  // UNLAYERED (`.style-nova .cn-menu-translucent { … }` at the top level of
  // style-nova.css), so they beat the utilities layer — that is how
  // bg-popover/70 wins over the inline `bg-popover` on the same element.
  // Emitting the rule inside @layer components (the first fix) lost to the
  // inline utility and still rendered opaque; the independent CSS oracle
  // (pipeline/oracle_css.go) is what showed it. Hence `unlayered`.
  const usedAllowlist = new Set([...Object.values(markers).flat(), ...[...anchorMarkers.values()].flat()]
    .filter((t) => SKIN_ALLOWLIST.has(t) && SKIN_MAP[t]))
  const unlayered = [...usedAllowlist].sort().map((t) => `.${t} { @apply ${SKIN_MAP[t]}; }`)
  // Every @apply list goes through twMerge, exactly as React's cn() does to
  // the element's class list. Inside ONE list Tailwind resolves a same-group
  // conflict by its internal utility order, not by position: a slot rule
  // carrying both text-sm (cva base) and text-[0.8rem] (skin) kept text-sm's
  // line-height under the arbitrary font-size — 18.29px where React, having
  // dropped text-sm, inherits 19.2px (gates/demo-parity.mjs, toggle). twMerge
  // keeps the later utility and drops the earlier conflicting one.
  const merged = (list) => list.map((r) => r.replace(/@apply ([^;]+);/g, (m, a) => `@apply ${twMerge(a)};`))
  return { rules: merged(rules), markers, anchors, anchorMarkers, unlayered: merged(unlayered) }
}

// Wrap one component's rule set exactly the way every writer must: the slot
// and anchor rules in @layer components, the skin-marker rules after it,
// unlayered. One definition, used by the static emitter and the demo builder.
export function wrapComponentCss(name, { rules, unlayered = [] }) {
  const parts = [`/* ${name} */\n@layer components {\n${rules.join("\n")}\n}`]
  if (unlayered.length) parts.push(`/* ${name}: skin markers (unlayered, as upstream ships them) */\n${unlayered.join("\n")}`)
  return parts.join("\n")
}
