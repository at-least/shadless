// runtime (src/runtime/core.js + components/*) — jsdom behavior tests (Wave H).
// Pattern proven by probes/h4/assert.mjs: jsdom covers state-machine
// assertions; real-pointer/browser stays in the contracts suite.
// Covers the R1 (sibling template via init(subRoot)) and R2 (malformed
// markup guards) regressions plus roving-focus math for all three groups.
// Wave J3: lifecycle additions — init(root,{force}) / refresh / start/stop.
// jsdom never loads images: img.complete stays false, so avatar settle is
// driven deterministically by dispatching "error" (the exact listener the
// runtime attaches for not-yet-complete images).
import { readFileSync } from "node:fs"
import { JSDOM } from "jsdom"
import { build as esbuild } from "esbuild"
// The ESM surface is composed by the Go builder (pipeline/jsbuild.go). This
// suite reads the SHIPPED files rather than recomposing them: recomposing
// meant a second implementation of the same format, and the point of the
// assertions below is what a bundler sees in dist/, not what a helper agrees
// to produce.
const dist = (rel) => readFileSync(new URL(`../../dist/esm/${rel}`, import.meta.url), "utf8")
// the base's public members, read back out of the shipped module so the list
// cannot drift from what is actually exported
const NAMED_EXPORTS = (dist("shadless.mjs").match(/export const \{ ([^}]*) \} = shadless/) ?? [, ""])[1]
  .split(",").map((s) => s.trim()).filter(Boolean)

// the runtime is now a base + one file per component: boot the base with
// every trivial-tier behavior registered (what a page that uses them loads)
import { readdirSync } from "node:fs"
const TRIVIAL = ["accordion", "avatar", "checkbox", "collapsible", "radio-group", "switch", "toggle", "toggle-group"]
const RUNTIME = [readFileSync(new URL("../../src/runtime/core.js", import.meta.url), "utf8"),
  ...TRIVIAL.map((c) => readFileSync(new URL(`../../src/runtime/components/${c}.js`, import.meta.url), "utf8"))].join("\n;\n")

// kernel-family boot: vendored kernel + base + one component file, the exact
// concatenation dist/shadless.js + dist/js/<name>.js ships
const KERNEL = readFileSync(new URL("../../vendor/radix-kernel.iife.js", import.meta.url), "utf8")
const CORE = readFileSync(new URL("../../src/runtime/core.js", import.meta.url), "utf8")
const component = (c) => readFileSync(new URL(`../../src/runtime/components/${c}.js`, import.meta.url), "utf8")
function bootKernel(html, comps) {
  const dom = new JSDOM(`<!doctype html><html><body>${html}</body></html>`,
    { runScripts: "dangerously", url: "http://localhost/", pretendToBeVisual: true })
  dom.window.eval([KERNEL, CORE, ...comps.map(component)].join("\n;\n"))
  return dom
}
function boot(html) {
  const dom = new JSDOM(`<!doctype html><html><body>${html}</body></html>`,
    { runScripts: "dangerously", url: "http://localhost/" })
  dom.window.eval(RUNTIME)
  return dom
}
const click = (dom, el) => el.dispatchEvent(new dom.window.MouseEvent("click", { bubbles: true }))
const key = (dom, el, k) =>
  el.dispatchEvent(new dom.window.KeyboardEvent("keydown", { key: k, bubbles: true }))
const tick = () => new Promise((r) => setTimeout(r, 0))
// fire the avatar settle path for every image currently listening
const settleAll = (dom, scope) =>
  [...scope.querySelectorAll("[data-slot=avatar-image]")].forEach((img) =>
    img.dispatchEvent(new dom.window.Event("error")))
const freshImg = (doc, src) => {
  const img = doc.createElement("img")
  img.setAttribute("data-slot", "avatar-image")
  img.setAttribute("src", src)
  return img
}

