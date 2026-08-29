// probe: radio-group / collapsible / accordion oracle facts (Wave C card 16)
import { build } from "esbuild"
import { chromium } from "playwright"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve } from "node:path"
const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { RadioGroup, RadioGroupItem } from "@/registry/new-york-v4/ui/radio-group";
import { Collapsible, CollapsibleTrigger, CollapsibleContent } from "@/registry/new-york-v4/ui/collapsible";
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@/registry/new-york-v4/ui/accordion";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement("div", null,
  React.createElement(RadioGroup, { id: "rg" },
    React.createElement(RadioGroupItem, { id: "ri1", value: "a" }),
    React.createElement(RadioGroupItem, { id: "ri2", value: "b" }),
  ),
  React.createElement(Collapsible, { id: "co" },
    React.createElement(CollapsibleTrigger, { id: "ct" }, "Toggle"),
    React.createElement(CollapsibleContent, { id: "cc" }, "Content"),
  ),
  React.createElement(Accordion, { type: "single", id: "ac", collapsible: true },
    React.createElement(AccordionItem, { value: "x", id: "ai1" },
      React.createElement(AccordionTrigger, { id: "at1" }, "First"),
      React.createElement(AccordionContent, { id: "acc1" }, "Body 1"),
    ),
    React.createElement(AccordionItem, { value: "y", id: "ai2" },
      React.createElement(AccordionTrigger, { id: "at2" }, "Second"),
      React.createElement(AccordionContent, { id: "acc2" }, "Body 2"),
    ),
  ),
));
`
mkdirSync("probes/t7/out", { recursive: true })
writeFileSync("probes/t7/out/.pc-entry.mjs", entry)
await build({ entryPoints: ["probes/t7/out/.pc-entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/t7/out/pc.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") }, loader: { ".tsx": "tsx" } })
writeFileSync("probes/t7/out/pc.html",
  `<!doctype html><html><body><div id="root"></div><script src="pc.js"></script></body></html>`)
const browser = await chromium.launch()
const page = await browser.newPage()
await page.goto(`file://${process.cwd()}/probes/t7/out/pc.html`)
await page.waitForTimeout(500)
const dump = (sel) => page.evaluate((sel) => {
  const snap = (el, depth) => {
    if (!el || depth > 3) return null
    const o = { tag: el.tagName.toLowerCase() }
    for (const a of el.getAttributeNames()) if (a !== "class") o[a] = el.getAttribute(a)
    o.children = [...el.children].map((c) => snap(c, depth + 1))
    return o
  }
  const out = {}
  for (const id of sel) out[id] = snap(document.getElementById(id), 0)
  out.ae = document.activeElement && (document.activeElement.id || document.activeElement.tagName)
  return out
}, sel)
async function show(label, sel) { console.log(label, JSON.stringify(await dump(sel))) }
await show("initial", ["rg", "ri1", "ri2", "co", "ct", "cc", "ac", "at1", "acc1", "at2"])
await page.click("#ri1"); await show("ri1-click", ["ri1", "ri2"])
await page.click("#ri2"); await show("ri2-click", ["ri1", "ri2"])
await page.click("#ct"); await show("coll-open", ["co", "ct", "cc"])
await page.click("#ct"); await show("coll-close", ["ct", "cc"])
await page.click("#at1"); await show("acc1-open", ["ac", "at1", "acc1", "at2", "acc2"])
await page.click("#at2"); await show("acc2-open", ["at1", "acc1", "at2", "acc2"])
await page.click("#at2"); await show("acc2-close", ["at2", "acc2"])
await browser.close()
