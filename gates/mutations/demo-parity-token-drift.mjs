// A theme token that drifts from upstream: every element painted with it
// differs on the shipped demo pages while the DOM stays byte-identical. (A
// slot-rule mutation would NOT do here — the demo DOM carries React's inline
// utilities, which shadow slot rules for the properties they set.)
import { edit } from "./_util.mjs"
const F = "dist/out.css"
export default {
  id: "demo-parity-token-drift", gate: "demo-parity", files: [F],
  why: "the --primary token in the shipped stylesheet no longer matches upstream's",
  apply() { edit(F, (s) => s.replace(/--primary: oklch\([^)]*\);/, "--primary: oklch(0.5 0.2 250);")) },
}
