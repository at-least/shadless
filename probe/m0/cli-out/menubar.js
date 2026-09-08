"use client";
import * as React from "react";
import { Menubar as MenubarPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Menubar({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.Root,
    {
      "data-slot": "menubar",
      className: cn("h-8 gap-0.5 rounded-lg border p-[3px] flex items-center", className),
      ...props
    }
  );
}
function MenubarMenu({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(MenubarPrimitive.Menu, { "data-slot": "menubar-menu", ...props });
}
function MenubarGroup({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(MenubarPrimitive.Group, { "data-slot": "menubar-group", ...props });
}
function MenubarPortal({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(MenubarPrimitive.Portal, { "data-slot": "menubar-portal", ...props });
}
function MenubarRadioGroup({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(MenubarPrimitive.RadioGroup, { "data-slot": "menubar-radio-group", ...props });
}
function MenubarTrigger({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.Trigger,
    {
      "data-slot": "menubar-trigger",
      className: cn(
        "hover:bg-muted aria-expanded:bg-muted rounded-sm px-1.5 py-[2px] text-sm font-medium flex items-center outline-hidden select-none",
        className
      ),
      ...props
    }
  );
}
function MenubarContent({
  className,
  align = "start",
  alignOffset = -4,
  sideOffset = 8,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(MenubarPortal, null, /* @__PURE__ */ React.createElement(
    MenubarPrimitive.Content,
    {
      "data-slot": "menubar-content",
      align,
      alignOffset,
      sideOffset,
      className: cn(
        "bg-popover text-popover-foreground data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg p-1 shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden",
        className
      ),
      ...props
    }
  ));
}
function MenubarItem({
  className,
  inset,
  variant = "default",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.Item,
    {
      "data-slot": "menubar-item",
      "data-inset": inset,
      "data-variant": variant,
      className: cn(
        "focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:text-destructive! not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-disabled:opacity-50 data-inset:pl-7 [&_svg:not([class*='size-'])]:size-4 group/menubar-item relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    }
  );
}
function MenubarCheckboxItem({
  className,
  children,
  checked,
  inset,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.CheckboxItem,
    {
      "data-slot": "menubar-checkbox-item",
      "data-inset": inset,
      className: cn(
        "focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-inset:pl-7 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      checked,
      ...props
    },
    /* @__PURE__ */ React.createElement("span", { className: "left-1.5 size-4 [&_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center" }, /* @__PURE__ */ React.createElement(MenubarPrimitive.ItemIndicator, null, /* @__PURE__ */ React.createElement(
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
function MenubarRadioItem({
  className,
  children,
  inset,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.RadioItem,
    {
      "data-slot": "menubar-radio-item",
      "data-inset": inset,
      className: cn(
        "focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-1.5 pl-7 text-sm data-disabled:opacity-50 data-inset:pl-7 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement("span", { className: "left-1.5 size-4 [&_svg:not([class*='size-'])]:size-4 pointer-events-none absolute flex items-center justify-center" }, /* @__PURE__ */ React.createElement(MenubarPrimitive.ItemIndicator, null, /* @__PURE__ */ React.createElement(
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
function MenubarLabel({
  className,
  inset,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.Label,
    {
      "data-slot": "menubar-label",
      "data-inset": inset,
      className: cn("px-1.5 py-1 text-sm font-medium data-inset:pl-7", className),
      ...props
    }
  );
}
function MenubarSeparator({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.Separator,
    {
      "data-slot": "menubar-separator",
      className: cn("bg-border -mx-1 my-1 h-px", className),
      ...props
    }
  );
}
function MenubarShortcut({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "span",
    {
      "data-slot": "menubar-shortcut",
      className: cn("text-muted-foreground group-focus/menubar-item:text-accent-foreground text-xs tracking-widest ml-auto", className),
      ...props
    }
  );
}
function MenubarSub({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(MenubarPrimitive.Sub, { "data-slot": "menubar-sub", ...props });
}
function MenubarSubTrigger({
  className,
  inset,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.SubTrigger,
    {
      "data-slot": "menubar-sub-trigger",
      "data-inset": inset,
      className: cn(
        "focus:bg-accent focus:text-accent-foreground data-open:bg-accent data-open:text-accent-foreground gap-1.5 rounded-md px-1.5 py-1 text-sm data-inset:pl-7 [&_svg:not([class*='size-'])]:size-4 flex cursor-default items-center outline-none select-none",
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
        className: "cn-rtl-flip ml-auto size-4"
      }
    )
  );
}
function MenubarSubContent({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    MenubarPrimitive.SubContent,
    {
      "data-slot": "menubar-sub-content",
      className: cn(
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-32 rounded-lg p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--radix-menubar-content-transform-origin) overflow-hidden",
        className
      ),
      ...props
    }
  );
}
export {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarGroup,
  MenubarItem,
  MenubarLabel,
  MenubarMenu,
  MenubarPortal,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger
};
