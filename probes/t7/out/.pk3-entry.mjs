
import React from "react";
import { createRoot } from "react-dom/client";
import { RadioGroup, RadioGroupItem } from "@/registry/new-york-v4/ui/radio-group";
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@/registry/new-york-v4/ui/accordion";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement("div", null,
  React.createElement(RadioGroup, { id: "rg" },
    React.createElement(RadioGroupItem, { id: "ri1", value: "a" }),
    React.createElement(RadioGroupItem, { id: "ri2", value: "b" }),
  ),
  React.createElement(Accordion, { type: "single", id: "ac", collapsible: true },
    React.createElement(AccordionItem, { value: "x", id: "ai1" },
      React.createElement(AccordionTrigger, { id: "at1" }, "First"),
      React.createElement(AccordionContent, { id: "acc1" }, "Body 1"),
    ),
    React.createElement(AccordionItem, { value: "y", id: "ai2" },
      React.createElement(AccordionTrigger, { id: "at2" }, "Second"),
      React.createElement(AccordionContent, { id: "acc2" }, "Body 2"),
    ),
  ),
));
