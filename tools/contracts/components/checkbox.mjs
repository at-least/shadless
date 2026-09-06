// Per-component contract: checkbox (Wave C, trivial runtime)
export default {

  usage: `
React.createElement(Checkbox, { id: "c1" })
`,

  imports: `
import { Checkbox } from "@/registry/bases/radix/ui/checkbox";
`,

  slots: ["checkbox", "checkbox-indicator"],

  // NB: radix Checkbox ignores Enter (Space only) — measured 2026-08-22
  scenarios: ["click:#c1", "focus:#c1+key:Space", "click:#c1+click:#c1"],

  stateProbe: `
var c = document.getElementById("c1");
var i = document.querySelector("[data-slot=checkbox-indicator]");
c.getAttribute("aria-checked") + "|" + c.getAttribute("data-state") + "|" + (i ? i.getAttribute("data-state") : "absent");
`,

  shadlessPage: "probes/t7/checkbox.html",
}
