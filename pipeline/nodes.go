// pipeline/nodes.go — THE pipeline graph. Single source of truth.
//
// Before this file the gate set lived in two places that had drifted apart:
// `Makefile:verify` ran 11 gates, `tools/verify.mjs` ran 23 steps, and the
// intersection was 3. Either one alone could be green while a whole class of
// checks never executed. Every entry point now derives from this array, so
// "the gate set" is one reviewable list.
//
// It was generated from gates/registry.mjs while wireit still executed the
// graph and the JS runner was the real one. Both are gone: the Go runner
// executes this array directly, so the generator, the JS registry and the
// pipeline-sync gate that guarded the gap between them were deleted together
// (they existed only to keep two copies in step). The definitions live in Go
// so the COMPILER checks them: `Needs` is []NodeID referencing the constants
// below, so a typo in a dependency list is a build error rather than a
// runtime lookup failure.
//
// Each node is a build step or a gate:
//
//	ID        stable name — what `pipeline run <id>` takes
//	Kind      "build" (produces artifacts) | "gate" (asserts, produces nothing)
//	Needs     ids that must run first. A real dependency, not an ordering
//	          preference: the runner topo-sorts and, for a targeted run,
//	          builds exactly the transitive closure and nothing else.
//	Run       argv arrays, executed in order
//	Tier      "fast" (~seconds, no browser) | "full" (everything else)
//	Inputs    globs of everything the node READS (its own script, the modules
//	          it imports, the data it opens). Outputs of Needs are implied —
//	          the runner folds a dependency's key into its dependents'. This
//	          is what lets `pipeline run <id>` skip a node nothing touched.
//	          nil = judges state outside the tree, never fresh.
//	Why       one line: what breaks if this node is deleted. TestMeta
//	          requires every gate to carry one.
//	Mutations ids under gates/mutations/ that MUST make this gate fail.
//	          A gate with no mutation is unproven and TestMeta rejects it.
//
// Ordering inside the array is irrelevant; Needs decides. Keep it grouped by
// phase for human reading only.

package main

// NodeID is a defined type, not an alias: a bare string cannot stand in for
// one, so every reference below has to go through a generated constant and a
// typo in a dependency list is a compile error.
type NodeID string

const (
	NPin                NodeID = "pin"
	NUnit               NodeID = "unit"
	NLedger             NodeID = "ledger"
	NDistComplete       NodeID = "dist-complete"
	NPack               NodeID = "pack"
	NCoverage           NodeID = "coverage"
	NOverlay            NodeID = "overlay"
	NConvert            NodeID = "convert"
	NEmit               NodeID = "emit"
	NBuildJs            NodeID = "build-js"
	NContractFixture    NodeID = "contract-fixture"
	NDemoBuild          NodeID = "demo-build"
	NExampleOracle      NodeID = "example-oracle"
	NExampleFixture     NodeID = "example-fixture"
	NDemoRtl            NodeID = "demo-rtl"
	NDemo               NodeID = "demo"
	NProductCss         NodeID = "product-css"
	NDemoCss            NodeID = "demo-css"
	NProductBuild       NodeID = "product-build"
	NProductVerify      NodeID = "product-verify"
	NConsumerSim        NodeID = "consumer-sim"
	NPathParity         NodeID = "path-parity"
	NDemoParity         NodeID = "demo-parity"
	NCssDirection       NodeID = "css-direction"
	NContracts          NodeID = "contracts"
	NOracleCss          NodeID = "oracle-css"
	NStyleParity        NodeID = "style-parity"
	NDemoSmoke          NodeID = "demo-smoke"
	NDocsCatalog        NodeID = "docs-catalog"
	NDocsBuild          NodeID = "docs-build"
	NDocsLinks          NodeID = "docs-links"
	NDocsConsistency    NodeID = "docs-consistency"
	NDocsFidelity       NodeID = "docs-fidelity"
	NDocsSmoke          NodeID = "docs-smoke"
	NDocsUpstream       NodeID = "docs-upstream"
	NInteractivitySweep NodeID = "interactivity-sweep"
	NReproducible       NodeID = "reproducible"
	NGoldenGate         NodeID = "golden-gate"
	NExampleGate        NodeID = "example-gate"
)

