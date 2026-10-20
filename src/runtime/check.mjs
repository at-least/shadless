#!/usr/bin/env node
// shadless-check — a static, offline, zero-dependency checker for the
// slot-only markup contract. The copy source (docs code tabs, demos,
// dist/components pages) carries `data-slot` + `data-*` attributes + only the
// classes the stylesheet cannot reproduce (the markers — group/peer markers
// and the skin classes a shipped rule keys on), so the contract a consumer
// or an agent must keep is the ATTRIBUTE vocabulary: which data-slot names
// exist, which data-variant/data-size/... values each slot accepts, and
// which markers every element of a slot must carry.
//
// vocabulary.json (generated from the IR by the same pipeline that emits
// dist/css, shipped as `shadless/vocabulary.json`) is the only list. A
// missing file is a crash with a clear message, never an empty vocabulary.
//
// What is checked (severities fixed by measurement over the shipped corpus —
// 5,276+1,657 slotted elements run before these were frozen):
//   error   unknown-value       data-<axis>="<v>" where <axis> is an axis of the
//                               slot and <v> is not one of its values. The
//                               empty value is accepted exactly when the axis
//                               has a default (the stylesheet's default twin
//                               selects [data-axis=""]). A slot's axes are its
//                               cva tables plus the data-<axis>={prop} the
//                               upstream renders from a typed prop (card
//                               data-size, menu-item data-variant: in-@apply
//                               hooks in the CSS, the value set from the prop's
//                               type); runtime state and boolean/numeric props
//                               are not axes and are not checked.
//   error   axis-without-slot   data-variant or data-size on an element with no
//                               data-slot (data-orientation is runtime state on
//                               many elements and is exempt).
//   error   duplicate-attribute the same attribute twice on one start tag —
//                               HTML parsers silently keep only the first, so a
//                               second class= is lost.
//   warning unknown-slot        data-slot value not in the vocabulary. Authoring
//                               hooks (data-slot="markdown") exist in the demos,
//                               so this is a warning, not an error.
//   warning missing-marker      the slot's vocabulary entry lists markers and
//                               the element's class list lacks one (e.g. a card
//                               without group/card loses every
//                               group-data-[size=…]/card:* descendant rule).
//                               Markers are the group/peer markers every
//                               render of the slot carries, plus the skin
//                               classes a shipped rule keys on (a skin body
//                               exists); dead utilities and body-less skin
//                               hooks are not demanded.
//   warning truncated           the start tag never closes before EOF (an
//                               unterminated quoted value runs to EOF) — a
//                               browser drops the whole tag and renders
//                               nothing of it; reported once instead of
//                               findings on markup no browser sees.
//
// Tokenizer scope: svg/math foreign content is tracked with the parts of
// tree construction that CHANGE TOKENIZATION (all parse5-measured): the
// integration points whose content a browser parses as HTML again (svg
// foreignObject/desc/title, math annotation-xml with a text/html-ish
// encoding, the MathML text integration points mi/mo/mn/ms/mtext — raw
// text switches there, and a CDATA section becomes a bogus comment to the
// first '>'), the WHATWG breakout list (an HTML start tag
// such as <div>/<p>/<span> inside svg/math pops the foreign elements and
// is reprocessed under HTML rules), and the end-tag pops (</svg> closes
// the svg through a desc or foreignObject; an end tag no tracked foreign
// element carries is ignored).
//
// Tree construction is modeled no further than tokenization: every start
// tag the tokenizer emits is checked, except the table family
// (caption/col/colgroup/frame/head/tbody/td/tfoot/th/thead/tr) with no
// <table> or <template> open — a browser builds no element for it
// (Chromium-measured). Content inside a <select> is built and checked: the
// HTML standard parses it under body rules since the customizable-select
// change (Chromium 151 builds a <div> there; parse5 8.0.1 still drops it).
// Foster parenting only MOVES built elements, so it cannot change a
// per-start-tag rule's result and needs no model.
//
// API: checkSource(text, vocabulary, file) returns the findings — the unit
// suite feeds it fence texts without temp files. The CLI is a thin wrapper.

