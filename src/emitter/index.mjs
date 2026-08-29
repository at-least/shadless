// T5 emitter: IR (tier=static) → semantic usage HTML + slot-keyed CSS.
// Gates: static file count, no non-anchor class= in HTML, jsdom slot-tree vs
// IR (exact tags + nesting edges), no literal PascalCase tags, no escaped
// markup artifacts. Playwright smoke runs separately (src/emitter/smoke.mjs).
//
// Pure helpers are exported for unit tests (tools/unit/emitter.mjs); the
// pipeline + gates run under the main guard only.
import { readFileSync, writeFileSync, mkdirSync, readdirSync } from "node:fs"
import { join } from "node:path"
import { pathToFileURL } from "node:url"
import { JSDOM } from "jsdom"
import { componentCss, wrapComponentCss, MARKER, splitMarkers } from "./css.mjs"
import { twMerge } from "tailwind-merge"
import { SKIN_ALLOWLIST } from "./skin.mjs"
import { NAT, VOID, normalizeTag } from "../tags.mjs"
import { THEME_PREPAINT_SCRIPT, SHADLESS_CSS_FIXES } from "../docs/theme-prepaint.mjs"


const IRDIR = "src/registry/ir"

// resolve a fn's element tree from the flat walk-order elements + sketches.
// Every tag (root AND children) goes through normalizeTag — the old code
// coerced every non-native root to <button> and emitted children like
// <ChevronLeftIcon> literally. anchors maps "fn#elementIndex" → class token
// for slotless-with-classes elements (componentCss assigns them).
export function buildTree(ir, fn, claimed = new Set(), anchors = new Map(), anchorMarkers = new Map()) {
  const byKey = fn.elements.map((e, i) => ({ e, i }))
  const resolve = (el, i) => {
    const kids = []
    for (const sk of el.children || []) {
      const m = /^<([^ >]+)(?: slot=([^ >]+))?/.exec(sk)
      if (m) {
        // raw-tag match on both sides (sketch carries the raw JSX name)
        const hit = byKey.find((o) => !claimed.has(o.i) &&
          (m[2] ? o.e.slot === m[2] && o.e.tag === m[1] : o.e.tag === m[1]))
        if (hit) { claimed.add(hit.i); kids.push(resolve(hit.e, hit.i)); continue }
        // unresolvable sketch: icon → svg; native → bare; else skip
        const tag = normalizeTag(m[1], ir.tagHints) ?? m[1]
        if (NAT.has(tag)) kids.push({ tag, slot: m[2] || null, anchor: null, anchorM: null, kids: [] })
      }
      // text/{children}/OPT?/expr → nothing structural
    }
    const tag = normalizeTag(el.tag, ir.tagHints)
    if (tag == null)
      throw new Error(`[${ir.name}] unresolvable tag in ${fn.fn}: ${el.tag}`)
    return {
      tag, slot: el.slot,
      anchor: anchors.get(`${fn.fn}#${i}`) ?? null,
      // allowlist skin markers for slotless elements — stay on class=,
      // never in @apply (upstream: inert markers, zero site CSS)
      anchorM: anchorMarkers.get(`${fn.fn}#${i}`) ?? null,
      kids,
    }
  }
  const root = fn.elements[0]
  claimed.add(0)
  const tree = resolve(root, 0)
  return tree
}

// Render an element tree. Semantics of the default-content args:
//   - defaultInner (the fn root's DEFAULT_CONTENT inner): REPLACES the root's
//     content entirely — hand-authored composition demos (alert, card,
//     pagination…) must win over the sketch tree. The old code only applied
//     it to leaf roots, silently dropping table/pagination compositions.
//   - defaultBySlot (slot → content): fills EMPTY leaves (native-select
//     options into the <select>).
export function renderTree(node, markers = {}, defaultInner = "", defaultBySlot = {}, isRoot = false) {
  const classes = []
  if (node.slot && markers[node.slot]?.length) classes.push(...new Set(markers[node.slot]))
  if (node.anchor) classes.push(node.anchor)
  if (node.anchorM?.length) classes.push(...node.anchorM)
  const cls = classes.length ? ` class="${classes.join(" ")}"` : ""
  const slot = node.slot ? ` data-slot="${node.slot}"` : ""
  const open = `<${node.tag}${slot}${cls}>`
  // Void elements: no closing tag, no children (would be invalid HTML)
  if (VOID.has(node.tag)) return open
  let inner
  if (isRoot && defaultInner) inner = defaultInner
  else if (node.kids.length)
    inner = node.kids.map((k) => renderTree(k, markers, "", defaultBySlot)).join("")
  else inner = defaultBySlot[node.slot] ?? ""
  return `${open}${inner}</${node.tag}>`
}

