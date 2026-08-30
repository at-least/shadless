#!/usr/bin/env node
// FT8/Step 9: multi-language RTL demo emission.
//
// Reads the dictionaries from src/registry/rtl-translations.json (en/ar/he —
// Persian added by hand for alert) and emits one HTML file per language for
// each RTL preview. Source of truth is still upstream — tools/rtl-dict.mjs
// lifts those dictionaries out of `examples/aria/<name>-rtl.tsx` and is now
// the ONLY thing in the pipeline that reads that tree. No LLM involved
// (Persian is a small hand-coded dictionary for the alert reference page).
//
// Approach: text substitution on the existing Arabic HTML
// (`docs/demos/<name>-rtl.html`). The structure (slot tree, classes,
// icons, dir wrapper) is identical across languages — only the user-
// facing strings differ. We swap each `translations.ar.values[key]`
// for the target language's value, then patch the dir + lang
// attributes. Robust because upstream's TSX stores the same Arabic
// strings the existing HTML uses verbatim (we ported the Arabic HTML
// from the upstream TSX originally).
//
// Every emitted file gets the theme pre-paint script (injectPrePaint,
// same convention as demo.mjs) — dist demos must flip dark mode
// independently, and the site copy mirrors dist content verbatim (Wave I
// audit bug #1: the fa page shipped without pre-paint because this tool
// skipped the injection).
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs"
import { substituteAndPatch } from "./rtl-lib.mjs"
import { injectPrePaint } from "../src/docs/theme-prepaint.mjs"

const DICT = JSON.parse(readFileSync("src/registry/rtl-translations.json", "utf8"))
// Whether a missing base page is a bug or an expected absence is a property of
// the component, and tiers.json already records it — no second list. Upstream
// ships -rtl examples for components we tombstoned (calendar, sidebar) and for
// things that are not our components at all (data-table, typography); those
// have no page to translate and never will.
const TIERS = JSON.parse(readFileSync("src/registry/tiers.json", "utf8"))
const WHOLESALE = new Set(["static", "kernel", "trivial-js"])
const shipped = (name) => {
  const t = TIERS[name.replace(/-rtl$/, "")]
  return !!t && (WHOLESALE.has(t.tier) || t.emit === true)
}
const DOCS_DEMOS = "docs/demos"
const DIST_COMPONENTS = "dist/components"
mkdirSync(DOCS_DEMOS, { recursive: true })
mkdirSync(DIST_COMPONENTS, { recursive: true })

// Persian translations for the alert reference page only (4 keys, 8
// strings — title + description × 2 alerts). Hand-coded by reviewing
// the upstream Arabic strings and using established RTL Persian UI
// vocabulary. Other components ship without Persian until upstream
// adds it (or we hand-code per-component).
const PERSIAN = {
  // Persian is RTL — without this the fa page used to flip to dir="ltr"
  dir: "rtl",
  paymentTitle: "پرداخت موفق",
  // formal register (حضرت / پست الکترونیکی / گردید) — was a find/replace
  // overlay in patches/build-rtl/alert-rtl-fa.json; now the dictionary itself
  paymentDescription: "پرداخت حضرت به مبلغ ۲۹.۹۹ دلار با موفقیت انجام شد. رسید نیز به نشانی پست الکترونیکی شما ارسال گردید.",
  featureTitle: "ویژگی جدید موجود است",
  featureDescription: "ما پشتیبانی از حالت تیره را به سیستم افزوده‌ایم. می‌توانید این قابلیت را از بخش تنظیمات حساب کاربری خود فعال نمایید.",
}

// pure helpers live in tools/rtl-lib.mjs (unit-tested)

let emitted = 0
// FT8/Step 9: track which language variants exist per RTL preview so
// the host page can emit language buttons that won't 404 on click.
// Written to build/rtl-langs.json — read by src/docs/components.mjs
// (via buildComponentMap()).
const langManifest = {}
// Two-phase, for the reason ddc03fb gave when example-oracle had the same
// shape of bug: this loop used to `continue` past every problem and still exit
// 0, writing a SMALLER manifest. That is what hid demo-rtl's missing
// `needs: example-fixture` — four base pages were absent, four components
// dropped out of rtl-langs.json, and the node stayed green.
const pending = []
const failures = []
const skipped = []
for (const [name, translations] of Object.entries(DICT)) {
  const existingHtmlPath = `${DOCS_DEMOS}/${name}.html`
  if (!existsSync(existingHtmlPath)) {
    if (shipped(name)) failures.push(`${name}: no ${existingHtmlPath} to translate from, but the component ships`)
    else skipped.push(name)
    continue
  }

  const arabicHtml = readFileSync(existingHtmlPath, "utf8")
  const langs = ["ar"]

  for (const lang of ["he", "en"]) {
    if (!translations[lang]) continue
    const html = injectPrePaint(substituteAndPatch(arabicHtml, translations, "ar", lang, translations[lang].values))
    pending.push([`${DOCS_DEMOS}/${name}-${lang}.html`, html], [`${DIST_COMPONENTS}/${name}-${lang}.html`, html])
    emitted++
    langs.push(lang)
  }

  // Persian: only for alert-rtl (the user-comparison reference page).
  if (name === "alert-rtl" && PERSIAN) {
    const { dir, ...values } = PERSIAN
    const html = injectPrePaint(substituteAndPatch(arabicHtml, translations, "ar", "fa", values, dir))
    pending.push([`${DOCS_DEMOS}/${name}-fa.html`, html], [`${DIST_COMPONENTS}/${name}-fa.html`, html])
    emitted++
    langs.push("fa")
  }
  langManifest[name] = langs
}

if (failures.length) {
  for (const f of failures) console.error(`FAIL [${f.split(":")[0]}]: ${f.slice(f.indexOf(":") + 2)}`)
  console.error(`FAIL  build-rtl (${failures.length} previews could not be built) — nothing written`)
  process.exit(1)
}

// every page rendered; commit them and the manifest together
for (const [path, html] of pending) writeFileSync(path, html)
// Write the manifest to docs/site/ so the host page can read it via
// fetch (avoids a server-side build dependency at mdx-compile time).
mkdirSync("build", { recursive: true })
writeFileSync("build/rtl-langs.json", JSON.stringify(langManifest, null, 2))
console.log(`build-rtl: ${emitted} language variants emitted (excluding ar default) + manifest for ${Object.keys(langManifest).length} previews` +
  (skipped.length ? ` (${skipped.length} dictionaries have no shipped page: ${skipped.join(", ")})` : ""))
