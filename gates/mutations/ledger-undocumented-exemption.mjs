// A new exemption appearing in a source with nobody writing down why.
import { edit } from "./_util.mjs"
const F = "probes/out/upstream-payload/exemptions.json"
export default {
  id: "ledger-undocumented-exemption", gate: "ledger", files: [F],
  why: "a golden exemption exists in the source with no ledger entry",
  apply() {
    edit(F, (s) => {
      const j = JSON.parse(s)
      j.examples["__mutation-demo"] = { reason: "mutation: an undocumented brand-new reason" }
      return JSON.stringify(j, null, 1) + "\n"
    })
  },
}
