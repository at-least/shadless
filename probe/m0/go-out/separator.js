"use client";
import * as React from "react";
import { Separator as SeparatorPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
function Separator({
  className,
  orientation = "horizontal",
  decorative = true,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SeparatorPrimitive.Root,
    {
      "data-slot": "separator",
      decorative,
      orientation,
      className: cn(
        "shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch",
        className
      ),
      ...props
    }
  );
}
export {
  Separator
};
