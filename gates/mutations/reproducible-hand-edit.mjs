// A hand-edit to a generated file. The old hook guessed at this from which
// paths were staged together; the gate simply rebuilds and compares.
import { edit } from "./_util.mjs"
const F = "dist/components/badge.html"
export default {
  id: "reproducible-hand-edit", gate: "reproducible", files: [F],
  why: "a shipped file differs from what the pipeline produces",
  apply() { edit(F, (s) => s.replace("</body>", "<!-- hand edit --></body>")) },
}
