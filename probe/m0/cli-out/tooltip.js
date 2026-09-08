"use client";
import * as React from "react";
import { Tooltip as TooltipPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
function TooltipProvider({
  delayDuration = 0,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    TooltipPrimitive.Provider,
    {
      "data-slot": "tooltip-provider",
      delayDuration,
      ...props
    }
  );
}
function Tooltip({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(TooltipPrimitive.Root, { "data-slot": "tooltip", ...props });
}
function TooltipTrigger({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(TooltipPrimitive.Trigger, { "data-slot": "tooltip-trigger", ...props });
}
function TooltipContent({
  className,
  sideOffset = 0,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(TooltipPrimitive.Portal, null, /* @__PURE__ */ React.createElement(
    TooltipPrimitive.Content,
    {
      "data-slot": "tooltip-content",
      sideOffset,
      className: cn(
        "data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm z-50 w-fit max-w-xs origin-(--radix-tooltip-content-transform-origin) bg-foreground text-background",
        className
      ),
      ...props
    },
    children,
    /* @__PURE__ */ React.createElement(TooltipPrimitive.Arrow, { className: "size-2.5 rotate-45 rounded-[2px] z-50 translate-y-[calc(-50%_-_2px)] bg-foreground fill-foreground" })
  ));
}
export {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger
};
