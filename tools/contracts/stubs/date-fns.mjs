// date-fns subset used by examples (display-only formatting)
export function format(date, _fmt) {
  const d = new Date(date)
  return d.toLocaleDateString("en-US", { month: "long", day: "numeric", year: "numeric" })
}
export function addDays(date, n) {
  const d = new Date(date)
  d.setDate(d.getDate() + n)
  return d
}
