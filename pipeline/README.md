# pipeline — the graph runner

One rule:

    key(n) = H( n.id, n.run, contents of every file n declares as input,
                key(d) for every d in n.needs )

A node runs iff its recorded key differs from its computed key. Because each
dependency's key is folded into its dependents', "something upstream changed"
propagates by construction — there is no second mechanism, and no special case
for the shadcn pin (it is just a file that some node declares as an input).

    pipeline plan   <tier|node…>          the closure, topologically sorted
    pipeline status <tier|node…>          fresh / STALE / NEVER-FRESH per node
    pipeline run    [--force] <tier|node…>  run the stale ones, stamp on success
    pipeline adopt  <tier|node…>          record current keys WITHOUT running

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

Not the runner yet — wireit still executes the graph. This binary is being
reconciled against it first, because a migration that silently drops a node
would silently drop a gate.

Reconciled so far:

- `plan` is byte-identical to `gates/registry.mjs` for all 3 tiers and all
  41 nodes (44 targets).
- On a green tree, exactly one node is not fresh: `reproducible`, which is
  the only `inputs: null` node — matching wireit's "Ran 1, skipped 40".
- `plan` stays equivalent after the fan-out: collapsing `contracts:*` back to
  `contracts` reproduces the registry's plan for all 44 targets.
- Propagation, checked against the graph: touching `src/emitter/skin.mjs`
  marks convert + its whole downstream stale, plus `unit` and `ledger`
  (which declare `src/**` directly). Touching `gates/coverage.mjs` marks
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
instead of 41, and the two most expensive builds are not among them:

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

Still missing before it can replace wireit: the node definitions are generated
from `gates/registry.mjs`, which stays the source of truth while both runners
exist — that is what keeps the reconciliation honest.

## Where the definitions live

`pipeline/nodes.go` — generated from `gates/registry.mjs` by
`export-graph.mjs`, checked with `--check` the way the wireit config is.

They are Go source, not JSON, on purpose. JSON arrives through
`encoding/json`, which silently ignores unknown fields and zero-fills missing
ones — so the 41 node declarations, the part most worth checking, would be the
one part no type ever saw. As typed literals with `Needs []NodeID` referencing
generated constants, a typo in a dependency is a compile error:

    ./nodes.go:89:22: undefined: NDemoCsss

Generated rather than hand-written because transcribing 41 nodes by hand is
how a gate goes quietly missing. `gates/registry.mjs` stays the source of truth
while wireit still executes the graph.
