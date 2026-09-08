import * as React from "react";
import { cn } from "@/registry/bases/radix/lib/utils";
import { Button } from "@/registry/bases/radix/ui/button";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Pagination({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "nav",
    {
      role: "navigation",
      "aria-label": "pagination",
      "data-slot": "pagination",
      className: cn(
        "mx-auto flex w-full justify-center",
        className
      ),
      ...props
    }
  );
}
function PaginationContent({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "ul",
    {
      "data-slot": "pagination-content",
      className: cn("gap-0.5 flex items-center", className),
      ...props
    }
  );
}
function PaginationItem({ ...props }) {
  return /* @__PURE__ */ React.createElement("li", { "data-slot": "pagination-item", ...props });
}
function PaginationLink({
  className,
  isActive,
  size = "icon",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    Button,
    {
      asChild: true,
      variant: isActive ? "outline" : "ghost",
      size,
      className: cn("", className)
    },
    /* @__PURE__ */ React.createElement(
      "a",
      {
        "aria-current": isActive ? "page" : void 0,
        "data-slot": "pagination-link",
        "data-active": isActive,
        ...props
      }
    )
  );
}
function PaginationPrevious({
  className,
  text = "Previous",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    PaginationLink,
    {
      "aria-label": "Go to previous page",
      size: "default",
      className: cn("pl-1.5!", className),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronLeftIcon",
        tabler: "IconChevronLeft",
        hugeicons: "ArrowLeft01Icon",
        phosphor: "CaretLeftIcon",
        remixicon: "RiArrowLeftSLine",
        "data-icon": "inline-start",
        className: "cn-rtl-flip"
      }
    ),
    /* @__PURE__ */ React.createElement("span", { className: "hidden sm:block" }, text)
  );
}
function PaginationNext({
  className,
  text = "Next",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    PaginationLink,
    {
      "aria-label": "Go to next page",
      size: "default",
      className: cn("pr-1.5!", className),
      ...props
    },
    /* @__PURE__ */ React.createElement("span", { className: "hidden sm:block" }, text),
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronRightIcon",
        tabler: "IconChevronRight",
        hugeicons: "ArrowRight01Icon",
        phosphor: "CaretRightIcon",
        remixicon: "RiArrowRightSLine",
        "data-icon": "inline-end",
        className: "cn-rtl-flip"
      }
    )
  );
}
function PaginationEllipsis({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "span",
    {
      "aria-hidden": true,
      "data-slot": "pagination-ellipsis",
      className: cn(
        "size-8 [&_svg:not([class*='size-'])]:size-4 flex items-center justify-center",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "MoreHorizontalIcon",
        tabler: "IconDots",
        hugeicons: "MoreHorizontalCircle01Icon",
        phosphor: "DotsThreeIcon",
        remixicon: "RiMoreLine"
      }
    ),
    /* @__PURE__ */ React.createElement("span", { className: "sr-only" }, "More pages")
  );
}
export {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious
};
