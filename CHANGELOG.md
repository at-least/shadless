# Changelog

All notable changes to the npm surface and the runtime contract. The pinned
upstream is `src/registry/pin.json`; re-pins land through the upstream drill
(`pipeline upstream --to=shadcn@X.Y.Z`) and are listed here by tag.

## Unreleased

### Changed
- Docs: the code tab under every non-RTL preview is **slot-only markup** —
  the `data-slot`/`data-*` attributes plus only the classes the component
  stylesheet cannot reproduce on its own (the example's own layout/size
  choices, group/peer markers, and the few supplied classes whose inline
  cascade position matters). Copy the
  tab, import `shadless` and the component's stylesheet, and you get exactly
  the preview; customize by adding utilities (they sit in the utilities
  layer and win over the component styles) or by setting the documented
  `data-*` attributes. The preview iframe keeps rendering the full inline
  DOM shadcn's React produces (`docs/demos/*.html`), and RTL examples keep
  every class in their tabs — their logical-direction utilities exist only
  inline. Class tokens on the 3,549 slotted elements of the projected demos
  drop from 50,929 to 13,096; 46% of them carry no class at all.

### Added
- `shadless-check`, a static, offline, zero-dependency markup checker for the
  slot-only contract, plus the artifact it checks against:
  `shadless/vocabulary.json` — generated from the IR by the same pipeline run
  that emits `dist/css` (every `data-slot` of every shipped component,
  the `data-<axis>` values and default each slot accepts from its cva table,
  the marker classes every render of a slot carries — group/peer markers and
  the skin classes a shipped stylesheet rule keys on). `npx
  shadless-check <file|dir>...` flags a `data-variant`/`data-size` value the
  slot does not list, `data-variant`/`data-size` on an element with no
  `data-slot`, a duplicated attribute on one start tag, an unknown
  `data-slot` (warning — authoring hooks exist), and a missing required
  marker class (warning); exit 1 on errors (warnings with `--strict`), 2 on
  usage errors. It checks the cva axes listed in `vocabulary.json`; hook-only
  attributes whose full value set no table names (e.g. `card data-size`) are
  deliberately not axes.
