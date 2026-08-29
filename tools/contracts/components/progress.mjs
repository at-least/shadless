// Per-component contract: progress (Wave C, zero JS)
export default {
  name: "progress",
  usage: `
React.createElement(Progress, { id: "pg", value: 42 })
`,
  imports: `
import { Progress } from "@/registry/bases/radix/ui/progress";
`,
  slots: ["progress", "progress-indicator"],
  scenarios: [],
  stateProbe: `
var p = document.querySelector("[data-slot=progress]");
var i = document.querySelector("[data-slot=progress-indicator]");
p.getAttribute("data-state") + "|" + p.getAttribute("aria-valuemax") + "|" + (i ? i.getAttribute("style") : "absent");
`,
  shadlessPage: "probes/t7/progress.html",
}
