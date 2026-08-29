# pipeline — the graph runner

One rule:

    key(n) = H( n.id, n.run, contents of every file n declares as input,
                key(d) for every d in n.needs )

A node runs iff its recorded key differs from its computed key. Because each
dependency's key is folded into its dependents', "something upstream changed"
propagates by construction — there is no second mechanism, and no special case
for the shadcn pin (it is just a file that some node declares as an input).

    pipeline plan   <tier|node…>   the closure, topologically sorted
    pipeline status <tier|node…>   fresh / STALE / NEVER-FRESH per node
    pipeline run    <tier|node…>   run the stale ones, stamp on success
    pipeline adopt  <tier|node…>   record current keys WITHOUT running

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
- Propagation, checked against the graph: touching `src/emitter/skin.mjs`
  marks convert + its whole downstream stale, plus `unit` and `ledger`
  (which declare `src/**` directly). Touching `gates/coverage.mjs` marks
  `coverage` and nothing else.
- Whole-graph `status` costs 0.2s.

Still missing before it can replace wireit: parallel execution, output
caching across machines (wireit has local + GitHub Actions caching), and
verification that a node writes only what it declares in `produces` — the
`convert` node currently writes `src/kernel/*.html` without declaring it.

`graph.json` is generated from `gates/registry.mjs`, which stays the single
source of truth until the reconciliation is finished.
