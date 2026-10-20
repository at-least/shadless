// checker (dist/bin/shadless-check.mjs + dist/css/vocabulary.json) — the
// slot-only markup contract. Reads the SHIPPED artifact (the file npm's bin
// resolves), not the src copy: what a consumer runs is what must pass.
//
// Sections:
//   1. corpus: ZERO errors and an EXACT multiset of warnings over every
//      docs/demos/*.html, every dist/components/*.html and every committed
//      code tab (```text,name=… fences — exactly what users copy). Every
//      expected entry carries its reason (measured 2026-10-19; see below).
//   2. crafted inputs: each rule fires at the expected line:col — and the
//      skip paths (comments, script/textarea raw text) stay silent.
//   3. the CLI: exit codes 0/1/2, --strict, --json, the summary line, and
//      the missing-vocabulary crash (spawned, not imported).
//   4. CSS ⊆ vocabulary: every vocabulary-bearing selector in dist/css/*.css
//      is present in vocabulary.json — and the converse holds except the two
//      measured empty-apply values.
import { spawnSync } from "node:child_process"
import { cpSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"

const BIN = new URL("../../dist/bin/shadless-check.mjs", import.meta.url)
const root = (rel) => new URL(`../../${rel}`, import.meta.url)
const read = (rel) => readFileSync(root(rel), "utf8")

export async function run(t) {
  const check = /** @type {import("../../dist/bin/shadless-check.mjs")} */ (
    await import(BIN.href)
  )
  const vocabulary = check.loadVocabulary()

  // ---- 1. the corpus -------------------------------------------------------
  // Zero errors, exact warning multiset. Keys are `file|rule|message`
  // (position-independent: files reflow, the contract does not).
  /** @type {string[]} */
  const got = []
  const scan = (text, rel) => {
    for (const f of check.checkSource(text, vocabulary, rel)) {
      got.push(`${rel}|${f.rule}|${f.message}`)
    }
  }
  for (const f of readdirSync(root("docs/demos"), { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : 1))) {
    if (f.name.endsWith(".html")) scan(read(`docs/demos/${f.name}`), `docs/demos/${f.name}`)
  }
  for (const f of readdirSync(root("dist/components"), { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : 1))) {
    if (f.name.endsWith(".html")) scan(read(`dist/components/${f.name}`), `dist/components/${f.name}`)
  }
  const walkTabs = (dir, rel) => {
    for (const e of readdirSync(new URL(`${dir}/`, root("")), { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : 1))) {
      const p = `${dir}/${e.name}`
      if (e.isDirectory()) walkTabs(p, `${rel}/${e.name}`)
      else if (e.name.endsWith(".md")) {
        // One line-paired pass over every fence: an opening ``` line pairs
        // with the next line that starts with ```.
        const text = read(p)
        const lines = text.split("\n")
        let i = 0
        while (i < lines.length) {
          const open = /^```([A-Za-z0-9_-]*)/.exec(lines[i])
          if (!open) { i++; continue }
          let j = i + 1
          while (j < lines.length && !lines[j].startsWith("```")) j++
          const body = lines.slice(i + 1, j).join("\n")
          const nm = open[1] === "text" ? /^text,name=(\S+)/.exec(lines[i].slice(3)) : null
          if (nm) scan(body, `${rel}/${e.name} ${nm[1]}`)
          // html fences are copy source too — the rendered site serves them
          // as copyable blocks. Measured 2026-10-19: they carried 25
          // missing-marker findings (17 from upstream's teaching snippets,
          // projected by rewrite_leaked_jsx_fences without the markers React
          // renders; 8 from hand-written html: jsx_overrides.inc and the
          // shimmer guide's SHIMMER_MARKER_HTML).
          else if (open[1] === "html") scan(body, `${rel}/${e.name} html`)
          i = j + 1
        }
      }
    }
  }
  walkTabs("docs/site/content", "docs/site/content")

  t.ok(`corpus carries zero errors (got ${got.filter((k) => k.includes("|error|")).length})`, got.every((k) => !k.includes("|error|")))

  // The ONE remaining corpus fact (2026-10-19, post Stage-4):
  //  - data-slot="markdown" is an upstream authoring hook inside
  //    bubble-content — upstream's own Markdown component renders
  //    <Streamdown data-slot="markdown"> (.upstream/shadcn-ui/apps/v4/
  //    components/markdown.tsx) and the pinned bubble-variants example uses
  //    it; it is unstyled (not a slot), which is why unknown-slot is a
  //    warning, not an error. Both the authored demo page and its generated
  //    docs tab carry it.
  // Everything else the corpus used to miss was fixed AT ITS SOURCE in
  // Stage 4 and must stay fixed: the hand-authored demo pages'
  // markers (docs/demos/mode-toggle + message-scroller-*, and the
  // docs-build tab generated from mode-toggle), the probes/t7|t8 fixture
  // markers the demo node passes through, the emitter's default-content
  // card-header (emit::default_content), the docs' html fences (the leaked-JSX
  // projection now adds each slot's vocabulary markers; the hand-written html
  // in jsx_overrides.inc and SHIMMER_MARKER_HTML carries them), and the invented
  // data-slot="select-item-indicator" in message-scroller-animation
  // (replaced with the unslotted indicator span the shipped select renders).
  const expectedAll = [
    "docs/demos/bubble-variants.html|unknown-slot|data-slot=\"markdown\" is not in the vocabulary (shadless/vocabulary.json)", // authoring hook, not a slot
    "docs/site/content/components/bubble.md bubble-variants.html|unknown-slot|data-slot=\"markdown\" is not in the vocabulary (shadless/vocabulary.json)", // same hook, generated tab
  ]
  const key = (s) => [...s].sort()
  t.eq("corpus warnings are exactly the measured set", key(got), key(expectedAll))

  // ---- 2. crafted inputs ---------------------------------------------------
  const findings = (text, vocab = vocabulary) => check.checkSource(text, vocab, "t.html")
  const first = (text, vocab) => findings(text, vocab)[0]

  // typo'd data-variant — data-variant starts at column 25
  let f = first(`<div data-slot="button" data-variant="outlined">x</div>`)
  t.eq("unknown-value position", [f?.line, f?.col, f?.rule], [1, 25, "unknown-value"])
  t.ok("unknown-value is an error", f?.severity === "error")

  // multi-line tag: line/col follow the attribute, not the tag
  const multi = findings(`<div\n  data-slot="button"\n  data-variant="outlined">x</div>`)
  t.eq("multi-line findings", multi.map((x) => [x.rule, x.line, x.col]), [
    ["unknown-value", 3, 3], // axes first, then markers (checker emit order)
    ["missing-marker", 2, 3], // group/button required
  ])

  // data-size with no data-slot (data-orientation is runtime state: exempt)
  f = first(`<button data-size="lg">x</button>`)
  t.ok("axis-without-slot",
    !!f && f.rule === "axis-without-slot" && f.line === 1 && f.col === 9 && f.severity === "error",
    `got ${f ? JSON.stringify([f.rule, f.line, f.col, f.severity]) : "no finding"}`)
  t.eq("orientation without slot is exempt", findings(`<div data-orientation="horizontal">x</div>`), [])

  // duplicate attribute — the second (lost by HTML parsers) is reported
  const dup = findings(`<div data-slot="card" class="px-2" class="group/card">x</div>`)
  t.eq("duplicate-attribute", dup.map((x) => [x.rule, x.line, x.col]), [
    ["duplicate-attribute", 1, 36],
    ["missing-marker", 1, 6], // the FIRST class wins, so group/card is missing
  ])

  // unknown slot (warning) and the slotRefs exemption
  f = first(`<div data-slot="nope">x</div>`)
  t.eq("unknown-slot", [f?.rule, f?.severity, f?.line, f?.col], ["unknown-slot", "warning", 1, 6])
  t.eq("slotRefs do not warn", findings(`<div data-slot="checkbox-group">x</div>`), [])

  // missing marker
  f = first(`<div data-slot="card">x</div>`)
  t.eq("missing-marker", [f?.rule, f?.severity, f?.line, f?.col], ["missing-marker", "warning", 1, 6])
  t.eq("marker present is silent", findings(`<div data-slot="card" class="group/card">x</div>`), [])

  // the empty value: accepted exactly when the axis has a default (the
  // stylesheet's default twin selects [data-axis=""]). Every shipped axis
  // has a default (measured — the emitter emits a twin for any default it
  // keeps), so the no-default half runs against a crafted entry.
  t.eq("empty value with default is accepted", findings(`<div data-slot="attachment-media" data-variant="">x</div>`), [])
  const noDefaultVocab = {
    ...vocabulary,
    slots: {
      ...vocabulary.slots,
      t: { component: "t", axes: { size: { values: ["lg"], default: null } }, markers: [] },
    },
  }
  f = first(`<div data-slot="t" data-size="">x</div>`, noDefaultVocab)
  t.eq("empty value without default errors", [f?.rule, f?.line, f?.col], ["unknown-value", 1, 20])

  // an author-set axis no cva table names is checked from the upstream prop
  // union (card renders data-size={size}, size?: "default" | "sm"; its CSS
  // keys on it only as an in-@apply data-[size=sm] hook)
  t.eq("prop-union axis: a wrong value errors",
    findings(`<div data-slot="card" class="group/card" data-size="huge">x</div>`).map((x) => [x.rule, x.col]),
    [["unknown-value", 42]])
  t.eq("prop-union axis: the union's values and the defaulted empty value are silent", [
    findings(`<div data-slot="card" class="group/card" data-size="sm">x</div>`),
    findings(`<div data-slot="card" class="group/card" data-size="default">x</div>`),
    findings(`<div data-slot="card" class="group/card" data-size="">x</div>`)], [[], [], []])
  t.eq("prop-union axis: a menu-item variant typo errors",
    findings(`<div data-slot="dropdown-menu-item" class="group/dropdown-menu-item" data-variant="destructve">x</div>`).map((x) => [x.rule, x.col]),
    [["unknown-value", 70]])
  // the value set is the prop's type, not the CSS hooks: toggle-group types
  // size as VariantProps<typeof toggleVariants> (default|lg|sm) while its
  // stylesheet hooks only data-[size=sm]
  t.eq("prop-union axis: a typed value with no CSS hook stays legal",
    findings(`<div data-slot="toggle-group" class="group/toggle-group" data-size="lg">x</div>`), [])

  // <template> contents are markup
  f = first(`<div><template><button data-variant="lg"></template></div>`)
  t.eq("template contents are checked", [f?.rule, f?.line, f?.col], ["axis-without-slot", 1, 24])

  // entities decode in values
  f = first(`<div data-slot="nope&quot;x">y</div>`)
  t.ok(`entity decoded: ${f?.message}`, f?.message.includes(`data-slot="nope"x"`))

  // single-quoted, unquoted and boolean attribute forms
  t.eq("single-quoted", [first(`<div data-slot='nope'>y</div>`)?.col], [6])
  const unq = first(`<div data-slot=nope>y</div>`)
  t.eq("unquoted", [unq?.rule, unq?.col], ["unknown-slot", 6])

  // skip paths: a fake bad tag in a comment, in raw text, in a script
  t.eq("comment is skipped", findings(`<!-- <div data-slot="button" data-variant="outlined"> -->`), [])
  t.eq("script raw text is skipped", findings(`<script>var s = "<div data-variant=\\"x\\">";</script>`), [])
  t.eq("textarea raw text is skipped", findings(`<textarea><div data-variant="x"></textarea>`), [])
  t.eq("title raw text is skipped", findings(`<title><div data-variant="x"></title>`), [])

  // a raw ">" inside a quoted value does not end the tag (the tabs carry
  // [&>svg] shapes)
  t.eq("raw > inside quotes", findings(`<div data-slot="card" class="[&>svg]:size-3">x</div>`).map((x) => x.rule), ["missing-marker"])

  // ---- verifier round 1: HTML5 tokenizer agreement -------------------------
  // the end tag of a raw-text element accepts the self-closing slash (and
  // junk) like whitespace: </script/> closes the script per HTML5
  t.eq("</script/> closes the raw text", findings(`<script>var a=1;</script/><div data-variant="x"></div>`).map((x) => [x.rule, x.col]), [["axis-without-slot", 32]])
  t.eq("</textarea/> closes the raw text", findings(`<textarea>a</textarea/><div data-variant="x"></div>`).map((x) => x.rule), ["axis-without-slot"])
  // comments also close at --!>, and <!--> / <!---> are complete empty
  // comments (abrupt close) — none of them swallows following markup
  t.eq("comment closes at --!>", findings(`<!-- x --!> <div data-variant="bad">y</div>`).map((x) => x.rule), ["axis-without-slot"])
  t.eq("<!--> is a complete comment", findings(`<!--><div data-variant="bad">y</div>`).map((x) => x.rule), ["axis-without-slot"])
  t.eq("<!---> is a complete comment", findings(`<!---><div data-variant="bad">y</div>`).map((x) => x.rule), ["axis-without-slot"])
  // a '/'-closed raw-text tag still switches to raw text — browsers ignore
  // the self-closing flag for script/style/textarea/title
  t.eq("<script/> still opens raw text", findings(`<script/><div data-variant="x"></script>`), [])
  // a start tag truncated at EOF (unterminated quoted value) renders nothing
  // in a browser — one truncated advisory instead of invented findings
  t.eq("truncated start tag", findings(`<div data-slot="button" data-variant="out`).map((x) => [x.rule, x.severity, x.line, x.col]), [["truncated", "warning", 1, 1]])
  // a numeric reference above U+10FFFF becomes U+FFFD (parse5-measured),
  // never a RangeError crash
  f = first(`<div data-slot="button" class="group/button" data-variant="&#x110000;">x</div>`)
  t.eq("entity above U+10FFFF does not crash", [f?.rule, f?.severity, f?.col, f?.message.includes(`data-variant="�"`)], ["unknown-value", "error", 46, true])
  // '=' directly before an attribute name joins the name (HTML5
  // unexpected-equals-sign-before-attribute-name): no data-slot is seen
  t.eq("leading = joins the attribute name", findings(`<div =data-slot="card" class="group/card">x</div>`).map((x) => x.rule), [])
  // numeric references decode like a browser: the semicolon is optional and
  // NUL/surrogates become U+FFFD
  f = first(`<div data-slot="nope&#x27x">y</div>`)
  t.ok(`semicolon-less numeric ref decodes: ${f?.message}`, f?.message.includes(`data-slot="nope'x"`))
  f = first(`<div data-slot="nope&#0x">y</div>`)
  t.ok(`NUL becomes U+FFFD: ${f?.message}`, f?.message.includes(`data-slot="nope�x"`))
  f = first(`<div data-slot="nope&#xd800x">y</div>`)
  t.ok(`surrogate becomes U+FFFD: ${f?.message}`, f?.message.includes(`data-slot="nope�x"`))

  // ---- verifier round 2: foreign content, line breaks, entities, CLI ----
  // svg/math foreign content (parse5-measured): the tokenizer has no raw
  // text there, so title/style/script contents are markup, and <![CDATA[
  // sections run to ]]>
  t.eq("svg <title> is foreign markup", findings(`<svg><title><div data-variant="bad"></title></svg>`).map((x) => x.rule), ["axis-without-slot"])
  t.eq("svg <style> is foreign markup", findings(`<svg><style><div data-variant="bad"></style></svg>`).map((x) => x.rule), ["axis-without-slot"])
  t.eq("svg <script> is foreign markup", findings(`<svg><script>var x="<div data-variant='bad'>"</script></svg>`).map((x) => x.rule), ["axis-without-slot"])
  t.eq("svg CDATA runs to ]]>", findings(`<svg><![CDATA[a>b<div data-variant="bad">]]></svg>`), [])
  t.eq("</svg> inside CDATA does not close it", findings(`<svg><![CDATA[</svg>]]><title><div data-variant="bad"></title></svg>`).map((x) => x.rule), ["axis-without-slot"])
  // outside foreign content a <![CDATA[ is a bogus comment ending at the
  // first '>' (parse5-measured), so the fake tag after it IS scanned
  t.eq("CDATA outside svg ends at the first >", findings(`<![CDATA[a>b<div data-variant="bad">]]>`).map((x) => x.rule), ["axis-without-slot"])
  // a lone CR is a line break (HTML5 input preprocessing normalizes CRLF and
  // CR to LF; parse5-measured: the attribute sits on line 2, col 1)
  t.eq("lone CR is a line break", findings(`<div\rdata-variant="bad">x</div>`).map((x) => [x.rule, x.line, x.col]), [["axis-without-slot", 2, 1]])
  t.eq("CRLF is one line break", findings(`<div\r\ndata-variant="bad">x</div>`).map((x) => [x.rule, x.line, x.col]), [["axis-without-slot", 2, 1]])
  // numeric references accept upper- and lowercase x (a browser decodes
  // both, so class="group&#X2f;card" carries the marker)
  t.eq("uppercase-X numeric ref decodes", findings(`<div data-slot="card" class="group&#X2f;card">y</div>`), [])
  // a valueless (boolean) data-slot is PRESENT with an empty value (parse5
  // sees data-slot="") — an unknown slot, not an axis without a slot
  f = first(`<div data-slot data-variant="ghost">x</div>`)
  t.eq("boolean data-slot counts as present", [f?.rule, f?.severity, f?.line, f?.col], ["unknown-slot", "warning", 1, 6])
  // semicolon-less NAMED references stay literal — a documented narrowing
  // (browsers decode the legacy attribute names); only the marker-bearing
  // shapes above must decode
  f = first(`<div data-slot="nope&amp">y</div>`)
  t.ok(`semicolon-less named ref stays literal: ${f?.message}`, f?.message.includes(`data-slot="nope&amp"`))

  // ---- verifier round 4: the raw-text family, end-tag quoting, CR ---------
  // every element whose content a browser parses as RAW TEXT (scripting on,
  // the browser default — parse5-measured) is skipped, and scanning resumes
  // after its end tag (the trailing card div yields its marker warning)
  for (const tag of ["script", "style", "textarea", "title", "xmp", "iframe", "noembed", "noframes", "noscript"]) {
    t.eq(`${tag} raw text is skipped`,
      findings(`<${tag}><div data-variant="x"></${tag}><div data-slot="card">after</div>`).map((x) => x.rule),
      ["missing-marker"])
  }
  // plaintext runs to EOF: no end tag closes it (the tokenizer never leaves
  // the PLAINTEXT state), so even the trailing div is inside the raw text
  // (parse5-measured) — nothing is reported
  t.eq("plaintext runs to EOF",
    findings(`<plaintext><div data-variant="x"></plaintext><div data-slot="card">after</div>`).map((x) => x.rule),
    [])
  // the end tag's attribute region is tokenized like a start tag's: a '>'
  // inside a quoted value does not end the raw text (parse5-measured)
  t.eq("quoted > in a raw-text end tag",
    findings(`<script>x</script a="><div data-variant=bad>">after`).map((x) => x.rule),
    [])
  t.eq("single-quoted > ends the tag, not the region",
    findings(`<textarea>a</textarea b='>'><div data-slot="card">after</div>`).map((x) => x.rule),
    ["missing-marker"])
  // svg/math have no raw text and no PLAINTEXT state: the inner div is real
  // markup there (parse5-measured)
  t.eq("foreign plaintext is markup",
    findings(`<svg><plaintext><div data-variant="bad"></svg>`).map((x) => x.rule),
    ["axis-without-slot"])
  // HTML5 input preprocessing normalizes CRLF and a lone CR to LF everywhere
  // — inside attribute values too (parse5-measured) — so decoded values echo
  // \n, never \r
  f = first(`<div data-slot="button\r-x">y</div>`)
  t.ok("CR normalized in the value", !!f && f.message.includes(`data-slot="button\n-x"`),
    `got ${f ? JSON.stringify(f.message) : "no finding"}`)

  // ---- verifier round 5: end tags tokenize their attributes ---------------
  // a quote opens a value only after '=' (HTML5 before-attribute-value
  // state); elsewhere it is a name or unquoted-value character, so the end
  // tag still ends at the next '>' and the following div is built
  // (parse5-measured)
  t.eq("quote in a raw-text end tag's attribute name",
    findings(`<script>x</script "><div data-variant=bad>`).map((x) => x.rule),
    ["axis-without-slot"])
  t.eq("quote in a raw-text end tag's unquoted value",
    findings(`<style>x</style b=c"d><div data-variant=bad>`).map((x) => x.rule),
    ["axis-without-slot"])
  // ordinary end tags tokenize the same way: a quoted '>' is literal, and an
  // end tag truncated at EOF swallows the rest (parse5 builds no span in
  // either)
  t.eq("quoted > in an ordinary end tag",
    findings(`<div></div a="><span data-variant=x>">z`), [])
  t.eq("ordinary end tag truncated at EOF",
    findings(`<div></div a="><span data-variant=x>`), [])
  t.eq("stray quote in an ordinary end tag",
    findings(`<div></div "><span data-variant=x>`).map((x) => x.rule),
    ["axis-without-slot"])

  // ---- 3. the CLI ----------------------------------------------------------
  const tmp = mkdtempSync(join(tmpdir(), "shadless-check-"))
  try {
    writeFileSync(join(tmp, "bad.html"), `<div data-slot="button" data-variant="outlined">x</div>\n`)
    writeFileSync(join(tmp, "clean.html"), `<button data-slot="button" class="group/button">x</button>\n`)
    writeFileSync(join(tmp, "warn.html"), `<div data-slot="card">x</div>\n`)
    const run = (args) => spawnSync(process.execPath, [new URL("../../dist/bin/shadless-check.mjs", import.meta.url).pathname, ...args], { encoding: "utf8" })

    let r = run([tmp])
    t.eq("exit 1 on errors", r.status, 1)
    t.ok(`finding line format: ${r.stdout.split("\n")[0]}`, r.stdout.includes(`bad.html:1:25: error: data-variant="outlined" is not a value of button's variant axis`))
    // bad.html's button also lacks group/button: 1 error + 2 warnings
    t.eq("summary line", r.stdout.trimEnd().split("\n").at(-1), "shadless-check: 3 files, 1 errors, 2 warnings")

    r = run([join(tmp, "warn.html")])
    t.eq("warnings alone exit 0", r.status, 0)
    r = run(["--strict", join(tmp, "warn.html")])
    t.eq("--strict turns warnings into exit 1", r.status, 1)
    r = run(["--strict", join(tmp, "clean.html")])
    t.eq("--strict with a clean file exits 0", r.status, 0)

    r = run(["--json", join(tmp, "warn.html")])
    t.eq("--json exits 0 on warnings", r.status, 0)
    const parsed = JSON.parse(r.stdout)
    t.eq("--json findings", parsed.map((x) => [x.rule, x.line, x.col, x.severity]), [["missing-marker", 1, 6, "warning"]])

    // a directory argument recurses into subdirectories (spec: dirs recurse
    // into *.html/*.htm)
    mkdirSync(join(tmp, "nested", "sub"), { recursive: true })
    writeFileSync(join(tmp, "nested", "top.html"), `<div data-slot="button" class="group/button">top</div>\n`)
    writeFileSync(join(tmp, "nested", "sub", "inner.html"), `<div data-slot="alert" data-variant="nope">nested</div>\n`)
    r = run([join(tmp, "nested")])
    t.eq("directory recurses into subdirectories", r.status, 1)
    t.ok(`nested file scanned: ${r.stdout.split("\n")[0]}`, r.stdout.includes(join(tmp, "nested", "sub", "inner.html")))
    t.ok(`summary counts the nested file: ${r.stdout.split("\n").at(-2)}`, r.stdout.includes("2 files, 1 errors"))

    // a numeric reference above U+10FFFF decodes literally — the CLI reports
    // the finding instead of dying in the catch-all
    writeFileSync(join(tmp, "big-entity.html"), `<div data-slot="button" class="group/button" data-variant="&#x110000;">x</div>\n`)
    r = run([join(tmp, "big-entity.html")])
    t.eq("entity above U+10FFFF does not crash the CLI", r.status, 1)
    t.ok(`finding reported: ${r.stdout.split("\n")[0]}`, r.stdout.includes("unknown-value"))

    // the same real path twice — directly, and once directly plus once via
    // its directory — is scanned once (dedupe keeps first-seen order)
    writeFileSync(join(tmp, "dupe.html"), `<div data-slot="button" data-variant="outlined">x</div>\n`)
    r = run([join(tmp, "dupe.html"), join(tmp, "dupe.html")])
    t.eq("duplicate file args scan once", (r.stdout.match(/unknown-value/g) ?? []).length, 1)
    t.eq("dedupe summary", r.stdout.trimEnd().split("\n").at(-1), "shadless-check: 1 files, 1 errors, 1 warnings")
    mkdirSync(join(tmp, "dedupe"))
    writeFileSync(join(tmp, "dedupe", "d.html"), `<div data-slot="button" data-variant="outlined">x</div>\n`)
    r = run([join(tmp, "dedupe", "d.html"), join(tmp, "dedupe")])
    t.eq("file+dir dedupe summary", r.stdout.trimEnd().split("\n").at(-1), "shadless-check: 1 files, 1 errors, 1 warnings")

    r = run([])
    t.eq("no inputs exits 2", r.status, 2)
    t.ok("usage goes to stderr", r.stderr.includes("usage:"))
    r = run(["--bogus", tmp])
    t.eq("unknown option exits 2", r.status, 2)
    r = run([join(tmp, "nope.html")])
    t.eq("missing input file exits 2", r.status, 2)

    // the vocabulary is resolved beside dist/bin — a bin copied without
    // dist/css crashes loudly instead of checking nothing
    const lone = join(tmp, "lone")
    mkdirSync(lone)
    cpSync(new URL("../../dist/bin/shadless-check.mjs", import.meta.url).pathname, join(lone, "shadless-check.mjs"))
    r = spawnSync(process.execPath, [join(lone, "shadless-check.mjs"), join(tmp, "clean.html")], { encoding: "utf8" })
    t.ok("missing vocabulary exits non-zero", r.status !== 0)
    t.ok(`crash message: ${(r.stderr ?? "").split("\n")[0]}`, (r.stderr ?? "").includes("cannot read the attribute vocabulary"))

    // loadVocabulary enforces the contract fields it documents: a dropped or
    // wrong-typed axisAttributes crashes instead of silently checking
    // without the axis list (schema and slots were already checked — this is
    // the same reader that catches a schema drift)
    const badvocab = join(tmp, "badvocab")
    mkdirSync(join(badvocab, "css"), { recursive: true })
    mkdirSync(join(badvocab, "bin"), { recursive: true })
    cpSync(new URL("../../dist/bin/shadless-check.mjs", import.meta.url).pathname, join(badvocab, "bin", "shadless-check.mjs"))
    const badvocabRun = () =>
      spawnSync(process.execPath, [join(badvocab, "bin", "shadless-check.mjs"), join(tmp, "clean.html")], { encoding: "utf8" })
    writeFileSync(join(badvocab, "css", "vocabulary.json"), JSON.stringify({ schema: 1, slots: {}, slotRefs: [] }))
    r = badvocabRun()
    t.ok("vocabulary without axisAttributes exits non-zero", r.status !== 0)
    t.ok("crash names the dropped field", (r.stderr ?? "").includes("axisAttributes"),
      `stderr: ${(r.stderr ?? "").split("\n")[0]}`)
    writeFileSync(join(badvocab, "css", "vocabulary.json"), JSON.stringify({ schema: 1, slots: {}, slotRefs: [], axisAttributes: "variant" }))
    r = badvocabRun()
    t.ok("non-array axisAttributes is refused too", r.status !== 0 && (r.stderr ?? "").includes("axisAttributes"),
      `status ${r.status}, stderr: ${(r.stderr ?? "").split("\n")[0]}`)
    // slotRefs is the fourth contract field (it feeds the unknown-slot
    // exemption): dropped, the checker crashes; a string would substring-match
    // slot names and silently suppress unknown-slot warnings
    writeFileSync(join(badvocab, "css", "vocabulary.json"), JSON.stringify({ schema: 1, slots: {}, axisAttributes: [] }))
    r = badvocabRun()
    t.ok("vocabulary without slotRefs exits non-zero", r.status !== 0)
    t.ok("crash names slotRefs", (r.stderr ?? "").includes("slotRefs"),
      `stderr: ${(r.stderr ?? "").split("\n")[0]}`)
    writeFileSync(join(badvocab, "css", "vocabulary.json"), JSON.stringify({ schema: 1, slots: {}, axisAttributes: [], slotRefs: "checkbox-group" }))
    r = badvocabRun()
    t.ok("non-array slotRefs is refused too", r.status !== 0 && (r.stderr ?? "").includes("slotRefs"),
      `status ${r.status}, stderr: ${(r.stderr ?? "").split("\n")[0]}`)
  } finally {
    rmSync(tmp, { recursive: true, force: true })
  }

  // ---- 4. CSS ⊆ vocabulary -------------------------------------------------
  // Grammar per the measured dist/css shapes (map §1): slot base, value
  // rules, default twins, tag restrictions and cross-file cva refs carry
  // vocabulary; residue rules, branch rules (comma segments after the first
  // are ancestor context) and anchor classes do not. An UNPARSED selector is
  // a failure: a new grammar form must be classified, not ignored.
  const RE_BASE = /^\[data-slot="([^"]+)"\]$/
  const RE_VAL = /^\[data-slot="([^"]+)"\]:where\(\[data-([0-9A-Za-z_-]+)="([^"]*)"\]\)$/
  const RE_TWIN = /^\[data-slot="([^"]+)"\]:where\(:not\(\[data-([0-9A-Za-z_-]+)\]\), \[data-\2="([^"]*)"\], \[data-\2=""\]\)$/
  const RE_TAG = /^\[data-slot="([^"]+)"\]:is\(([a-z]+)\)$/
  const RE_RESIDUE = /^\[data-slot="([^"]+)"\]:where\(\.[^)]+\)$/
  const RE_DBLE = /^\[data-slot="([^"]+)"\]\[data-([0-9A-Za-z_-]+)="([^"]*)"\]$/
  const splitTop = (s) => {
    const out = []
    let depth = 0
    let cur = ""
    for (const ch of s) {
      if ("([".includes(ch)) depth++
      if (")]".includes(ch)) depth--
      if (ch === "," && depth === 0) {
        out.push(cur)
        cur = ""
      } else cur += ch
    }
    out.push(cur)
    return out
  }
  /** slot, (slot,axis,value) pairs, unparsed selectors */
  const cssSlots = new Set()
  const cssPairs = new Set()
  const unparsed = []
  for (const f of readdirSync(root("dist/css"), { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : 1))) {
    if (!f.name.endsWith(".css")) continue
    for (const m of read(`dist/css/${f.name}`).matchAll(/^  (.+?) \{ @apply (.+?); \}$/gm)) {
      const sel = m[1]
      let sm
      if ((sm = RE_BASE.exec(sel))) {
        cssSlots.add(sm[1])
      } else if ((sm = RE_VAL.exec(sel))) {
        cssSlots.add(sm[1]); cssPairs.add(`${sm[1]}|${sm[2]}|${sm[3]}`)
      } else if ((sm = RE_TWIN.exec(sel))) {
        cssSlots.add(sm[1]); cssPairs.add(`${sm[1]}|${sm[2]}|${sm[3]}`)
      } else if ((sm = RE_TAG.exec(sel))) {
        cssSlots.add(sm[1])
      } else if (RE_RESIDUE.exec(sel)) {
        // residue resets are class-anchored, vocabulary-inert
      } else if ((sm = RE_DBLE.exec(sel))) {
        cssSlots.add(sm[1]); cssPairs.add(`${sm[1]}|${sm[2]}|${sm[3]}`)
      } else if (sel.startsWith("[data-slot=") && sel.includes(":where(")) {
        // branch rule: segment 1 is self-conditions (runtime state, not
        // vocabulary); later segments are ancestor context whose slots are
        // vocabulary
        const arg = sel.slice(sel.indexOf(":where(") + 7, sel.lastIndexOf(")"))
        for (const seg of splitTop(arg).slice(1)) {
          for (const m2 of seg.matchAll(/\[data-slot="([^"]+)"\]/g)) cssSlots.add(m2[1])
        }
      } else if (sel.startsWith(".")) {
        // anchor / unlayered skin rules — no vocabulary
      } else {
        unparsed.push(`${f.name}: ${sel}`)
      }
    }
  }
  t.eq("every dist/css selector is classified", unparsed, [])

  const known = new Set([...Object.keys(vocabulary.slots), ...vocabulary.slotRefs])
  t.eq("CSS slot mentions are in the vocabulary",
    [...cssSlots].filter((s) => !known.has(s)), [])

  const vocabPairs = new Set()
  for (const [slot, entry] of Object.entries(vocabulary.slots)) {
    for (const [axis, info] of Object.entries(entry.axes)) {
      for (const v of info.values) vocabPairs.add(`${slot}|${axis}|${v}`)
    }
  }
  t.eq("CSS (slot,axis,value) pairs are in the vocabulary",
    [...cssPairs].filter((p) => !vocabPairs.has(p)), [])

  // The axes the upstream renders from a typed prop (IR element dataAxes)
  // reach the CSS only as in-@apply hooks — data-[size=sm] in the slot's own
  // rule, group-data-[size=sm]/card in a descendant's — never as
  // [data-axis=v] selectors, and a hook names only the values the CSS styles
  // (toggle-group hooks size sm, its prop types default|lg|sm). So: every
  // hooked value of a prop axis is listed, and the selector census below
  // covers the cva axes alone.
  const propAxes = new Set()
  for (const f of readdirSync(root("generated/ir")).sort()) {
    for (const c of JSON.parse(read(`generated/ir/${f}`)).components) {
      for (const e of c.elements) for (const axis of Object.keys(e.dataAxes ?? {})) propAxes.add(`${e.slot}|${axis}`)
    }
  }
  const unlisted = []
  let hooks = 0
  for (const f of readdirSync(root("dist/css")).sort()) {
    if (!f.endsWith(".css")) continue
    for (const m of read(`dist/css/${f}`).matchAll(/^  \[data-slot="([^"]+)"\] \{ @apply (.+?); \}$/gm)) {
      for (const h of m[2].matchAll(/(?:^|[\s:])(group-)?data-\[([a-z-]+)=([^\]]+)\](?:\/([a-z-]+))?/g)) {
        const slot = h[1] ? h[4] : m[1]
        if (slot === undefined || !propAxes.has(`${slot}|${h[2]}`)) continue
        hooks++
        if (!vocabPairs.has(`${slot}|${h[2]}|${h[3]}`)) unlisted.push(`${f}: ${h[0].trim()}`)
      }
    }
  }
  t.eq("every prop-axis value the CSS hooks is in the vocabulary", unlisted, [])
  t.ok("the prop-axis hook census sees the hooks", hooks > 0, `${hooks} hooks`)

  // the converse holds for the cva axes (those with a selector; a prop axis
  // can be one too, attachment-media variant) except the measured
  // empty-apply values (the emitter skips a value whose @apply resolves
  // empty, so attachment-media icon and marker default are invisible in
  // selectors by construction)
  const axisOf = (p) => p.split("|").slice(0, 2).join("|")
  const cvaAxes = new Set([...cssPairs].map(axisOf))
  const missing = [...vocabPairs].filter((p) => !cssPairs.has(p) && (cvaAxes.has(axisOf(p)) || !propAxes.has(axisOf(p))))
  t.eq("vocabulary values missing from CSS are exactly the measured empty-apply ones",
    missing, ["attachment-media|variant|icon", "marker|variant|default"])

  // ---- 5. tree-construction agreement (Chromium-measured) -----------------
  // Every shape here was measured against the elements Chromium BUILDS
  // (DOMParser): the rules must fire exactly on the elements a browser
  // constructs, never on a start tag it drops, and never miss one it builds.
  // select content parses under body rules (the current HTML standard;
  // parse5 8.0.1 still drops it): built, so checked
  t.eq("div inside select is built and checked",
    findings(`<select><div data-slot="button" data-variant="outlined"></div></select>`).map((x) => x.rule),
    ["unknown-value", "missing-marker"])
  t.eq("div inside option inside select is built and checked",
    findings(`<select><option><div data-slot="card"></div></option></select>`).map((x) => x.rule),
    ["missing-marker"])
  t.eq("td inside select is dropped (no table open)", findings(`<select><td data-slot="card"></td></select>`), [])
  t.eq("style inside select is raw text",
    findings(`<select><style><div data-variant="bad"></style></select>`), [])
  t.eq("textarea inside select is RCDATA",
    findings(`<select><textarea><div data-variant="bad"></textarea></select>`), [])
  // table-family drops: a browser builds no element without an open table
  t.eq("stray td is dropped", findings(`<div><td data-slot="card"></td></div>`), [])
  t.eq("stray tr is dropped", findings(`<div><tr data-slot="card"></tr></div>`), [])
  t.eq("stray tbody is dropped", findings(`<div><tbody data-slot="card"></tbody></div>`), [])
  t.eq("stray caption is dropped", findings(`<caption data-slot="card"></caption>`), [])
  t.eq("td in a table is built",
    findings(`<table><tbody><tr><td data-slot="card"></td></tr></tbody></table>`).map((x) => x.rule),
    ["missing-marker"])
  t.eq("div in a table is foster-parented, still built",
    findings(`<table><div data-slot="card"></div></table>`).map((x) => x.rule), ["missing-marker"])
  t.eq("td in a template is built (body rules)",
    findings(`<template><td data-slot="card"></td></template>`).map((x) => x.rule), ["missing-marker"])
  t.eq("head and frame build nothing, even inside a table or template",
    [findings(`<table><head data-slot="card"></head></table>`),
      findings(`<template><frame data-slot="card"></template>`)], [[], []])
  t.eq("</table> inside a template opened after the table is ignored",
    findings(`<table><template></table></template><td data-slot="card"></td>`).map((x) => x.rule),
    ["missing-marker"])
  t.eq("table closes at </table>",
    findings(`<table><tr><td>x</td></tr></table><div data-variant="bad">y</div>`).map((x) => x.rule),
    ["axis-without-slot"])
  // HTML integration points: their content parses as HTML again
  t.eq("style inside svg foreignObject is raw text",
    findings(`<svg><foreignObject><style><div data-slot="card"></style></svg>`), [])
  t.eq("script inside svg desc is script data",
    findings(`<svg><desc><script>var s = "<div data-variant='bad'>";</script></desc></svg>`), [])
  t.eq("textarea inside svg foreignObject is RCDATA",
    findings(`<svg><foreignObject><textarea><div data-slot="nope"></textarea></svg>`), [])
  t.eq("style inside svg title (itself an integration point) is raw text",
    findings(`<svg><title><style><div data-slot="card"></style></title></svg>`), [])
  t.eq("style inside math annotation-xml encoding=text/html is raw text",
    findings(`<math><annotation-xml encoding="text/html"><style><div data-slot="card"></style></annotation-xml></math>`), [])
  t.eq("annotation-xml with a non-HTML encoding is not an integration point",
    findings(`<math><annotation-xml encoding="application/mathml+xml"><style><div data-slot="card"></style></annotation-xml></math>`).map((x) => x.rule),
    ["missing-marker"])
  t.eq("style inside math mi is raw text (text integration point)",
    findings(`<math><mi><style><div data-slot="card"></style></mi></math>`), [])
  t.eq("unterminated style inside svg foreignObject swallows the rest",
    findings(`<svg><foreignObject><style>never closes <div data-slot="card">`), [])
  t.eq("plaintext inside math mi runs to EOF",
    findings(`<math><mi><plaintext><div data-variant="bad">`), [])
  t.eq("HTML title inside svg foreignObject is RCDATA",
    findings(`<svg><foreignObject><title><div data-slot="card"></title></svg>`), [])
  t.eq("integration content without raw text is still checked",
    findings(`<svg><foreignObject><div data-slot="card"></div></foreignObject></svg>`).map((x) => x.rule),
    ["missing-marker"])
  t.eq("CDATA inside svg desc is a bogus comment to the first >",
    findings(`<svg><desc><![CDATA[x><div data-variant="bad">]]></svg>`).map((x) => x.rule),
    ["axis-without-slot"])
  t.eq("CDATA inside math mi is a bogus comment: it swallows a tag only up to the first >",
    findings(`<math><mi><![CDATA[<div data-variant="bad">]]></mi></math>`), [])
  t.eq("CDATA inside math mi is a bogus comment: a tag after its first > is built",
    findings(`<math><mi><![CDATA[x><div data-variant="bad"]]></mi></math>`).map((x) => x.rule),
    ["axis-without-slot"])
  t.eq("an entity-encoded text/html encoding is still an integration point",
    findings(`<math><annotation-xml encoding="&#116;ext/html"><style><div data-slot="card"></style></annotation-xml></math>`), [])
  // foreign-content breakout: an HTML start tag pops the foreign elements
  t.eq("p breaks out; a later title is HTML raw text",
    findings(`<svg><p><title><div data-slot="button" data-variant="outlined"></title></svg>`), [])
  t.eq("p breaks out; CDATA is a bogus comment to the first >",
    findings(`<svg><p><![CDATA[<div data-slot="card"><span data-variant="nope">]]></svg>`).map((x) => x.rule),
    ["axis-without-slot"])
  t.eq("breakout closed by </svg> before the raw text",
    findings(`<svg><div></svg><title><div data-variant="bad"></title>`), [])
  t.eq("HTML tag inside an integration point does not pop the svg",
    findings(`<svg><desc><div data-slot="card"></div></desc></svg>`).map((x) => x.rule),
    ["missing-marker"])
  t.eq("font with color breaks out; a later style is raw text",
    findings(`<svg><font color="red"><style><div data-variant="bad"></style></svg>`), [])
  t.eq("font with face or size breaks out too",
    [findings(`<svg><font face="x"><style><div data-variant="bad"></style></svg>`),
      findings(`<svg><font size="2"><style><div data-variant="bad"></style></svg>`)], [[], []])
  t.eq("font without those attributes stays foreign (style is markup)",
    findings(`<svg><font><style><div data-variant="bad"></style></svg>`).map((x) => x.rule),
    ["axis-without-slot"])
  t.eq("mglyph returns to foreign mode inside mi",
    findings(`<math><mi><mglyph><style><div data-slot="card"></style></mglyph></mi></math>`).map((x) => x.rule),
    ["missing-marker"])
  // end-tag pops
  t.eq("</math> inside an svg is ignored (svg title content is HTML)",
    findings(`<svg></math><title><div data-variant="bad"></title>`).map((x) => x.rule),
    ["axis-without-slot"])
  t.eq("</svg> closes the svg through a foreignObject",
    findings(`<svg><foreignObject></svg><title><div data-variant="bad"></title>`), [])
  t.eq("</desc> pops back to foreign svg (svg title content is HTML)",
    findings(`<svg><desc>x</desc><title><div data-slot="card"></title></svg>`).map((x) => x.rule),
    ["missing-marker"])
}
