// H1 probe: statically classify shadcn v4 ui components by mechanical decomposability.
// Heuristic regex pass (AST proof is H4's job). Output: table + JSON to probes/out/h1.json
import { readFileSync, readdirSync, mkdirSync, writeFileSync } from "node:fs"
import { join } from "node:path"

const UI = ".upstream/shadcn-ui/apps/v4/registry/new-york-v4/ui"
const files = readdirSync(UI).filter(f => f.endsWith(".tsx"))

const EXTERNAL = ["lucide-react", "cmdk", "sonner", "vaul", "embla-carousel-react",
  "react-day-picker", "recharts", "input-otp", "react-hook-form", "next-themes",
  "react-resizable-panels", "expo-image", "use-controllable-state", "react-photo-view"]

const stripComments = s => s
  .replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\n]*/g, "")
  // remove string literals but keep them out of heuristics
  .replace(/"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|`(?:[^`\\]|\\.)*`/g, '""')

const out = []
for (const f of files) {
  const raw = readFileSync(join(UI, f), "utf8")
  const src = stripComments(raw)

  const imports = [...raw.matchAll(/from\s+"([^"]+)"/g)].map(m => m[1])
  const extDeps = imports.filter(d => EXTERNAL.includes(d))
  const radixParts = imports.filter(d => d === "radix-ui")
  // "import { Dialog as DialogPrimitive } from 'radix-ui'" → namespace alias used: DialogPrimitive.Root
  const radixAliases = [...raw.matchAll(/import\s*\{([^}]+)\}\s*from\s*"radix-ui"/g)]
    .flatMap(m => [...m[1].matchAll(/(\w+)\s+as\s+(\w+)/g)].map(x => x[2]))
  const primitiveUses = radixAliases.flatMap(a =>
    [...new Set([...src.matchAll(new RegExp(a + String.raw`\.(\w+)`, "g"))].map(m => m[1]))])

  // split function components (top-level "function X(" ... before next top-level function/export)
  const fnBlocks = []
  const fnRe = /^(?:export\s+)?(?:function\s+(\w+)|(?:const|let)\s+(\w+)\s*=\s*(?:React\.memo\()?\s*\()/gm
  const starts = []
  let m; while ((m = fnRe.exec(src))) starts.push({ name: m[1] || m[2], i: m.index })
  for (let k = 0; k < starts.length; k++) {
    const end = k + 1 < starts.length ? starts[k + 1].i : src.length
    fnBlocks.push({ name: starts[k].name, body: src.slice(starts[k].i, end) })
  }

  const hooks = [...src.matchAll(/\buse[A-Z]\w*\s*\(/g)].map(m => m[0].replace(/\($/, ""))
  const reactHooks = hooks.filter(h => /^use(State|Effect|Ref|Memo|Callback|Context|Id|LayoutEffect|ImperativeHandle|Transition|DeferredValue|SyncExternalStore)$/.test(h))

  const sub = fnBlocks.map(b => {
    const hasCvaCall = /cva\(/.test(b.body)
    const condChildren = (b.body.match(/\?\s*\(|&&\s*\(/g) || []).length
    const maps = (b.body.match(/\.map\(/g) || []).length
    const handlers = [...b.body.matchAll(/on[A-Z]\w*\s*[=:]/g)].map(x => x[0].replace(/[=:]/, ""))
    const styledWrap = /className\s*=\s*\{?\s*cn\(/.test(b.body) || /className="[^"]+"/.test(b.body)
    return { fn: b.name, styledWrap, hasCvaCall, condChildren, maps, handlers: [...new Set(handlers)] }
  })

  const logicBearing = reactHooks.length > 0
  const classify = extDeps.filter(d => d !== "lucide-react").length > 0
    ? "external-dep"
    : logicBearing ? "logic"
    : "wrapper"

  out.push({ file: f.replace(".tsx", ""), classify, extDeps,
    radixAliases, primitiveUses: [...new Set(primitiveUses)],
    reactHooks: [...new Set(reactHooks)], allHooks: [...new Set(hooks)],
    nFns: fnBlocks.length, sub })
}

mkdirSync("probes/out", { recursive: true })
writeFileSync("probes/out/h1.json", JSON.stringify(out, null, 2))

const wrap = out.filter(o => o.classify === "wrapper")
const logic = out.filter(o => o.classify === "logic")
const ext = out.filter(o => o.classify === "external-dep")
console.log(`total=${out.length}  wrapper=${wrap.length}  logic=${logic.length}  external-dep=${ext.length}`)
console.log(`\nLOGIC (react hooks present):`); logic.forEach(o =>
  console.log(`  ${o.file}: hooks=[${o.reactHooks}] all=[${o.allHooks.filter(h=>!o.reactHooks.includes(h)).join(",")}]`))
console.log(`\nEXTERNAL-DEP:`); ext.forEach(o => console.log(`  ${o.file}: ${o.extDeps}`))
console.log(`\nRADIX PRIMITIVES USED (union):`)
console.log("  " + [...new Set(out.flatMap(o => o.radixAliases))].sort().join(", "))
const totalFns = out.reduce((s, o) => s + o.nFns, 0)
const styledFns = out.reduce((s, o) => s + o.sub.filter(x => x.styledWrap).length, 0)
const cvaFns = out.reduce((s, o) => s + o.sub.filter(x => x.hasCvaCall).length, 0)
const handlerFns = out.flatMap(o => o.sub.filter(x => x.handlers.length))
console.log(`\nsubcomponents: ${totalFns}, styled-wrappers: ${styledFns}, cva-defs: ${cvaFns}`)
console.log(`subcomponents with event handlers (non passthrough heuristics):`)
handlerFns.forEach(x => console.log(`  ${x.fn}: ${x.handlers.join(",")}`))
