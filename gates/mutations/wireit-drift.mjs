// package.json's wireit block is GENERATED from gates/registry.mjs. A hand
// edit (or a registry change without --write) would make `npm run w:*` run
// a different graph from the one the registry and meta prove.
import { edit } from "./_util.mjs"
const F = "package.json"
export default {
  id: "wireit-drift", gate: "wireit-sync", files: [F],
  why: "the incremental runner's graph silently diverges from the registry",
  apply() {
    edit(F, (s) => {
      const j = JSON.parse(s)
      j.wireit["w:unit"].command = "true"
      return JSON.stringify(j, null, 2) + "\n"
    })
  },
}
