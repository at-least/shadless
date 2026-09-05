// Shape of the converter's IR JSON (generated/ir/<name>.json, schema 2).
// Written from the actual emitted files, not from the Go structs — the
// emitter only consumes these fields, so anything unconsumed stays out.
// If the converter adds a field the emitter reads, add it here; tsc (checkJs)
// will flag accesses that don't match.

export interface IrElement {
  /** raw JSX tag as written in the upstream fn (e.g. "AccordionPrimitive.Root") */
  tag: string
  slot: string | null
  /** tailwind classes recorded for the element, including conditional branches */
  classes: string[]
  spread: boolean
  /** child sketches: "<Tag slot=x class=[1]>" | "{children}" | text | expr */
  children: string[]
  attrs?: Record<string, string>
}

export interface IrComponent {
  fn: string
  export: boolean
  elements: IrElement[]
}

export interface IrCondTest {
  name: string
  op: "==="
  value: string
  /** the fn's default prop value; absent ⇒ the prop is optional with no default */
  default?: string
}

export interface IrConditional {
  kind: string
  fn: string
  /** kind === "class-cond": the two branch class strings */
  then?: string
  else?: string
  test?: IrCondTest
  /** other conditional kinds (wrapper insertion) carry the parent fn instead */
  parent?: string
  slot?: string
}

export interface CvaTable {
  base: string
  variants: Record<string, Record<string, string>>
  defaults?: Record<string, string>
}

export interface CvaRefDyn {
  attr: string
  when: string
  classes: string
}

export interface CvaRef {
  slot: string
  ref: string
  table: CvaTable
  dyn?: CvaRefDyn[]
  dynAxes?: string[]
  defaults?: Record<string, string>
}

export interface Ir {
  schema: number
  source: { commit: string }
  name: string
  tier: string
  imports: string[]
  icons: string[]
  cva: Record<string, CvaTable>
  components: IrComponent[]
  conditionals?: IrConditional[]
  cvaRefs?: CvaRef[]
  /** raw JSX tag → native tag hint recorded by the converter */
  tagHints: Record<string, string>
}

/** One tiers.json entry: { tier, radix: [...] } */
export interface TierEntry {
  tier: string
  radix?: string[]
}
