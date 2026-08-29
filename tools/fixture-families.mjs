// The kernel-tier behavior protocol — ONE table, read by:
//   tools/example-fixture.mjs   to generate every interactive demo and
//                               contract fixture that ships (so the protocol
//                               is proven against the React oracle on every run)
//   tools/docs-build.mjs        to tell a consumer, on each component's
//                               Installation section, exactly how to author
//                               the markup the glue wires
//
// Every entry: which component file (dist/js/<js>.js), how an instance is opened, and where the
// content lives. `<k>` is any prefix (d1, menu-account, …) — the trigger's
// id names the instance and the template is found from it.
export const FAMILY = {
  "alert-dialog": { js: "alert-dialog", kind: "dialog" },
  "dialog": { js: "dialog", kind: "dialog" },
  "sheet": { js: "sheet", kind: "dialog" },
  "popover": { js: "popover", kind: "portal", open: "click" },
  "tooltip": { js: "tooltip", kind: "portal", open: "hover" },
  "hover-card": { js: "hover-card", kind: "portal", open: "hover" },
  "tabs": { js: "tabs", kind: "inline" },
  "slider": { js: "slider", kind: "none" },
  "scroll-area": { js: "scroll-area", kind: "none" },
  "dropdown-menu": { js: "dropdown-menu", kind: "menu", open: "click", attr: "data-radixuigo-menu-trigger" },
  "context-menu": { js: "context-menu", kind: "menu", open: "contextmenu", attr: "data-radixuigo-context-trigger" },
  "menubar": { js: "menubar", kind: "menu", open: "click", attr: "data-radixuigo-menu-trigger" },
  "select": { js: "select", kind: "select" },
  "carousel": { js: "carousel", kind: "none" },
  "navigation-menu": { js: "navigation-menu", kind: "nav" },
}

// Consumer-facing description of the protocol, per family. Generated into
// the docs so it cannot drift from the table above.
export function protocolMdx(comp) {
  const f = FAMILY[comp]
  if (!f) return ""
  const t = `${comp}-trigger`, c = `${comp}-content`
  const gesture = f.open === "hover" ? "hovering" : f.open === "contextmenu" ? "right-clicking" : "clicking"
  const rows = []
  if (f.kind === "dialog" || f.kind === "portal") rows.push(
    [`\`<… data-slot="${t}" id="<k>-trigger">\``, `opens on ${gesture}; \`<k>\` names the instance`],
    [`\`<template id="<k>-portal">\``, `holds the overlay/content subtree (\`data-slot="${c}"\` …) that the glue mounts into \`<body>\` while open`],
  )
  if (f.kind === "menu") rows.push(
    [`\`<… data-slot="${t}" id="<k>-trigger" ${f.attr}="<k>">\``, `opens on ${gesture}`],
    [`\`<template id="<k>-tpl">\``, `holds the \`data-slot="${c}"\` subtree`],
    [`\`<… data-slot="${comp}-sub-trigger" id="<k>s0-trigger" data-radixuigo-menu-subtrigger="<k>s0">\``, `a sub menu inside a layer; its own \`<template id="<k>s0-tpl">\``],
  )
  if (f.kind === "select") rows.push(
    [`\`<button data-slot="${t}" id="<k>-trigger">\``, `opens on click / Enter / Space / arrows; the \`data-slot="select-value"\` child shows the selection`],
    [`\`<template id="<k>-tpl">\``, `holds the \`data-slot="${c}"\` listbox subtree`],
  )
  if (f.kind === "nav") rows.push(
    [`\`<… data-slot="${t}" id="<k>-trigger" data-radixuigo-nav-trigger="<k>">\``, `opens on click`],
    [`\`<template id="<k>-content-tpl">\``, `holds the \`data-slot="${c}"\` subtree; the glue creates the shared viewport inside the root`],
  )
  if (f.kind === "inline") rows.push(
    [`\`<div data-slot="tabs">\` with \`data-slot="tabs-trigger" aria-controls="<panel-id>"\` and \`data-slot="tabs-content" id="<panel-id>"\``, `no template: every panel is in the markup, inactive ones \`hidden\`; the glue wires every root it finds`],
  )
  if (f.kind === "none") rows.push(
    [`\`<… data-slot="${comp}">\``, `no ids, no templates: the glue wires every root it finds`],
  )
  const api = f.kind === "inline" ? `\`shadless.get(rootEl)\` → \`activate(i)\`, \`active()\`; the root dispatches \`shadless:change\` (\`detail: { index, trigger }\`)`
    : f.kind === "none" && comp === "slider" ? `\`shadless.get(rootEl)\` → \`values()\`, \`setValue(value, index)\`; the root dispatches \`shadless:change\` (\`detail: { values }\`, live) and \`shadless:commit\` (once per gesture)`
    : f.kind === "none" && comp === "carousel" ? `\`shadless.get(rootEl)\` → the embla api (\`scrollNext()\`, \`scrollTo(i)\`, \`on("select", …)\`)`
    : f.kind === "none" ? null
    : `\`shadless.get("#<k>-trigger")\` → \`open()\`, \`close()\`, \`toggle()\`, \`isOpen()\`${f.kind === "select" ? ", `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger also dispatches `shadless:change` (`detail: { value, label, item }`). An option's value is its `value` / `data-value` attribute or id — React's value prop never reaches the DOM, so add `data-value` to options whose value differs from their label" : ""}`
  return `
**Behavior protocol**

The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.

| Markup | Meaning |
| --- | --- |
${rows.map(([a, b]) => `| ${a} | ${b} |`).join("\n")}

Content that React would render inside the component's portal lives in the \`<template>\`; the glue clones it into \`<body>\` while open and removes it on close, exactly as radix mounts and unmounts.
${api ? `\n**From code:** ${api}. \`shadless.get\` accepts an element or a selector and walks up from any element inside the instance.${f.kind === "inline" || f.kind === "none" ? "" : ` The trigger dispatches \`shadless:open\` / \`shadless:close\` (bubbling, \`detail: { component, api }\`) on every open and close, however it was caused.`}\n` : ""}`
}