import { readdirSync, readFileSync, realpathSync, statSync } from "node:fs"
import { join, resolve } from "node:path"
import { pathToFileURL } from "node:url"

/**
 * @typedef {{
 *   schema: number,
 *   axisAttributes: string[],
 *   slotRefs: string[],
 *   slots: Record<string, {
 *     component: string,
 *     axes: Record<string, { values: string[], default: string | null }>,
 *     markers: string[],
 *   }>,
 * }} Vocabulary
 *
 * @typedef {{
 *   file: string,
 *   line: number,
 *   col: number,
 *   severity: "error" | "warning",
 *   rule: "unknown-value" | "axis-without-slot" | "duplicate-attribute" |
 *         "unknown-slot" | "missing-marker" | "truncated",
 *   message: string,
 * }} Finding
 */

// The content of these elements parses as RAW TEXT (scripting assumed on —
// the browser default, parse5-measured): markup inside them is never
// tokenized, so it is never checked. plaintext is handled separately: its
// content runs to EOF, no end tag closes it. Inside svg/math none of them
// switches (foreign content has no raw text).
const RAW_TEXT_TAGS = new Set([
  "script", "style", "textarea", "title",
  "xmp", "iframe", "noembed", "noframes", "noscript",
])

/**
 * The foreign elements that CHANGE HOW THE NEXT TOKENS TOKENIZE
 * (parse5-measured). Untracked foreign elements (circle, path, ...) tokenize
 * exactly like their parent, so tracking them would change nothing. `ns` is
 * the namespace the browser builds them in: foreignObject/desc/title
 * integrate only in svg, annotation-xml/mi/mo/mn/ms/mtext only in math
 * (`<math><desc>` and `<math><foreignObject>` are plain foreign elements).
 * `htmlIntegration` (svg foreignObject/desc/title; math annotation-xml with
 * an encoding of text/html or application/xhtml+xml) marks an HTML
 * integration point: a browser parses its CONTENT as HTML again — raw-text
 * elements switch there, plaintext runs to EOF, and a `<![CDATA[` section is
 * a bogus comment running to the first `>`. `textIntegration` (math
 * mi/mo/mn/ms/mtext) does the same (Chromium- and parse5-measured); the
 * two differ only in which tags return to foreign rules (mglyph/malignmark
 * at a text integration point).
 * @typedef {{
 *   tag: string,
 *   ns: "svg" | "math",
 *   htmlIntegration: boolean,
 *   textIntegration: boolean,
 * }} ForeignEntry
 */
const FOREIGN_PUSH = new Set([
  "svg", "math", "foreignobject", "desc", "title", "annotation-xml",
  "mi", "mo", "mn", "ms", "mtext", "mglyph", "malignmark",
])

// The WHATWG breakout list: an HTML start tag with one of these names in
// foreign content pops the foreign elements until the nearest integration
// point (else all of them) and is reprocessed under HTML rules
// (parse5-measured — after it, raw text is raw text and CDATA is a bogus
// comment again). font breaks out only with a color, face or size
// attribute.
const BREAKOUT_TAGS = new Set([
  "b", "big", "blockquote", "body", "br", "center", "code", "dd", "div",
  "dl", "dt", "em", "embed", "font", "h1", "h2", "h3", "h4", "h5", "h6", "head",
  "hr", "i", "img", "li", "listing", "menu", "meta", "nobr", "ol", "p",
  "pre", "ruby", "s", "small", "span", "strong", "strike", "sub", "sup",
  "table", "tt", "u", "ul", "var",
])

// The table-family start tags a browser builds NO element for while no
// <table> or <template> is open (Chromium-measured; a template's content
// builds them — <template><td> keeps the td). head and frame build nothing
// in body content wherever they sit.
const TABLE_DROPPED = new Set([
  "caption", "col", "colgroup", "tbody", "td", "tfoot", "th", "thead", "tr",
])
const ALWAYS_DROPPED = new Set(["frame", "head"])

