// Contract harness: record facts from the shadcn oracle (React, real browser)
// and from the shadless page, diff after normalizing recorded differences.
// Usage: npm run contracts -- dialog
import { buildContractOracle } from "./oracle-build.mjs"
import { chromium } from "playwright"
import { readFileSync, writeFileSync, mkdirSync, readdirSync } from "node:fs"
import { resolve } from "node:path"
import { execFileSync } from "node:child_process"

const name = process.argv[2]
if (!name) {
  // no arg: run every component def in a child; exit 1 if any FAILs
  const all = readdirSync("tools/contracts/components")
    .filter((f) => f.endsWith(".mjs")).map((f) => f.slice(0, -4)).sort()
  let failed = 0
  for (const c of all) {
    console.log(`\n=== ${c} ===`)
    try { execFileSync(process.execPath, ["tools/contracts/run.mjs", c], { stdio: "inherit" }) }
    catch { failed++ }
  }
  console.log(failed ? `\nFAIL  contracts full-run (${failed}/${all.length} failed)` : `\nPASS  contracts full-run (${all.length} components)`)
  process.exit(failed ? 1 : 0)
}
// load only the requested def (a broken def file used to take down every
// other component through the unconditional dialog import)
const def = (await import(`./components/${name}.mjs`)).default

const OUT = `tools/contracts/out/${name}`

// ---------- recorder (injected into both pages) ---------------------------
function recorderSrc(slots) {
  return `
window.__facts = function (tag) {
  var doc = document;
  function attrs(el) {
    if (!el) return null;
    var o = { tag: el.tagName.toLowerCase() };
    for (var a of el.getAttributeNames()) if (a !== "class") o[a] = el.getAttribute(a);
    o.text = (el.textContent || "").trim().replace(/\\s+/g, "").slice(0, 24);
    return o;
  }
  var f = { step: tag };
  ${JSON.stringify(slots)}.forEach(function (s) {
    var el = s.charAt(0) === "&"
      ? doc.querySelector(s.slice(1)) // "&<raw-css>" — full selector
      : doc.querySelector("[data-slot=" + s + "]");
    f[s] = attrs(el);
  });
  f.activeElement = doc.activeElement
    ? (doc.activeElement.getAttribute("data-slot") ||
       doc.activeElement.tagName.toLowerCase()) : null;
  f.scrollLock = {
    attr: doc.body.getAttribute("data-scroll-locked"),
    pointerEvents: doc.body.style.pointerEvents,
  };
  return f;
};
`
}

// ---------- oracle build (shared with tools/example-fixture.mjs --contracts) --
await buildContractOracle(def, OUT, recorderSrc(def.slots))

// ---------- shadless page (relative-path rewrite until emitter lands) -------
let shadlessHtml = readFileSync(def.shadlessPage, "utf8")
{
  const dir = resolve(def.shadlessPage, "..")
  shadlessHtml = shadlessHtml.replace(/(src|href)="(?!https?:|file:|data:|\/\/)([^"]+)"/g,
    (m, k, v) => `${k}="${dir}/${v}"`)
}
writeFileSync(`${OUT}/shadless.html`, shadlessHtml)

// ---------- normalization of recorded differences --------------------------
// radix auto ids (`radix-_r_N…`, `radix-:rN:`) vs fixture ids (`d1-…`,
// `t1-…`, `p1-…`): both -> <auto-id>, including values referenced in
// aria-* attributes. Local fixture prefixes are any letter+digits stem
// (d1-, p1-, t1-, s1- …) so new fixture naming doesn't silently diff.
const AUTO = /^(radix-[\w:-]*|[a-z]+\d[\w-]*)$/
const normVal = (v, key) => {
  // style attr: keep only author styles — radix runtime injects --radix-* vars,
  // outline:none (focus) and pointer-events:auto (modal layer) into style
  if (key === "style" && typeof v === "string") {
    return v.split(";").map((s) => s.trim()).filter((s) =>
      s && !s.startsWith("--radix-") && !/^outline:\s*none$/.test(s) && !/^pointer-events:\s*auto$/.test(s)
    ).join("; ")
  }
  // (a bare "--radix-…" value can only be a style sub-declaration, which the
  // style branch above already filters — no separate case needed)
  if (typeof v === "string" && AUTO.test(v)) return "<auto-id>"
  if (typeof v === "string" && v.split(/\s+/).every((p) => AUTO.test(p) || p === "") && v)
    return v.split(/\s+/).map(() => "<auto-id>").join(" ")
  return v
}
function normFact(f) {
  if (!f) return f
  const out = {}
  for (const [k, v] of Object.entries(f)) {
    if (v && typeof v === "object" && "tag" in v) {
      const el = { tag: v.tag }
      for (const [a, av] of Object.entries(v))
        if (a !== "tag" && a !== "data-radix-collection-item") // radix collection internals
          el[a] = normVal(av, a)
      if (el.style === "") delete el.style // runtime-only style → absent
      out[k] = el
    } else out[k] = v
  }
  return out
}

