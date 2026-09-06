// Per-component contract: toggle (Wave C, trivial runtime)
export default {

  usage: `
React.createElement(Toggle, { id: "tg1" }, "Bold")
`,

  imports: `
import { Toggle } from "@/registry/bases/radix/ui/toggle";
`,

  slots: ["toggle"],

  scenarios: ["click:#tg1", "focus:#tg1+key:Space", "focus:#tg1+key:Enter", "click:#tg1+click:#tg1"],

  stateProbe: `
var t = document.getElementById("tg1");
t.getAttribute("aria-pressed") + "|" + t.getAttribute("data-state");
`,

  shadlessPage: "probes/t7/toggle.html",
}
