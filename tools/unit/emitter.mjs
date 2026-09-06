// emitter (src/emitter/index.mjs) + tags.mjs — Wave H: tree building, render
// semantics (root-replace defaults, leaf fills, anchors), attr merging,
// DEFAULT_CONTENT key validation.
import { buildTree, renderTree, renderFn, escHtml, mergeRootAttrs, validateDefaultContent, resolveDefault, DEFAULT_CONTENT }
  from "../../src/emitter/index.mjs"
import { normalizeTag, kebab, NAT, VOID, externalMemberTag } from "../../src/tags.mjs"

export function run(t) {
  // ---- tags.mjs ----
  t.eq("normalizeTag: native passthrough", normalizeTag("div", {}), "div")
  t.eq("normalizeTag: ternary alternate wins", normalizeTag("<ternary:Slot.Root/span>", {}), "span")
  t.eq("normalizeTag: hint", normalizeTag("Comp", { Comp: "div" }), "div")
  t.eq("normalizeTag: unknown → null (never a silent <button>)", normalizeTag("Mystery", {}), null)
  t.eq("kebab: PascalCase", kebab("ButtonGroupText"), "button-group-text")
  t.eq("kebab: member tag", kebab("PaginationLink"), "pagination-link")
  t.eq("externalMemberTag: Button suffix", externalMemberTag("MessageScrollerPrimitive.Button"), "button")
  t.eq("externalMemberTag: default div", externalMemberTag("MessageScrollerPrimitive.Viewport"), "div")
  t.eq("externalMemberTag: known radix Label", externalMemberTag("LabelPrimitive.Root"), "label")
  t.ok("NAT: fieldset/legend present", NAT.has("fieldset") && NAT.has("legend"))
  t.ok("VOID: input void", VOID.has("input") && !VOID.has("div"))

  // ---- buildTree ----
  {
    // icon child + imported root, both must normalize (old code emitted
    // literal <ChevronLeftIcon> and coerced roots to <button>)
    const ir = {
      name: "x", tagHints: { PaginationLink: "a", ChevronLeftIcon: "svg" },
      components: [{ fn: "Prev", export: true, elements: [
        { tag: "PaginationLink", slot: null, classes: [], spread: false,
          children: ["<ChevronLeftIcon>", '<span class=[1] slot=sr>'] },
        { tag: "ChevronLeftIcon", slot: null, classes: ["size-4"], spread: false, children: [] },
        { tag: "span", slot: "sr", classes: ["sr-only"], spread: false, children: [] },
      ] }],
    }
    const fn = ir.components[0]
    const tree = buildTree(ir, fn)
    t.eq("buildTree: root via tagHints", tree.tag, "a")
    t.eq("buildTree: icon child → svg", tree.kids[0].tag, "svg")
    t.eq("buildTree: native child slot binds", tree.kids[1].slot, "sr")
  }
  t.throws("buildTree: unresolvable root throws", () => {
    buildTree({ name: "x", tagHints: {}, components: [{ fn: "F", export: true,
      elements: [{ tag: "Mystery", slot: null, classes: [], spread: false, children: [] }] }] },
      { fn: "F", elements: [{ tag: "Mystery", slot: null, classes: [], spread: false, children: [] }] })
  }, /unresolvable/)

  // ---- renderTree semantics ----
  {
    const tree = { tag: "div", slot: "root", anchor: null, kids: [
      { tag: "span", slot: "child", anchor: null, kids: [] },
    ] }
    // E2 regression: fn-root default REPLACES content even when kids exist
    // (old code dropped table/pagination compositions)
    t.eq("render: root default replaces kids",
      renderTree(tree, {}, "<p>authored</p>", {}, true),
      '<div data-slot="root"><p>authored</p></div>')
    // no default → kids render
    t.eq("render: kids render without default",
      renderTree(tree, {}, "", {}, true),
      '<div data-slot="root"><span data-slot="child"></span></div>')
    // leaf fill via defaultBySlot (native-select options)
    t.eq("render: leaf fill by slot",
      renderTree({ tag: "select", slot: "native-select", anchor: null, kids: [] }, {}, "",
        { "native-select": "<option>A</option>" }, false),
      '<select data-slot="native-select"><option>A</option></select>')
    // anchor class + markers merge into one class=
    t.eq("render: anchor class emitted",
      renderTree({ tag: "span", slot: null, anchor: "pagination-previous-span", kids: [] },
        {}, "", {}, false),
      '<span class="pagination-previous-span"></span>')
    t.eq("render: void tag has no closing",
      renderTree({ tag: "input", slot: "i", anchor: null, kids: [] }, {}, "", {}, false),
      '<input data-slot="i">')
  }

  // ---- renderFn table wrapping ----
  t.eq("renderFn: thead wrapped in table",
    renderFn({ tag: "thead", slot: "h", anchor: null, kids: [] }),
    '<table><thead data-slot="h"></thead></table>')

  // ---- escHtml / mergeRootAttrs ----
  t.eq("escHtml: all four", escHtml('<a & "b">'), "&lt;a &amp; &quot;b&quot;&gt;")
  t.eq("mergeRootAttrs: basic", mergeRootAttrs('<input data-slot="i">', { placeholder: "Type…" }),
    '<input data-slot="i" placeholder="Type…">')
  t.eq("mergeRootAttrs: attr value containing > (quote-aware)",
    mergeRootAttrs('<button data-slot="b" aria-label="→ go"></button>', { "data-x": "1" }),
    '<button data-slot="b" aria-label="→ go" data-x="1"></button>')

  // ---- resolveDefault: html + attrs + children compose, none get dropped ----
  {
    // alert/Alert: the historical regression shape — html AND attrs on the
    // same entry; a prior bug picked one and dropped the other, so role=
    // on an entry that already had html was a no-op.
    const alert = resolveDefault({ name: "alert" }, { fn: "Alert" })
    t.eq("resolveDefault: html+attrs — inner survives", alert.inner, DEFAULT_CONTENT.alert.Alert.html)
    t.eq("resolveDefault: html+attrs — attrs survive alongside inner",
      alert.attrs, DEFAULT_CONTENT.alert.Alert.attrs)

    // input/Input: attrs-only entry — no inner, no children keys present
    const input = resolveDefault({ name: "input" }, { fn: "Input" })
    t.eq("resolveDefault: attrs-only", input, { attrs: DEFAULT_CONTENT.input.Input.attrs })

    // native-select/NativeSelect: children-only entry
    const nativeSelect = resolveDefault({ name: "native-select" }, { fn: "NativeSelect" })
    t.eq("resolveDefault: children-only",
      nativeSelect, { children: DEFAULT_CONTENT["native-select"].NativeSelect.children })

    // spinner/Spinner: object-form html is used verbatim — escHtml only
    // applies to the string-shorthand branch (a bare text default), never
    // to an object entry's html chunk (already markup).
    const spinner = resolveDefault({ name: "spinner" }, { fn: "Spinner" })
    t.eq("resolveDefault: object-form html is NOT escaped", spinner.inner, DEFAULT_CONTENT.spinner.Spinner.html)
    t.ok("resolveDefault: object-form html keeps raw markup", spinner.inner.includes("<svg"))

    // string-shorthand entry → escaped text node
    t.eq("resolveDefault: string shorthand is escaped",
      resolveDefault({ name: "badge" }, { fn: "Badge" }), { inner: escHtml(DEFAULT_CONTENT.badge.Badge) })

    // explicit null entry (a real fn with deliberately no default) → null,
    // same result as a fn absent from the map entirely
    t.eq("resolveDefault: explicit null entry → null",
      resolveDefault({ name: "message" }, { fn: "MessageGroup" }), null)
    // no entry for this fn at all → null
    t.eq("resolveDefault: no entry → null", resolveDefault({ name: "alert" }, { fn: "NoSuchFn" }), null)
  }

  // ---- validateDefaultContent: stale keys must fail ----
  {
    // fake statics export only Alert/AlertTitle — every other key present in
    // the real DEFAULT_CONTENT (AlertDescription; AlertAction was removed as
    // a dead key in Wave H) must be flagged
    const statics = [{ name: "alert", components: [
      { fn: "Alert", export: true }, { fn: "AlertTitle", export: true },
    ] }]
    const errs = validateDefaultContent(statics)
    t.ok("defaults: non-exported fn key flagged",
      errs.some((e) => e.includes("AlertDescription")), JSON.stringify(errs))
    t.ok("defaults: stale fn key count honest", errs.length >= 1)
  }
}
