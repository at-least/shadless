// A per-component stylesheet that silently loses a utility: the slot-only
// path diverges from React while every DOM-structural gate stays green.
import { edit } from "./_util.mjs"
const F = "dist/css/badge.css"
export default {
  id: "path-parity-drop-utility", gate: "path-parity", files: [F],
  why: "badge's slot rule drops its padding — css-import consumers get an unpadded badge",
  apply() { edit(F, (s) => s.replace(/ px-[\d.]+/, "")) },
}
