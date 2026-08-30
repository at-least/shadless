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

This one is permanent, and for a stronger reason than the missing Go library:
upstream authors its documentation in MDX, and mirroring that content is the
point — `docs-fidelity` exists to assert that every built page still matches
its `.mdx` source. Escaping mdx+shiki would mean giving up the mirror and
generating our own pages from the IR instead, which is a different product,
not a port. The toolchain stays JS; under Dagger it runs in a container like
everything else.

Worth separating, because the two are easy to conflate: the IR drives what is
INSIDE the preview iframes (the React-free demo pages, which load only the
compiled Tailwind CSS, shadless.js and the component's own js). MDX compiles
the page AROUND them — the prose, headings, install instructions and fenced
code. `convert` already removed React from the components; docs-build is not
converting components at all, it is compiling upstream's documentation.

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

## Why the pipeline has its own freshness, and not Go's test cache

A reasonable question is why the gates run with `-count=1` instead of letting
Go's build cache decide. Measured, on this repo:

| what a test reads | cached? | invalidates on change? |
|---|---|---|
| a fixed file **inside** the module | yes | yes |
| a fixed file **outside** the module | yes | **no — silently stale** |
| a directory it enumerates (`os.ReadDir`) | never | — |

The module root is `pipeline/`, and every input the gates read — `gates/*.json`,
`src/**`, `dist/**` — is above it. Editing `gates/ledger.json` and re-running
`TestLedger` still reports `(cached)`. So `-count=1` is load-bearing: without
it the gates would replay stale verdicts.

Moving `go.mod` to the repo root would fix the second row, but not the third,
and the third is the common case here: `ledger` enumerates
`tools/contracts/components/`, `coverage` the IR, `pack` and `dist-complete`
walk `dist/`. Those are never cached anyway. Gates that shell out (`pin` runs
git, several run node) are invisible to the cache regardless, since a
subprocess's file access is not in the testlog.

`go:embed` would close both holes — embedded content is part of the package
hash, so even "a new file appeared in that directory" changes the build — but
it cannot be used here: embed patterns may not contain `..`, so from
`pipeline/` there is no way to reach `.upstream` (129MB, 70MB of it `.git`)
without moving the module root, and embedding build OUTPUTS would be circular
— `reproducible`, `pack` and `dist-complete` must judge the tree on disk, not
a compile-time snapshot of it.

The stamp mechanism also does something Go's cache cannot: it folds each
dependency's key into its dependents', so "something upstream changed"
propagates by construction, and it covers the JS build nodes, which are
outside Go entirely.

### What was worth taking from the idea

The cache is the wrong lever, but the concern behind it was right: `inputs` in
`nodes.go` was hand-maintained and nothing compared it against reality. A glob
that misses a file the gate actually reads is a stale green — the same failure
class, arrived at from the other side.

So the runner now attaches `-test.testlogfile` to every `go test` command it
runs and checks the files the gate actually opened against its declared
`inputs` (`verify.go`, mirroring the existing undeclared-**write** check). It
costs no extra execution: the gate was going to run anyway.

It found three real holes on its first run, two of them long-standing:

- `unit` read `probes/h4/globals.css` and `dist/shadless.js` without declaring
  either.
- `TestUnitCssDirectionRealCSS` read `dist/shadless.css` — an `emit` output —
  from inside the `unit` gate, which declares neither the input nor a
  dependency on the node that builds it. The assertion belonged to the
  `css-direction` gate, which declares both; it now lives there.

The JS half works the same way through a different door. `tools/fs-record.mjs`
is loaded into every node command via `NODE_OPTIONS=--import` and records the
files the tool reads; no tool imports it and no tool knows it exists.
`NODE_OPTIONS` is inherited by child processes, so a tool that spawns another
node tool is covered with no extra wiring.

The trick there is worth writing down, because the obvious version does not
work: patching the ESM default export (`import fs from "node:fs";
fs.readFileSync = ...`) does NOT affect `import { readFileSync } from
"node:fs"`, which is how every tool here reads files — Node snapshots the
builtin's default export separately. Patching the CJS exports object through
`createRequire` DOES, because the ESM named exports are live bindings onto it.

That half found the worst hole yet, in the most important build node in the
graph. `convert` — registry `.tsx` → IR — declared nothing under `.upstream/`
at all. It relied on `pin`'s key, and `pin` hashes only `.git/HEAD`, so a
change to the registry sources it converts left it fresh. Demonstrated
directly: edit a registry `.tsx` and `pipeline status convert` still said
`fresh`. That is not a theoretical case — `overlays/upstream/*.patch` is
applied with `git apply --3way`, which modifies the working tree and leaves
HEAD alone, so the re-pin drill would apply the patch series and then skip
conversion entirely.

### What counts as covered

A file is not a violation merely because the node does not name it. The key
folds in each dependency's key, and that key already hashes the dependency's
own inputs, so anything a node in the dependency CLOSURE declares — input or
output — reaches this node transitively. `pipeline/README.md` states half of
it ("outputs of `needs` are implied"); the inputs half follows from the same
identity. Without that exemption `emit` reports 64 findings for reading
`src/registry/ir`, which `convert` produces and `emit` needs; with it, one
real finding remains. A check that cries wolf 64 times teaches people to stop
reading it.

Two exclusions are deliberate, in `fsRecordOpens`: `node_modules/` (a bundler
reads thousands of files under it, and `package-lock.json` is this repo's
accepted proxy for "the dependency set changed") and `build/` (the pipeline's
own scratch space).

### Limits

Stated in `verify.go`, and worth repeating because the gaps are where the next
stale green will come from:

- Non-node subprocesses are invisible. `pin` shells out to git, so its
  declarations are still unverified.
- A browser's reads are invisible. The playwright gates load `file://` pages;
  chromium opens them, not node, so a demo page a gate renders is not
  recorded.
- The Go side tracks `open`, not `stat`, and an `open` for WRITING looks the
  same — which is why the report names both `inputs` and `produces` as fixes.

It under-reports, which is the safe direction: it finds real undeclared access
and never invents one.

## The Dagger port

Three slices are ported and verified byte-for-byte against the committed tree:
`convert` (61 IR files), `emit` (dist/shadless.css + all 52 component pages),
and `contract` (the React oracle replayed in a containerised chromium).

The module names no tool versions. node comes from `.nvmrc`, the Go toolchain
from `pipeline/go.mod` (`toolchain` if present, else `go`), and the browser is
installed by whatever `package-lock.json` resolved, so driver and browser match
by construction. An image tag like `mcr.microsoft.com/playwright:v1.62.1` would
be a second declaration of a version the repo already states — the same drift
this port exists to remove, reintroduced by the port.

### What a container dissolves, and what it does not

`emit` used to shell out to `pipeline tw`. That wrapper does three things, and
a container hands over two of them for free: resolving `in`/`out` against the
repo root rather than the compile cwd (paths are absolute), and taking the CLI
from THIS repo's `node_modules` rather than whatever package sits above the
scratch directory (there is exactly one). Both exist to survive running at an
arbitrary cwd on a developer's machine, which containers do not do.

The third job is real and stays: controlling the compile cwd, which is what
Tailwind auto-scans for utility classes. In a container that is `WithWorkdir`,
so `emit` calls the CLI directly and the Go binary left that step entirely —
taking the `SHADLESS_ROOT` workaround with it.

`tw.go` itself stays until its remaining callers move. Three are in-process Go
calls (`oracle_css.go`, `gate_consumer_sim.go` ×2) and one graph node passes no
`--cwd` at all, meaning "fresh empty scratch dir, zero content scanning". That
last one is load-bearing, and it was measured rather than assumed:

	scratch dir (current)   345,892 bytes  == committed dist/shadless.full.css
	repo-root scan          606,137 bytes  (+260KB)

Dropping the scratch dir leaks 260KB of scanned utilities into the shipped
product CSS — a 75% increase, silently, with nothing failing except
`reproducible` catching the committed diff. Deleting `tw.go` today would
scatter that semantic across five call sites (two shell, three Go). It goes
when `oracle-css` and `consumer-sim` become container steps and their cwd
becomes a `WithWorkdir`, the way `emit`'s did.

### Still tangled

`emit` mounts host `dist/` because the Tailwind step scans it, which leaves
`dist/` as both an input and an output of that step — the mixing this port is
meant to remove. Untangling it means deciding what that scan is supposed to
see, which decides which utilities reach the shipped stylesheet. That is a
product question, not a mechanical port.

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
