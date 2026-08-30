// demo script extraction (src/docs/demo-scripts.mjs) + demo path rewrites
// (tools/demo-lib.mjs) — Wave H: each assertion pins a shipped bug
// (pre-paint leaking into JS tabs, the 2026-08-25 rewritePaths incident).
//
// Wave 0 (2026-08-31): the addHeadingIds / prevNextFor assertions dropped
// with tools/docs-page-lib.mjs itself — that library only existed for the
// retired hand-rolled docs site; VitePress generates the TOC and the pager
// now. What remains here covers behavior that still ships.
import { extractDemoScripts } from "../../src/docs/demo-scripts.mjs"
import { rewritePaths, ensureLink } from "../demo-lib.mjs"

export function run(t) {
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

