// Per-component contract: toggle-group (Wave C, trivial runtime, type=single)
export default {
  name: "toggle-group",

  usage: `
React.createElement(ToggleGroup, { type: "single", id: "grp" },
  React.createElement(ToggleGroupItem, { id: "gi1", value: "a" }, "A"),
  React.createElement(ToggleGroupItem, { id: "gi2", value: "b" }, "B"),
)
`,

  imports: `
import { ToggleGroup, ToggleGroupItem } from "@/registry/bases/radix/ui/toggle-group";
`,

  slots: ["toggle-group", "toggle-group-item"],

  // radix runtime style (outline:none stripped by normalizer; --gap left)
  ignoreAttrs: { "toggle-group": ["style"] },

  scenarios: [
    "click:#gi1",
    "click:#gi1+click:#gi2",
    "click:#gi1+click:#gi1", // deselect active item
    "focus:#grp+key:ArrowRight", // arrow from group focus lands on LAST item
    "click:#gi1+key:ArrowRight", // arrow from active item moves focus only
    "focus:#grp+key:ArrowRight+key:Space", // Space selects the focused item
,
    "focus:#grp+key:Home",
    "focus:#grp+key:End",
  ],

  stateProbe: `
var g = function (id) {
  var el = document.getElementById(id);
  return el.getAttribute("data-state") + "/" + el.getAttribute("aria-checked") + "/" + el.getAttribute("tabindex");
};
g("gi1") + "|" + g("gi2") + "|ae=" + (document.activeElement.id || document.activeElement.tagName);
`,

  shadlessPage: "probes/t7/toggle-group.html",
}
