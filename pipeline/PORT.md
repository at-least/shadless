# The Go/JS boundary

This repo is mid-migration from JS tooling to a Go pipeline. The split is not
arbitrary and it is not "whatever has been done so far" — it is drawn by one
rule, and the tools that remain in JS remain there for reasons that were
measured, not assumed.

**A tool moves to Go when Go can produce the same bytes.**

Everything this pipeline builds is committed — `dist/`, `docs/site/`,
`src/registry/ir/` — and the `reproducible` gate rebuilds those trees and
compares them byte for byte. A port that produced *valid but different* output
would fail that gate forever, or, worse, would be "fixed" by re-recording the
committed tree and silently changing the product. So byte-identity against the
tool being replaced is the acceptance test for every port here, and where it
cannot be met the tool stays in JS.

## What is Go

| Tool | Entry point |
|---|---|
| the pipeline graph | `nodes.go` — the graph itself, not a generated copy |
| the runner | `pipeline plan/list/status/run/adopt` |
| the meta-gate + 31 mutations | `TestMetaWiring` (always) / `TestMeta` (opt-in) |
| the ledger | `TestLedger` + `pipeline ledger --record\|--render\|--dissolve` |
| the re-pin drill | `pipeline upstream` |
| the boundary audit | `pipeline audit-boundary` |
| the JS surface builder | `pipeline build-js` |
| pin, coverage, pack, dist-complete, reproducible, css-direction, consumer-sim, product-verify | `go test -C pipeline` |
| tw, oracle-css, product-css, docs-catalog, ir-diff, hooks | subcommands |

A gate is a Go **test**, never a subcommand: a gate asserts and produces
nothing, which is what a test is, and `go test` finds it by itself — so a gate
cannot go missing from a hand-maintained dispatch map. The halves that *write*
(`ledger --record`, `css-direction --update`) are subcommands, because they
mutate and assert nothing.

## What stays in JS, and why

Four toolchains have no Go equivalent that produces identical output. This is
the whole of the remainder; nothing is left in JS for lack of effort.

### `@babel/parser` — parsing TSX

`src/converter/index.mjs` (the registry `.tsx` → IR converter, the heart of the
product), `tools/rtl-lib.mjs`, `tools/five-components.mjs`, and
`gates/overlay.mjs` (its Persian-dictionary rule walks a Babel AST via
`extractTranslations(parseTs(...))`).

Go has no TypeScript/JSX parser that yields the same AST. Reimplementing the
converter would not be a port — it would be a second implementation of the
conversion rules, which is the failure mode this repo spends most of its effort
avoiding.

### `mdx` + `remark` + `shiki` — compiling the docs

`tools/docs-build.mjs`, `src/docs/components.mjs`, `src/docs/highlight.mjs`,
`src/docs/assets.mjs`, `tools/docs-upstream.mjs`.

`docs-build` runs MDX's `evaluate()` over upstream `.mdx` and renders it
through the vanilla JSX shim in `src/docs/jsx.mjs`; `shiki` highlights the
fences. There is no Go stack that emits the same HTML, and `reproducible`
compares every built page byte for byte, so a different renderer changes every
page in `docs/site/`.

This is also why the docs *gates* stay: `docs-links`, `docs-consistency` and
`docs-fidelity` are pure logic and would port cleanly on their own, but they
share `docs-guides.mjs` (`resolveDocsRoute` + the `GUIDES` table),
`transforms.mjs` and `frontmatter.mjs` with `docs-build`. Porting only the
gates would mean two implementations of those tables that must agree — exactly
what the shared-table arrangement was built to prevent.

### `jsdom` + `tailwind-merge` — emitting the product

`src/emitter/index.mjs`, `src/emitter/css.mjs`.

`tailwind-merge` resolves Tailwind's own class-conflict semantics; `jsdom`
gives the emitter a DOM to manipulate. Neither has a faithful Go port.

### `playwright` — the browser gates

`style-parity`, `demo-parity`, `path-parity`, `demo-smoke`, `docs-smoke`,
`docs-upstream`, `interactivity-sweep`, `emit-smoke`, and the oracle chain
(`oracle-lib`, `example-oracle`, `example-fixture`, `example-golden`,
`contracts/run`).

This one was attempted and backed out, and the reason is worth recording so it
is not attempted again blindly:

- the `playwright-go` releases matching this repo's playwright `1.62.1`
  (`v0.6201.0`, `v0.6201.1`) and `1.61` (`v0.6100.0`) are **unusable**: their
  `go.mod` still declares the pre-rename module path
  `github.com/mxschmitt/playwright-go`, so the Go toolchain refuses them.
- the newest release that resolves is `v0.6000.0`, which pins
  `playwrightCliVersion = "1.60.0"`.

Taking it would put **two chromium versions in one pipeline** — 1.60.0 driving
the Go gates, 1.62.1 driving the React oracle, which stays JS regardless
because it renders React. `style-parity`, `demo-parity` and `path-parity`
compare computed styles between a shadless page and that oracle; holding the
browser constant is the entire premise of those gates, and comparing across two
chromium builds would turn browser drift into recorded "dirty cells".

Contrast `esbuild`, which *was* taken: esbuild is written in Go, the npm
package ships this same implementation, and `github.com/evanw/esbuild` pins to
`v0.28.2` — the exact version `package.json` carries. The minified output is
byte-identical. That is the bar; playwright-go did not clear it.

## Content, not tooling

Separate from all of the above: some `.mjs` in this repo is not tooling at all
and will never be Go. It is the subject matter.

- `dist/esm/*.mjs` — the shipped library.
- `src/docs/highlight-client.mjs`, the pre-paint script string in
  `src/docs/theme-prepaint.mjs` — code injected into pages and run by the
  browser.
- `tools/contracts/components/*.mjs`, `tools/contracts/stubs/*.mjs` — React
  contract specimens and browser stubs; test *data*, not test *harness*.
- `probes/**` — experiments.
- `.upstream/**` — vendored shadcn.

## Reading a JS module's data from Go

Where a Go tool must agree with a JS module that cannot move, it reads that
module's declarations as **data** rather than keeping a second copy.
`jssource.go` is the shared implementation (`jsSetLiteral`, `jsObjectField`,
`jsAttrMap`, `jsFieldIsFalse`); `ledger.go` uses it for `DEAD_UTILITIES` and
`SKIN_ALLOWLIST`, and `product_css.go` predates it with the same idea for
`SHADLESS_CSS_FIXES`.

Every extractor **fails loudly** when its anchor moves. Returning an empty set
instead would let a gate pass over nothing — the vacuous-green failure mode the
meta-gate exists to catch, and the hardest one to notice, because the report
still says PASS.

## Porting the next one

1. Check the npm imports first. If any is in the four toolchains above, stop —
   the honest answer is that it does not move yet.
2. Check who imports it. A leaf cannot move alone while JS still imports it;
   either the importer moves too, or the JS consumer spawns the binary (what
   `tools/demo.mjs` does for `build-js`), or it reads the built artifact (what
   `tools/unit/runtime.mjs` does for the ESM surface).
3. Capture the JS output first, port, then **diff**. Compare against the JS
   tool's own output, not against the committed file — `gates/ledger.json`
   carried `—` escapes that neither writer produces, and comparing against
   it would have sent the port chasing a difference that was never real.
4. A gate is a `go test`. A writer is a subcommand.
5. Name the file carefully: Go gives any file ending in `_<goos>.go` an
   implicit build constraint. `build_js.go` compiles only for `GOOS=js` and is
   silently dropped — hence `jsbuild.go`.
