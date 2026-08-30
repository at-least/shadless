// pipeline/nodes.go is GENERATED from gates/registry.mjs. A hand edit (or a
// registry change without regenerating) would make the runner execute a
// different graph from the one the registry and meta prove.
import { edit } from "./_util.mjs"
const F = "pipeline/nodes.go"
export default {
  id: "pipeline-drift", gate: "pipeline-sync", files: [F],
  why: "the runner's graph silently diverges from the registry",
  apply() {
    edit(F, (s) => s.replace('Kind: "gate"', 'Kind: "build"'))
  },
}
