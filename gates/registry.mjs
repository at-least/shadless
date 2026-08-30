// gates/registry.mjs — THE pipeline graph. Single source of truth.
//
// Before this file the gate set lived in two places that had drifted apart:
// `Makefile:verify` ran 11 gates, `tools/verify.mjs` ran 23 steps, and the
// intersection was 3. Either one alone could be green while a whole class of
// checks never executed. Both entry points now derive from this array, so
// "the gate set" is one reviewable list.
//
// Each node is a build step or a gate:
//
//   id        stable name — what `gates/run.mjs --only=<id>` takes
//   kind      "build" (produces artifacts) | "gate" (asserts, produces nothing)
//   needs     ids that must run first. This is a real dependency, not an
//             ordering preference: the runner topo-sorts and, for a targeted
//             run, builds exactly the transitive closure and nothing else.
//   run       array of argv arrays, executed in order
//   tier      "fast" (~seconds, no browser) | "medium" (compiles, no browser)
//             | "full" (playwright)
//   inputs    globs of everything the node READS (its own script, the modules
//             it imports, the data it opens). Outputs of `needs` are implied —
//             the runner folds a dependency's key into its dependents'. This is
//             what lets `make only ID=<id>` skip a node nothing has touched
//             (pipeline/nodes.go is generated from here).
//             `null` = judges state outside the tree, never fresh.
//   why       one line: what breaks if this node is deleted. Not decoration —
//             `gates/meta.mjs` requires every gate to carry one.
//   mutations ids under gates/mutations/ that MUST make this gate fail.
//             A gate with no mutation is unproven and the meta-gate rejects it.
//
// Ordering inside the array is irrelevant; `needs` decides. Keep it grouped
// by phase for human reading only.

const node = (n) => n

