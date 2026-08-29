// Tiny zero-framework test harness shared by tools/unit/*.mjs modules.
// Same convention as every gate in this repo: inline asserts, honest PASS
// line, exit 1 on any failure. Modules export run(t); the runner aggregates.
export function makeT(suite, failures) {
  let n = 0
  const eq = (label, actual, expected) => {
    n++
    const a = JSON.stringify(actual), e = JSON.stringify(expected)
    if (a !== e) { failures.push(`${suite}/${label}`); console.error(`FAIL  ${suite}/${label}: got ${a}, want ${e}`) }
  }
  const ok = (label, cond, detail = "") => {
    n++
    if (!cond) { failures.push(`${suite}/${label}`); console.error(`FAIL  ${suite}/${label}${detail ? ` — ${detail}` : ""}`) }
  }
  const throws = (label, fn, match) => {
    n++
    try { fn(); failures.push(`${suite}/${label}`); console.error(`FAIL  ${suite}/${label}: did not throw`) }
    catch (e) {
      if (match && !new RegExp(match).test(e.message)) {
        failures.push(`${suite}/${label}`)
        console.error(`FAIL  ${suite}/${label}: threw wrong error: ${e.message}`)
      }
    }
  }
  return { eq, ok, throws, get count() { return n } }
}
