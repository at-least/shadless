
import React from "react";
import { createRoot } from "react-dom/client";
import { Toggle } from "@/registry/new-york-v4/ui/toggle";
import { ToggleGroup, ToggleGroupItem } from "@/registry/new-york-v4/ui/toggle-group";
import { Label } from "@/registry/new-york-v4/ui/label";
import { Input } from "@/registry/new-york-v4/ui/input";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement("div", null,
  React.createElement(Toggle, { id: "tg1" }, "Bold"),
  React.createElement(ToggleGroup, { type: "single", id: "grp" },
    React.createElement(ToggleGroupItem, { id: "gi1", value: "a" }, "A"),
    React.createElement(ToggleGroupItem, { id: "gi2", value: "b" }, "B"),
  ),
  React.createElement(Label, { htmlFor: "in1" }, "Name"),
  React.createElement(Input, { id: "in1" }),
));
