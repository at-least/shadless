// The baseline used to record cell IDS only. Once a cell was on the list it
// could compute anything at all against the oracle — 4px today, 400px
// tomorrow — and the gate stayed green over 210 of them. Values are pinned
// now (gates/parity-baseline.mjs); moving a recorded one must fail.
//
// Perturbing the RECORDED value rather than the fixture is the point: it is
// the only mutation that can distinguish "the gate reads the values" from
// "the gate reads the ids", which is exactly what was broken.
import { readFileSync, writeFileSync } from "node:fs"
const F = "gates/style-parity-baseline.json"
export default {
  id: "style-parity-recorded-value-drift", gate: "style-parity", files: [F],
  why: "a cell that is already on the baseline starts differing by a different amount",
  apply() {
    const b = JSON.parse(readFileSync(F, "utf8"))
    if (!b.cells.length) throw new Error(`${F} has no recorded cells to perturb`)
    if (typeof b.cells[0] === "string") throw new Error(`${F} still records bare ids — re-record it with values`)
    b.cells[0].shadless = "999px /* mutation */"
    writeFileSync(F, JSON.stringify(b, null, 1) + "\n")
  },
}
