// Per-component contract: accordion (Wave C, trivial runtime, single+collapsible)
export default {
  name: "accordion",
  usage: `
React.createElement(Accordion, { type: "single", id: "ac", collapsible: true },
  React.createElement(AccordionItem, { value: "x" },
    React.createElement(AccordionTrigger, { id: "d1-at1" }, "First"),
    React.createElement(AccordionContent, null, "Body 1"),
  ),
  React.createElement(AccordionItem, { value: "y" },
    React.createElement(AccordionTrigger, { id: "d1-at2" }, "Second"),
    React.createElement(AccordionContent, null, "Body 2"),
  ),
)`,
  imports: `
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@/registry/bases/radix/ui/accordion";
`,
  slots: ["accordion", "accordion-item", "accordion-trigger", "accordion-content"],
  scenarios: [
    "click:#d1-at1",
    "click:#d1-at1+click:#d1-at2", // single: opening y closes x
    "click:#d1-at2+click:#d1-at2", // collapsible: can close
    "click:#d1-at1+key:ArrowDown", // focus moves to next trigger, x stays open
    "focus:#d1-at1+key:End",
    "focus:#d1-at2+key:Home",
  ],
  // text on content-bearing slots: radix Presence UNMOUNTS closed content,
  // shadless keeps it in DOM hidden — textContent exposes that recorded
  // structural difference, not a content drift (trigger texts still compared)
  ignoreAttrs: {
    "accordion": ["text"],
    "accordion-item": ["text"],
    "accordion-content": ["text"],
  },
  stateProbe: `
var s = function (sel) { var e = document.querySelector(sel + " [data-slot=accordion-trigger], " + sel);
  var t = document.querySelector(sel); return t ? t.getAttribute("data-state") : "?"; };
var t1 = document.getElementById("d1-at1"), t2 = document.getElementById("d1-at2");
var c1 = t1.closest("[data-slot=accordion-item]").querySelector("[data-slot=accordion-content]");
var c2 = t2.closest("[data-slot=accordion-item]").querySelector("[data-slot=accordion-content]");
var f = function (t, c) { return t.getAttribute("data-state") + "/" + t.getAttribute("aria-expanded")
  + "/" + c.getAttribute("data-state") + "/" + (c.hasAttribute("hidden") ? "h" : "s"); };
f(t1, c1) + "|" + f(t2, c2) + "|ae=" + (document.activeElement.id || document.activeElement.tagName);
`,
  shadlessPage: "probes/t7/accordion.html",
}
