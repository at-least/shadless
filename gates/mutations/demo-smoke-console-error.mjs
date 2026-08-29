// demo-smoke's whole job is "no page throws". A page that throws on load looks
// fine in a screenshot and in any DOM-structural comparison.
import { edit } from "./_util.mjs"
const F = "dist/components/accordion.html"
export default {
  id: "demo-smoke-console-error", gate: "demo-smoke", files: [F],
  why: "a shipped demo page throws on load",
  apply() { edit(F, (s) => s.replace("</body>", '<script>throw new Error("mutation: page error")</script></body>')) },
}
