// docs routes (tools/docs-guides.mjs resolveDocsRoute) — migrated from
// tools/unit-check.mjs, assertions unchanged.
import { resolveDocsRoute, GUIDES } from "../docs-guides.mjs"

export function run(t) {
  const members = new Set(["alert", "button"])
  t.eq("route: radix member", resolveDocsRoute("/docs/components/alert", members), { file: "alert.html", frag: undefined })
  t.eq("route: radix-prefixed member", resolveDocsRoute("/docs/components/radix/alert", members), { file: "alert.html", frag: undefined })
  // variant mirror retired (2026-08-26): base/aria routes must grey out
  t.eq("route: base variant greyed (retired)", resolveDocsRoute("/docs/components/base/alert", members), { grey: true })
  t.eq("route: aria variant greyed (retired)", resolveDocsRoute("/docs/components/aria/alert#x", members), { grey: true })
  t.eq("route: unknown component greyed", resolveDocsRoute("/docs/components/nope", members), { grey: true })
  t.eq("route: guide", resolveDocsRoute("/docs/introduction", members), { file: "introduction.html", frag: undefined })
  t.eq("route: pruned greyed", resolveDocsRoute("/docs/react", members), { grey: true })
  t.eq("route: relative href → null", resolveDocsRoute("alert.html", members), null)
  t.ok("route: GUIDES list sane", GUIDES.length >= 9 && GUIDES.every((g) => g.slug && g.route && g.source !== undefined),
    `guides=${GUIDES.length}`)
}
