# pipeline — the graph runner

One rule:

    key(n) = H( n.id, n.run, contents of every file n declares as input,
                key(d) for every d in n.needs )

A node runs iff its recorded key differs from its computed key. Because each
dependency's key is folded into its dependents', "something upstream changed"
propagates by construction — there is no second mechanism, and no special case
for the shadcn pin (it is just a file that some node declares as an input).

    pipeline plan   <tier|node…>          the closure, topologically sorted
    pipeline list   <tier|node…>          the closure, annotated, no execution
    pipeline status <tier|node…>          fresh / STALE / NEVER-FRESH per node
    pipeline run    [--force] <tier|node…>  run the stale ones, stamp on success
    pipeline adopt  <tier|node…>          record current keys WITHOUT running

A target is a tier (`fast|medium|full`), `builds`, `all`, or node ids;
`--gates-only` / `--builds-only` filter the resolved plan and `--keep-going`
runs past the first red, writing `build/gates/run-report.json` (which the
re-pin drill reads to classify each failure).

**Which tools are Go and which are still JS — and why — is [PORT.md](PORT.md).**
Read it before porting anything else: the four toolchains that cannot move are
listed there with the measurements behind each.

    PIPELINE_PARALLEL=<n>       concurrency (default: NumCPU)
    PIPELINE_FAILURES=continue  keep going past a failed node
    PIPELINE_VERBOSE=1          print each node's output even when it passes

Independent nodes run concurrently; a node is dispatched once everything it
`needs` has finished, and its key is computed at that moment — the key folds in
its dependencies' keys, which are only final once they have run. Each node's
output is buffered and printed as one block, so parallel logs stay readable.

`adopt` asserts the tree already is what the pipeline would produce. It is
only valid immediately after a green full run; it exists to migrate onto an
already-built tree and nothing else should use it.

Stamps are written only after every command of a node exits 0, so a killed or
failed run leaves that node stale instead of claiming work it did not do.

A node with `inputs: null` judges state outside the tree (`reproducible`). It
has no key, can never be skipped, and neither can anything downstream of it.

## Status

This IS the runner. wireit, `gates/wireit.mjs`, the generated `wireit` block in
package.json and the `wireit-sync` gate are gone; `make build/fast/medium/only`
and CI go through this binary. `nodes.go` is now the graph itself rather than a
copy of one, so the sync guards (`wireit-sync`, then `pipeline-sync`) are gone
with the second copy they were guarding.

Reconciled against wireit before the switch:

- `plan` was byte-identical to the JS registry for all 3 tiers and all
  41 nodes (44 targets) before that registry was retired.
- On a green tree, exactly one node is not fresh: `reproducible`, which is
  the only `inputs: null` node — matching wireit's "Ran 1, skipped 40".
- `plan` stays equivalent after the fan-out: collapsing `contracts:*` back to
  `contracts` reproduces the registry's plan for all 44 targets.
- Propagation, checked against the graph: touching `src/emitter/skin.mjs`
  marks convert + its whole downstream stale, plus `unit` and `ledger`
  (which declare `src/**` directly). Touching `pipeline/gate_coverage.go` marks
  `coverage` and nothing else.
- Whole-graph `status` costs 0.2s.

`run` also checks, after each node, that it wrote nothing outside its declared
`produces` — narrowed to the files some node declares as an input, since a
write nobody reads cannot affect freshness. wireit is structurally blind to
this: it manages declared outputs and cannot see the others. No violation has
been found yet; the check is the guarantee, not a fix for a known bug. It
compares the input universe before and after a node runs, which only means
anything when nothing else is writing, so it is enforced at `-j1` and skipped
above it with a note.

Measured: forcing the medium tier (7 nodes) takes 4.7s at -j1 and 2.4s at -j8,
tree byte-identical afterwards. A warm `run full` is 0.8s — one node
(`reproducible`) and 40 skipped.

## The gates are Go tests

A gate asserts and produces nothing, which is what a test is. The ported
gates live in `pipeline/gate_*.go` and are entered from `pipeline/gates_test.go`:

    go test -C pipeline -count=1 -v                     every ported gate
    go test -C pipeline -count=1 -v -run '^TestPack$'   one gate

`-count=1` is load-bearing rather than a habit: the test cache keys on the
package's own sources, not on `dist/` or `docs/site/`, so a cached verdict
would be replayed over a tree that has since changed. The registry's `goTest()`
helper is the only place that argv is written, so no node can forget it. `-v`
is what surfaces the gate's own PASS line instead of a bare `ok`.

Why tests rather than the `pipeline gate <name>` subcommand they replaced: that
subcommand dispatched through a hand-maintained `map[string]gateFn` populated
by `init()`. That map was a SECOND registry of gates that the compiler never
checked — a ported gate that nobody added to it was simply absent, and nothing
said so. `go test` enumerates them from the toolchain instead.

Only gates that are pure file and process I/O are ported. A gate that imports a
module from `src/` reads the pipeline's own definitions as data; porting one
would mean a second implementation of an emitter or converter rule, and two
implementations that must agree is the failure mode this repo spends most of
its effort on. Those stay in JS.

## What is in Go, and what is not

Builds are subcommands (they produce artifacts); gates are Go tests (they
assert and produce nothing):

    pipeline tw <in> <out> [--minify] [--cwd DIR]   was tools/tw.mjs
    pipeline oracle-css                             was gates/oracle-css.mjs
    pipeline product-css                            was tools/product-css.mjs
    pipeline docs-catalog                           was tools/docs-catalog.mjs
    pipeline ir-diff <ref>|<a> <b> [--json]         was gates/ir-diff.mjs
    pipeline pin [--check-only]                     was gates/pin.mjs

    TestPin TestDistComplete TestPack TestCoverage TestReproducible
    TestCssDirection TestProductVerify TestConsumerSim

