import { cn } from "@/registry/bases/radix/lib/utils";
function Skeleton({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "skeleton",
      className: cn("bg-muted rounded-md animate-pulse", className),
      ...props
    }
  );
}
export {
  Skeleton
};