// ---------- scenarios --------------------------------------------------------
async function overlayPoint(page, overlaySlot, contentSlot) {
  const ov = await page.locator(`[data-slot=${overlaySlot}]`).boundingBox()
  const ct = await page.locator(`[data-slot=${contentSlot}]`).boundingBox()
  if (!ov || !ct)
    throw new Error(`overlay-point: element not visible (overlay=${!!ov} content=${!!ct})`)
  let x = ov.x + 15, y = ov.y + 15
  const inCt = (px, py) =>
    px >= ct.x && px <= ct.x + ct.width && py >= ct.y && py <= ct.y + ct.height
  if (inCt(x, y)) { x = ov.x + ov.width - 15; y = ov.y + 15 }
  if (inCt(x, y)) throw new Error("no overlay-only point")
  return { x, y }
}
async function stepIt(page, step) {
  // ops chain ("focus:#t1+key:ArrowRight"); legacy single-op names still work
  for (const op of step.split("+")) {
    if (op === "overlay-mouse-click") {
      const { x, y } = await overlayPoint(page,
        def.overlaySlot || "dialog-overlay", def.contentSlot || "dialog-content")
      await page.mouse.move(x, y); await page.mouse.down(); await page.mouse.up()
    } else if (op === "escape") await page.keyboard.press("Escape")
    else if (op === "close-button") await page.click(def.closeSelector || "[data-slot=dialog-close]")
    else if (op === "outside-click") { await page.mouse.move(5, 5); await page.mouse.down(); await page.mouse.up() }
    // steps: real pointers move continuously; a single-jump move races radix's
    // async grace-tracker attach (stays open forever) — artifact, not semantics
    else if (op === "pointer-away") { await page.mouse.move(5, 5, { steps: 10 }); await page.waitForTimeout(400) }
    else if (op === "trigger-toggle") await page.click("[data-slot=" + def.triggerSlot + "]")
    // radix modal dropdown: body pointer-events:none while open — a real
    // playwright click on the trigger never lands; dispatch a DOM click instead
    else if (op.startsWith("js-click:"))
      await page.$eval(op.slice(9), (el) => el.click())
    // mouse click at element center — bypasses playwright actionability
    // (radix modal layers set pointer-events:none on the background)
    else if (op.startsWith("mouse-click:")) {
      const css = op.slice("mouse-click:".length)
      const box = await page.locator(css).boundingBox()
      if (!box) throw new Error(`mouse-click: ${css} not visible`)
      await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2)
    }
    // click at x%,y% inside element box (e.g. track clicks on a slider)
    else if (op.startsWith("clickAt:")) {
      const [sel, xy] = op.slice(8).split("@")
      const [fx, fy] = xy.split(",").map(Number)
      const box = await page.locator(sel).boundingBox()
      if (!box) throw new Error(`clickAt: ${sel} not visible`)
      await page.mouse.click(box.x + box.width * fx / 100, box.y + box.height * fy / 100)
    }
    // move pointer to element center (hover state), then optional wheel deltas
    else if (op.startsWith("move:")) {
      const css = op.slice(5)
      const box = await page.locator(css).boundingBox()
      if (!box) throw new Error(`move: ${css} not visible`)
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2, { steps: 5 })
    }
    else if (op.startsWith("wheel:")) {
      const [dx, dy] = op.slice(6).split(",").map(Number)
      await page.mouse.wheel(dx, dy)
    }
    // generic steps: click:<css> / focus:<css> / key:<Key>
    else if (op.startsWith("click:")) await page.click(op.slice(6))
    else if (op.startsWith("focus:")) {
      await page.focus(op.slice(6))
      await page.waitForTimeout(120) // radix roving-focus moves via rAF — settle
    } else if (op.startsWith("key:")) {
      await page.keyboard.press(op.slice(4))
      await page.waitForTimeout(120)
    }
  }
  await page.waitForTimeout(350)
  if (def.stateProbe) return page.evaluate(def.stateProbe)
  // presence probe: both sides remove the content (incl. portal wrapper) on
  // close — verified per-kernel (wrapper.remove / glue onClosed / React unmount)
  const contentSlot = def.contentSlot || "dialog-content"
  return page.evaluate(
    `!document.querySelector("[data-slot=${contentSlot}]") ? "closes" : "open"`)
}

