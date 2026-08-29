// Budgets are the ratchet. tools/style-parity.mjs recorded counts but compared
// only presence, so tracked numbers could grow forever while the gate stayed
// green. This proves the ratchet actually bites.
import { edit } from "./_util.mjs"
const F = "probes/out/upstream-payload/exemptions.json"
export default {
  id: "ledger-budget-exceeded", gate: "ledger", files: [F],
  why: "the count of golden-exempt demos grows past its recorded budget",
  apply() {
    edit(F, (s) => {
      const j = JSON.parse(s)
      // reuse an EXISTING reason so only the BUDGET moves, not the id set
      const reason = Object.values(j.examples)[0].reason
      j.examples["__mutation-extra-demo"] = { reason }
      return JSON.stringify(j, null, 1) + "\n"
    })
  },
}
