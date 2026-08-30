# Upgrading the shadcn-ui pin

One command. Everything below it is what the command does, so that when it
reports work, you know where the report came from.

```sh
make upstream TO=shadcn@4.20.0          # the drill (add --fetch via node if tags are missing)
node gates/upstream.mjs --to=shadcn@4.20.0 --fetch
```

Exit 0 means: every artifact rebuilt, every gate green, every manual
intervention still applies, nothing for a person to do. Commit the result
(source + regenerated `dist/` and `docs/` together — the `dist/` diff is the
review).

Exit 1 means `build/gates/upstream-report.md` exists and says exactly what is
left. The nightly workflow (`.github/workflows/upstream.yml`) runs the same
drill against the newest release and opens one PR carrying the report.

## What the drill does

| Step | Command | What it settles |
|---|---|---|
| 1 | `git -C .upstream checkout <tag>` + `tools/pin.mjs --force` | the pin |
| 2 | `gates/ledger.mjs --dissolve` | every **auto-dissolve** exemption is deleted. The rebuild must re-earn each one with evidence; nobody walks a list by hand |
| 3 | `git apply --3way overlays/upstream/*.patch` | source-level patches rebase onto the new tag; a conflict is a conflict, not a silent no-apply |
| 4 | `gates/run.mjs --tier=full --keep-going` | the whole picture — every failing gate, not the first |
| 5 | `pipeline ir-diff` | slot-level semantic diff old pin → new pin: which components, slots, class lists, cva axes actually moved |
| 6 | `gates/overlay.mjs --tasks` | every hand-written unit re-proves it still applies; the ones that don't become task packets |
| 7 | classification | each failed gate is **EXPECTED** (its components changed upstream per step 5) or **UNEXPECTED** (nothing moved upstream — our pipeline regressed) |

## Reading the report

Work it top-down:

1. **UNEXPECTED failures** — ours. Fix the pipeline; `node gates/run.mjs --only=<gate>` reproduces one gate with exactly the builds it needs.
2. **Task packets** (`build/gates/tasks/*.md`) — one per stale or orphaned manual unit. Each carries the upstream diff for its inputs, the current file, and the gates that must be green afterwards. This is the bounded LLM/human step: deterministic input, gate-verified output.
3. **EXPECTED failures** — consequences of upstream changes, usually resolved by the packets above or by a legitimately new exemption. Record survivors: `node gates/ledger.mjs --record`.
4. `make upstream-snapshot` (network) refreshes the ui.shadcn.com golden snapshot for the new release; hop 1 of the golden gate compares against it.
5. `make` green → commit.

## Where manual interventions live (and how they are carried forward)

The conversion is mechanical but not complete. Every exception has a home
that `gates/overlay.mjs` audits on every run — never a find/replace on
generated output (the retired `patches/` mechanism).

| Kind | Home | Anchor | On re-pin |
|---|---|---|---|
| **rule** | `DEFAULT_CONTENT`, `TEXT_ADJUSTMENTS`, `DEAD_UTILITIES`, `SKIN_ALLOWLIST`, `KNOWN_ICONS`, tier sets, the Persian dictionary, contract `ignoreAttrs` | a structural predicate on the IR / upstream tree | `orphaned` when the anchor is gone; `dissolved` when upstream no longer needs it |
| **authored** | `src/runtime/core.js`, `src/runtime/components/*.js`, `tools/contracts/components/*.mjs`, hand-authored `docs/demos/*.html` | sha256 of the upstream inputs, in `overlays/manifest.json` | `stale` when an input changed → task packet with the diff; `gates/overlay.mjs --record` after re-authoring |
| **source** | `overlays/upstream/*.patch` | git blob ids | 3-way merge; `conflict` bucket |

## Exemptions

`gates/ledger.json` (rendered to `EXEMPTIONS.md`, never edited by hand). Every
entry has a class — `permanent`, `auto-dissolve`, `debt` — and budgets that
may only shrink. `node gates/ledger.mjs --verify` fails on growth and on
unrecorded improvement alike.

## Vendored engines

`vendor/radix-kernel.iife.js` and `vendor/embla-carousel.iife.js` are
sha-pinned in `src/registry/pin.json`. Re-vendoring is a separate, rare
event: replace the file, `npm run pin -- --force`, and every `behavior:*` unit in `overlays/manifest.json` goes stale by design (the
kernel sha is part of their anchor) — the drill turns those into packets.