// Nodes is the pipeline graph, in declaration order.
var Nodes = []Node{
	{
		ID: NPin, Kind: "gate", Tier: "fast",
		Needs:     nil,
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestPin$", "."}},
		Inputs:    []string{"pipeline/gate_pin.go", "pipeline/gates_test.go", "src/registry/pin.json", "vendor/**", ".upstream/shadcn-ui/.git/HEAD"},
		Produces:  nil,
		Why:       "the .upstream checkout must sit exactly at the pinned release tag; upgrade tools write pin.json directly and nothing else checks the result",
		Mutations: []string{"pin-commit-drift"},
	},
	{
		ID: NUnit, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NBuildJs},
		Run:       [][]string{{"node", "tools/unit-check.mjs"}, {"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestUnit", "."}},
		Inputs:    []string{"tools/unit-check.mjs", "tools/unit/**", "src/**", "tools/**/*.mjs", "vendor/**", "package.json", "dist/esm/**", "dist/shadless.js", "probes/h4/globals.css", ".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css", "pipeline/*_test.go", "pipeline/gate_css_direction.go", "pipeline/product_css.go"},
		Produces:  nil,
		Why:       "seconds-level guard over the pure functions cleanup rounds touch; born from a dead-code delete in rewritePaths that only surfaced minutes later",
		Mutations: []string{"unit-break-pure-fn"},
	},
	{
		ID: NLedger, Kind: "gate", Tier: "fast",
		Needs:     nil,
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestLedger$", "."}},
		Inputs:    []string{"pipeline/ledger.go", "pipeline/jssource.go", "pipeline/gates_test.go", "gates/ledger.json", "gates/*-baseline.json", "src/registry/pin.json", "src/registry/upstream-snapshot/exemptions.json", "tools/contracts/components/**", "tools/interactivity-sweep.mjs", "src/**"},
		Produces:  nil,
		Why:       "every recorded exemption must be schema-valid, still present in its source, and inside its budget — scattered flags rot silently",
		Mutations: []string{"ledger-undocumented-exemption", "ledger-budget-exceeded"},
	},
	{
		ID: NDistComplete, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NDemoCss, NProductCss},
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestDistComplete$", "."}},
		Inputs:    []string{"pipeline/gate_dist_complete.go", "pipeline/gates_test.go", "dist/css/**", "dist/out.css"},
		Produces:  nil,
		Why:       "the tracked no-build dist/out.css must carry every slot selector its per-component sources declare — a partial-build out.css (static pages only) got committed once and no gate asked whether the file was whole",
		Mutations: []string{"dist-complete-drop-component"},
	},
	{
		ID: NPack, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NBuildJs, NProductBuild},
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestPack$", "."}},
		Inputs:    []string{"pipeline/gate_pack.go", "pipeline/gates_test.go", "package.json", "README.md", "dist/**"},
		Produces:  nil,
		Why:       "the npm surface — exports map, tarball contents, README specifiers, an empty dependencies — must agree: a bare-string ./runtime.min export served an IIFE to `import`, README documented a specifier that does not resolve, and a React-free package installed React through dependencies",
		Mutations: []string{"pack-broken-export"},
	},
	{
		ID: NCoverage, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NConvert, NExampleOracle},
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestCoverage$", "."}},
		Inputs:    []string{"pipeline/gate_coverage.go", "pipeline/gate_coverage_budget.go", "pipeline/gates_test.go", "gates/ledger.json", "src/registry/tiers.json", "src/registry/ir/**", "docs/example-oracle.json", "docs/demos/**", "tools/contracts/components/**", "tools/interactivity-sweep.mjs"},
		Produces:  []string{"build/gates/coverage.json"},
		Why:       "the product surface (component x path x theme x dir x state) as a matrix with the gate covering each cell; the UNCOVERED count is budgeted and may only shrink — every historical bug sat in a cell no gate had been written for",
		Mutations: []string{"coverage-drop-contract"},
	},
	{
		ID: NOverlay, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NConvert, NExampleOracle},
		Run:       [][]string{{"node", "gates/overlay.mjs", "--audit"}},
		Inputs:    []string{"gates/overlay.mjs", "overlays/**", "src/**", "tools/**/*.mjs", "docs/example-oracle.json", "docs/example-fixture-targets.json", "docs/demos/**", "src/registry/pin.json"},
		Produces:  nil,
		Why:       "every manual intervention on top of the mechanical conversion (rule tables, hand-written fixtures/glue/runtime, upstream patches) must still apply to the pinned upstream — orphaned rules and stale authored files fail here, with task packets, instead of silently no-op'ing like the old find/replace overlay",
		Mutations: []string{"overlay-stale-authored", "overlay-orphaned-rule"},
	},
	{
		ID: NConvert, Kind: "build", Tier: "full",
		Needs:     []NodeID{NPin},
		Run:       [][]string{{"node", "tools/resolve-skins.mjs"}, {"node", "src/converter/index.mjs"}},
		Inputs:    []string{"tools/resolve-skins.mjs", "src/converter/**", "src/tags.mjs", "src/emitter/skin.mjs", "src/registry/tiers.json", "src/registry/pin.json", "src/kernel/**", ".upstream/shadcn-ui/apps/v4/registry/bases/radix/**", ".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css"},
		Produces:  []string{"src/registry/ir", "build/resolved-ui"},
		Why:       "registry .tsx -> IR JSON, with its own drift gate against the pinned source",
		Mutations: nil,
	},
	{
		ID: NEmit, Kind: "build", Tier: "full",
		Needs:    []NodeID{NConvert},
		Run:      [][]string{{"node", "src/emitter/index.mjs"}},
		Inputs:   []string{"src/emitter/**", "src/tags.mjs", "src/docs/theme-prepaint.mjs", "src/registry/tiers.json", "src/registry/pin.json", "probes/h4/globals.css", "package.json", "pipeline/tw.go", "pipeline/main.go"},
		Produces: []string{"dist/components/*.html", "!dist/components/*-rtl-*.html", "dist/shadless.css", "build/emit"},
		// build/emit is still produced (the emitter writes globals.css and a
		// demo index there) but nothing reads it now that emit-smoke is gone.
		Why:       "static-tier emit: IR -> component html + per-slot css",
		Mutations: nil,
	},
	{
		ID: NBuildJs, Kind: "build", Tier: "fast",
		Needs:     nil,
		Run:       [][]string{{"./build/pipeline", "build-js"}},
		Inputs:    []string{"pipeline/jsbuild.go", "src/runtime/**", "vendor/**"},
		Produces:  []string{"dist/shadless.js", "dist/js", "dist/shadless.min.js", "dist/esm"},
		Why:       "the JS surface: dist/shadless.js (kernel + base) and dist/js/<name>.js per component",
		Mutations: nil,
	},
	{
		ID: NContractFixture, Kind: "build", Tier: "full",
		Needs:     []NodeID{NConvert, NBuildJs},
		Run:       [][]string{{"node", "tools/example-fixture.mjs", "--contracts"}},
		Inputs:    []string{"tools/example-fixture.mjs", "tools/contracts/oracle-build.mjs", "tools/contracts/components/**", "tools/fixture-families.mjs", "src/docs/theme-prepaint.mjs", "docs/example-fixture-targets.json", "tools/oracle-lib.mjs", "tools/contracts/stubs/**", "tools/resolve-skins.mjs", "src/registry/pin.json", "package-lock.json"},
		Produces:  []string{"src/kernel/*.html"},
		Why:       "the kernel contract fixtures (src/kernel/*.html) are harvested from the contract defs' own React render — hand-mirrored fixtures drifted in content and classes (the bulk of style-parity's recorded cells)",
		Mutations: nil,
	},
	{
		ID: NDemoBuild, Kind: "build", Tier: "full",
		Needs:     []NodeID{NEmit},
		Run:       [][]string{{"node", "tools/build-demo.mjs"}},
		Inputs:    []string{"tools/build-demo.mjs", "src/docs/theme-prepaint.mjs"},
		Produces:  []string{"dist/components/*-demo.html", "!dist/components/*-rtl-*.html"},
		Why:       "single-demo compositions from the upstream examples",
		Mutations: nil,
	},
	{
		ID: NExampleOracle, Kind: "build", Tier: "full",
		Needs:     []NodeID{NDemoBuild},
		Run:       [][]string{{"node", "tools/example-oracle.mjs"}},
		Inputs:    []string{"tools/example-oracle.mjs", "src/docs/theme-prepaint.mjs", "src/registry/tiers.json", "src/runtime/components/**", "docs/catalog.json", "tools/oracle-lib.mjs", "tools/contracts/stubs/**", "tools/resolve-skins.mjs", "src/registry/pin.json", "src/registry/upstream-snapshot/exemptions.json", "package-lock.json"},
		Produces:  []string{"docs/demos/*.html", "!docs/demos/*-rtl-*.html", "docs/example-oracle.json", "docs/example-fixture-targets.json"},
		Why:       "upstream examples rendered by real React+chromium BECOME the demo pages — 1:1 with upstream by construction, not by hand-mirroring",
		Mutations: []string{"example-oracle-render-failure"},
	},
	{
		ID: NExampleFixture, Kind: "build", Tier: "full",
		Needs:     []NodeID{NExampleOracle},
		Run:       [][]string{{"node", "tools/example-fixture.mjs"}},
		Inputs:    []string{"tools/example-fixture.mjs", "tools/contracts/oracle-build.mjs", "tools/contracts/components/**", "tools/fixture-families.mjs", "src/docs/theme-prepaint.mjs", "docs/example-fixture-targets.json", "dist/js/**", "tools/oracle-lib.mjs", "tools/contracts/stubs/**", "tools/resolve-skins.mjs", "src/registry/pin.json", "package-lock.json"},
		Produces:  []string{"docs/demos/*.html", "!docs/demos/*-rtl-*.html"},
		Why:       "kernel-tier examples as INTERACTIVE fixtures harvested from the oracle; the oracle alone emits static snapshots with dead buttons",
		Mutations: nil,
	},
	{
		ID: NDemoRtl, Kind: "build", Tier: "full",
		Needs:     []NodeID{NExampleOracle},
		Run:       [][]string{{"node", "tools/build-rtl.mjs"}},
		Inputs:    []string{"tools/build-rtl.mjs", "tools/rtl-lib.mjs", "src/docs/theme-prepaint.mjs", "src/registry/pin.json"},
		Produces:  []string{"dist/components/*-rtl-*.html", "docs/demos/*-rtl-*.html", "build/rtl-langs.json"},
		Why:       "AR/HE/EN/FA variants derived from the Arabic oracle page + upstream dictionaries",
		Mutations: nil,
	},
	{
		ID: NDemo, Kind: "build", Tier: "full",
		Needs:     []NodeID{NDemoRtl, NExampleFixture, NContractFixture, NBuildJs},
		Run:       [][]string{{"node", "tools/demo.mjs"}},
		Inputs:    []string{"tools/demo.mjs", "tools/demo-lib.mjs", "tools/build-js.mjs", "src/emitter/css.mjs", "src/docs/theme-prepaint.mjs", "src/registry/tiers.json", "src/registry/ir/**", "src/kernel/**", "probes/h4/globals.css", "probes/t7/**", "probes/t8/**"},
		Produces:  []string{"dist/globals.css", "dist/demo-index.html", "dist/css", "dist/components/*.html", "!dist/components/*-rtl-*.html"},
		Why:       "unified globals.css (slot rules folded in) + the demo index + the per-component @apply sources the npm surface exports",
		Mutations: nil,
	},
	{
		ID: NProductCss, Kind: "build", Tier: "full",
		Needs:     []NodeID{NDemo},
		Run:       [][]string{{"./build/pipeline", "product-css"}},
		Inputs:    []string{"pipeline/product_css.go", "pipeline/main.go", "src/docs/theme-prepaint.mjs", "probes/h4/globals.css", "package-lock.json"},
		Produces:  []string{"dist/shadless-core.css", "dist/shadless.product.css"},
		Why:       "token extraction + the product entry — the consumer-facing surface",
		Mutations: nil,
	},
	{
		ID: NDemoCss, Kind: "build", Tier: "full",
		Needs:     []NodeID{NDemo},
		Run:       [][]string{{"./build/pipeline", "tw", "dist/globals.css", "dist/out.css", "--cwd", "."}},
		Inputs:    []string{"pipeline/tw.go", "pipeline/main.go", "dist/globals.css", "dist/components/**", "dist/js/**", "docs/demos/**", "docs/content/**", "src/kernel/**", "tools/contracts/out/**", "src/registry/ir/**", "probes/t7/**", "probes/t8/**"},
		Produces:  []string{"dist/out.css"},
		Why:       "the stylesheet every demo page and contract fixture actually loads",
		Mutations: nil,
	},
	{
		ID: NProductBuild, Kind: "build", Tier: "full",
		Needs:     []NodeID{NProductCss},
		Run:       [][]string{{"./build/pipeline", "tw", "dist/shadless.product.css", "dist/shadless.full.css"}, {"./build/pipeline", "tw", "dist/shadless.product.css", "dist/shadless.full.min.css", "--minify"}},
		Inputs:    []string{"pipeline/tw.go", "pipeline/main.go", "dist/shadless.product.css"},
		Produces:  []string{"dist/shadless.full.css", "dist/shadless.full.min.css"},
		Why:       "the no-build distribution artifact",
		Mutations: nil,
	},
	{
		ID: NProductVerify, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NProductBuild, NDemoCss},
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestProductVerify$", "."}},
		Inputs:    []string{"pipeline/product_css.go", "pipeline/gates_test.go", "dist/**"},
		Produces:  nil,
		Why:       "slot rules survive the product compile and docs chrome stays out of it",
		Mutations: []string{"product-drop-slot-rule"},
	},
	{
		ID: NConsumerSim, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NProductCss},
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestConsumerSim$", "."}},
		Inputs:    []string{"pipeline/gate_consumer_sim.go", "pipeline/tw.go", "pipeline/gates_test.go", "dist/css/**", "dist/shadless-core.css", "package.json", "node_modules/.bin/tailwindcss"},
		Produces:  nil,
		Why:       "the PRIMARY consume path, machine-checked: a scratch consumer importing core + N component files gets exactly those styles, and every component compiles ALONE (not only as part of the full product entry)",
		Mutations: []string{"consumer-sim-unknown-utility"},
	},
	{
		ID: NPathParity, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NProductBuild, NOracleCss},
		Run:       [][]string{{"node", "gates/path-parity.mjs"}},
		Inputs:    []string{"gates/path-parity.mjs", "gates/parity-baseline.mjs", "gates/path-parity-baseline.json", "gates/ledger.json", "src/emitter/css.mjs", "src/tags.mjs", "src/registry/ir/**", "dist/css/**", "dist/shadless.full.css", "build/gates/oracle.css", "src/registry/pin.json", "pipeline/tw.go", "pipeline/main.go"},
		Produces:  nil,
		Why:       "for EVERY slot, slot-only markup via css-import and via full.css must compute what React's inline classes compute under upstream's own stylesheet, in both themes and directions, at rest, per cva variant value and per attribute-driven state, with referenced child slots rendered on both sides — subsumes the retired variant-parity; found cva defaults living in fn params (attachment, marker), toggle's pressed state losing to the (0,2,0) variant qualifier, and twMerge residue (text-sm line-height)",
		Mutations: []string{"path-parity-drop-utility", "variant-merge-defaults"},
	},
	{
		ID: NDemoParity, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NDemoCss, NOracleCss, NExampleOracle},
		Run:       [][]string{{"node", "gates/demo-parity.mjs"}},
		Inputs:    []string{"gates/demo-parity.mjs", "gates/parity-baseline.mjs", "gates/demo-parity-baseline.json", "gates/ledger.json", "build/gates/oracle.css", "dist/out.css", "docs/demos/**", "docs/example-oracle.json"},
		Produces:  nil,
		Why:       "every shipped demo page's DOM under our css must compute what the SAME DOM computes under upstream's stylesheet, light/dark x ltr/rtl — same DOM on both sides, so every cell is emitted css (skin markers, slot rules leaking under inline utilities, tokens)",
		Mutations: []string{"demo-parity-token-drift"},
	},
	{
		ID: NCssDirection, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NDemoCss},
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestCssDirection", "."}},
		Inputs:    []string{"pipeline/gate_css_direction.go", "pipeline/gate_css_direction_test.go", "pipeline/gates_test.go", "dist/shadless.css"},
		Produces:  nil,
		Why:       "emitted physical reading-direction utilities must match the recorded set — new/gone entries mean upstream moved the RTL story",
		Mutations: []string{"css-direction-new-physical"},
	},
	{
		ID: NContracts, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NDemoCss},
		Run:       [][]string{{"npm", "run", "contracts"}},
		Inputs:    []string{"tools/contracts/**/*.mjs", "tools/contracts/stubs/**", "tools/contracts/components/**", "tools/oracle-lib.mjs", "dist/**", "src/kernel/**", "src/registry/pin.json", "package-lock.json", "gates/ledger.json"},
		Produces:  []string{"tools/contracts/out"},
		Why:       "THE oracle: the pinned registry bundled with real React+radix, replayed against the shipped pages with real mouse/keyboard, incl. mounted-DOM structure",
		Mutations: []string{"contracts-strip-glue"},
	},
	{
		ID: NOracleCss, Kind: "build", Tier: "full",
		Needs:     []NodeID{NConvert},
		Run:       [][]string{{"./build/pipeline", "oracle-css"}},
		Inputs:    []string{"pipeline/oracle_css.go", "pipeline/tw.go", "pipeline/main.go", "src/registry/pin.json", "build/resolved-ui/**"},
		Produces:  []string{"build/gates/oracle.css"},
		Why:       "a stylesheet for the React oracle built from upstream's own globals/skin and the resolved registry — reads nothing under src/, so style-parity is no longer circular",
		Mutations: nil,
	},
	{
		ID: NStyleParity, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NContracts, NOracleCss},
		Run:       [][]string{{"node", "tools/style-parity.mjs"}},
		Inputs:    []string{"tools/style-parity.mjs", "gates/parity-baseline.mjs", "gates/style-parity-baseline.json", "gates/ledger.json", "tools/contracts/out/**", "tools/contracts/components/**", "build/gates/oracle.css", "src/registry/pin.json"},
		Produces:  nil,
		Why:       "computed STYLE parity vs the React oracle — 'same DOM + same css => same styles' was an inference no gate ever tested",
		Mutations: []string{"style-parity-perturb-padding", "style-parity-recorded-value-drift"},
	},
	{
		ID: NDemoSmoke, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NDemoCss},
		Run:       [][]string{{"node", "tools/demo-smoke.mjs"}},
		Inputs:    []string{"tools/demo-smoke.mjs", "dist/**", "src/registry/ir/**", "src/registry/tiers.json"},
		Produces:  nil,
		Why:       "every dist demo page loads with zero console errors",
		Mutations: []string{"demo-smoke-console-error"},
	},
	{
		ID: NDocsCatalog, Kind: "build", Tier: "full",
		Needs:     []NodeID{NDemo},
		Run:       [][]string{{"./build/pipeline", "docs-catalog"}},
		Inputs:    []string{"pipeline/docs_catalog.go", "pipeline/jsonorder.go", "pipeline/main.go", "src/registry/pin.json", "src/registry/tiers.json", "dist/components/**", "docs/demos/**"},
		Produces:  []string{"docs/catalog.json"},
		Why:       "the preview catalog the site is generated from",
		Mutations: nil,
	},
	{
		ID: NDocsBuild, Kind: "build", Tier: "full",
		Needs:     []NodeID{NDocsCatalog, NDemoCss},
		Run:       [][]string{{"node", "tools/docs-build.mjs"}},
		Inputs:    []string{"tools/docs-build.mjs", "tools/docs-guides.mjs", "tools/docs-page-lib.mjs", "tools/fixture-families.mjs", "src/docs/**", "docs/catalog.json", "docs/content/**", "docs/fonts/**", "dist/**", "docs/demos/**", "build/rtl-langs.json", "src/registry/ir/**", "src/registry/pin.json", "package.json", "package-lock.json"},
		Produces:  []string{"docs/site", "docs/content-map.json"},
		Why:       "mdx -> the mirrored site, with the dist demos copied in under the site skin",
		Mutations: nil,
	},
	{
		ID: NDocsLinks, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NDocsBuild},
		Run:       [][]string{{"node", "tools/docs-links.mjs"}},
		Inputs:    []string{"tools/docs-links.mjs", "tools/docs-guides.mjs", "docs/site/**"},
		Produces:  nil,
		Why:       "no dangling internal link across the built pages",
		Mutations: []string{"docs-dangling-link"},
	},
	{
		ID: NDocsConsistency, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NDocsBuild},
		Run:       [][]string{{"node", "tools/docs-consistency.mjs"}},
		Inputs:    []string{"tools/docs-consistency.mjs", "src/docs/theme-prepaint.mjs", "docs/site/**", "dist/**", "docs/demos/**", "docs/catalog.json", "package.json"},
		Produces:  nil,
		Why:       "the site tree must be the byte-exact skinned image of dist + authored demos, every taught @import must resolve, and no page may teach React imports",
		Mutations: []string{"docs-consistency-site-drift", "docs-consistency-react-import"},
	},
	{
		ID: NDocsFidelity, Kind: "gate", Tier: "fast",
		Needs:     []NodeID{NDocsBuild},
		Run:       [][]string{{"node", "tools/docs-fidelity.mjs"}},
		Inputs:    []string{"tools/docs-fidelity.mjs", "tools/docs-fidelity-lib.mjs", "tools/docs-guides.mjs", "src/docs/transforms.mjs", "docs/site/**", "docs/content-map.json", "docs/content/**", "src/registry/pin.json"},
		Produces:  nil,
		Why:       "every built page matches its mdx source (headings/TOC/previews/fences) — catches silent content loss that render and console checks cannot see",
		Mutations: []string{"docs-fidelity-drop-heading"},
	},
	{
		ID: NDocsSmoke, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NDocsBuild},
		Run:       [][]string{{"node", "tools/docs-smoke.mjs", "--all"}},
		Inputs:    []string{"tools/docs-smoke.mjs", "tools/docs-guides.mjs", "docs/site/**"},
		Produces:  nil,
		Why:       "every page and every iframe loads with zero console/page errors",
		Mutations: []string{"docs-smoke-broken-iframe"},
	},
	{
		ID: NDocsUpstream, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NDocsBuild},
		Run:       [][]string{{"node", "tools/docs-upstream.mjs"}},
		Inputs:    []string{"tools/docs-upstream.mjs", "docs/site/**"},
		Produces:  nil,
		Why:       "the built site matches the ui.shadcn.com visual contract (chrome, shiki palettes, gutters, layout regression guards)",
		Mutations: []string{"docs-upstream-chrome-drift"},
	},
	{
		ID: NInteractivitySweep, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NDocsBuild},
		Run:       [][]string{{"node", "tools/interactivity-sweep.mjs"}},
		Inputs:    []string{"tools/interactivity-sweep.mjs", "docs/site/**", "src/registry/tiers.json", "gates/ledger.json"},
		Produces:  nil,
		Why:       "every page that OFFERS an interaction must RESPOND — contracts click fixtures, golden compares snapshots, smoke listens to the console; the dead-button bug lived in exactly that responsibility gap",
		Mutations: []string{"interactivity-strip-script"},
	},
	{
		ID: NReproducible, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NDocsBuild, NProductBuild, NDemoRtl, NExampleFixture},
		Run:       [][]string{{"go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestReproducible$", "."}},
		Inputs:    nil, // judges state outside the tree: never fresh
		Produces:  nil,
		Why:       "the committed generated trees must equal a fresh pipeline run — the only authority on hand-edited outputs, replacing the pre-commit hook's guesswork",
		Mutations: []string{"reproducible-hand-edit"},
	},
	{
		ID: NGoldenGate, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NExampleOracle},
		Run:       [][]string{{"node", "tools/example-golden.mjs"}},
		Inputs:    []string{"tools/example-golden.mjs", "src/registry/upstream-snapshot/**", "gates/ledger.json", "tools/oracle-lib.mjs", "tools/contracts/stubs/**", "tools/resolve-skins.mjs", "src/registry/pin.json", "package-lock.json"},
		Produces:  nil,
		Why:       "hop 1 — the local React oracle render must equal the committed ui.shadcn.com snapshot",
		Mutations: []string{"golden-perturb-oracle"},
	},
	{
		ID: NExampleGate, Kind: "gate", Tier: "full",
		Needs:     []NodeID{NDocsBuild},
		Run:       [][]string{{"node", "tools/example-oracle.mjs", "--check"}},
		Inputs:    []string{"tools/example-oracle.mjs", "docs/demos/**", "docs/example-oracle.json", "src/registry/tiers.json", "tools/oracle-lib.mjs", "tools/contracts/stubs/**", "tools/resolve-skins.mjs", "src/registry/pin.json", "package-lock.json"},
		Produces:  nil,
		Why:       "hop 2 — each shipped demo page must equal a fresh oracle render. hop1 + hop2 together prove shipped == React == live",
		Mutations: []string{"example-perturb-shipped"},
	},
}
