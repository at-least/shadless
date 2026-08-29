// The incident: out.css committed without the interactive components' rules.
// Reproduce the smallest version — one component's slot rules gone.
import { edit } from "./_util.mjs"
const F = "dist/out.css"
export default {
  id: "dist-complete-drop-component", gate: "dist-complete", files: [F],
  why: "dist/out.css loses a component's slot rules (a partial-build out.css got committed)",
  apply() { edit(F, (s) => s.replaceAll('[data-slot="dialog-content"]', '[data-slot="dialog-MUTATED"]')) },
}