/**
 * The shipped vocabulary. Fails loudly when absent, unreadable, or not the
 * documented shape — an empty or partial vocabulary would silently check
 * nothing.
 * @returns {Vocabulary}
 */
export function loadVocabulary() {
  const url = new URL("../css/vocabulary.json", import.meta.url)
  let text
  try {
    text = readFileSync(url, "utf8")
  } catch (e) {
    throw new Error(
      `cannot read the attribute vocabulary at ${url.pathname} — dist/css/vocabulary.json must ship beside dist/bin (${/** @type {Error} */ (e).message})`,
    )
  }
  const v = /** @type {unknown} */ (JSON.parse(text))
  const doc = /** @type {{schema?: unknown, slots?: unknown, axisAttributes?: unknown, slotRefs?: unknown}} */ (v)
  const badShape =
    typeof doc !== "object" || doc === null ||
    doc.schema !== 1 ||
    typeof doc.slots !== "object" || doc.slots === null ||
    // the axisAttributes and slotRefs contracts are enforced like the
    // others: a dropped or wrong-typed field would silently check without
    // it (slotRefs feeds the unknown-slot exemption — a string would
    // substring-match slot names)
    !Array.isArray(doc.axisAttributes) ||
    /** @type {unknown[]} */ (doc.axisAttributes).some((a) => typeof a !== "string") ||
    !Array.isArray(doc.slotRefs) ||
    /** @type {unknown[]} */ (doc.slotRefs).some((s) => typeof s !== "string")
  if (badShape) {
    throw new Error(
      `${url.pathname} is not a schema-1 vocabulary — the contract is schema: 1, slots: object, slotRefs: string[], axisAttributes: string[]`,
    )
  }
  return /** @type {Vocabulary} */ (v)
}

/**
 * Decodes the entities a copied attribute value can carry: the named four
 * plus decimal/hex numeric references, both x cases, with an optional
 * semicolon (browsers decode these the same way — parse5-measured). A
 * documented narrowing: every other NAMED reference stays literal (`&sol;`,
 * and `&amp` without `;`), where a browser decodes every name — the
 * full table is not worth carrying for token matching. NUL, surrogates and
 * references above U+10FFFF become U+FFFD, never a crash.
 * @param {string} v
 * @returns {string}
 */
function decodeEntities(v) {
  if (!v.includes("&")) return v
  return v.replace(/&(?:quot;|amp;|lt;|gt;|#\d+;?|#[xX][0-9A-Fa-f]+;?)/g, (e) => {
    if (e === "&quot;") return '"'
    if (e === "&amp;") return "&"
    if (e === "&lt;") return "<"
    if (e === "&gt;") return ">"
    if (e[1] === "#") {
      const hex = e[2] === "x" || e[2] === "X"
      const digits = e.endsWith(";") ? e.slice(hex ? 3 : 2, -1) : e.slice(hex ? 3 : 2)
      const n = Number.parseInt(digits, hex ? 16 : 10)
      if (Number.isNaN(n)) return e // unreachable: the regex requires >= 1 digit
      return n > 0x10ffff || n === 0 || (n >= 0xd800 && n <= 0xdfff)
        ? "�"
        : String.fromCodePoint(n)
    }
    return e
  })
}

/**
 * 1-based line/column of a source offset. checkSource normalizes the input
 * (the HTML5 preprocessing step: CRLF and a lone CR become LF), so LF is the
 * only line break here.
 * @param {string} text
 * @param {number} pos
 * @returns {{ line: number, col: number }}
 */
function lineCol(text, pos) {
  let line = 1
  let nl = -1
  for (let i = 0; i < pos; i++) {
    if (text.charCodeAt(i) === 10) {
      line++
      nl = i
    }
  }
  return { line, col: pos - nl }
}

/**
 * One parsed attribute. `value`/`valuePos` are undefined for boolean
 * attributes; positions are absolute source offsets.
 * @typedef {{ name: string, lower: string, value: string | undefined, namePos: number, valuePos: number | undefined }} Attr
 */

