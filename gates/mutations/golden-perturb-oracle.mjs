// Hop 1 of the dual hop: the local React oracle render must equal what
// ui.shadcn.com actually serves. Perturbing the recorded snapshot must break it.
import { edit } from "./_util.mjs"
const F = "src/registry/upstream-snapshot/accordion.json"
export default {
  id: "golden-perturb-oracle", gate: "golden-gate", files: [F],
  why: "the local oracle render stops matching the recorded live-site snapshot",
  apply() { edit(F, (s) => s.replace('data-slot=\\"accordion\\"', 'data-slot=\\"accordion-mutated\\"')) },
}
