// H3b v2: record radix primitives Dialog contract in a REAL browser
// (playwright + chromium), same facts from the kernel-driven spike, diff.
// Pages own the recording: window.__facts(tag) pushes a snapshot into
// window.__log; window.__scenario(kind) performs a step + records + returns
// "closes"|"open". Node orchestrates, pulls __log, diffs, writes report.
import { build } from "esbuild"
import { chromium } from "playwright"
import { readFileSync, writeFileSync, mkdirSync } from "node:fs"

mkdirSync("probes/h3b", { recursive: true })

// shared vanilla recorder injected into BOTH pages
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
        : c.getAttribute("role") ? "[role=" + c.getAttribute("role") + "]"
        : c.tagName === "SPAN" ? "[guard?]" : "");
    }),
    trigger: attrs(doc.querySelector("[data-slot=dialog-trigger]")),
    overlay: attrs(doc.querySelector("[data-slot=dialog-overlay]")),
    content: attrs(doc.querySelector("[data-slot=dialog-content], [role=dialog]")),
    title: attrs(doc.querySelector("[data-slot=dialog-title]")),
    description: attrs(doc.querySelector("[data-slot=dialog-description]")),
    close: attrs(doc.querySelector("[data-slot=dialog-close]")),
    activeElement: doc.activeElement
      ? (doc.activeElement.getAttribute("data-slot") || doc.activeElement.id ||
         doc.activeElement.tagName.toLowerCase()) : null,
    scrollLock: {
      attr: doc.body.getAttribute("data-scroll-locked"),
      overflow: doc.body.style.overflow,
      pointerEvents: doc.body.style.pointerEvents,
      paddingRight: doc.body.style.paddingRight !== "",
    },
  });
};
`

// --- primitives entry (bundled React + radix) --------------------------------
const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import * as D from "@radix-ui/react-dialog";
${recorder}
const tree = (open) => React.createElement(D.Root, {
  open,
  onOpenChange: (o) => { window.__open = o; root.render(tree(o)) },
},
  React.createElement(D.Trigger, { "data-slot": "dialog-trigger", id: "rt-trigger" }, "Open"),
  React.createElement(D.Portal, { "data-slot": "dialog-portal" },
    React.createElement(D.Overlay, { "data-slot": "dialog-overlay", className: "ov" }),
    React.createElement(D.Content, { "data-slot": "dialog-content", className: "ct" },
      React.createElement(D.Title, { "data-slot": "dialog-title" }, "Sure?"),
      React.createElement(D.Description, { "data-slot": "dialog-description" }, "Undone."),
      React.createElement(D.Close, { "data-slot": "dialog-close" }, "x"),
    ),
  ),
);
const root = createRoot(document.getElementById("root"));
root.render(tree(true));
window.__scenario = async function (kind) {
  var doc = document;
  if (kind === "overlay-pointerdown")
    doc.querySelector("[data-slot=dialog-overlay]").dispatchEvent(
      new PointerEvent("pointerdown", { bubbles: true, cancelable: true, pointerId: 1 }));
  else if (kind === "overlay-click")
    doc.querySelector("[data-slot=dialog-overlay]").dispatchEvent(
      new MouseEvent("click", { bubbles: true, cancelable: true }));
  else if (kind === "escape")
    doc.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true }));
  else if (kind === "close-button")
    doc.querySelector("[data-slot=dialog-close]").click();
  else if (kind === "reopen") root.render(tree(true));
  await new Promise(function (r) { setTimeout(r, 250) });
  var gone = !doc.querySelector("[data-slot=dialog-content], [role=dialog]");
  window.__facts("after-" + kind);
  return gone ? "closes" : "open";
};
setTimeout(function () { window.__facts("primitives-open") }, 150);
`
writeFileSync("probes/h3b/.entry.mjs", entry)
await build({
  entryPoints: ["probes/h3b/.entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/h3b/primitives-bundle.js", logLevel: "error",
})
writeFileSync("probes/h3b/primitives.html",
  `<!doctype html><html><body><div id="root"></div><script src="primitives-bundle.js"></script></body></html>`)

// --- spike page (kernel + glue from h4) + vanilla scenario -------------------
const demo = readFileSync("probes/h4/demo.html", "utf8")
  .replace('href="out.css"', 'href="../h4/out.css"')
  .replace('src="vendor/', 'src="../h4/vendor/')
  .replace('src="shadless-glue.js"', 'src="../h4/shadless-glue.js"')
writeFileSync("probes/h3b/spike.html", demo)
writeFileSync("probes/h3b/spike-driver.js", `
${recorder}
window.__scenario = async function (kind) {
  var doc = document;
  if (kind === "open") doc.getElementById("d1-trigger").click();
  else if (kind === "overlay-pointerdown")
    doc.querySelector("[data-slot=dialog-overlay]").dispatchEvent(
      new PointerEvent("pointerdown", { bubbles: true, cancelable: true, pointerId: 1 }));
  else if (kind === "overlay-click")
    doc.querySelector("[data-slot=dialog-overlay]").dispatchEvent(
      new MouseEvent("click", { bubbles: true, cancelable: true }));
  else if (kind === "escape")
    doc.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true }));
  else if (kind === "close-button")
    doc.querySelector("[data-slot=dialog-close]").click();
  else if (kind === "reopen") doc.getElementById("d1-trigger").click();
  await new Promise(function (r) { setTimeout(r, 300) });
  var gone = !doc.querySelector("[data-slot=dialog-content], [role=dialog]");
  window.__facts("after-" + kind);
  return gone ? "closes" : "open";
};
`)

// --- run ---------------------------------------------------------------------
const browser = await chromium.launch()

// each dismiss step on a FRESH page: no cross-step state contamination
// (primitives is controlled via onOpenChange; spike trigger is a toggle).
// overlay steps use REAL mouse (synthetic PointerEvent gets filtered by
// radix's click-like checks — pointerType/button/isPrimary).
async function freshRun(file, driver, openStep, step) {
  const page = await browser.newPage()
  await page.goto("file://" + process.cwd() + "/probes/h3b/" + file)
  if (driver) await page.addScriptTag({ path: "probes/h3b/" + driver })
  await page.waitForTimeout(400)
  if (openStep)
    await page.evaluate(`window.__scenario(${JSON.stringify(openStep)})`)
  let result = null
  if (step === "overlay-mouse-click" || step === "overlay-mouse-down-only") {
    const ov = await page.locator("[data-slot=dialog-overlay]").boundingBox()
    const ct = await page.locator("[data-slot=dialog-content], [role=dialog]").boundingBox()
    // pick a point inside the overlay but OUTSIDE the centered content
    let x = ov.x + 15, y = ov.y + 15
    const inCt = (px, py) => px >= ct.x && px <= ct.x + ct.width && py >= ct.y && py <= ct.y + ct.height
    if (inCt(x, y)) { x = ov.x + ov.width - 15; y = ov.y + 15 }
    if (inCt(x, y)) { x = ov.x + 15; y = ov.y + ov.height - 15 }
    if (inCt(x, y)) throw new Error("no overlay-only point found")
    await page.mouse.move(x, y)
    await page.mouse.down()
    if (step === "overlay-mouse-click") await page.mouse.up()
    await page.waitForTimeout(350)
    result = await page.evaluate(
      `!document.querySelector("[data-slot=dialog-content], [role=dialog]") ? "closes" : "open"`)
    await page.evaluate(`window.__facts("after-${step}")`)
  } else if (step) {
    result = await page.evaluate(`window.__scenario(${JSON.stringify(step)})`)
  }
  const log = await page.evaluate("window.__log")
  const openFact = openStep
    ? log.find(f => f.step === "after-" + openStep)
    : log.find(f => f.step === "primitives-open")
  await page.close()
  return { result, openFact }
}

const STEPS = ["overlay-mouse-click", "overlay-mouse-down-only", "escape", "close-button"]
const primScenarios = {}, spikeScenarios = {}
let primOpen, spikeOpen
for (let i = 0; i <= STEPS.length; i++) {
  const step = STEPS[i] || null
  const p = await freshRun("primitives.html", null, null, step)
  const s = await freshRun("spike.html", "spike-driver.js", "open", step)
  if (i === 0) { primOpen = p.openFact; spikeOpen = s.openFact }
  if (step) { primScenarios[step] = p.result; spikeScenarios[step] = s.result }
}
await browser.close()

const out = { primOpen, spikeOpen, primScenarios, spikeScenarios }
writeFileSync("probes/out/h3b.json", JSON.stringify(out, null, 2))

// --- diff report ---------------------------------------------------------------
const openFact = (f) => f || {}
const keys = ["trigger", "overlay", "content", "title", "description", "close"]
console.log("=== OPEN-state diff (primitives vs spike) ===")
for (const k of keys) {
  const a = openFact(primOpen)[k] || {}, b = openFact(spikeOpen)[k] || {}
  const all = [...new Set([...Object.keys(a), ...Object.keys(b)])]
  const diffs = all.filter(kk => (a[kk] ?? "<absent>") !== (b[kk] ?? "<absent>"))
    .map(kk => `    ${kk}: primitives=${JSON.stringify(a[kk])}  spike=${JSON.stringify(b[kk])}`)
  console.log(`  ${k}: ${diffs.length ? "DIFF" : "match"}`)
  diffs.forEach(d => console.log(d))
}
console.log("  bodyChildren:", JSON.stringify(primOpen?.bodyChildren), "vs", JSON.stringify(spikeOpen?.bodyChildren))
console.log("  activeElement:", primOpen?.activeElement, "vs", spikeOpen?.activeElement)
console.log("  scrollLock:", JSON.stringify(primOpen?.scrollLock), "vs", JSON.stringify(spikeOpen?.scrollLock))
console.log("\n=== dismiss scenarios (fresh page each) ===")
for (const s of STEPS)
  console.log(`  ${s}: primitives=${primScenarios[s]}  spike=${spikeScenarios[s]}`)
