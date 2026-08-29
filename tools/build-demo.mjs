#!/usr/bin/env node
// FT8 / Step 1: upstream-faithful alert-demo emission.
//
// Reads examples/radix/<name>-demo.tsx, parses with @babel/parser, extracts
// the per-Alert composition (icon-name, title-text, description-text), and
// emits dist/components/<name>-demo.html with inline-styled HTML that
// matches the upstream aria render — programmatic, no LLM hand-authoring.
//
// Why inline styles: the emitter's no-class= gate (src/emitter/index.mjs:
// class tokens must match MARKER = /^(group|peer)…$/) rejects tailwind
// utility classes. The dist/components/*.html demos that the docs site
// iframe-load live outside the emitter gate (they're written directly by
// this tool), so we still avoid class= for consistency with the rest of
// the demo tree (docs/demos/*.html uses inline styles for the same
// reason — see docs/demos/alert-basic.html for the pattern).
//
// Scope (Step 1): alert-demo only. Generalization for other components is
// future work — the Alert composition pattern (icon + title + description)
// is the dominant shape for shadcn's -demo.tsx files but not the only
// one (button-demo has a Button with text; avatar-demo has an Avatar with
// image), so a generic emitter would need per-component slot trees.
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs"
import { parse } from "@babel/parser"
import { injectPrePaint } from "../src/docs/theme-prepaint.mjs"

const EXAMPLES = ".upstream/shadcn-ui/apps/v4/examples/radix"
const OUT_DIR = "dist/components"
mkdirSync(OUT_DIR, { recursive: true })

// lucide-react canonical SVG path bodies (just the inner shapes, no svg
// wrapper — we add the wrapper here to keep stroke attrs explicit). Only
// icons actually used by the demos we emit; extend as needed.
const ICONS = {
  CheckCircle2Icon: '<circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/>',
  InfoIcon: '<circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/>',
}

const svg = (body) =>
  `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${body}</svg>`

