// Per-component contract: collapsible (Wave C, trivial runtime)
export default {
  name: "collapsible",
  usage: `
React.createElement(Collapsible, { id: "co" },
  React.createElement(CollapsibleTrigger, { id: "d1-ct" }, "Toggle"),
  React.createElement(CollapsibleContent, null, "Content"),
)`,
  imports: `
import { Collapsible, CollapsibleTrigger, CollapsibleContent } from "@/registry/bases/radix/ui/collapsible";
`,
  slots: ["collapsible", "collapsible-trigger", "collapsible-content"],
  scenarios: [
    "click:#d1-ct",
    "click:#d1-ct+click:#d1-ct",
    "focus:#d1-ct+key:Space+key:Space",
    "focus:#d1-ct+key:Enter",
  ],
  // text: radix Presence unmounts closed content, shadless keeps it hidden
  // in DOM — recorded structural difference, not content drift
  ignoreAttrs: { "collapsible": ["text"], "collapsible-content": ["text"] },
  stateProbe: `
var t = document.querySelector("[data-slot=collapsible-trigger]");
var c = document.querySelector("[data-slot=collapsible-content]");
var r = document.querySelector("[data-slot=collapsible]");
t.getAttribute("data-state") + "/" + t.getAttribute("aria-expanded") + "/" + (t.getAttribute("aria-controls") ? "ctl" : "noctl")
  + "|" + c.getAttribute("data-state") + "/" + (c.hasAttribute("hidden") ? "hidden" : "shown")
  + "|" + r.getAttribute("data-state");
`,
  shadlessPage: "probes/t7/collapsible.html",
}
