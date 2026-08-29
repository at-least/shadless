// H4 spike: AST-parse shadcn button.tsx + dialog.tsx → IR → emit demo page.
// Proves: (a) babel AST extraction of classes/cva/DOM-shape is mechanical;
// (b) emitted markup + kernel wireDialog reproduces the radix contract.
import { parse } from "@babel/parser"
import { readFileSync, writeFileSync, mkdirSync } from "node:fs"
import { join } from "node:path"

const UI = ".upstream/shadcn-ui/apps/v4/registry/new-york-v4/ui"
mkdirSync("probes/h4", { recursive: true })

const parseTs = src => parse(src, {
  sourceType: "module", plugins: ["typescript", "jsx"],
})

// --- AST helpers -----------------------------------------------------------
const str = n => n && n.type === "StringLiteral" ? n.value : null
const jsxName = n =>
  n.type === "JSXIdentifier" ? n.name :
  n.type === "JSXMemberExpression" ? `${jsxName(n.object)}.${n.property.name}` : null

function extractAttrs(el) {
  let slot = null, classes = [], spread = false
  for (const a of el.openingElement.attributes) {
    if (a.type === "JSXSpreadAttribute") { spread = true; continue }
    if (a.name.name === "data-slot") slot = str(a.value)
    if (a.name.name === "className") {
      const v = a.value
      if (!v) continue
      if (v.type === "StringLiteral") classes.push(v.value)
      if (v.type === "JSXExpressionContainer" && v.expression.type === "CallExpression"
          && v.expression.callee.name === "cn")
        for (const arg of v.expression.arguments)
          if (arg.type === "StringLiteral") classes.push(arg.value)
          else if (arg.type === "TemplateLiteral" && arg.quasis.length === 1)
            classes.push(arg.quasis[0].value.cooked)
    }
  }
  return { slot, classes, spread }
}

// children sketch: literal element names / {children} / conditional blocks
function sketchChildren(el) {
  const out = []
  for (const c of el.children) {
    if (c.type === "JSXText") { if (c.value.trim()) out.push("text"); continue }
    if (c.type === "JSXExpressionContainer") {
      const e = c.expression
      if (e.type === "Identifier") out.push(`{${e.name}}`)
      else if (e.type === "LogicalExpression" || e.type === "ConditionalExpression")
        out.push("OPT?")
      else out.push("expr")
      continue
    }
    if (c.type === "JSXElement") {
      const n = jsxName(c.openingElement.name)
      const a = extractAttrs(c)
      out.push(`<${n}${a.slot ? ` slot=${a.slot}` : ""}${a.classes.length ? ` class=[${a.classes.length} str]` : ""}>`)
    }
  }
  return out
}

function extractComponent(src, fnName) {
  const ast = parseTs(src)
  let fn = null
  for (const n of ast.program.body) {
    if (n.type === "FunctionDeclaration" && n.id.name === fnName) { fn = n; break }
  }
  if (!fn) throw new Error(`fn ${fnName} not found`)
  // collect every JSXElement in the component body with tag/slot/classes
  const elements = []
  const visit = node => {
    if (node.type === "JSXElement") {
      const tag = jsxName(node.openingElement.name)
      const { slot, classes, spread } = extractAttrs(node)
      elements.push({ tag, slot, classes, spread, children: sketchChildren(node) })
    }
    for (const k of Object.keys(node)) {
      const v = node[k]
      if (Array.isArray(v)) v.forEach(c => c && typeof c.type === "string" && visit(c))
      else if (v && typeof v.type === "string") visit(v)
    }
  }
  visit(fn.body)
  return { fn: fnName, elements }
}

