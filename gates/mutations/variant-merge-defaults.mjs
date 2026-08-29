// The FINDING bug, reproduced exactly: merge the default variant's utilities
// into the bare slot rule. CSS has no un-apply, so every variant that does not
// restate the same group inherits the default look — ghost/outline buttons
// rendered invisible for a downstream consumer (0dd7391, 30 broken cells).
import { edit } from "./_util.mjs"
const F = "dist/css/button.css"
export default {
  id: "variant-merge-defaults", gate: "path-parity", files: [F],
  why: "default-variant utilities cascade into non-default variants",
  apply() {
    edit(F, (s) => s.replace(
      '[data-slot="button"] { @apply ',
      '[data-slot="button"] { @apply bg-primary text-primary-foreground '))
  },
}
