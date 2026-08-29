// The regression that created the unit gate: a "dead code" delete in a pure
// helper passed `node --check` and only surfaced minutes later downstream.
import { replaceOnce } from "./_util.mjs"
export default {
  id: "unit-break-pure-fn", gate: "unit", files: ["src/emitter/css.mjs"],
  why: "a pure helper (splitMarkers) stops separating marker classes from utilities",
  apply() {
    replaceOnce("src/emitter/css.mjs",
      "apply: toks.filter((t) => !MARKER.test(t) &&",
      "apply: toks.filter((t) => (true) &&")
  },
}
