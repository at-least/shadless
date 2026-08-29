// RTL transform (tools/rtl-lib.mjs) — Wave H: the Wave G fixes (bare
// <html> lang injection, Persian dir=rtl, longest-first substitution,
// dir attribute-boundary) finally have regression tests.
import { extractTranslations, substituteAndPatch, parseTs } from "../rtl-lib.mjs"

export function run(t) {
  // ---- extractTranslations ----
  {
    const src = `const translations = {
  en: { dir: "ltr", values: { title: "Payment successful", nested: \`template string\` } },
  ar: { dir: "rtl", values: { title: "تم الدفع بنجاح" } },
  he: { dir: "rtl", values: { title: "התשלום בוצע" } },
}`
    const tr = extractTranslations(parseTs(src))
    t.eq("rtl: default dir ltr", tr.en.dir, "ltr")
    t.eq("rtl: rtl dir kept", tr.ar.dir, "rtl")
    t.eq("rtl: string value", tr.ar.values.title, "تم الدفع بنجاح")
    t.eq("rtl: template-literal value", tr.en.values.nested, "template string")
  }
  t.eq("rtl: no dict → null", extractTranslations(parseTs("const x = 1")), null)

  // ---- substituteAndPatch ----
  const TR = {
    ar: { dir: "rtl", values: { title: "عنوان", loading: "جار التحميل" } },
    en: { dir: "ltr", values: { title: "Title", loading: "Loading…" } },
    he: { dir: "rtl", values: { title: "כותרת" } },
  }
  {
    const warns = []
    const out = substituteAndPatch(
      '<!doctype html><html lang="ar" dir="rtl"><body><div dir="rtl">عنوان</div></body></html>',
      TR, "ar", "en", TR.en.values, undefined, { onUnmatched: (m) => warns.push(m) })
    t.ok("sub: lang patched", out.includes('<html lang="en"'))
    t.ok("sub: dir flips to ltr for en", !/dir="rtl"/.test(out), out)
    t.ok("sub: text substituted", out.includes("Title"))
    // missing target key on ar side → warned, not crashed
    t.ok("sub: missing key warned", warns.length === 1 && warns[0].includes("loading"), warns.join("; "))
  }
  {
    // bare <html> (no lang) gets lang injected (Wave G fix)
    const out = substituteAndPatch("<html><body>x</body></html>", TR, "ar", "he", TR.he.values, undefined, { onUnmatched: () => {} })
    t.ok("sub: bare html lang injected", out.includes('<html lang="he"'), out)
    // he dir=rtl kept (Wave G: fa was flipped to ltr by the old code)
    t.ok("sub: he stays rtl by default", !/dir="ltr"/.test(out.replace(/lang="he"/, "")), out)
  }
  {
    // explicit toDir wins (Persian case)
    const out = substituteAndPatch('<html lang="ar" dir="rtl"></html>', TR, "ar", "fa",
      { title: "پ" }, "rtl", { onUnmatched: () => {} })
    t.ok("sub: explicit rtl dir honored", out.includes('dir="rtl"'))
  }
  {
    // longest-first: "Loading" inside "Loading…" must not clobber the longer
    const tr = {
      ar: { dir: "rtl", values: { a: "جار", b: "جار التحميل" } },
      en: { dir: "ltr", values: { a: "Loading", b: "Loading…" } },
    }
    const out = substituteAndPatch("<p>جار التحميل</p>", tr, "ar", "en", tr.en.values, undefined, { onUnmatched: () => {} })
    t.eq("sub: longest-first ordering", out, "<p>Loading…</p>")
  }
  {
    // attribute boundary: data-dir="ltr" must NOT be rewritten
    const out = substituteAndPatch('<html lang="ar" dir="rtl" data-dir="ltr"></html>',
      TR, "ar", "en", { title: "T" }, undefined, { onUnmatched: () => {} })
    t.ok("sub: data-dir untouched", out.includes('data-dir="ltr"'), out)
    t.ok("sub: real dir rewritten", out.includes('dir="ltr" data-dir'), out)
  }
}
