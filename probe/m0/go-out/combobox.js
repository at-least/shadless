"use client";
import * as React from "react";
import { Combobox as ComboboxPrimitive } from "@base-ui/react";
import { cn } from "@/registry/bases/radix/lib/utils";
import { Button } from "@/registry/bases/radix/ui/button";
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput
} from "@/registry/bases/radix/ui/input-group";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
const Combobox = ComboboxPrimitive.Root;
function ComboboxValue({ ...props }) {
  return /* @__PURE__ */ React.createElement(ComboboxPrimitive.Value, { "data-slot": "combobox-value", ...props });
}
function ComboboxTrigger({
  className,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Trigger,
    {
      "data-slot": "combobox-trigger",
      className: cn("[&_svg:not([class*='size-'])]:size-4", className),
      ...props
    },
    children,
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronDownIcon",
        tabler: "IconChevronDown",
        hugeicons: "ArrowDown01Icon",
        phosphor: "CaretDownIcon",
        remixicon: "RiArrowDownSLine",
        className: "text-muted-foreground size-4 pointer-events-none"
      }
    )
  );
}
function ComboboxClear({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Clear,
    {
      "data-slot": "combobox-clear",
      render: /* @__PURE__ */ React.createElement(InputGroupButton, { variant: "ghost", size: "icon-xs" }),
      className: cn("", className),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "XIcon",
        tabler: "IconX",
        hugeicons: "Cancel01Icon",
        phosphor: "XIcon",
        remixicon: "RiCloseLine",
        className: "pointer-events-none"
      }
    )
  );
}
function ComboboxInput({
  className,
  children,
  disabled = false,
  showTrigger = true,
  showClear = false,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(InputGroup, { className: cn("w-auto", className) }, /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Input,
    {
      render: /* @__PURE__ */ React.createElement(InputGroupInput, { disabled }),
      ...props
    }
  ), /* @__PURE__ */ React.createElement(InputGroupAddon, { align: "inline-end" }, showTrigger && /* @__PURE__ */ React.createElement(
    InputGroupButton,
    {
      size: "icon-xs",
      variant: "ghost",
      asChild: true,
      "data-slot": "input-group-button",
      className: "group-has-data-[slot=combobox-clear]/input-group:hidden data-pressed:bg-transparent",
      disabled
    },
    /* @__PURE__ */ React.createElement(ComboboxTrigger, null)
  ), showClear && /* @__PURE__ */ React.createElement(ComboboxClear, { disabled })), children);
}
function ComboboxContent({
  className,
  side = "bottom",
  sideOffset = 6,
  align = "start",
  alignOffset = 0,
  anchor,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(ComboboxPrimitive.Portal, null, /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Positioner,
    {
      side,
      sideOffset,
      align,
      alignOffset,
      anchor,
      className: "isolate z-50"
    },
    /* @__PURE__ */ React.createElement(
      ComboboxPrimitive.Popup,
      {
        "data-slot": "combobox-content",
        "data-chips": !!anchor,
        className: cn(
          "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 *:data-[slot=input-group]:bg-input/30 *:data-[slot=input-group]:border-input/30 overflow-hidden rounded-lg shadow-md ring-1 duration-100 *:data-[slot=input-group]:m-1 *:data-[slot=input-group]:mb-0 *:data-[slot=input-group]:h-8 *:data-[slot=input-group]:shadow-none data-[side=inline-start]:slide-in-from-right-2 data-[side=inline-end]:slide-in-from-left-2 cn-menu-target cn-menu-translucent group/combobox-content relative max-h-(--available-height) w-(--anchor-width) max-w-(--available-width) min-w-[calc(var(--anchor-width)+--spacing(7))] origin-(--transform-origin) data-[chips=true]:min-w-(--anchor-width)",
          className
        ),
        ...props
      }
    )
  ));
}
function ComboboxList({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.List,
    {
      "data-slot": "combobox-list",
      className: cn(
        "no-scrollbar max-h-[min(calc(--spacing(72)---spacing(9)),calc(var(--available-height)---spacing(9)))] scroll-py-1 p-1 data-empty:p-0 overflow-y-auto overscroll-contain",
        className
      ),
      ...props
    }
  );
}
function ComboboxItem({
  className,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Item,
    {
      "data-slot": "combobox-item",
      className: cn(
        "data-highlighted:bg-accent data-highlighted:text-accent-foreground not-data-[variant=destructive]:data-highlighted:**:text-accent-foreground gap-2 rounded-md py-1 pr-8 pl-1.5 text-sm [&_svg:not([class*='size-'])]:size-4 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className
      ),
      ...props
    },
    children,
    /* @__PURE__ */ React.createElement(
      ComboboxPrimitive.ItemIndicator,
      {
        render: /* @__PURE__ */ React.createElement("span", { className: "pointer-events-none absolute right-2 flex size-4 items-center justify-center" })
      },
      /* @__PURE__ */ React.createElement(
        IconPlaceholder,
        {
          lucide: "CheckIcon",
          tabler: "IconCheck",
          hugeicons: "Tick02Icon",
          phosphor: "CheckIcon",
          remixicon: "RiCheckLine",
          className: "pointer-events-none"
        }
      )
    )
  );
}
function ComboboxGroup({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Group,
    {
      "data-slot": "combobox-group",
      className: cn("", className),
      ...props
    }
  );
}
function ComboboxLabel({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.GroupLabel,
    {
      "data-slot": "combobox-label",
      className: cn("text-muted-foreground px-2 py-1.5 text-xs", className),
      ...props
    }
  );
}
function ComboboxCollection({ ...props }) {
  return /* @__PURE__ */ React.createElement(ComboboxPrimitive.Collection, { "data-slot": "combobox-collection", ...props });
}
function ComboboxEmpty({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Empty,
    {
      "data-slot": "combobox-empty",
      className: cn("text-muted-foreground hidden w-full justify-center py-2 text-center text-sm group-data-empty/combobox-content:flex", className),
      ...props
    }
  );
}
function ComboboxSeparator({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Separator,
    {
      "data-slot": "combobox-separator",
      className: cn("bg-border -mx-1 my-1 h-px", className),
      ...props
    }
  );
}
function ComboboxChips({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Chips,
    {
      "data-slot": "combobox-chips",
      className: cn("dark:bg-input/30 border-input focus-within:border-ring focus-within:ring-ring/50 has-aria-invalid:ring-destructive/20 dark:has-aria-invalid:ring-destructive/40 has-aria-invalid:border-destructive dark:has-aria-invalid:border-destructive/50 flex min-h-8 flex-wrap items-center gap-1 rounded-lg border bg-transparent bg-clip-padding px-2.5 py-1 text-sm transition-colors focus-within:ring-3 has-aria-invalid:ring-3 has-data-[slot=combobox-chip]:px-1", className),
      ...props
    }
  );
}
function ComboboxChip({
  className,
  children,
  showRemove = true,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Chip,
    {
      "data-slot": "combobox-chip",
      className: cn(
        "bg-muted text-foreground flex h-[calc(--spacing(5.25))] w-fit items-center justify-center gap-1 rounded-sm px-1.5 text-xs font-medium whitespace-nowrap has-data-[slot=combobox-chip-remove]:pr-0 has-disabled:pointer-events-none has-disabled:cursor-not-allowed has-disabled:opacity-50",
        className
      ),
      ...props
    },
    children,
    showRemove && /* @__PURE__ */ React.createElement(
      ComboboxPrimitive.ChipRemove,
      {
        render: /* @__PURE__ */ React.createElement(Button, { variant: "ghost", size: "icon-xs" }),
        className: "-ml-1 opacity-50 hover:opacity-100",
        "data-slot": "combobox-chip-remove"
      },
      /* @__PURE__ */ React.createElement(
        IconPlaceholder,
        {
          lucide: "XIcon",
          tabler: "IconX",
          hugeicons: "Cancel01Icon",
          phosphor: "XIcon",
          remixicon: "RiCloseLine",
          className: "pointer-events-none"
        }
      )
    )
  );
}
function ComboboxChipsInput({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    ComboboxPrimitive.Input,
    {
      "data-slot": "combobox-chip-input",
      className: cn(
        "min-w-16 flex-1 outline-none",
        className
      ),
      ...props
    }
  );
}
function useComboboxAnchor() {
  return React.useRef(null);
}
export {
  Combobox,
  ComboboxChip,
  ComboboxChips,
  ComboboxChipsInput,
  ComboboxCollection,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxGroup,
  ComboboxInput,
  ComboboxItem,
  ComboboxLabel,
  ComboboxList,
  ComboboxSeparator,
  ComboboxTrigger,
  ComboboxValue,
  useComboboxAnchor
};
