// gates/parity-baseline.mjs — the recorded-difference baseline shared by the
// three parity gates (style-parity, demo-parity, path-parity).
//
// Why it exists twice over:
//
// 1. The three gates had three copies of the same record/compare block, and
//    they had already drifted (style-parity grew a `flaky` list and a --strict
//    mode the other two never got).
//
// 2. All three recorded only the cell ID. The comment in style-parity called
//    that deliberate — "a legitimate upstream change to an already-drifting
//    cell should not churn the baseline" — but the consequence was that the
//    210 + 76 recorded cells were value-BLIND: once `button/padding-left` was
//    on the list, it could compute 4px against the oracle's 16px today and
//    400px tomorrow and the gate stayed green. A recorded cell is supposed to
//    be a difference someone LOOKED at; the amount is the part they looked at.
//    Values are now pinned too, so a recorded cell that MOVES fails and has to
//    be re-recorded — the same ratchet the ids already had.
//
// Baseline shape:
//   { pin, note, flaky?: string[], cells: [{ id, oracle, shadless }, …] }
import { readFileSync, writeFileSync, existsSync } from "node:fs"

// Live cells (each { id, oracle, shadless }) as an id-keyed map. Duplicate ids
// would silently drop a difference, so they are rejected rather than merged.
export function cellMap(cells) {
  const m = new Map()
  for (const c of cells) {
    if (m.has(c.id)) throw new Error(`duplicate parity cell id: ${c.id} — the id is not unique enough to ratchet on`)
    m.set(c.id, { oracle: String(c.oracle ?? ""), shadless: String(c.shadless ?? "") })
  }
  return m
}

export function loadBaseline(path) {
  if (!existsSync(path)) return null
  const b = JSON.parse(readFileSync(path, "utf8"))
  const cells = new Map()
  for (const c of b.cells ?? []) {
    if (typeof c === "string")
      throw new Error(`${path} is in the pre-value format (bare cell ids). Re-record it:\n` +
        `  the gate's own --record, then ./build/pipeline ledger --record`)
    cells.set(c.id, { oracle: c.oracle, shadless: c.shadless })
  }
  return { ...b, cells }
}

export function writeBaseline(path, { note, flaky, cells }) {
  const pin = JSON.parse(readFileSync("src/registry/pin.json", "utf8")).shadcn_ui.tag
  const body = { pin, note }
  if (flaky) body.flaky = [...flaky].sort()
  body.cells = [...cells].sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)
    .map(([id, v]) => ({ id, oracle: v.oracle, shadless: v.shadless }))
  writeFileSync(path, JSON.stringify(body, null, 1) + "\n")
}

// Three ways a baseline can be wrong, all of them failures:
//   appeared  a cell differs that no one recorded          (a NEW regression)
//   fixed     a recorded cell no longer differs            (record the win, so
//             the slack cannot be silently re-spent)
//   changed   a recorded cell still differs, by a DIFFERENT amount
export function diffBaseline(recorded, actual) {
  const appeared = [], fixed = [], changed = []
  for (const [id, v] of actual) {
    const was = recorded.get(id)
    if (!was) appeared.push(id)
    else if (was.oracle !== v.oracle || was.shadless !== v.shadless)
      changed.push({ id, was, now: v })
  }
  for (const id of recorded.keys()) if (!actual.has(id)) fixed.push(id)
  appeared.sort(); fixed.sort(); changed.sort((a, b) => a.id < b.id ? -1 : 1)
  return { appeared, fixed, changed }
}

const trunc = (s, n = 60) => s.length > n ? s.slice(0, n) + "…" : s
export const showCell = (v) => `oracle=${trunc(v.oracle)} shadless=${trunc(v.shadless)}`
export const showChange = (c) =>
  `${c.id}\n      recorded: ${showCell(c.was)}\n      now:      ${showCell(c.now)}`