// ---------- run both sides ---------------------------------------------------
const browser = await chromium.launch()
// mounted-diff bag: multiset of (tag|data-slot|sorted-class) triples for
// every element present AFTER open but not before — i.e. the JS-MOUNTED
// dynamic DOM (portal content, menu layers, viewport). Harness wrappers,
// focus guards (no class/slot) and pre-existing static markup are immune;
// class drift inside mounted content is exactly what this catches (the
// facts recorder deliberately ignores class, the golden gates only ever
// see the CLOSED initial state).
const mountedBagSrc = (withClasses) => `(() => {
  const bag = []
  for (const el of document.body.querySelectorAll("*")) {
    if (el.tagName === "SCRIPT" || el.tagName === "TEMPLATE") continue
    const cls = ${withClasses} ? (el.getAttribute("class") || "").split(/\\s+/).filter(Boolean).sort().join(" ") : ""
    bag.push(el.tagName + "|" + (el.getAttribute("data-slot") || "") + "|" + cls)
  }
  bag.sort()
  return bag
})()`
function bagDiff(before, after) {
  const remaining = [...before]
  const added = []
  for (const item of after) {
    const i = remaining.indexOf(item)
    if (i >= 0) remaining.splice(i, 1)
    else added.push(item)
  }
  return added
}
const MOUNTED_CLASSES = def.mountedClasses !== false
async function mountedDiff(page, openExpr) {
  const before = await page.evaluate(mountedBagSrc(MOUNTED_CLASSES))
  await eval(`(async (page) => { ${openExpr} })(page)`)
  await page.waitForTimeout(400)
  const after = await page.evaluate(mountedBagSrc(MOUNTED_CLASSES))
  return bagDiff(before, after)
}
async function oracleRun(step) {
  const page = await browser.newPage()
  await page.goto(`file://${process.cwd()}/${OUT}/oracle.html`)
  await page.waitForTimeout(500)
  let mounted = null
  if (def.open) {
    if (!step) mounted = await mountedDiff(page, def.open)
    else { await eval(`(async (page) => { ${def.open} })(page)`); await page.waitForTimeout(400) }
  }
  const result = step ? await stepIt(page, step) : null
  const fact = await page.evaluate(`window.__facts("oracle")`)
  await page.close()
  return { result, fact, mounted }
}
async function shadlessRun(step) {
  const page = await browser.newPage()
  page.on("pageerror", (e) => console.log("  [shadless pageerror]", e.message))
  await page.goto(`file://${process.cwd()}/${OUT}/shadless.html`)
  await page.addScriptTag({ content: recorderSrc(def.slots) })
  await page.waitForTimeout(400)
  let mounted = null
  if (def.openShadless) {
    if (!step) mounted = await mountedDiff(page, def.openShadless)
    else await eval(`(async (page) => { ${def.openShadless} })(page)`) // def may use await
  }
  await page.waitForTimeout(300)
  const result = step ? await stepIt(page, step) : null
  const fact = await page.evaluate(`window.__facts("shadless")`)
  await page.close()
  return { result, fact, mounted }
}

