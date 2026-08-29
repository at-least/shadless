// One missing newline in a transform once glued a heading into the previous
// paragraph and silently removed it from 51 pages (144e820-era). Render checks
// and console checks cannot see content loss; only the mdx compare can.
import { edit } from "./_util.mjs"
const F = "docs/site/accordion.html"
export default {
  id: "docs-fidelity-drop-heading", gate: "docs-fidelity", files: [F],
  why: "a built page silently loses a heading its mdx source has",
  apply() { edit(F, (s) => s.replace('<h2 id="installation">', '<h2 id="installation-mutated">')) },
}
