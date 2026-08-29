// "same DOM + same css => same styles" was an inference. A fixture whose
// computed style silently differs from the React oracle must fail — and
// with the per-cell ratchet, it must fail even for a component that already
// carries recorded drift elsewhere.
import { edit } from "./_util.mjs"
const F = "tools/contracts/out/dialog/shadless.html"
export default {
  id: "style-parity-perturb-padding", gate: "style-parity", files: [F],
  why: "a shipped fixture's computed padding diverges from the oracle on one slot",
  apply() {
    edit(F, (s) => s.replace('data-slot="dialog-content"', 'data-slot="dialog-content" style="padding-top:37px"'))
  },
}