const oracleS = {}, shadlessS = {}
let oracleOpen, shadlessOpen, oracleMounted, shadlessMounted
const flaky = []
for (const step of [null, ...def.scenarios]) {
  let o = await oracleRun(step)
  let c = await shadlessRun(step)
  if (!step) { oracleOpen = o.fact; shadlessOpen = c.fact; oracleMounted = o.mounted; shadlessMounted = c.mounted; continue }
  if (o.result !== c.result) {
    // A real behavioral difference reproduces; a timing race in either
    // browser page does not. Re-run BOTH sides once from a fresh page. If
    // they now agree, record the agreed value and say so — the radio-group
    // focus+ArrowRight scenario hit exactly this in the React oracle (radix
    // checks-on-focus only while its document keydown ref is set; the
    // oracle side reported ind=absent once, ind=checked every other run).
    const o2 = await oracleRun(step), c2 = await shadlessRun(step)
    if (o2.result === c2.result) { flaky.push(`${step}: first run oracle=${o.result} shadless=${c.result}`); o = o2; c = c2 }
    else { o = o2; c = c2 }
  }
  oracleS[step] = o.result; shadlessS[step] = c.result
}
if (flaky.length) console.log(`contracts[${name}]: ${flaky.length} scenario(s) agreed only on re-run (timing flake, not a diff)\n    ${flaky.join("\n    ")}`)
await browser.close()

// ---------- diff --------------------------------------------------------------
let pass = true
console.log(`contracts[${name}]: open-state facts`)
const a = normFact(oracleOpen), b = normFact(shadlessOpen)
for (const k of def.slots) {
  const diffs = []
  const keys = [...new Set([...Object.keys(a?.[k] || {}), ...Object.keys(b?.[k] || {})])]
  const ign = new Set([
    // "text" was permanently ignored here even though the recorder captures
    // it — content drift never failed a contract. It is now compared
    // (whitespace-normalized upstream); per-component exclusions belong in
    // def.ignoreAttrs.
    "tag",
    ...(def.ignoreAttrs?.[k] ?? []),
  ])
  for (const kk of keys) {
    if (kk.startsWith("data-radixuigo-") || kk.startsWith("data-radix-popper-")) continue // kernel glue protocol internals (positioning feedback on the anchor)
    if (kk === "data-radix-menu-content") continue // radix internals marker
    if (ign.has(kk)) continue
    if ((a?.[k]?.[kk] ?? "<absent>") !== (b?.[k]?.[kk] ?? "<absent>"))
      diffs.push(`    ${kk}: oracle=${JSON.stringify(a?.[k]?.[kk])} shadless=${JSON.stringify(b?.[k]?.[kk])}`)
  }
  console.log(`  ${k}: ${diffs.length ? "DIFF" : "match"}`)
  diffs.forEach((d) => console.log(d))
  if (diffs.length) pass = false
}
for (const k of ["activeElement", "scrollLock"]) {
  const same = JSON.stringify(a?.[k]) === JSON.stringify(b?.[k])
  console.log(`  ${k}: ${same ? "match" : `DIFF oracle=${JSON.stringify(a?.[k])} shadless=${JSON.stringify(b?.[k])}`}`)
  if (!same) pass = false
}
console.log(`contracts[${name}]: scenarios`)
for (const s of def.scenarios) {
  const same = oracleS[s] === shadlessS[s]
  console.log(`  ${s}: oracle=${oracleS[s]} shadless=${shadlessS[s]} ${same ? "" : "DIFF"}`)
  if (!same) pass = false
}

// mounted-diff structural check: the JS-created DOM must match too —
// class drift inside portaled/mounted content had NO guard before (facts
// drop class; golden only sees the closed initial state)
if ((oracleMounted || shadlessMounted) && def.mountedCheck !== false) {
  console.log(`contracts[${name}]: mounted DOM`)
  const om = [...(oracleMounted ?? [])]
  const onlyShadless = []
  for (const x of [...(shadlessMounted ?? [])]) {
    const i = om.indexOf(x)
    if (i >= 0) om.splice(i, 1)
    else onlyShadless.push(x)
  }
  // om now holds oracle-only entries
  const same = om.length === 0 && onlyShadless.length === 0
  if (same) console.log(`  ${(oracleMounted ?? []).length} mounted elements match`)
  else {
    pass = false
    for (const x of om.slice(0, 4)) console.log(`  only-oracle:   ${x.slice(0, 160)}`)
    for (const x of onlyShadless.slice(0, 4)) console.log(`  only-shadless: ${x.slice(0, 160)}`)
  }
}
writeFileSync(`${OUT}/result.json`,
  JSON.stringify({ oracleOpen, shadlessOpen, oracle: oracleS, shadless: shadlessS, pass }, null, 2))
console.log(`\n${pass ? "PASS" : "FAIL"}  contracts ${name}`)
process.exit(pass ? 0 : 1)
