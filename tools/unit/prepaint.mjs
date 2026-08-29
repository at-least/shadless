// theme pre-paint (src/docs/theme-prepaint.mjs + assets.mjs copy) — migrated
// from tools/unit-check.mjs; strengthened: the derived inline copy must
// actually be script-free (the old assertion computed its expectation with
// the same .replace calls, so a no-op strip passed tautologically).
import { injectPrePaint, THEME_PREPAINT_SCRIPT } from "../../src/docs/theme-prepaint.mjs"
import { themePrePaintInline } from "../../src/docs/assets.mjs"

export function run(t) {
  const SIG = '<script>(function(){try{var k="shadless-docs-theme"'
  const once = injectPrePaint("<html><head><title>t</title></head><body></body></html>")
  t.ok("prePaint: injected into head", once.includes(SIG), once.slice(0, 120))
  const twice = injectPrePaint(once)
  t.eq("prePaint: idempotent", twice, once)
  t.eq("prePaint: assets.mjs copy derived (no drift)", themePrePaintInline,
    THEME_PREPAINT_SCRIPT.replace(/^<script>/, "").replace(/<\/script>$/, ""))
  t.ok("prePaint: derived copy actually stripped (not tautological)",
    !themePrePaintInline.includes("<script") && themePrePaintInline.startsWith("(function(){try{var k="),
    themePrePaintInline.slice(0, 40))
}
