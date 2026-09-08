"use client";
import * as React from "react";
import { Select as SelectPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Select({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(SelectPrimitive.Root, { "data-slot": "select", ...props });
}
function SelectGroup({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SelectPrimitive.Group,
    {
      "data-slot": "select-group",
      className: cn("scroll-my-1 p-1", className),
      ...props
    }
  );
}
function SelectValue({
  ...props
}) {
  return /* @__PURE__ */ React.createElement(SelectPrimitive.Value, { "data-slot": "select-value", ...props });
}
function SelectTrigger({
  className,
  size = "default",
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SelectPrimitive.Trigger,
    {
      "data-slot": "select-trigger",
      "data-size": size,
      className: cn(
        "border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    },
    children,
    /* @__PURE__ */ React.createElement(SelectPrimitive.Icon, { asChild: true }, /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronDownIcon",
        tabler: "IconSelector",
        hugeicons: "UnfoldMoreIcon",
        phosphor: "CaretDownIcon",
        remixicon: "RiArrowDownSLine",
        className: "text-muted-foreground size-4 pointer-events-none"
      }
    ))
  );
}
function SelectContent({
  className,
  children,
  position = "item-aligned",
  align = "center",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(SelectPrimitive.Portal, null, /* @__PURE__ */ React.createElement(
    SelectPrimitive.Content,
    {
      "data-slot": "select-content",
      "data-align-trigger": position === "item-aligned",
      className: cn(
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none",
        position === "popper" && "data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1",
        className
      ),
      position,
      align,
      ...props
    },
    /* @__PURE__ */ React.createElement(SelectScrollUpButton, null),
    /* @__PURE__ */ React.createElement(
      SelectPrimitive.Viewport,
      {
        "data-position": position,
        className: cn(
          "data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)",
          position === "popper" && ""
        )
      },
      children
    ),
    /* @__PURE__ */ React.createElement(SelectScrollDownButton, null)
  ));
}
function SelectLabel({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SelectPrimitive.Label,
    {
      "data-slot": "select-label",
      className: cn("text-muted-foreground px-1.5 py-1 text-xs", className),
      ...props
    }
  );
}
function SelectItem({
  className,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SelectPrimitive.Item,
    {
      "data-slot": "select-item",
      className: cn(
        "focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement("span", { className: "pointer-events-none absolute right-2 flex size-4 items-center justify-center" }, /* @__PURE__ */ React.createElement(SelectPrimitive.ItemIndicator, null, /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "CheckIcon",
        tabler: "IconCheck",
        hugeicons: "Tick02Icon",
        phosphor: "CheckIcon",
        remixicon: "RiCheckLine",
        className: "pointer-events-none"
      }
    ))),
    /* @__PURE__ */ React.createElement(SelectPrimitive.ItemText, null, children)
  );
}
function SelectSeparator({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SelectPrimitive.Separator,
    {
      "data-slot": "select-separator",
      className: cn("bg-border -mx-1 my-1 h-px pointer-events-none", className),
      ...props
    }
  );
}
function SelectScrollUpButton({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SelectPrimitive.ScrollUpButton,
    {
      "data-slot": "select-scroll-up-button",
      className: cn("bg-popover z-10 flex cursor-default items-center justify-center py-1 [&_svg:not([class*='size-'])]:size-4", className),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronUpIcon",
        tabler: "IconChevronUp",
        hugeicons: "ArrowUp01Icon",
        phosphor: "CaretUpIcon",
        remixicon: "RiArrowUpSLine"
      }
    )
  );
}
function SelectScrollDownButton({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    SelectPrimitive.ScrollDownButton,
    {
      "data-slot": "select-scroll-down-button",
      className: cn("bg-popover z-10 flex cursor-default items-center justify-center py-1 [&_svg:not([class*='size-'])]:size-4", className),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronDownIcon",
        tabler: "IconChevronDown",
        hugeicons: "ArrowDown01Icon",
        phosphor: "CaretDownIcon",
        remixicon: "RiArrowDownSLine"
      }
    )
  );
}
export {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectScrollDownButton,
  SelectScrollUpButton,
  SelectSeparator,
  SelectTrigger,
  SelectValue
};
