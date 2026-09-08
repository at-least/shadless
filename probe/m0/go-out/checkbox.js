"use client";
import * as React from "react";
import { Checkbox as CheckboxPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Checkbox({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    CheckboxPrimitive.Root,
    {
      "data-slot": "checkbox",
      className: cn(
        "border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      CheckboxPrimitive.Indicator,
      {
        "data-slot": "checkbox-indicator",
        className: "[&>svg]:size-3.5 grid place-content-center text-current transition-none"
      },
      /* @__PURE__ */ React.createElement(
        IconPlaceholder,
        {
          lucide: "CheckIcon",
          tabler: "IconCheck",
          hugeicons: "Tick02Icon",
          phosphor: "CheckIcon",
          remixicon: "RiCheckLine"
        }
      )
    )
  );
}
export {
  Checkbox
};
