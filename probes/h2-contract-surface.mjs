// H2 probe: extract the state-attribute contract surface that shadcn Tailwind
// classes depend on. Scans RAW source for tailwind state-selector tokens
// (`data-[...]:`, `peer-data-[...]:`, `group-data-[...]:`, `aria-x:` — the
// trailing colon is what distinguishes a tailwind variant from a JSX attr).
// Output: probes/out/h2.json
import { readFileSync, readdirSync, mkdirSync, writeFileSync } from "node:fs"
import { join } from "node:path"

const UI = ".upstream/shadcn-ui/apps/v4/registry/new-york-v4/ui"
const EXCLUDE = new Set(["calendar", "carousel", "chart", "command", "drawer",
  "form", "input-otp", "resizable", "sonner", "sidebar", "_registry"])
const files = readdirSync(UI).filter(f => f.endsWith(".tsx") &&
  !EXCLUDE.has(f.replace(".tsx", "")))

const tokens = new Map()
for (const f of files) {
  const name = f.replace(".tsx", "")
  const src = readFileSync(join(UI, f), "utf8")
  const re = /(?:(peer|group)-)?data-\[?([a-z-]+)(?:=([^\]"'`\s]+))?\]?(?=:)/g
  let m
  while ((m = re.exec(src))) {
    const key = `${m[1] ? m[1] + "-data" : "data"}-${m[2]}${m[3] ? "=" + m[3] : ""}`
    if (!tokens.has(key)) tokens.set(key, new Set())
    tokens.get(key).add(name)
  }
  const re2 = /(?<![\w-[(])(aria-[a-z-]+):(?=[a-z\[])/g
  while ((m = re2.exec(src))) {
    if (!tokens.has(m[1])) tokens.set(m[1], new Set())
    tokens.get(m[1]).add(name)
  }
}

const sorted = [...tokens.entries()].sort((a, b) => a[0].localeCompare(b[0]))
mkdirSync("probes/out", { recursive: true })
writeFileSync("probes/out/h2.json", JSON.stringify(Object.fromEntries(
  sorted.map(([k, v]) => [k, [...v].sort()])), null, 2))

console.log(`${sorted.length} distinct state-selector tokens across ${files.length} wave-1 files\n`)
for (const [k, v] of sorted) console.log(`  ${k.padEnd(36)} ${[...v].join(", ")}`)
