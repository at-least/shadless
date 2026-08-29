// css-direction-gate (J5) — pure-function tests for the physical-utility
// classifier and scanner. Negative-case evidence (drift detection end to
// end); here we pin the classification rules.
import { readFileSync } from "node:fs"
import {
  utilitySegment, isPhysicalUtility, extractApplyTokens, scanDirections,
} from "../css-direction-gate.mjs"

// Fixture tokens are CONCATENATED at runtime on purpose: this file is
// tracked, and the tailwind CLI's repo-wide auto-scan (see tools/tw.mjs)
// would otherwise absorb these literals into dist/out.css as phantom
// utility rules. Do not "simplify" them back to literals.
const u = (...parts) => parts.join("")

export function run(t) {
  // variant-prefix stripping: keep the segment after the LAST ':'
  t.eq("segment: plain", utilitySegment(u("p", "r-9")), u("p", "r-9"))
  t.eq("segment: responsive", utilitySegment(u("sm:p", "l-2.5")), u("p", "l-2.5"))
  t.eq("segment: has-variant", utilitySegment(u("has-[>kbd]:m", "r-[-0.35rem]")), u("m", "r-[-0.35rem]"))
  t.eq("segment: named group", utilitySegment(u("group-data-[orientation=vertical]/attachment:right", "-3")), u("right", "-3"))
  t.eq("segment: arbitrary-variant chain", utilitySegment(u("[&>*:not(:last-child)]:", u("rounded", "-r-none"))), u("rounded", "-r-none"))
  t.eq("segment: pseudo", utilitySegment(u("before:", u("m", "r-1"))), u("m", "r-1"))

  // physical shapes
  for (const tok of [u("m", "l-1"), u("mr", "-[-0.45rem]"), u("pl", "-3"), u("p", "r-0"), u("left", "-3"), u("right", "-3.5"),
    u("text", "-left"), u("text", "-right"), u("border", "-l-0"), u("rounded", "-r-none"), u("rounded", "-l-md"),
    u("space", "-x-2"), u("divide", "-x-2")])
    t.ok(`physical: ${tok}`, isPhysicalUtility(tok))
  // lookalikes that must NOT match
  for (const tok of [u("sr", "-only"), u("rounded", "-lg"), u("rounded", "-sm"), u("border", "-ring"), u("border", "-l"),
    u("inset", "-s-1/2"), "inset-0", u("m", "s-auto"), u("me", "-1"), u("p", "s-2"), u("pe", "-2"), u("start", "-0"), u("end", "-2"),
    "text-muted-foreground", u("focus-visible:", u("border", "-ring")), u("placeholder:", "text-sm"),
    u("translate", "-x-2"), "mx-2", "px-3", "slashed-zero"])
    t.ok(`not physical: ${tok}`, !isPhysicalUtility(tok))

  // scanner: counts physical tokens once per occurrence, sorted output
  const css = `@layer components {
  .a { @apply ${u("m", "l-1")} ${u("m", "l-1")} ${u("p", "r-2")} rounded-lg sr-only border-ring; }
  .b::before { @apply ${u("sm:p", "l-4")} ${u("has-[>kbd]:m", "r-1")} ${u("inset", "-s-1/2")}; }
  .c { color: var(--x); } /* not an @apply */
}`
  t.eq("scan: counts + filtering + sort", scanDirections(css),
    [[u("m", "l-1"), 2], [u("m", "r-1"), 1], [u("p", "l-4"), 1], [u("p", "r-2"), 1]])
  t.eq("extract: token list", extractApplyTokens("@apply a b; x @apply c;").length, 3)

  // the real emitted CSS must stay scannable (catches syntax drift in the
  // @apply extraction regex when the emitter changes shape)
  const real = readFileSync(new URL("../../dist/shadless.css", import.meta.url), "utf8")
  t.ok("scan: real dist/shadless.css yields entries", scanDirections(real).length >= 20)
}
