// RTL transform pure functions (Wave H) — extracted from tools/build-rtl.mjs
// for unit testing. The Wave G fixes (bare-<html> lang patch, Persian dir,
// missing-key ordering, dir attribute-boundary) had zero regression tests.
import { parse } from "@babel/parser"

export const parseTs = (src) =>
  parse(src, { sourceType: "module", plugins: ["typescript", "jsx"] })

// Walk the AST looking for `const translations = { en: { dir, values: {...} }, ... }`.
// Returns { en: {dir, values: {key: string}}, ar: {...}, he: {...} } or null.
export function extractTranslations(ast) {
  for (const node of ast.program.body) {
    if (node.type !== "VariableDeclaration") continue
    for (const d of node.declarations) {
      if (d.id?.name !== "translations") continue
      const init = d.init
      if (init?.type !== "ObjectExpression") continue
      const result = {}
      for (const prop of init.properties) {
        const langKey = prop.key?.name || prop.key?.value
        if (!langKey) continue
        const langNode = prop.value
        if (langNode?.type !== "ObjectExpression") continue
        const lang = { dir: "ltr", values: {} }
        for (const p of langNode.properties) {
          if (p.key?.name === "dir" && p.value?.type === "StringLiteral") {
            lang.dir = p.value.value
          } else if (p.key?.name === "values" && p.value?.type === "ObjectExpression") {
            for (const v of p.value.properties) {
              const k = v.key?.name || v.key?.value
              let val
              if (v.value?.type === "StringLiteral") val = v.value.value
              else if (v.value?.type === "TemplateLiteral" && v.value.quasis.length === 1) {
                val = v.value.quasis[0].value.cooked
              }
              if (k && val !== undefined) lang.values[k] = val
            }
          }
        }
        result[langKey] = lang
      }
      return result
    }
  }
  return null
}

// warn hook injectable for tests (build-rtl passes nothing → console.warn)
export function substituteAndPatch(arabicHtml, translations, fromLang, toLang, toValues, toDir,
  { onUnmatched } = {}) {
  let out = arabicHtml
  // Substitute each translation key's value from the source language
  // to the target language. Order by descending value length so longer
  // strings (which may contain shorter ones as substrings) get matched
  // first; otherwise "Loading" inside "Loading…" would only match the
  // shorter token. Keys missing on the from-side sort last instead of
  // crashing (upstream asymmetry is reported below).
  const fromValues = translations[fromLang]?.values ?? {}
  const keys = Object.keys(toValues).sort((a, b) =>
    (fromValues[b]?.length ?? 0) - (fromValues[a]?.length ?? 0))
  const unmatched = []
  for (const key of keys) {
    const fromVal = fromValues[key]
    const toVal = toValues[key]
    if (fromVal && toVal && fromVal !== toVal) {
      if (!out.includes(fromVal)) { unmatched.push(key); continue }
      out = out.split(fromVal).join(toVal)
    } else if (toVal && !fromVal) {
      unmatched.push(`${key}(no ${fromLang} source)`)
    }
  }
  if (unmatched.length)
    (onUnmatched ?? ((msg) => console.warn(msg)))(
      `warn ${toLang}: translation keys not found in HTML (left as ${fromLang}): ${unmatched.join(", ")}`)
  // Patch lang attribute + every dir attribute in the document so the
  // whole layout flips (cascading dir from body alone isn't enough when
  // the demo's outer div has its own explicit dir — e.g. alert-rtl
  // keeps dir="rtl" on the inner grid wrapper). Handles templates with
  // OR without an existing lang attribute.
  const langDir = toDir ?? translations[toLang]?.dir ?? "ltr"
  if (/<html[^>]*\slang="/.test(out))
    out = out.replace(/(<html[^>]*\slang=")[^"]*(")/, `$1${toLang}$2`)
  else
    out = out.replace(/<html(\s[^>]*)?>/, (m) =>
      m.endsWith(">") ? `${m.slice(0, -1)} lang="${toLang}">` : m)
  // attribute-boundary anchored: a bare global would also rewrite the tail
  // of e.g. data-dir="ltr" (no live case today — guard is cheap)
  out = out.replace(/([\s"'])dir="(rtl|ltr)"/g, `$1dir="${langDir}"`)
  return out
}
