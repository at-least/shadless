"use client";
import * as React from "react";
import { ScrollArea as ScrollAreaPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
function ScrollArea({
  className,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ScrollAreaPrimitive.Root,
    {
      "data-slot": "scroll-area",
      className: cn("relative", className),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      ScrollAreaPrimitive.Viewport,
      {
        "data-slot": "scroll-area-viewport",
        className: "size-full rounded-[inherit] transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1"
      },
      children
    ),
    /* @__PURE__ */ React.createElement(ScrollBar, null),
    /* @__PURE__ */ React.createElement(ScrollAreaPrimitive.Corner, null)
  );
}
function ScrollBar({
  className,
  orientation = "vertical",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ScrollAreaPrimitive.ScrollAreaScrollbar,
    {
      "data-slot": "scroll-area-scrollbar",
      "data-orientation": orientation,
      orientation,
      className: cn(
        "data-horizontal:h-2.5 data-horizontal:flex-col data-horizontal:border-t data-horizontal:border-t-transparent data-vertical:h-full data-vertical:w-2.5 data-vertical:border-l data-vertical:border-l-transparent flex touch-none p-px transition-colors select-none",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      ScrollAreaPrimitive.ScrollAreaThumb,
      {
        "data-slot": "scroll-area-thumb",
        className: "rounded-full relative flex-1 bg-border"
      }
    )
  );
}
export {
  ScrollArea,
  ScrollBar
};
