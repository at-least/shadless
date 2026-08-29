
import React from "react";
import { createRoot } from "react-dom/client";
import { RadioGroup, RadioGroupItem } from "@/registry/new-york-v4/ui/radio-group";
createRoot(document.getElementById("root")).render(
  React.createElement(RadioGroup, { id: "rg" },
    React.createElement(RadioGroupItem, { id: "ri1", value: "a" })));
