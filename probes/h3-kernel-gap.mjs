// H3 probe: map shadcn wave-1 primitives → kernel wire functions; list gaps.
import { readFileSync, existsSync } from "node:fs"

const KERNEL = "vendor/radix-kernel.iife.js"
const ksrc = readFileSync(KERNEL, "utf8")
const wires = new Set([...ksrc.matchAll(/\bwire[A-Z]\w*/g)].map(m => m[0]))

// primitive → kernel feature (Sheet=Dialog; ContextMenu/DropdownMenu=menu)
const MAP = {
  AlertDialog: ["wireDialog"], Dialog: ["wireDialog"], Sheet: ["wireDialog"],
  ContextMenu: ["wireMenu"], DropdownMenu: ["wireMenu"],
  HoverCard: ["wireHoverCard"], Popover: ["wirePopover"], Tooltip: ["wireTooltip"],
  Select: ["wireSelect"], Tabs: ["wireTabs"], Slider: ["wireSlider"],
  ScrollArea: ["wireScrollArea"],
}
const TRIVIAL = { // behavior needed but no kernel machinery required
  AspectRatio: "CSS aspect-ratio + wrapper (no JS state)",
  Avatar: "img load/error → fallback swap",
  Checkbox: "checked/indeterminate + data-state + form value",
  Collapsible: "open/close state + data-state",
  Label: "label↔control focus (native, no JS)",
  Progress: "value→aria + width",
  RadioGroup: "roving focus + form value",
  Separator: "static, no JS",
  Switch: "checkbox variant",
  Toggle: "pressed state + data-state=on",
  ToggleGroup: "group context (single/multiple) + Toggle",
  Accordion: "collapsible ×N + single/multiple + arrow roving focus",
}
const MEDIUM = {
  Menubar: "menu machines + horizontal root bar + root-level roving focus",
  NavigationMenu: "viewport motion (data-motion=*) + tree nav — hardest in wave 1",
}

const rows = []
for (const [p, ks] of Object.entries(MAP))
  rows.push({ primitive: p, kernel: ks.map(k => wires.has(k) ? k : `${k}(MISSING)`),
    status: ks.every(k => wires.has(k)) ? "covered" : "missing" })
for (const [p, note] of Object.entries(TRIVIAL)) rows.push({ primitive: p, kernel: [], status: "gap-trivial", note })
for (const [p, note] of Object.entries(MEDIUM)) rows.push({ primitive: p, kernel: [], status: "gap-medium", note })

for (const r of rows) console.log(
  `${r.primitive.padEnd(17)} ${r.status.padEnd(12)} ${r.kernel.join(",") || ""} ${r.note || ""}`)
const cov = rows.filter(r => r.status === "covered").length
console.log(`\nkernel-covered: ${cov}/${rows.length}  trivial-gaps: ${rows.filter(r => r.status === "gap-trivial").length}  medium-gaps: ${rows.filter(r => r.status === "gap-medium").length}`)