function extractCva(src, varName) {
  const ast = parseTs(src)
  for (const n of ast.program.body) {
    if (n.type === "VariableDeclaration")
      for (const d of n.declarations) {
        if (d.id.name !== varName) continue
        const call = d.init
        const base = str(call.arguments[0])
        const cfg = call.arguments[1]
        let variants = {}, defaults = {}
        if (cfg) for (const p of cfg.properties) {
          if (p.key.name === "variants") {
            for (const vk of p.value.properties) {
              variants[vk.key.name] = {}
              for (const vv of vk.value.properties)
                variants[vk.key.name][vv.key.name] = str(vv.value)
            }
          }
          if (p.key.name === "defaultVariants") {
            for (const dv of p.value.properties) defaults[dv.key.name] = str(dv.value)
          }
        }
        return { base, variants, defaults }
      }
  }
  throw new Error(`cva ${varName} not found`)
}

// --- extract ---------------------------------------------------------------
const buttonSrc = readFileSync(join(UI, "button.tsx"), "utf8")
const dialogSrc = readFileSync(join(UI, "dialog.tsx"), "utf8")

const buttonVariants = extractCva(buttonSrc, "buttonVariants")
const Button = extractComponent(buttonSrc, "Button")
const dialogSubs = ["Dialog", "DialogTrigger", "DialogPortal", "DialogClose",
  "DialogOverlay", "DialogContent", "DialogHeader", "DialogFooter",
  "DialogTitle", "DialogDescription"].map(f => extractComponent(dialogSrc, f))

const IR = {
  source: { repo: "shadcn-ui", commit: "c06da1d0", path: "apps/v4/registry/new-york-v4/ui" },
  button: { variants: buttonVariants, comp: Button },
  dialog: dialogSubs,
}
writeFileSync("probes/h4/ir.json", JSON.stringify(IR, null, 2))

// --- primitive DOM model (the once-per-primitive hand table) ---------------
// what radix primitives render for the non-styling part of the contract
const DOM_MODEL = {
  "DialogPrimitive.Root": { kind: "state", note: "no DOM" },
  "DialogPrimitive.Trigger": { tag: "button", attrs: { type: "button", "aria-haspopup": "dialog" }, stateOwner: ["open", "closed"], aria: { "aria-expanded": "state" } },
  "DialogPrimitive.Portal": { tag: "template", note: "portal chain root appended to body" },
  "DialogPrimitive.Overlay": { tag: "div", stateOwner: ["open", "closed"] },
  "DialogPrimitive.Content": { tag: "div", attrs: { role: "dialog", tabindex: "-1" }, stateOwner: ["open", "closed"], note: "primitives sets NO aria-modal (h3b recording)" },
  "DialogPrimitive.Close": { tag: "button", attrs: { type: "button" } },
  "DialogPrimitive.Title": { tag: "h2", note: "aria-labelledby wiring" },
  "DialogPrimitive.Description": { tag: "p", note: "aria-describedby wiring" },
}

const byFn = Object.fromEntries(dialogSubs.map(s => [s.fn, s]))
// class lookup by data-slot (falls back to first element of the sub)
const slotClasses = {}
for (const s of dialogSubs) for (const el of s.elements)
  if (el.slot && el.classes.length && !(el.slot in slotClasses))
    slotClasses[el.slot] = el.classes.join(" ")
