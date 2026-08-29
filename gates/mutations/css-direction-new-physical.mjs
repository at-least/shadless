// Physical reading-direction utilities are inventoried against a baseline: a
// new one means upstream moved the RTL story and RTL pages need a look.
import { edit } from "./_util.mjs"
const F = "dist/shadless.css"
export default {
  id: "css-direction-new-physical", gate: "css-direction", files: [F],
  why: "a new physical (non-logical) direction utility enters the emitted css",
  apply() { edit(F, (s) => s.replace("@apply ", "@apply right-[13px] ")) },
}
