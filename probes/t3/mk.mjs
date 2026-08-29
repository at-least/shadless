// T3 mk: generate model-A (slot-keyed CSS) + model-B (class-in-HTML) pages,
// their globals, and the shadcn oracle page (React, real browser target).
import { readFileSync, writeFileSync } from "node:fs"
import { build } from "esbuild"
import { resolve } from "node:path"

const BASE = readFileSync("probes/h4/globals.css", "utf8")

// class strings [measured] from probes/h4/ir.json / demo.html
const BTN_BASE = "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"
const BTN_DEFAULT = "bg-primary text-primary-foreground hover:bg-primary/90"
const BTN_SIZE = "h-9 px-4 py-2 has-[>svg]:px-3"
const OVERLAY = "fixed inset-0 z-50 bg-black/50 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
const CONTENT = "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg duration-200 outline-none data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 sm:max-w-lg"
const TITLE = "text-lg leading-none font-semibold"
const DESC = "text-sm text-muted-foreground"
const CLOSE = "absolute top-4 right-4 rounded-xs opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"

// ---------- model A: semantic HTML, slot-keyed CSS --------------------------
const X_SVG = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18M6 6l12 12"/></svg>`
writeFileSync("probes/t3/page-a.html", `<!doctype html>
<html><head><meta charset="utf-8"><link rel="stylesheet" href="out-a.css"></head>
<body>
  <div style="padding:32px">
    <button data-slot="button" type="button">Button</button>
    <button data-slot="dialog-trigger" id="d1-trigger" type="button" aria-haspopup="dialog" aria-expanded="true">Open dialog</button>
  </div>
  <div data-slot="dialog-portal">
    <div data-slot="dialog-overlay" data-state="open"></div>
    <div data-slot="dialog-content" data-state="open" role="dialog" tabindex="-1">
      <div data-slot="dialog-header">
        <h2 data-slot="dialog-title">Are you absolutely sure?</h2>
        <p data-slot="dialog-description">This action cannot be undone.</p>
      </div>
      <button data-slot="dialog-close" type="button">${X_SVG}<span class="sr-only">Close</span></button>
    </div>
  </div>
</body></html>`)

const slotCss = `@layer components {
  body { @apply bg-background text-foreground; }
  [data-slot="button"], [data-slot="dialog-trigger"] { @apply ${BTN_BASE} ${BTN_DEFAULT} ${BTN_SIZE}; }
  [data-slot="dialog-overlay"] { @apply ${OVERLAY}; }
  [data-slot="dialog-content"] { @apply ${CONTENT}; }
  [data-slot="dialog-header"] { @apply flex flex-col gap-2; }
  [data-slot="dialog-title"] { @apply ${TITLE}; }
  [data-slot="dialog-description"] { @apply ${DESC}; }
  [data-slot="dialog-close"] { @apply ${CLOSE}; }
}`
writeFileSync("probes/t3/globals-a.css",
  BASE.replace('@source "./demo.html";', '@source "./page-a.html";') + "\n" + slotCss)

// ---------- model B: class-in-HTML (shadcn as-is) ---------------------------
const cls = (s) => `class="${s}"`
writeFileSync("probes/t3/page-b.html", `<!doctype html>
<html><head><meta charset="utf-8"><link rel="stylesheet" href="out-b.css"></head>
<body ${cls("bg-background text-foreground")}>
  <div style="padding:32px">
    <button ${cls(BTN_BASE + " " + BTN_DEFAULT + " " + BTN_SIZE)} data-slot="button" type="button">Button</button>
    <button ${cls(BTN_BASE + " " + BTN_DEFAULT + " " + BTN_SIZE)} data-slot="dialog-trigger" id="d1-trigger" type="button" aria-haspopup="dialog" aria-expanded="true">Open dialog</button>
  </div>
  <div ${cls("")} data-slot="dialog-portal">
    <div ${cls(OVERLAY)} data-slot="dialog-overlay" data-state="open"></div>
    <div ${cls(CONTENT)} data-slot="dialog-content" data-state="open" role="dialog" tabindex="-1">
      <div class="flex flex-col gap-2" data-slot="dialog-header">
        <h2 ${cls(TITLE)} data-slot="dialog-title">Are you absolutely sure?</h2>
        <p ${cls(DESC)} data-slot="dialog-description">This action cannot be undone.</p>
      </div>
      <button ${cls(CLOSE)} data-slot="dialog-close" type="button">${X_SVG.replace("<svg ", `<svg class="size-4" `)}<span class="sr-only">Close</span></button>
    </div>
  </div>
</body></html>`)
writeFileSync("probes/t3/globals-b.css",
  BASE.replace('@source "./demo.html";', '@source "./page-b.html";'))

// ---------- oracle: shadcn original, React ----------------------------------
const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
import { Button } from "@/registry/new-york-v4/ui/button";
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/registry/new-york-v4/ui/dialog";
const root = createRoot(document.getElementById("root"));
root.render(React.createElement(Dialog, { open: true },
  React.createElement(Button, null, "Button"),
  " ",
  React.createElement(DialogTrigger, null, "Open dialog"),
  React.createElement(DialogContent, null,
    React.createElement(DialogHeader, null,
      React.createElement(DialogTitle, null, "Are you absolutely sure?"),
      React.createElement(DialogDescription, null, "This action cannot be undone."),
    ),
  ),
));
`
writeFileSync("probes/t3/.oracle-entry.mjs", entry)
await build({
  entryPoints: ["probes/t3/.oracle-entry.mjs"], bundle: true, format: "iife",
  outfile: "probes/t3/oracle.js", logLevel: "error",
  alias: { "@": resolve(".upstream/shadcn-ui/apps/v4") },
  loader: { ".tsx": "tsx" },
})
writeFileSync("probes/t3/oracle.html",
  `<!doctype html><html><head><meta charset="utf-8"><link rel="stylesheet" href="${resolve("probes/h4/out.css")}"></head><body class="bg-background text-foreground"><div id="root" style="padding:32px"></div><script src="oracle.js"></script></body></html>`)
console.log("t3: wrote page-a/b, globals-a/b, oracle")