export const NODES = [
  // ---------------------------------------------------------------- inputs
  node({
    id: "pin", kind: "gate", tier: "fast", needs: [],
    run: [["./build/pipeline", "pin", "--check-only"]],
    // the checkout's HEAD file IS the state this gate judges; declaring it
    // (rather than `null`) matters: no dependent of a node without a declared
    // input set can ever be fresh, and everything descends from pin
    inputs: ["pipeline/gate_pin.go", "src/registry/pin.json", "vendor/**", ".upstream/shadcn-ui/.git/HEAD"],
    why: "the .upstream checkout must sit exactly at the pinned release tag; " +
         "upgrade tools write pin.json directly and nothing else checks the result",
    mutations: ["pin-commit-drift"],
  }),

  // ------------------------------------------------------------ pure logic
  node({
    id: "unit", kind: "gate", tier: "fast", needs: [],
    run: [["node", "tools/unit-check.mjs"]],
    inputs: ["tools/unit-check.mjs", "tools/unit/**", "src/**", "tools/**/*.mjs", "vendor/**",
             "package.json", "dist/esm/shadless.d.ts"],
    why: "seconds-level guard over the pure functions cleanup rounds touch; " +
         "born from a dead-code delete in rewritePaths that only surfaced minutes later",
    mutations: ["unit-break-pure-fn"],
  }),
  node({
    id: "ledger", kind: "gate", tier: "fast", needs: [],
    run: [["node", "gates/ledger.mjs", "--verify"]],
    inputs: ["gates/ledger.mjs", "gates/ledger.json", "gates/*-baseline.json",
             "src/registry/pin.json", "src/registry/upstream-snapshot/exemptions.json",
             "tools/contracts/components/**", "tools/interactivity-sweep.mjs", "src/**"],
    why: "every recorded exemption must be schema-valid, still present in its " +
         "source, and inside its budget — scattered flags rot silently",
    mutations: ["ledger-undocumented-exemption", "ledger-budget-exceeded"],
  }),

  node({
    id: "pipeline-sync", kind: "gate", tier: "fast", needs: [],
    run: [["node", "pipeline/export-graph.mjs", "--check"]],
    inputs: ["pipeline/export-graph.mjs", "pipeline/nodes.go", "gates/registry.mjs"],
    why: "pipeline/nodes.go (the runner's graph, as typed Go) is generated from this file; " +
         "a registry edit without regenerating, or a hand edit, would make the runner execute " +
         "a different graph from the one meta proves",
    mutations: ["pipeline-drift"],
  }),
  node({
    // reads dist/out.css + dist/css — must not race their producers under the parallel runner
    id: "dist-complete", kind: "gate", tier: "fast", needs: ["demo-css", "product-css"],
    run: [["./build/pipeline", "gate", "dist-complete"]],
    inputs: ["pipeline/gate_dist_complete.go", "dist/css/**", "dist/out.css"],
    why: "the tracked no-build dist/out.css must carry every slot selector its per-component " +
         "sources declare — a partial-build out.css (static pages only) got committed once and " +
         "no gate asked whether the file was whole",
    mutations: ["dist-complete-drop-component"],
  }),
  node({
    // reads dist/ — must not race build-js / product-build under the parallel runner
    id: "pack", kind: "gate", tier: "fast", needs: ["build-js", "product-build"],
    run: [["./build/pipeline", "gate", "pack"]],
    inputs: ["pipeline/gate_pack.go", "package.json", "README.md", "dist/**"],
    why: "the npm surface — exports map, tarball contents, README specifiers, an empty " +
         "dependencies — must agree: a bare-string ./runtime.min export served an IIFE to " +
         "`import`, README documented a specifier that does not resolve, and a React-free " +
         "package installed React through dependencies",
    mutations: ["pack-broken-export"],
  }),
  node({
    // needs example-oracle: the ownership manifest this gate counts cells
    // from is example-oracle's output. Without the edge it judged whatever
    // the PREVIOUS run left behind — a full run could regenerate the
    // manifest after coverage had already passed over the old one.
    id: "coverage", kind: "gate", tier: "fast", needs: ["convert", "example-oracle"],
    run: [["./build/pipeline", "gate", "coverage", "--check"]],
    inputs: ["pipeline/gate_coverage.go", "pipeline/gate_coverage_budget.go", "gates/ledger.json", "src/registry/tiers.json",
             "src/registry/ir/**", "docs/example-oracle.json", "docs/demos/**",
             "tools/contracts/components/**", "tools/interactivity-sweep.mjs"],
    why: "the product surface (component x path x theme x dir x state) as a matrix with the " +
         "gate covering each cell; the UNCOVERED count is budgeted and may only shrink — every " +
         "historical bug sat in a cell no gate had been written for",
    mutations: ["coverage-drop-contract"],
  }),
  node({
    // needs example-oracle: same stale-manifest problem as coverage —
    // "is this authored file still needed?" is answered from
    // docs/example-oracle.json + docs/example-fixture-targets.json, both
    // written by example-oracle.
    id: "overlay", kind: "gate", tier: "fast", needs: ["convert", "example-oracle"],
    run: [["node", "gates/overlay.mjs", "--audit"]],
    inputs: ["gates/overlay.mjs", "overlays/**", "src/**", "tools/**/*.mjs",
             "docs/example-oracle.json", "docs/example-fixture-targets.json", "docs/demos/**",
             "src/registry/pin.json"],
    why: "every manual intervention on top of the mechanical conversion (rule tables, " +
         "hand-written fixtures/glue/runtime, upstream patches) must still apply to the " +
         "pinned upstream — orphaned rules and stale authored files fail here, with task " +
         "packets, instead of silently no-op'ing like the old find/replace overlay",
    mutations: ["overlay-stale-authored", "overlay-orphaned-rule"],
  }),

  // ------------------------------------------------------------- IR + emit
  node({
    id: "convert", kind: "build", tier: "medium", needs: ["pin"],
    run: [["node", "tools/resolve-skins.mjs"], ["node", "src/converter/index.mjs"]],
    inputs: ["tools/resolve-skins.mjs", "src/converter/**", "src/tags.mjs", "src/emitter/skin.mjs",
             "src/registry/tiers.json", "src/registry/pin.json", "src/kernel/**"],
    why: "registry .tsx -> IR JSON, with its own drift gate against the pinned source",
    produces: ["src/registry/ir", "build/resolved-ui"],
  }),
  node({
    id: "emit", kind: "build", tier: "medium", needs: ["convert"],
    // emit writes ONLY the static pages + dist/shadless.css into dist/; its own
    // globals/out.css/index live in build/emit/ so a medium-tier run never
    // damages the committed demo chain outputs (it used to wipe dist/components)
    run: [["node", "src/emitter/index.mjs"],
          ["node", "tools/tw.mjs", "build/emit/globals.css", "build/emit/out.css", "--cwd", "dist"]],
    inputs: ["src/emitter/**", "src/tags.mjs", "src/docs/theme-prepaint.mjs",
             "src/registry/tiers.json", "src/registry/pin.json", "probes/h4/globals.css",
             "tools/tw.mjs"],
    why: "static-tier emit: IR -> component html + per-slot css",
    produces: ["dist/components/*.html", "!dist/components/*-rtl-*.html", "dist/shadless.css", "build/emit"],
  }),
  node({
    id: "emit-smoke", kind: "gate", tier: "medium", needs: ["emit"],
    // was buried inside the emit build step, where no tier could select it and
    // no mutation could prove it
    run: [["node", "src/emitter/smoke.mjs"]],
    inputs: ["src/emitter/smoke.mjs", "src/registry/tiers.json", "src/registry/ir/**", "dist/components/**"],
    why: "emitted markup parses to exactly the expected tags and nesting (jsdom)",
    mutations: ["emit-smoke-slotless-page"],
  }),

  node({
    id: "build-js", kind: "build", tier: "fast", needs: [],
    run: [["node", "tools/build-js.mjs"]],
    inputs: ["tools/build-js.mjs", "src/runtime/**", "vendor/**"],
    why: "the JS surface: dist/shadless.js (kernel + base) and dist/js/<name>.js per component",
    produces: ["dist/shadless.js", "dist/js", "dist/shadless.min.js", "dist/esm"],
  }),
  node({
    id: "contract-fixture", kind: "build", tier: "full", needs: ["convert", "build-js"],
    run: [["node", "tools/example-fixture.mjs", "--contracts"]],
    inputs: ["tools/example-fixture.mjs", "tools/contracts/oracle-build.mjs",
             "tools/contracts/components/**", "tools/fixture-families.mjs",
             "src/docs/theme-prepaint.mjs", "docs/example-fixture-targets.json",
             "tools/oracle-lib.mjs", "tools/contracts/stubs/**", "tools/resolve-skins.mjs",
             "src/registry/pin.json", "package-lock.json"],
    why: "the kernel contract fixtures (src/kernel/*.html) are harvested from the contract " +
         "defs' own React render — hand-mirrored fixtures drifted in content and classes " +
         "(the bulk of style-parity's recorded cells)",
    produces: ["src/kernel/*.html"],
  }),

  // ------------------------------------------------------------ demo pages
  node({
    id: "demo-build", kind: "build", tier: "medium", needs: ["emit"],
    run: [["node", "tools/build-demo.mjs"]],
    inputs: ["tools/build-demo.mjs", "src/docs/theme-prepaint.mjs"],
    why: "single-demo compositions from the upstream examples",
    produces: ["dist/components/*-demo.html", "!dist/components/*-rtl-*.html"],
  }),
  node({
    id: "example-oracle", kind: "build", tier: "full", needs: ["demo-build"],
    run: [["node", "tools/example-oracle.mjs"]],
    inputs: ["tools/example-oracle.mjs", "src/docs/theme-prepaint.mjs", "src/registry/tiers.json",
             "src/runtime/components/**", "docs/catalog.json", "tools/oracle-lib.mjs",
             "tools/contracts/stubs/**", "tools/resolve-skins.mjs", "src/registry/pin.json",
             "src/registry/upstream-snapshot/exemptions.json", "package-lock.json"],
    why: "upstream examples rendered by real React+chromium BECOME the demo pages — " +
         "1:1 with upstream by construction, not by hand-mirroring",
    // rtl language variants are demo-rtl's: output sets must be DISJOINT between
    // nodes that can run in parallel, or their output sets race
    // build/example-oracle is SCRATCH, not an output: each render harness is
    // rmSync'd after use and the directory ends up empty, nothing declares it
    // as an input. Declaring it made the node look unproducible on a clean
    // checkout, where its real outputs (the pages + both manifests) are all
    // committed and need no rebuild.
    produces: ["docs/demos/*.html", "!docs/demos/*-rtl-*.html", "docs/example-oracle.json", "docs/example-fixture-targets.json"],
    // a build node with a mutation: the ownership manifest is a gate input,
    // so "the build swallowed a render failure" is a gate-level lie
    mutations: ["example-oracle-render-failure"],
  }),
  node({
    id: "example-fixture", kind: "build", tier: "full", needs: ["example-oracle"],
    run: [["node", "tools/example-fixture.mjs"]],
    inputs: ["tools/example-fixture.mjs", "tools/contracts/oracle-build.mjs",
             "tools/contracts/components/**", "tools/fixture-families.mjs",
             "src/docs/theme-prepaint.mjs", "docs/example-fixture-targets.json", "dist/js/**",
             "tools/oracle-lib.mjs", "tools/contracts/stubs/**", "tools/resolve-skins.mjs",
             "src/registry/pin.json", "package-lock.json"],
    why: "kernel-tier examples as INTERACTIVE fixtures harvested from the oracle; " +
         "the oracle alone emits static snapshots with dead buttons",
    // build/example-fixture likewise: tools/example-fixture.mjs:39 `const TMP`
    produces: ["docs/demos/*.html", "!docs/demos/*-rtl-*.html"],
  }),
  node({
    id: "demo-rtl", kind: "build", tier: "medium", needs: ["example-oracle"],
    run: [["node", "tools/build-rtl.mjs"]],
    inputs: ["tools/build-rtl.mjs", "tools/rtl-lib.mjs", "src/docs/theme-prepaint.mjs",
             "src/registry/pin.json"],
    why: "AR/HE/EN/FA variants derived from the Arabic oracle page + upstream dictionaries",
    produces: ["dist/components/*-rtl-*.html", "docs/demos/*-rtl-*.html", "build/rtl-langs.json"],
  }),
  node({
    id: "demo", kind: "build", tier: "medium", needs: ["demo-rtl", "example-fixture", "contract-fixture", "build-js"],
    run: [["node", "tools/demo.mjs"]],
    inputs: ["tools/demo.mjs", "tools/demo-lib.mjs", "tools/build-js.mjs", "src/emitter/css.mjs",
             "src/docs/theme-prepaint.mjs", "src/registry/tiers.json", "src/registry/ir/**",
             "src/kernel/**", "probes/h4/globals.css", "probes/t7/**", "probes/t8/**"],
    why: "unified globals.css (slot rules folded in) + the demo index + the " +
         "per-component @apply sources the npm surface exports",
    produces: ["dist/globals.css", "dist/demo-index.html", "dist/css", "dist/components/*.html", "!dist/components/*-rtl-*.html"],
  }),

  // ------------------------------------------------------------ css builds
  node({
    // dist/css is DEMO's output now (one writer, next to the componentCss call
    // that computes it); this node only reads it back into the product entry.
    // theme-prepaint supplies SHADLESS_CSS_FIXES and package-lock pins the
    // tw-animate-css that gets inlined — both were read but undeclared.
    id: "product-css", kind: "build", tier: "medium", needs: ["demo"],
    run: [["node", "tools/product-css.mjs"]],
    inputs: ["tools/product-css.mjs", "src/docs/theme-prepaint.mjs", "probes/h4/globals.css",
             "package-lock.json"],
    why: "token extraction + the product entry — the consumer-facing surface",
    produces: ["dist/shadless-core.css", "dist/shadless.product.css"],
  }),
  node({
    id: "demo-css", kind: "build", tier: "medium", needs: ["demo"],
    // repo-root cwd: the repo-wide content scan is load-bearing for the
    // docs/demos iframes (@source not excludes tool fixtures — see tools/demo.mjs)
    run: [["node", "tools/tw.mjs", "dist/globals.css", "dist/out.css", "--cwd", "."]],
    // == the @source list in tools/demo.mjs (explicit scan, no auto-detection)
    inputs: ["tools/tw.mjs", "dist/globals.css", "dist/components/**", "dist/js/**", "docs/demos/**",
             "docs/content/**", "src/kernel/**", "tools/contracts/out/**", "src/registry/ir/**",
             "probes/t7/**", "probes/t8/**"],
    why: "the stylesheet every demo page and contract fixture actually loads",
    produces: ["dist/out.css"],
  }),
  node({
    id: "product-build", kind: "build", tier: "medium", needs: ["product-css"],
    // compiled in an empty scratch dir so ONLY @apply-driven rules survive
    run: [["node", "tools/tw.mjs", "dist/shadless.product.css", "dist/shadless.full.css"],
          ["node", "tools/tw.mjs", "dist/shadless.product.css", "dist/shadless.full.min.css", "--minify"]],
    inputs: ["tools/tw.mjs", "dist/shadless.product.css"],
    why: "the no-build distribution artifact",
    produces: ["dist/shadless.full.css", "dist/shadless.full.min.css"],
  }),

  // ------------------------------------------------------- library gates
  node({
    // compares the product build against the DEMO build too, so it needs
    // out.css fresh — a targeted run that rebuilt emit (whose out.css has no
    // slot rules) but not demo-css reported every slot as dropped
    id: "product-verify", kind: "gate", tier: "medium", needs: ["product-build", "demo-css"],
    run: [["node", "tools/product-css.mjs", "--verify"]],
    inputs: ["tools/product-css.mjs", "dist/**"],
    why: "slot rules survive the product compile and docs chrome stays out of it",
    mutations: ["product-drop-slot-rule"],
  }),
  node({
    id: "consumer-sim", kind: "gate", tier: "medium", needs: ["product-css"],
    run: [["node", "tools/consumer-sim.mjs"]],
    inputs: ["tools/consumer-sim.mjs", "dist/css/**", "dist/shadless-core.css", "package.json"],
    why: "the PRIMARY consume path, machine-checked: a scratch consumer importing " +
         "core + N component files gets exactly those styles, and every component " +
         "compiles ALONE (not only as part of the full product entry)",
    mutations: ["consumer-sim-unknown-utility"],
  }),
  node({
    id: "path-parity", kind: "gate", tier: "full", needs: ["product-build", "oracle-css"],
    run: [["node", "gates/path-parity.mjs"]],
    inputs: ["gates/path-parity.mjs", "gates/parity-baseline.mjs", "gates/path-parity-baseline.json", "gates/ledger.json",
             "src/emitter/css.mjs", "src/tags.mjs", "src/registry/ir/**", "dist/css/**",
             "dist/shadless.full.css", "build/gates/oracle.css", "src/registry/pin.json",
             "tools/tw.mjs"],
    why: "for EVERY slot, slot-only markup via css-import and via full.css must compute what " +
         "React's inline classes compute under upstream's own stylesheet, in both themes and " +
         "directions, at rest, per cva variant value and per attribute-driven state, with " +
         "referenced child slots rendered on both sides — subsumes the retired variant-parity; " +
         "found cva defaults living in fn params (attachment, marker), toggle's pressed state " +
         "losing to the (0,2,0) variant qualifier, and twMerge residue (text-sm line-height)",
    mutations: ["path-parity-drop-utility", "variant-merge-defaults"],
  }),
  node({
    id: "demo-parity", kind: "gate", tier: "full", needs: ["demo-css", "oracle-css", "example-oracle"],
    run: [["node", "gates/demo-parity.mjs"]],
    inputs: ["gates/demo-parity.mjs", "gates/parity-baseline.mjs", "gates/demo-parity-baseline.json", "gates/ledger.json",
             "build/gates/oracle.css", "dist/out.css", "docs/demos/**", "docs/example-oracle.json"],
    why: "every shipped demo page's DOM under our css must compute what the SAME DOM computes " +
         "under upstream's stylesheet, light/dark x ltr/rtl — same DOM on both sides, so every " +
         "cell is emitted css (skin markers, slot rules leaking under inline utilities, tokens)",
    mutations: ["demo-parity-token-drift"],
  }),
  node({
    id: "css-direction", kind: "gate", tier: "fast", needs: ["demo-css"],
    run: [["node", "tools/css-direction-gate.mjs"]],
    inputs: ["tools/css-direction-gate.mjs", "dist/shadless.css"],
    why: "emitted physical reading-direction utilities must match the recorded set — " +
         "new/gone entries mean upstream moved the RTL story",
    mutations: ["css-direction-new-physical"],
  }),
  node({
    id: "contracts", kind: "gate", tier: "full", needs: ["demo-css"],
    run: [["npm", "run", "contracts"]],
    inputs: ["tools/contracts/**/*.mjs", "tools/contracts/stubs/**",
             "tools/contracts/components/**", "tools/oracle-lib.mjs", "dist/**", "src/kernel/**",
             "src/registry/pin.json", "package-lock.json", "gates/ledger.json"],
    why: "THE oracle: the pinned registry bundled with real React+radix, replayed " +
         "against the shipped pages with real mouse/keyboard, incl. mounted-DOM structure",
    mutations: ["contracts-strip-glue"],
    produces: ["tools/contracts/out"], // a gate that also leaves review surfaces behind
  }),
  node({
    id: "oracle-css", kind: "build", tier: "medium", needs: ["convert"],
    run: [["node", "gates/oracle-css.mjs"]],
    inputs: ["gates/oracle-css.mjs", "tools/tw.mjs", "src/registry/pin.json", "build/resolved-ui/**"],
    why: "a stylesheet for the React oracle built from upstream's own globals/skin and the " +
         "resolved registry — reads nothing under src/, so style-parity is no longer circular",
    produces: ["build/gates/oracle.css"],
  }),
  node({
    id: "style-parity", kind: "gate", tier: "full", needs: ["contracts", "oracle-css"],
    run: [["node", "tools/style-parity.mjs"]],
    inputs: ["tools/style-parity.mjs", "gates/parity-baseline.mjs", "gates/style-parity-baseline.json", "gates/ledger.json",
             "tools/contracts/out/**", "tools/contracts/components/**", "build/gates/oracle.css",
             "src/registry/pin.json"],
    why: "computed STYLE parity vs the React oracle — 'same DOM + same css => same " +
         "styles' was an inference no gate ever tested",
    mutations: ["style-parity-perturb-padding", "style-parity-recorded-value-drift"],
  }),
  node({
    id: "demo-smoke", kind: "gate", tier: "full", needs: ["demo-css"],
    run: [["node", "tools/demo-smoke.mjs"]],
    inputs: ["tools/demo-smoke.mjs", "dist/**", "src/registry/ir/**", "src/registry/tiers.json"],
    why: "every dist demo page loads with zero console errors",
    mutations: ["demo-smoke-console-error"],
  }),

  // ------------------------------------------------------------------ docs
  node({
    id: "docs-catalog", kind: "build", tier: "medium", needs: ["demo"],
    run: [["node", "tools/docs-catalog.mjs"]],
    inputs: ["tools/docs-catalog.mjs", "src/registry/pin.json", "src/registry/tiers.json",
             "dist/components/**", "docs/demos/**"],
    why: "the preview catalog the site is generated from",
    produces: ["docs/catalog.json"],
  }),
  node({
    id: "docs-build", kind: "build", tier: "medium", needs: ["docs-catalog", "demo-css"],
    run: [["node", "tools/docs-build.mjs"]],
    inputs: ["tools/docs-build.mjs", "tools/docs-guides.mjs", "tools/docs-page-lib.mjs",
             "tools/fixture-families.mjs", "src/docs/**", "docs/catalog.json", "docs/content/**",
             "docs/fonts/**", "dist/**", "docs/demos/**", "build/rtl-langs.json",
             "src/registry/ir/**", "src/registry/pin.json", "package.json", "package-lock.json"],
    why: "mdx -> the mirrored site, with the dist demos copied in under the site skin",
    produces: ["docs/site", "docs/content-map.json"],
  }),
  node({
    id: "docs-links", kind: "gate", tier: "fast", needs: ["docs-build"],
    run: [["node", "tools/docs-links.mjs"]],
    inputs: ["tools/docs-links.mjs", "tools/docs-guides.mjs", "docs/site/**"],
    why: "no dangling internal link across the built pages",
    mutations: ["docs-dangling-link"],
  }),
  node({
    id: "docs-consistency", kind: "gate", tier: "fast", needs: ["docs-build"],
    run: [["node", "tools/docs-consistency.mjs"]],
    inputs: ["tools/docs-consistency.mjs", "src/docs/theme-prepaint.mjs", "docs/site/**", "dist/**",
             "docs/demos/**", "docs/catalog.json", "package.json"],
    why: "the site tree must be the byte-exact skinned image of dist + authored demos, " +
         "every taught @import must resolve, and no page may teach React imports",
    mutations: ["docs-consistency-site-drift", "docs-consistency-react-import"],
  }),
  node({
    id: "docs-fidelity", kind: "gate", tier: "fast", needs: ["docs-build"],
    run: [["node", "tools/docs-fidelity.mjs"]],
    inputs: ["tools/docs-fidelity.mjs", "tools/docs-fidelity-lib.mjs", "tools/docs-guides.mjs",
             "src/docs/transforms.mjs", "docs/site/**", "docs/content-map.json", "docs/content/**",
             "src/registry/pin.json"],
    why: "every built page matches its mdx source (headings/TOC/previews/fences) — " +
         "catches silent content loss that render and console checks cannot see",
    mutations: ["docs-fidelity-drop-heading"],
  }),
  node({
    id: "docs-smoke", kind: "gate", tier: "full", needs: ["docs-build"],
    run: [["node", "tools/docs-smoke.mjs", "--all"]],
    inputs: ["tools/docs-smoke.mjs", "tools/docs-guides.mjs", "docs/site/**"],
    why: "every page and every iframe loads with zero console/page errors",
    mutations: ["docs-smoke-broken-iframe"],
  }),
  node({
    id: "docs-upstream", kind: "gate", tier: "full", needs: ["docs-build"],
    run: [["node", "tools/docs-upstream.mjs"]],
    inputs: ["tools/docs-upstream.mjs", "docs/site/**"],
    why: "the built site matches the ui.shadcn.com visual contract (chrome, shiki " +
         "palettes, gutters, layout regression guards)",
    mutations: ["docs-upstream-chrome-drift"],
  }),
  node({
    id: "interactivity-sweep", kind: "gate", tier: "full", needs: ["docs-build"],
    run: [["node", "tools/interactivity-sweep.mjs"]],
    inputs: ["tools/interactivity-sweep.mjs", "docs/site/**", "src/registry/tiers.json",
             "gates/ledger.json"],
    why: "every page that OFFERS an interaction must RESPOND — contracts click " +
         "fixtures, golden compares snapshots, smoke listens to the console; the " +
         "dead-button bug lived in exactly that responsibility gap",
    mutations: ["interactivity-strip-script"],
  }),

  node({
    id: "reproducible", kind: "gate", tier: "medium",
    needs: ["docs-build", "product-build", "demo-rtl", "example-fixture"],
    run: [["./build/pipeline", "gate", "reproducible"]],
    inputs: null, // never fresh: judges state outside the tree
    why: "the committed generated trees must equal a fresh pipeline run — the only " +
         "authority on hand-edited outputs, replacing the pre-commit hook's guesswork",
    mutations: ["reproducible-hand-edit"],
  }),

  // ------------------------------------------------------- golden dual hop
  node({
    id: "golden-gate", kind: "gate", tier: "full", needs: ["example-oracle"],
    run: [["node", "tools/example-golden.mjs"]],
    inputs: ["tools/example-golden.mjs", "src/registry/upstream-snapshot/**", "gates/ledger.json",
             "tools/oracle-lib.mjs", "tools/contracts/stubs/**", "tools/resolve-skins.mjs",
             "src/registry/pin.json", "package-lock.json"],
    why: "hop 1 — the local React oracle render must equal the committed " +
         "ui.shadcn.com snapshot",
    mutations: ["golden-perturb-oracle"],
  }),
  node({
    id: "example-gate", kind: "gate", tier: "full", needs: ["docs-build"],
    run: [["node", "tools/example-oracle.mjs", "--check"]],
    inputs: ["tools/example-oracle.mjs", "docs/demos/**", "docs/example-oracle.json",
             "src/registry/tiers.json", "tools/oracle-lib.mjs", "tools/contracts/stubs/**",
             "tools/resolve-skins.mjs", "src/registry/pin.json", "package-lock.json"],
    why: "hop 2 — each shipped demo page must equal a fresh oracle render. " +
         "hop1 + hop2 together prove shipped == React == live",
    mutations: ["example-perturb-shipped"],
  }),
]