/**
 * Parses one start tag's attribute region, starting just past the tag name.
 * Returns the attributes in source order and the offset of the closing `>`.
 * Quote-aware: a raw `>` inside a quoted value is literal (the code tabs
 * carry `[&>svg]` shapes since the class un-escaping change), so the tag end
 * cannot be a plain indexOf('>').
 * @param {string} text
 * @param {number} from offset just past the tag name
 * @returns {{ attrs: Attr[], end: number, truncated: boolean, selfClosed: boolean }} end = offset
 *   of the closing `>` (or of the last character when `truncated`);
 *   selfClosed is set only when a `/` immediately precedes the `>` (the
 *   HTML5 self-closing flag — a `/` inside an unquoted value or dropped
 *   elsewhere does not set it)
 */
function parseAttrs(text, from) {
  /** @type {Attr[]} */
  const attrs = []
  let i = from
  let selfClosed = false
  const n = text.length
  while (i < n) {
    const ch = text.charAt(i)
    if (/\s/.test(ch) || ch === "/") {
      if (ch === "/" && text.charAt(i + 1) === ">") selfClosed = true
      i++
      continue
    }
    if (ch === ">") {
      break
    }
    // attribute name: everything up to whitespace, =, > or /. A '=' directly
    // before the name joins the name (HTML5
    // unexpected-equals-sign-before-attribute-name — a browser sees an
    // attribute named "=data-slot", not "data-slot").
    let j = i
    if (text.charAt(j) === "=") j++
    while (j < n && !/\s/.test(text.charAt(j)) && text.charAt(j) !== "=" && text.charAt(j) !== ">" && text.charAt(j) !== "/") {
      j++
    }
    const name = text.slice(i, j)
    const namePos = i
    let k = j
    while (k < n && /\s/.test(text.charAt(k))) k++
    let value
    let valuePos
    if (text.charAt(k) === "=") {
      k++
      while (k < n && /\s/.test(text.charAt(k))) k++
      const q = text.charAt(k)
      if (q === '"' || q === "'") {
        const close = text.indexOf(q, k + 1)
        if (close >= 0) {
          value = text.slice(k + 1, close)
          valuePos = k + 1
          k = close + 1
        } else {
          // unterminated quote runs to EOF (malformed; the tag never ends)
          value = text.slice(k + 1)
          valuePos = k + 1
          k = n
        }
      } else {
        let e = k
        while (e < n && !/\s/.test(text.charAt(e)) && text.charAt(e) !== ">") e++
        value = text.slice(k, e)
        valuePos = k
        k = e
      }
    }
    attrs.push({ name, lower: name.toLowerCase(), value, namePos, valuePos })
    i = k
  }
  return { attrs, end: i < n ? i : n - 1, truncated: i >= n, selfClosed }
}

/**
 * Checks one HTML source text against the vocabulary. The input is
 * normalized first (the HTML5 preprocessing step: CRLF and a lone CR become
 * LF — parse5-measured), so line/column positions and decoded attribute
 * values are what a browser sees. `<template>` contents are scanned like
 * normal markup (the portal DOM is markup a consumer copies too); comments,
 * doctypes and the raw text of script/style/textarea/title/xmp/iframe/
 * noembed/noframes/noscript (scripting on, the browser default) are skipped,
 * and plaintext's content runs to EOF, so a commented-out, scripted or
 * embedded bad tag is never reported. svg/math foreign content has no raw
 * text and no PLAINTEXT state (title/style/script/plaintext there are normal
 * elements — parse5-measured) and allows `<![CDATA[` sections, so the
 * foreign elements that change tokenization are tracked: integration points
 * (raw text switches there; a CDATA section is a bogus comment at an HTML
 * integration point), the WHATWG breakout list and the end-tag pops (see
 * the header note). The table family outside an open table is skipped
 * without findings — a browser builds no element for it.
 *
 * @param {string} text
 * @param {Vocabulary} vocabulary
 * @param {string} file display name for the findings
 * @returns {Finding[]}
 */
