"use client";
import * as React from "react";
import { Accordion as AccordionPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Accordion({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    AccordionPrimitive.Root,
    {
      "data-slot": "accordion",
      className: cn("flex w-full flex-col", className),
      ...props
    }
  );
}
function AccordionItem({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    AccordionPrimitive.Item,
    {
      "data-slot": "accordion-item",
      className: cn("not-last:border-b", className),
      ...props
    }
  );
}
function AccordionTrigger({
  className,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(AccordionPrimitive.Header, { className: "flex" }, /* @__PURE__ */ React.createElement(
    AccordionPrimitive.Trigger,
    {
      "data-slot": "accordion-trigger",
      className: cn(
        "focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-left text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50",
        className
      ),
      ...props
    },
    children,
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronDownIcon",
        tabler: "IconChevronDown",
        "data-slot": "accordion-trigger-icon",
        hugeicons: "ArrowDown01Icon",
        phosphor: "CaretDownIcon",
        remixicon: "RiArrowDownSLine",
        className: "pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden"
      }
    ),
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronUpIcon",
        tabler: "IconChevronUp",
        "data-slot": "accordion-trigger-icon",
        hugeicons: "ArrowUp01Icon",
        phosphor: "CaretUpIcon",
        remixicon: "RiArrowUpSLine",
        className: "pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline"
      }
    )
  ));
}
function AccordionContent({
  className,
  children,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    AccordionPrimitive.Content,
    {
      "data-slot": "accordion-content",
      className: "data-open:animate-accordion-down data-closed:animate-accordion-up text-sm overflow-hidden",
      ...props
    },
    /* @__PURE__ */ React.createElement(
      "div",
      {
        className: cn(
          "pt-0 pb-2.5 h-(--radix-accordion-content-height) [&_a]:underline [&_a]:underline-offset-3 [&_a]:hover:text-foreground [&_p:not(:last-child)]:mb-4",
          className
        )
      },
      children
    )
  );
}
export {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger
};
