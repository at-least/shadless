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
//   error   unknown-value       data-<axis>="<v>" where <axis> is a cva axis of
//                               the slot and <v> is not one of its values. The
//                               empty value is accepted exactly when the axis
//                               has a default (the stylesheet's default twin
//                               selects [data-axis=""]). Axes NOT in the slot's
//                               vocabulary are skipped: card data-size,
//                               select-trigger data-size, field-legend and
//                               dropdown-menu-item data-variant exist only as
//                               in-@apply hooks whose full value set no table
//                               names — checking them would produce false
//                               errors, so the checker covers exactly the cva
//                               axes listed in vocabulary.json.
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
// Tokenizer scope: svg/math foreign content is tracked (no raw text there,
// <![CDATA[ sections run to ]]>), with a known limitation: HTML integration
// points inside foreign content (svg foreignObject/desc/title, math
// annotation-xml/mi/mo/mn/ms/mtext — whose contents a browser parses as
// HTML again) are not modeled: their content is scanned as foreign markup.
// That finds the elements a browser builds there, but when a raw-text
// element inside an integration point never closes, a browser's raw text
// swallows the rest of the document while the checker keeps scanning — such
// shapes can produce extra findings, not just missed tokenizer quirks.
//
// Tree construction is not modeled either: every start tag the tokenizer
// emits is checked. A browser's tree builder drops some of them (a <div>
// inside <select>, a <td>/<tr> outside a table), and an HTML start tag such
// as <div>/<p>/<span> inside svg/math breaks out of foreign content, which
// the svg/math depth does not follow — raw text after such a breakout is
// scanned as markup and a later <![CDATA[ runs to ]]>. These shapes are
// invalid authoring (parse5-measured, absent from the shipped corpus); they
// can produce findings on elements no browser builds, or miss one after a
// breakout.
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
 * documented narrowing: semicolon-less NAMED references (`&amp` without
 * `;`) stay literal, where a browser decodes the legacy names in attribute
 * values — the full legacy follow-character table is not worth carrying for
 * token matching. NUL, surrogates and references above U+10FFFF become
 * U+FFFD, never a crash.
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
 * nesting depth is tracked; HTML integration points inside it are not
 * modeled (see the header note).
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
  // svg/math nesting depth: > 0 means foreign content (no raw text, CDATA
  // sections allowed)
  let foreign = 0
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
    if (foreign > 0 && text.startsWith("<![CDATA[", lt)) {
      // a CDATA section in foreign content is one token: everything to the
      // closing ]]>, or to EOF when it never closes (parse5-measured)
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
      if (lower === "svg" || lower === "math") {
        foreign = Math.max(0, foreign - 1)
      }
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
    if ((lower === "svg" || lower === "math") && !selfClosed) {
      // a self-closed <svg/> carries no content, so it is not foreign depth
      foreign++
    }

    // rule: duplicate-attribute (case-insensitive; HTML keeps the first)
    /** @type {Map<string, Attr>} */
    const byLower = new Map()
    for (const a of attrs) {
      const seen = byLower.get(a.lower)
      if (seen) {
        push(a.namePos, "error", "duplicate-attribute",
          `duplicate attribute "${a.name}" — HTML parsers keep only the first one (line ${at(seen.namePos).line}, col ${at(seen.namePos).col})`)
      } else {
        byLower.set(a.lower, a)
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

    if (foreign === 0 && RAW_TEXT_TAGS.has(lower)) {
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
    if (foreign === 0 && lower === "plaintext") {
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
