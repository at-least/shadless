// d62284f: the site "looked different from upstream" while every class,
// colour and size matched. Chrome-level parity needs its own assertions.
import { edit } from "./_util.mjs"
const F = "docs/site/site.css"
export default {
  id: "docs-upstream-chrome-drift", gate: "docs-upstream", files: [F],
  why: "the site chrome stops matching the ui.shadcn.com neutral background",
  apply() { edit(F, (s) => s.replace("oklch(1 0 0)", "oklch(0.97 0.02 250)")) },
}
