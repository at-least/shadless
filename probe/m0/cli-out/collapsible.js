"use client";
import { Collapsible as CollapsiblePrimitive } from "radix-ui";
function Collapsible({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(CollapsiblePrimitive.Root, { "data-slot": "collapsible", ...props });
}
function CollapsibleTrigger({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CollapsiblePrimitive.CollapsibleTrigger,
    {
      "data-slot": "collapsible-trigger",
      ...props
    }
  );
}
function CollapsibleContent({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CollapsiblePrimitive.CollapsibleContent,
    {
      "data-slot": "collapsible-content",
      ...props
    }
  );
}
export {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger
};
