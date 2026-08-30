// oracle-lib — single source for the React oracle environment shared by
// example-oracle.mjs (hop 2: oracle → shipped page) and example-golden.mjs
// (hop 1: oracle → live upstream snapshot). The render env and the DOM
// comparator live HERE and nowhere else: hand-copied renderers are how the
// hop-1 debugging drifted before (five near-identical canonOf copies, each
// missing different normalizations).
//
// Render environment = upstream's app/layout.tsx provider stack. Only the
// radix TooltipProvider carries DOM/contract for radix examples (ThemeProvider
// sets attrs on <html>, outside #root; NuqsAdapter renders nothing; the
// base-flavor tooltip provider serves base-flavor tooltips only). Examples
// are authored against that context — tooltip-demo.tsx uses <Tooltip> bare,
// which crashes without the provider. The crash used to be invisible:
// __done was set on the synchronous path and the EMPTY #root was emitted
// as a demo page. The entry template below closes both holes: flushSync
// propagates render exceptions into __err, and an empty #root is itself
// an error.
import { build } from "esbuild"
import { writeFileSync, mkdirSync, readFileSync, existsSync, readdirSync } from "node:fs"
import { resolve, join, relative } from "node:path"
import { createHash } from "node:crypto"

// Bundle cache: esbuild output for a demo is a pure function of (upstream
// pin, example source, stubs, entry template). The RENDER still runs fresh
// every call — the gate's whole point is re-rendering the oracle — but the
// ~1-2s per-demo esbuild pass is skippable when nothing it reads has moved.
// Keyed on: pin commit (upstream tree identity), the example file, the
// stubs dir, resolve-skins + the skin tables it imports (they regenerate the
// resolved tree the aliases point at) and this module itself (entry template
// + alias matrix live here), plus package-lock.json
// (react/radix/esbuild versions) now that the cache is shared and restored
// across CI runs.
//
// Where the bundles live: a CACHE path, not the tracked tree. The bundles
// are ~7 MB of React each and a pure function of their inputs; kept under
// build/ (then probes/out/) they were committed through reproducible's GENERATED list
// (500 MB of a 670 MB checkout). node_modules/.cache is gitignored,
// survives between runs, and is shared by every oracle consumer
// (example-oracle, example-fixture, example-golden, contracts) — one build
// per demo instead of one per tool. Override with SHADLESS_CACHE.
export const CACHE_DIR = process.env.SHADLESS_CACHE ?? "node_modules/.cache/shadless/oracle"

function bundleCacheKey(name) {
  const h = createHash("sha256")
  const pin = JSON.parse(readFileSync("src/registry/pin.json", "utf8"))
  h.update(pin.shadcn_ui.commit + "\n")
  h.update(readFileSync("package-lock.json"))
  h.update(readFileSync(join(".upstream/shadcn-ui/apps/v4/examples/radix", `${name}.tsx`)))
  for (const f of readdirSync("tools/contracts/stubs").sort())
    h.update(f).update(readFileSync(join("tools/contracts/stubs", f)))
  // resolve-skins is what BUILDS build/resolved-ui, which every alias below
  // points at — so both the script and the tables it imports are part of the
  // bundle's identity. skin.mjs was missing: editing SKIN_ALLOWLIST changed
  // the resolved tree, the runner correctly re-ran the oracle consumers, and this
  // key did not move — so a bundle compiled against the OLD resolved tree was
  // reused and the gates compared against a stale oracle, green.
  h.update(readFileSync("tools/resolve-skins.mjs"))
  h.update(readFileSync("src/emitter/skin.mjs"))
  h.update(readFileSync(new URL("./oracle-lib.mjs", import.meta.url)))
  return h.digest("hex")
}

