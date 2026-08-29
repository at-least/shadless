
import React from "react";
import { createRoot } from "react-dom/client";
import { Avatar, AvatarImage, AvatarFallback } from "@/registry/new-york-v4/ui/avatar";
import { Progress } from "@/registry/new-york-v4/ui/progress";
import { AspectRatio } from "@/registry/new-york-v4/ui/aspect-ratio";
createRoot(document.getElementById("root")).render(React.createElement("div", null,
  React.createElement(Avatar, { id: "av1" },
    React.createElement(AvatarImage, { id: "avimg", src: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16'%3E%3Crect width='16' height='16' fill='red'/%3E%3C/svg%3E" }),
    React.createElement(AvatarFallback, { id: "avfb" }, "CN"),
  ),
  React.createElement(Avatar, { id: "av2" },
    React.createElement(AvatarImage, { id: "avimg2", src: "file:///nonexistent.png" }),
    React.createElement(AvatarFallback, { id: "avfb2" }, "XX"),
  ),
  React.createElement(Progress, { id: "pg", value: 42 }),
  React.createElement(AspectRatio, { id: "ar", ratio: 16/9 }, "Inside"),
));