// stray table-parts get dropped by HTML parsers at body level — wrap ancestors
const TABLE_WRAP = { thead: "table", tbody: "table", tfoot: "table",
  caption: "table", colgroup: "table", tr: "table", th: "table", td: "table" }
export function renderFn(tree, markers = {}, defaultInner = "", defaultBySlot = {}) {
  let html = renderTree(tree, markers, defaultInner, defaultBySlot, true)
  let tag = tree.tag
  while (TABLE_WRAP[tag]) { html = `<${TABLE_WRAP[tag]}>${html}</${TABLE_WRAP[tag]}>`; tag = TABLE_WRAP[tag] }
  return html
}

// ---- DEFAULT_CONTENT ----------------------------------------------------------
// The static 23 IRs only carry slot structure, not example content — this map
// injects a working default per (component, fn) so the iframe previews show a
// sensible example:
//   - string  → text node (auto-escaped) — REPLACES the fn root's content
//   - { html } → pre-escaped inner HTML chunk (compositions) — same replace
//   - { attrs } → additional attributes merged into the root open tag
//   - { children: { slot: html } } → fills EMPTY leaf slots (select options)
//   - null     → leave empty
// Keys are validated at emit time: component names must exist in the static
// set and fn names must be exported fns of that IR (a stale key previously
// survived silently, e.g. the old AlertAction entry).
//
// Inline styles reference theme tokens as `var(--x)`, NEVER `hsl(var(--x))`.
// The theme ships oklch() values (dist/globals.css `--muted: oklch(...)`), so
// `hsl(oklch(...))` is an invalid color and the whole declaration is dropped
// by the parser — 51 such declarations were live in 12 dist/components pages,
// styling nothing. Nothing gates inline style, so this is the only place the
// rule is written down. Token/opacity uses color-mix(in oklab, …), tailwind
// v4's own spelling, not the `/alpha` slash form (also hsl-only).
export const DEFAULT_CONTENT = {
  badge: { Badge: "Badge" },
  button: { Button: "Button" },
  input: { Input: { attrs: { placeholder: "Type here…" } } },
  textarea: { Textarea: { attrs: { placeholder: "Type here…" } } },
  skeleton: { Skeleton: { attrs: { style: "width:250px;height:1rem;display:block" } } },
  spinner: {
    // No class= — Wave A gate (no class= in HTML except markers/anchors)
    // rejects it; svg defaults render fine and the `animate-spin` comes from
    // the host's own runtime class. The path is the canonical Loader2.
    Spinner: { html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1rem;height:1rem" aria-hidden="true"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>' }
  },
  alert: {
    // role="alert" mirrors shadcn's accessibility contract; the slot-CSS in
    // out.css drives all layout and typography.
    Alert: { html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg><div data-slot="alert-title">Heads up!</div><div data-slot="alert-description">You can add components to your app using the cli.</div>', attrs: { role: "alert" } },
    AlertTitle: "Heads up!",
    AlertDescription: "You can add components to your app using the cli.",
  },
  attachment: {
    Attachment: { html: '<div data-slot="attachment-media" style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem" aria-hidden="true"><path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path></svg></div><div data-slot="attachment-content" style="display:flex;flex-direction:column;gap:0.125rem"><span data-slot="attachment-title" style="font-weight:500">document.pdf</span><span data-slot="attachment-description" style="font-size:0.75rem;color:var(--muted-foreground)">2.4 MB</span></div>' },
    AttachmentMedia: { html: '<div style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem" aria-hidden="true"><path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path></svg></div>' },
    AttachmentContent: { html: '<span data-slot="attachment-title" style="font-weight:500">document.pdf</span><span data-slot="attachment-description" style="font-size:0.75rem;color:var(--muted-foreground)">2.4 MB</span>' },
    AttachmentTitle: "document.pdf",
    AttachmentDescription: "2.4 MB",
    AttachmentActions: null,
    AttachmentTrigger: null,
    AttachmentGroup: null,
  },
  breadcrumb: {
    Breadcrumb: { html: '<ol data-slot="breadcrumb-list"><li data-slot="breadcrumb-item"><a data-slot="breadcrumb-link" href="#" style="transition:color;hover:{color:var(--foreground)}">Home</a></li><li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" style="display:inline-flex;align-items:center;color:var(--muted-foreground)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:0.875rem;height:0.875rem"><path d="m9 18 6-6-6-6"></path></svg></li><li data-slot="breadcrumb-item"><a data-slot="breadcrumb-link" href="#">Components</a></li><li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" style="display:inline-flex;align-items:center;color:var(--muted-foreground)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:0.875rem;height:0.875rem"><path d="m9 18 6-6-6-6"></path></svg></li><li data-slot="breadcrumb-item"><span data-slot="breadcrumb-page" style="font-weight:normal;color:var(--foreground)">Breadcrumb</span></li></ol>' },
    BreadcrumbList: { html: '<li data-slot="breadcrumb-item"><a data-slot="breadcrumb-link" href="#">Home</a></li><li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true">/</li><li data-slot="breadcrumb-item"><span data-slot="breadcrumb-page">Current</span></li>' },
    BreadcrumbItem: { html: '<a data-slot="breadcrumb-link" href="#">Home</a>' },
    BreadcrumbLink: "Home",
    BreadcrumbPage: "Current",
    BreadcrumbSeparator: "/",
    BreadcrumbEllipsis: "…",
  },
  bubble: {
    BubbleGroup: null,
    Bubble: { html: '<div data-slot="bubble-content" style="display:inline-block;border-radius:1rem;padding:0.5rem 0.75rem;background:var(--muted)">Did you remove the stale route?</div>' },
    BubbleContent: "Did you remove the stale route?",
    BubbleReactions: null,
  },
  card: {
    Card: { html: '<div data-slot="card-header"><div data-slot="card-title" style="font-weight:600">Create project</div><div data-slot="card-description" style="font-size:0.875rem;color:var(--muted-foreground)">Deploy your new project in one-click.</div></div><div data-slot="card-content" style="margin-top:1rem"><p>Set up your project with our intuitive wizard.</p></div><div data-slot="card-footer" style="display:flex;justify-content:space-between;align-items:center;margin-top:1rem"><button>Cancel</button><button style="background:var(--primary);color:var(--primary-foreground);padding:0.375rem 0.75rem;border-radius:0.375rem">Deploy</button></div>' },
    CardHeader: { html: '<div data-slot="card-title" style="font-weight:600">Title</div><div data-slot="card-description" style="font-size:0.875rem;color:var(--muted-foreground)">Description</div>' },
    CardTitle: "Title",
    CardDescription: "Description",
    CardAction: null,
    CardContent: "Content",
    CardFooter: "Footer",
  },
  direction: {
    // provider-only — no visible content possible; leaving empty is correct
    DirectionProvider: null,
  },
  empty: {
    Empty: { html: '<div data-slot="empty-header"><div data-slot="empty-icon" style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted);margin-bottom:0.75rem"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem" aria-hidden="true"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></svg></div><div data-slot="empty-title" style="font-weight:600">No results</div><div data-slot="empty-description" style="font-size:0.875rem;color:var(--muted-foreground)">Try adjusting your search or filters.</div></div><div data-slot="empty-content" style="margin-top:0.75rem"><button style="background:var(--primary);color:var(--primary-foreground);padding:0.375rem 0.75rem;border-radius:0.375rem">Clear filters</button></div>' },
    EmptyHeader: { html: '<div data-slot="empty-icon" style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted);margin-bottom:0.5rem">⌕</div><div data-slot="empty-title" style="font-weight:600">No results</div><div data-slot="empty-description" style="font-size:0.875rem;color:var(--muted-foreground)">Adjust your search.</div>' },
    EmptyMedia: "⌕",
    EmptyTitle: "No results",
    EmptyDescription: "Try adjusting your search or filters.",
    EmptyContent: "Content",
  },
  "input-group": {
    InputGroup: { html: '<div data-slot="input-group-addon" style="display:flex;align-items:center;padding:0 0.75rem;border:1px solid var(--input);border-right:0;border-radius:0.375rem 0 0 0.375rem;background:var(--muted);color:var(--muted-foreground)">@</div><input data-slot="input-group-control" placeholder="Username" style="border-radius:0;border-left:0;border-right:0"><div data-slot="input-group-addon" style="display:flex;align-items:center;padding:0 0.75rem;border:1px solid var(--input);border-left:0;border-radius:0 0.375rem 0.375rem 0;background:var(--muted);color:var(--muted-foreground)">@example.com</div>' },
    InputGroupAddon: "@",
    InputGroupButton: "Button",
    InputGroupText: "Text",
    InputGroupInput: { attrs: { placeholder: "Type here…" } },
    InputGroupTextarea: { attrs: { placeholder: "Type here…" } },
  },
  item: {
    ItemGroup: null,
    ItemSeparator: null,
    Item: { html: '<div data-slot="item-media" style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem" aria-hidden="true"><circle cx="12" cy="12" r="10"></circle></svg></div><div data-slot="item-content" style="display:flex;flex-direction:column;gap:0.125rem"><div data-slot="item-title" style="font-weight:500">Item title</div><p data-slot="item-description" style="font-size:0.875rem;color:var(--muted-foreground);margin:0">Item description.</p></div><div data-slot="item-actions" style="display:flex;align-items:center;gap:0.25rem"><button style="padding:0.25rem 0.5rem;border-radius:0.375rem;font-size:0.875rem">Edit</button></div>' },
    ItemMedia: { html: '<div style="display:flex;align-items:center;justify-content:center;width:2.5rem;height:2.5rem;border-radius:0.5rem;background:var(--muted)"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1.25rem;height:1.25rem"><circle cx="12" cy="12" r="10"></circle></svg></div>' },
    ItemContent: { html: '<div data-slot="item-title" style="font-weight:500">Title</div><p data-slot="item-description" style="font-size:0.875rem;color:var(--muted-foreground);margin:0">Description</p>' },
    ItemTitle: "Title",
    ItemDescription: "Description",
    ItemActions: null,
    ItemHeader: "Header",
    ItemFooter: "Footer",
  },
  kbd: {
    Kbd: { html: '⌘<span style="margin:0 0.25rem">+</span>K' },
    KbdGroup: { html: '<kbd data-slot="kbd">⌘</kbd><kbd data-slot="kbd">⇧</kbd><kbd data-slot="kbd">K</kbd>' },
  },
  marker: {
    Marker: { html: '<span data-slot="marker-icon" style="display:inline-flex;align-items:center;justify-content:center;width:1.25rem;height:1.25rem;border-radius:9999px;background:var(--destructive);color:white;font-size:0.75rem">1</span><span data-slot="marker-content">New</span>' },
    MarkerIcon: "1",
    MarkerContent: "New",
  },
  "native-select": {
    // Options are injected into the <select> (leaf fill via children map);
    // the icon child resolves to a real <svg> via tagHints.
    NativeSelect: { children: { "native-select": '<option>Choose a fruit</option><option>Apple</option><option>Banana</option><option>Blueberry</option>' } },
    NativeSelectOption: "Option",
    NativeSelectOptGroup: null,
  },
  pagination: {
    // pagination-link gets real slot CSS from the converter's cross-file cva
    // resolution (buttonVariants ghost + size-9, [data-active=true] → outline).
    Pagination: { html: '<ul data-slot="pagination-content"><li data-slot="pagination-item"><a data-slot="pagination-link" href="#" aria-label="Previous">‹</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#">1</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#" aria-current="page" data-active="true">2</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#">3</a></li><li data-slot="pagination-item"><span data-slot="pagination-ellipsis">…</span></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#" aria-label="Next">›</a></li></ul>' },
    PaginationContent: { html: '<li data-slot="pagination-item"><a data-slot="pagination-link" href="#" aria-current="page" data-active="true">1</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#">2</a></li><li data-slot="pagination-item"><a data-slot="pagination-link" href="#">3</a></li>' },
    PaginationItem: { html: '<a data-slot="pagination-link" href="#">1</a>' },
    PaginationLink: "1",
    // root is already an <a> (PaginationLink root, slotless — upstream sets
    // no data-slot on Previous/Next); inner content only, no nested link
    PaginationPrevious: { html: "‹", attrs: { href: "#", "aria-label": "Go to the previous page" } },
    PaginationNext: { html: "›", attrs: { href: "#", "aria-label": "Go to the next page" } },
    PaginationEllipsis: "…",
  },
  table: {
    Table: { html: '<table data-slot="table" style="width:100%;caption-side:bottom;font-size:0.875rem"><thead data-slot="table-header"><tr data-slot="table-row"><th data-slot="table-head" style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)">Name</th><th data-slot="table-head" style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)">Status</th><th data-slot="table-head" style="text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)">Amount</th></tr></thead><tbody data-slot="table-body"><tr data-slot="table-row"><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Alice</td><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Active</td><td data-slot="table-cell" style="text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)">$250</td></tr><tr data-slot="table-row"><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Bob</td><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Inactive</td><td data-slot="table-cell" style="text-align:right;padding:0.5rem;border-bottom:1px solid var(--border)">$150</td></tr></tbody></table>' },
    TableHeader: { html: '<tr data-slot="table-row"><th data-slot="table-head" style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--border)">Header</th></tr>' },
    TableBody: { html: '<tr data-slot="table-row"><td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Cell</td></tr>' },
    TableFooter: { html: '<tr data-slot="table-row"><td data-slot="table-cell" style="padding:0.5rem">Footer</td></tr>' },
    TableRow: { html: '<td data-slot="table-cell" style="padding:0.5rem;border-bottom:1px solid var(--border)">Cell</td>' },
    TableHead: "Header",
    TableCell: "Cell",
    TableCaption: "Caption",
  },
  "button-group": {
    // Plain <button>s (no data-slot="button") — the IR has no Button slot pair
    // inside ButtonGroup, so any data-slot would trip the jsdom slot-tree gate.
    ButtonGroup: { html: '<button style="display:inline-flex;align-items:center;border:1px solid var(--border);border-radius:0.375rem;padding:0.25rem 0.75rem;font-size:0.875rem;background:transparent">Text</button><div data-slot="button-group-separator" style="display:inline-block;width:1px;height:1.25rem;background:var(--input)"></div><button style="display:inline-flex;align-items:center;justify-content:center;width:2.25rem;height:2.25rem;border:1px solid var(--border);border-radius:0.375rem;background:transparent" aria-label="Add">+</button>' },
    ButtonGroupText: "Text",
    ButtonGroupSeparator: null,
  },
  message: {
    MessageGroup: null,
    Message: { html: '<div data-slot="message-avatar" style="display:flex;align-items:flex-start;gap:0.75rem"><span style="display:inline-flex;align-items:center;justify-content:center;width:2rem;height:2rem;border-radius:9999px;background:var(--muted);font-size:0.875rem;font-weight:500">CN</span></div><div data-slot="message-content" style="display:flex;flex-direction:column;gap:0.25rem"><div data-slot="message-header" style="font-size:0.875rem;font-weight:600">Header</div><div data-slot="message-footer" style="font-size:0.75rem;color:var(--muted-foreground)">Footer</div></div>' },
    MessageAvatar: { html: '<span style="display:inline-flex;align-items:center;justify-content:center;width:2rem;height:2rem;border-radius:9999px;background:var(--muted);font-size:0.875rem;font-weight:500">CN</span>' },
    MessageContent: { html: '<div data-slot="message-header" style="font-weight:600">Header</div><div data-slot="message-footer" style="font-size:0.75rem;color:var(--muted-foreground)">Footer</div>' },
    MessageHeader: "Header",
    MessageFooter: "Footer",
  },
  "message-scroller": {
    MessageScrollerProvider: null,
    MessageScroller: { html: '<div data-slot="message-scroller-viewport" style="height:160px;overflow:hidden;border:1px solid var(--border);border-radius:0.5rem;padding:0.75rem;background:color-mix(in oklab, var(--muted) 30%, transparent)"><div data-slot="message-scroller-content"><div data-slot="message-scroller-item" style="margin-bottom:0.5rem">Top message</div><div data-slot="message-scroller-item" style="margin-bottom:0.5rem;margin-top:3rem">Middle message</div><div data-slot="message-scroller-item" style="margin-top:6rem">Bottom message</div></div></div>' },
    MessageScrollerViewport: { html: '<div style="padding:0.75rem">Scrollable content</div>' },
    MessageScrollerContent: { html: '<div style="padding:0.75rem">Item content</div>' },
    MessageScrollerItem: { html: '<div style="padding:0.5rem;border:1px solid var(--border);border-radius:0.375rem">Item</div>' },
    MessageScrollerButton: { html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:1rem;height:1rem"><path d="M12 5v14M5 12l7 7 7-7"></path></svg>' },
  },
}

export const escHtml = (s) => String(s)
  .replace(/&/g, "&amp;").replace(/</g, "&lt;")
  .replace(/>/g, "&gt;").replace(/"/g, "&quot;")

export function resolveDefault(ir, fn) {
  const entry = DEFAULT_CONTENT[ir.name]?.[fn.fn]
  if (entry === undefined || entry === null) return null
  if (typeof entry === "string") return { inner: escHtml(entry) }
  // Compose html + attrs + children; previously picked one and dropped the
  // others, so adding role= to an entry that already had html was a no-op.
  const out = {}
  if (entry.html) out.inner = entry.html
  if (entry.attrs) out.attrs = entry.attrs
  if (entry.children) out.children = entry.children
  return Object.keys(out).length ? out : null
}

// Apply extra attrs (e.g. placeholder, style) to the root open tag of the
// rendered fn output. Quote-aware scan for the end of the first open tag —
// a plain [^>]* break on attribute values containing ">" (aria-label="→").
export function mergeRootAttrs(html, attrs) {
  const extra = Object.entries(attrs)
    .map(([k, v]) => `${k}="${escHtml(v)}"`)
    .join(" ")
  if (!extra) return html
  const m = /^<([a-zA-Z][\w-]*)/.exec(html)
  if (!m) return html
  let i = m[0].length, quote = null
  while (i < html.length) {
    const ch = html[i]
    if (quote) { if (ch === quote) quote = null }
    else if (ch === '"' || ch === "'") quote = ch
    else if (ch === ">") break
    i++
  }
  return `${html.slice(0, i)} ${extra}${html.slice(i)}`
}

// validate DEFAULT_CONTENT keys against the actual IRs (stale keys previously
// survived silently)
export function validateDefaultContent(statics) {
  const errs = []
  const names = new Set(statics.map((ir) => ir.name))
  for (const [comp, fns] of Object.entries(DEFAULT_CONTENT)) {
    if (!names.has(comp)) { errs.push(`unknown component key: ${comp}`); continue }
    const ir = statics.find((s) => s.name === comp)
    const exported = new Set(ir.components.filter((c) => c.export).map((c) => c.fn))
    for (const fn of Object.keys(fns))
      if (!exported.has(fn)) errs.push(`[${comp}] unknown fn key: ${fn}`)
  }
  return errs
}

// ---------- emit --------------------------------------------------------------
function main() {
  // emit OWNS the static-tier pages and dist/shadless.css, nothing else in
  // dist/. It used to wipe dist/components and overwrite globals.css /
  // out.css / demo-index.html with static-only versions that the demo chain
  // (full tier) later replaced — so every medium-tier run left a committed
  // tree broken until the next full build, and twice a partial out.css got
  // committed. Its own stylesheet/index now go to build/emit/ (emit-smoke
  // reads them there); dist/components is written into, never wiped.
  mkdirSync("dist/components", { recursive: true })
  mkdirSync("build/emit", { recursive: true })

  const files = readdirSync(IRDIR).filter((f) => f.endsWith(".json")).sort()
  const statics = files.map((f) => JSON.parse(readFileSync(join(IRDIR, f), "utf8")))
    .filter((ir) => ir.tier === "static")
  const EXPECTED_STATIC = JSON.parse(readFileSync("src/registry/tiers.json", "utf8"))
  const wantStatic = Object.values(EXPECTED_STATIC).filter((t) => t.tier === "static").length
  let fail = false
  if (statics.length !== wantStatic) {
    console.error(`FAIL expected ${wantStatic} static (from tiers.json), got ${statics.length}`)
    process.exit(1)
  }
  for (const e of validateDefaultContent(statics)) { console.error(`FAIL defaults: ${e}`); fail = true }

  const cssParts = []
  const allAnchors = new Set()
  const treesByIr = new Map() // for the jsdom nesting gate
  let totalSlots = 0
  for (const ir of statics) {
    const fns = ir.components.filter((c) => c.export)
    let rules, markers, anchors, anchorMarkers, unlayered
    try { ({ rules, markers, anchors, anchorMarkers, unlayered } = componentCss(ir)) }
    catch (e) { console.error(`FAIL css[${ir.name}]: ${e.message}`); fail = true; continue }
    for (const t of anchors.values()) allAnchors.add(t)
    const trees = []
    const body = fns.map((c) => {
      try {
        const tree = buildTree(ir, c, new Set(), anchors, anchorMarkers)
        trees.push(tree)
        const def = resolveDefault(ir, c)
        const inner = def?.inner ?? ""
        const children = def?.children ?? {}
        let html = renderFn(tree, markers, inner, children)
        if (def?.attrs) html = mergeRootAttrs(html, def.attrs)
        return html
      } catch (e) { console.error(`FAIL emit[${ir.name}.${c.fn}]: ${e.message}`); fail = true; return "" }
    }).join("\n")
    treesByIr.set(ir.name, { ir, trees })
    const html = `<!doctype html>
<html><head><meta charset="utf-8"><title>shadless ${ir.name}</title>
<link rel="stylesheet" href="../out.css">${THEME_PREPAINT_SCRIPT}</head>
<body>
${body}
</body></html>`
    writeFileSync(`dist/components/${ir.name}.html`, html)
    cssParts.push(wrapComponentCss(ir.name, { rules, unlayered }))
    totalSlots += (html.match(/data-slot="/g) || []).length
  }
  writeFileSync("dist/shadless.css", cssParts.join("\n\n"))

  // CSS completeness gate: every class TOKEN (markers split off — group/peer
  // tokens live in HTML) must appear in some emitted rule. Token-level, not
  // string-level: same-slot complementary elements (accordion-trigger-icon)
  // legitimately split their tokens across the shared slot rule and
  // per-element anchor rules.
  {
    const css = cssParts.join("\n")
    for (const ir of statics)
      for (const c of ir.components)
        for (const el of c.elements)
          for (const cs of el.classes) {
            // the emitted lists are twMerge'd (css.mjs): a token the element's
            // own merged list drops (rounded-lg under a later rounded-[…]) is
            // absent by design, exactly as in React's class attribute
            const kept = new Set(twMerge(el.classes.map((x) => splitMarkers(x).apply).join(" ")).split(/\s+/))
            const apply = splitMarkers(cs).apply
            const missing = apply.split(/\s+/).filter((t) => t && kept.has(t) && !css.includes(t))
            if (missing.length) {
              console.error(`FAIL css[${ir.name}]: class tokens not in CSS: ${JSON.stringify(missing.slice(0, 6))}…`)
              fail = true
            }
          }
  }

  // globals: h4 globals minus demo source + slot rules + shadless fixes
  // (cn-* is resolved at source — no skin @utility layer, no sentinel)
  const g = readFileSync("probes/h4/globals.css", "utf8").replace('@source "./demo.html";\n', "")
  writeFileSync("build/emit/globals.css", g + "\n" + SHADLESS_CSS_FIXES + "\n" + cssParts.join("\n\n"))

  // static-only demo index (the shipped dist/demo-index.html is tools/demo.mjs's)
  writeFileSync("build/emit/demo-index.html", `<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="out.css"></head><body>
<ul>${statics.map((ir) => `<li><a href="components/${ir.name}.html">${ir.name}</a></li>`).join("")}</ul>
</body></html>`)
  console.log(`emit: ${statics.length} pages, ${totalSlots} slots, shadless.css`)

  // ---------- gate: no class= in generated component HTML --------------------
  // allowed: marker tokens (group/peer) + anchor tokens registered by
  // css.mjs + upstream skin ALLOWLIST markers (cn-rtl-flip etc. ride the
  // markup with zero site CSS — verified against the live payload)
  for (const ir of statics) {
    const html = readFileSync(`dist/components/${ir.name}.html`, "utf8")
    const bad = [...html.matchAll(/class="([^"]*)"/g)]
      .filter((m) => m[1].split(/\s+/).some((t) => t && !MARKER.test(t) && !SKIN_ALLOWLIST.has(t) && !allAnchors.has(t)))
    if (bad.length) {
      console.error(`FAIL [${ir.name}]: non-anchor class= in HTML: ${bad.map((b) => b[1]).join(" | ")}`)
      fail = true
    }
  }

  // ---------- gate: literal PascalCase / ternary tags in HTML ----------------
  for (const ir of statics) {
    const html = readFileSync(`dist/components/${ir.name}.html`, "utf8")
    const bad = [...html.matchAll(/<\/?([A-Z][A-Za-z0-9]*(?:\.[A-Z][A-Za-z0-9]*)?)[\s>]/g)]
    if (bad.length) {
      console.error(`FAIL [${ir.name}]: literal component tag in HTML: ${bad.map((b) => b[0].trim()).join(" | ")}`)
      fail = true
    }
  }

  // ---------- gate: jsdom slot-tree vs IR (exact tags + nesting) -------------
  for (const { ir, trees } of treesByIr.values()) {
    const dom = new JSDOM(readFileSync(`dist/components/${ir.name}.html`, "utf8"))
    const doc = dom.window.document

    // union of (tag, slot) pairs across fn trees — exactly what we render
    const treePairs = new Set(), treeEdges = new Set()
    const collect = (n, parentSlot) => {
      if (n.slot) {
        treePairs.add(`${n.tag}@${n.slot}`)
        if (parentSlot) treeEdges.add(`${parentSlot}>${n.slot}`)
      }
      for (const k of n.kids) collect(k, n.slot ?? parentSlot)
    }
    for (const t of trees) collect(t, null)

    // hand-authored DEFAULT_CONTENT html chunks are sanctioned sources of
    // pairs + nesting (composition demos cross fn boundaries). Wrap the chunk
    // in the fn's own root tag+slot so root-level pairs resolve correctly.
    const defPairs = new Set(), defEdges = new Set()
    for (const c of ir.components.filter((c) => c.export)) {
      const def = resolveDefault(ir, c)
      const inner = def?.inner
      if (!inner) continue
      const rootEl = c.elements[0]
      const rootTag = normalizeTag(rootEl.tag, ir.tagHints) ?? "div"
      const rootSlot = rootEl.slot
      // table-part roots (thead/tbody/tfoot/tr…) need a <table> scope or the
      // HTML parser drops them at body level
      const scope = TABLE_WRAP[rootTag] ? "<table>" : ""
      const wrapOpen = scope + (rootSlot ? `<${rootTag} data-slot="${rootSlot}">` : `<${rootTag}>`)
      const d = new JSDOM(`${wrapOpen}${inner}</${rootTag}>${scope ? "</table>" : ""}`).window.document
      for (const e of d.querySelectorAll("[data-slot]")) {
        defPairs.add(`${e.tagName.toLowerCase()}@${e.getAttribute("data-slot")}`)
        let p = e.parentElement
        while (p && !p.hasAttribute("data-slot")) p = p.parentElement
        if (p) defEdges.add(`${p.getAttribute("data-slot")}>${e.getAttribute("data-slot")}`)
      }
    }

    // IR-side pairs (any slotted element of any fn must appear in DOM via
    // some tree or DEFAULT html)
    const irPairs = new Set()
    for (const c of ir.components) for (const el of c.elements) if (el.slot) {
      const tag = normalizeTag(el.tag, ir.tagHints)
      irPairs.add(tag == null ? `?@${el.slot}` : `${tag}@${el.slot}`)
    }

    const domNodes = [...doc.querySelectorAll("[data-slot]")]
      .map((e) => `${e.tagName.toLowerCase()}@${e.getAttribute("data-slot")}`)
    const domSet = new Set(domNodes)
    const sanctionedPairs = new Set([...treePairs, ...irPairs, ...defPairs])
    const sanctionedEdges = new Set([...treeEdges, ...defEdges])
    for (const p of new Set([...irPairs, ...treePairs])) {
      if (![...domSet].some((d) => d.endsWith("@" + p.split("@")[1]))) {
        console.error(`FAIL [${ir.name}]: IR slot missing in DOM: ${p}`); fail = true
      }
    }
    for (const d of domNodes) {
      if (!sanctionedPairs.has(d)) {
        console.error(`FAIL [${ir.name}]: DOM slot not sanctioned (tree/IR/default): ${d}`); fail = true
      }
    }
    // nesting: every DOM parent→child slot edge must be sanctioned
    const domEdges = new Set()
    for (const e of doc.querySelectorAll("[data-slot] [data-slot]")) {
      const childSlot = e.getAttribute("data-slot")
      let p = e.parentElement
      while (p && !p.hasAttribute("data-slot")) p = p.parentElement
      if (p) domEdges.add(`${p.getAttribute("data-slot")}>${childSlot}`)
    }
    for (const edge of domEdges)
      if (!sanctionedEdges.has(edge)) {
        console.error(`FAIL [${ir.name}]: DOM nesting not sanctioned: ${edge}`); fail = true
      }
    if ((irPairs.size || treePairs.size) && !domNodes.length) {
      console.error(`FAIL [${ir.name}]: no slots rendered`); fail = true
    }
  }

  if (fail) { console.log("FAIL  emit gates"); process.exit(1) }
  console.log(`PASS  emit static gates (${statics.length} files, 0 class=, exact slot-tree, ${allAnchors.size} anchors)`)
}

const isMain = import.meta.url === pathToFileURL(process.argv[1] ?? "x").href
if (isMain) main()
