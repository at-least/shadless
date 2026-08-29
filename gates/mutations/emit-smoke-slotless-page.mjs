// An emitted page that renders nothing recognisable. The emitter has shipped
// literal <ChevronLeftIcon> and <button placeholder> tags before (Wave H).
import { edit } from "./_util.mjs"
const F = "dist/components/badge.html"
export default {
  id: "emit-smoke-slotless-page", gate: "emit-smoke", files: [F],
  why: "an emitted static page loses every data-slot — it renders, but as nothing",
  apply() { edit(F, (s) => s.replaceAll("data-slot=", "data-mutated=")) },
}