export function checkSource(text, vocabulary, file) {
  // HTML5 input preprocessing: CRLF and a lone CR normalize to LF before any
  // scanning — one replacement up front, so lineCol and every decoded value
  // see what a browser sees
  text = text.replace(/\r\n?/g, "\n")
  /** @type {Finding[]} */
  const findings = []
  /** @param {number} pos */
  const at = (pos) => lineCol(text, pos)
  /**
   * @param {number} pos
   * @param {"error" | "warning"} severity
   * @param {Finding["rule"]} rule
   * @param {string} message
   */
  const push = (pos, severity, rule, message) => {
    const { line, col } = at(pos)
    findings.push({ file, line, col, severity, rule, message })
  }

  const n = text.length
  let i = 0
  // The open foreign elements that change tokenization (see ForeignEntry);
  // empty means HTML. The open <table>/<template> elements: while none is,
  // the table family builds nothing (TABLE_DROPPED).
  /** @type {ForeignEntry[]} */
  const foreign = []
  /** @type {("table" | "template")[]} */
  const tableMarkers = []
  /** a </table> or </template> closes everything opened inside it; no-op
   * when none is open, and a </table> inside a template opened after the
   * table is ignored (the template ends table scope — Chromium-measured)
   * @param {"table" | "template"} kind
   */
  const popTable = (kind) => {
    const k = tableMarkers.lastIndexOf(kind)
    if (k >= 0 && !(kind === "table" && tableMarkers.indexOf("template", k) >= 0)) tableMarkers.length = k
  }
  /** does a browser tokenize the next tokens under HTML rules (raw-text
   * elements switch, plaintext runs to EOF, a CDATA section is a bogus
   * comment, no foreign breakout)? True in HTML, and inside every
   * integration point */
  const htmlMode = () => {
    const top = foreign[foreign.length - 1]
    return top === undefined || top.htmlIntegration || top.textIntegration
  }
  /** a breakout start tag pops foreign elements until the nearest
   * integration point (else all of them) — the tag itself is reprocessed
   * under HTML rules, so the checker checks it like any HTML start tag and
   * drops the popped foreign state (parse5-measured) */
  const breakOut = () => {
    foreign.length = foreign.findLastIndex((e) => e.htmlIntegration || e.textIntegration) + 1
  }
  while (i < n) {
    const lt = text.indexOf("<", i)
    if (lt < 0) break
    if (text.startsWith("<!--", lt)) {
      // HTML5 also closes a comment at --!> (incorrectly closed) and treats
      // <!--> / <!---> as complete abruptly-closed empty comments
      if (text.startsWith("<!-->", lt)) {
        i = lt + 5
        continue
      }
      if (text.startsWith("<!--->", lt)) {
        i = lt + 6
        continue
      }
      const m = /--!?>/.exec(text.slice(lt + 4))
      i = m ? lt + 4 + m.index + m[0].length : n
      continue
    }
    if (!htmlMode() && text.startsWith("<![CDATA[", lt)) {
      // a CDATA section in foreign content is one token: everything to the
      // closing ]]>, or to EOF when it never closes (parse5-measured). At
      // an integration point it is a bogus comment instead (the generic <!
      // branch below) — Chromium- and parse5-measured.
      const close = text.indexOf("]]>", lt + 9)
      i = close >= 0 ? close + 3 : n
      continue
    }
    if (text.startsWith("<!", lt) || text.startsWith("<?", lt)) {
      // bogus comment: ends at the first '>' — a <![CDATA[ outside foreign
      // content included (parse5-measured)
      const end = text.indexOf(">", lt)
      i = end >= 0 ? end + 1 : n
      continue
    }
    if (text.startsWith("</", lt)) {
      const m = /^[a-zA-Z][^\s/>]*/.exec(text.slice(lt + 2, lt + 2 + 64))
      if (!m || m[0] === undefined) {
        // "</" not followed by a letter: a bogus comment ending at the first
        // '>' (</> is ignored the same way)
        const end = text.indexOf(">", lt)
        i = end >= 0 ? end + 1 : n
        continue
      }
      const lower = m[0].toLowerCase()
      // A foreign end tag pops through the nearest tracked element with the
      // same name; untracked foreign elements above it are popped along,
      // like the browser's walk (parse5-measured: </svg> closes the svg
      // through a desc or foreignObject, </math> inside an svg is ignored,
      // and a stray </svg> in HTML is ignored — nothing is popped).
      const k = foreign.findLastIndex((e) => e.tag === lower)
      if (k >= 0) foreign.length = k
      if (lower === "table" || lower === "template") popTable(lower)
      // an end tag's attribute region tokenizes like a start tag's (a quoted
      // '>' is literal); EOF inside it drops the tag and swallows the rest
      // (parse5-measured)
      const r = parseAttrs(text, lt + 2 + m[0].length)
      i = r.truncated ? n : r.end + 1
      continue
    }
    const nameM = /^[a-zA-Z][^\s/>]*/.exec(text.slice(lt + 1, lt + 1 + 64))
    if (!nameM || nameM[0] === undefined) {
      i = lt + 1 // "<" that opens no tag is text
      continue
    }
    const tag = nameM[0]
    const nameEnd = lt + 1 + tag.length
    const { attrs, end, truncated, selfClosed } = parseAttrs(text, nameEnd)
    if (truncated) {
      // EOF inside the tag (commonly an unterminated quoted value running to
      // EOF): a browser drops the whole start tag and renders nothing of it,
      // so report the truncation instead of findings on markup nobody renders
      push(lt, "warning", "truncated",
        "the start tag is truncated at EOF — a browser drops the whole tag")
      i = end + 1
      continue
    }
    const lower = tag.toLowerCase()

    // The start tag's attributes by lowercased name, first occurrence wins
    // (HTML parsers keep the first) — one map for every rule below.
    /** @type {Map<string, Attr>} */
    const byLower = new Map()
    for (const a of attrs) {
      if (!byLower.has(a.lower)) byLower.set(a.lower, a)
    }

    // Mode BEFORE this tag: raw-text/plaintext switching and breakout
    // suppression follow the OPEN elements, not this tag (an svg <title> is
    // not raw text — it is an integration point whose content is HTML).
    let inHtml = htmlMode()
    if (inHtml && foreign.at(-1)?.textIntegration && (lower === "mglyph" || lower === "malignmark")) {
      // at a MathML text integration point, mglyph/malignmark go back to
      // foreign rules (parse5-measured)
      inHtml = false
    }
    if (!inHtml && BREAKOUT_TAGS.has(lower) &&
        (lower !== "font" || byLower.has("color") || byLower.has("face") || byLower.has("size"))) {
      // an HTML start tag in the breakout list pops foreign elements until
      // the nearest integration point (or all of them) and is reprocessed
      // under HTML rules (parse5-measured)
      breakOut()
      inHtml = true
    }

    // The table family builds nothing while no table or template is open
    // (Chromium-measured), so the rules must not fire on it.
    let dropped = false
    if (inHtml) {
      if (lower === "table" || lower === "template") tableMarkers.push(lower)
      else if (ALWAYS_DROPPED.has(lower) || (TABLE_DROPPED.has(lower) && tableMarkers.length === 0)) dropped = true
    }

    if (!dropped && !selfClosed &&
        (inHtml ? lower === "svg" || lower === "math" : FOREIGN_PUSH.has(lower))) {
      // a self-closed foreign element carries no content (the self-closing
      // flag is acknowledged in foreign content), so it changes no mode
      /** @type {"svg" | "math"} */
      let ns
      if (inHtml) {
        ns = lower === "svg" ? "svg" : "math"
      } else {
        const parent = foreign[foreign.length - 1]
        if (parent === undefined) {
          // unreachable: !inHtml means a plain foreign element is open
          // (htmlMode() is false only then)
          throw new Error("internal: foreign push with no open foreign element")
        }
        ns = parent.ns
      }
      foreign.push({
        tag: lower,
        ns,
        htmlIntegration:
          (ns === "svg" && (lower === "foreignobject" || lower === "desc" || lower === "title")) ||
          (ns === "math" && lower === "annotation-xml" &&
            ["text/html", "application/xhtml+xml"].includes(
              decodeEntities(byLower.get("encoding")?.value ?? "").toLowerCase())),
        textIntegration: ns === "math" &&
          (lower === "mi" || lower === "mo" || lower === "mn" || lower === "ms" || lower === "mtext"),
      })
    }

    if (!dropped) {
      // rule: duplicate-attribute (case-insensitive; HTML keeps the first)
      for (const a of attrs) {
        const seen = byLower.get(a.lower)
        if (seen !== undefined && seen !== a) {
          push(a.namePos, "error", "duplicate-attribute",
            `duplicate attribute "${a.name}" — HTML parsers keep only the first one (line ${at(seen.namePos).line}, col ${at(seen.namePos).col})`)
        }
      }
      const slot = byLower.get("data-slot")
      // a valueless (boolean) data-slot is present with an empty value, like
      // every HTML parser sees it — it is an (unknown) slot, never "no slot"
      const slotName = slot ? decodeEntities(slot.value ?? "") : undefined
      if (slotName !== undefined) {
        const entry = vocabulary.slots[slotName]
        if (entry === undefined) {
          if (!vocabulary.slotRefs.includes(slotName)) {
            push(/** @type {number} */ (slot?.namePos), "warning", "unknown-slot",
              `data-slot="${slotName}" is not in the vocabulary (shadless/vocabulary.json)`)
          }
        } else {
          // rule: unknown-value (cva axes of this slot only)
          for (const [axis, info] of Object.entries(entry.axes)) {
            const a = byLower.get(`data-${axis}`)
            if (!a) continue
            const v = a.value !== undefined ? decodeEntities(a.value) : ""
            if (v === "") {
              if (info.default === null) {
                push(/** @type {number} */ (a.namePos), "error", "unknown-value",
                  `data-${axis}="" on data-slot="${slotName}" — the ${axis} axis has no default, so the empty value selects nothing`)
              }
              continue
            }
            if (!info.values.includes(v)) {
              push(/** @type {number} */ (a.namePos), "error", "unknown-value",
                `data-${axis}="${v}" is not a value of ${slotName}'s ${axis} axis (${info.values.join(", ")})`)
            }
          }
          // rule: missing-marker
          if (entry.markers.length > 0) {
            const cls = byLower.get("class")
            const have = new Set(
              (cls && cls.value !== undefined ? decodeEntities(cls.value) : "").split(/\s+/),
            )
            for (const marker of entry.markers) {
              if (!have.has(marker)) {
                push(/** @type {number} */ (slot?.namePos), "warning", "missing-marker",
                  `class is missing "${marker}" — every data-slot="${slotName}" element carries it (${entry.component})`)
              }
            }
          }
        }
      } else {
        // rule: axis-without-slot — variant/size name an axis; orientation is
        // runtime state on many elements and is exempt
        for (const axis of ["variant", "size"]) {
          const a = byLower.get(`data-${axis}`)
          if (a) {
            push(a.namePos, "error", "axis-without-slot",
              `data-${axis} without a data-slot — the attribute is an axis of a slot's vocabulary; set data-slot first`)
          }
        }
      }
    }

    if (inHtml && !dropped && RAW_TEXT_TAGS.has(lower)) {
      // skip the raw text; the end tag is not markup either. The tokenizer
      // ignores the self-closing flag on these elements (<script/> still
      // switches to raw text), and an end tag may carry a '/' or junk after
      // the name (</script/> closes the element per HTML5). The end tag's
      // attribute region is tokenized by parseAttrs, like a start tag's: a
      // '>' inside a quoted value is literal (</script a=">"> ends the tag),
      // and a quote opens a value only after '=' (parse5-measured). Inside
      // svg/math none of them switches: foreign content has no raw text.
      const close = new RegExp(`</${tag}(?=[\\s/>])`, "i").exec(text.slice(end + 1))
      if (!close) {
        i = n // never closes: the raw text runs to EOF, like a browser
        continue
      }
      // EOF inside the end tag drops it, so the element never closes — the
      // raw text runs to EOF
      const r = parseAttrs(text, end + 1 + close.index + close[0].length)
      i = r.truncated ? n : r.end + 1
      continue
    }
    if (inHtml && !dropped && lower === "plaintext") {
      // plaintext's content runs to EOF: the tokenizer never leaves the
      // PLAINTEXT state, so not even </plaintext> closes it
      // (parse5-measured). Foreign content has no PLAINTEXT state — the
      // svg/math guard above runs first.
      i = n
      continue
    }
    i = end + 1
  }
  return findings
}

