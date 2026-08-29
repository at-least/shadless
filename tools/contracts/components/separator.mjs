// Per-component contract: separator (Wave C, zero JS)
export default {
  name: "separator",
  usage: `
React.createElement(Separator, null)
`,
  imports: `
import { Separator } from "@/registry/bases/radix/ui/separator";
`,
  slots: ["separator"],
  scenarios: [],
  stateProbe: `
var s = document.querySelector("[data-slot=separator]");
s ? s.tagName.toLowerCase() + "|" + (s.getAttribute("role") || "norole") + "|" + s.getAttribute("data-orientation") : "absent";
`,
  shadlessPage: "probes/t7/separator.html",
}
