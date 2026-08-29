// Per-component contract: switch (Wave C, trivial runtime)
export default {
  name: "switch",

  usage: `
React.createElement(Switch, { id: "s1" })
`,

  imports: `
import { Switch } from "@/registry/bases/radix/ui/switch";
`,

  slots: ["switch", "switch-thumb"],

  scenarios: ["click:#s1", "focus:#s1+key:Space", "focus:#s1+key:Enter", "click:#s1+click:#s1"],

  stateProbe: `
var s = document.getElementById("s1");
var t = document.querySelector("[data-slot=switch-thumb]");
s.getAttribute("aria-checked") + "|" + s.getAttribute("data-state") + "|" + t.getAttribute("data-state");
`,

  shadlessPage: "probes/t7/switch.html",
}
