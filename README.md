# shadless

React-free shadcn/ui components — HTML + Tailwind CSS + vanilla JS,
mechanically converted from the pinned shadcn-ui registry.

Every component is converted by an automated pipeline (babel AST → IR →
emit), and every converted component is proven equivalent to the React
original by machine: contract tests replay real-browser recordings of the
React source against the emitted pages, and a golden dual-hop gate proves
each shipped demo page equals both a fresh React render of the upstream
example and the live ui.shadcn.com snapshot.

## Consume

Primary path — your own Tailwind v4 build (tree-shaken to exactly the
components you import; machine-checked by the repo's consumer-sim gate):

```css
@import "shadless";            /* theme + animate layer, self-contained */
@import "shadless/button.css"; /* each component you use, one import    */
```

…plus the markup (inline utilities — your content scan picks them up) and,
where the component has behavior, its JS. No build? `shadless/full.min.css`
ships every component precompiled as the zero-setup alternative.

| Export | What it is |
|---|---|
| `shadless` | theme + animate layer (self-contained — needs only your tailwindcss) |
| `shadless/<name>.css` | per-component `@apply` source, one import per component you use |
| `shadless/full.css` / `shadless/full.min.css` | every component precompiled (no-build path) |
| `shadless/runtime` / `runtime.min` | the JS base: engine + registry + theme (+ the vendored radix kernel), auto-initialises. `<script>` gets the IIFE (`window.shadless`); `import` gets the ES module (`dist/esm/shadless.mjs` — `export default shadless` + named `init`, `get`, `theme`, …) |
| `shadless/js/<name>` | one behavior file per interactive component, registers with the base; `shadless.get(el)` returns its handle (`open()`, `close()`, `toggle()`, `isOpen()`; tabs `activate(i)`; slider `values()` / `setValue()`; carousel the embla api). Under `import` each file is a module that imports the base itself, so import order does not matter |
| `shadless/esm/<name>` | the ES-module files by explicit path (`shadless.mjs` is the base) |

```js
// bundler / <script type="module">
import shadless, { get } from "shadless/runtime"
import "shadless/js/dialog"
get("#d1-trigger").open()
```

The trivial tier (checkbox, switch, toggle, radio-group, toggle-group,
collapsible, accordion, avatar) has no handle on purpose: its state *is* the
attribute radix renders (`aria-checked`, `aria-pressed`, `aria-expanded`,
`data-state`) and `el.click()` drives it — `shadless.get(el)` returns
`null` there rather than wrap what the DOM already says.

Events (bubbling `CustomEvent`s, `detail.component` always set, `detail.api`
where the element has a handle), fired after the state change whether the
user, the keyboard, or the handle caused it — listen on the element or
delegate on `document` instead of polling:

| Event | On | `detail` |
|---|---|---|
| `shadless:open` / `shadless:close` | the trigger of every openable (dialog family, popover, tooltip, hover-card, select, menus, navigation-menu, collapsible, accordion) | — |
| `shadless:change` | checkbox / switch | `{ checked }` |
| | toggle | `{ pressed }` |
| | radio-group / toggle-group root | `{ value, item }` (`value` is the item's `value` attr or id; an array for multiple toggle-groups) |
| | tabs root | `{ index, trigger }` |
| | slider root | `{ values }` (live, every step of a drag) |
| `shadless:commit` | slider root | `{ values }` once per gesture (radix `onValueCommit`) — the value to persist |
| | select trigger | `{ value, label, item }` (`value` is the option's `value` / `data-value` attr or id, else its label) |
| `shadless:themechange` | `document` | `{ mode }` |

Forms work the way they do with radix: give a checkbox, switch, radio-group
root, select trigger or slider root a `name` attribute and the runtime keeps
hidden inputs beside it (one per value, none while unchecked / unselected)
— a plain `<form>` submits them and `form.reset()` restores the initial
state. checkbox / switch submit their `value` attribute (default `on`).

`dist/esm/shadless.d.ts` types the module entry, the handles and these
events (`GlobalEventHandlersEventMap` is augmented, so
`addEventListener("shadless:change", e => e.detail.values)` type-checks).

Theming follows upstream's own model — CSS variables on `:root` /
`.dark`. The runtime also exposes `shadless.theme.{get,set,toggle}` with
localStorage-backed theme semantics.

## Tiers

| Tier | Components | Needs |
|---|---|---|
| static | 23 | CSS only — zero JS |
| interactive | 23 (one file each in `dist/js/`) | CSS + `shadless.js` + `js/<name>.js` |
| CSS, no behavior | label, progress, separator, field | CSS only (radix renders them with no state) |

48 components ship CSS, 23 ship behavior; `aspect-ratio` and `collapsible`
carry no classes upstream, so they have JS (collapsible) or nothing to
ship (aspect-ratio) but no stylesheet. `field` is the one shipped
component outside the oracle-backed matrix (logic tier — its demo is
hand-authored; see `pipeline/gate_coverage.go`).

## Not included (recorded, not silently dropped)

calendar, chart, combobox, command, data-table, date-picker, drawer,
form, input-otp, questionnaire, resizable, sidebar, sonner, toast,
typography — these wrap React-only dependencies (react-day-picker,
recharts, cmdk, vaul, react-hook-form, …) or React-only composition with
no vanilla upstream; converting them would mean rewriting the dependency,
which is outside mechanical-conversion scope. The docs site marks them
"Not available in shadless" (the list is `GREY_COMPONENTS` in
`tools/docs-build.mjs`, cross-checked against the catalog on every build).

## Why it stays honest

Every check is a node in one graph — `pipeline/nodes.go` — and every gate
in that graph is proven able to fail (`make meta` mutates a real
artifact and requires the gate to go red). Nothing is verified by a list
that lives in two places, and nothing is trusted because it happened to be
green once.

- **Oracle = the React original.** Contract tests bundle the pinned
  registry source with real React + radix, render it in chromium, and
  replay real-mouse/keyboard scenarios against the emitted pages
  (`contracts`), then compare computed styles element by element
  (`style-parity`, per-cell ratchet).
- **Golden dual hop.** hop 1: the local React render must equal a
  committed snapshot of what ui.shadcn.com actually serves. hop 2: each
  shipped demo page must equal the React render. Both green ⇒ shipped ==
  React == live, by machine.
- **Consume-path parity.** For every slot, slot-only markup through the
  css-import path and through `full.css` must compute what React's inline
  classes compute under upstream's own stylesheet, in both themes and both
  directions — at rest, per cva variant value, and per attribute-driven
  state, with referenced child slots rendered on both sides
  (`path-parity`); every component stylesheet compiles alone
  (`consumer-sim`).
- **Reproducible.** The committed `dist/` and `docs/` must equal a fresh
  pipeline run — the only authority on hand-edited outputs.
- **Manual interventions are audited, not remembered.** Every rule table,
  hand-written behavior/runtime file and upstream patch re-proves on
  each run that it still applies to the pinned upstream
  (`gates/overlay.mjs`); accepted differences live in `gates/ledger.json`
  with a class and a budget that may only shrink.

## Develop

```sh
make            # full pipeline + every gate (= CI)
make fast       # browser-free gates, < 1s (pre-commit hook)
make meta       # prove every gate can fail
make only ID=path-parity        # one gate + exactly what it needs
make list       # the graph
```

Upstream upgrades: `make upstream TO=shadcn@X.Y.Z` — see `UPGRADING.md`.
A nightly workflow runs the same drill against the newest release and opens
a PR. Vendored engine integrity is sha-pinned in `src/registry/pin.json`.

## License

MIT — see [LICENSE](LICENSE). The vendored runtime bundles in `vendor/`
ship under their own MIT licenses; the docs-site fonts are SIL OFL 1.1
(`docs/fonts/LICENSE.txt`).
