// probe: toggle-group arrow semantics from various focus positions
import { build } from "esbuild"
import { chromium } from "playwright"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve } from "node:path"
const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { ToggleGroup, ToggleGroupItem } from "@/registry/new-york-v4/ui/toggle-group";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement(ToggleGroup, { type: "single", id: "grp" },
  React.createElement(ToggleGroupItem, { id: "gi1", value: "a" }, "A"),
  React.createElement(ToggleGroupItem, { id: "gi2", value: "b" }, "B"),
));
`
mkdirSync("probes/t7/out", { recursive: true })
writeFileSync("probes/t7/out/.pk-entry.mjs", entry)
await build({ entryPoints: ["probes/t7/out/.pk-entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/t7/out/pk.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") }, loader: { ".tsx": "tsx" } })
writeFileSync("probes/t7/out/pk.html",
  `<!doctype html><html><body><div id="root"></div><script src="pk.js"></script></body></html>`)
const browser = await chromium.launch()
const state = (page) => page.evaluate(`["gi1","gi2"].map(id=>{var e=document.getElementById(id);
  return e.getAttribute("data-state")+"/"+e.getAttribute("tabindex")}).join(" ")+" ae="+(document.activeElement.id||document.activeElement.tagName)`)
async function run(ops) {
  const page = await browser.newPage()
  await page.goto(`file://${process.cwd()}/probes/t7/out/pk.html`)
  await page.waitForTimeout(400)
  for (const op of ops) {
    if (op.startsWith("focus:")) await page.focus(op.slice(6))
    else if (op.startsWith("key:")) await page.keyboard.press(op.slice(4))
    else if (op.startsWith("click:")) await page.click(op.slice(6))
    await page.waitForTimeout(150)
  }
  const r = await state(page)
  await page.close()
  return r
}
for (const ops of [
  ["focus:#grp","key:ArrowRight"],
  ["focus:#grp","key:ArrowRight","key:ArrowRight"],
  ["focus:#grp","key:ArrowLeft"],
  ["focus:#grp","key:ArrowRight","key:Space"],
  ["click:#gi1","key:ArrowRight"],
  ["click:#gi1","key:ArrowRight","key:Space"],
  ["click:#gi1","key:ArrowDown"],
  ["click:#gi2","key:ArrowRight"],
  ["click:#gi1","key:Home"],
  ["focus:#grp","key:Home"],
]) console.log(JSON.stringify(ops), "→", await run(ops))
await browser.close()
