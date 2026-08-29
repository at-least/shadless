// The contract runner replays real mouse/keyboard against the SHIPPED page.
// Remove the behavior and the replay must stop matching the React oracle.
import { edit } from "./_util.mjs"
const F = "src/kernel/dialog.html"
export default {
  id: "contracts-strip-glue", gate: "contracts", files: [F],
  why: "a kernel-tier page ships without its behavior file — the oracle opens, the page does not",
  apply() { edit(F, (s) => s.replace('<script src="../../dist/js/dialog.js">', '<script data-mutated src="../../dist/js/MISSING.js">')) },
}
