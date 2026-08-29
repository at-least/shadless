// Per-component contract: label (Wave C, native `for` — zero JS)
export default {
  name: "label",

  usage: `
React.createElement("div", null,
  React.createElement(Label, { htmlFor: "in1" }, "Name"),
  React.createElement(Input, { id: "in1" }),
  React.createElement(Label, { id: "lb2" }, React.createElement(Input, { id: "in2" })),
)
`,

  imports: `
import { Label } from "@/registry/bases/radix/ui/label";
import { Input } from "@/registry/bases/radix/ui/input";
`,

  slots: ["label", "&input#in1"],

  scenarios: ["click:label", "focus:label+key:Tab", "js-click:#lb2"],

  stateProbe: `
"ae=" + (document.activeElement.id || document.activeElement.tagName);
`,

  shadlessPage: "probes/t7/label.html",
}
