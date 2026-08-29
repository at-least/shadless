// A hand-written file whose upstream input changed since it was written.
// Simulated from the other side: the manifest records a different hash than
// the pinned upstream produces — exactly what a re-pin does.
import { edit } from "./_util.mjs"
const F = "overlays/manifest.json"
export default {
  id: "overlay-stale-authored", gate: "overlay", files: [F],
  why: "the upstream input a kernel fixture was written against has changed",
  apply() {
    edit(F, (s) => {
      const j = JSON.parse(s)
      j.units["behavior:sheet"].hash = "0".repeat(64)
      return JSON.stringify(j, null, 2) + "\n"
    })
  },
}
