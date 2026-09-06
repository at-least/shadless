// Per-component contract: toggle-group type=multiple (Wave C audit fix)
export default {
  usage: `
React.createElement(ToggleGroup, { type: "multiple", id: "grp" },
  React.createElement(ToggleGroupItem, { id: "gi1", value: "a" }, "A"),
  React.createElement(ToggleGroupItem, { id: "gi2", value: "b" }, "B"),
)`,
  imports: `
import { ToggleGroup, ToggleGroupItem } from "@/registry/bases/radix/ui/toggle-group";
`,
  slots: ["toggle-group", "toggle-group-item"],
  ignoreAttrs: { "toggle-group": ["style"] },
  scenarios: [
    "click:#gi1",
    "click:#gi1+click:#gi2", // independent: both stay on
    "click:#gi1+click:#gi2+click:#gi2", // deselect one, other stays
  ],
  stateProbe: `
var g = function (id) { var e = document.getElementById(id);
  return e.getAttribute("data-state") + "/" + (e.getAttribute("aria-pressed") || e.getAttribute("aria-checked")) + "/" + e.getAttribute("tabindex"); };
g("gi1") + "|" + g("gi2") + "|ae=" + (document.activeElement.id || document.activeElement.tagName);
`,
  shadlessPage: "probes/t7/toggle-group-multiple.html",
}
