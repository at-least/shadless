"use client";
import * as React from "react";
import { ContextMenu as ContextMenuPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function ContextMenu({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(ContextMenuPrimitive.Root, { "data-slot": "context-menu", ...props });
}
function ContextMenuTrigger({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.Trigger,
    {
      "data-slot": "context-menu-trigger",
      className: cn("select-none", className),
      ...props
    }
  );
}
function ContextMenuGroup({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(ContextMenuPrimitive.Group, { "data-slot": "context-menu-group", ...props });
}
function ContextMenuPortal({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(ContextMenuPrimitive.Portal, { "data-slot": "context-menu-portal", ...props });
}
function ContextMenuSub({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(ContextMenuPrimitive.Sub, { "data-slot": "context-menu-sub", ...props });
}
function ContextMenuRadioGroup({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.RadioGroup,
    {
      "data-slot": "context-menu-radio-group",
      ...props
    }
  );
}
function ContextMenuContent({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(ContextMenuPrimitive.Portal, null, /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.Content,
    {
      "data-slot": "context-menu-content",
      className: cn(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 bg-popover text-popover-foreground min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 max-h-(--radix-context-menu-content-available-height) origin-(--radix-context-menu-content-transform-origin) overflow-x-hidden overflow-y-auto",
        className
      ),
      ...props
    }
  ));
}
function ContextMenuItem({
  className,
  inset,
  variant = "default",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.Item,
    {
      "data-slot": "context-menu-item",
      "data-inset": inset,
      "data-variant": variant,
      className: cn(
        "focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive focus:*:[svg]:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&_svg:not([class*='size-'])]:size-4 group/context-menu-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    }
  );
}
function ContextMenuSubTrigger({
  className,
  inset,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.SubTrigger,
    {
      "data-slot": "context-menu-sub-trigger",
      "data-inset": inset,
      className: cn(
        "focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-hidden select-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    },
    children,
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronRightIcon",
        tabler: "IconChevronRight",
        hugeicons: "ArrowRight01Icon",
        phosphor: "CaretRightIcon",
        remixicon: "RiArrowRightSLine",
        className: "cn-rtl-flip ml-auto"
      }
    )
  );
}
function ContextMenuSubContent({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.SubContent,
    {
      "data-slot": "context-menu-sub-content",
      className: cn(
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 bg-popover text-popover-foreground min-w-32 rounded-lg border p-1 shadow-lg duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-context-menu-content-transform-origin) overflow-hidden",
        className
      ),
      ...props
    }
  );
}
function ContextMenuCheckboxItem({
  className,
  children,
  checked,
  inset,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.CheckboxItem,
    {
      "data-slot": "context-menu-checkbox-item",
      "data-inset": inset,
      className: cn(
        "focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      checked,
      ...props
    },
    /* @__PURE__ */ React.createElement("span", { className: "absolute right-2 pointer-events-none" }, /* @__PURE__ */ React.createElement(ContextMenuPrimitive.ItemIndicator, null, /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "CheckIcon",
        tabler: "IconCheck",
        hugeicons: "Tick02Icon",
        phosphor: "CheckIcon",
        remixicon: "RiCheckLine"
      }
    ))),
    children
  );
}
function ContextMenuRadioItem({
  className,
  children,
  inset,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.RadioItem,
    {
      "data-slot": "context-menu-radio-item",
      "data-inset": inset,
      className: cn(
        "focus:bg-accent focus:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm data-inset:pl-7 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement("span", { className: "absolute right-2 pointer-events-none" }, /* @__PURE__ */ React.createElement(ContextMenuPrimitive.ItemIndicator, null, /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "CheckIcon",
        tabler: "IconCheck",
        hugeicons: "Tick02Icon",
        phosphor: "CheckIcon",
        remixicon: "RiCheckLine"
      }
    ))),
    children
  );
}
function ContextMenuLabel({
  className,
  inset,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.Label,
    {
      "data-slot": "context-menu-label",
      "data-inset": inset,
      className: cn("text-muted-foreground px-1.5 py-1 text-xs font-medium data-inset:pl-7", className),
      ...props
    }
  );
}
function ContextMenuSeparator({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ContextMenuPrimitive.Separator,
    {
      "data-slot": "context-menu-separator",
      className: cn("bg-border -mx-1 my-1 h-px", className),
      ...props
    }
  );
}
function ContextMenuShortcut({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "span",
    {
      "data-slot": "context-menu-shortcut",
      className: cn("text-muted-foreground group-focus/context-menu-item:text-accent-foreground ml-auto text-xs tracking-widest", className),
      ...props
    }
  );
}
export {
  ContextMenu,
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuGroup,
  ContextMenuItem,
  ContextMenuLabel,
  ContextMenuPortal,
  ContextMenuRadioGroup,
  ContextMenuRadioItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger
};
