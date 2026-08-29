// A per-component stylesheet that only compiles as part of the full product
// entry is a file consumers cannot import alone (ed1bef4).
import { edit } from "./_util.mjs"
const F = "dist/css/switch.css"
export default {
  id: "consumer-sim-unknown-utility", gate: "consumer-sim", files: [F],
  why: "a component stylesheet references a utility that does not exist",
  apply() { edit(F, (s) => s.replace("@apply ", "@apply mutation-not-a-real-utility ")) },
}