A tool's unit tests move with it (`TestUnit*`), so a JS test never stands over
a Go function; the `unit` node runs both suites.

The line is not "how hard is it" — it is whether porting would create a SECOND
implementation of something. Everything above is pure file and process I/O over
data. What stays in JS, and why:

- **Reads the pipeline's own definitions from `src/`.** `ledger`, `overlay`,
  `path-parity`, `docs-consistency`, `docs-fidelity`, `resolve-skins`,
  `upstream-snapshot`. These import the emitter/converter/transform modules and
  judge their output; a Go port would have to restate an emitter rule, and two
  implementations that must agree is the failure mode this repo spends most of
  its effort on.
- **Needs a Node-only runtime.** playwright (`demo-parity`, `style-parity`,
  `demo-smoke`, `docs-smoke`, `docs-upstream`, `example-oracle`,
  `example-fixture`, `example-golden`, `interactivity-sweep`, `contracts`),
  esbuild + React + radix (`oracle-lib`, `contracts/oracle-build`, `build-js`,
  `demo`, `docs-build`), `@babel/parser` (`build-demo`, `rtl-lib`),
  mdx/remark/shiki (`docs-build`, `docs-upstream`).
- **Shares a table with a JS consumer.** `docs-links` resolves `/docs/…` routes
  through `resolveDocsRoute` + `GUIDES` in `tools/docs-guides.mjs`, which
  `docs-build` also imports to rewrite and grey those same links. One table, no
  drift, is worth more than one more Go node.


Byte-identity is the acceptance test where an output is committed:
`docs/catalog.json` regenerated by the Go port is byte-identical to the JS
one, stdout included, and `ir-diff` matches its predecessor line for line on a
synthetic IR set exercising every change kind.

## Fan-out

`fanout.go` may give the runner a FINER view than the registry: a node whose work is really N independent jobs becomes N nodes, each
with its own key. Only legal when the jobs are genuinely independent —
separate outputs, no shared mutable state, each failing on its own. It lives in
Go rather than the generator so the split stays live: adding a contract def
adds a node without regenerating anything.

`contracts` is the one that pays for the exercise. It was a single node running
29 components serially, 46% of the full tier, already spawning a child process
per component:

    serial, 1 node (wireit)     350.9s
    29 nodes, -j8                55.0s     6.4x

Per-component durations sum to 353s, matching the serial baseline, so the win
is scheduling and not a measurement error; the longest single component
(dropdown-menu, 24.7s) is the floor.

The win is parallelism ONLY. Editing one contract def still invalidates all 29,
and correctly so: `contract-fixture` declares `tools/contracts/components/**`
because it generates the kernel fixtures from every def, and `contracts:*`
needs demo-css -> demo -> contract-fixture. The narrow input set on each
`contracts:<component>` node does not help while that edge exists — the cascade
arrives through the dependency key, which is exactly what the one rule is
supposed to do.

## Cold clones

wireit carries a content-addressed output cache (local + GitHub Actions) so a
fresh checkout does not rebuild everything. This runner needs a much smaller
mechanism, because almost every output here is already committed — dist/,
docs/, src/registry/ir are in git. What a clone lacks is not the outputs but
the record of which inputs produced them, so `pipeline/stamps.json` is TRACKED.

A stamp is safe to commit because it is verified rather than trusted: the key
hashes the actual contents of every declared input, so it cannot match a tree
whose sources differ, and `reproducible` — which never goes fresh — is the
backstop against a committed output that does not match its inputs.

The second half is that a matching key is necessary but not sufficient: the
node's declared outputs have to still exist. On a clean checkout every stamp
matches while the gitignored half of the outputs is absent, and checking the
key alone would skip the work and hand the next node an empty directory.

Measured, `build/` deleted with everything else committed — 16 of 41 nodes run
instead of 41 (the check that catches this is `OutputsPresent`), and the two most expensive builds are not among them:

    convert     STALE (output missing: build/resolved-ui)     0.4s
    emit        STALE (output missing: build/emit)            1.0s
    demo-rtl    STALE (output missing: build/rtl-langs.json)  0.1s
    oracle-css, the three parity gates, the docs chain, …     (inputs gone)

    example-oracle   fresh   (105s saved — its pages are committed)
    example-fixture  fresh   (~300s saved — same)
    contracts:*      fresh   (29 nodes)

Two nodes used to declare a scratch directory as an output — `build/example-oracle`
is empty after every run and `build/example-fixture` is a `const TMP` — which
made both look unbuildable on a clean checkout. Removing those declarations is
what moves them into the skipped column.

`nodes.go` is the source of truth. Nothing generates it and nothing else
declares the graph, so there is no drift to guard against.

## Where the definitions live

`pipeline/nodes.go` — the graph, hand-maintained. It was generated from
`gates/registry.mjs` while wireit executed the graph and JS was the real
runner; both are gone, and the generator went with them.

They are Go source, not JSON, on purpose. JSON arrives through
`encoding/json`, which silently ignores unknown fields and zero-fills missing
ones — so the 41 node declarations, the part most worth checking, would be the
one part no type ever saw. As typed literals with `Needs []NodeID` referencing
generated constants, a typo in a dependency is a compile error:

    ./nodes.go:89:22: undefined: NDemoCsss

What used to protect against a node going quietly missing was the generator.
What protects against it now is `TestMetaWiring`: every gate must carry a `Why`
and at least one mutation that is proven to make it fail, so a node that is
half-declared does not compile or does not pass.
