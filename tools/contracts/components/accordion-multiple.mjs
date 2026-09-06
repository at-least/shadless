// Per-component contract: accordion type=multiple (Wave C audit fix)
export default {
  usage: `
React.createElement(Accordion, { type: "multiple", id: "ac" },
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
  // shadless carries data-type=multiple as the DOM discriminator (oracle keeps
  // it as a React prop only) — recorded difference
  ignoreAttrs: {
    "accordion": ["data-type", "text"],
    // text: radix Presence unmounts closed content, shadless keeps it
    // hidden in DOM — recorded structural difference, not content drift
    "accordion-item": ["text"],
    "accordion-content": ["text"],
  },
  scenarios: [
    "click:#d1-at1+click:#d1-at2", // both open
    "click:#d1-at1+click:#d1-at2+click:#d1-at1", // close one, other stays
  ],
  stateProbe: `
var t1 = document.getElementById("d1-at1"), t2 = document.getElementById("d1-at2");
var c1 = t1.closest("[data-slot=accordion-item]").querySelector("[data-slot=accordion-content]");
var c2 = t2.closest("[data-slot=accordion-item]").querySelector("[data-slot=accordion-content]");
var f = function (t, c) { return t.getAttribute("data-state") + "/" + c.getAttribute("data-state") + "/" + (c.hasAttribute("hidden") ? "h" : "s"); };
f(t1, c1) + "|" + f(t2, c2);
`,
  shadlessPage: "probes/t7/accordion-multiple.html",
}
