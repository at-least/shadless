// The pin must describe the checkout it was taken from. Upgrade tools write
// pin.json directly, so a tag(old)/commit(new) record could ship silently.
import { edit } from "./_util.mjs"
export default {
  id: "pin-commit-drift", gate: "pin", files: ["src/registry/pin.json"],
  why: "pin.json records a commit the .upstream checkout is not sitting at",
  apply() { edit("src/registry/pin.json", (s) => s.replace(/"commit": "[0-9a-f]{40}"/, '"commit": "0000000000000000000000000000000000000000"')) },
}
