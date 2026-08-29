// css.mjs (src/emitter/css.mjs) — migrated assertions + Wave H cases:
// anchors for slotless elements, :is(tag) slot-conflict disambiguation,
// loud failure on same-tag conflicts.
import { MARKER, splitMarkers, componentCss, residueResets, cssEscape } from "../../src/emitter/css.mjs"

export function run(t) {
  // twMerge residue: a value that drops a multi-property base utility must
  // reset the properties it does not set itself (gates/path-parity.mjs)
  t.eq("residue: text-sm dropped by text-[0.8rem] resets line-height",
    residueResets("text-sm h-8", "h-7 text-[0.8rem]"), ["leading-[inherit]"])
  t.eq("residue: text-xs sets line-height itself", residueResets("text-sm", "text-xs"), [])
  t.eq("residue: unrelated value, nothing dropped", residueResets("text-sm rounded-lg", "px-2"), [])
  t.eq("cssEscape: arbitrary value", cssEscape("text-[0.8rem]"), "text-\\[0\\.8rem\\]")
  t.eq("MARKER: group", MARKER.test("group"), true)
  t.eq("MARKER: group/name", MARKER.test("group/attachment"), true)
  t.eq("MARKER: peer", MARKER.test("peer"), true)
  t.eq("MARKER: not groups", MARKER.test("groups"), false)
  t.eq("MARKER: not utility", MARKER.test("flex"), false)
  t.eq("splitMarkers: separates", splitMarkers("group/att flex items-center peer"),
    { apply: "flex items-center", markers: ["group/att", "peer"] })

  // cvaSlot must NOT mis-key to component root (G2 regression):
  // inputGroupButtonVariants' fn renders no slotted element of its own.
  {
    const ir = {
      name: "input-group",
      cva: { inputGroupButtonVariants: {
        base: "flex items-center gap-2 text-sm shadow-none",
        variants: { size: { xs: "h-6", "icon-sm": "size-8" } },
        defaults: { size: "xs" },
      } },
      components: [
        { fn: "InputGroupButton", export: true, elements: [{ tag: "Button", slot: null, classes: [], spread: true, children: [] }] },
        { fn: "InputGroup", export: true, elements: [{ tag: "div", slot: "input-group", classes: ["flex w-full items-center rounded-md border"], spread: true, children: [] }] },
      ],
      cvaRefs: [],
    }
    const { rules } = componentCss(ir)
    const rootRule = rules.find((r) => r.includes('[data-slot="input-group"]'))
    t.ok("componentCss: table w/o slot emits no rule", !rules.some((r) => r.includes("h-6")), JSON.stringify(rules))
    t.ok("componentCss: root keeps its own classes only",
      rootRule.includes("rounded-md") && !rootRule.includes("shadow-none"), rootRule)
  }

  // cross-file cvaRefs emit dynamic attr rules (G1 regression)
  {
    const ir = {
      name: "pagination", cva: {},
      components: [{ fn: "PaginationLink", export: true, elements: [
        { tag: "a", slot: "pagination-link", classes: ["inline-flex"], spread: true, children: [] },
      ] }],
      cvaRefs: [{
        slot: "pagination-link", ref: "buttonVariants",
        table: { base: "", variants: { variant: { outline: "border bg-background", ghost: "hover:bg-accent" } }, defaults: {} },
        dyn: [{ attr: "data-active", when: "true", classes: "border bg-background" }],
        dynAxes: ["variant"],
      }],
    }
    const { rules } = componentCss(ir)
    t.ok("componentCss: cvaRefs dyn rule", rules.some((r) => r.includes('[data-slot="pagination-link"][data-active="true"]')))
    t.ok("componentCss: cvaRefs axis rule (outline value)",
      rules.some((r) => r.includes('[data-slot="pagination-link"][data-variant="outline"]')),
      JSON.stringify(rules))
    t.ok("componentCss: cvaRefs axis rule (ghost value)",
      rules.some((r) => r.includes('[data-slot="pagination-link"][data-variant="ghost"]')))
  }

  // Wave H E4: slotless elements with classes get anchor rules + tokens
  {
    const ir = {
      name: "button-group", cva: {},
      components: [
        // slotless root with classes → anchor = kebab(fn)
        { fn: "ButtonGroupText", export: true, elements: [
          { tag: "Comp", slot: null, classes: ["flex items-center gap-2 rounded-md border"], spread: true, children: [] },
        ] },
        // slotted root → normal slot rule, no anchor
        { fn: "ButtonGroup", export: true, elements: [
          { tag: "div", slot: "button-group", classes: ["flex w-fit"], spread: true, children: [] },
        ] },
      ],
      cvaRefs: [], tagHints: { Comp: "div" },
    }
    const { rules, markers, anchors } = componentCss(ir)
    t.eq("anchors: slotless root token", anchors.get("ButtonGroupText#0"), "button-group-text")
    t.ok("anchors: anchor rule emitted", rules.some((r) => r.includes(".button-group-text { @apply flex items-center gap-2 rounded-md border; }")),
      JSON.stringify(rules))
    t.ok("anchors: slotted element gets slot rule not anchor",
      rules.some((r) => r.includes('[data-slot="button-group"]')) && !anchors.has("ButtonGroup#0"))
    t.eq("anchors: no markers for these", markers, {})
  }

  // Wave H E5: same slot, different classes, different tags → :is() rules;
  // Wave L: same tag different classes no longer throws — intersection into
  // the slot rule + per-element remainder anchors (accordion-trigger-icon
  // down/up precedent). Disjoint sets anchor every element, no slot rule.
  {
    const mk = (classesA, classesB, tagA = "input", tagB = "textarea") => ({
      name: "x", cva: {}, tagHints: {},
      components: [
        { fn: "F1", export: true, elements: [{ tag: tagA, slot: "ctl", classes: [classesA], spread: false, children: [] }] },
        { fn: "F2", export: true, elements: [{ tag: tagB, slot: "ctl", classes: [classesB], spread: false, children: [] }] },
      ],
      cvaRefs: [],
    })
    const { rules } = componentCss(mk("flex-1 border-0", "flex-1 resize-none py-3"))
    t.ok("conflict: :is(input) rule", rules.some((r) => r.includes('[data-slot="ctl"]:is(input) { @apply flex-1 border-0; }')), JSON.stringify(rules))
    t.ok("conflict: :is(textarea) rule", rules.some((r) => r.includes('[data-slot="ctl"]:is(textarea) { @apply flex-1 resize-none py-3; }')))
    const merged = componentCss(mk("shared a-1", "shared b-2", "input", "input"))
    t.ok("conflict: same-tag shared tokens ride the slot rule",
      merged.rules.some((r) => r.includes('[data-slot="ctl"] { @apply shared; }')), JSON.stringify(merged.rules))
    t.eq("conflict: same-tag remainders get one anchor each",
      merged.rules.filter((r) => r.includes("@apply a-1;") || r.includes("@apply b-2;")).length, 2)
    const disjoint = componentCss(mk("a-1", "b-2", "input", "input"))
    t.ok("conflict: disjoint classes emit no slot rule",
      !disjoint.rules.some((r) => r.includes("[data-slot=\"ctl\"]")))
    t.eq("conflict: disjoint classes anchor every element",
      disjoint.rules.filter((r) => r.includes("@apply a-1;") || r.includes("@apply b-2;")).length, 2)
  }
}