// esbuild alias matrix: examples import GENERATED style dirs
// @/styles/<flavor>-<skin>/ui[-rtl]/* (gitignored). The resolved tree
// (tools/resolve-skins.mjs) is their tracked equivalent — cn-* already
// expanded to nova utilities, upstream generation parity. Resolved files
// keep their @/registry/bases/radix/{ui,lib,hooks} specifiers, so those
// alias into the resolved tree too.
function aliases() {
  const SKINS = ["nova", "vega", "lyra", "maia", "mira", "luma", "sera", "rhea"]
  const RESOLVED = resolve("build/resolved-ui")
  const a = {
    "@": resolve(".upstream/shadcn-ui/apps/v4"),
    "@/registry/bases/radix/ui": `${RESOLVED}/ui`,
    "@/registry/bases/radix/lib": `${RESOLVED}/lib`,
    "@/registry/bases/radix/hooks": `${RESOLVED}/hooks`,
    "@/components/language-selector": resolve("tools/contracts/stubs/app-components.jsx"),
    "@/components/markdown": resolve("tools/contracts/stubs/app-components.jsx"),
    "@/components/message-animated": resolve("tools/contracts/stubs/message-animated.jsx"),
    "@/app/(create)/components/icon-placeholder": resolve("tools/contracts/stubs/icon-placeholder.jsx"),
    "next/image": resolve("tools/contracts/stubs/next-image.jsx"),
    "next/link": resolve("tools/contracts/stubs/next-link.jsx"),
    "date-fns": resolve("tools/contracts/stubs/date-fns.mjs"),
    sonner: resolve("tools/contracts/stubs/sonner.jsx"),
    "embla-carousel-autoplay": resolve("tools/contracts/stubs/embla-autoplay.mjs"),
    "react-textarea-autosize": resolve("tools/contracts/stubs/textarea-autosize.jsx"),
  }
  for (const f of ["radix", "base", "aria"])
    for (const s of SKINS) {
      a[`@/styles/${f}-${s}/ui`] = `${RESOLVED}/ui`
      a[`@/styles/${f}-${s}/ui-rtl`] = `${RESOLVED}/ui-rtl`
    }
  return a
}

// Bundle the pinned example and return a file:// page that renders it in
// the upstream provider context. Resolves to { htmlFile } — load it, wait
// for `window.__done === true` (a crash leaves `window.__err` instead).
// *-rtl demos render inside DirectionProvider dir="rtl" (upstream's RTL
// previews wrap the demo in it; radix reads direction from React context,
// never from the DOM).
export async function buildOracle(name, { tmp } = { tmp: "build/example-oracle" }) {
  mkdirSync(tmp, { recursive: true })
  mkdirSync(CACHE_DIR, { recursive: true })
  const dir = /-rtl$/.test(name) ? "rtl" : "ltr"
  const entry = join(CACHE_DIR, `.entry-${name}.mjs`)
  writeFileSync(entry, `
import * as React from "react"
import { createRoot } from "react-dom/client"
import { flushSync } from "react-dom"
import { Direction } from "radix-ui"
import * as Mod from "@/examples/radix/${name}"
import { TooltipProvider } from "@/registry/bases/radix/ui/tooltip"
const Demo = Mod.default ?? Object.values(Mod).find((v) => typeof v === "function")
try {
  const root = createRoot(document.getElementById("root"))
  flushSync(() => root.render(
    React.createElement(TooltipProvider, { delayDuration: 0 },
      React.createElement(Direction.Provider, { dir: "${dir}" }, React.createElement(Demo)))
  ))
  if (!document.querySelector("#root").hasChildNodes()) throw new Error("empty #root after render")
  window.__done = true
} catch (e) { window.__err = String(e?.message ?? e) }
`)
  const outfile = join(CACHE_DIR, `bundle-${name}.js`)
  const keyFile = join(CACHE_DIR, `.key-${name}`)
  const key = bundleCacheKey(name)
  if (!(existsSync(outfile) && existsSync(keyFile) && readFileSync(keyFile, "utf8") === key)) {
    await build({
      entryPoints: [entry], bundle: true, format: "iife",
      outfile, logLevel: "error", alias: aliases(),
      loader: { ".tsx": "tsx" },
      // classic JSX (default) emits free `React.createElement` for sources
      // without an explicit React import (aspect-ratio, …) — automatic runtime
      // resolves via jsx-runtime instead
      jsx: "automatic",
    })
    writeFileSync(keyFile, key)
  }
  const htmlFile = join(tmp, `oracle-${name}.html`)
  writeFileSync(htmlFile, `<!doctype html><html${dir === "rtl" ? ' dir="rtl"' : ""}><head><meta charset="utf-8"></head>
<body><div id="root"></div><script src="${relative(tmp, outfile)}"></script></body></html>`)
  return { htmlFile }
}

