import * as React from "react";
import { Slot } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Breadcrumb({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "nav",
    {
      "aria-label": "breadcrumb",
      "data-slot": "breadcrumb",
      className: cn("", className),
      ...props
    }
  );
}
function BreadcrumbList({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "ol",
    {
      "data-slot": "breadcrumb-list",
      className: cn(
        "text-muted-foreground gap-1.5 text-sm flex flex-wrap items-center wrap-break-word",
        className
      ),
      ...props
    }
  );
}
function BreadcrumbItem({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "li",
    {
      "data-slot": "breadcrumb-item",
      className: cn("gap-1 inline-flex items-center", className),
      ...props
    }
  );
}
function BreadcrumbLink({
  asChild,
  className,
  ...props
}) {
  const Comp = asChild ? Slot.Root : "a";
  return /* @__PURE__ */ React.createElement(
    Comp,
    {
      "data-slot": "breadcrumb-link",
      className: cn("hover:text-foreground transition-colors", className),
      ...props
    }
  );
}
function BreadcrumbPage({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "span",
    {
      "data-slot": "breadcrumb-page",
      role: "link",
      "aria-disabled": "true",
      "aria-current": "page",
      className: cn("text-foreground font-normal", className),
      ...props
    }
  );
}
function BreadcrumbSeparator({
  children,
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "li",
    {
      "data-slot": "breadcrumb-separator",
      role: "presentation",
      "aria-hidden": "true",
      className: cn("[&>svg]:size-3.5", className),
      ...props
    },
    children ?? /* @__PURE__ */ React.createElement(
      IconPlaceholder,
      {
        lucide: "ChevronRightIcon",
        tabler: "IconChevronRight",
        hugeicons: "ArrowRight01Icon",
        phosphor: "CaretRightIcon",
        remixicon: "RiArrowRightSLine",
        className: "cn-rtl-flip"
      }
    )
  );
}
function BreadcrumbEllipsis({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "span",
    {
      "data-slot": "breadcrumb-ellipsis",
      role: "presentation",
      "aria-hidden": "true",
      className: cn(
        "size-5 [&>svg]:size-4 flex items-center justify-center",
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
    /* @__PURE__ */ React.createElement("span", { className: "sr-only" }, "More")
  );
}
export {
  Breadcrumb,
  BreadcrumbEllipsis,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator
};
