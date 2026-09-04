# Changelog

All notable changes to the npm surface and the runtime contract. The pinned
upstream is `src/registry/pin.json`; re-pins land through the upstream drill
(`pipeline upstream --to=shadcn@X.Y.Z`) and are listed here by tag.

## Unreleased

### Fixed
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

## 1.0.0 — pinned to `shadcn@4.19.0`

Initial release: 48 components with CSS, 23 with behavior, converted
mechanically from the pinned registry and proven against the React
original by the gate graph in `pipeline/nodes.go`. Not included (React-only
dependencies): calendar, chart, combobox, command, data-table,
date-picker, drawer, form, input-otp, questionnaire, resizable, sidebar,
sonner, toast, typography.
