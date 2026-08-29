// The oracle build used to SURVIVE a broken example: a render failure
// printed `KEEP …` and the run then rewrote docs/example-oracle.json to
// list only the pages that happened to render. One bad run shrank the
// owned set to zero pages, and every downstream gate that counts from the
// manifest (coverage, overlay, demo-parity, example-gate) went green over
// an empty surface. A failing example must stop the build.
import { readFileSync, existsSync } from "node:fs"
import { edit } from "./_util.mjs"

const EXAMPLES = ".upstream/shadcn-ui/apps/v4/examples/radix"
let target
export default {
  id: "example-oracle-render-failure", gate: "example-oracle",
  // the target is whichever owned page still has an upstream example, so a
  // re-pin that retires one file does not silently disarm this mutation
  files: () => {
    const owned = JSON.parse(readFileSync("docs/example-oracle.json", "utf8"))
    const t = owned.find((o) => existsSync(`${EXAMPLES}/${o.name}.tsx`))
    if (!t) throw new Error(`no page in docs/example-oracle.json has an upstream example under ${EXAMPLES}`)
    target = `${EXAMPLES}/${t.name}.tsx`
    return [target]
  },
  why: "an upstream example stops rendering — the build must fail instead of dropping that page from the ownership manifest",
  apply() {
    edit(target, (s) => s.replace(/(export default function \w+\([^)]*\)\s*\{)/,
      '$1\n  throw new Error("example-oracle mutation: render failure")'))
  },
}
