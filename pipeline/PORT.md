# PORT — where the engine ends and the toolchains begin

> **A step moves into the Rust engine when it can produce the same bytes.**

The build engine is a single Rust crate (this directory). Everything that
must be byte-stable lives here: the graph (nodes, keys, stamps), the
converter that produces `generated/ir/`, the emitters behind `dist/`, every
gate, and the CLI (`./build/pipeline`). Byte-identity is the acceptance bar:
`reproducible` byte-compares the committed generated trees with a fresh
rebuild, so a step that cannot promise identical bytes must not move in.

## What stays external, and why

| Toolchain | Used by | Why it stays |
|---|---|---|
| the pinned `node_modules/.bin/esbuild` binary | `convert`, `rtl-dict` (Transform), `build-js` (minify) | The committed `dist/shadless.min.js` and the IR scanner are byte-pinned to esbuild's printer. Measured (probe/oxc REPORT): 0/61 inputs byte-identical from pure-Rust printers — the divergence is in codegen order/renaming, not configurable. |
| tailwind CLI | `tw` steps | The committed `dist/*.css` are tailwind's bytes. |
| playwright (chromium) | browser gates, contracts, fixture self-tests | The oracle IS React in a real browser; jsdom does not produce comparable bytes. |
| node + `tools/*.mjs` | the JS runtime surface, unit/contract harnesses, prettier batching | The product ships JS; its tests run where the product runs. |
| zola | docs site serving/build | The docs theme is a zola theme. |

The oracle's JS bundle step (example-oracle/example-fixture/contracts) has
no byte contract: it defaults to the pure-Rust rolldown bundler
(`SHADLESS_ORACLE_BUNDLER=esbuild` opts back out per run; `--no-default-features`
builds the lean esbuild-only binary).

## History

This engine is a byte-for-byte port of the Go pipeline that used to live in
this directory (29k lines, 41 nodes / 24 gates). The last dual-engine state
— every artifact, CLI verdict, and gate verdict byte-compared green — is
tagged `go-parity-final`; the Go module's last state is tagged
`go-engine-final` (check that tag out to regenerate Go-era fixtures). The port's design notes and the record of every bug the
byte-parity process caught are in [PROGRESS.md](PROGRESS.md) and
[PLAN.md](PLAN.md).
