// docs-fidelity-lib.mjs (Wave I gate B pure helpers) — each assertion
// pins a bug hit while building the gate or the audit that motivated it:
// fence scanning on info-string fences, generics-as-tags false alarms,
// inline-code table cells opening phantom raw headings, install-section
// replacement heading, chips key-order, radix unknown-status rule.
import {
  scanFences, blankFences, withoutCodeTabs, withoutInstallSection, withoutRtlMigrate,
  mdxPageFacts, htmlPageFacts, comparePage,
} from "../docs-fidelity-lib.mjs"

const FENCE_OK = [
  "```bash",
  "npx shadcn@latest add accordion",
  "```",
  "",
  "```tsx showLineNumbers",
  "<Accordion type=\"single\" />",
  "```",
].join("\n")

export function run(t) {
  // ---- scanFences ----
  {
    const f = scanFences(FENCE_OK)
    t.eq("fences: count + langs (info strings)", f.map((x) => x.lang), ["bash", "tsx"])
    t.eq("fences: content verbatim", f[0].content, "npx shadcn@latest add accordion")
  }
  {
    // closing fence of an info-string fence must not open a phantom fence
    const f = scanFences("```tsx show\ncode\n```\nprose stays out")
    t.eq("fences: no phantom from info-string close", f.length, 1)
  }
  {
    // 4-backtick fence containing a 3-backtick line stays one fence
    const f = scanFences("````mdx\n```tsx inner\n````\nplain")
    t.eq("fences: 4-backtick swallows inner 3", f.length, 1)
    t.ok("fences: inner content kept", f[0].content.includes("```tsx inner"))
  }
  {
    // unclosed fence: content until EOF (scanner must not crash)
    t.eq("fences: unclosed tolerated", scanFences("```js\nx").length, 1)
  }

  // ---- blankFences keeps offsets stable ----
  {
    const src = "```js\nabc\n```\nX marks the spot"
    const blanked = blankFences(src)
    t.eq("blankFences: length preserved", blanked.length, src.length)
    t.ok("blankFences: prose untouched", blanked.includes("X marks the spot"))
  }

  // ---- withoutCodeTabs ----
  {
    const raw = [
      "<CodeTabs>",
      "<TabsContent value=\"cli\">",
      "```bash",
      "npx shadcn@latest add x",
      "```",
      "</TabsContent>",
      "<TabsContent value=\"manual\">npm install radix-ui</TabsContent>",
      "</CodeTabs>",
      "## Usage",
    ].join("\n")
    const out = withoutCodeTabs(raw)
    t.ok("codeTabs: whole block dropped (CLI fence + manual tab)",
      !out.includes("npx shadcn") && !out.includes("npm install radix-ui") && out.includes("## Usage"))
    // a fenced look-alike must NOT be treated as a block (blankFences)
    const rawFenced = "```\n<CodeTabs>fenced</CodeTabs>\n```\nkeep"
    t.ok("codeTabs: fenced look-alike survives", withoutCodeTabs(rawFenced).includes("fenced"))
  }

  // ---- withoutRtlMigrate ----
  {
    const raw = [
      "## Get Started",
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
    const out = withoutRtlMigrate(raw)
    t.ok("rtlMigrate: section dropped incl. the CLI fence",
      !out.includes("migrate rtl") && out.includes("## Get Started"))
    t.ok("rtlMigrate: absent section → unchanged", withoutRtlMigrate("nothing") === "nothing")
  }

  // ---- withoutInstallSection (mirrors guideTransform incl. its heading) ----
  {
    const raw = "intro\n## Installation\n\n```bash\nnpm i\n```\n\n## Usage\nbody"
    const out = withoutInstallSection(raw)
    t.ok("install: section body dropped", !out.includes("npm i"))
    t.ok("install: replacement heading present", /\n## Installation\n/.test(out))
    t.ok("install: Usage kept", out.includes("## Usage\nbody"))
  }

  // ---- mdxPageFacts: heading merge + inline-code protection ----
  {
    const raw = [
      "---",
      "title: T",
      "description: D",
      "---",
      "## Reference",
      "| `h3` | `x` | `<h3>` |",
      "## Examples",
      "### `createChat()`",
      "<h2 class=\"demo\">The People</h2>",
    ].join("\n")
    const M = mdxPageFacts(raw)
    t.eq("facts: headings merged in order (backticks stripped, table cell inert)",
      M.headings, [
        { depth: 2, text: "Reference" },
        { depth: 2, text: "Examples" },
        { depth: 3, text: "createChat()" },
        { depth: 2, text: "The People" },
      ])
    t.eq("facts: frontmatter", M.frontmatter.title, "T")
  }
  {
    // audit lesson: generics in fence content must survive mdx-side facts
    const M = mdxPageFacts("```ts\nconst x = useChat<ChatMessage>({\n})\n```")
    t.ok("facts: generic kept in fence content (no tag stripping)", M.fences[0].content.includes("useChat<ChatMessage>({"))
  }

  // ---- htmlPageFacts + comparePage: detections ----
  const mdx = [
    "---",
    "title: Accordion",
    "description: A stacked set.",
    "links:",
    "  doc: https://example.com/doc",
    "---",
    "## Usage",
    "",
    "```tsx",
    "<Accordion />",
    "```",
    "",
    "<ComponentPreview name=\"accordion-demo\" />",
    "",
    "### Nested",
  ].join("\n")
  const goodHtml = [
    "<html><body>",
    "<h1>Accordion</h1>",
    '<p class="lead">A stacked set.</p>',
    '<p class="links"><a href="https://example.com/doc" rel="noopener">doc</a></p>',
    "<article>",
    '<h2 id="usage">Usage</h2>',
    '<pre><code class="language-tsx">&lt;Accordion /&gt;\n</code></pre>',
    '<div data-component-preview="accordion-demo" data-status="authored"></div>',
    '<h3 id="nested">Nested</h3>',
    "</article>",
    '<nav class="toc"><ul><li class="toc-2"><a href="#usage">Usage</a></li><li class="toc-3"><a href="#nested">Nested</a></li></ul></nav>',
    "</body></html>",
  ].join("\n")

  {
    const issues = comparePage(mdxPageFacts(mdx), htmlPageFacts(goodHtml), { pageName: "accordion", isComponentPage: true, expectedManualRef: null })
    t.eq("compare: clean pair → no issues", issues, [])
  }
  {
    // heading dropped from html
    const bad = goodHtml.replace('<h3 id="nested">Nested</h3>', "")
    const kinds = comparePage(mdxPageFacts(mdx), htmlPageFacts(bad), { pageName: "accordion", isComponentPage: true, expectedManualRef: null }).map((x) => x.split(": ")[0]).sort()
    t.ok("compare: heading loss detected (headings + toc)", kinds.includes("headings") && kinds.includes("toc"), kinds.join(","))
  }
  {
    // fence content dropped from html
    const bad = goodHtml.replace('<pre><code class="language-tsx">&lt;Accordion /&gt;\n</code></pre>', "")
    const kinds = comparePage(mdxPageFacts(mdx), htmlPageFacts(bad), { pageName: "accordion", isComponentPage: true, expectedManualRef: null }).map((x) => x.split(": ")[0])
    t.ok("compare: dropped fence detected", kinds.includes("fence"), kinds.join(","))
  }
  {
    // unknown preview status on a component page
    const bad = goodHtml.replace('data-status="authored"', 'data-status="unknown"')
    const kinds = comparePage(mdxPageFacts(mdx), htmlPageFacts(bad), { pageName: "accordion", isComponentPage: true, expectedManualRef: null }).map((x) => x.split(": ")[0])
    t.ok("compare: unknown status on component page flagged", kinds.includes("preview-status"), kinds.join(","))
    // off-component pages (guides) don't carry the radix-catalog
    // completeness rule
    const okGuide = comparePage(mdxPageFacts(mdx), htmlPageFacts(bad), { pageName: "intro", isComponentPage: false, expectedManualRef: null })
    t.eq("compare: unknown status tolerated off-component", okGuide, [])
  }
  {
    // chips mismatch (key-order-safe comparison)
    const bad = goodHtml.replace('href="https://example.com/doc" rel="noopener">doc<', 'href="https://example.com/other" rel="noopener">doc<')
    const kinds = comparePage(mdxPageFacts(mdx), htmlPageFacts(bad), { pageName: "accordion", isComponentPage: true, expectedManualRef: null }).map((x) => x.split(": ")[0])
    t.ok("compare: chip href drift detected", kinds.includes("chips"), kinds.join(","))
  }
  {
    // shim headings (data-slot) are excluded from comparison
    const withShim = goodHtml.replace("</article>", '<h3 class="flex"><button data-slot="accordion-trigger">Q</button></h3></article>')
    const issues = comparePage(mdxPageFacts(mdx), htmlPageFacts(withShim), { pageName: "accordion", isComponentPage: true, expectedManualRef: null })
    t.eq("compare: Accordion-shim h3 ignored", issues, [])
  }
}
