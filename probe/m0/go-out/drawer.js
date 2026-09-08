"use client";
import * as React from "react";
import { Drawer as DrawerPrimitive } from "vaul";
import { cn } from "@/registry/bases/radix/lib/utils";
function Drawer({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DrawerPrimitive.Root, { "data-slot": "drawer", ...props });
}
function DrawerTrigger({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DrawerPrimitive.Trigger, { "data-slot": "drawer-trigger", ...props });
}
function DrawerPortal({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DrawerPrimitive.Portal, { "data-slot": "drawer-portal", ...props });
}
function DrawerClose({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DrawerPrimitive.Close, { "data-slot": "drawer-close", ...props });
}
function DrawerOverlay({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    DrawerPrimitive.Overlay,
    {
      "data-slot": "drawer-overlay",
      className: cn(
        "bg-black/10 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50 data-open:animate-in data-open:fade-in-0 data-closed:animate-out data-closed:fade-out-0",
        className
      ),
      ...props
    }
  );
}
function DrawerContent({
  className,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DrawerPortal, { "data-slot": "drawer-portal" }, /* @__PURE__ */ React.createElement(DrawerOverlay, null), /* @__PURE__ */ React.createElement(
    DrawerPrimitive.Content,
    {
      "data-slot": "drawer-content",
      className: cn(
        "bg-popover text-popover-foreground flex h-auto flex-col text-sm data-[vaul-drawer-direction=bottom]:rounded-t-xl data-[vaul-drawer-direction=bottom]:border-t data-[vaul-drawer-direction=left]:rounded-r-xl data-[vaul-drawer-direction=left]:border-r data-[vaul-drawer-direction=right]:rounded-l-xl data-[vaul-drawer-direction=right]:border-l data-[vaul-drawer-direction=top]:rounded-b-xl data-[vaul-drawer-direction=top]:border-b group/drawer-content fixed z-50 data-[vaul-drawer-direction=bottom]:inset-x-0 data-[vaul-drawer-direction=bottom]:bottom-0 data-[vaul-drawer-direction=bottom]:mt-24 data-[vaul-drawer-direction=bottom]:max-h-[80vh] data-[vaul-drawer-direction=left]:inset-y-0 data-[vaul-drawer-direction=left]:left-0 data-[vaul-drawer-direction=left]:w-3/4 data-[vaul-drawer-direction=right]:inset-y-0 data-[vaul-drawer-direction=right]:right-0 data-[vaul-drawer-direction=right]:w-3/4 data-[vaul-drawer-direction=top]:inset-x-0 data-[vaul-drawer-direction=top]:top-0 data-[vaul-drawer-direction=top]:mb-24 data-[vaul-drawer-direction=top]:max-h-[80vh] data-[vaul-drawer-direction=left]:sm:max-w-sm data-[vaul-drawer-direction=right]:sm:max-w-sm",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement("div", { className: "bg-muted mt-4 h-1 w-[100px] rounded-full mx-auto hidden shrink-0 group-data-[vaul-drawer-direction=bottom]/drawer-content:block" }),
    children
  ));
}
function DrawerHeader({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "drawer-header",
      className: cn(
        "gap-0.5 p-4 md:gap-0.5 md:text-left flex flex-col group-data-[vaul-drawer-direction=bottom]/drawer-content:text-center group-data-[vaul-drawer-direction=top]/drawer-content:text-center",
        className
      ),
      ...props
    }
  );
}
function DrawerFooter({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "drawer-footer",
      className: cn("gap-2 p-4 mt-auto flex flex-col", className),
      ...props
    }
  );
}
function DrawerTitle({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    DrawerPrimitive.Title,
    {
      "data-slot": "drawer-title",
      className: cn("text-foreground text-base font-medium cn-font-heading", className),
      ...props
    }
  );
}
function DrawerDescription({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    DrawerPrimitive.Description,
    {
      "data-slot": "drawer-description",
      className: cn("text-muted-foreground text-sm", className),
      ...props
    }
  );
}
export {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerOverlay,
  DrawerPortal,
  DrawerTitle,
  DrawerTrigger
};
