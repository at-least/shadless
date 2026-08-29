// src/docs/transforms.mjs — single-source transform locators shared by
// the builder (tools/docs-build.mjs replaces spans) and the gate
// (tools/docs-fidelity-lib.mjs drops spans). Each assertion pins the
// single-source CONTRACT: the span the builder replaces must be exactly
// what the gate drops. If someone re-inlines a locator on either side,
// these fail.
import {
  fenceShadow, locateCodeTabsSpans, locateInstallSection, locateRtlMigrateSpan,
  applyTextAdjustments, TEXT_ADJUSTMENTS,
} from "../../src/docs/transforms.mjs"
import { withoutCodeTabs, withoutInstallSection, withoutRtlMigrate } from "../docs-fidelity-lib.mjs"

const RAW = [
  "intro prose",
  "<CodeTabs>",
  "<TabsList><TabsTrigger value=\"cli\">Command</TabsTrigger></TabsList>",
  "<TabsContent value=\"cli\">",
  "```bash",
  "npx shadcn@latest add x",
  "```",
  "</TabsContent>",
  "<TabsContent value=\"manual\">",
  "npm install radix-ui",
  "</TabsContent>",
  "</CodeTabs>",
  "## Usage",
  "<CodeTabs><TabsContent value=\"manual\">second</TabsContent></CodeTabs>",
  "tail",
].join("\n")

const RTL_RAW = [
  "intro",
  "## Get Started",
  "body",
  "",
  "## Migrating existing components",
  "",
  "<Steps>",
  "",
  "```bash",
  "npx shadcn@latest migrate rtl",
  "```",
  "",
  "</Steps>",
].join("\n")

const GUIDE_RAW = "intro\n## Installation\n\n```bash\nnpm i\n```\n\n## Usage\nbody"

const builderReplaceSpan = (raw, start, end, replacement) =>
  raw.slice(0, start) + replacement + raw.slice(end)

export function run(t) {
  // ---- fenceShadow ----
  {
    const src = "```js\nabc\n```\nX marks the spot"
    const shadow = fenceShadow(src)
    t.eq("fenceShadow: length preserved", shadow.length, src.length)
    t.ok("fenceShadow: prose untouched", shadow.includes("X marks the spot"))
    t.ok("fenceShadow: fence content blanked", !shadow.includes("abc"))
  }

  // ---- locateCodeTabsSpans ----
  {
    const spans = locateCodeTabsSpans(fenceShadow(RAW))
    t.eq("locate: both code-tabs blocks found (fenced look-alike safe)", spans.length, 2)
    t.ok("locate: span 0 wraps the shadcn CLI fence",
      RAW.slice(spans[0].start, spans[0].end).includes("npx shadcn@latest add x"))
    t.ok("locate: span 0 also wraps the manual tab",
      RAW.slice(spans[0].start, spans[0].end).includes("npm install radix-ui"))
    t.ok("locate: span 1 wraps second block",
      RAW.slice(spans[1].start, spans[1].end).includes("second"))
  }

  // ---- locateRtlMigrateSpan + contract ----
  {
    const span = locateRtlMigrateSpan(fenceShadow(RTL_RAW))
    t.ok("rtl: span located at the migrate heading", span && RTL_RAW.slice(span.start, span.start + 2) === "##")
    t.ok("rtl: end after </Steps>", RTL_RAW.slice(span.end - "</Steps>".length, span.end) === "</Steps>")
    const gated = withoutRtlMigrate(RTL_RAW)
    t.ok("rtl: gate drops the CLI fence", !gated.includes("migrate rtl"))
    t.ok("rtl: earlier sections survive", gated.includes("## Get Started"))
    t.eq("rtl: absent section → unchanged", [locateRtlMigrateSpan(fenceShadow("nope"))], [null])
  }

  // ---- SINGLE-SOURCE CONTRACT: builder replace == gate drop ----
  {
    const raw = RAW.split("\n").slice(0, 12).join("\n") + "\n## Usage\nbody"
    const spans = locateCodeTabsSpans(fenceShadow(raw))
    t.eq("contract: single span", spans.length, 1)
    const built = builderReplaceSpan(raw, spans[0].start, spans[0].end, "REPLACEMENT")
    const gated = withoutCodeTabs(raw)
    // gate drops the span (inserts "\n" in its place); builder replaces
    // it — removing the replacement must yield exactly the gate's view
    t.eq("contract: builder span == gate drop",
      built.replace("REPLACEMENT", "\n"), gated)
  }

  // ---- locateInstallSection + contract ----
  {
    const span = locateInstallSection(fenceShadow(GUIDE_RAW))
    t.ok("install: span located", span && GUIDE_RAW.slice(span.start, span.start + 2) === "##")
    t.ok("install: end at Usage heading", GUIDE_RAW.slice(span.end, span.end + 8) === "## Usage")
    // builder replaces [start,end) with utilsInstallMdx-shaped text
    // (heading + body); gate keeps only the heading — the headings both
    // sides carry must agree
    const built = builderReplaceSpan(GUIDE_RAW, span.start, span.end, "## Installation\n\nvanilla truth")
    const gated = withoutInstallSection(GUIDE_RAW)
    t.ok("contract: both views carry the Installation heading",
      /^## Installation$/m.test(built) && /^## Installation$/m.test(gated))
    t.ok("contract: both views drop the npm fence",
      !built.includes("npm i") && !gated.includes("npm i"))
    t.eq("contract: install absent → null/unchanged",
      [locateInstallSection(fenceShadow("no sections")), withoutInstallSection("no sections")],
      [null, "no sections"])
  }

  // ---- applyTextAdjustments ----
  {
    const raw = "The `AvatarImage` component displays the avatar image. It accepts all Radix UI Avatar Image props.\nThe `AvatarFallback` component displays a fallback when the image fails to load. It accepts all Radix UI Avatar Fallback props.\n"
    const out = applyTextAdjustments("avatar.mdx", raw)
    t.ok("adjust: false claim rewritten", out.includes("plain `<img data-slot=\"avatar-image\">`"))
    t.ok("adjust: original claim gone", !out.includes("It accepts all Radix UI Avatar Image props."))
    t.eq("adjust: unlisted file untouched", applyTextAdjustments("badge.mdx", raw), raw)
    let threw = null
    try { applyTextAdjustments("avatar.mdx", "unrelated prose") } catch (e) { threw = e.message }
    t.ok("adjust: missing find throws (re-anchor required)", /text adjustment avatar-props-prose/.test(threw ?? ""), threw ?? "no throw")
  }

  // ---- TEXT_ADJUSTMENTS shape (every op usable by the gate) ----
  {
    let ok = TEXT_ADJUSTMENTS.length >= 1
    for (const adj of TEXT_ADJUSTMENTS) {
      ok = ok && adj.id && adj.files.length >= 1 && adj.ops.length >= 1
      for (const op of adj.ops) ok = ok && op.find.length >= 10 && op.replace.length > 0 && op.find !== op.replace
    }
    t.ok("adjust: every descriptor complete (id/files/ops)", ok)
  }
}
