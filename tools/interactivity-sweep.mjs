#!/usr/bin/env node
// interactivity-sweep — every demo page that OFFERS an interaction must
// RESPOND to one. Born from the dead-button bug (2026-08-27): every kernel
// example beyond -demo shipped as a static oracle snapshot; no gate noticed
// because contracts click FIXTURES, golden compares SNAPSHOTS, and smoke
// only listens to the console — "does the page respond" was nobody's job.
//
// Method: pages under docs/site/components (the tree iframes serve) are
// pre-filtered statically for interactive candidates (triggers, switches,
// tabs, carousel arrows). Each candidate page is loaded, the first
// candidate is clicked (hovered for hover-driven families), and a state
// fingerprint — every data-state / aria-expanded / aria-checked value plus
// body child count — must CHANGE. Static-by-design pages (no candidates)
// pass trivially.
//
// KNOWN_DEAD families ship static today (oracle snapshots — the family
// migration into tools/example-fixture.mjs is the recorded follow-up).
// They are counted and reported here, and any page OUTSIDE those families
// that offers interaction but does not respond FAILS — new dead pages
// cannot land silently anymore.
import { chromium } from "playwright"
import { readFileSync, readdirSync } from "node:fs"
import { resolve } from "node:path"

const SITE = "docs/site/components"
const STATIC_FAMILIES = Object.entries(JSON.parse(readFileSync("probes/out/tiers.json", "utf8"))).filter(([, t]) => t.tier === "static").map(([c]) => c)
const CANDIDATE = /data-slot="[^"]*-trigger"|aria-expanded=|role="(switch|checkbox|tab)"|data-slot="(carousel-next|carousel-prev)"/
const HOVER_FAMILIES = new Set(["tooltip", "hover-card"])

// static oracle snapshots pending family migration (EXEMPTIONS manual
// follow-up) — keyed by example name prefix
// trivial-js families (accordion, checkbox, collapsible, radio-group, switch,
// toggle, toggle-group, …) left this list when tools/example-oracle.mjs
// started loading the runtime on their pages
// popover / tooltip / hover-card / tabs / slider / scroll-area left when
// tools/example-fixture.mjs learned their protocols (multi-instance glue)
const KNOWN_DEAD = new Set([
  "message-scroller",  // hand-authored pages (oracle cannot bundle the example)
])
const familyOf = (name) => {
  const m = /^(alert-dialog|navigation-menu|context-menu|dropdown-menu|hover-card|button-group|message-scroller|input-group|native-select|radio-group|scroll-area|toggle-group|carousel|accordion|attachment|avatar|breadcrumb|bubble|collapsible|checkbox|combobox|dialog|drawer|field|input|item|kbd|label|marker|menubar|message|pagination|popover|progress|select|sheet|slider|switch|table|tabs|toggle|tooltip)-/.exec(name)
  return m ? m[1] : name.replace(/-rtl(-|$).*/, "")
}

const FINGERPRINT = `JSON.stringify({
  // per ELEMENT (index), not a sorted multiset: switching tabs keeps the set
  // {active, inactive, inactive} identical and read as "nothing responded"
  states: [...document.querySelectorAll("[data-state]")].map((e, i) => i + ":" + e.getAttribute("data-slot") + ":" + e.getAttribute("data-state")),
  expanded: [...document.querySelectorAll("[aria-expanded]")].map(e => (e.id || e.getAttribute("data-slot")) + ":" + e.getAttribute("aria-expanded")).sort(),
  checked: [...document.querySelectorAll("[aria-checked]")].map(e => (e.id || e.getAttribute("data-slot")) + ":" + e.getAttribute("aria-checked")).sort(),
  // a carousel responds by (un)disabling its arrows
  disabled: [...document.querySelectorAll("button, input")].map((e, i) => i + ":" + e.disabled),
  kids: document.body.children.length,
})`

const pages = readdirSync(SITE).filter((x) => x.endsWith(".html") && !/-rtl-(en|he|fa)\.html$/.test(x))
const candidates = pages.filter((f) => CANDIDATE.test(readFileSync(`${SITE}/${f}`, "utf8")))

