// esbuild stub for the oracle build: upstream ui components import
// IconPlaceholder from the demo-app subtree (@/app/(create)/…), which
// transitively pulls next/navigation + nuqs. The oracle needs the REAL
// icon svg the site renders. Site behavior (verified across the
// snapshot): lucide is the primary library — icons whose lucide name
// doesn't resolve (e.g. FolderCodeIcon) fall back to the tabler glyph
// (204 lucide vs 8 tabler-icon occurrences across 444 previews).
import * as React from "react"
import * as Tabler from "@tabler/icons-react"
import * as Lucide from "lucide-react"

export function IconPlaceholder({
  lucide,
  tabler,
  hugeicons,
  phosphor,
  remixicon,
  ...props
}) {
  const C = (lucide && Lucide[lucide])
    || (tabler && Tabler[tabler])
    || null
  if (!C) {
    return React.createElement(
      "svg",
      {
        xmlns: "http://www.w3.org/2000/svg",
        viewBox: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        strokeWidth: 2,
        strokeLinecap: "round",
        strokeLinejoin: "round",
        "aria-hidden": true,
        ...props,
      },
      React.createElement("rect", { x: 4, y: 4, width: 16, height: 16, rx: 2 })
    )
  }
  return React.createElement(C, props)
}
