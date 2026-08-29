// Losing a contract def silently removes behavioral + computed coverage for
// a component. The matrix must notice.
import { readFileSync, unlinkSync } from "node:fs"
const F = "tools/contracts/components/dialog.mjs"
export default {
  id: "coverage-drop-contract", gate: "coverage", files: [F],
  why: "a contract def disappears and its component's cells go uncovered",
  apply() { readFileSync(F); unlinkSync(F) },
}
