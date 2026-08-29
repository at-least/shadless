// converter (src/converter/index.mjs) — Wave H: synthetic .tsx → exact IR.
// Targets the parsing layer the drift gates can't see: class attribution,
// cva bounded value shapes, array/empty variant values, tagHints resolution.
import {
  parseTs, tierOf, str, jsxName, collectExportedNames, cvaTablesOf,
  resolveCvaArgs, classStrings, convertFile, buildTagHints, tagVarsOf,
} from "../../src/converter/index.mjs"

const REG = { cvaByExport: new Map(), compCva: new Map() }

export function run(t) {
  // ---- tierOf ----
  t.eq("tierOf: kernel set", tierOf("dialog", ["radix-ui"]), "kernel")
  t.eq("tierOf: external dep", tierOf("calendar", ["react-day-picker", "react"]), "external")
  t.eq("tierOf: static (only framework imports)", tierOf("badge", ["react", "@/lib/utils"]), "static")

  // ---- collectExportedNames ----
  {
    const ast = parseTs(`import { x } from "p"
export function A() {}
export const B = () => null
const C = 1
export { C as D }
export default function E() {}`)
    t.eq("exports: honest set", [...collectExportedNames(ast)].sort(), ["A", "B", "D", "E"])
  }

  // ---- cvaTablesOf: array values join, quoted keys, empty-string values ----
  {
    const ast = parseTs(`
const v = cva("base-cls", {
  variants: {
    orientation: { vertical: ["flex-col [&>*]:w-full", "gap-y-2"], horizontal: "flex-row" },
    variant: { icon: "", image: "bg-cover" },
    "quoted-axis": { "icon-xs": "size-8" },
  },
  defaultVariants: { orientation: "vertical" },
})`)
    const tables = cvaTablesOf(ast)
    t.eq("cva: base", tables.v.base, "base-cls")
    t.eq("cva: array value joined", tables.v.variants.orientation.vertical,
      "flex-col [&>*]:w-full gap-y-2")
    t.eq("cva: empty string value preserved", tables.v.variants.variant.icon, "")
    t.eq("cva: quoted keys", tables.v.variants["quoted-axis"]["icon-xs"], "size-8")
    t.eq("cva: defaults", tables.v.defaults, { orientation: "vertical" })
  }

  // ---- resolveCvaArgs: bounded value shapes ----
  const mkEl = (attrs = []) => ({ openingElement: { attributes: attrs } })
  const table = { base: "base-cls", variants: {
    size: { sm: "h-8", lg: "h-10", "icon-xs": "size-8" },
    variant: { outline: "border", ghost: "hover:bg" },
    active: { on: "font-bold", off: "" },
  }, defaults: { size: "sm" } }
  const lit = (v) => ({ type: "StringLiteral", value: v })

  {
    const acc = []
    resolveCvaArgs({}, null, table, { size: lit("lg") }, acc, "t", false)
    t.eq("resolve: literal value", acc, ["base-cls", "h-10"])
  }
  {
    const acc = []
    resolveCvaArgs({ paramDefaults: { size: "icon-xs" } }, null, table,
      { size: { type: "Identifier", name: "size" } }, acc, "t", false)
    t.eq("resolve: param default", acc, ["base-cls", "size-8"])
  }
  {
    const acc = []
    resolveCvaArgs({ paramDefaults: { size: "sm" } }, null, table,
      { size: { type: "LogicalExpression", left: { type: "Identifier", name: "x" }, right: { type: "Identifier", name: "size" } } },
      acc, "t", false)
    t.eq("resolve: x ?? param", acc, ["base-cls", "h-8"])
  }
  {
    // dynamic axis: no arg → default merges into base + dynAxes records axis
    const cvaRefs = []
    const ctx = { recordCvaRef: (el, ref, tb, d, ax) => { cvaRefs.push({ d, ax }) } }
    const acc = []
    resolveCvaArgs(ctx, mkEl(), table, {}, acc, "t", true)
    t.eq("resolve: dynamic axis merges default", acc, ["base-cls", "h-8"])
    t.eq("resolve: dynAxes recorded", cvaRefs.map((r) => r.ax), [["size", "variant", "active"]])
  }
  {
    // attr-driven ident-ternary: <tag data-active={isActive}> + isActive ? "on-cls" : "off-cls"
    const el = mkEl([
      { type: "JSXAttribute", name: { name: "data-active" },
        value: { type: "JSXExpressionContainer", expression: { type: "Identifier", name: "isActive" } } },
    ])
    const refs = []
    const ctx = { recordCvaRef: (e2, ref, tb, d) => { refs.push(...d) } }
    const acc = []
    resolveCvaArgs(ctx, el, { base: "", variants: { active: { on: "font-bold", off: "" } }, defaults: {} },
      { active: { type: "ConditionalExpression",
        test: { type: "Identifier", name: "isActive" },
        consequent: lit("on"), alternate: lit("off") } }, acc, "t", true)
    t.eq("resolve: ternary alternate with empty value not pushed", acc, [])
    t.eq("resolve: ternary dyn via data-attr", refs, [{ attr: "data-active", when: "true", classes: "font-bold" }])
  }
  // loud failures (Wave H): unknown variant value + ternary without attr binding
  t.throws("resolve: unknown value throws", () => {
    const acc = []
    resolveCvaArgs({}, null, table, { size: lit("xl") }, acc, "t", false)
  }, /unknown variant value/)
  t.throws("resolve: ternary without data-attr binding throws", () => {
    const ctx = { recordCvaRef: () => {} }
    const acc = []
    resolveCvaArgs(ctx, mkEl(), { base: "", variants: { v: { a: "x", b: "" } }, defaults: {} },
      { v: { type: "ConditionalExpression", test: { type: "Identifier", name: "nope" },
        consequent: lit("a"), alternate: lit("b") } }, acc, "t", false)
  }, /without data-\* attr/)

  // ---- classStrings ----
  {
    const ast = parseTs(`const x = <div className={cn("a-1", "b-2", isActive ? "on-x" : "off-x")} />`)
    const classNameAttr = ast.program.body[0].declarations[0].init.openingElement.attributes[0]
    const acc = []
    classStrings(classNameAttr.value, acc, null, null)
    t.eq("classStrings: cn literal args + both ternary branches", acc.sort(),
      ["a-1", "b-2", "off-x", "on-x"])
  }
  {
    const ast = parseTs("const x = <div className={`flex`} />")
    const attr = ast.program.body[0].declarations[0].init.openingElement.attributes[0]
    const acc = []
    classStrings(attr.value, acc, null, null)
    t.eq("classStrings: single-quasi template", acc, ["flex"])
    const ast2 = parseTs("const x = <div className={`flex ${y}`} />")
    const attr2 = ast2.program.body[0].declarations[0].init.openingElement.attributes[0]
    const acc2 = []
    classStrings(attr2.value, acc2, null, null)
    t.eq("classStrings: interpolated template skipped (documented limit)", acc2, [])
  }

  // ---- tagVarsOf ----
  {
    const ast = parseTs(`function F() {
  const Comp = asChild ? Slot.Root : "span"
  return <Comp data-slot="x">t</Comp>
}`)
    t.eq("tagVars: Comp alternate", tagVarsOf(ast.program.body[0]), { Comp: "span" })
  }

  // ---- convertFile + buildTagHints end-to-end (synthetic 2-file registry) ----
  {
    const btnSrc = `import * as React from "react"
import { Slot } from "radix-ui"
import { cva } from "class-variance-authority"
const buttonVariants = cva("btn-base", { variants: { size: { sm: "h-8" } }, defaultVariants: { size: "sm" } })
function Button({ asChild }) {
  const Comp = asChild ? Slot.Root : "button"
  return <Comp data-slot="button" className="inline-flex btn-base">Go</Comp>
}
export { Button, buttonVariants }`
    const pagerSrc = `import * as React from "react"
import { ChevronLeftIcon } from "lucide-react"
import { Button } from "@/components/ui/button"
function PaginationPrevious() {
  return <Button data-slot="pagination-previous" className="gap-1"><ChevronLeftIcon /><span className="sr-only">Previous</span></Button>
}
function Provider() { return <ExtPrimitive.Viewport data-slot="vp" /> }
export { PaginationPrevious, Provider }`
    // build the cross-file cva registry the converter main builds from all files
    const REG2 = { cvaByExport: new Map(), compCva: new Map(), pinCommit: null }
    {
      const astBtn = parseTs(btnSrc)
      const exportedBtn = collectExportedNames(astBtn)
      for (const [cvaName, tbl] of Object.entries(cvaTablesOf(astBtn))) {
        if (!exportedBtn.has(cvaName)) continue
        REG2.cvaByExport.set(cvaName, { file: "button", table: tbl })
        REG2.compCva.set("Button", { cvaName, table: tbl, file: "button" })
      }
    }
    const btn = convertFile("button", btnSrc, REG2)
    const pager = convertFile("pagination-previous", pagerSrc, REG2)
    buildTagHints([btn, pager])
    t.eq("tagHints: Comp via local ternary var", btn.tagHints.Comp, "button")
    t.eq("tagHints: icon → svg", pager.tagHints.ChevronLeftIcon, "svg")
    t.eq("tagHints: imported component root (cross-file)", pager.tagHints.Button, "button")
    t.eq("tagHints: external member suffix Viewport → div", pager.tagHints["ExtPrimitive.Viewport"], "div")
    // class attribution: Button-wrap classes resolved onto the element
    const prev = pager.components.find((c) => c.fn === "PaginationPrevious")
    t.ok("convert: wrapped classes recorded", prev.elements[0].classes.includes("btn-base"),
      JSON.stringify(prev.elements[0].classes))
    t.ok("convert: wrap recorded as cross-file cvaRef",
      pager.cvaRefs.some((r) => r.slot === "pagination-previous"), JSON.stringify(pager.cvaRefs))
  }

  // ---- buildTagHints loud failure on unresolvable tags (non-external) ----
  t.throws("tagHints: unresolvable fails loud", () => {
    const ir = convertFile("weird", `function W() { return <Mystery data-slot="m" /> }\nexport { W }`, REG)
    buildTagHints([ir])
  }, /unresolved/)
}
