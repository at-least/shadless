// Hop 2 of the dual hop: each shipped demo page must equal a fresh oracle
// render of its upstream example.
import { edit, findFile } from "./_util.mjs"
let target
export default {
  id: "example-perturb-shipped", gate: "example-gate",
  files: () => { target = findFile("docs/demos", (s) => s.includes('data-slot="badge"')); return [target] },
  why: "a shipped demo page drifts from the React oracle render of its example",
  apply() { edit(target, (s) => s.replace('data-slot="badge"', 'data-slot="badge" data-mutation="1"')) },
}
