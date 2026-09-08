"use client";
import * as React from "react";
import { Dialog as DialogPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
import { Button } from "@/registry/bases/radix/ui/button";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Dialog({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DialogPrimitive.Root, { "data-slot": "dialog", ...props });
}
function DialogTrigger({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DialogPrimitive.Trigger, { "data-slot": "dialog-trigger", ...props });
}
function DialogPortal({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DialogPrimitive.Portal, { "data-slot": "dialog-portal", ...props });
}
function DialogClose({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DialogPrimitive.Close, { "data-slot": "dialog-close", ...props });
}
function DialogOverlay({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    DialogPrimitive.Overlay,
    {
      "data-slot": "dialog-overlay",
      className: cn("data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50", className),
      ...props
    }
  );
}
function DialogContent({
  className,
  children,
  showCloseButton = true,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(DialogPortal, null, /* @__PURE__ */ React.createElement(DialogOverlay, null), /* @__PURE__ */ React.createElement(
    DialogPrimitive.Content,
    {
      "data-slot": "dialog-content",
      className: cn(
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 grid max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-sm ring-1 duration-100 sm:max-w-sm fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 outline-none",
        className
      ),
      ...props
    },
    children,
    showCloseButton && /* @__PURE__ */ React.createElement(DialogPrimitive.Close, { "data-slot": "dialog-close", asChild: true }, /* @__PURE__ */ React.createElement(Button, { variant: "ghost", className: "absolute top-2 right-2", size: "icon-sm" }, /* @__PURE__ */ React.createElement(
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
function DialogHeader({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "dialog-header",
      className: cn("gap-2 flex flex-col", className),
      ...props
    }
  );
}
function DialogFooter({
  className,
  showCloseButton = false,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "dialog-footer",
      className: cn(
        "bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4 flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
        className
      ),
      ...props
    },
    children,
    showCloseButton && /* @__PURE__ */ React.createElement(DialogPrimitive.Close, { asChild: true }, /* @__PURE__ */ React.createElement(Button, { variant: "outline" }, "Close"))
  );
}
function DialogTitle({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    DialogPrimitive.Title,
    {
      "data-slot": "dialog-title",
      className: cn("text-base leading-none font-medium cn-font-heading", className),
      ...props
    }
  );
}
function DialogDescription({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    DialogPrimitive.Description,
    {
      "data-slot": "dialog-description",
      className: cn("text-muted-foreground *:[a]:hover:text-foreground text-sm *:[a]:underline *:[a]:underline-offset-3", className),
      ...props
    }
  );
}
export {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger
};
