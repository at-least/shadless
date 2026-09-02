# Contributing

shadless is a mechanical conversion of the pinned shadcn-ui registry. The
rule that shapes every change: **nothing is verified by a list that lives
in two places, and nothing is trusted because it was green once.** Every
check is a node in `pipeline/nodes.go`, every gate is proven able to fail
by `make meta`, and every accepted difference from upstream is a
ledger entry with a reason.

## Setup

```sh
npm ci
npx playwright install --with-deps chromium   # the full tier renders in chromium
npm run pin                                   # auto-clones the pinned upstream into .upstream/ on first run
```

## The loop

| Command | What | When |
|---|---|---|
| `make fast` / `npm test` | pin, unit, ledger, dist-complete, pack — seconds, no browser | every commit (the hook runs it) |
| `make` | the full pipeline + every gate, ending in `reproducible` — what CI runs | before a PR |
| `make meta` | mutation-test every gate (each must go red on its declared mutation) | when you add or change a gate |
| `make only ID=<node>` | one node and exactly what it needs | iterating on one gate |
| `make list` | the graph | |

`reproducible` byte-compares the committed generated trees (`dist/`,
`generated/ir/`, `docs/catalog.json`, `docs/demos/`, …) with what the
pipeline just produced, so **commit regenerated outputs with the source
change that caused them**.
After touching `src/runtime/**`: `node tools/build-js.mjs` → `npm run
demo` → `npm run docs`, in that order (the emitter wipes the interactive
demo pages; only the full demo build restores them).

## Where changes go

- **Conversion rules** — `pipeline/convert.go`, `src/emitter`: mechanical, driven
  by the IR in `generated/ir/`. A manual intervention on top of the
  conversion is an *overlay* (`overlays/`, audited by `make overlay`), never
  a hand edit of `dist/`.
- **Runtime** — `src/runtime/core.js` (engine, registry, helpers, forms,
  theme) + `src/runtime/components/<name>.js` (one behavior per component,
  radix semantics as measured from the oracle). Types in
  `src/runtime/shadless.d.ts`; the unit gate compiles them with `tsc
  --strict`. Behaviors are unit-tested in jsdom (`tools/unit/runtime.mjs`)
  and contract-tested against real React in chromium
  (`tools/contracts/components/`).
- **Docs** — generated from the upstream mdx by `tools/docs-build.mjs`; the
  runtime protocol text comes from `tools/fixture-families.mjs` (the same
  tables that generate the fixtures). Hand-authored demos live in
  `docs/demos/`.
The Go/JS split in this repo is deliberate and documented in
[pipeline/PORT.md](pipeline/PORT.md); a tool moves to Go when Go can produce
the same bytes, and the four toolchains that cannot are listed there.

- **A new gate** — a node in `pipeline/nodes.go` with a `Why`, at least one
  mutation under `gates/mutations/` that makes it fail, and a tier. `make
  meta` rejects anything less.
- **An accepted difference from upstream** — `gates/ledger.json` (never a
  scattered flag); `make ledger-render` regenerates `EXEMPTIONS.md`.
  Budgets may only shrink.

## Pull requests

- One concern per commit; the message says *why* (the incident or the
  upstream fact), not what the diff already shows.
- CI runs the full tier and the meta gate on every PR. A red
  `reproducible` means a generated tree was not regenerated — rebuild and
  commit it.
- Re-pins are automated: the nightly drill opens one PR per upstream
  release with the regenerated trees and a classified report.
