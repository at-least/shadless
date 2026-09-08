"use client";
import * as React from "react";
import { Slider as SliderPrimitive } from "radix-ui";
import { cn } from "@/registry/bases/radix/lib/utils";
function Slider({
  className,
  defaultValue,
  value,
  min = 0,
  max = 100,
  ...props
}) {
  const _values = React.useMemo(
    () => Array.isArray(value) ? value : Array.isArray(defaultValue) ? defaultValue : [min, max],
    [value, defaultValue, min, max]
  );
  return /* @__PURE__ */ React.createElement(
    SliderPrimitive.Root,
    {
      "data-slot": "slider",
      defaultValue,
      value,
      min,
      max,
      className: cn(
        "data-vertical:min-h-40 relative flex w-full touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      SliderPrimitive.Track,
      {
        "data-slot": "slider-track",
        className: "bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
      },
      /* @__PURE__ */ React.createElement(
        SliderPrimitive.Range,
        {
          "data-slot": "slider-range",
          className: "bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
        }
      )
    ),
    Array.from({ length: _values.length }, (_, index) => /* @__PURE__ */ React.createElement(
      SliderPrimitive.Thumb,
      {
        "data-slot": "slider-thumb",
        key: index,
        className: "border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
      }
    ))
  );
}
export {
  Slider
};
