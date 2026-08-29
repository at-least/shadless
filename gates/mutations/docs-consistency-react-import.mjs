// bc755b5: a no-React product must not teach React imports. The detector
// exists so a NEW upstream mdx shape the transforms miss lands loudly.
import { edit } from "./_util.mjs"
const F = "docs/site/accordion.html"
export default {
  id: "docs-consistency-react-import", gate: "docs-consistency", files: [F],
  why: "a built page teaches `@/components/ui` again after an upstream mdx reshape",
  apply() { edit(F, (s) => s.replace("</body>", '<pre><code>import { Accordion } from "@/components/ui/accordion"</code></pre></body>')) },
}
