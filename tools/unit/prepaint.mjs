// theme pre-paint (src/docs/theme-prepaint.mjs + inline-script derivation) —
// migrated from tools/unit-check.mjs; strengthened: the derived inline copy
// must actually be script-free (the old assertion computed its expectation
// with the same .replace calls, so a no-op strip passed tautologically).
//
// Wave 0 (2026-08-31): the drift check used to compare against
// themePrePaintInline in src/docs/assets.mjs — that consumer is gone with
// the VitePress port. The derivation itself is still pinned: callers that
// need the bare inline script (no <script> wrapper) derive it with exactly
// this strip, so the test now performs the derivation locally and asserts
// the stripped shape directly.
import { injectPrePaint, THEME_PREPAINT_SCRIPT } from "../../src/docs/theme-prepaint.mjs"

export function run(t) {
  const SIG = '<script>(function(){try{var k="shadless-docs-theme"'
  const once = injectPrePaint("<html><head><title>t</title></head><body></body></html>")
  t.ok("prePaint: injected into head", once.includes(SIG), once.slice(0, 120))
  const twice = injectPrePaint(once)
  t.eq("prePaint: idempotent", twice, once)

  const inline = THEME_PREPAINT_SCRIPT.replace(/^<script>/, "").replace(/<\/script>$/, "")
  t.ok("prePaint: wrapper actually stripped (not tautological)",
    !inline.includes("<script"),
    inline.slice(0, 40))
  t.ok("prePaint: inline form keeps the function body",
    inline.startsWith("(function(){try{var k="),
    inline.slice(0, 40))
}
