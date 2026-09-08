"use client";
import * as React from "react";
import { Command as CommandPrimitive } from "cmdk";
import { cn } from "@/registry/bases/radix/lib/utils";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle
} from "@/registry/bases/radix/ui/dialog";
import {
  InputGroup,
  InputGroupAddon
} from "@/registry/bases/radix/ui/input-group";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Command({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CommandPrimitive,
    {
      "data-slot": "command",
      className: cn(
        "bg-popover text-popover-foreground rounded-xl! p-1 flex size-full flex-col overflow-hidden",
        className
      ),
      ...props
    }
  );
}
function CommandDialog({
  title = "Command Palette",
  description = "Search for a command to run...",
  children,
  className,
  showCloseButton = false,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(Dialog, { ...props }, /* @__PURE__ */ React.createElement(DialogHeader, { className: "sr-only" }, /* @__PURE__ */ React.createElement(DialogTitle, null, title), /* @__PURE__ */ React.createElement(DialogDescription, null, description)), /* @__PURE__ */ React.createElement(
    DialogContent,
    {
      className: cn(
        "rounded-xl! top-1/3 translate-y-0 overflow-hidden p-0",
        className
      ),
      showCloseButton
    },
    children
  ));
}
function CommandInput({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement("div", { "data-slot": "command-input-wrapper", className: "p-1 pb-0" }, /* @__PURE__ */ React.createElement(InputGroup, { className: "bg-input/30 border-input/30 h-8! rounded-lg! shadow-none! *:data-[slot=input-group-addon]:pl-2!" }, /* @__PURE__ */ React.createElement(
    CommandPrimitive.Input,
    {
      "data-slot": "command-input",
      className: cn(
        "w-full text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50",
        className
      ),
      ...props
    }
  ), /* @__PURE__ */ React.createElement(InputGroupAddon, null, /* @__PURE__ */ React.createElement(
    IconPlaceholder,
    {
      lucide: "SearchIcon",
      tabler: "IconSearch",
      hugeicons: "SearchIcon",
      phosphor: "MagnifyingGlassIcon",
      remixicon: "RiSearchLine",
      className: "size-4 shrink-0 opacity-50"
    }
  ))));
}
function CommandList({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CommandPrimitive.List,
    {
      "data-slot": "command-list",
      className: cn(
        "no-scrollbar max-h-72 scroll-py-1 outline-none overflow-x-hidden overflow-y-auto",
        className
      ),
      ...props
    }
  );
}
function CommandEmpty({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CommandPrimitive.Empty,
    {
      "data-slot": "command-empty",
      className: cn("py-6 text-center text-sm", className),
      ...props
    }
  );
}
function CommandGroup({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CommandPrimitive.Group,
    {
      "data-slot": "command-group",
      className: cn("text-foreground **:[[cmdk-group-heading]]:text-muted-foreground overflow-hidden p-1 **:[[cmdk-group-heading]]:px-2 **:[[cmdk-group-heading]]:py-1.5 **:[[cmdk-group-heading]]:text-xs **:[[cmdk-group-heading]]:font-medium", className),
      ...props
    }
  );
}
function CommandSeparator({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CommandPrimitive.Separator,
    {
      "data-slot": "command-separator",
      className: cn("bg-border -mx-1 h-px", className),
      ...props
    }
  );
}
function CommandItem({
  className,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CommandPrimitive.Item,
    {
      "data-slot": "command-item",
      className: cn(
        "data-selected:bg-muted data-selected:text-foreground data-selected:*:[svg]:text-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none in-data-[slot=dialog-content]:rounded-lg! [&_svg:not([class*='size-'])]:size-4 group/command-item data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    },
    children,
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "CheckIcon",
        tabler: "IconCheck",
        hugeicons: "Tick02Icon",
        phosphor: "CheckIcon",
        remixicon: "RiCheckLine",
        className: "ml-auto opacity-0 group-has-data-[slot=command-shortcut]/command-item:hidden group-data-[checked=true]/command-item:opacity-100"
      }
    )
  );
}
function CommandShortcut({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "span",
    {
      "data-slot": "command-shortcut",
      className: cn("text-muted-foreground group-data-selected/command-item:text-foreground ml-auto text-xs tracking-widest", className),
      ...props
    }
  );
}
export {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  CommandShortcut
};
