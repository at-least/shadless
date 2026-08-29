// f4759ef: contracts click FIXTURES, golden compares SNAPSHOTS, smoke listens
// to the CONSOLE — the dead-button bug lived in the gap where nobody asked
// whether the shipped page RESPONDS.
import { edit } from "./_util.mjs"
const F = "docs/site/components/dialog.html"
export default {
  id: "interactivity-strip-script", gate: "interactivity-sweep", files: [F],
  why: "an interactive example ships without its behavior — a dead button",
  apply() { edit(F, (s) => s.replace(/<script src="[^"]*\/js\/dialog\.js"><\/script>/, "")) },
}
