// Per-component contract: radio-group (Wave C, trivial runtime)
export default {
  name: "radio-group",
  usage: `
React.createElement(RadioGroup, { id: "rg" },
  React.createElement(RadioGroupItem, { id: "ri1", value: "a" }),
  React.createElement(RadioGroupItem, { id: "ri2", value: "b" }),
)`,
  imports: `
import { RadioGroup, RadioGroupItem } from "@/registry/bases/radix/ui/radio-group";
`,
  slots: ["radio-group", "radio-group-item", "radio-group-indicator"],
  scenarios: [
    "click:#ri1",
    "click:#ri1+click:#ri2",
    "click:#ri1+click:#ri1", // no deselect
    "focus:#rg+key:ArrowRight", // no selection yet: arrow checks LAST item (radix quirk)
    "click:#ri1+key:ArrowRight", // selection exists: arrow moves focus only
  ],
  stateProbe: `
var g = function (id) { var e = document.getElementById(id);
  return e.getAttribute("aria-checked") + "/" + e.getAttribute("data-state") + "/" + e.getAttribute("tabindex"); };
var i = document.querySelector("[data-slot=radio-group-indicator]");
g("ri1") + "|" + g("ri2") + "|ind=" + (i ? i.getAttribute("data-state") : "absent")
  + "|ae=" + (document.activeElement.id || document.activeElement.tagName);
`,
  shadlessPage: "probes/t7/radio-group.html",
}
