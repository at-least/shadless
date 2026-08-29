// probe: toggle-group / accordion type=multiple oracle facts (refuter GAP 1)
import { build } from "esbuild"
import { chromium } from "playwright"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve } from "node:path"
const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { ToggleGroup, ToggleGroupItem } from "@/registry/new-york-v4/ui/toggle-group";
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@/registry/new-york-v4/ui/accordion";
createRoot(document.getElementById("root")).render(React.createElement("div", null,
  React.createElement(ToggleGroup, { type: "multiple", id: "grp" },
    React.createElement(ToggleGroupItem, { id: "gi1", value: "a" }, "A"),
    React.createElement(ToggleGroupItem, { id: "gi2", value: "b" }, "B"),
  ),
  React.createElement(Accordion, { type: "multiple", id: "ac" },
    React.createElement(AccordionItem, { value: "x" },
      React.createElement(AccordionTrigger, { id: "d1-at1" }, "First"),
      React.createElement(AccordionContent, null, "Body 1"),
    ),
    React.createElement(AccordionItem, { value: "y" },
      React.createElement(AccordionTrigger, { id: "d1-at2" }, "Second"),
      React.createElement(AccordionContent, null, "Body 2"),
    ),
  ),
));
`
mkdirSync("probes/t7/out", { recursive: true })
writeFileSync("probes/t7/out/.pm-entry.mjs", entry)
await build({ entryPoints: ["probes/t7/out/.pm-entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/t7/out/pm.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") }, loader: { ".tsx": "tsx" } })
writeFileSync("probes/t7/out/pm.html", `<!doctype html><html><body><div id="root"></div><script src="pm.js"></script></body></html>`)
const browser = await chromium.launch()
const page = await browser.newPage()
await page.goto(`file://${process.cwd()}/probes/t7/out/pm.html`)
await page.waitForTimeout(500)
const st = () => page.evaluate(() => {
  const a = (id) => { const e = document.getElementById(id); if (!e) return null
    const o = { tag: e.tagName.toLowerCase() }
    for (const x of e.getAttributeNames()) if (x !== "class") o[x] = e.getAttribute(x)
    return o }
  return { grp: a("grp"), gi1: a("gi1"), gi2: a("gi2"),
    at1: a("d1-at1"), at2: a("d1-at2"),
    acc1: document.querySelector("[data-slot=accordion-item] [data-slot=accordion-content]").getAttribute("data-state"),
    ae: document.activeElement.id || document.activeElement.tagName }
})
const step = async (label, fn) => { await fn(); await page.waitForTimeout(200); console.log(label, JSON.stringify(await st())) }
console.log("initial", JSON.stringify(await st()))
await step("gi1", async () => page.click("#gi1"))
await step("gi2-after-gi1", async () => page.click("#gi2"))
await step("gi2-again", async () => page.click("#gi2"))
await step("at1-open", async () => page.click("#d1-at1"))
await step("at2-open", async () => page.click("#d1-at2"))
await step("at1-close", async () => page.click("#d1-at1"))
await browser.close()
