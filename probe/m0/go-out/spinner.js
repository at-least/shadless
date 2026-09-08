import { cn } from "@/registry/bases/radix/lib/utils";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Spinner({ className, ...props }) {
  return /* @__PURE__ */ React.createElement(
    IconPlaceholder,
    {
      lucide: "Loader2Icon",
      tabler: "IconLoader",
      hugeicons: "Loading03Icon",
      phosphor: "SpinnerIcon",
      remixicon: "RiLoaderLine",
      "data-slot": "spinner",
      role: "status",
      "aria-label": "Loading",
      className: cn("size-4 animate-spin", className),
      ...props
    }
  );
}
export {
  Spinner
};
