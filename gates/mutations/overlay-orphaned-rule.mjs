// A conversion rule whose anchor no longer exists: DEFAULT_CONTENT keyed on
// a component fn upstream stopped exporting. The old table validated keys
// only when the emitter ran; the overlay gate checks it in the fast tier.
import { replaceOnce } from "./_util.mjs"
const F = "src/emitter/index.mjs"
export default {
  id: "overlay-orphaned-rule", gate: "overlay", files: [F],
  why: "DEFAULT_CONTENT carries an entry for a fn the component no longer exports",
  apply() { replaceOnce(F, "  badge: { Badge: \"Badge\" },", "  badge: { Badge: \"Badge\", BadgeGoneUpstream: \"x\" },") },
}
