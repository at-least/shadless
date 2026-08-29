// Tier classification of ALL shadcn registry ui files by conversion cost.
// The conversion unit is the shadcn file; radix/kernel is a means.
// Tiers: static (pure markup+CSS) / kernel (behavior = kernel wire fn) /
// trivial-js (tiny state machine) / medium (composition or nav trees) /
// external (foreign runtime dep) / logic (react state logic to redesign).
import { parse } from "@babel/parser"
import { readFileSync, readdirSync, mkdirSync, writeFileSync } from "node:fs"
import { join } from "node:path"

const UI = ".upstream/shadcn-ui/apps/v4/registry/bases/radix/ui"
const files = readdirSync(UI).filter(f => f.endsWith(".tsx") && f !== "_registry.tsx")
  .map(f => f.replace(".tsx", "")).sort()

// radix primitive family (from the import alias) → behavior source
const KERNEL = new Set(["Dialog", "AlertDialog", "Sheet", "ContextMenu",
  "DropdownMenu", "Popover", "Tooltip", "HoverCard", "Select", "Tabs",
  "Slider", "ScrollArea"])
const TRIVIAL = new Set(["Accordion", "Collapsible", "Checkbox", "RadioGroup",
  "Switch", "Toggle", "ToggleGroup", "Progress", "Avatar", "AspectRatio",
  "Label", "Separator"])
const MEDIUM = new Set(["Menubar", "NavigationMenu", "Breadcrumb"])
const EXTERNAL = ["cmdk", "sonner", "vaul", "embla-carousel-react",
  "react-day-picker", "recharts", "input-otp", "react-hook-form",
  "next-themes", "react-resizable-panels", "react-photo-view", "expo-image",
  "@shadcn/react/questionnaire"]

const out = {}
for (const name of files) {
  const raw = readFileSync(join(UI, name + ".tsx"), "utf8")
  const deps = [...raw.matchAll(/from "([^"]+)"/g)].map(m => m[1])
  const ext = deps.filter(d => EXTERNAL.includes(d))
  if (ext.length) { out[name] = { tier: "external", ext }; continue }

  const radixAlias = [...raw.matchAll(/import\s*\{[^}]*?\bas\s+(\w+)Primitive\b[^}]*\}\s*from\s*"radix-ui"/g)].map(m => m[1])
  const ast = parse(raw.replace(/^"use client"/m, ""), { sourceType: "module", plugins: ["typescript", "jsx"] })
  let hooks = new Set()
  const walk = (n) => {
    if (!n || !n.type) return
    if (n.type === "CallExpression" && n.callee.type === "Identifier" &&
        /^use[A-Z]/.test(n.callee.name)) hooks.add(n.callee.name)
    for (const k of Object.keys(n)) {
      if (k === "loc") continue
      const v = n[k]
      if (Array.isArray(v)) v.forEach(c => c && typeof c.type === "string" && walk(c))
      else if (v && typeof v.type === "string") walk(v)
    }
  }
  walk(ast.program)
  const reactHooks = [...hooks].filter(h =>
    /^use(State|Effect|Ref|Memo|Callback|Context|Id|LayoutEffect|ImperativeHandle|Transition|DeferredValue)$/.test(h))

  if (reactHooks.length) { out[name] = { tier: "logic", hooks: reactHooks }; continue }
  if (name === "sidebar" || name === "combobox") { out[name] = { tier: "logic" }; continue }
  if (name === "carousel" || name === "drawer" || name === "sonner") { out[name] = { tier: "external" }; continue }

  const k = radixAlias.filter(a => KERNEL.has(a))
  const t = radixAlias.filter(a => TRIVIAL.has(a))
  const m = radixAlias.filter(a => MEDIUM.has(a))
  if (m.length) out[name] = { tier: "medium", radix: radixAlias }
  else if (k.length) out[name] = { tier: "kernel", radix: radixAlias }
  else if (t.length) out[name] = { tier: "trivial-js", radix: radixAlias }
  else out[name] = { tier: "static", radix: radixAlias }
}

const by = t => Object.entries(out).filter(([, v]) => v.tier === t).map(([k]) => k)
for (const t of ["static", "kernel", "trivial-js", "medium", "logic", "external"])
  console.log(`${t} (${by(t).length}): ${by(t).join(", ")}`)
mkdirSync("probes/out", { recursive: true })
writeFileSync("src/registry/tiers.json", JSON.stringify(out, null, 2))