// The trivial tier — delegated behaviors with no handle: state IS the
// attribute radix renders, el.click() drives it. Documented from this table
// on each page (Installation → Behavior, and the API Reference surface) so
// the docs cannot drift from src/runtime/components/<name>.js.
export const TRIVIAL = {
  checkbox: {
    state: "`role=\"checkbox\"` root with `aria-checked` + `data-state=\"checked|unchecked\"`; the `checkbox-indicator` mounts from a `<template data-for=\"checkbox-indicator\">` while checked (radix Presence)",
    events: "the root dispatches `shadless:change` (`detail: { checked }`)",
    form: "a `name` attribute submits its `value` (default `on`) while checked",
    keys: "Space / click toggles",
  },
  switch: {
    state: "`role=\"switch\"` root with `aria-checked` + `data-state`; the `switch-thumb` mirrors `data-state`",
    events: "the root dispatches `shadless:change` (`detail: { checked }`)",
    form: "a `name` attribute submits its `value` (default `on`) while checked",
    keys: "Space / click toggles",
  },
  toggle: {
    state: "`aria-pressed` + `data-state=\"on|off\"` on the root",
    events: "the root dispatches `shadless:change` (`detail: { pressed }`)",
    keys: "Space / click toggles",
  },
  "radio-group": {
    state: "`role=\"radiogroup\"` root; items are `role=\"radio\"` with `aria-checked` + `data-state`, the checked one carries the `radio-group-indicator` mounted from `<template data-for=\"radio-group-indicator\">`; an item's value is its `value` / `data-value` attribute or id",
    events: "the root dispatches `shadless:change` (`detail: { value, item }`)",
    form: "a `name` attribute on the root submits the checked item's value",
    keys: "arrows / Home / End move focus over enabled items (rtl-aware); with nothing checked an arrow also checks its target (radix)",
  },
  "toggle-group": {
    state: "`role=\"group\"` root; single mode items are `role=\"radio\"` with `aria-checked`, multiple mode items carry `aria-pressed`; `data-state=\"on|off\"` in both",
    events: "the root dispatches `shadless:change` (`detail: { value, item }` — `value` is the on item's value, or an array in multiple mode)",
    keys: "arrows / Home / End move focus over enabled items (rtl-aware); Space / Enter / click selects",
  },
  accordion: {
    state: "root `data-type=\"single|multiple\"`; each `accordion-trigger` carries `aria-expanded` + `data-state=\"open|closed\"` and `aria-controls` → its `accordion-content`, which is `hidden` while closed (kept in the DOM — radix unmounts it)",
    events: "each trigger dispatches `shadless:open` / `shadless:close` (a sibling closed by single mode gets its own `close`)",
    keys: "arrows / Home / End move focus between triggers (rtl-aware); Enter / Space / click toggles",
  },
  collapsible: {
    state: "`collapsible-trigger` carries `aria-expanded` + `data-state` and `aria-controls` → the `collapsible-content`, `hidden` while closed",
    events: "the trigger dispatches `shadless:open` / `shadless:close`",
    keys: "Enter / Space / click toggles",
  },
  avatar: {
    state: "`avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup",
  },
}

