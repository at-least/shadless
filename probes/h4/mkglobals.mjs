// H4: build probe globals.css from shadcn app globals (strip app-specific
// imports/sources, inline shadcn/tailwind.css, point @source at demo.html)
import { readFileSync, writeFileSync } from "node:fs"

const app = readFileSync(".upstream/shadcn-ui/apps/v4/app/globals.css", "utf8")
const shadcnTw = readFileSync(".upstream/shadcn-ui/packages/shadcn/src/tailwind.css", "utf8")

let out = []
for (const line of app.split("\n")) {
  if (line.startsWith('@source')) continue                     // app-local sources
  if (line.includes('legacy-themes.css')) continue             // app-only legacy
  if (line.includes('"shadcn/tailwind.css"')) continue         // inlined below
  out.push(line)
}
// inline shadcn/tailwind.css where its import was (custom variants + keyframes)
const idx = out.findIndex(l => l.startsWith('@import "tw-animate-css"'))
out.splice(idx + 1, 0, "/* === begin inlined shadcn/tailwind.css === */",
  ...shadcnTw.split("\n"), "/* === end inlined shadcn/tailwind.css === */",
  '@source "./demo.html";')
writeFileSync("probes/h4/globals.css", out.join("\n"))
console.log("wrote probes/h4/globals.css", out.length, "lines")
