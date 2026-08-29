// Bundle a contract def's React usage tree into OUT/oracle.{js,html}. Used by
// the contracts runner and by tools/example-fixture.mjs --contracts (which
// generates the shadless fixture pages FROM this render).
import { build } from "esbuild"
import { writeFileSync, mkdirSync } from "node:fs"
import { resolve, basename, relative } from "node:path"
import { CACHE_DIR } from "../oracle-lib.mjs"

export async function buildContractOracle(def, OUT, recorder = "") {
  const entry = `
import React from "react";
import { createRoot } from "react-dom/client";
${def.imports}
${recorder}
window.__open = true;
const root = createRoot(document.getElementById("root"));
const render = () => root.render((${def.usage}));
window.__setOpen = (o) => { window.__open = o; render(); };
render();
`
  // the ~7 MB bundle goes to the cache dir (see CACHE_DIR); OUT keeps only
  // the small review surfaces (oracle.html, shadless.html, result.json)
  mkdirSync(OUT, { recursive: true })
  mkdirSync(CACHE_DIR, { recursive: true })
  const entryFile = `${CACHE_DIR}/.contract-entry-${basename(OUT)}.mjs`
  const bundle = resolve(`${CACHE_DIR}/contract-${basename(OUT)}.js`)
  writeFileSync(entryFile, entry)
  await build({
    entryPoints: [entryFile], bundle: true, format: "iife",
    outfile: bundle, logLevel: "error",
    alias: {
      // resolved tree (tools/resolve-skins.mjs): cn-* already expanded —
      // the oracle and shadless compare against identical class semantics
      "@": resolve(".upstream/shadcn-ui/apps/v4"),
      "@/registry/bases/radix/ui": resolve("build/resolved-ui/ui"),
      "@/registry/bases/radix/lib": resolve("build/resolved-ui/lib"),
      "@/registry/bases/radix/hooks": resolve("build/resolved-ui/hooks"),
      // route-group indirection + subtree cut: ui components import the
      // demo-app icon switcher @/app/(create)/components/icon-placeholder,
      // which pulls next/navigation + nuqs into the oracle bundle — stub it
      "@/app/(create)/components/icon-placeholder": resolve("tools/contracts/stubs/icon-placeholder.jsx"),
    },
    loader: { ".tsx": "tsx" },
    // classic JSX (default) emits free `React.createElement` for sources
    // without an explicit React import — automatic runtime resolves via jsx-runtime
    jsx: "automatic",
  })
  writeFileSync(`${OUT}/oracle.html`,
    `<!doctype html><html><head>${def.oracleCss ? `<style>${def.oracleCss}</style>` : ""}</head><body><div id="root"></div><script src="${relative(resolve(OUT), bundle)}"></script></body></html>`)
}