/**
 * Expands the CLI's positional arguments: a directory recurses (sorted) into
 * *.html/*.htm, subdirectories included, so `shadless-check src` covers the
 * whole tree; explicit files of any extension are scanned as HTML text. The
 * same real path (a repeated argument, a file also covered by a directory
 * argument) is scanned once — dedupe by resolved real path, first-seen
 * order.
 * @param {string[]} inputs
 * @returns {string[]}
 */
function collectFiles(inputs) {
  /** @type {string[]} */
  const out = []
  const seen = new Set()
  /** @param {string} p */
  const push = (p) => {
    const real = realOrGiven(p)
    if (!seen.has(real)) {
      seen.add(real)
      out.push(p)
    }
  }
  /** @param {string} abs */
  const walk = (abs) => {
    for (const e of readdirSync(abs, { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : 1))) {
      const p = join(abs, e.name)
      if (e.isDirectory()) walk(p)
      else if (e.name.endsWith(".html") || e.name.endsWith(".htm")) push(p)
    }
  }
  for (const input of inputs) {
    const abs = resolve(input)
    if (statSync(abs).isDirectory()) walk(abs)
    else push(abs)
  }
  return out
}

/**
 * @param {string} p
 * @returns {string} the real path when resolvable, the input otherwise
 * (a not-yet-existing path is a usage error reported by the caller)
 */
