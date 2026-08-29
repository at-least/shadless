// probe: avatar + progress + aspect-ratio oracle facts (card 17)
import { build } from "esbuild"
import { chromium } from "playwright"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve } from "node:path"
const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { Avatar, AvatarImage, AvatarFallback } from "@/registry/new-york-v4/ui/avatar";
import { Progress } from "@/registry/new-york-v4/ui/progress";
import { AspectRatio } from "@/registry/new-york-v4/ui/aspect-ratio";
createRoot(document.getElementById("root")).render(React.createElement("div", null,
  React.createElement(Avatar, { id: "av1" },
    React.createElement(AvatarImage, { id: "avimg", src: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16'%3E%3Crect width='16' height='16' fill='red'/%3E%3C/svg%3E" }),
    React.createElement(AvatarFallback, { id: "avfb" }, "CN"),
  ),
  React.createElement(Avatar, { id: "av2" },
    React.createElement(AvatarImage, { id: "avimg2", src: "file:///nonexistent.png" }),
    React.createElement(AvatarFallback, { id: "avfb2" }, "XX"),
  ),
  React.createElement(Progress, { id: "pg", value: 42 }),
  React.createElement(AspectRatio, { id: "ar", ratio: 16/9 }, "Inside"),
));
`
mkdirSync("probes/t7/out", { recursive: true })
writeFileSync("probes/t7/out/.av-entry.mjs", entry)
await build({ entryPoints: ["probes/t7/out/.av-entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/t7/out/av.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") }, loader: { ".tsx": "tsx" } })
writeFileSync("probes/t7/out/av.html", `<!doctype html><html><body><div id="root"></div><script src="av.js"></script></body></html>`)
const browser = await chromium.launch()
const page = await browser.newPage()
await page.goto(`file://${process.cwd()}/probes/t7/out/av.html`)
await page.waitForTimeout(700)
console.log(JSON.stringify(await page.evaluate(() => {
  const snap = (sel) => { const el = document.querySelector(sel); if (!el) return null
    const o = { tag: el.tagName.toLowerCase() }
    for (const a of el.getAttributeNames()) if (a !== "class") o[a] = el.getAttribute(a)
    const snap2 = (e) => { const o2 = { tag: e.tagName.toLowerCase() }
      for (const a of e.getAttributeNames()) if (a !== "class") o2[a] = e.getAttribute(a)
      o2.children = [...e.children].map(snap2); return o2 }
    o.children = [...el.children].map(snap2); return o }
  return { av1: snap("#av1"), av2: snap("#av2"), pg: snap("#pg"), ar: snap("#ar") }
}), null, 1))
await browser.close()
