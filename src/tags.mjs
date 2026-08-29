// Tag normalization shared by converter / emitter / css.mjs.
// Single source of truth for "what native tag does this IR tag render as".
//
// Resolution order for a raw IR tag:
//   1. native tag            → itself
//   2. `<ternary:A/B>` form  → alternate B (asChild ? Slot.Root : "div" — the
//                              Slot branch is React-only, the native string wins)
//   3. ir.tagHints[tag]      → explicit hint recorded by the converter
//                              (fn-local `const Comp = cond ? X : "div"`,
//                              same-file / imported component root tags,
//                              lucide icons → "svg",
//                              external member tags → suffix rule)
//   4. anything else         → null (UNRESOLVED — callers must fail loudly;
//                              the old emitter silently coerced these to
//                              <button>, shipping <button placeholder=…>)
//
// External member tags (MessageScrollerPrimitive.Viewport, Direction.*) come
// from packages we don't vendor, so their root tags can't be read statically.
// Documented suffix rule: Button/Trigger/Link → button, everything else → div.

export const NAT = new Set(["div","span","p","a","button","h1","h2","h3","h4","h5","h6",
  "ul","ol","li","nav","table","thead","tbody","tfoot","tr","th","td","caption",
  "input","select","option","optgroup","textarea","label","form","img","svg","path",
  "circle","line","rect","polygon","polyline","ellipse","g","defs","use",
  "section","header","footer","main","article","aside","small","strong","em","kbd",
  "dl","dt","dd","fieldset","legend","output","datalist","meter","progress",
  "details","summary","picture","time","mark","sub","sup","i","b","u","s",
  "abbr","address","hgroup","dialog","search","blockquote","code","pre",
  "template","style","script","title","head","body","html",
  "figure","figcaption"])

// HTML5 void elements — never emit a closing tag
export const VOID = new Set(["br","hr","img","input","meta","link","area","base",
  "col","embed","source","track","wbr"])

// External member-expression tag suffix rule (see header). `X.Button` → button.
// KNOWN_MEMBER_TAGS overrides first — primitives whose real root tag is known
// (verified against node_modules) and differs from the suffix rule.
const KNOWN_MEMBER_TAGS = {
  "LabelPrimitive.Root": "label", // @radix-ui/react-label renders <label>
}
export function externalMemberTag(tag) {
  if (Object.hasOwn(KNOWN_MEMBER_TAGS, tag)) return KNOWN_MEMBER_TAGS[tag]
  const suffix = tag.slice(tag.lastIndexOf(".") + 1)
  return ["Button", "Trigger", "Link"].includes(suffix) ? "button" : "div"
}

// kebab-case a PascalCase identifier (ButtonGroupText → button-group-text)
export function kebab(s) {
  return s.replace(/([a-z0-9])([A-Z])/g, "$1-$2").replace(/[\s.]+/g, "-").toLowerCase()
}

export function normalizeTag(tag, hints = {}) {
  if (tag == null) return null
  if (NAT.has(tag)) return tag
  const m = /^<ternary:([^/]+)\/(.+)>$/.exec(tag)
  if (m) return normalizeTag(m[2], hints)
  const h = hints[tag]
  if (h != null) return NAT.has(h) ? h : null
  return null
}
