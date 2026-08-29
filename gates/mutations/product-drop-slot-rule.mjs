// The no-build artifact must actually carry the component rules; a compile
// that silently drops them still produces a plausible-looking css file.
import { edit } from "./_util.mjs"
const F = "dist/shadless.full.css"
export default {
  id: "product-drop-slot-rule", gate: "product-verify", files: [F],
  why: "the product build loses a component's slot rules",
  apply() { edit(F, (s) => s.replaceAll('[data-slot="badge"]', '[data-slot="badge-MUTATED"]')) },
}
