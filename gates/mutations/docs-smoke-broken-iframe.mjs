import { edit } from "./_util.mjs"
const F = "docs/site/accordion.html"
export default {
  id: "docs-smoke-broken-iframe", gate: "docs-smoke", files: [F],
  why: "a preview iframe points at a page that does not exist",
  apply() { edit(F, (s) => s.replace('src="components/accordion-demo.html"', 'src="components/mutation-missing.html"')) },
}
