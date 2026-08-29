// product-css (J2) — pure-function tests for the tokens extractor and the
// product verify checks. The extractor's job is a NARROW keep-list out of a
// globals.css that mixes library config with oracle-site chrome; these tests
// pin both directions (keep + exclude) on a synthetic replica of that shape.
import { readFileSync } from "node:fs"
import { extractTokens, buildProductEntry, verifyProduct } from "../product-css.mjs"

// Fixture tokens are CONCATENATED at runtime on purpose: tracked files are
// scanned by the tailwind CLI's repo-wide auto-scan (see tools/tw.mjs) and
// literals here would surface as phantom utility rules in dist/out.css.
const u = (...parts) => parts.join("")

const SYNTHETIC = `@import "tailwindcss";
@import "tw-animate-css";
/* === begin inlined shadcn/tailwind.css === */
@custom-variant data-open {
  &:where([data-state="open"]) {
    @slot;
  }
}
@utility shimmer {
  color: red;
}
/* === end inlined shadcn/tailwind.css === */
@source "./demo.html";

@custom-variant style-vega (&:where(.style-vega *));
@custom-variant dark (&:is(.dark *));

@theme inline {
  --color-background: var(--background);
}

@theme inline {
  --font-ar: "Noto Naskh Arabic";
}

:root {
  --background: oklch(1 0 0);
}

.dark {
  --background: oklch(0.145 0 0);
}

@layer base {
  * {
    @apply ${u("border", "-border")} ${u("outline", "-ring")}/50;
  }
  a:active,
  button:active {
    @apply opacity-60;
  }
}

@layer components {
  .dialog-ring {
    @apply rounded-xl;
  }
}
`

export function run(t) {
  const tokens = extractTokens(SYNTHETIC)

  // keeps
  t.ok("tokens: keeps @import lines", (tokens.match(/^@import .+;$/gm) || []).length === 2)
  t.ok("tokens: keeps inlined library block", tokens.includes("data-open") && tokens.includes("@utility shimmer"))
  t.ok("tokens: keeps dark variant", tokens.includes("@custom-variant dark"))
  t.ok("tokens: keeps token @theme (color-background)", tokens.includes("--color-background:"))
  t.ok("tokens: keeps :root/.dark vars", tokens.includes("--background: oklch(1 0 0)") && tokens.includes(".dark"))
  t.ok("tokens: keeps base border reset", tokens.includes(u("border", "-border")))

  // excludes (docs-site chrome)
  t.ok("tokens: excludes style packs", !tokens.includes("style-vega"))
  t.ok("tokens: excludes site-only @theme", !tokens.includes("--font-ar"))
  t.ok("tokens: excludes a:active dimming", !tokens.includes("opacity-60"))
  t.ok("tokens: excludes dialog-ring chrome", !tokens.includes("dialog-ring"))
  t.ok("tokens: excludes @source", !tokens.includes("@source"))

  // entry assembly is ordered: tokens, fixes, parts
  const entry = buildProductEntry("TOKENS", "FIXES", "PARTS")
  t.eq("entry: order", [entry.indexOf("TOKENS") < entry.indexOf("FIXES"),
    entry.indexOf("FIXES") < entry.indexOf("PARTS")], [true, true])

  // verify: missing slot in product build is caught; chrome leak is caught
  const parts = '[data-slot="alert"] { @apply grid; } [data-slot="alert-title"] { @apply font-medium; }'
  const out = ':root { --background: 1; --radius: 1 } [data-slot="alert"] { display: grid } [data-slot="alert-title"] { font-weight: 500 }'
  {
    const { missing, demoDropped, chrome, tokens: toks } = verifyProduct(out, out, parts)
    t.eq("verify: healthy build", [missing.length, demoDropped.length, chrome.length, toks.length], [0, 0, 0, 0])
  }
  {
    const broken = out.replace('[data-slot="alert-title"] { font-weight: 500 }', "")
    const { missing } = verifyProduct(broken, out, parts)
    t.eq("verify: dropped slot rule caught", missing, ["alert-title"])
  }
  {
    const leaky = out + ' [data-rehype-pretty-code-figure] { color: red } [data-slot="docs"] { margin: 0 }'
    const { chrome } = verifyProduct(leaky, out, parts)
    t.ok("verify: chrome leak caught", chrome.length >= 2)
  }
  {
    const noTokens = out.replace(/--/g, "XX")
    const { tokens: toks } = verifyProduct(noTokens, out, parts, "")
    t.eq("verify: missing token vars caught", toks.length, 2)
  }
  {
    // stray-class check: a standalone class with no origin in the product
    // source is a content-scan leak
    const phantom = u("m", "s-auto")
    const leaky = out + `\n  .${phantom} {\n    margin-inline-start: auto;\n  }\n`
    const { stray } = verifyProduct(leaky, out, parts, "TOKENS shadless.css PARTS")
    t.eq("verify: stray class caught", stray, [phantom])
    const clean = verifyProduct(out, out, parts, "grid font-medium")
    t.eq("verify: sourced classes not stray", clean.stray.length, 0)
  }

  // the real globals must stay extractable (marker/format drift fails loud)
  const real = readFileSync(new URL("../../probes/h4/globals.css", import.meta.url), "utf8")
  const realTokens = extractTokens(real)
  t.ok("tokens: real globals extracts", realTokens.includes("--color-background:") && realTokens.includes("scroll-fade"))
  t.ok("tokens: real globals excludes chrome", !realTokens.includes("rehype") && !realTokens.includes("style-vega"))
}