export const BY_ID = new Map(NODES.map((n) => [n.id, n]))

export const TIERS = ["fast", "medium", "full"]
export const tierRank = (t) => TIERS.indexOf(t)

// Transitive closure of `needs`, topologically sorted. Throws on unknown id
// and on cycles — both are authoring errors in this file, caught at load.
export function plan(targetIds) {
  const seen = new Set()
  const order = []
  const visiting = new Set()
  const visit = (id, path) => {
    if (seen.has(id)) return
    if (visiting.has(id)) throw new Error(`cycle in registry: ${[...path, id].join(" -> ")}`)
    const n = BY_ID.get(id)
    if (!n) throw new Error(`unknown node id: ${id}${path.length ? ` (needed by ${path.at(-1)})` : ""}`)
    visiting.add(id)
    for (const dep of n.needs) visit(dep, [...path, id])
    visiting.delete(id)
    seen.add(id)
    order.push(n)
  }
  for (const id of targetIds) visit(id, [])
  return order
}

// EFFECTIVE tier of a node = the most expensive tier in its dependency
// closure, not the tier it declares. A "fast" gate that can only run after a
// playwright build is not fast, and pretending otherwise made --tier=fast pull
// the whole oracle render in. Computed once, memoized.
const effCache = new Map()
export function effectiveTier(id) {
  if (effCache.has(id)) return effCache.get(id)
  const n = BY_ID.get(id)
  let worst = tierRank(n.tier)
  effCache.set(id, TIERS[worst]) // cycle guard; plan() reports real cycles
  for (const dep of n.needs) worst = Math.max(worst, tierRank(effectiveTier(dep)))
  const t = TIERS[worst]
  effCache.set(id, t)
  return t
}

// Every gate whose ENTIRE closure fits within `tier`, plus its builds.
export function planTier(tier) {
  const max = tierRank(tier)
  const gates = NODES.filter((n) => n.kind === "gate" && tierRank(effectiveTier(n.id)) <= max)
  return plan(gates.map((n) => n.id))
}

// Build nodes with no gate anywhere downstream: artifacts that ship without
// anything asserting they are correct. Surfaced by gates/meta.mjs.
export function ungatedBuilds() {
  const gated = new Set()
  for (const n of NODES) if (n.kind === "gate") for (const d of plan([n.id])) gated.add(d.id)
  return NODES.filter((n) => n.kind === "build" && !gated.has(n.id)).map((n) => n.id)
}