const escHtml = (s) => String(s)
  .replace(/&/g, "&amp;").replace(/</g, "&lt;")
  .replace(/>/g, "&gt;").replace(/"/g, "&quot;")

const parseTs = (src) =>
  parse(src, { sourceType: "module", plugins: ["typescript", "jsx"] })

const jsxName = (n) => {
  if (!n) return null
  if (n.type === "JSXIdentifier") return n.name
  if (n.type === "JSXMemberExpression") return `${jsxName(n.object)}.${jsxName(n.property)}`
  return null
}

// Find every JSXElement whose tag name matches `tag` inside `node` (DFS).
// Returns an array of matching JSXElement nodes in document order.
function findElements(node, tag) {
  const out = []
  const walk = (n) => {
    if (!n || typeof n.type !== "string") return
    if (n.type === "JSXElement" && jsxName(n.openingElement.name) === tag) {
      out.push(n)
    }
    for (const k of Object.keys(n)) {
      const v = n[k]
      if (Array.isArray(v)) v.forEach(walk)
      else if (v && typeof v === "object") walk(v)
    }
  }
  walk(node)
  return out
}

// Concatenate JSX children as text. Handles:
//   - JSXText (raw value preserved — the upstream demo .tsx files use
//     soft line wraps that we collapse via normalizeWhitespace below)
//   - JSXExpressionContainer containing StringLiteral/TemplateLiteral
//   - skips JSXElement children (those are nested slots, not text)
function childText(node) {
  let out = ""
  for (const c of node.children || []) {
    if (c.type === "JSXText") out += c.value
    else if (c.type === "JSXExpressionContainer") {
      const e = c.expression
      if (e.type === "StringLiteral") out += e.value
      else if (e.type === "TemplateLiteral" && e.quasis.length === 1)
        out += e.quasis[0].value.cooked
    }
  }
  // Collapse any whitespace runs (newlines + indentation from the source
  // .tsx's soft line wraps) to single spaces, then trim. Matches the
  // browser render where the text wraps to a single paragraph.
  return out.replace(/\s+/g, " ").trim()
}

// Extract (icon, title, description) from one <Alert> JSXElement by walking
// its children. Children expected (in any order):
//   <IconFoo />           → JSXElement with no slot attr
//   <AlertTitle>…</AlertTitle>
//   <AlertDescription>…</AlertDescription>
function parseAlert(el) {
  let icon = null, title = "", description = ""
  for (const c of el.children) {
    if (c.type !== "JSXElement") continue
    const tag = jsxName(c.openingElement.name)
    if (tag === "AlertTitle") title = childText(c)
    else if (tag === "AlertDescription") description = childText(c)
    else if (!icon) icon = tag  // first non-AlertTitle/Description element = icon
  }
  return { icon, title, description }
}

// Emit the 2-alert (or N-alert) grid HTML. Inline styles mirror upstream
// tailwind classes: grid w-full max-w-md items-start gap-4.
function renderAlertGrid(alerts) {
  const grid = alerts.map((a) => {
    const iconBody = ICONS[a.icon]
    if (!iconBody) throw new Error(`unknown icon: ${a.icon}`)
    return `<div data-slot="alert" role="alert" style="position:relative;display:grid;width:100%;grid-template-columns:auto 1fr;align-items:start;gap:0.5rem 0.75rem;border-radius:0.5rem;border:1px solid hsl(var(--border));background:hsl(var(--card));color:hsl(var(--card-foreground));padding:0.75rem 1rem;font-size:0.875rem;line-height:1.25rem">
  ${svg(iconBody)}
  <div data-slot="alert-title" style="grid-column:2;font-weight:500;letter-spacing:-0.01em;line-height:1.25rem">${escHtml(a.title)}</div>
  <div data-slot="alert-description" style="grid-column:2;color:hsl(var(--muted-foreground))">${escHtml(a.description)}</div>
</div>`
  }).join("\n")
  return `<div style="display:grid;width:100%;max-width:28rem;align-items:start;gap:1rem">
${grid}
</div>`
}

// Per-component emit rules. Each entry maps `<name>-demo.tsx` → function
// that reads the file, extracts composition data, returns the HTML body
// to put inside <body>. Add new components here as the demo pattern
// generalization expands.
const findDefaultExportBody = (ast) => {
  for (const n of ast.program.body) {
    if (n.type === "ExportDefaultDeclaration") {
      const d = n.declaration
      if (d?.type === "FunctionDeclaration") return d.body
      if (d?.type === "ArrowFunctionExpression") return d.body
    }
  }
  return null
}

const findJSXRoot = (n) => {
  if (!n) return null
  if (n.type === "JSXElement") return n
  if (n.type === "ReturnStatement") return findJSXRoot(n.argument)
  if (n.type === "ParenthesizedExpression") return findJSXRoot(n.expression)
  if (n.type === "BlockStatement") {
    for (const s of n.body) {
      const r = findJSXRoot(s); if (r) return r
    }
  }
  if (n.type === "ArrowFunctionExpression") return findJSXRoot(n.body)
  return null
}

// Per-component emit rules. RETIRED 2026-08-26: the alert-demo hand
// emitter (inline styles that were invalid dead code — hsl(var(--border))
// against oklch tokens) was replaced by tools/example-oracle.mjs, which
// renders the REAL React example from the pinned checkout and extracts
// the DOM — 1:1 with upstream by construction. This tool remains the
// pipeline anchor (Makefile demo-build) with an empty table.
const EMITTERS = {}

// ---- drive ------------------------------------------------------------------
let emitted = 0
for (const [demoName, emit] of Object.entries(EMITTERS)) {
  // demoName is the `<name>-demo` form; the file is examples/radix/<name>-demo.tsx
  const tsxPath = `${EXAMPLES}/${demoName}.tsx`
  if (!existsSync(tsxPath)) {
    // The demo-smoke gate counts these pages (expected 50 base pages), so a
    // missing upstream .tsx must fail the build — a green exit here would
    // push the failure downstream with a much less actionable message.
    console.error(`FAIL [${demoName}]: upstream .tsx missing: ${tsxPath}`)
    process.exit(1)
  }
  const src = readFileSync(tsxPath, "utf8")
  try {
    const body = emit(src)
    // pre-paint comes from the single source (theme-prepaint.mjs) — this
    // file previously carried a hand-copied duplicate of the script that
    // would silently drift from the real one (Wave I audit bug #3)
    const html = injectPrePaint(`<!doctype html>
<html><head><meta charset="utf-8"><title>shadless ${demoName}</title>
<link rel="stylesheet" href="../out.css">
</head>
<body style="padding:1rem">
${body}
</body></html>`)
    const out = `${OUT_DIR}/${demoName}.html`
    writeFileSync(out, html)
    emitted++
    console.log(`emit: ${demoName} → ${out}`)
  } catch (err) {
    console.error(`FAIL [${demoName}]: ${err.message}`)
    process.exit(1)
  }
}
console.log(`build-demo: ${emitted} files emitted`)