export async function run(t) {
  // ---- checkbox: state + indicator via SIBLING template (R1 regression) ----
  {
    const dom = boot(`
<div id="app"><button data-slot="checkbox" role="checkbox" aria-checked="false"></button></div>
<template data-for="checkbox-indicator"><span data-slot="checkbox-indicator">✓</span></template>`)
    const root = dom.window.document.getElementById("app")
    dom.window.shadless.init(root) // sub-root init, template is a SIBLING
    const box = root.querySelector("[data-slot=checkbox]")
    click(dom, box)
    t.eq("checkbox: checked attrs", [box.getAttribute("aria-checked"), box.getAttribute("data-state")], ["true", "checked"])
    t.ok("checkbox: indicator mounted from sibling template (R1)",
      !!box.querySelector("[data-slot=checkbox-indicator]"))
    click(dom, box)
    t.ok("checkbox: indicator unmounted", !box.querySelector("[data-slot=checkbox-indicator]"))
    t.eq("checkbox: unchecked attrs", box.getAttribute("aria-checked"), "false")
  }

  // ---- checkbox: EMPTY template must not throw (R2) ----
  {
    const dom = boot(`
<button data-slot="checkbox" aria-checked="false"></button>
<template data-for="checkbox-indicator"></template>`)
    dom.window.shadless.initAll()
    const box = dom.window.document.querySelector("[data-slot=checkbox]")
    let threw = false
    try { click(dom, box) } catch { threw = true }
    t.ok("checkbox: empty template doesn't throw (R2)", !threw)
    t.eq("checkbox: state still toggles", box.getAttribute("aria-checked"), "true")
  }

  // ---- switch / toggle ----
  {
    const dom = boot(`
<button data-slot="switch" role="switch" aria-checked="false"><span data-slot="switch-thumb"></span></button>
<button data-slot="toggle" aria-pressed="false"></button>`)
    dom.window.shadless.initAll()
    const sw = dom.window.document.querySelector("[data-slot=switch]")
    click(dom, sw)
    t.eq("switch: root+thumb sync", [sw.getAttribute("data-state"),
      sw.querySelector("[data-slot=switch-thumb]").getAttribute("data-state")], ["checked", "checked"])
    const tg = dom.window.document.querySelector("[data-slot=toggle]")
    click(dom, tg); click(dom, tg)
    t.eq("toggle: double click returns off", tg.getAttribute("data-state"), "off")
  }

  // ---- radio-group: click exclusive + arrows check-if-none quirk ----
  {
    const dom = boot(`
<div data-slot="radio-group" role="radiogroup">
  <button data-slot="radio-group-item" role="radio" aria-checked="false" tabindex="0">1</button>
  <button data-slot="radio-group-item" role="radio" aria-checked="false" tabindex="-1">2</button>
  <button data-slot="radio-group-item" role="radio" aria-checked="false" tabindex="-1">3</button>
</div>
<template data-for="radio-group-indicator"><span data-slot="radio-group-indicator">●</span></template>`)
    dom.window.shadless.initAll()
    const doc = dom.window.document
    const group = doc.querySelector("[data-slot=radio-group]")
    const items = [...group.querySelectorAll("[data-slot=radio-group-item]")]
    click(dom, items[1])
    t.eq("radio: exclusive check", items.map((i) => i.getAttribute("aria-checked")), ["false", "true", "false"])
    t.ok("radio: indicator mounted", !!items[1].querySelector("[data-slot=radio-group-indicator]"))
    // ArrowDown from item[1] → focus item[2]; selection unchanged (one checked)
    key(dom, items[1], "ArrowDown")
    t.eq("radio: arrow moves focus", doc.activeElement, items[2])
    t.eq("radio: selection unchanged", items[1].getAttribute("aria-checked"), "true")
    // Home → first; End → last (roving math)
    key(dom, items[2], "Home")
    t.eq("radio: Home → first", doc.activeElement, items[0])
    key(dom, items[0], "End")
    t.eq("radio: End → last", doc.activeElement, items[2])
  }
  {
    // no-checked quirk: first arrow BOTH moves focus and checks the target
    const dom = boot(`
<div data-slot="radio-group" role="radiogroup">
  <button data-slot="radio-group-item" role="radio" aria-checked="false" tabindex="0">1</button>
  <button data-slot="radio-group-item" role="radio" aria-checked="false" tabindex="-1">2</button>
</div>`)
    dom.window.shadless.initAll()
    const items = [...dom.window.document.querySelectorAll("[data-slot=radio-group-item]")]
    key(dom, items[0], "ArrowDown")
    t.eq("radio: none-checked arrow checks target", items.map((i) => i.getAttribute("aria-checked")), ["false", "true"])
  }

  // ---- toggle-group: single (items role=radio) vs multiple (aria-pressed) ----
  {
    const dom = boot(`
<div data-slot="toggle-group" role="group">
  <button data-slot="toggle-group-item" role="radio" aria-checked="false" data-state="off" tabindex="0">A</button>
  <button data-slot="toggle-group-item" role="radio" aria-checked="false" data-state="off" tabindex="-1">B</button>
</div>
<div data-slot="toggle-group" role="toolbar">
  <button data-slot="toggle-group-item" aria-pressed="false" data-state="off" tabindex="0">X</button>
  <button data-slot="toggle-group-item" aria-pressed="false" data-state="off" tabindex="-1">Y</button>
</div>
<button data-slot="toggle-group-item" data-state="off">orphan</button>`)
    dom.window.shadless.initAll()
    const doc = dom.window.document
    const single = [...doc.querySelectorAll('[role=group] [data-slot=toggle-group-item]')]
    click(dom, single[0])
    t.eq("tg single: select", single[0].getAttribute("data-state"), "on")
    click(dom, single[1])
    t.eq("tg single: exclusive", [single[0].getAttribute("data-state"), single[1].getAttribute("data-state")], ["off", "on"])
    click(dom, single[1])
    t.eq("tg single: click-again deselects", single[1].getAttribute("data-state"), "off")
    const toolbar = [...doc.querySelectorAll('[role=toolbar] [data-slot=toggle-group-item]')]
    click(dom, toolbar[0]); click(dom, toolbar[1])
    t.eq("tg toolbar: aria-pressed both on", toolbar.map((i) => i.getAttribute("aria-pressed")), ["true", "true"])
    // roving arrows in toolbar move focus with wrap
    key(dom, toolbar[1], "ArrowRight")
    t.eq("tg: arrow wraps to first", doc.activeElement, toolbar[0])
    // R2: orphan item (no group) must not throw
    const orphan = doc.querySelector("button:last-of-type")
    let threw = false
    try { click(dom, orphan) } catch { threw = true }
    t.ok("tg: orphan item doesn't throw (R2)", !threw)
  }

  // ---- accordion: single closes siblings; multiple independent ----
  {
    const dom = boot(`
<div data-slot="accordion">
  <div data-slot="accordion-item"><button data-slot="accordion-trigger" aria-expanded="false" data-state="closed">1</button><div data-slot="accordion-content" hidden data-state="closed"></div></div>
  <div data-slot="accordion-item"><button data-slot="accordion-trigger" aria-expanded="false" data-state="closed">2</button><div data-slot="accordion-content" hidden data-state="closed"></div></div>
</div>`)
    dom.window.shadless.initAll()
    const doc = dom.window.document
    const triggers = [...doc.querySelectorAll("[data-slot=accordion-trigger]")]
    click(dom, triggers[0])
    t.eq("accordion: first opens", triggers[0].getAttribute("data-state"), "open")
    click(dom, triggers[1])
    t.eq("accordion: single closes sibling",
      [triggers[0].getAttribute("data-state"), triggers[1].getAttribute("data-state")], ["closed", "open"])
    t.ok("accordion: content unhidden", !doc.querySelectorAll("[data-slot=accordion-content]")[1].hasAttribute("hidden"))
  }
  {
    // R2: malformed sibling trigger (no item/content ancestors) mid-loop
    const dom = boot(`
<div data-slot="accordion">
  <div data-slot="accordion-item"><button data-slot="accordion-trigger" data-state="closed">1</button><div data-slot="accordion-content" hidden></div></div>
  <button data-slot="accordion-trigger" data-state="closed">orphan</button>
  <div data-slot="accordion-item"><button data-slot="accordion-trigger" data-state="closed">2</button><div data-slot="accordion-content" hidden></div></div>
</div>`)
    dom.window.shadless.initAll()
    const triggers = [...dom.window.document.querySelectorAll("[data-slot=accordion-trigger]")]
    let threw = false
    try { click(dom, triggers[2]) } catch { threw = true }
    t.ok("accordion: malformed sibling doesn't throw (R2)", !threw)
    t.eq("accordion: clicked still opens", triggers[2].getAttribute("data-state"), "open")
    t.eq("accordion: well-formed sibling closed", triggers[0].getAttribute("data-state"), "closed")
  }
  {
    // multiple mode + missing content degrades to trigger-only state (R2)
    const dom = boot(`
<div data-slot="accordion" data-type="multiple">
  <div data-slot="accordion-item"><button data-slot="accordion-trigger" data-state="closed">1</button></div>
</div>`)
    dom.window.shadless.initAll()
    const tr = dom.window.document.querySelector("[data-slot=accordion-trigger]")
    let threw = false
    try { click(dom, tr) } catch { threw = true }
    t.ok("accordion: missing content doesn't throw (R2)", !threw)
    t.eq("accordion: trigger state syncs without content", tr.getAttribute("data-state"), "open")
  }

  // ---- init/destroy lifecycle ----
  {
    const dom = boot(`<div id="a"><button data-slot="toggle" aria-pressed="false"></button></div>`)
    const doc = dom.window.document
    const a = doc.getElementById("a")
    dom.window.shadless.init(a)
    const tg = a.querySelector("[data-slot=toggle]")
    click(dom, tg)
    t.eq("lifecycle: init works", tg.getAttribute("aria-pressed"), "true")
    dom.window.shadless.destroy(a)
    click(dom, tg)
    t.eq("lifecycle: destroy stops dispatch", tg.getAttribute("aria-pressed"), "true")
  }

  // ---- J3: init(root, {force}) — framework cache-restored DOM ----
  {
    const dom = boot(`
<div id="a"><span data-slot="avatar"><img data-slot="avatar-image" src="x"><span data-slot="avatar-fallback">FB</span></span>
<button data-slot="toggle" aria-pressed="false"></button></div>`)
    const doc = dom.window.document
    const a = doc.getElementById("a")
    dom.window.shadless.init(a)
    settleAll(dom, a)
    t.ok("force: initial settle ran", !a.querySelector("[data-slot=avatar-image]"))
    // framework restores DOM: fresh img re-inserted into the live root —
    // delegation covers clicks, but the init-time settle has not run for it
    const av = a.querySelector("[data-slot=avatar]")
    av.appendChild(freshImg(doc, "y"))
    settleAll(dom, a) // no listener on the fresh img → must NOT settle
    t.ok("force: restored img unsettled before re-init", !!av.querySelector("[data-slot=avatar-image]"))
    dom.window.shadless.init(a, { force: true })
    settleAll(dom, a)
    t.ok("force: re-init re-ran settle", !av.querySelector("[data-slot=avatar-image]"))
    t.ok("force: fallback survived", !!av.querySelector("[data-slot=avatar-fallback]"))
    // delegation alive and not duplicated (one click = one toggle)
    const tg = a.querySelector("[data-slot=toggle]")
    click(dom, tg)
    t.eq("force: delegation alive", tg.getAttribute("aria-pressed"), "true")
    click(dom, tg)
    t.eq("force: no double dispatch", tg.getAttribute("aria-pressed"), "false")
  }

  // ---- forms: `name` → hidden inputs mirror the value; reset restores ----
  {
    const dom = boot(`
<form id="f">
<button type="button" data-slot="checkbox" role="checkbox" aria-checked="false" name="tos" value="yes"></button>
<button type="button" data-slot="switch" role="switch" aria-checked="true" name="notify"></button>
<div data-slot="radio-group" role="radiogroup" name="plan">
  <button type="button" data-slot="radio-group-item" role="radio" aria-checked="true" value="free"></button>
  <button type="button" data-slot="radio-group-item" role="radio" aria-checked="false" value="pro"></button>
</div>
<button type="button" data-slot="checkbox" role="checkbox" aria-checked="false" id="anon"></button>
</form>`)
    const doc = dom.window.document
    dom.window.shadless.initAll()
    const form = doc.getElementById("f")
    const data = () => [...new dom.window.FormData(form).entries()]
    t.eq("form: initial state mirrored (unchecked checkbox absent, switch on, radio value)", data(), [["notify", "on"], ["plan", "free"]])
    click(dom, doc.querySelector("[name=tos]"))
    click(dom, doc.querySelector("[name=notify]"))
    click(dom, doc.querySelectorAll("[data-slot=radio-group-item]")[1])
    t.eq("form: changes mirrored (checkbox submits its value attr)", data(), [["tos", "yes"], ["plan", "pro"]])
    t.ok("form: a control without name adds nothing", !doc.getElementById("anon").nextElementSibling)
    form.reset()
    await tick()
    t.eq("form: reset restores the initial state", data(), [["notify", "on"], ["plan", "free"]])
    t.eq("form: reset restored the visible state too", doc.querySelector("[name=tos]").getAttribute("aria-checked"), "false")
  }
  {
    const dom = bootKernel(`
<form id="f">
<span data-slot="slider" id="sl" name="vol"><span data-slot="slider-track"><span data-slot="slider-range"></span></span><span data-slot="slider-thumb" role="slider" aria-valuenow="20"></span></span>
<button type="button" data-slot="select-trigger" id="s1-trigger" name="fruit"><span data-slot="select-value">a</span></button>
<template id="s1-tpl"><div data-slot="select-content"><div data-slot="select-viewport"><div role="option" aria-selected="true" data-value="A">a</div><div role="option" aria-selected="false" data-value="B">b</div></div></div></template>
</form>`, ["slider", "select"])
    const doc = dom.window.document
    dom.window.shadless.initAll()
    const form = doc.getElementById("f")
    const data = () => [...new dom.window.FormData(form).entries()]
    t.eq("form: slider + select initial", data(), [["vol", "20"], ["fruit", "A"]])
    dom.window.shadless.get("#sl").setValue(60)
    const sel = dom.window.shadless.get("#s1-trigger")
    sel.open(); sel.select(sel.selected().nextElementSibling)
    t.eq("form: slider + select after change", data(), [["vol", "60"], ["fruit", "B"]])
    form.reset()
    await tick()
    t.eq("form: slider + select reset", data(), [["vol", "20"], ["fruit", "A"]])
    t.eq("form: select handle reflects the reset", sel.value(), "A")
  }

  // ---- roving focus skips disabled items; horizontal arrows swap under dir=rtl ----
  {
    const dom = boot(`
<div data-slot="radio-group" id="rg" role="radiogroup">
  <button data-slot="radio-group-item" role="radio" aria-checked="false" value="a" tabindex="0"></button>
  <button data-slot="radio-group-item" role="radio" aria-checked="false" value="b" disabled></button>
  <button data-slot="radio-group-item" role="radio" aria-checked="false" value="c" tabindex="-1"></button>
</div>
<div data-slot="toggle-group" id="tg" role="group" dir="rtl">
  <button data-slot="toggle-group-item" role="radio" aria-checked="false" data-state="off" value="x" tabindex="0"></button>
  <button data-slot="toggle-group-item" role="radio" aria-checked="false" data-state="off" value="y" tabindex="-1"></button>
  <button data-slot="toggle-group-item" role="radio" aria-checked="false" data-state="off" value="z" tabindex="-1"></button>
</div>`)
    const doc = dom.window.document
    dom.window.shadless.initAll()
    const items = [...doc.querySelectorAll("#rg [data-slot=radio-group-item]")]
    const changes = []
    doc.addEventListener("shadless:change", (e) => changes.push(e.detail.value))
    items[0].focus()
    key(dom, items[0], "ArrowDown")
    t.eq("disabled: ArrowDown skips the disabled radio", doc.activeElement, items[2])
    t.eq("disabled: with nothing checked the arrow checks the ENABLED target, never the disabled one", changes, ["c"])
    t.eq("disabled: disabled item stays unchecked", items[1].getAttribute("aria-checked"), "false")
    key(dom, items[2], "ArrowDown")
    t.eq("disabled: wraps to the first, over the disabled one", doc.activeElement, items[0])
    const tg = [...doc.querySelectorAll("#tg [data-slot=toggle-group-item]")]
    tg[0].focus()
    key(dom, tg[0], "ArrowRight")
    t.eq("rtl: ArrowRight moves BACKWARD under dir=rtl (wraps to the last)", doc.activeElement, tg[2])
    key(dom, tg[2], "ArrowLeft")
    t.eq("rtl: ArrowLeft moves forward under dir=rtl", doc.activeElement, tg[0])
    key(dom, tg[0], "ArrowDown")
    t.eq("rtl: vertical arrows are not swapped", doc.activeElement, tg[1])
  }
  {
    const dom = bootKernel(`
<div data-slot="tabs" id="tabs" dir="rtl"><div data-slot="tabs-list"><button data-slot="tabs-trigger" aria-controls="p0" data-state="active">0</button><button data-slot="tabs-trigger" aria-controls="p1" data-state="inactive" disabled>1</button><button data-slot="tabs-trigger" aria-controls="p2" data-state="inactive">2</button></div>
<div data-slot="tabs-content" id="p0"></div><div data-slot="tabs-content" id="p1" hidden></div><div data-slot="tabs-content" id="p2" hidden></div></div>`, ["tabs"])
    const doc = dom.window.document
    dom.window.shadless.initAll()
    const tr = [...doc.querySelectorAll("[data-slot=tabs-trigger]")]
    tr[0].focus()
    key(dom, tr[0], "ArrowLeft") // rtl: forward
    t.eq("tabs rtl: ArrowLeft is forward and skips the disabled trigger", dom.window.shadless.get("#tabs").active(), 2)
    key(dom, tr[2], "ArrowLeft")
    t.eq("tabs rtl: wraps", dom.window.shadless.get("#tabs").active(), 0)
    key(dom, tr[0], "ArrowRight") // rtl: backward → wraps to 2 (1 disabled)
    t.eq("tabs rtl: ArrowRight is backward", dom.window.shadless.get("#tabs").active(), 2)
  }

  // ---- destroy tears down kernel-family wiring: open portals come down, glue listeners go, re-init wires ONCE ----
  {
    const dom = bootKernel(`
<div id="a"><button data-slot="dialog-trigger" id="d1-trigger">open</button></div>
<template id="d1-portal"><div data-slot="dialog-overlay"></div><div data-slot="dialog-content" id="d1-content" role="dialog"><button data-slot="dialog-close">x</button></div></template>
<div id="b"><div data-slot="tabs" id="tabs"><div data-slot="tabs-list"><button data-slot="tabs-trigger" aria-controls="p0" data-state="active">0</button><button data-slot="tabs-trigger" aria-controls="p1" data-state="inactive">1</button></div>
<div data-slot="tabs-content" id="p0"></div><div data-slot="tabs-content" id="p1" hidden></div></div></div>`, ["dialog", "tabs"])
    const doc = dom.window.document
    const a = doc.getElementById("a"), b = doc.getElementById("b")
    const opens = []
    doc.addEventListener("shadless:open", (e) => opens.push(e.detail.component))
    dom.window.shadless.init(a)
    const tr = doc.getElementById("d1-trigger")
    click(dom, tr)
    t.ok("destroy: dialog opened (portal in body)", !!doc.querySelector("[data-slot=dialog-portal]"))
    dom.window.shadless.destroy(a)
    t.ok("destroy: open portal removed", !doc.querySelector("[data-slot=dialog-portal]"))
    t.eq("destroy: handle dropped", dom.window.shadless.get(tr), null)
    click(dom, tr)
    t.ok("destroy: trigger inert after destroy", !doc.querySelector("[data-slot=dialog-portal]"))
    dom.window.shadless.init(a)
    click(dom, tr)
    t.eq("destroy: re-init wires the trigger exactly once (one open event)", opens.filter((c) => c === "dialog").length, 2)
    t.ok("destroy: re-init opens", !!doc.querySelector("[data-slot=dialog-portal]"))
    dom.window.shadless.get(tr).close()
    // tabs: kernel-held wiring is persistent — destroy + init reuse it, no double emit
    const changes = []
    doc.addEventListener("shadless:change", (e) => changes.push(e.detail.index))
    dom.window.shadless.init(b)
    dom.window.shadless.destroy(b)
    dom.window.shadless.init(b)
    dom.window.shadless.get("#tabs").activate(1)
    t.eq("destroy: persistent tabs wiring reused, change emitted once", changes, [1])
    t.ok("destroy: persistent handle survives", typeof dom.window.shadless.get("#tabs").active === "function")
    t.eq("handles carry their component (the TS discriminant)", [dom.window.shadless.get("#tabs").component, dom.window.shadless.get(tr).component], ["tabs", "dialog"])
  }

  // ---- J3: refresh(element) — init-time behaviors on injected subtrees ----
  {
    const dom = boot(`
<div id="a"><span data-slot="avatar"><img data-slot="avatar-image" src="x"><span data-slot="avatar-fallback">FB</span></span></div>`)
    const doc = dom.window.document
    const a = doc.getElementById("a")
    dom.window.shadless.init(a)
    // injected SECOND avatar (realistic shape: one img per avatar — avatar
    // init grabs querySelector's first match, so two imgs in ONE avatar
    // would not reach the new one)
    const av2 = doc.createElement("span")
    av2.setAttribute("data-slot", "avatar")
    av2.innerHTML = '<img data-slot="avatar-image" src="y"><span data-slot="avatar-fallback">FB2</span>'
    a.appendChild(av2)
    dom.window.shadless.refresh(a) // attaches settle listener on the new img
    av2.querySelector("[data-slot=avatar-image]")
      .dispatchEvent(new dom.window.Event("error"))
    t.ok("refresh: injected avatar settled", !av2.querySelector("[data-slot=avatar-image]"))
    t.ok("refresh: existing avatar untouched",
      !!a.querySelector("#a > span[data-slot=avatar] > [data-slot=avatar-image]"))
    let threw = false
    try { dom.window.shadless.refresh(doc.createTextNode("x")) } catch { threw = true }
    t.ok("refresh: non-element ignored", !threw)
  }

  // ---- J3: start()/stop() — opt-in observer over added subtrees ----
  {
    const dom = boot(`<div id="a"></div>`)
    const doc = dom.window.document
    const a = doc.getElementById("a")
    dom.window.shadless.init(a)
    dom.window.shadless.start()
    dom.window.shadless.start() // idempotent
    const av1 = doc.createElement("span")
    av1.setAttribute("data-slot", "avatar")
    av1.innerHTML = '<img data-slot="avatar-image" src="x"><span data-slot="avatar-fallback">FB</span>'
    a.appendChild(av1)
    await tick() // observer callbacks run as microtasks
    settleAll(dom, a)
    t.ok("observer: added subtree settled", !av1.querySelector("[data-slot=avatar-image]"))
    dom.window.shadless.stop()
    const av2 = doc.createElement("span")
    av2.setAttribute("data-slot", "avatar")
    av2.innerHTML = '<img data-slot="avatar-image" src="x"><span data-slot="avatar-fallback">FB</span>'
    a.appendChild(av2)
    await tick()
    settleAll(dom, a)
    t.ok("observer: stop() halts rescans", !!av2.querySelector("[data-slot=avatar-image]"))
    let threw = false
    try { dom.window.shadless.stop() } catch { threw = true } // stop without start
    t.ok("observer: stop idempotent", !threw)
  }

  // ---- handles + events: dialog through shadless.get and the user gesture ----
  {
    const dom = bootKernel(`
<div id="app"><button data-slot="dialog-trigger" id="d1-trigger">open</button></div>
<template id="d1-portal"><div data-slot="dialog-overlay"></div><div data-slot="dialog-content" id="d1-content" role="dialog"><button data-slot="dialog-close">x</button></div></template>`, ["dialog"])
    const doc = dom.window.document
    dom.window.shadless.initAll()
    const trig = doc.getElementById("d1-trigger")
    const events = []
    doc.addEventListener("shadless:open", (e) => events.push(["open", e.detail.component, e.target.id, e.detail.api && e.detail.api.isOpen()]))
    doc.addEventListener("shadless:close", (e) => events.push(["close", e.detail.component, e.target.id, e.detail.api && e.detail.api.isOpen()]))
    const api = dom.window.shadless.get("#d1-trigger")
    t.ok("handle: get(selector) finds the dialog", !!api && typeof api.open === "function")
    t.eq("handle: closed initially", api.isOpen(), false)
    api.open()
    t.ok("handle: open() mounts the portal", !!doc.querySelector("[data-slot=dialog-portal] [data-slot=dialog-content]"))
    t.eq("handle: isOpen after open()", api.isOpen(), true)
    t.ok("handle: get(inner element) walks up to the instance", dom.window.shadless.get(trig.firstChild) === api)
    api.close()
    await new Promise((r) => setTimeout(r, 250)) // presence exit
    t.eq("handle: isOpen after close()", api.isOpen(), false)
    t.ok("handle: portal unmounted", !doc.querySelector("[data-slot=dialog-portal]"))
    click(dom, trig)
    click(dom, trig)
    await new Promise((r) => setTimeout(r, 250))
    t.eq("events: open/close bubble to document, detail {component, api}, state already flipped", events, [
      ["open", "dialog", "d1-trigger", true], ["close", "dialog", "d1-trigger", false],
      ["open", "dialog", "d1-trigger", true], ["close", "dialog", "d1-trigger", false],
    ])
    t.ok("handle: get() returns null outside any instance", dom.window.shadless.get(doc.body) === null)
  }
  // ---- shadless:change / open / close across the trivial tier (one boot, document delegation) ----
  {
    const dom = boot(`
<button data-slot="checkbox" role="checkbox" aria-checked="false"></button>
<button data-slot="switch" role="switch" aria-checked="false"><span data-slot="switch-thumb"></span></button>
<button data-slot="toggle" aria-pressed="false"></button>
<div data-slot="radio-group" id="rg"><button data-slot="radio-group-item" value="a" aria-checked="false"></button><button data-slot="radio-group-item" value="b" aria-checked="false"></button></div>
<div data-slot="toggle-group" id="tg1"><button data-slot="toggle-group-item" role="radio" value="x" data-state="off"></button><button data-slot="toggle-group-item" role="radio" value="y" data-state="off"></button></div>
<div data-slot="toggle-group" id="tgm"><button data-slot="toggle-group-item" value="p" data-state="off"></button><button data-slot="toggle-group-item" value="q" data-state="off"></button></div>
<div data-slot="collapsible"><button data-slot="collapsible-trigger" id="ct" data-state="closed"></button><div data-slot="collapsible-content" id="cc" hidden></div></div>
<div data-slot="accordion"><div data-slot="accordion-item"><button data-slot="accordion-trigger" id="a1" data-state="closed"></button><div data-slot="accordion-content" id="a1c" hidden></div></div>
<div data-slot="accordion-item"><button data-slot="accordion-trigger" id="a2" data-state="closed"></button><div data-slot="accordion-content" id="a2c" hidden></div></div></div>`)
    const doc = dom.window.document
    dom.window.shadless.initAll()
    const log = []
    for (const type of ["shadless:change", "shadless:open", "shadless:close"])
      doc.addEventListener(type, (e) => log.push([type.slice(9), e.detail.component, e.target.id || e.target.getAttribute("data-slot"),
        JSON.stringify(Object.fromEntries(Object.entries(e.detail).filter(([k]) => !["component", "api", "item", "trigger"].includes(k))))]))
    const $ = (sel) => doc.querySelector(sel)
    click(dom, $("[data-slot=checkbox]")); click(dom, $("[data-slot=checkbox]"))
    click(dom, $("[data-slot=switch]"))
    click(dom, $("[data-slot=toggle]"))
    const [ra, rb] = doc.querySelectorAll("[data-slot=radio-group-item]")
    click(dom, ra); click(dom, ra); click(dom, rb) // re-clicking the checked radio is not a change
    const [tx, ty] = $("#tg1").querySelectorAll("[data-slot=toggle-group-item]")
    click(dom, tx); click(dom, ty); click(dom, ty) // single: x → y → none
    const [tp, tq] = $("#tgm").querySelectorAll("[data-slot=toggle-group-item]")
    click(dom, tp); click(dom, tq); click(dom, tp) // multiple: [p] → [p,q] → [q]
    click(dom, $("#ct")); click(dom, $("#ct"))
    click(dom, $("#a1")); click(dom, $("#a2")) // single accordion: opening a2 closes a1 (close a1, open a2)
    t.eq("events: change/open/close across the trivial tier", log, [
      ["change", "checkbox", "checkbox", '{"checked":true}'], ["change", "checkbox", "checkbox", '{"checked":false}'],
      ["change", "switch", "switch", '{"checked":true}'],
      ["change", "toggle", "toggle", '{"pressed":true}'],
      ["change", "radio-group", "rg", '{"value":"a"}'], ["change", "radio-group", "rg", '{"value":"b"}'],
      ["change", "toggle-group", "tg1", '{"value":"x"}'], ["change", "toggle-group", "tg1", '{"value":"y"}'], ["change", "toggle-group", "tg1", '{"value":null}'],
      ["change", "toggle-group", "tgm", '{"value":["p"]}'], ["change", "toggle-group", "tgm", '{"value":["p","q"]}'], ["change", "toggle-group", "tgm", '{"value":["q"]}'],
      ["open", "collapsible", "ct", "{}"], ["close", "collapsible", "ct", "{}"],
      ["open", "accordion", "a1", "{}"], ["close", "accordion", "a1", "{}"], ["open", "accordion", "a2", "{}"],
    ])
  }

  // ---- shadless:change on the kernel families: tabs (kernel onChange), slider (handle), select (Enter path through handles.select) ----
  {
    const dom = bootKernel(`
<div data-slot="tabs" id="tabs"><div data-slot="tabs-list"><button data-slot="tabs-trigger" aria-controls="p0" data-state="active">0</button><button data-slot="tabs-trigger" aria-controls="p1" data-state="inactive">1</button></div>
<div data-slot="tabs-content" id="p0"></div><div data-slot="tabs-content" id="p1" hidden></div></div>
<span data-slot="slider" id="sl"><span data-slot="slider-track"><span data-slot="slider-range"></span></span><span data-slot="slider-thumb" role="slider" aria-valuenow="20"></span></span>
<button data-slot="select-trigger" id="s1-trigger"><span data-slot="select-value">a</span></button>
<template id="s1-tpl"><div data-slot="select-content"><div data-slot="select-viewport"><div role="option" aria-selected="true" id="oa" data-value="A">a</div><div role="option" aria-selected="false" id="ob" data-value="B">b</div><div role="option" aria-selected="false">c</div></div></div></template>`,
      ["tabs", "slider", "select"])
    const doc = dom.window.document
    dom.window.shadless.initAll()
    const log = []
    doc.addEventListener("shadless:change", (e) => log.push([e.detail.component, e.target.id, e.detail.index, e.detail.values, e.detail.value]))
    doc.addEventListener("shadless:commit", (e) => log.push(["commit:" + e.detail.component, e.target.id, undefined, e.detail.values, undefined]))
    dom.window.shadless.get("#tabs").activate(1)
    dom.window.shadless.get("#sl").setValue(60)
    dom.window.shadless.get("#sl").setValue(70, 0, { commit: true }) // change + commit
    const sel = dom.window.shadless.get("#s1-trigger")
    sel.open()
    sel.select("#ob")
    t.eq("events: change on tabs / slider / select", log, [
      ["tabs", "tabs", 1, undefined, undefined],
      ["slider", "sl", undefined, [60], undefined],
      ["slider", "sl", undefined, [70], undefined], ["commit:slider", "sl", undefined, [70], undefined],
      ["select", "s1-trigger", undefined, undefined, "B"],
    ])
    t.eq("select: value() is the option's data-value, not its label", sel.value(), "B")
    t.eq("select: label() is the shown text", sel.label(), "b")
    t.eq("select: selected() is the option", sel.selected().id, "ob")
    sel.select(sel.selected().parentElement.lastElementChild) // no value, no data-value, no id
    t.eq("select: value() falls back to the label for an option with no value/data-value/id", sel.value(), "c")
  }

  // ---- trivial tier: no handle by design — state is the attribute, click is the API ----
  {
    const dom = boot(`<button data-slot="checkbox" role="checkbox" aria-checked="false"><span>x</span></button>`)
    dom.window.shadless.initAll()
    const box = dom.window.document.querySelector("[data-slot=checkbox]")
    t.ok("trivial: get(checkbox) is null (no wrapper by design)", dom.window.shadless.get(box) === null)
    box.click()
    t.eq("trivial: el.click() is the API", box.getAttribute("aria-checked"), "true")
  }

  // ---- ESM entry: what a bundler sees — base + component modules, any import order ----
  {
    // take the SHIPPED esm surface and bundle a consumer that imports the
    // COMPONENT before the base: the component module's own
    // `import "./shadless.mjs"` must still evaluate the base first
    const files = { "/esm/shadless.mjs": dist("shadless.mjs"), "/esm/dialog.mjs": dist("dialog.mjs") }
    const consumer = `import "./esm/dialog.mjs"
import shadless, { get, theme, init } from "./esm/shadless.mjs"
import * as ns from "./esm/shadless.mjs"
window.__esm = { default: shadless, get, theme, init, named: Object.keys(ns).sort() }`
    const virtual = { name: "virtual", setup(b) {
      b.onResolve({ filter: /.*/ }, (a) => ({ path: a.path.startsWith("./esm/") ? a.path.slice(1) : a.path, namespace: "v" }))
      b.onLoad({ filter: /.*/, namespace: "v" }, (a) => ({ contents: files[a.path] ?? consumer, loader: "js" }))
    } }
    const out = (await esbuild({ stdin: { contents: consumer, resolveDir: "/", loader: "js" }, bundle: true, format: "iife",
      write: false, plugins: [virtual], logLevel: "silent" })).outputFiles[0].text
    const dom = new JSDOM(`<!doctype html><html><body><button data-slot="dialog-trigger" id="d1-trigger">o</button>
<template id="d1-portal"><div data-slot="dialog-content" id="d1-content"></div></template></body></html>`,
      { runScripts: "dangerously", url: "http://localhost/", pretendToBeVisual: true })
    dom.window.eval(out)
    await tick()
    const esm = dom.window.__esm
    t.ok("esm: default export is window.shadless", esm.default === dom.window.shadless)
    t.eq("esm: named exports = the base's public members", esm.named, ["default", ...NAMED_EXPORTS].sort())
    t.ok("esm: named get() is the same function", esm.get === dom.window.shadless.get)
    const api = esm.get("#d1-trigger")
    t.ok("esm: component imported BEFORE the base still registers (import order independent)", !!api && typeof api.open === "function")
    api.open()
    t.eq("esm: handle works through the bundle", api.isOpen(), true)
  }

  // ---- J5: theme API — next-themes vanilla semantics ----
  {
    const dom = boot(`<div data-slot="toggle"></div>`)
    const doc = dom.window.document
    const events = []
    doc.addEventListener("shadless:themechange", (e) => events.push(e.detail.mode))
    t.eq("theme: initial get", dom.window.shadless.theme.get(), "light")
    dom.window.shadless.theme.set("dark")
    t.ok("theme: set toggles html.dark", doc.documentElement.classList.contains("dark"))
    t.eq("theme: persisted", dom.window.localStorage.getItem("shadless-theme"), "dark")
    t.eq("theme: change event", events, ["dark"])
    t.eq("theme: get after set", dom.window.shadless.theme.get(), "dark")
    dom.window.shadless.theme.toggle()
    t.ok("theme: toggle clears dark", !doc.documentElement.classList.contains("dark"))
    t.eq("theme: toggle persists", dom.window.localStorage.getItem("shadless-theme"), "light")
    t.eq("theme: toggle event", events, ["dark", "light"])
  }
}
