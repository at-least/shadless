# EXEMPTIONS — the recorded-difference ledger

<!-- GENERATED from gates/ledger.json by `pipeline ledger --render`.
     Do not edit by hand: the `ledger` gate checks the JSON, not
     this file, and the next render will overwrite whatever you wrote. -->

Pin: `shadcn@4.19.0` · 66 exemptions · 6 budgets

Every "known difference, accepted for a reason" lives here, and every entry
declares **how it ends**. The `ledger` gate keeps this list in lockstep
with the sources in both directions: a new exemption with no entry fails,
an entry whose source flag vanished fails.

## Permanent (54)

Real engine or by-design differences. These do not dissolve; upstream would have to change.

| Id | Reason | Recorded at |
|---|---|---|
| `dead-utility:origin-top-center` | referenced by the pinned registry, defined nowhere upstream (content scan renders it as no-rule). Automation: none — upstream defining it would surface as a compile change. | shadcn@4.19.0 |
| `golden:external dep @ai-sdk/react + @shadcn/react/message-scroller — page stays hand-authored` | external dep @ai-sdk/react + @shadcn/react/message-scroller — page stays hand-authored | shadcn@4.19.0 |
| `golden:external dep @base-ui/react (combobox) — tombstone; oracle cannot bundle` | external dep @base-ui/react (combobox) — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `golden:external dep @shadcn/react/questionnaire (+ zod) — tombstone; oracle cannot bundle` | external dep @shadcn/react/questionnaire (+ zod) — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `golden:external dep @tanstack/react-table (data-table example) — page stays hand-authored` | external dep @tanstack/react-table (data-table example) — page stays hand-authored | shadcn@4.19.0 |
| `golden:external dep cmdk (command) — tombstone; oracle cannot bundle` | external dep cmdk (command) — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `golden:external dep input-otp — tombstone; oracle cannot bundle` | external dep input-otp — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `golden:external dep react-day-picker (calendar) — tombstone; oracle cannot bundle` | external dep react-day-picker (calendar) — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `golden:external dep react-day-picker (date-picker) — tombstone; oracle cannot bundle` | external dep react-day-picker (date-picker) — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `golden:external dep react-resizable-panels — tombstone; oracle cannot bundle` | external dep react-resizable-panels — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `golden:external dep recharts (chart) — tombstone; oracle cannot bundle` | external dep recharts (chart) — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `golden:external dep vaul + recharts (drawer) — tombstone; oracle cannot bundle` | external dep vaul + recharts (drawer) — tombstone; oracle cannot bundle | shadcn@4.19.0 |
| `ignore-attrs:accordion-multiple:accordion-content:text` | same family as accordion | shadcn@4.19.0 |
| `ignore-attrs:accordion-multiple:accordion-item:text` | same family as accordion | shadcn@4.19.0 |
| `ignore-attrs:accordion-multiple:accordion:data-type` | same family as accordion | shadcn@4.19.0 |
| `ignore-attrs:accordion-multiple:accordion:text` | same family as accordion | shadcn@4.19.0 |
| `ignore-attrs:accordion:accordion-content:text` | radix Presence unmounts closed content; shadless keeps it hidden — textContent exposes that recorded structural difference | shadcn@4.19.0 |
| `ignore-attrs:accordion:accordion-item:text` | radix Presence unmounts closed content; shadless keeps it hidden — textContent exposes that recorded structural difference | shadcn@4.19.0 |
| `ignore-attrs:accordion:accordion:text` | radix Presence unmounts closed content; shadless keeps it hidden — textContent exposes that recorded structural difference | shadcn@4.19.0 |
| `ignore-attrs:carousel:carousel-content:style` | embla layout inline styles (vanilla port uses inline, oracle class=) | shadcn@4.19.0 |
| `ignore-attrs:carousel:carousel-item:style` | embla layout inline styles (vanilla port uses inline, oracle class=) | shadcn@4.19.0 |
| `ignore-attrs:carousel:carousel:data-orientation` | data-orientation only in shadless DOM (recorded, Wave E) | shadcn@4.19.0 |
| `ignore-attrs:collapsible:collapsible-content:text` | same Presence family | shadcn@4.19.0 |
| `ignore-attrs:collapsible:collapsible:text` | same Presence family | shadcn@4.19.0 |
| `ignore-attrs:dropdown-menu:dropdown-menu-content:data-align` | popper alignment attrs differ in form, not semantics | shadcn@4.19.0 |
| `ignore-attrs:dropdown-menu:dropdown-menu-trigger:data-radix-popper-align` | popper alignment attrs differ in form, not semantics | shadcn@4.19.0 |
| `ignore-attrs:scroll-area:scroll-area-scrollbar:style` | kernel runtime measurement styles (Wave B recorded) | shadcn@4.19.0 |
| `ignore-attrs:scroll-area:scroll-area-thumb:style` | kernel runtime measurement styles (Wave B recorded) | shadcn@4.19.0 |
| `ignore-attrs:scroll-area:scroll-area-viewport:data-radix-scroll-area-viewport` | kernel runtime measurement styles (Wave B recorded) | shadcn@4.19.0 |
| `ignore-attrs:scroll-area:scroll-area-viewport:style` | kernel runtime measurement styles (Wave B recorded) | shadcn@4.19.0 |
| `ignore-attrs:scroll-area:scroll-area:style` | kernel runtime measurement styles (Wave B recorded) | shadcn@4.19.0 |
| `ignore-attrs:scroll-area:scroll-area:text` | kernel runtime measurement styles (Wave B recorded) | shadcn@4.19.0 |
| `ignore-attrs:select:select-content:style` | runtime layout style; Presence text | shadcn@4.19.0 |
| `ignore-attrs:select:select-content:text` | runtime layout style; Presence text | shadcn@4.19.0 |
| `ignore-attrs:toggle-group-multiple:toggle-group:style` | `--gap` var set by runtime vs cva class on oracle | shadcn@4.19.0 |
| `ignore-attrs:toggle-group:toggle-group:style` | `--gap` var set by runtime vs cva class on oracle | shadcn@4.19.0 |
| `ignore-attrs:tooltip:tooltip-content:id` | radix moves id/role to an inner sr-only duplicate (textContent doubles); kernel keeps them on the root | shadcn@4.19.0 |
| `ignore-attrs:tooltip:tooltip-content:role` | radix moves id/role to an inner sr-only duplicate (textContent doubles); kernel keeps them on the root | shadcn@4.19.0 |
| `ignore-attrs:tooltip:tooltip-content:text` | radix moves id/role to an inner sr-only duplicate (textContent doubles); kernel keeps them on the root | shadcn@4.19.0 |
| `mounted-check:alert-dialog` | controlled-open oracle: content mounts at INITIAL render — before/after bag diff inapplicable; fixture is slot-keyed (styles ride out.css slot rules) | shadcn@4.19.0 |
| `mounted-check:dialog` | controlled-open oracle: content mounts at INITIAL render, so the before/after bag diff has nothing to diff; the fixture is slot-keyed (styles ride out.css slot rules) — same mechanism as alert-dialog (dialog) | shadcn@4.19.0 |
| `mounted-check:navigation-menu` | radix mounts sibling items' links into the viewport machinery; shadless glue mounts only the active content — REAL structural difference | shadcn@4.19.0 |
| `mounted-check:scroll-area` | runtime mounts only measurement styles (Wave B recorded inline-style difference) — no structural additions to compare | shadcn@4.19.0 |
| `mounted-check:select` | controlled-open oracle: content mounts at INITIAL render, so the before/after bag diff has nothing to diff; the fixture is slot-keyed (styles ride out.css slot rules) — same mechanism as alert-dialog (select) | shadcn@4.19.0 |
| `mounted-check:sheet` | controlled-open oracle: content mounts at INITIAL render, so the before/after bag diff has nothing to diff; the fixture is slot-keyed (styles ride out.css slot rules) — same mechanism as alert-dialog (sheet) | shadcn@4.19.0 |
| `mounted-classes:alert-dialog` | controlled-open oracle: content mounts at INITIAL render — before/after bag diff inapplicable; fixture is slot-keyed (styles ride out.css slot rules) | shadcn@4.19.0 |
| `mounted-classes:dialog` | controlled-open oracle: content mounts at INITIAL render, so the before/after bag diff has nothing to diff; the fixture is slot-keyed (styles ride out.css slot rules) — same mechanism as alert-dialog (dialog) | shadcn@4.19.0 |
| `mounted-classes:scroll-area` | runtime mounts only measurement styles (Wave B recorded inline-style difference) — no structural additions to compare | shadcn@4.19.0 |
| `mounted-classes:select` | controlled-open oracle: content mounts at INITIAL render, so the before/after bag diff has nothing to diff; the fixture is slot-keyed (styles ride out.css slot rules) — same mechanism as alert-dialog (select) | shadcn@4.19.0 |
| `mounted-classes:sheet` | controlled-open oracle: content mounts at INITIAL render, so the before/after bag diff has nothing to diff; the fixture is slot-keyed (styles ride out.css slot rules) — same mechanism as alert-dialog (sheet) | shadcn@4.19.0 |
| `skin-allowlist:cn-font-heading` | inert CLI-install-time markers; live CSS defines zero rules for them. Automation: upstream emitting rules for one would show as a fidelity diff. | shadcn@4.19.0 |
| `skin-allowlist:cn-menu-target` | inert CLI-install-time markers; live CSS defines zero rules for them. Automation: upstream emitting rules for one would show as a fidelity diff. | shadcn@4.19.0 |
| `skin-allowlist:cn-menu-translucent` | inert CLI-install-time markers; live CSS defines zero rules for them. Automation: upstream emitting rules for one would show as a fidelity diff. | shadcn@4.19.0 |
| `skin-allowlist:cn-rtl-flip` | inert CLI-install-time markers; live CSS defines zero rules for them. Automation: upstream emitting rules for one would show as a fidelity diff. | shadcn@4.19.0 |

