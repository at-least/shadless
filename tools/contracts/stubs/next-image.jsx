// next/image stub for oracle bundling (examples import it; shadless has
// no Next runtime). Reproduces next/image's SSR-rendered attributes for
// the props examples use (fill, alt, className, priority→eager): the
// CDN indirection (/_next/image?url=…) is a Next server concern — the
// comparator normalizes it back to the raw url on both sides.
import * as React from "react"

export default function Image({
  src, alt, width, height, fill, priority, sizes, quality, style, ...props
}) {
  return React.createElement("img", {
    src, alt, width, height,
    ...(fill ? { "data-nimg": "fill", sizes: sizes ?? "100vw" } : { "data-nimg": "1" }),
    decoding: "async",
    loading: priority ? "eager" : "lazy",
    style: fill
      ? { bottom: 0, color: "transparent", height: "100%", left: 0, position: "absolute", right: 0, top: 0, width: "100%", ...style }
      // next/image sets color:transparent on ALL imgs (avoids alt-text
      // flash while loading); upstream's SSR output carries it either way
      : { color: "transparent", ...style },
    ...props,
  })
}
