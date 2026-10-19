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
  const fence = /```text,name=(\S+)\n(.*?)^```/gms
  const walkTabs = (dir, rel) => {
    for (const e of readdirSync(new URL(`${dir}/`, root("")), { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : 1))) {
      const p = `${dir}/${e.name}`
      if (e.isDirectory()) walkTabs(p, `${rel}/${e.name}`)
      else if (e.name.endsWith(".md")) {
        const text = read(p)
        for (const m of text.matchAll(fence)) scan(m[2], `${rel}/${e.name} ${m[1]}`)
      }
    }
  }
  walkTabs("docs/site/content", "docs/site/content")

  t.ok(got.every((k) => !k.includes("|error|")), `corpus carries zero errors (got ${got.filter((k) => k.includes("|error|")).length})`)

  // Every entry is a measured, legitimate corpus fact (2026-10-19):
  //  - message-scroller-* / mode-toggle demos + their tabs (and the
  //    dark-mode guide's tab): hand-authored composition markup written
  //    before the vocabulary existed; the author omitted the marker classes
  //    React renders (IR cva base / skin markers). Tab and demo agree with
  //    each other — docs-code-parity proves that — so the pages are
  //    self-consistent; they deviate from React only in these markers.
  //    cn-menu-translucent is among them (a skin class with a skin body —
  //    the shipped menu stylesheets key a rule on it); cn-menu-target and
  //    cn-font-heading are NOT: no packaged stylesheet rule keys on them
  //    (cn-font-heading's only rule is the docs site's globals.css), so the
  //    vocabulary does not demand them.
  //  - dist/components/{accordion,avatar,card,carousel,checkbox,empty,
  //    field,radio-group,switch,toggle,toggle-group}.html: the minimal
  //    slot-only pages projected from the probes/t7 fixtures (initial
  //    commit), which predate the markers. The emitter's own render adds
  //    markers from the same vocabulary, so the static-tier pages (card,
  //    empty, field) are clean where they come from emit.
  //  - data-slot="markdown": authoring hook inside bubble-content (unstyled,
  //    not a slot) — the reason unknown-slot is a warning, not an error.
  //  - data-slot="select-item-indicator": hand-authored element in the
  //    message-scroller-animation demo, likewise unstyled.
  const expected = [
    "docs/demos/bubble-variants.html|unknown-slot|data-slot=\"markdown\" is not in the vocabulary (shadless/vocabulary.json)", // authoring hook, not a slot
    "docs/demos/mode-toggle.html|missing-marker|class is missing \"group/button-group\" — every data-slot=\"button-group\" element carries it (button-group)",
    "docs/demos/mode-toggle.html|missing-marker|class is missing \"group/button\" — every data-slot=\"button\" element carries it (button)",
    "docs/demos/mode-toggle.html|missing-marker|class is missing \"group/button\" — every data-slot=\"button\" element carries it (button)",
    "dist/components/accordion.html|missing-marker|class is missing \"group/accordion-trigger\" — every data-slot=\"accordion-trigger\" element carries it (accordion)",
    "dist/components/accordion.html|missing-marker|class is missing \"group/accordion-trigger\" — every data-slot=\"accordion-trigger\" element carries it (accordion)",
    "dist/components/avatar.html|missing-marker|class is missing \"group/avatar\" — every data-slot=\"avatar\" element carries it (avatar)",
    "dist/components/avatar.html|missing-marker|class is missing \"group/avatar\" — every data-slot=\"avatar\" element carries it (avatar)",
    "dist/components/card.html|missing-marker|class is missing \"group/card-header\" — every data-slot=\"card-header\" element carries it (card)",
    "dist/components/carousel.html|missing-marker|class is missing \"group/button\" — every data-slot=\"carousel-previous\" element carries it (carousel)",
    "dist/components/carousel.html|missing-marker|class is missing \"group/button\" — every data-slot=\"carousel-next\" element carries it (carousel)",
    "dist/components/checkbox.html|missing-marker|class is missing \"peer\" — every data-slot=\"checkbox\" element carries it (checkbox)",
    "dist/components/field.html|missing-marker|class is missing \"group/field-group\" — every data-slot=\"field-group\" element carries it (field)",
    "dist/components/radio-group.html|missing-marker|class is missing \"group/radio-group-item\" — every data-slot=\"radio-group-item\" element carries it (radio-group)",
    "dist/components/radio-group.html|missing-marker|class is missing \"peer\" — every data-slot=\"radio-group-item\" element carries it (radio-group)",
    "dist/components/radio-group.html|missing-marker|class is missing \"group/radio-group-item\" — every data-slot=\"radio-group-item\" element carries it (radio-group)",
    "dist/components/radio-group.html|missing-marker|class is missing \"peer\" — every data-slot=\"radio-group-item\" element carries it (radio-group)",
    "dist/components/switch.html|missing-marker|class is missing \"group/switch\" — every data-slot=\"switch\" element carries it (switch)",
    "dist/components/switch.html|missing-marker|class is missing \"peer\" — every data-slot=\"switch\" element carries it (switch)",
    "dist/components/toggle-group.html|missing-marker|class is missing \"group/toggle-group\" — every data-slot=\"toggle-group\" element carries it (toggle-group)",
    "dist/components/toggle-group.html|missing-marker|class is missing \"group/toggle\" — every data-slot=\"toggle-group-item\" element carries it (toggle-group)",
    "dist/components/toggle-group.html|missing-marker|class is missing \"group/toggle\" — every data-slot=\"toggle-group-item\" element carries it (toggle-group)",
    "dist/components/toggle.html|missing-marker|class is missing \"group/toggle\" — every data-slot=\"toggle\" element carries it (toggle)",
    "docs/site/content/components/bubble.md bubble-variants.html|unknown-slot|data-slot=\"markdown\" is not in the vocabulary (shadless/vocabulary.json)", // authoring hook, not a slot
    "docs/site/content/guides/dark-mode.md mode-toggle.html|missing-marker|class is missing \"group/button-group\" — every data-slot=\"button-group\" element carries it (button-group)",
    "docs/site/content/guides/dark-mode.md mode-toggle.html|missing-marker|class is missing \"group/button\" — every data-slot=\"button\" element carries it (button)",
    "docs/site/content/guides/dark-mode.md mode-toggle.html|missing-marker|class is missing \"group/button\" — every data-slot=\"button\" element carries it (button)",
  ]
  // expand the message-scroller cluster (identical shape per file; spelled
  // out below from the shipped checker so nothing is hand-transcribed)
  const COMPONENT_OF = {
    button: "button",
    "toggle-group": "toggle-group",
    "toggle-group-item": "toggle-group",
    "card": "card",
    "card-header": "card",
    "tabs-list": "tabs",
    "select-content": "select",
    "dropdown-menu-content": "dropdown-menu",
    "dropdown-menu-item": "dropdown-menu",
  }
  const demo = (name, slot, marker, n = 1) =>
    Array.from({ length: n }, () =>
      `docs/demos/message-scroller-${name}.html|missing-marker|class is missing "${marker}" — every data-slot="${slot}" element carries it (${COMPONENT_OF[slot]})`)
  const ms = [
    ...demo("anchoring", "card", "group/card"),
    ...demo("anchoring", "card-header", "group/card-header"),
    ...demo("anchoring", "button", "group/button"),
    ...demo("anchoring", "toggle-group", "group/toggle-group"),
    ...demo("anchoring", "toggle-group-item", "group/toggle", 2),
    ...demo("anchoring", "button", "group/button"),
    ...demo("animation", "card", "group/card"),
    ...demo("animation", "card-header", "group/card-header"),
    ...demo("animation", "button", "group/button"),
    ...demo("animation", "button", "group/button"),
    ...demo("animation", "select-content", "cn-menu-translucent"),
    ...Array.from({ length: 3 }, () =>
      "docs/demos/message-scroller-animation.html|unknown-slot|data-slot=\"select-item-indicator\" is not in the vocabulary (shadless/vocabulary.json)"), // hand-authored element, unstyled
    ...demo("commands", "card", "group/card"),
    ...demo("commands", "card-header", "group/card-header"),
    ...demo("commands", "button", "group/button"),
    ...demo("commands", "dropdown-menu-content", "cn-menu-translucent"),
    ...demo("commands", "dropdown-menu-item", "group/dropdown-menu-item", 4),
    ...demo("group-chat", "card", "group/card"),
    ...demo("group-chat", "card-header", "group/card-header"),
    ...demo("group-chat", "button", "group/button", 2),
    ...demo("load-history", "card", "group/card"),
    ...demo("load-history", "card-header", "group/card-header"),
    ...demo("load-history", "button", "group/button", 2),
    ...demo("opening-position", "card", "group/card"),
    ...demo("opening-position", "card-header", "group/card-header"),
    ...demo("opening-position", "tabs-list", "group/tabs-list"),
    ...demo("previous-context", "card", "group/card"),
    ...demo("previous-context", "card-header", "group/card-header"),
    ...demo("previous-context", "button", "group/button", 3),
    ...demo("previous-context", "dropdown-menu-content", "cn-menu-translucent"),
    ...demo("previous-context", "dropdown-menu-item", "group/dropdown-menu-item", 4),
    ...demo("scrollable", "card", "group/card"),
    ...demo("scrollable", "card-header", "group/card-header"),
    ...demo("streaming", "card", "group/card"),
    ...demo("streaming", "card-header", "group/card-header"),
    ...demo("streaming", "button", "group/button"),
    ...demo("streaming", "button", "group/button", 2),
    ...demo("streaming", "dropdown-menu-content", "cn-menu-translucent"),
    ...demo("streaming", "dropdown-menu-item", "group/dropdown-menu-item", 4),
    ...demo("visibility", "card", "group/card"),
    ...demo("visibility", "card-header", "group/card-header"),
  ]
  const expectedAll = [...expected, ...ms]
  const key = (s) => [...s].sort()
  t.eq("corpus warnings are exactly the measured set", key(got), key(expectedAll))

  // ---- 2. crafted inputs ---------------------------------------------------
  const findings = (text, vocab = vocabulary) => check.checkSource(text, vocab, "t.html")
  const first = (text, vocab) => findings(text, vocab)[0]

  // typo'd data-variant — data-variant starts at column 25
  let f = first(`<div data-slot="button" data-variant="outlined">x</div>`)
  t.eq("unknown-value position", [f?.line, f?.col, f?.rule], [1, 25, "unknown-value"])
  t.ok(f?.severity === "error", "unknown-value is an error")

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

  // an axis the slot's vocabulary does not list is skipped (card's data-size
  // exists only as an in-@apply hook — decision: cva axes only)
  t.eq("non-vocabulary axis is skipped", findings(`<div data-slot="card" class="group/card" data-size="sm">x</div>`), [])

  // <template> contents are markup
  f = first(`<div><template><button data-variant="lg"></template></div>`)
  t.eq("template contents are checked", [f?.rule, f?.line, f?.col], ["axis-without-slot", 1, 24])

  // entities decode in values
  f = first(`<div data-slot="nope&quot;x">y</div>`)
  t.ok(f?.message.includes(`data-slot="nope"x"`), `entity decoded: ${f?.message}`)

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
  t.ok(f?.message.includes(`data-slot="nope'x"`), `semicolon-less numeric ref decodes: ${f?.message}`)
  f = first(`<div data-slot="nope&#0x">y</div>`)
  t.ok(f?.message.includes(`data-slot="nope�x"`), `NUL becomes U+FFFD: ${f?.message}`)
  f = first(`<div data-slot="nope&#xd800x">y</div>`)
  t.ok(f?.message.includes(`data-slot="nope�x"`), `surrogate becomes U+FFFD: ${f?.message}`)

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
  t.ok(f?.message.includes(`data-slot="nope&amp"`), `semicolon-less named ref stays literal: ${f?.message}`)

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
    t.ok(r.stdout.includes(`bad.html:1:25: error: data-variant="outlined" is not a value of button's variant axis`), `finding line format: ${r.stdout.split("\n")[0]}`)
    t.ok(r.stdout.trimEnd().endsWith("shadless-check: 3 files, 1 errors, 0 warnings"), `summary line: ${r.stdout.split("\n").at(-2)}`)

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
    t.ok(r.stdout.includes(join(tmp, "nested", "sub", "inner.html")), `nested file scanned: ${r.stdout.split("\n")[0]}`)
    t.ok(r.stdout.includes("2 files, 1 errors"), `summary counts the nested file: ${r.stdout.split("\n").at(-2)}`)

    // a numeric reference above U+10FFFF decodes literally — the CLI reports
    // the finding instead of dying in the catch-all
    writeFileSync(join(tmp, "big-entity.html"), `<div data-slot="button" class="group/button" data-variant="&#x110000;">x</div>\n`)
    r = run([join(tmp, "big-entity.html")])
    t.eq("entity above U+10FFFF does not crash the CLI", r.status, 1)
    t.ok(r.stdout.includes("unknown-value"), `finding reported: ${r.stdout.split("\n")[0]}`)

    // the same real path twice — directly, and once directly plus once via
    // its directory — is scanned once (dedupe keeps first-seen order)
    writeFileSync(join(tmp, "dupe.html"), `<div data-slot="button" data-variant="outlined">x</div>\n`)
    r = run([join(tmp, "dupe.html"), join(tmp, "dupe.html")])
    t.eq("duplicate file args scan once", (r.stdout.match(/unknown-value/g) ?? []).length, 1)
    t.ok(r.stdout.trimEnd().endsWith("shadless-check: 1 files, 1 errors, 0 warnings"), `dedupe summary: ${r.stdout.split("\n").at(-2)}`)
    mkdirSync(join(tmp, "dedupe"))
    writeFileSync(join(tmp, "dedupe", "d.html"), `<div data-slot="button" data-variant="outlined">x</div>\n`)
    r = run([join(tmp, "dedupe", "d.html"), join(tmp, "dedupe")])
    t.ok(r.stdout.trimEnd().endsWith("shadless-check: 1 files, 1 errors, 0 warnings"), `file+dir dedupe summary: ${r.stdout.split("\n").at(-2)}`)

    r = run([])
    t.eq("no inputs exits 2", r.status, 2)
    t.ok(r.stderr.includes("usage:"), "usage goes to stderr")
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
    t.ok(r.status !== 0, "missing vocabulary exits non-zero")
    t.ok((r.stderr ?? "").includes("cannot read the attribute vocabulary"), `crash message: ${(r.stderr ?? "").split("\n")[0]}`)

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

  // the converse holds except the measured empty-apply values (the emitter
  // skips a value whose @apply resolves empty, so attachment-media icon and
  // marker default are invisible in selectors by construction)
  const missing = [...vocabPairs].filter((p) => !cssPairs.has(p))
  t.eq("vocabulary values missing from CSS are exactly the measured empty-apply ones",
    missing, ["attachment-media|variant|icon", "marker|variant|default"])
}
