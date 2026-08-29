
import React from "react";
import { createRoot } from "react-dom/client";
import { ToggleGroup, ToggleGroupItem } from "@/registry/new-york-v4/ui/toggle-group";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement(ToggleGroup, { type: "single", id: "grp" },
  React.createElement(ToggleGroupItem, { id: "gi1", value: "a" }, "A"),
  React.createElement(ToggleGroupItem, { id: "gi2", value: "b" }, "B"),
));
