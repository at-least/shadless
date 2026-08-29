// probe: record radix toggle/toggle-group/label DOM facts (Wave C card 15)
import { build } from "esbuild"
import { chromium } from "playwright"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve } from "node:path"

const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { Toggle } from "@/registry/new-york-v4/ui/toggle";
import { ToggleGroup, ToggleGroupItem } from "@/registry/new-york-v4/ui/toggle-group";
import { Label } from "@/registry/new-york-v4/ui/label";
import { Input } from "@/registry/new-york-v4/ui/input";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement("div", null,
  React.createElement(Toggle, { id: "tg1" }, "Bold"),
  React.createElement(ToggleGroup, { type: "single", id: "grp" },
    React.createElement(ToggleGroupItem, { id: "gi1", value: "a" }, "A"),
    React.createElement(ToggleGroupItem, { id: "gi2", value: "b" }, "B"),
  ),
  React.createElement(Label, { htmlFor: "in1" }, "Name"),
  React.createElement(Input, { id: "in1" }),
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
    for (const a of el.getAttributeNames()) if (a !== "class") o[a] = el.getAttribute(a)
    o.children = [...el.children].map(snap)
    return o
  }
  const g = (id) => snap(document.getElementById(id))
  return { tg1: g("tg1"), grp: g("grp"), gi1: g("gi1"), gi2: g("gi2"),
    label: snap(document.querySelector("label")), ae: document.activeElement && document.activeElement.id }
})
const out = {}
out.initial = await dump()
await page.click("#tg1"); out.tgClick = await dump()
await page.click("#gi1"); out.gi1Click = await dump()
await page.click("#gi2"); out.gi2Click = await dump()
await page.click("#gi2"); out.gi2Click2 = await dump()
await page.keyboard.press("Escape")
out.kGrpR = await (async()=>{ await page.focus("#grp"); await page.keyboard.press("ArrowRight"); await page.waitForTimeout(150); return dump() })()
await page.keyboard.press("ArrowRight"); out.kGrpRR = await dump()
out.kGrpL = await (async()=>{ const p2=await browser.newPage(); await p2.goto(`file://${process.cwd()}/probes/t7/out/probe.html`); await p2.waitForTimeout(500); await p2.focus("#grp"); await p2.keyboard.press("ArrowLeft"); await p2.waitForTimeout(150); const r=await p2.evaluate(dumpFn); await p2.close(); return r })()
await browser.close()
console.log(JSON.stringify(out, null, 1))
