// H5: ORACLE = shadcn itself. Bundle the actual registry dialog.tsx (+button
// dependency) with esbuild, render in chromium via React, record the same
// facts as the spike, diff. Proves the source of truth is the shadcn file,
// not bare radix primitives.
import { build } from "esbuild"
import { chromium } from "playwright"
import { readFileSync, writeFileSync } from "node:fs"
import { resolve } from "node:path"

const recorder = `
window.__log = [];
window.__facts = function (tag) {
  var doc = document;
  function attrs(el) {
    if (!el) return null;
    var o = { tag: el.tagName.toLowerCase() };
    for (var a of el.getAttributeNames()) if (a !== "class") o[a] = el.getAttribute(a);
    o.text = (el.textContent || "").trim().slice(0, 24);
    return o;
  }
  window.__log.push({
    step: tag,
    bodyChildren: Array.from(doc.body.children).map(function (c) {
      var s = c.getAttribute("data-slot");
      return c.tagName.toLowerCase() + (s ? "[slot=" + s + "]"
        : c.tagName === "SPAN" ? "[guard?]" : "");
    }),
    trigger: attrs(doc.querySelector("[data-slot=dialog-trigger]")),
    overlay: attrs(doc.querySelector("[data-slot=dialog-overlay]")),
    content: attrs(doc.querySelector("[data-slot=dialog-content]")),
    title: attrs(doc.querySelector("[data-slot=dialog-title]")),
    description: attrs(doc.querySelector("[data-slot=dialog-description]")),
    close: attrs(doc.querySelector("[data-slot=dialog-close]")),
    activeElement: doc.activeElement
      ? (doc.activeElement.getAttribute("data-slot") ||
         doc.activeElement.tagName.toLowerCase()) : null,
    scrollLock: {
      attr: doc.body.getAttribute("data-scroll-locked"),
      pointerEvents: doc.body.style.pointerEvents,
    },
  });
};
`

const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/registry/new-york-v4/ui/dialog";
${recorder}
const tree = (open) => React.createElement(Dialog, {
  open, onOpenChange: (o) => { root.render(tree(o)) },
},
  React.createElement(DialogTrigger, { id: "d1-trigger" }, "Open dialog"),
  React.createElement(DialogContent, null,
    React.createElement(DialogHeader, null,
      React.createElement(DialogTitle, null, "Are you absolutely sure?"),
      React.createElement(DialogDescription, null, "This action cannot be undone."),
    ),
  ),
);
const root = createRoot(document.getElementById("root"));
root.render(tree(true));
setTimeout(function () { window.__facts("shadcn-open") }, 200);
`
writeFileSync("probes/h3b/.entry-shadcn.mjs", entry)
await build({
  entryPoints: ["probes/h3b/.entry-shadcn.mjs"], bundle: true, format: "iife",
  outfile: "probes/h3b/shadcn-bundle.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") },
  // jsx runtime is fine (react/jsx-runtime auto); tsx parsed automatically
  loader: { ".tsx": "tsx" },
})
writeFileSync("probes/h3b/shadcn.html",
  `<!doctype html><html><body><div id="root"></div><script src="shadcn-bundle.js"></script></body></html>`)

// spike page + driver (same as h3b)
const demo = readFileSync("probes/h4/demo.html", "utf8")
  .replace('href="out.css"', 'href="../h4/out.css"')
  .replace('src="vendor/', 'src="../h4/vendor/')
  .replace('src="shadless-glue.js"', 'src="../h4/shadless-glue.js"')
writeFileSync("probes/h3b/spike.html", demo)

const browser = await chromium.launch()

const overlayPoint = async (page) => {
  const ov = await page.locator("[data-slot=dialog-overlay]").boundingBox()
  const ct = await page.locator("[data-slot=dialog-content]").boundingBox()
  let x = ov.x + 15, y = ov.y + 15
  const inCt = (px, py) => px >= ct.x && px <= ct.x + ct.width && py >= ct.y && py <= ct.y + ct.height
  if (inCt(x, y)) { x = ov.x + ov.width - 15; y = ov.y + 15 }
  if (inCt(x, y)) throw new Error("no overlay-only point")
  return { x, y }
}
const stepIt = async (page, step) => {
  if (step === "overlay-mouse-click") {
    const { x, y } = await overlayPoint(page)
    await page.mouse.move(x, y); await page.mouse.down(); await page.mouse.up()
  } else if (step === "escape") await page.keyboard.press("Escape")
  else if (step === "close-button") await page.click("[data-slot=dialog-close]")
  await page.waitForTimeout(350)
  return page.evaluate(
    `!document.querySelector("[data-slot=dialog-content]") ? "closes" : "open"`)
}

async function oracleRun(step) {
  const page = await browser.newPage()
  await page.goto("file://" + process.cwd() + "/probes/h3b/shadcn.html")
  await page.waitForTimeout(500)
  const result = step ? await stepIt(page, step) : null
  const log = await page.evaluate("window.__log")
  await page.close()
  return { result, log }
}
async function spikeRun(step) {
  const page = await browser.newPage()
  await page.goto("file://" + process.cwd() + "/probes/h3b/spike.html")
  await page.addScriptTag({ content: recorder })
  await page.waitForTimeout(400)
  await page.click("#d1-trigger")
  await page.waitForTimeout(300)
  await page.evaluate(`window.__facts("spike-open")`)
  const result = step ? await stepIt(page, step) : null
  const log = await page.evaluate("window.__log")
  await page.close()
  return { result, log }
}

const STEPS = ["overlay-mouse-click", "escape", "close-button"]
const oracleS = {}, spikeS = {}
let oracleOpen, spikeOpen
for (let i = 0; i <= STEPS.length; i++) {
  const step = STEPS[i] || null
  const p = await oracleRun(step)
  const s = await spikeRun(step)
  if (i === 0) { oracleOpen = p.log.find(f => f.step === "shadcn-open"); spikeOpen = s.log[0] }
  if (step) { oracleS[step] = p.result; spikeS[step] = s.result }
}
await browser.close()

writeFileSync("probes/out/h5.json",
  JSON.stringify({ oracleOpen, spikeOpen, oracle: oracleS, spike: spikeS }, null, 2))

const keys = ["trigger", "overlay", "content", "title", "description", "close"]
console.log("=== OPEN-state diff (shadcn oracle vs spike) ===")
for (const k of keys) {
  const a = oracleOpen?.[k] || {}, b = spikeOpen?.[k] || {}
  const all = [...new Set([...Object.keys(a), ...Object.keys(b)])]
  const diffs = all.filter(kk => (a[kk] ?? "<absent>") !== (b[kk] ?? "<absent>"))
    .map(kk => `    ${kk}: shadcn=${JSON.stringify(a[kk])}  spike=${JSON.stringify(b[kk])}`)
  console.log(`  ${k}: ${diffs.length ? "DIFF" : "match"}`)
  diffs.forEach(d => console.log(d))
}
console.log("  bodyChildren:", JSON.stringify(oracleOpen?.bodyChildren), "vs", JSON.stringify(spikeOpen?.bodyChildren))
console.log("  activeElement:", oracleOpen?.activeElement, "vs", spikeOpen?.activeElement)
console.log("  scrollLock:", JSON.stringify(oracleOpen?.scrollLock), "vs", JSON.stringify(spikeOpen?.scrollLock))
console.log("\n=== dismiss scenarios ===")
for (const s of STEPS)
  console.log(`  ${s}: shadcn=${oracleS[s]}  spike=${spikeS[s]}`)