// Wait for the oracle page to settle; throws with the render error if the
// example crashed (never returns on an empty render — that is the point).
export async function awaitOracle(page, htmlFile) {
  await page.goto(`file://${resolve(htmlFile)}`)
  await page.waitForFunction(
    "window.__done === true || window.__err !== undefined", null, { timeout: 5000 }
  )
  const err = await page.evaluate("window.__err")
  if (err) throw new Error(err)
}

// radix auto-ids (radix-:rN: / radix-_r_N runtime, radix-_R_N…_ SSR
// react-useId) — runtime-generated, not part of the contract.
export const norm = (html) => html
  .replace(/radix-:r[a-z0-9]*:?/g, "radix-<auto>")
  .replace(/radix-_r_[a-z0-9-]*/g, "radix-<auto>")
  .replace(/radix-_R_[a-z0-9-]*/gi, "radix-<auto>")

// DOM-STRUCTURAL canonical form: parse in the real browser, canonicalize
// to (tag, sorted attr map, text) trees — SSR/CSR attribute-order and
// entity-escaping differences cannot false-positive. Every rule below is
// a serialization-form or runtime-frame normalization, never a semantic
// bridge; value-level drift must go through exemptions instead.
export async function canonOf(page, html) {
  return page.evaluate(([h]) => {
    const doc = new DOMParser().parseFromString(`<body>${h}</body>`, "text/html")
    const canon = (node, parentRole = null, inCarousel = false) => {
      if (node.nodeType === 3) {
        // whitespace-only text nodes: JSX/serializer spacing artifacts —
        // they differ between SSR strings and innerHTML round-trips while
        // carrying no content
        const t = node.nodeValue.replace(/\s+/g, " ").trim()
        return t ? { t } : null
      }
      if (node.nodeType !== 1) return null
      const attrs = {}
      const ownRole = [...(node.attributes || [])].find((x) => x.name === "role")?.value ?? null
      // roving-focus containers (radiogroup/toolbar/tablist/menubar): radix
      // rotates tabindex across the subtree at runtime; the SSR initial
      // state and CSR settled state differ by design — tabindex inside such
      // subtrees is focus state, not structure
      const roving = ["radiogroup", "toolbar", "tablist", "menubar", "group"].includes(parentRole ?? ownRole ?? "")
      const hasPopup = [...(node.attributes || [])].some((x) => x.name === "aria-haspopup")
      const hasState = [...(node.attributes || [])].some((x) => x.name === "data-state")
      const carousel = inCarousel || [...(node.attributes || [])].some((x) => x.name === "data-slot" && x.value === "carousel-content")
      for (const a of node.attributes || []) {
        if (roving && a.name === "tabindex") continue
        // asChild Slot-merged triggers (deploy lag): our pin renders the
        // wrapper slot ("dialog-trigger"), live renders the child's
        // ("button") — same interactive element either way. The button
        // signature is gated on data-state (radix triggers carry it,
        // plain buttons don't); aria-haspopup is NOT a reliable gate —
        // newer radix dropped it from tooltip triggers.
        if (a.name === "data-slot" && (/-trigger$/.test(a.value) || (a.value === "button" && (hasPopup || hasState)))) { attrs["data-slot"] = "trigger"; continue }
        // aria-controls targets radix content ids — CSR drops it when the
        // controlled content is unmounted (closed), SSR renders it; the value
        // is a runtime id either way, so the key carries no comparable info
        if (a.name === "aria-controls") continue
        // srcset: Next-server responsive variants (/_next/image?...&w=640
        // 640w, …) — CDN artifact like the src indirection below
        if (a.name === "srcset") continue
        let v = a.value
        // next/image CDN indirection: SSR rewrites src through
        // /_next/image?url=<enc>&w=&q= — a Next-server concern, not DOM
        // semantics; recover the raw url both sides share
        if (a.name === "src" && v.startsWith("/_next/image?")) {
          try { v = new URLSearchParams(v.split("?")[1]).get("url") ?? v } catch {}
        }
        if (a.name === "class") v = [...new Set(v.split(/\s+/).filter(Boolean))].sort().join(" ")
        if (a.name === "style") {
          let decls = v.split(";").map((s) => s.trim()).filter(Boolean)
            .flatMap(normDecl)
          // radix Presence injects measured/animated declarations AFTER
          // mount (transition-duration, animation-name, resolved
          // --radix-collapsible-content-* measurements) that the SSR
          // snapshot cannot carry — runtime state noise, dropped on both
          // sides. SSR serializes without spaces (--a:var(--b)); CSR with
          // them.
          const RUNTIME_STYLE = new Set([
            "transition-duration", "animation-name", "animation-duration",
            "--radix-collapsible-content-height", "--radix-collapsible-content-width",
          ])
          // embla writes translate3d on the container once it measures —
          // the SSR frame cannot carry it; position-from-measurement is
          // runtime state, not structure
          if (carousel) RUNTIME_STYLE.add("transform")
          decls = decls
            .filter((s) => !RUNTIME_STYLE.has(s.split(":")[0].trim()))
            .map((s) => { const i = s.indexOf(":"); return s.slice(0, i).trim() + ":" + s.slice(i + 1).trim() })
            // React client collapses top/right/bottom/left:0 into `inset`; the
            // SSR serializer does not — expand inset so both sides canonicalize
            .flatMap((d) => d.startsWith("inset:") ? ["bottom:" + d.slice(6), "left:" + d.slice(6), "right:" + d.slice(6), "top:" + d.slice(6)] : [d])
            // unit normalization: CSR serializes 0 as "0px", SSR as "0"
            .map((d) => d.replace(/:0(px|rem|em|%|ch|vh|vw)$/, ":0"))
            // percentage precision: Chrome CSSOM serializes 3 decimals
            // (177.778%), React SSR writes the full float
            // (177.77777777777377%) — round both to 3
            .map((d) => d.replace(/:(\d+(\.\d+)?)%/, (m, num) => ":" + parseFloat(Number(num).toFixed(3)) + "%"))
          v = decls.sort().join(";")
          // CSR may leave a literally empty style="" behind (radix Presence
          // on a closed collapsible); SSR omits the attribute entirely
          if (v === "") continue
        }
        attrs[a.name] = v
      }
      // radix form-participation hidden controls (Switch/Checkbox/Radio
      // bubble <input>, Select's native <select>, Field's sr-only input):
      // rendered while the control ref is unsettled — the SSR frame sees
      // isFormControl=true — and removed after hydration when the demo has
      // no form ancestor. Identified by aria-hidden plus one of the two
      // hidden-control style fingerprints. Presence-only runtime state,
      // dropped wherever it appears.
      if ((node.tagName === "INPUT" || node.tagName === "SELECT") && attrs["aria-hidden"] === "true") {
        const s = attrs.style ?? ""
        if ((s.includes("opacity:0") && s.includes("pointer-events:none")) || s.includes("clip:rect(")) return null
      }
      // attrs as a SORTED map — SSR and CSR order attributes differently
      const sorted = {}
      for (const k of Object.keys(attrs).sort()) sorted[k] = attrs[k]
      const myRole = attrs.role ?? parentRole
      // adjacent text nodes: React SSR separates them with <!-- -->
      // comments (dropped by canon), while an innerHTML round-trip merges
      // them — join with a space on both sides so the boundary carries no
      // signal
      const kids = [...node.childNodes].map((c) => canon(c, myRole, carousel)).filter(Boolean)
      for (let i = kids.length - 1; i > 0; i--) {
        if (kids[i].t !== undefined && kids[i - 1].t !== undefined) {
          kids[i - 1] = { t: (kids[i - 1].t + " " + kids[i].t).replace(/\s+/g, " ").trim() }
          kids.splice(i, 1)
        }
      }
      return { tag: node.tagName, attrs: sorted, kids }
    }
    // Chrome CSSOM re-serializes the overflow longhand pair as the
    // shorthand ("overflow:hidden scroll"); React SSR writes the longhands
    // verbatim — same declarations, different spelling. Expand before the
    // RUNTIME_STYLE filter so both spellings meet as longhands.
    function normDecl(s) {
      const i = s.indexOf(":")
      const prop = s.slice(0, i).trim(), val = s.slice(i + 1).trim()
      if (prop !== "overflow") return [s]
      const parts = val.split(/\s+/)
      return parts.length === 1
        ? [`overflow-x:${parts[0]}`, `overflow-y:${parts[0]}`]
        : [`overflow-x:${parts[0]}`, `overflow-y:${parts[1]}`]
    }
    return canon(doc.body)
  }, [html])
}
