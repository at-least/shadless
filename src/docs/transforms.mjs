// transforms.mjs — single source of truth for mdx→site content transforms.
//
// Two consumers MUST agree on WHERE each transform touches the raw mdx:
//   - tools/docs-build.mjs (builder): locates the span and REPLACES it
//   - tools/docs-fidelity-lib.mjs (gate): locates the same span and DROPS
//     it before extracting mdx facts
// Before this module they were hand-mirrored implementations and could
// silently diverge (the gate would then check the wrong invariant).
// Both sides now call the same locate*/shadow functions below — a
// divergence is impossible by construction.
//
// TEXT_ADJUSTMENTS carries declared product decisions (2026-08-26):
// prose rewrites where upstream claims something false about shadless.
// The builder applies the substitution; docs-fidelity asserts the
// retired phrasing never survives into a built page.

// Fence shadow: blank out fenced regions (newlines and offsets
// preserved) so span searches see only prose/markup. Line-based (3+
// backtick fences, info strings, unclosed fences blank to EOF) —
// strictly more robust than the regex-pair scan it replaced.
/** @param {string} text @returns {string} */
export const fenceShadow = (text) => {
  const lines = text.split("\n")
  /** @type {number | null} */
  let open = null
  return lines.map((line) => {
    if (open === null) {
      if (/^`{3,}/.test(line)) { open = (line.match(/^`+/) || [""])[0].length; return " ".repeat(line.length) }
      return line
    }
    if ((line.match(/^`{3,}/) || [""])[0].length >= open) { open = null; return " ".repeat(line.length) }
    return " ".repeat(line.length)
  }).join("\n")
}

// All <TabsContent value="manual"> blocks outside fences.
// Returns [{start, end}] with end just past the matching
// </TabsContent>. Malformed (no close) stops the scan — the builder
// throws on anything other than exactly one well-formed span.
/**
 * @typedef {{ start: number, end: number }} Span
 */
/**
 * @param {string} shadow
 * @returns {Span[]}
 */
export function locateManualTabSpans(shadow) {
  const spans = []
  const re = /<TabsContent value=("manual"|manual)>/g
  let m
  while ((m = re.exec(shadow))) {
    const close = shadow.indexOf("</TabsContent>", m.index)
    if (close === -1) break
    const end = close + "</TabsContent>".length
    spans.push({ start: m.index, end })
    re.lastIndex = end
  }
  return spans
}

// The `## Installation` … `## Usage` span in utils guides (outside
// fences). end = the '#' of "## Usage" (guideTransform replaces up to
// it, keeping the Usage heading). Null when absent/malformed.
/** @param {string} shadow @returns {Span | null} */
export function locateInstallSection(shadow) {
  const open = /^## Installation$/m.exec(shadow)
  if (!open) return null
  const next = /^## Usage$/m.exec(shadow)
  if (!next || next.index <= open.index) return null
  return { start: open.index, end: next.index }
}

// The full <CodeTabs> install block on component pages (CLI tab
// `npx shadcn@latest add …` + manual tab). Exactly one per mirrored
// component mdx. The builder replaces it wholesale with the shadless
// copy-files steps — shadless has no CLI, so a Command tab would be a lie
// — and the gate drops the same span from mdx facts (replacement content
// adds no headings; extra html fences pass the one-directional fence
// check).
/** @param {string} shadow @returns {Span[]} */
export function locateCodeTabsSpans(shadow) {
  const spans = []
  const re = /<CodeTabs>/g
  let m
  while ((m = re.exec(shadow))) {
    const close = shadow.indexOf("</CodeTabs>", m.index)
    if (close === -1) break
    const end = close + "</CodeTabs>".length
    spans.push({ start: m.index, end })
    re.lastIndex = end
  }
  return spans
}

// The shadcn-CLI migration section of the rtl guide (## Migrating existing
// components … </Steps>, the file's last section). CLI-only instructions
// (migrate command, base-component link list, add direction,
// DirectionProvider wiring) with no shadless equivalent; the builder
// replaces it with vanilla prose, the gate drops it from mdx facts.
/** @param {string} shadow @returns {Span | null} */
export function locateRtlMigrateSpan(shadow) {
  const open = /^## Migrating existing components$/m.exec(shadow)
  if (!open) return null
  const close = shadow.indexOf("</Steps>", open.index)
  if (close === -1) return null
  return { start: open.index, end: close + "</Steps>".length }
}

// The `## Usage` … next `## ` span on component pages. Upstream fills it
// with React import + JSX fences; the builder replaces it with the shadless
// usage story (copy-markup + the JSX-prop → data-attribute API table).
// end = the '#' of the next section heading. Null when absent/malformed.
/** @param {string} shadow @returns {Span | null} */
export function locateUsageSpan(shadow) {
  const open = /^## Usage$/m.exec(shadow)
  if (!open) return null
  const next = /^## (?!Usage$)/m.exec(shadow.slice(open.index + 9))
  if (!next) return null
  return { start: open.index, end: open.index + 9 + next.index }
}

// The `## Composition` … next `## ` span on component pages. Upstream
// shows the component tree (PascalCase names) + React composition
// examples; the builder keeps ONLY the tree, translated to slot names.
/** @param {string} shadow @returns {Span | null} */
export function locateCompositionSpan(shadow) {
  const open = /^## Composition$/m.exec(shadow)
  if (!open) return null
  const next = /^## (?!Composition$)/m.exec(shadow.slice(open.index + 15))
  if (!next) return null
  return { start: open.index, end: open.index + 15 + next.index }
}

// Pure React import fences (content is ONLY import statements from
// @/components/ui/*) carry zero information for a no-React product — the
// advanced sections that show them document composition via the JSX that
// FOLLOWS, which stays. Dropped from the built page and, in lockstep, from
// the gate's mdx-side fence facts (single predicate, both sides).
// The tempered (?:(?!```)[\s\S]) keeps an import statement from spanning
// across a closing fence and swallowing whatever sits between two import
// fences (that bug ate a whole CodeTabs block before the locator saw it).
const IMPORT_FENCE = /```tsx[^\n]*\n((?:import(?:(?!```)[\s\S])*?from\s+"[@/][^"]+"[^\n]*\n?)+)```\n?/g
/** @param {string} raw @returns {string} */
export function dropReactImportFences(raw) {
  return raw.replace(IMPORT_FENCE, "")
}

// Mixed fences (import statements + JSX example in one block): strip the
// import statements and shift the shiki line-number highlights ({1,4-6})
// past them, so the remaining JSX keeps correct highlight positions. Same
// lockstep rule — builder and gate both apply this.
/** @param {string} raw @returns {string} */
export function stripImportsFromMixedFences(raw) {
  return raw.replace(/```tsx([^\n]*)\n([\s\S]*?)```/g, (whole, meta, body) => {
    const lines = body.split("\n")
    // count leading import statements (each ends on its `from "…"` line)
    // plus the blank separator line that follows
    let removed = 0
    let i = 0
    while (i < lines.length) {
      const l = lines[i]
      if (l.startsWith("import")) {
        removed++; i++
        while (i < lines.length && !/from\s+"[@/][^"]+"\s*;?\s*$/.test(lines[i])) { removed++; i++ }
        if (i < lines.length) { removed++; i++ }
      } else if (l.trim() === "" && removed > 0) { removed++; i++ }
      else break
    }
    if (removed === 0) return whole
    const kept = lines.slice(removed - (lines[removed - 1]?.trim() === "" ? 1 : 0))
    if (kept.join("").trim() === "") return "" // fence was imports-only — drop it
    // renumber {…} highlight refs: drop refs into the removed range, shift the rest
    const metaOut = meta.replace(/\{([0-9,\s-]+)\}/, (/** @type {string} */ m0, /** @type {string} */ list) => {
      const shifted = list.split(",").map((/** @type {string} */ s) => s.trim()).filter(Boolean).map((/** @type {string} */ part) => {
        /** @type {number[]} */
        const range = part.split("-").map(Number)
        if (range.some((n) => Number.isNaN(n))) return part
        if ((/** @type {number} */ (range[0])) <= removed) return null
        return range.map((n) => n - removed).join("-")
      }).filter(Boolean)
      return shifted.length ? `{${shifted.join(",")}}` : ""
    })
    return "```tsx" + metaOut.replace(/\s+$/, "") + "\n" + kept.join("\n") + "```"
  })
}

// Declared product decisions: prose rewrites where upstream mdx claims
// React-library specifics that are false for the no-React product.
// The external "see the Radix UI documentation" LINKS stay — they
// document the behavior contract our kernel mirrors.
export const TEXT_ADJUSTMENTS = [
  {
    id: "button-pointer-cli-prose",
    files: ["button.mdx"],
    note: "shadless has no CLI — the init --pointer sentence is a shadcn-CLI instruction",
    ops: [
      {
        find: "You can also enable this during project setup with \`npx shadcn@latest init --pointer\`.",
        replace: "In shadless just keep the CSS rule above — there is no CLI flag to set.",
      },
    ],
  },
  {
    id: "avatar-props-prose",
    files: ["avatar.mdx"],
    note: "shadless Avatar slots are plain HTML elements driven by the runtime — 'accepts all Radix UI props' is false here",
    ops: [
      {
        find: "It accepts all Radix UI Avatar Image props.",
        replace: 'It is a plain `<img data-slot="avatar-image">` — the shadless runtime switches to the fallback from its load state.',
      },
      {
        find: "It accepts all Radix UI Avatar Fallback props.",
        replace: 'It is a plain `<span data-slot="avatar-fallback">` shown by the shadless runtime while the image is loading or failed.',
      },
    ],
  },
]

// Apply the text adjustments declared for a source file (matched by
// basename). Returns the raw text unchanged for unlisted files; throws
// if a declared find string is missing (upstream changed the prose —
// the adjustment must be re-anchored, never silently skipped).
/** @param {string} basename @param {string} raw @returns {string} */
export function applyTextAdjustments(basename, raw) {
  let out = raw
  for (const adj of TEXT_ADJUSTMENTS) {
    if (!adj.files.includes(basename)) continue
    for (const op of adj.ops) {
      const i = fenceShadow(out).indexOf(op.find)
      if (i === -1) {
        throw new Error(`text adjustment ${adj.id}: find string not present in ${basename} — re-anchor against the new upstream prose`)
      }
      out = out.slice(0, i) + op.replace + out.slice(i + op.find.length)
    }
  }
  return out
}