const classes = slot => { if (!(slot in slotClasses)) throw new Error("no classes for " + slot); return slotClasses[slot] }
const esc = s => s.replace(/"/g, "&quot;")
const clsAttr = slot => `class="${esc(classes(slot))}"`

// --- emit demo.html (markup shape per DOM_MODEL, classes per IR) -----------
const btn = (variant, size, label) => {
  const v = buttonVariants.variants.variant[variant]
  const s = buttonVariants.variants.size[size]
  return `    <button ${clsAttrOf([buttonVariants.base, v, s])} data-slot="button" data-variant="${variant}" data-size="${size}">${label}</button>`
}
const clsAttrOf = parts => `class="${esc(parts.filter(Boolean).join(" "))}"`

const html = `<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>shadless h4 probe</title>
<link rel="stylesheet" href="out.css">
</head>
<body class="bg-background text-foreground p-8 grid gap-8">
  <section data-probe="button" class="flex flex-wrap gap-3 items-center">
${["default", "destructive", "outline", "secondary", "ghost", "link"].map(v => btn(v, "default", v)).join("\n")}
  </section>
  <section data-probe="dialog">
    <button id="d1-trigger" ${clsAttrOf([buttonVariants.base, buttonVariants.variants.variant.default, buttonVariants.variants.size.default])} data-slot="dialog-trigger" type="button" aria-haspopup="dialog" aria-expanded="false">Open dialog</button>
    <template id="d1-portal" data-slot="dialog-portal">
      <div ${clsAttr("dialog-overlay")} data-slot="dialog-overlay" data-state="closed"></div>
      <div id="d1" ${clsAttr("dialog-content")} data-slot="dialog-content" role="dialog" aria-labelledby="d1-title" aria-describedby="d1-desc" tabindex="-1" data-state="closed">
        <h2 ${clsAttr("dialog-title")} data-slot="dialog-title" id="d1-title">Are you absolutely sure?</h2>
        <p ${clsAttr("dialog-description")} data-slot="dialog-description" id="d1-desc">This action cannot be undone.</p>
        <button ${clsAttr("dialog-close")} data-slot="dialog-close" type="button">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="size-4"><path d="M18 6 6 18M6 6l12 12"/></svg>
          <span class="sr-only">Close</span>
        </button>
      </div>
    </template>
  </section>
  <script src="vendor/radix-kernel-dialog.iife.js"></script>
  <script src="shadless-glue.js"></script>
</body>
</html>`
writeFileSync("probes/h4/demo.html", html)

// --- emit glue (dialog open/close on kernel wireDialog) ---------------------
const glue = `// generated: shadless dialog glue (h4 spike)
(function () {
  var trigger = document.getElementById("d1-trigger");
  var tpl = document.getElementById("d1-portal");
  var open = false, handles = null, portal = null;

  function setState(s) {
    trigger.setAttribute("data-state", s);
    trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
    var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
    var c = portal && portal.querySelector("[data-slot=dialog-content]");
    if (o) o.setAttribute("data-state", s);
    if (c) c.setAttribute("data-state", s);
  }

  function mount() {
    portal = document.createElement("div");
    portal.setAttribute("data-slot", "dialog-portal");
    portal.appendChild(tpl.content.cloneNode(true));
    document.body.appendChild(portal);
    var overlay = portal.querySelector("[data-slot=dialog-overlay]");
    var content = portal.querySelector("[data-slot=dialog-content]");

    // h3b contract: pointer-events restored on portal chain (scroll-lock sets
    // body pointer-events:none), overlay aria-hidden, trigger↔content wiring
    overlay.style.pointerEvents = "auto";
    overlay.setAttribute("aria-hidden", "true");
    overlay.setAttribute("data-aria-hidden", "true");
    content.style.pointerEvents = "auto";
    trigger.setAttribute("aria-controls", content.id);

    setState("open");
    handles = RadixKernel.wireDialog({
      content: content,
      portal: portal,
      trigger: trigger,
      onClosed: function () {
        setState("closed");
        portal.remove();
        portal = null; handles = null; open = false;
      },
    });
    // h3b contract: Close buttons inside content dismiss
    portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
      b.addEventListener("click", function () { handles && handles.close(true); });
    });
  }

  function unmount(restoreFocus) {
    if (handles) handles.close(restoreFocus !== false);
  }

  trigger.addEventListener("click", function () { open ? unmount() : mount(); });
})();`
writeFileSync("probes/h4/shadless-glue.js", glue)

console.log("IR subs:", { button: Object.keys(buttonVariants.variants), dialog: dialogSubs.length })
console.log("wrote probes/h4/{ir.json,demo.html,shadless-glue.js}")
