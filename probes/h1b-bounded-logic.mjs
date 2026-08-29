// H1b: enumerate conditional logic kinds across wave-1 (radix-core) files.
// Wave-1 set = files importing "radix-ui", minus external-dep files.
// Question: are non-mechanical parts a bounded set of condition kinds
// (asChild/position/size/orientation...) with no bespoke state machines?
import { parse } from "@babel/parser"
import { readFileSync, readdirSync, mkdirSync, writeFileSync } from "node:fs"
import { join } from "node:path"

const UI = ".upstream/shadcn-ui/apps/v4/registry/new-york-v4/ui"
const EXT = new Set(["calendar", "carousel", "chart", "command", "drawer", "form",
  "input-otp", "resizable", "sonner", "sidebar"])
const files = readdirSync(UI).filter(f => f.endsWith(".tsx"))
  .map(f => f.replace(".tsx", ""))
  .filter(n => !EXT.has(n) && !n.startsWith("_"))
  .filter(n => /from "radix-ui"/.test(readFileSync(join(UI, n + ".tsx"), "utf8")))
console.log(`wave-1 (imports radix-ui): ${files.length} files`)

const walk = (node, fn, parent = null) => {
  fn(node, parent)
  for (const k of Object.keys(node)) {
    if (k === "loc" || k === "start" || k === "end") continue
    const v = node[k]
    if (Array.isArray(v)) v.forEach(c => c && typeof c.type === "string" && walk(c, fn, node))
    else if (v && typeof v.type === "string") walk(v, fn, node)
  }
}
const idents = (node, acc = new Set()) => {
  walk(node, n => { if (n.type === "Identifier") acc.add(n.name) })
  return acc
}

const findings = []   // {file, kind, detail}
for (const name of files) {
  const src = readFileSync(join(UI, name + ".tsx"), "utf8")
  const ast = parse(src, { sourceType: "module", plugins: ["typescript", "jsx"] })
  walk(ast, (n, parent) => {
    if (!n || !n.type) return
    // conditional JSX children
    if ((n.type === "LogicalExpression" || n.type === "ConditionalExpression") &&
        parent && parent.type === "JSXExpressionContainer") {
      let hasJsx = false
      walk(n, x => { if (x.type === "JSXElement") hasJsx = true })
      if (hasJsx) findings.push({ file: name, kind: "child-cond",
        detail: [...idents(n)].filter(i => !/^(true|false)$/.test(i)).slice(0, 4).join(",") })
    }
    // tag-selecting ternary (Comp = cond ? A : B)
    if (n.type === "ConditionalExpression" && parent?.type === "VariableDeclarator") {
      findings.push({ file: name, kind: "tag-cond", detail: parent.id.name })
    }
    // className with conditional composition
    if (n.type === "JSXAttribute" && n.name.name === "className" &&
        n.value?.type === "JSXExpressionContainer") {
      let cond = null
      walk(n.value.expression, x => {
        if (!cond && (x.type === "ConditionalExpression" || x.type === "LogicalExpression")) cond = x
      })
      if (cond) findings.push({ file: name, kind: "class-cond",
        detail: [...idents(cond)].slice(0, 4).join(",") })
    }
    // template-literal interpolation inside className
    if (n.type === "TemplateLiteral" && parent?.type === "JSXExpressionContainer" &&
        parent.parent?.type === "JSXAttribute" && parent.parent.name?.name === "className") {
      findings.push({ file: name, kind: "class-template",
        detail: n.expressions.map(e => e.type === "Identifier" ? e.name : e.type).join(",") })
    }
  })
}

const byKind = {}
for (const f of findings) (byKind[f.kind] ??= []).push(f)
console.log(`\nconditional findings: ${findings.length}`)
for (const [k, v] of Object.entries(byKind)) {
  console.log(`  ${k}: ${v.length}  files={${[...new Set(v.map(x => x.file))].join(",")}}`)
  const details = [...new Set(v.map(x => x.detail))]
  console.log(`    detail-union (bounded?): ${details.slice(0, 20).join(" | ")}${details.length > 20 ? " …" : ""}`)
}
mkdirSync("probes/out", { recursive: true })
writeFileSync("probes/out/h1b.json", JSON.stringify(findings, null, 1))
