
import React from "react";
import { createRoot } from "react-dom/client";
import { Button } from "@/registry/new-york-v4/ui/button";
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/registry/new-york-v4/ui/dialog";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement(Dialog, { open: true },
  React.createElement(Button, null, "Button"),
  " ",
  React.createElement(DialogTrigger, null, "Open dialog"),
  React.createElement(DialogContent, null,
    React.createElement(DialogHeader, null,
      React.createElement(DialogTitle, null, "Are you absolutely sure?"),
      React.createElement(DialogDescription, null, "This action cannot be undone."),
    ),
  ),
));