## Auto-dissolve on re-pin (12)

Deploy lag, SSR-frame lag and other pin-relative differences. `make upstream` DELETES every one of these after a re-pin and lets the gates re-earn them — nobody reviews this section by hand.

| Id | Reason | Recorded at |
|---|---|---|
| `golden:ICU Arabic percent-usage spacing differs between local chromium and upstream's SSR ICU — environment; re-check on re-pin` | ICU Arabic percent-usage spacing differs between local chromium and upstream's SSR ICU — environment; re-check on re-pin | shadcn@4.19.0 |
| `golden:embla live-region text (Slide N of N) computed after mount (SSR frame 0/0) — frame lag; re-check on re-pin` | embla live-region text (Slide N of N) computed after mount (SSR frame 0/0) — frame lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:example copy drift vs live ("font-normal" vs "font- normal") — deploy lag; re-check on re-pin` | example copy drift vs live ("font-normal" vs "font- normal") — deploy lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:lucide icon path encoding drift vs live — deploy lag; re-check on re-pin` | lucide icon path encoding drift vs live — deploy lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:physical→logical utility drift vs live (-ml-4 vs -ms-4, pinned carousel skin) — deploy lag; re-check on re-pin` | physical→logical utility drift vs live (-ml-4 vs -ms-4, pinned carousel skin) — deploy lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:physical→logical utility drift vs live (-right-1 vs -end-1, pinned tabs.tsx) — deploy lag; re-check on re-pin` | physical→logical utility drift vs live (-right-1 vs -end-1, pinned tabs.tsx) — deploy lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:physical→logical utility drift vs live (pinned skin pr-2 vs live pe-2) — deploy lag; re-check on re-pin` | physical→logical utility drift vs live (pinned skin pr-2 vs live pe-2) — deploy lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:radix SelectValue renders value only after hydration on live (SSR frame empty) — deploy lag; re-check on re-pin` | radix SelectValue renders value only after hydration on live (SSR frame empty) — deploy lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:radix context-menu trigger vendor style (-webkit-touch-callout) added after pin — deploy lag; re-check on re-pin` | radix context-menu trigger vendor style (-webkit-touch-callout) added after pin — deploy lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:radix scroll-area viewport overflow differs in SSR frame (hidden vs settled scroll) — deploy lag; re-check on re-pin` | radix scroll-area viewport overflow differs in SSR frame (hidden vs settled scroll) — deploy lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:slider/progress thumb position is runtime-measured (SSR frame at 0%) — frame lag; re-check on re-pin` | slider/progress thumb position is runtime-measured (SSR frame at 0%) — frame lag; re-check on re-pin | shadcn@4.19.0 |
| `golden:token drift vs live (ring-ring/50 vs /30) — deploy lag; re-check on re-pin` | token drift vs live (ring-ring/50 vs /30) — deploy lag; re-check on re-pin | shadcn@4.19.0 |

## Debt (0)

Accepted for now, tracked to zero. Governed by the budgets below.

| Id | Reason | Recorded at |
|---|---|---|

## Budgets

A budget may only shrink. Growing fails the `ledger` gate; shrinking without
re-recording also fails, so slack cannot be silently re-spent.

| Metric | Max | Target | Reason |
|---|---|---|---|
| `golden.exempt-demos` | 147 | 0 | demos exempt from the golden dual-hop: external-dep tombstones (permanent until a vanilla port exists) + deploy-lag families (dissolve on re-pin) |
| `interactivity.dead-families` | 1 | 0 | example families still shipping as static oracle snapshots (dead buttons); each needs its glue protocol mapped into tools/example-fixture.mjs |
| `style-parity.dirty-cells` | 210 | 0 | (component, slot#n, property) cells where the shipped fixture's computed style differs from the React oracle; fix fixtures toward oracle values, --record, shrink |
| `coverage.uncovered-cells` | 46 | 0 | cells of the product matrix (component x path x theme x dir x state) no gate makes a computed-style or behavioral assertion about; see pipeline/gate_coverage.go |
| `path-parity.dirty-cells` | 0 | 0 | (component/slot/property @ path @ theme @ dir) cells where slot-only markup via css-import or full.css computes differently from React under upstream css; zero at shadcn@4.19.0 |
| `demo-parity.dirty-cells` | 76 | 0 | (page/slot#n/property @ theme @ dir) cells where a shipped demo page under our css computes differently from the same DOM under upstream css; see gates/demo-parity.mjs |

## Work items

Not cross-checked — these track work, not accepted differences.

- [ ] style-parity baseline (~23 components carry fixture/computed drift vs the oracle — accordion trigger stretch, trigger icons on some fixtures, alert-dialog trigger unstyled in fixture, carousel item width): shrink the baseline by fixing fixture styles to oracle computed values, then re-record; `--strict` is the end state.
- [ ] Fixture-content parity: kernel-tier -demo pages ARE the contract fixtures — their copy/items are simplified vs the real upstream examples (alert-dialog was stale-wrong and got fixed; these are simplified): context-menu, dropdown-menu, hover-card, menubar, navigation-menu, scroll-area, select, sheet, tabs, tooltip. Automation idea: render each example's closed state through the oracle and diff its text/content against the fixture (the audit script exists in git history — resurrect as a periodic gate once the fixtures are aligned).
- [ ] navigation-menu mounted structure: radix mounts sibling links; decide glue parity vs permanent record.
- [ ] JSX composition examples remain in feature sections (in React syntax, imports stripped): translating them to vanilla markup is the deeper completion of the docs de-Reactification.
- [ ] Example pages that still ship as static oracle snapshots (dead buttons) are counted by tools/interactivity-sweep.mjs every run and budgeted as `interactivity.dead-families` (families in KNOWN_DEAD, currently message-scroller only — pages, not families, is the larger number; run the sweep for it). example-fixture.mjs covers the dialog family, tabs, slider, scroll-area, select, tooltip, popover, hover-card and the menus; each remaining family needs its glue protocol mapped there.
- [ ] Controlled-open oracles (dialog/alert-dialog/sheet/select): switch the usage to trigger-driven so the mounted-DOM check applies, then drop the mounted-check/mounted-classes exemptions.
- [ ] Coverage matrix scope: gates/coverage.mjs enumerates tiers other than `external` and `logic`, so `carousel` (external, ported onto vendored embla — contract-tested and in the golden gate) and `field` (logic — ships CSS + a hand-authored demo in tools/demo.mjs, no contract, no oracle) sit outside the uncovered-cells count. field is the one shipped component with no oracle-backed assertion; adding it to the matrix (or a contract) is the work item.
- [ ] Not gated at all (outside the CSS-parity surface): accessibility beyond attribute equality (no axe, no Tab / focus-trap / roving-focus scenario in contracts — 16 of 30 defs press a key, mostly Escape), animations (every parity gate freezes them), browsers other than chromium, TypeScript (dist/esm/*.d.ts are concatenated, never type-checked), a bundle-size budget, a no-JS render. The npm surface is gated since gates/pack.mjs.
- [ ] Big-ticket gaps, recorded so they are not rediscovered: (1) 15 grey components have no vanilla port (calendar/date-picker → react-day-picker, chart → recharts, command/combobox → cmdk/@base-ui, drawer → vaul, resizable → react-resizable-panels, input-otp, sonner/toast, form → react-hook-form, sidebar, questionnaire, data-table → tanstack, typography) — each is a dependency rewrite, not a conversion. (2) The JS base is not tree-shakeable: tools/build-js.mjs concatenates the whole 143 KB radix kernel into dist/shadless.js unconditionally and the ESM entry re-exports a runtime global; a checkbox-only page pays for the kernel. Splitting the kernel per feature needs the kernel's source, which this repo does not hold (vendor/radix-kernel.iife.js is a pinned blob, no source map). (3) No a11y gate (axe, Tab / focus-trap scenarios), chromium only, animations frozen in every parity gate, no no-JS render — see the previous note. (4) The dialog family's mounted-DOM check is off (controlled-open oracles) and navigation-menu carries a real structural difference — see the notes above.
- [ ] Plain (non-cva) path @apply order: src/emitter/css.mjs joins a slot's class strings into ONE @apply, where Tailwind's internal utility order — not composition order — resolves same-group conflicts. Concrete case: attachment-action ships `rounded-lg … rounded-[min(var(--radius-md),10px)]` in one @apply and compiles to `var(--radius)` while React/twMerge keeps the later `rounded-[min(...)]`. The cva path was fixed with twin blocks (2026-08-28; regression-gated by path-parity); extending path-parity to plain-path slots with intra-list same-group conflicts would close the family.
