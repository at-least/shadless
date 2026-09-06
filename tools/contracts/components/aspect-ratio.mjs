// Per-component contract: aspect-ratio (Wave C, zero JS)
export default {
  usage: `
React.createElement(AspectRatio, { id: "ar", ratio: 16/9 }, "Inside")
`,
  imports: `
import { AspectRatio } from "@/registry/bases/radix/ui/aspect-ratio";
`,
  slots: ["aspect-ratio"],
  scenarios: [],
  stateProbe: `
var a = document.querySelector("[data-slot=aspect-ratio]");
a ? a.getAttribute("style") : "absent";
`,
  shadlessPage: "probes/t7/aspect-ratio.html",
}
