// docs page-planning helpers (tools/docs-page-lib.mjs) + demo script
// extraction (src/docs/demo-scripts.mjs) + demo path rewrites
// (tools/demo-lib.mjs) — Wave H: each assertion pins a shipped bug
// (missing Next links, accordion h3 in TOC, pre-paint leaking into JS
// tabs, the 2026-08-25 rewritePaths incident).
import { addHeadingIds, prevNextFor } from "../docs-page-lib.mjs"
import { extractDemoScripts } from "../../src/docs/demo-scripts.mjs"
import { rewritePaths, ensureLink } from "../demo-lib.mjs"

export function run(t) {
  // ---- addHeadingIds ----
  {
    const { body, toc } = addHeadingIds("<h2>Getting Started</h2><h3>Advanced Tips</h3>")
    t.ok("headings: id added", body.includes('<h2 id="getting-started">'))
    t.eq("headings: toc entries", toc, [
      { depth: 2, text: "Getting Started", id: "getting-started" },
      { depth: 3, text: "Advanced Tips", id: "advanced-tips" },
    ])
  }
  {
    // duplicate slugs get -N suffixes
    const { toc } = addHeadingIds("<h2>Notes</h2><h2>Notes</h2>")
    t.eq("headings: slug dedupe", toc.map((x) => x.id), ["notes", "notes-1"])
  }
  {
    // D2 regression: component-embedded h3 (AccordionTrigger) must be
    // skipped — no id, no TOC entry, no clipboard-anchor pollution
    const src = '<h3 class="flex"><button data-slot="accordion-trigger" data-state="closed">Is it accessible?</button></h3><h2>Real Heading</h2>'
    const { body, toc } = addHeadingIds(src)
    t.ok("headings: component h3 skipped (no id)",
      body.includes('<h3 class="flex"><button data-slot="accordion-trigger"') && !body.includes('<h3 id='))
    t.eq("headings: toc excludes component h3", toc.map((x) => x.id), ["real-heading"])
  }

  // ---- prevNextFor (D4 regression: next must resolve from the planned page set) ----
  {
    const ctx = {
      sidebarOrder: ["accordion", "alert", "badge", "button"],
      mirrorSet: ["accordion", "alert", "badge", "button"],
      plannedPages: new Set(["accordion", "alert", "badge"]), // button page NOT planned (simulated missing mdx)
      guides: [{ slug: "intro" }, { slug: "rtl" }],
    }
    t.eq("prevNext: mid component gets both", prevNextFor("alert", ctx), { prev: "accordion", next: "badge" })
    t.eq("prevNext: unplanned neighbor skipped",
      prevNextFor("badge", ctx), { prev: "alert", next: null }, "button unplanned → next null, not a dangling link")
    t.eq("prevNext: first has no prev", prevNextFor("accordion", ctx), { prev: null, next: "alert" })
    t.eq("prevNext: index null", prevNextFor("index", ctx), null)
    t.eq("prevNext: guide chain", prevNextFor("intro", ctx), { prev: null, next: "rtl" })
  }

  // ---- extractDemoScripts (D3 regression) ----
  {
    const html = [
      '<script>(function(){try{var k="shadless-docs-theme",v=localStorage.getItem(k);}})()</script>', // pre-paint
      '<script src="../shadless.js"></script>',
      '<script src="../js/tabs.js"></script>',
      '<script>shadless.initAll()</script>',
      '<script src="../radix-kernel.iife.js"></script>',
    ].join("\n")
    const { srcScripts, inlineScripts } = extractDemoScripts(html)
    t.eq("scripts: local srcs only (vendor excluded)", srcScripts, ["shadless.js", "js/tabs.js"])
    t.eq("scripts: pre-paint filtered from inline", inlineScripts, ["shadless.initAll()"])
  }

  // ---- rewritePaths (2026-08-25 incident: bare out.css is NOT dead code) ----
  {
    const t6 = '<link href="tooltip-out.css"><script src="../../dist/shadless.js"></script><script src="../../dist/js/tabs.js"></script>'
    const out = rewritePaths(t6)
    t.ok("rewrite: per-component css → unified", out.includes('href="../out.css"'), out)
    t.ok("rewrite: base path", out.includes('src="../shadless.js"'))
    t.ok("rewrite: component file into dist/js/", out.includes('src="../js/tabs.js"'))
    const bare = '<link href="out.css">'
    t.ok("rewrite: bare out.css form (the deleted-regex incident)",
      rewritePaths(bare).includes('href="../out.css"'), rewritePaths(bare))
  }
  // ---- ensureLink ----
  {
    t.eq("ensureLink: adds when missing", ensureLink("<head><title>x</title></head>"),
      '<head>\n<link rel="stylesheet" href="../out.css"><title>x</title></head>')
    t.eq("ensureLink: keeps existing", ensureLink('<head><link rel="stylesheet" href="../out.css"></head>'),
      '<head><link rel="stylesheet" href="../out.css"></head>')
  }
}
