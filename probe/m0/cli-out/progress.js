"use client";
import * as React from "react";
import { Progress as ProgressPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
function Progress({
  className,
  value,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ProgressPrimitive.Root,
    {
      "data-slot": "progress",
      className: cn(
        "bg-muted h-1 rounded-full relative flex w-full items-center overflow-x-hidden",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      ProgressPrimitive.Indicator,
      {
        "data-slot": "progress-indicator",
        className: "bg-primary size-full flex-1 transition-all",
        style: { transform: `translateX(-${100 - (value || 0)}%)` }
      }
    )
  );
}
export {
  Progress
};
