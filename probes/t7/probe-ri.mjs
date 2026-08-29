import { build } from "esbuild"
import { chromium } from "playwright"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve } from "node:path"
const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { RadioGroup, RadioGroupItem } from "@/registry/new-york-v4/ui/radio-group";
createRoot(document.getElementById("root")).render(
  React.createElement(RadioGroup, { id: "rg" },
    React.createElement(RadioGroupItem, { id: "ri1", value: "a" })));
`
mkdirSync("probes/t7/out", { recursive: true })
writeFileSync("probes/t7/out/.ri-entry.mjs", entry)
await build({ entryPoints: ["probes/t7/out/.ri-entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/t7/out/ri.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") }, loader: { ".tsx": "tsx" } })
writeFileSync("probes/t7/out/ri.html", `<!doctype html><html><body><div id="root"></div><script src="ri.js"></script></body></html>`)
const browser = await chromium.launch()
const page = await browser.newPage()
await page.goto(`file://${process.cwd()}/probes/t7/out/ri.html`)
await page.waitForTimeout(400)
await page.click("#ri1")
console.log(JSON.stringify(await page.evaluate(() => {
  const snap = (el) => { const o = { tag: el.tagName.toLowerCase() }
    for (const a of el.getAttributeNames()) o[a] = el.getAttribute(a)
    o.children = [...el.children].map(snap); return o }
  return snap(document.getElementById("ri1"))
}), null, 1))
await browser.close()