- `docs-code-parity`, a full-tier browser gate: for every built page and
  every ` ```text,name=… ` code tab — RTL tabs included, which keep every
  class and are compared against their `-rtl` demo sources — the committed
  tab text must render element-for-element identically to the raw demo
  markup it was projected from — zero diffs, no baseline — under the
  precompiled `dist/out.css` AND under a fresh consumer compile
  (`@import "shadless"` + every component stylesheet, Tailwind's content
  scan restricted to each side's own text, proven by a control compile).
  Both themes and directions, 28 computed properties per element. The
  element walk also covers `<template>` contents (the portal DOM), which
  do not render and are therefore asserted structurally (element counts
  and tag sequences) rather than by computed style.

### Fixed
- Docs: the copy-source formatting no longer reflows text inside
  white-space-sensitive subtrees. Prettier's html printer cannot see the
  `whitespace-pre*` utilities a class list carries, so its rewrap and
  element gaps rendered as extra line boxes inside any `pre`/`pre-line`/
  `pre-wrap` subtree — the bubble-collapsible tab rendered 300px tall
  against the preview's 209px. The generator now formats such subtrees
  the one way prettier leaves byte-for-byte (as `<pre>` content) and
  restores the real tags on its output; the shipped tab carries no trace
  of the detour.
- Docs: the copyable code tab under every preview no longer carries HTML
  entity escapes inside `class` attributes. The tab's source is the demo
  page's browser serialization, which ships `[&amp;_svg]` and
  `has-[&gt;svg]` for what the DOM holds as `[&_svg]` and `has-[>svg]` —
  legal HTML that rendered fine, but a Tailwind content scan extracts
  nothing from an escaped candidate, so markup copied out of the tab
  silently lost every `[…&…]`-shaped utility (measured on tailwindcss
  4.3.3: the copied accordion demo produced zero `underline-offset`
  rules). The generator now unescapes `&amp;`/`&gt;`/`&lt;` in class
  values only (`&amp;` stays escaped where the decoded `&` would start a
  character reference); every other attribute and text node is untouched,
  and every built tab parses to the identical DOM it parsed to before.
- `package.json`: React and the conversion toolchain moved from
  `dependencies` to `devDependencies` — installing shadless no longer
  installs React 19. `tailwindcss` is an optional peer.
- `shadless/js.min` resolves to the one ES-module base under `import`.
  It briefly pointed at a separate `dist/esm/shadless.min.mjs`, but every
  `dist/esm/<name>.mjs` hardcodes `import "./shadless.mjs"` and registers on
  the global, so importing the minified base beside any component gave two
  instances and `shadless.get()` returned `null` forever. The minified ESM
  base is no longer built; `<script>` still gets `dist/shadless.min.js`.
- The tarball carries the product surface only (`dist/css`, `dist/js`,
  `dist/esm`, the entry stylesheets and runtimes) — no demo pages, oracle
  stylesheets or the retired `dist/glue/` tree.
- Select: `shadless:change` `detail.value` and `SelectHandle.value()` are
  the option's value (`value` / `data-value` attribute or id, else its
  label) — they were the label. `detail.label`, `label()` and `selected()`
  added.
- `destroy(root)` tears down kernel-family wiring (element listeners, open
  portals, handles), so `init(root, { force: true })` really re-initializes;
  wiring the vendored kernel holds with no unwire (tabs, slider) is reused
  rather than doubled. Tabs no longer emit `shadless:change` on wire or on
  a same-index activate.
- Roving focus (radio-group, toggle-group, accordion, tabs) skips disabled
  items — a radio-group with nothing checked could check a disabled item —
  and swaps the horizontal arrows under `dir="rtl"`.

### Added
- Docs: every component page's API Reference embeds the component's
  stylesheet (`shadless/<name>.css`) verbatim in a `css` fence, introduced
  as the authoritative default look of each slot — every rule keys on the
  `data-slot` / `data-*` attributes the tables above list, so a reader (or
  an LLM agent) can see what each `data-slot` does by default without
  opening the package. The embed is checked byte-exact against
  `dist/css/<name>.css` by `docs-consistency`, and code tabs are checked
  for re-escaped class attributes.
- Forms: a checkbox, switch, radio-group root, select trigger or slider
  root carrying a `name` attribute keeps hidden inputs beside it (like
  radix's BubbleInput); a plain `<form>` submits them and `form.reset()`
  restores the initial state.
- Types: handles carry `component` (a discriminant), `shadless.h` is typed
  (`Helpers`), `window.shadlessNoAutoInit` is declared; the declarations
  are compiled by `tsc --strict` through the real exports map in the unit
  gate.
- Gates: `pack` (fast tier) — exports resolve to files in the tarball,
  README specifiers resolve through the exports map, `dependencies` stays
  empty. `npm test` runs the fast tier.
- Docs: every interactive page documents its runtime (events, handle or
  the no-handle contract, form `name`); `API Reference` carries the
  shadless surface (data-slot vocabulary + runtime) beside the Radix link.

### Changed
- The declared Tailwind floor is now a machine-checked one:
  `peerDependencies.tailwindcss` says `^4.3.0` (was `^4`) and the
  `consumer-sim` gate compiles every component stylesheet on 4.3.0 (the
  `@tailwindcss/cli-floor` dev alias) as well as on the dev toolchain. The
  shipped `@apply` sources use `scrollbar-none` and `scrollbar-thin`,
  built-ins no tailwindcss line before 4.3.0 has, so the old range promised
  compatibility with tailwindcss 4.0–4.2 that the CSS never had.
  The gate fails if the peer range drifts off the floor toolchain; raising
  the floor means bumping the peer range and the alias together.

## 1.0.0 — pinned to `shadcn@4.19.0`

Initial release: 48 components with CSS, 23 with behavior, converted
mechanically from the pinned registry and proven against the React
original by the gate graph in `pipeline/nodes.go`. Not included (React-only
dependencies): calendar, chart, combobox, command, data-table,
date-picker, drawer, form, input-otp, questionnaire, resizable, sidebar,
sonner, toast, typography.
