// probe: record radix checkbox/switch DOM facts (oracle shapes for Wave C)
import { build } from "esbuild"
import { chromium } from "playwright"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve } from "node:path"

const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { Checkbox } from "@/registry/new-york-v4/ui/checkbox";
import { Switch } from "@/registry/new-york-v4/ui/switch";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement("div", null,
  React.createElement(Checkbox, { id: "c1" }),
  React.createElement(Switch, { id: "s1" }),
));
`
mkdirSync("probes/t7/out", { recursive: true })
writeFileSync("probes/t7/out/.probe-entry.mjs", entry)
await build({
  entryPoints: ["probes/t7/out/.probe-entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/t7/out/probe.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") },
  loader: { ".tsx": "tsx" },
})
writeFileSync("probes/t7/out/probe.html",
  `<!doctype html><html><body><div id="root"></div><script src="probe.js"></script></body></html>`)

const browser = await chromium.launch()
const page = await browser.newPage()
await page.goto(`file://${process.cwd()}/probes/t7/out/probe.html`)
await page.waitForTimeout(500)
const dump = () => page.evaluate(() => {
  const snap = (el) => {
    if (!el) return null
    const o = { tag: el.tagName.toLowerCase() }
    for (const a of el.getAttributeNames()) o[a] = el.getAttribute(a)
    o.children = [...el.children].map(snap)
    return o
  }
  return { checkbox: snap(document.getElementById("c1")), switch: snap(document.getElementById("s1")) }
})
const out = {}
out.initial = await dump()
await page.click("#c1"); out.checkboxClick1 = await dump()
await page.click("#s1"); out.switchClick1 = await dump()
await page.keyboard.press("Space"); out.checkboxSpace = await dump()
await browser.close()
console.log(JSON.stringify(out, null, 1))