const browser = await chromium.launch()
const page = await browser.newPage()
let verified = 0, staticPages = pages.length - candidates.length
const failures = []
const deadCount = {}
for (const f of candidates) {
  const name = f.replace(".html", "")
  const fam = familyOf(name)
  if (KNOWN_DEAD.has(fam)) {
    deadCount[fam] = (deadCount[fam] ?? 0) + 1
    continue
  }
  try {
    await page.goto(`file://${resolve(SITE, f)}`)
    await page.waitForTimeout(350)
    // every candidate in DOM order; skip disabled ones (checkbox-disabled,
    // switch-disabled: the FIRST control is disabled by design) and try up
    // to three — a nested example's first trigger may be an inert wrapper
    const els = await page.$$('[data-slot$="-trigger"], [aria-expanded], [role="switch"], [role="checkbox"], [role="tab"], [data-slot="carousel-next"], [data-slot="carousel-prev"]')
    const usable = []
    for (const el of els) {
      // an already-active tab offers nothing to do (tabs-disabled: one active
      // tab + one disabled tab is a static page)
      // a *-trigger slot of a STATIC family (attachment-trigger is a link
      // overlay) offers no behavior — only families with a runtime count
      const ok = await el.evaluate(({ statics }) => {
        const e = this ?? null; return null
      }, { statics: [] }).catch(() => null) ?? await el.evaluate((e, statics) => !e.disabled && e.getAttribute("aria-disabled") !== "true" && !e.closest("[hidden]") && e.getClientRects().length > 0 &&
        !(e.getAttribute("role") === "tab" && e.getAttribute("data-state") === "active") &&
        !statics.some((s) => (e.getAttribute("data-slot") || "") === s + "-trigger"), STATIC_FAMILIES)
      if (ok) usable.push(el)
      if (usable.length === 3) break
    }
    // every control disabled by design (checkbox-disabled, switch-disabled):
    // the page offers no interaction — static, not dead
    if (!usable.length) { staticPages++; continue }
    // the element's OWN family decides deadness: an avatar example whose
    // interaction is a dropdown-menu trigger is dead for dropdown-menu's
    // reason, not avatar's
    const ownFam = await usable[0].evaluate((e) => (e.getAttribute("data-slot") || "").replace(/-(trigger|item|next|prev)$/, ""))
    if (ownFam && KNOWN_DEAD.has(ownFam) && ownFam !== fam) { deadCount[ownFam] = (deadCount[ownFam] ?? 0) + 1; continue }
    let responded = false
    for (const el of usable) {
      const before = await page.evaluate(FINGERPRINT)
      const hoverEl = await el.evaluate((e) => /^(tooltip|hover-card)-trigger$/.test(e.getAttribute("data-slot") || ""))
      if (HOVER_FAMILIES.has(fam) || hoverEl) {
        const box = await el.boundingBox()
        if (box) await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2, { steps: 5 })
      } else if (fam === "context-menu" || await el.evaluate((e) => e.getAttribute("data-slot") === "context-menu-trigger")) {
        await el.click({ timeout: 3000, button: "right" })
      } else {
        await el.click({ timeout: 3000 })
      }
      // hover families open after radix's delay (tooltip 700ms provider
      // default, hover-card 700ms) — wait past it
      await page.waitForTimeout(HOVER_FAMILIES.has(fam) || hoverEl ? 1100 : 600)
      if (before !== await page.evaluate(FINGERPRINT)) { responded = true; break }
    }
    if (!responded) failures.push(`${name}: interaction offered but nothing responded (${fam})`)
    else verified++
  } catch (e) {
    failures.push(`${name}: ${e.message.split("\n")[0]}`)
  }
}
await browser.close()
const deadTotal = Object.values(deadCount).reduce((a, b) => a + b, 0)
if (failures.length) {
  console.error(`FAIL  interactivity-sweep\n  ${failures.slice(0, 12).join("\n  ")}${failures.length > 12 ? `\n  … +${failures.length - 12} more` : ""}`)
  process.exit(1)
}
console.log(`PASS  interactivity-sweep (${verified} pages responded, ${staticPages} static-by-design, ${deadTotal} known-dead across ${Object.keys(deadCount).length} families pending migration — see EXEMPTIONS)`)
