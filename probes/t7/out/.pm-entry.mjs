
import React from "react";
import { createRoot } from "react-dom/client";
import { ToggleGroup, ToggleGroupItem } from "@/registry/new-york-v4/ui/toggle-group";
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@/registry/new-york-v4/ui/accordion";
createRoot(document.getElementById("root")).render(React.createElement("div", null,
  React.createElement(ToggleGroup, { type: "multiple", id: "grp" },
    React.createElement(ToggleGroupItem, { id: "gi1", value: "a" }, "A"),
    React.createElement(ToggleGroupItem, { id: "gi2", value: "b" }, "B"),
  ),
  React.createElement(Accordion, { type: "multiple", id: "ac" },
    React.createElement(AccordionItem, { value: "x" },
      React.createElement(AccordionTrigger, { id: "d1-at1" }, "First"),
      React.createElement(AccordionContent, null, "Body 1"),
    ),
    React.createElement(AccordionItem, { value: "y" },
      React.createElement(AccordionTrigger, { id: "d1-at2" }, "Second"),
      React.createElement(AccordionContent, null, "Body 2"),
    ),
  ),
));
