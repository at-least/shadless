"use client";
import { useTheme } from "next-themes";
import { Toaster as Sonner } from "sonner";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
const Toaster = ({ ...props }) => {
  const { theme = "system" } = useTheme();
  return /* @__PURE__ */ React.createElement(
    Sonner,
    {
      theme,
      className: "toaster group",
      icons: {
        success: /* @__PURE__ */ React.createElement(
          IconPlaceholder,
          {
            lucide: "CircleCheckIcon",
            tabler: "IconCircleCheck",
            hugeicons: "CheckmarkCircle02Icon",
            phosphor: "CheckCircleIcon",
            remixicon: "RiCheckboxCircleLine",
            className: "size-4"
          }
        ),
        info: /* @__PURE__ */ React.createElement(
          IconPlaceholder,
          {
            lucide: "InfoIcon",
            tabler: "IconInfoCircle",
            hugeicons: "InformationCircleIcon",
            phosphor: "InfoIcon",
            remixicon: "RiInformationLine",
            className: "size-4"
          }
        ),
        warning: /* @__PURE__ */ React.createElement(
          IconPlaceholder,
          {
            lucide: "TriangleAlertIcon",
            tabler: "IconAlertTriangle",
            hugeicons: "Alert02Icon",
            phosphor: "WarningIcon",
            remixicon: "RiErrorWarningLine",
            className: "size-4"
          }
        ),
        error: /* @__PURE__ */ React.createElement(
          IconPlaceholder,
          {
            lucide: "OctagonXIcon",
            tabler: "IconAlertOctagon",
            hugeicons: "MultiplicationSignCircleIcon",
            phosphor: "XCircleIcon",
            remixicon: "RiCloseCircleLine",
            className: "size-4"
          }
        ),
        loading: /* @__PURE__ */ React.createElement(
          IconPlaceholder,
          {
            lucide: "Loader2Icon",
            tabler: "IconLoader",
            hugeicons: "Loading03Icon",
            phosphor: "SpinnerIcon",
            remixicon: "RiLoaderLine",
            className: "size-4 animate-spin"
          }
        )
      },
      style: {
        "--normal-bg": "var(--popover)",
        "--normal-text": "var(--popover-foreground)",
        "--normal-border": "var(--border)",
        "--border-radius": "var(--radius)"
      },
      toastOptions: {
        classNames: {
          toast: "rounded-2xl"
        }
      },
      ...props
    }
  );
};
export {
  Toaster
};
