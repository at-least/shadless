"use client";
import * as React from "react";
import { Dialog as SheetPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
import { Button } from "@/registry/bases/radix/ui/button";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Sheet({ ...props }) {
  return /* @__PURE__ */ React.createElement(SheetPrimitive.Root, { "data-slot": "sheet", ...props });
}
function SheetTrigger({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(SheetPrimitive.Trigger, { "data-slot": "sheet-trigger", ...props });
}
function SheetClose({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(SheetPrimitive.Close, { "data-slot": "sheet-close", ...props });
}
function SheetPortal({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(SheetPrimitive.Portal, { "data-slot": "sheet-portal", ...props });
}
function SheetOverlay({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SheetPrimitive.Overlay,
    {
      "data-slot": "sheet-overlay",
      className: cn(
        "bg-black/10 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 z-50 duration-100 data-open:animate-in data-open:fade-in-0 data-closed:animate-out data-closed:fade-out-0",
        className
      ),
      ...props
    }
  );
}
function SheetContent({
  className,
  children,
  side = "right",
  showCloseButton = true,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(SheetPortal, null, /* @__PURE__ */ React.createElement(SheetOverlay, null), /* @__PURE__ */ React.createElement(
    SheetPrimitive.Content,
    {
      "data-slot": "sheet-content",
      "data-side": side,
      className: cn(
        "bg-popover text-popover-foreground fixed z-50 flex flex-col gap-4 bg-clip-padding text-sm shadow-lg transition duration-200 ease-in-out data-[side=bottom]:inset-x-0 data-[side=bottom]:bottom-0 data-[side=bottom]:h-auto data-[side=bottom]:border-t data-[side=left]:inset-y-0 data-[side=left]:left-0 data-[side=left]:h-full data-[side=left]:w-3/4 data-[side=left]:border-r data-[side=right]:inset-y-0 data-[side=right]:right-0 data-[side=right]:h-full data-[side=right]:w-3/4 data-[side=right]:border-l data-[side=top]:inset-x-0 data-[side=top]:top-0 data-[side=top]:h-auto data-[side=top]:border-b data-[side=left]:sm:max-w-sm data-[side=right]:sm:max-w-sm data-open:animate-in data-open:fade-in-0 data-[side=bottom]:data-open:slide-in-from-bottom-10 data-[side=left]:data-open:slide-in-from-left-10 data-[side=right]:data-open:slide-in-from-right-10 data-[side=top]:data-open:slide-in-from-top-10 data-closed:animate-out data-closed:fade-out-0 data-[side=bottom]:data-closed:slide-out-to-bottom-10 data-[side=left]:data-closed:slide-out-to-left-10 data-[side=right]:data-closed:slide-out-to-right-10 data-[side=top]:data-closed:slide-out-to-top-10",
        className
      ),
      ...props
    },
    children,
    showCloseButton && /* @__PURE__ */ React.createElement(SheetPrimitive.Close, { "data-slot": "sheet-close", asChild: true }, /* @__PURE__ */ React.createElement(Button, { variant: "ghost", className: "absolute top-3 right-3", size: "icon-sm" }, /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "XIcon",
        tabler: "IconX",
        hugeicons: "Cancel01Icon",
        phosphor: "XIcon",
        remixicon: "RiCloseLine"
      }
    ), /* @__PURE__ */ React.createElement("span", { className: "sr-only" }, "Close")))
  ));
}
function SheetHeader({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "sheet-header",
      className: cn("gap-0.5 p-4 flex flex-col", className),
      ...props
    }
  );
}
function SheetFooter({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "sheet-footer",
      className: cn("gap-2 p-4 mt-auto flex flex-col", className),
      ...props
    }
  );
}
function SheetTitle({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SheetPrimitive.Title,
    {
      "data-slot": "sheet-title",
      className: cn("text-foreground text-base font-medium cn-font-heading", className),
      ...props
    }
  );
}
function SheetDescription({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SheetPrimitive.Description,
    {
      "data-slot": "sheet-description",
      className: cn("text-muted-foreground text-sm", className),
      ...props
    }
  );
}
export {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger
};
