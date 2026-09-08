"use client";
import * as React from "react";
import { HoverCard as HoverCardPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
function HoverCard({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(HoverCardPrimitive.Root, { "data-slot": "hover-card", ...props });
}
function HoverCardTrigger({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(HoverCardPrimitive.Trigger, { "data-slot": "hover-card-trigger", ...props });
}
function HoverCardContent({
  className,
  align = "center",
  sideOffset = 4,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(HoverCardPrimitive.Portal, { "data-slot": "hover-card-portal" }, /* @__PURE__ */ React.createElement(
    HoverCardPrimitive.Content,
    {
      "data-slot": "hover-card-content",
      align,
      sideOffset,
      className: cn(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground w-64 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 origin-(--radix-hover-card-content-transform-origin) outline-hidden",
        className
      ),
      ...props
    }
  ));
}
export {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger
};
