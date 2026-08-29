// probe: radio-group / accordion keyboard semantics + item attrs (card 16)
import { build } from "esbuild"
import { chromium } from "playwright"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve } from "node:path"
const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { RadioGroup, RadioGroupItem } from "@/registry/new-york-v4/ui/radio-group";
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@/registry/new-york-v4/ui/accordion";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement("div", null,
  React.createElement(RadioGroup, { id: "rg" },
    React.createElement(RadioGroupItem, { id: "ri1", value: "a" }),
    React.createElement(RadioGroupItem, { id: "ri2", value: "b" }),
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
writeFileSync("probes/t7/out/.pk3-entry.mjs", entry)
await build({ entryPoints: ["probes/t7/out/.pk3-entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/t7/out/pk3.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") }, loader: { ".tsx": "tsx" } })
writeFileSync("probes/t7/out/pk3.html",
  `<!doctype html><html><body><div id="root"></div><script src="pk3.js"></script></body></html>`)
const browser = await chromium.launch()
const page = await browser.newPage()
await page.goto(`file://${process.cwd()}/probes/t7/out/pk3.html`)
await page.waitForTimeout(500)
const st = () => page.evaluate(() => {
  const a = (id) => { const e = document.getElementById(id); const o = {}
    for (const x of e.getAttributeNames()) if (x !== "class") o[x] = e.getAttribute(x)
    return o }
  return { ri1: a("ri1"), ri2: a("ri2"), ai1: a("ai1"), ai2: a("ai2"),
    at1: a("at1"), acc1: a("acc1"), ri1kids: document.getElementById("ri1").children.length,
    ae: document.activeElement.id || document.activeElement.tagName }
})
const step = async (label, fn) => { await fn(); await page.waitForTimeout(200); console.log(label, JSON.stringify(await st())) }
console.log("initial", JSON.stringify(await st()))
await step("rgArrowR", async () => { await page.focus("#rg"); await page.waitForTimeout(120); await page.keyboard.press("ArrowRight") })
await step("ri1checked+arrowR", async () => { await page.click("#ri1"); await page.keyboard.press("ArrowRight") })
await step("arrowR-again", async () => { await page.keyboard.press("ArrowRight") })
await step("click-checked-ri", async () => { await page.click("#ri1"); await page.waitForTimeout(100); await page.click("#ri1") })
await step("at1open", async () => { await page.click("#at1") })
await step("at1+arrowDown", async () => { await page.keyboard.press("ArrowDown") })
await step("at1+arrowDown+space", async () => { await page.keyboard.press("Space") })
await step("home", async () => { await page.keyboard.press("Home") })
await browser.close()
