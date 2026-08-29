// H4 assert: (1) compiled CSS contains theme + state-variant + animate utilities;
// (2) jsdom: kernel-wired dialog reproduces the radix attribute/focus contract.
import { readFileSync } from "node:fs"
import { JSDOM } from "jsdom"

let fails = 0
const ok = (name, cond) => { console.log(`${cond ? "PASS" : "FAIL"}  ${name}`); if (!cond) fails++ }

// --- 1. CSS ---------------------------------------------------------------
const css = readFileSync("probes/h4/out.css", "utf8")
ok("css: theme color wired (--primary var consumed)", /var\(--primary\)/.test(css))
ok("css: .bg-primary utility", /\.bg-primary\s*\{/.test(css))
ok("css: data-[state=open] variant utility (animate-in)",
  css.includes(".data-\\[state\\=open\\]\\:animate-in"))
ok("css: tw-animate-css zoom-in-95 state variant",
  css.includes("zoom-in-95"))
ok("css: responsive sm:max-w-lg", css.includes(".sm\\:max-w-lg"))
ok("css: dark variant custom", /\.dark/.test(css))
ok("css: sr-only", /\.sr-only/.test(css))

// --- 2. jsdom behavior ------------------------------------------------------
const html = readFileSync("probes/h4/demo.html", "utf8")
const kernel = readFileSync("probes/h4/vendor/radix-kernel-dialog.iife.js", "utf8")
const glue = readFileSync("probes/h4/shadless-glue.js", "utf8")

const dom = new JSDOM(html.replace(/<script[^>]*><\/script>/g, ""), {
  url: "http://probe.local/", pretendToBeVisual: true, runScripts: "outside-only",
})
const { window: win } = dom
win.eval(kernel)
win.eval(glue)

const doc = win.document
const trigger = doc.getElementById("d1-trigger")
const click = el => el.dispatchEvent(new win.MouseEvent("click", { bubbles: true, cancelable: true }))

click(trigger)
const overlay = doc.querySelector("[data-slot=dialog-overlay]")
const content = doc.querySelector("[data-slot=dialog-content]")
ok("dom: portal mounted to body", !!overlay && !!content && content.closest("body") === doc.body)
ok("dom: trigger data-state=open + aria-expanded", trigger.getAttribute("data-state") === "open" && trigger.getAttribute("aria-expanded") === "true")
ok("dom: overlay data-state=open + aria-hidden + pointer-events:auto", overlay?.getAttribute("data-state") === "open" && overlay.getAttribute("aria-hidden") === "true" && overlay.style.pointerEvents === "auto")
ok("dom: content data-state=open + role=dialog + no aria-modal (h3b)", content.getAttribute("data-state") === "open" && content.getAttribute("role") === "dialog" && !content.hasAttribute("aria-modal"))
ok("dom: title is h2 + description is p + aria wiring", doc.querySelector("[data-slot=dialog-title]").tagName === "H2" && doc.querySelector("[data-slot=dialog-description]").tagName === "P" && content.getAttribute("aria-labelledby") === "d1-title" && content.getAttribute("aria-describedby") === "d1-desc" && trigger.getAttribute("aria-controls") === "d1")
ok("dom: focus landed on first focusable (close button)", doc.activeElement?.getAttribute("data-slot") === "dialog-close")
ok("dom: scroll lock armed (data-scroll-locked + pointer-events)", doc.body.getAttribute("data-scroll-locked") === "1")

// escape closes
doc.dispatchEvent(new win.KeyboardEvent("keydown", { key: "Escape", bubbles: true }))
await new Promise(r => setTimeout(r, 250)) // exit window
ok("dom: portal removed after Escape", !doc.querySelector("[data-slot=dialog-content]"))
ok("dom: trigger data-state=closed + aria-expanded=false", trigger.getAttribute("data-state") === "closed" && trigger.getAttribute("aria-expanded") === "false")
ok("dom: focus returned to trigger", doc.activeElement === trigger)

// reopen: close-button inside content closes
click(trigger)
doc.querySelector("[data-slot=dialog-close]").dispatchEvent(
  new win.MouseEvent("click", { bubbles: true, cancelable: true }))
await new Promise(r => setTimeout(r, 250))
ok("dom: dialog-close button closes", !doc.querySelector("[data-slot=dialog-content]"))

// reopen: overlay click closes (kernel semantics = click, matches primitives h3b)
click(trigger)
const ov2 = doc.querySelector("[data-slot=dialog-overlay]")
ov2.dispatchEvent(new win.MouseEvent("click", { bubbles: true, cancelable: true }))
await new Promise(r => setTimeout(r, 250))
ok("dom: overlay click closes", !doc.querySelector("[data-slot=dialog-content]"))

console.log(fails ? `\n${fails} FAILURES` : "\nALL PASS")
process.exit(fails ? 1 : 0)