export function trivialMdx(comp) {
  const t = TRIVIAL[comp]
  if (!t) return ""
  return `
**Behavior**

Delegated from the base — no handle (\`shadless.get(el)\` is \`null\`): the state is the attribute radix renders and \`el.click()\` drives it. ${t.state}.${t.keys ? ` Keys: ${t.keys}.` : ""}
${t.events ? `\n${t.events[0].toUpperCase()}${t.events.slice(1)}, bubbling, after the state change, whichever path caused it.` : ""}${t.form ? `\n\nForms: ${t.form}; \`form.reset()\` restores the initial state.` : ""}
`
}

// The API Reference surface for a component page: its data-slot vocabulary
// (from the IR that drives the emitted markup) plus the runtime contract —
// the upstream section links the React prop tables, which are not this
// product's API.
export function apiReferenceMdx(comp, slots) {
  const t = TRIVIAL[comp], f = FAMILY[comp]
  const rows = slots.map((s) => `| \`data-slot="${s}"\` |`)
  const slotTable = rows.length ? `\n| Slot |\n| --- |\n${rows.join("\n")}\n` : ""
  let runtime = ""
  if (t) runtime = `\n**Runtime:** ${t.state}.${t.keys ? ` Keys: ${t.keys}.` : ""}${t.events ? ` ${t.events[0].toUpperCase()}${t.events.slice(1)}.` : ""}${t.form ? ` Forms: ${t.form}.` : ""} No handle — \`shadless.get(el)\` returns \`null\`; \`el.click()\` is the driver.\n`
  else if (f) {
    const api = f.kind === "inline" ? "`activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`)"
      : comp === "slider" ? "`values()`, `setValue(value, index)`; the root dispatches `shadless:change` (live) and `shadless:commit` (once per gesture) with `detail: { values }`; a `name` attribute submits one input per thumb"
      : comp === "carousel" ? "the embla api (`scrollNext()`, `scrollTo(i)`, `on(\"select\", …)`)"
      : comp === "scroll-area" ? null
      : `\`open()\`, \`close()\`, \`toggle()\`, \`isOpen()\`${f.kind === "select" ? ", `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger dispatches `shadless:change` (`detail: { value, label, item }`); a `name` attribute submits the selected value" : ""}; the trigger dispatches \`shadless:open\` / \`shadless:close\``
    runtime = api ? `\n**Runtime:** \`shadless.get(el)\` (element or selector, any element inside the instance) → ${api}. Markup protocol: see Installation → Behavior protocol.\n` : `\n**Runtime:** wired from \`data-slot\` alone — no handle, no events; see Installation → Behavior protocol.\n`
  }
  if (!slotTable && !runtime) return ""
  return `\n**shadless surface** — every node is a \`data-slot\` attribute in the shipped markup; state lives in the attributes radix renders (\`data-state\`, \`aria-*\`), never in classes.\n${slotTable}${runtime}`
}
