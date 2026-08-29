// Wave I: the site's iframe copies drifted from dist because the mirror step
// wrote un-skinned bytes. The site tree must be the byte-exact image of dist.
import { edit } from "./_util.mjs"
const F = "docs/site/components/accordion-basic.html"
export default {
  id: "docs-consistency-site-drift", gate: "docs-consistency", files: [F],
  why: "a site demo copy drifts from the dist page it mirrors",
  apply() { edit(F, (s) => s.replace("</body>", "<!-- mutation --></body>")) },
}