function realOrGiven(p) {
  try {
    return realpathSync(p)
  } catch {
    return p
  }
}

/**
 * The CLI. `npx shadless-check [--strict] [--json] <file|dir>...`
 * Exit 1 when errors are found (or warnings, with --strict), 2 on usage
 * errors, else 0.
 * @param {string[]} argv process.argv.slice(2)
 * @returns {number} the process exit code
 */
export function main(argv) {
  /** @type {string[]} */
  const inputs = []
  let strict = false
  let json = false
  for (const a of argv) {
    if (a === "--strict") strict = true
    else if (a === "--json") json = true
    else if (a === "--help" || a === "-h") {
      console.log("usage: shadless-check [--strict] [--json] <file|dir>...")
      return 0
    } else if (a.startsWith("-")) {
      console.error(`shadless-check: unknown option ${a}`)
      return 2
    } else {
      inputs.push(a)
    }
  }
  if (inputs.length === 0) {
    console.error("usage: shadless-check [--strict] [--json] <file|dir>...")
    return 2
  }
  const vocabulary = loadVocabulary()
  /** @type {string[]} */
  let files
  /** @type {Finding[]} */
  let findings
  try {
    files = collectFiles(inputs)
    findings = []
    for (const f of files) {
      const text = readFileSync(f, "utf8")
      findings.push(...checkSource(text, vocabulary, f))
    }
  } catch (e) {
    console.error(`shadless-check: ${/** @type {Error} */ (e).message}`)
    return 2
  }
  if (json) {
    console.log(JSON.stringify(findings, null, 2))
  } else {
    for (const f of findings) {
      console.log(`${f.file}:${f.line}:${f.col}: ${f.severity}: ${f.message} [${f.rule}]`)
    }
    const errors = findings.filter((x) => x.severity === "error").length
    console.log(`shadless-check: ${files.length} files, ${errors} errors, ${findings.length - errors} warnings`)
  }
  const hasErrors = findings.some((x) => x.severity === "error")
  return hasErrors || (strict && findings.length > 0) ? 1 : 0
}

// Direct invocation (also through npm's .bin symlink, hence the realpath).
const invoked = process.argv[1]
  ? import.meta.url === pathToFileURL(realOrGiven(process.argv[1])).href
  : false
if (invoked) {
  process.exit(main(process.argv.slice(2)))
}
