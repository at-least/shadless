import { edit } from "./_util.mjs"
const F = "docs/site/accordion.html"
export default {
  id: "docs-dangling-link", gate: "docs-links", files: [F],
  why: "a built page links to a route that does not exist",
  apply() { edit(F, (s) => s.replace("</body>", '<a href="./this-page-does-not-exist.html">mutation</a></body>')) },
}
