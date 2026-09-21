//! Port of pipeline/nodes.go — THE pipeline graph, single source of truth.
//! Transcribed field-for-field: any drift shows up as a key mismatch in the
//! `status` golden-diff against the Go binary. `inputs: None` is Go's nil
//! (never fresh); `produces: None` is Go's nil (no declared outputs).
//!
//! Two graph shapes live here:
//!
//! - `all_go()` is the authored, Go-verbatim table — the byte-parity oracle
//!   the acceptance harness (tests/gen_golden.sh + tests/golden.rs) runs
//!   against the Go binary. Transcription errors surface there as key
//!   mismatches.
//! - `all()` returns the graph the engine actually runs. Default is the
//!   self-hosted shape (`self_host`): every node executes this binary, not
//!   the Go one. `SHADLESS_GRAPH=go-mirror` selects `all_go()` verbatim for
//!   the harness.

// Two pinned-upstream trees that several nodes read DIRECTLY (nodes.go:
// declared whole rather than per-file; over-declaring an input only costs a
// rerun).
const UPSTREAM_EXAMPLES_GLOB: &str = ".upstream/shadcn-ui/apps/v4/examples/**";
const UPSTREAM_DOCS_GLOB: &str = ".upstream/shadcn-ui/apps/v4/content/docs/**";

// docsUpstreamMirror: the tracked, git-diffable copy of the upstream docs .mdx.
const DOCS_UPSTREAM_MIRROR: &str = "generated/docs-upstream";

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub kind: String, // "build" (produces artifacts) | "gate" (asserts, produces nothing)
    pub tier: String, // "fast" | "full"
    pub needs: Vec<String>,
    pub run: Vec<Vec<String>>,
    pub inputs: Option<Vec<String>>, // None = judges state outside the tree, never fresh
    pub produces: Option<Vec<String>>,
    pub why: String,
    pub mutations: Vec<String>,
}

impl Node {
    pub fn never_fresh(&self) -> bool {
        self.inputs.is_none()
    }
}

#[allow(clippy::too_many_arguments)]
fn node(
    id: &str,
    kind: &str,
    tier: &str,
    needs: &[&str],
    run: &[&[&str]],
    inputs: Option<&[&str]>,
    produces: Option<&[&str]>,
    why: &str,
    mutations: &[&str],
) -> Node {
    Node {
        id: id.to_string(),
        kind: kind.to_string(),
        tier: tier.to_string(),
        needs: needs.iter().map(|s| s.to_string()).collect(),
        run: run
            .iter()
            .map(|c| c.iter().map(|s| s.to_string()).collect())
            .collect(),
        inputs: inputs.map(|v| v.iter().map(|s| s.to_string()).collect()),
        produces: produces.map(|v| v.iter().map(|s| s.to_string()).collect()),
        why: why.to_string(),
        mutations: mutations.iter().map(|s| s.to_string()).collect(),
    }
}

/// The Go-verbatim graph, in declaration order (nodes.go `var Nodes`).
/// Authored parity data — do not edit except to track upstream nodes.go
/// changes: `self_host` transforms this table into the engine-run graph and
/// the byte-compare acceptance harness against the real Go keyer lives at
/// the go-engine-final tag (pipeline/probe/keys-go), not in this tree. The
/// pipeline/* inputs and `go test` commands below are load-bearing parity
/// history — the transform strips the former and rewrites the latter.
pub fn all_go() -> Vec<Node> {
    vec![
        node(
            "pin", "gate", "fast",
            &[],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestPin$", "."]],
            Some(&["pipeline/gate_pin.go", "pipeline/gates_test.go", "src/registry/pin.json", "vendor/**", ".upstream/shadcn-ui/.git/HEAD"]),
            None,
            "the .upstream checkout must sit exactly at the pinned release tag; upgrade tools write pin.json directly and nothing else checks the result",
            &["pin-commit-drift", "pin-base-drift"],
        ),
        node(
            "unit", "gate", "fast",
            &["build-js"],
            &[&["node", "tools/unit-check.mjs"], &["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestUnit", "./..."]],
            Some(&[
                "tools/unit-check.mjs", "tools/unit/**", "src/**", "generated/ir/**", "tools/**/*.mjs",
                "vendor/**", "package.json", "dist/esm/**", "dist/shadless.js", "probes/h4/globals.css",
                ".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css", "pipeline/*.go",
                "pipeline/internal/**", "docs/example-oracle.json", "docs/example-fixture-targets.json",
            ]),
            None,
            "seconds-level guard over the pure functions cleanup rounds touch; born from a dead-code delete in rewritePaths that only surfaced minutes later",
            &["unit-break-pure-fn"],
        ),
        node(
            "typecheck", "gate", "fast",
            &[],
            &[&["npx", "tsc", "-p", "tsconfig.json"]],
            Some(&["src/**", "tsconfig.json", "package.json", "package-lock.json"]),
            None,
            "the JSDoc types are the IR/emit contract — checkJs keeps them machine-checked; npm run verify alone would not enforce it for direct pipeline runs",
            &["typecheck-break-ir-contract"],
        ),
        node(
            "ledger", "gate", "fast",
            &[],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestLedger$", "."]],
            Some(&[
                "pipeline/ledger.go", "pipeline/jssource.go", "pipeline/interactivity_sweep.go",
                "pipeline/gates_test.go", "gates/ledger.json", "gates/*-baseline.json",
                "src/registry/pin.json", "src/registry/upstream-snapshot/exemptions.json",
                "tools/contracts/components/**", "src/**", "EXEMPTIONS.md",
            ]),
            None,
            "every recorded exemption must be schema-valid, still present in its source, and inside its budget — scattered flags rot silently",
            &["ledger-undocumented-exemption", "ledger-budget-exceeded"],
        ),
        node(
            "script-refs", "gate", "fast",
            &[],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestScriptRefs$", "."]],
            Some(&["pipeline/gate_script_refs.go", "pipeline/gates_test.go", "pipeline/main.go", "pipeline/*_test.go", "package.json", "Makefile"]),
            None,
            "a node/pipeline call in Makefile or package.json scripts must resolve to a real file, a real pipeline subcommand, or a real go test — the Go port deleted gates/overlay.mjs, gates/path-parity.mjs, tools/example-oracle.mjs, tools/example-fixture.mjs and tools/css-direction-gate.mjs while three Makefile targets and four npm scripts kept calling them, silent until run",
            &["script-refs-dead-node-call"],
        ),
        node(
            "dist-complete", "gate", "fast",
            &["demo-css", "product-css"],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestDistComplete$", "."]],
            Some(&["pipeline/gate_dist_complete.go", "pipeline/gates_test.go", "dist/css/**", "dist/out.css"]),
            None,
            "the tracked no-build dist/out.css must carry every slot selector its per-component sources declare — a partial-build out.css (static pages only) got committed once and no gate asked whether the file was whole",
            &["dist-complete-drop-component"],
        ),
        node(
            "pack", "gate", "fast",
            &["build-js", "product-build"],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestPack$", "."]],
            Some(&["pipeline/gate_pack.go", "pipeline/gates_test.go", "package.json", "README.md", "dist/**"]),
            None,
            "the npm surface — exports map, tarball contents, README specifiers, an empty dependencies — must agree: a bare-string ./runtime.min export served an IIFE to `import`, README documented a specifier that does not resolve, and a React-free package installed React through dependencies",
            &["pack-broken-export"],
        ),
        node(
            "coverage", "gate", "fast",
            &["convert", "example-oracle"],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestCoverage$", "."]],
            Some(&[
                "pipeline/gate_coverage.go", "pipeline/gate_coverage_budget.go", "pipeline/gates_test.go",
                "gates/ledger.json", "src/registry/tiers.json", "generated/ir/**", "docs/example-oracle.json",
                "docs/demos/**", "tools/contracts/components/**",
            ]),
            Some(&["build/gates/coverage.json"]),
            "the product surface (component x path x theme x dir x state) as a matrix with the gate covering each cell; the UNCOVERED count is budgeted and may only shrink — every historical bug sat in a cell no gate had been written for",
            &["coverage-drop-contract"],
        ),
        node(
            "overlay", "gate", "fast",
            &["convert", "example-oracle"],
            &[&["./build/pipeline", "overlay", "--audit"]],
            Some(&[
                "pipeline/overlay.go", "pipeline/default_content.go", "pipeline/docs_transforms.go",
                "pipeline/emitter_css.go", "pipeline/resolve_skins.go", "pipeline/jssource.go",
                "pipeline/browser_shell.go", "pipeline/build_rtl.go", "pipeline/main.go",
                "tools/browser-shell.mjs", "tools/contracts/components/**", "src/emitter/skin.mjs",
                "src/registry/rtl-translations.json", "generated/ir/**", "src/registry/pin.json",
                "src/runtime/components/**", "src/runtime/core.js", "overlays/**", "docs/example-oracle.json",
                "docs/example-fixture-targets.json", "docs/demos/**",
                UPSTREAM_EXAMPLES_GLOB, UPSTREAM_DOCS_GLOB,
            ]),
            None,
            "every manual intervention on top of the mechanical conversion (rule tables, hand-written fixtures/glue/runtime, upstream patches) must still apply to the pinned upstream — orphaned rules and stale authored files fail here, with task packets, instead of silently no-op'ing like the old find/replace overlay",
            &["overlay-stale-authored", "overlay-orphaned-rule"],
        ),
        node(
            "convert", "build", "full",
            &["pin", "build-js"],
            &[&["./build/pipeline", "resolve-skins"], &["./build/pipeline", "convert"]],
            Some(&[
                "pipeline/convert.go", "pipeline/resolve_skins.go", "pipeline/internal/twmerge/**",
                "pipeline/internal/tsx/**", "src/registry/tiers.json", "src/registry/pin.json",
                "src/kernel/**", ".upstream/shadcn-ui/apps/v4/registry/bases/radix/**",
                ".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css",
            ]),
            Some(&["generated/ir", "build/resolved-ui"]),
            "registry .tsx -> IR JSON, with its own drift gate against the pinned source",
            &[],
        ),
        node(
            "emit", "build", "full",
            &["convert", "build-js"],
            &[&["./build/pipeline", "emit"]],
            Some(&[
                "pipeline/emit.go", "pipeline/emitter_css.go", "pipeline/emitter_html.go",
                "pipeline/default_content.go", "pipeline/prepaint.go", "pipeline/tags.go",
                "pipeline/resolve_skins.go", "src/registry/tiers.json", "generated/ir/**",
                "probes/h4/globals.css",
            ]),
            Some(&["dist/components/*.html", "!dist/components/*-rtl-*.html", "dist/shadless.css", "build/emit"]),
            "static-tier emit: IR -> component html + per-slot css",
            &[],
        ),
        node(
            "build-js", "build", "fast",
            &[],
            &[&["./build/pipeline", "build-js"]],
            Some(&["pipeline/jsbuild.go", "src/runtime/**", "vendor/**"]),
            Some(&["dist/shadless.js", "dist/js", "dist/shadless.min.js", "dist/esm"]),
            "the JS surface: dist/shadless.js (kernel + base) and dist/js/<name>.js per component",
            &[],
        ),
        node(
            "contract-fixture", "build", "full",
            &["convert", "build-js"],
            &[&["./build/pipeline", "example-fixture", "--contracts"]],
            Some(&[
                "pipeline/example_fixture.go", "pipeline/ef_harvest_layer.js", "pipeline/ef_menu_ids.js",
                "pipeline/ef_nav_ids.js", "pipeline/ef_tabs.js", "pipeline/ef_api.js",
                "pipeline/oracle_lib.go", "pipeline/browser_shell.go", "tools/browser-shell.mjs",
                "pipeline/prepaint.go", "pipeline/docs_families.go", "pipeline/resolve_skins.go",
                "src/emitter/skin.mjs", "tools/contracts/components/**", "tools/contracts/stubs/**",
                "src/registry/pin.json", "package-lock.json", "dist/out.css",
            ]),
            Some(&["src/kernel/*.html"]),
            "the kernel contract fixtures (src/kernel/*.html) are harvested from the contract defs' own React render — hand-mirrored fixtures drifted in content and classes (the bulk of style-parity's recorded cells)",
            &[],
        ),
        node(
            "example-oracle", "build", "full",
            &["emit", "build-js"],
            &[&["./build/pipeline", "example-oracle"]],
            Some(&[
                "pipeline/example_oracle.go", "pipeline/oracle_lib.go", "pipeline/resolve_skins.go",
                "pipeline/oracle_canon.js", "pipeline/browser_shell.go", "tools/browser-shell.mjs",
                "pipeline/prepaint.go", "src/registry/tiers.json", "src/runtime/components/**",
                "docs/catalog.json", "tools/contracts/stubs/**", "src/emitter/skin.mjs",
                "src/registry/pin.json", "src/registry/upstream-snapshot/exemptions.json",
                "package-lock.json", UPSTREAM_EXAMPLES_GLOB,
            ]),
            Some(&[
                "docs/demos/*.html", "!docs/demos/*-rtl-*.html", "docs/example-oracle.json",
                "docs/example-fixture-targets.json", "dist/components/alert-demo.html",
            ]),
            "upstream examples rendered by real React+chromium BECOME the demo pages — 1:1 with upstream by construction, not by hand-mirroring",
            &["example-oracle-render-failure"],
        ),
        node(
            "example-fixture", "build", "full",
            &["example-oracle"],
            &[&["./build/pipeline", "example-fixture"]],
            Some(&[
                "pipeline/example_fixture.go", "pipeline/ef_harvest_layer.js", "pipeline/ef_menu_ids.js",
                "pipeline/ef_nav_ids.js", "pipeline/ef_tabs.js", "pipeline/ef_api.js",
                "pipeline/oracle_lib.go", "pipeline/browser_shell.go", "tools/browser-shell.mjs",
                "pipeline/prepaint.go", "pipeline/docs_families.go", "pipeline/resolve_skins.go",
                "src/emitter/skin.mjs", "docs/example-fixture-targets.json", "dist/js/**",
                "tools/contracts/stubs/**", "src/registry/pin.json", "package-lock.json",
                "dist/out.css", UPSTREAM_EXAMPLES_GLOB,
            ]),
            Some(&["docs/demos/*.html", "!docs/demos/*-rtl-*.html"]),
            "kernel-tier examples as INTERACTIVE fixtures harvested from the oracle; the oracle alone emits static snapshots with dead buttons",
            &[],
        ),
        node(
            "rtl-dict", "build", "fast",
            &["pin", "build-js"],
            &[&["./build/pipeline", "rtl-dict"]],
            Some(&["pipeline/rtl_dict.go", "src/registry/tiers.json", ".upstream/shadcn-ui/apps/v4/examples/aria/**"]),
            Some(&["src/registry/rtl-translations.json"]),
            "the RTL translation dictionaries, lifted out of upstream's aria registry into a file this repo owns — so exactly one declared edge reaches a registry we do not build from, and `reproducible` catches it drifting",
            &["rtl-dict-missing-dictionary"],
        ),
        node(
            "demo-rtl", "build", "full",
            &["example-oracle", "example-fixture", "rtl-dict", "build-js"],
            &[&["./build/pipeline", "build-rtl"]],
            Some(&["pipeline/build_rtl.go", "pipeline/rtl_dict.go", "src/registry/tiers.json"]),
            Some(&["dist/components/*-rtl-*.html", "docs/demos/*-rtl-*.html", "build/rtl-langs.json"]),
            "AR/HE/EN/FA variants derived from the Arabic oracle page + upstream dictionaries",
            &[],
        ),
        node(
            "demo", "build", "full",
            &["demo-rtl", "example-fixture", "contract-fixture", "build-js"],
            &[&["./build/pipeline", "demo"]],
            Some(&[
                "pipeline/demo.go", "pipeline/emitter_css.go", "pipeline/default_content.go",
                "pipeline/prepaint.go", "pipeline/tags.go", "pipeline/resolve_skins.go",
                "src/registry/tiers.json", "generated/ir/**", "src/kernel/**", "probes/h4/globals.css",
                "probes/t7/**", "probes/t8/**",
            ]),
            Some(&[
                "dist/globals.css", "dist/demo-index.html", "dist/css",
                "dist/components/*.html", "!dist/components/*-rtl-*.html",
            ]),
            "unified globals.css (slot rules folded in) + the demo index + the per-component @apply sources the npm surface exports",
            &[],
        ),
        node(
            "product-css", "build", "full",
            &["demo"],
            &[&["./build/pipeline", "product-css"]],
            Some(&["pipeline/product_css.go", "pipeline/main.go", "src/docs/theme-prepaint.mjs", "probes/h4/globals.css", "package-lock.json"]),
            Some(&["dist/shadless-core.css", "dist/shadless.product.css"]),
            "token extraction + the product entry — the consumer-facing surface",
            &[],
        ),
        node(
            "demo-css", "build", "full",
            &["demo"],
            &[&["./build/pipeline", "tw", "dist/globals.css", "dist/out.css", "--cwd", "."]],
            Some(&[
                "pipeline/tw.go", "pipeline/main.go", "dist/globals.css", "dist/components/**",
                "dist/js/**", "docs/demos/**", "docs/content/**", "src/kernel/**",
                "tools/contracts/out/**", "generated/ir/**", "probes/t7/**", "probes/t8/**", "package.json",
            ]),
            Some(&["dist/out.css"]),
            "the stylesheet every demo page and contract fixture actually loads",
            &[],
        ),
        node(
            "product-build", "build", "full",
            &["product-css"],
            &[
                &["./build/pipeline", "tw", "dist/shadless.product.css", "dist/shadless.full.css"],
                &["./build/pipeline", "tw", "dist/shadless.product.css", "dist/shadless.full.min.css", "--minify"],
            ],
            Some(&["pipeline/tw.go", "pipeline/main.go", "dist/shadless.product.css", "package.json"]),
            Some(&["dist/shadless.full.css", "dist/shadless.full.min.css"]),
            "the no-build distribution artifact",
            &[],
        ),
        node(
            "product-verify", "gate", "full",
            &["product-build", "demo-css"],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestProductVerify$", "."]],
            Some(&["pipeline/product_css.go", "pipeline/gates_test.go", "dist/**"]),
            None,
            "slot rules survive the product compile and docs chrome stays out of it",
            &["product-drop-slot-rule"],
        ),
        node(
            "consumer-sim", "gate", "full",
            &["product-css"],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestConsumerSim$", "."]],
            Some(&[
                "pipeline/gate_consumer_sim.go", "pipeline/tw.go", "pipeline/gates_test.go",
                "dist/css/**", "dist/shadless-core.css", "package.json", "node_modules/.bin/tailwindcss",
            ]),
            None,
            "the PRIMARY consume path, machine-checked: a scratch consumer importing core + N component files gets exactly those styles, and every component compiles ALONE (not only as part of the full product entry)",
            &["consumer-sim-unknown-utility"],
        ),
        node(
            "path-parity", "gate", "full",
            &["product-build", "oracle-css", "build-js"],
            &[&["./build/pipeline", "path-parity"]],
            Some(&[
                "pipeline/path_parity.go", "pipeline/pp_readall.js", "pipeline/parity_baseline.go",
                "pipeline/emitter_css.go", "pipeline/tags.go", "pipeline/browser_shell.go",
                "tools/browser-shell.mjs", "gates/path-parity-baseline.json", "gates/ledger.json",
                "src/emitter/skin.mjs", "generated/ir/**", "dist/css/**", "dist/shadless.full.css",
                "build/gates/oracle.css", "src/registry/pin.json",
            ]),
            None,
            "for EVERY slot, slot-only markup via css-import and via full.css must compute what React's inline classes compute under upstream's own stylesheet, in both themes and directions, at rest, per cva variant value and per attribute-driven state, with referenced child slots rendered on both sides — subsumes the retired variant-parity; found cva defaults living in fn params (attachment, marker), toggle's pressed state losing to the (0,2,0) variant qualifier, and twMerge residue (text-sm line-height)",
            &["path-parity-drop-utility", "variant-merge-defaults"],
        ),
        node(
            "demo-parity", "gate", "full",
            &["demo-css", "oracle-css", "example-oracle", "build-js"],
            &[&["./build/pipeline", "demo-parity"]],
            Some(&[
                "pipeline/demo_parity.go", "pipeline/demo_parity_collect.js", "pipeline/parity_baseline.go",
                "pipeline/browser_shell.go", "tools/browser-shell.mjs", "gates/demo-parity-baseline.json",
                "gates/ledger.json", "build/gates/oracle.css", "dist/out.css", "docs/demos/**",
                "docs/example-oracle.json",
            ]),
            None,
            "every shipped demo page's DOM under our css must compute what the SAME DOM computes under upstream's stylesheet, light/dark x ltr/rtl — same DOM on both sides, so every cell is emitted css (skin markers, slot rules leaking under inline utilities, tokens)",
            &["demo-parity-token-drift"],
        ),
        node(
            "css-direction", "gate", "fast",
            &["demo-css"],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestCssDirection", "."]],
            Some(&["pipeline/gate_css_direction.go", "pipeline/gate_css_direction_test.go", "pipeline/gates_test.go", "dist/shadless.css"]),
            None,
            "emitted physical reading-direction utilities must match the recorded set — new/gone entries mean upstream moved the RTL story",
            &["css-direction-new-physical"],
        ),
        node(
            "contracts", "gate", "full",
            &["demo-css"],
            &[&["./build/pipeline", "contracts"]],
            Some(&[
                "pipeline/contract.go", "pipeline/oracle_lib.go", "pipeline/resolve_skins.go",
                "pipeline/browser_shell.go", "tools/browser-shell.mjs", "tools/contracts/stubs/**",
                "tools/contracts/components/**", "dist/**", "src/kernel/**", "src/registry/pin.json",
                "package-lock.json", "gates/ledger.json",
            ]),
            Some(&["tools/contracts/out"]),
            "THE oracle: the pinned registry bundled with real React+radix, replayed against the shipped pages with real mouse/keyboard, incl. mounted-DOM structure",
            &["contracts-strip-glue"],
        ),
        node(
            "oracle-css", "build", "full",
            &["convert"],
            &[&["./build/pipeline", "oracle-css"]],
            Some(&[
                "pipeline/oracle_css.go", "pipeline/tw.go", "pipeline/main.go", "src/registry/pin.json",
                "build/resolved-ui/**", ".upstream/shadcn-ui/apps/v4/app/legacy-themes.css",
                ".upstream/shadcn-ui/apps/v4/package.json", "package.json",
            ]),
            Some(&["build/gates/oracle.css"]),
            "a stylesheet for the React oracle built from upstream's own globals/skin and the resolved registry — reads nothing under src/, so style-parity is no longer circular",
            &[],
        ),
        node(
            "style-parity", "gate", "full",
            &["contracts", "oracle-css", "build-js"],
            &[&["./build/pipeline", "style-parity"]],
            Some(&[
                "pipeline/style_parity.go", "pipeline/parity_baseline.go", "pipeline/browser_shell.go",
                "tools/browser-shell.mjs", "gates/style-parity-baseline.json", "gates/ledger.json",
                "tools/contracts/out/**", "tools/contracts/components/**", "build/gates/oracle.css",
                "src/registry/pin.json",
            ]),
            None,
            "computed STYLE parity vs the React oracle — 'same DOM + same css => same styles' was an inference no gate ever tested",
            &["style-parity-perturb-padding", "style-parity-recorded-value-drift"],
        ),
        node(
            "demo-smoke", "gate", "full",
            &["demo-css", "build-js"],
            &[&["./build/pipeline", "demo-smoke"]],
            Some(&[
                "pipeline/demo_smoke.go", "pipeline/browser_shell.go", "tools/browser-shell.mjs",
                "dist/**", "generated/ir/**", "src/registry/tiers.json",
            ]),
            None,
            "every dist demo page loads with zero console errors",
            &["demo-smoke-console-error"],
        ),
        node(
            "docs-catalog", "build", "full",
            &["demo"],
            &[&["./build/pipeline", "docs-catalog"]],
            Some(&[
                "pipeline/docs_catalog.go", "pipeline/jsonorder.go", "pipeline/main.go",
                "src/registry/pin.json", "src/registry/tiers.json", "dist/components/**", "docs/demos/**",
            ]),
            Some(&["docs/catalog.json"]),
            "the preview catalog the site is generated from",
            &[],
        ),
        node(
            "docs-upstream-mirror", "build", "fast",
            &["pin"],
            &[&["./build/pipeline", "docs-upstream-mirror"]],
            Some(&["pipeline/docs_upstream_mirror.go", "src/registry/pin.json", UPSTREAM_DOCS_GLOB]),
            Some(&[DOCS_UPSTREAM_MIRROR]),
            "a tracked, git-diffable copy of the upstream docs .mdx docs-build/docs-fidelity read and docs_overrides.go hand-patches — the analogue of generated/ir for the docs pipeline (pipeline/convert.go's registry .tsx -> versioned IR), so a re-pin's content change shows up as a normal git diff in this repo instead of only a build-time anchor failure",
            &[],
        ),
        node(
            "docs-build", "build", "full",
            &["docs-catalog", "docs-upstream-mirror", "demo-css", "build-js"],
            &[&["./build/pipeline", "docs-build"]],
            Some(&[
                "pipeline/docs_build.go", "pipeline/docs_guides.go", "pipeline/docs_transforms.go",
                "pipeline/docs_overrides.go", "pipeline/docs_frontmatter.go", "pipeline/docs_scripts.go",
                "pipeline/docs_families.go", "pipeline/docs_fidelity.go", "tools/prettier-batch.mjs",
                "docs/catalog.json", "docs/content/**", "dist/**", "docs/demos/**",
                "build/rtl-langs.json", "generated/ir/**", "src/registry/pin.json", "package.json",
                "package-lock.json", "generated/docs-upstream/**",
            ]),
            Some(&["docs/site/content", "docs/site/static", "docs/content-map.json"]),
            "mdx -> the mirrored site, with the dist demos copied in under the site skin",
            &[],
        ),
        node(
            "docs-site", "build", "full",
            &["docs-build"],
            &[&["zola", "--root", "docs/site", "build"]],
            Some(&[
                "docs/site/content/**", "docs/site/static/**", "docs/site/config.toml",
                "docs/site/templates/**", "docs/site/sass/**", "docs/site/themes/**",
            ]),
            Some(&["docs/site/public"]),
            "the markdown becomes a site, and a link to a page this mirror does not carry stops the build",
            &[],
        ),
        node(
            "docs-consistency", "gate", "fast",
            &["docs-build", "build-js"],
            &[&["./build/pipeline", "docs-consistency"]],
            Some(&[
                "pipeline/docs_consistency.go", "pipeline/resolve_skins.go", "docs/site/content/**",
                "dist/components/**", "dist/css/**", "docs/demos/**", "package.json",
            ]),
            None,
            "no shipped page carries skin residue, every taught @import resolves to a file, and no page teaches React imports",
            &["docs-consistency-react-import"],
        ),
        node(
            "docs-fidelity", "gate", "fast",
            &["docs-build", "build-js"],
            &[&["./build/pipeline", "docs-fidelity"]],
            Some(&[
                "pipeline/docs_fidelity_driver.go", "pipeline/docs_fidelity.go",
                "pipeline/docs_transforms.go", "pipeline/docs_overrides.go", "pipeline/docs_guides.go",
                "pipeline/docs_frontmatter.go", "docs/site/content/**", "docs/site/static/demos/**",
                "docs/content-map.json", "docs/content/**", "src/registry/pin.json",
                "generated/docs-upstream/**",
            ]),
            None,
            "every built page matches its mdx source (headings/TOC/previews/fences) — catches silent content loss that render and console checks cannot see",
            &["docs-fidelity-drop-heading"],
        ),
        node(
            "docs-smoke", "gate", "full",
            &["docs-site", "build-js"],
            &[&["./build/pipeline", "docs-smoke", "--all"]],
            Some(&["pipeline/docs_smoke.go", "pipeline/browser_shell.go", "tools/browser-shell.mjs", "docs/site/public/**"]),
            None,
            "every page and every iframe loads with zero console/page errors",
            &["docs-smoke-broken-iframe"],
        ),
        node(
            "interactivity-sweep", "gate", "full",
            &["docs-build", "build-js"],
            &[&["./build/pipeline", "interactivity-sweep"]],
            Some(&[
                "pipeline/interactivity_sweep.go", "pipeline/browser_shell.go", "tools/browser-shell.mjs",
                "docs/site/static/demos/**", "src/registry/tiers.json", "gates/ledger.json",
            ]),
            None,
            "every page that OFFERS an interaction must RESPOND — contracts click fixtures, golden compares snapshots, smoke listens to the console; the dead-button bug lived in exactly that responsibility gap",
            &["interactivity-strip-script"],
        ),
        node(
            "reproducible", "gate", "full",
            &["docs-build", "product-build", "demo-rtl", "example-fixture"],
            &[&["go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestReproducible$", "."]],
            None, // judges state outside the tree: never fresh
            None,
            "the committed generated trees must equal a fresh pipeline run — the only authority on hand-edited outputs, replacing the pre-commit hook's guesswork",
            &["reproducible-hand-edit"],
        ),
        node(
            "golden-gate", "gate", "full",
            &["example-oracle", "build-js"],
            &[&["./build/pipeline", "example-golden"]],
            Some(&[
                "pipeline/example_golden.go", "pipeline/oracle_lib.go", "pipeline/resolve_skins.go",
                "pipeline/oracle_canon.js", "pipeline/browser_shell.go", "tools/browser-shell.mjs",
                "src/registry/upstream-snapshot/**", "gates/ledger.json", "tools/contracts/stubs/**",
                "src/emitter/skin.mjs", "src/registry/pin.json", "package-lock.json",
                UPSTREAM_EXAMPLES_GLOB,
            ]),
            None,
            "hop 1 — the local React oracle render must equal the committed ui.shadcn.com snapshot",
            &["golden-perturb-oracle"],
        ),
        node(
            "example-gate", "gate", "full",
            &["docs-build", "build-js"],
            &[&["./build/pipeline", "example-oracle", "--check"]],
            Some(&[
                "pipeline/example_oracle.go", "pipeline/oracle_lib.go", "pipeline/oracle_canon.js",
                "pipeline/browser_shell.go", "tools/browser-shell.mjs", "docs/demos/**",
                "docs/example-oracle.json", "src/registry/tiers.json", "tools/contracts/stubs/**",
                "src/emitter/skin.mjs", "src/registry/pin.json", "package-lock.json",
                UPSTREAM_EXAMPLES_GLOB,
            ]),
            None,
            "hop 2 — each shipped demo page must equal a fresh oracle render. hop1 + hop2 together prove shipped == React == live",
            &["example-perturb-shipped"],
        ),
    ]
}

/// Implementation groups hashed by build.rs (order-free; the pairing test
/// against build.rs's own tables keeps the two lists identical).
pub const ENGINE_GROUPS: &[&str] = &[
    "convert", "emit", "gates", "oracle", "tools", "twmerge", "tsx",
];

/// The hull: engine-level files every node executes through (dispatch, key
/// folding, stamp/runner semantics). Must pair with build.rs::HULL_FILES —
/// the pairing test compares both lists element-for-element. A src root file
/// missing here would silently never stale the graph.
#[cfg(test)]
const HULL_FILES: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "build.rs",
    "rust-toolchain.toml",
    "src/lib.rs",
    "src/main.rs",
    "src/engine.rs",
    "src/fanout.rs",
    "src/fsutil.rs",
    "src/glob.rs",
    "src/graph.rs",
    "src/jsonorder.rs",
    "src/key.rs",
    "src/nodes.rs",
    "src/produces.rs",
    "src/runner.rs",
    "src/stamps.rs",
    "src/verify.rs",
];

/// Which groups a group's code references. Hand written and enforced by the
/// raw-text grep test below — any `crate::<dir>` / `super::super::<dir>`
/// reference found in a group's sources must be declared here (hull modules
/// need no declaration; they are in every fp). The relation may be cyclic
/// (convert→emit→convert); node_fp folds the closure as a set.
/// `tools` is absent: its files are hashed and audited per-file (see
/// TOOLS_FILE_DEPS).
const GROUP_DEPS: &[(&str, &[&str])] = &[
    ("convert", &["emit", "tsx"]),
    ("emit", &["convert", "twmerge", "tsx"]),
    ("oracle", &["convert", "emit"]),
    ("gates", &["convert", "emit", "tools"]),
    ("twmerge", &[]),
    ("tsx", &[]),
    ("jsbuild", &[]),
];

/// Per-file dependency table for src/tools/*.rs — the tools are independent
/// commands, so their fingerprints fold only the file's own hash, its
/// intra-tools imports, and the groups it references. Hand written and
/// enforced by the same raw-text grep audit (file granularity): every
/// `crate::<group>` / `super::super::<group>` head in a tools file must be
/// declared here, and every `crate::tools::<sibling>` / `super::<sibling>`
/// import must be declared in TOOLS_FILE_INTRA. `docs_transforms` and
/// `parity_baseline` are shared helpers; importers must list them so their
/// edits propagate.
const TOOLS_FILE_DEPS: &[(&str, &[&str])] = &[
    ("css_direction_update", &["gates"]),
    ("demo_parity", &["oracle"]),
    ("demo_smoke", &["emit", "oracle"]),
    ("docs_build", &["emit"]),
    ("docs_catalog", &[]),
    ("docs_consistency", &["emit"]),
    ("docs_fidelity", &["emit"]),
    ("docs_smoke", &["emit", "oracle"]),
    ("docs_transforms", &["emit"]),
    ("docs_upstream_mirror", &[]),
    ("interactivity_sweep", &["emit", "oracle"]),
    ("ir_diff", &["emit"]),
    ("mod", &[]),
    ("oracle_css", &["emit"]),
    ("overlay", &["convert", "emit", "gates", "oracle"]),
    ("oxc_probe", &["convert"]),
    ("parity_baseline", &[]),
    ("path_parity", &["emit", "oracle", "twmerge"]),
    ("resolve_skins", &["emit", "tsx", "twmerge"]),
    ("rtl_dict", &["convert", "tsx"]),
    ("style_parity", &["emit", "oracle"]),
    ("upstream", &["gates"]),
    ("upstream_snapshot", &[]),
];

/// Intra-tools imports: `use super::<sibling>` / `use crate::tools::<sibling>`
/// heads per file, same grep enforcement. Transitively closed by node_fp.
const TOOLS_FILE_INTRA: &[(&str, &[&str])] = &[
    ("demo_parity", &["parity_baseline"]),
    ("docs_build", &["docs_transforms"]),
    ("docs_catalog", &["docs_transforms"]),
    ("docs_consistency", &["docs_transforms"]),
    ("docs_fidelity", &["docs_transforms"]),
    ("overlay", &["docs_transforms"]),
    ("path_parity", &["parity_baseline"]),
    ("style_parity", &["parity_baseline"]),
    ("upstream_snapshot", &["docs_transforms"]),
];

/// Which implementation groups each node executes — the mirror of main.rs's
/// verb dispatch. `contracts:<x>` fanout shards use the `contracts` entry.
/// Nodes WITHOUT an entry carry no `__self__` argv (typecheck's tsc, docs
/// site's zola, unit's cmd[0] unit-check.mjs) and never re-key on engine
/// edits. `global` is the whole-crate hash: unit's `__gate` half runs
/// `cargo test --lib`, which compiles everything.
const NODE_ENTRIES: &[(&str, &[&str])] = &[
    ("pin", &["gates"]),
    ("unit", &["global"]),
    ("ledger", &["gates"]),
    ("script-refs", &["gates"]),
    ("dist-complete", &["gates"]),
    ("pack", &["gates"]),
    ("coverage", &["gates"]),
    ("product-verify", &["gates"]),
    ("consumer-sim", &["gates"]),
    ("css-direction", &["gates"]),
    ("reproducible", &["gates"]),
    ("overlay", &["tools:overlay"]),
    ("convert", &["convert", "tools:resolve_skins"]), // resolve-skins + convert verbs
    ("emit", &["emit"]),
    ("build-js", &["jsbuild"]),
    ("contract-fixture", &["oracle"]),
    ("example-oracle", &["oracle"]),
    ("example-fixture", &["oracle"]),
    ("rtl-dict", &["tools:rtl_dict"]),
    ("demo-rtl", &["emit"]), // build-rtl verb
    ("demo", &["emit"]),
    ("product-css", &["emit"]),
    ("demo-css", &["emit"]),      // tw verb → emit/tw.rs
    ("product-build", &["emit"]), // tw + tw --minify
    ("path-parity", &["tools:path_parity"]),
    ("demo-parity", &["tools:demo_parity"]),
    ("contracts", &["oracle"]),
    ("oracle-css", &["tools:oracle_css"]),
    ("style-parity", &["tools:style_parity"]),
    ("demo-smoke", &["tools:demo_smoke"]),
    ("docs-catalog", &["tools:docs_catalog"]),
    ("docs-upstream-mirror", &["tools:docs_upstream_mirror"]),
    ("docs-build", &["tools:docs_build"]),
    ("docs-consistency", &["tools:docs_consistency"]),
    ("docs-fidelity", &["tools:docs_fidelity"]),
    ("docs-smoke", &["tools:docs_smoke"]),
    ("interactivity-sweep", &["tools:interactivity_sweep"]),
    ("golden-gate", &["oracle"]),  // example-golden verb
    ("example-gate", &["oracle"]), // example-oracle --check
];

/// Parsed build.rs ENGINE_FPS ("name=hex;...").
fn engine_fps() -> &'static std::collections::HashMap<String, String> {
    static FPS: std::sync::OnceLock<std::collections::HashMap<String, String>> =
        std::sync::OnceLock::new();
    FPS.get_or_init(|| {
        env!("ENGINE_FPS")
            .split(';')
            .map(|pair| {
                let (k, v) = pair.split_once('=').unwrap_or_else(|| {
                    panic!("ENGINE_FPS: malformed pair {pair:?}")
                });
                (k.to_string(), v.to_string())
            })
            .collect()
    })
}

/// The entry groups for a node id, or None for nodes with no engine-run
/// command (their argv never carries a fingerprint).
fn node_entries(id: &str) -> Option<&'static [&'static str]> {
    NODE_ENTRIES
        .iter()
        .find(|(nid, _)| *nid == id)
        .map(|(_, e)| *e)
        .or_else(|| id.starts_with("contracts:").then_some(&["oracle"][..]))
}

/// Per-node engine fingerprint: the hull hash plus, per entry token — a
/// group: its hash and the transitive GROUP_DEPS closure of groups; a
/// `tools:<stem>` file: its hash, its TOOLS_FILE_INTRA closure's hashes,
/// `tools:mod` (the shared module root), and the transitive group closure
/// of every covered file's TOOLS_FILE_DEPS. All folded by name+hash in
/// sorted order. Deterministic across runs; changes iff the hull or an
/// involved piece changes.
pub fn node_fp(id: &str) -> String {
    use sha2::{Digest, Sha256};
    let entries = node_entries(id).unwrap_or_else(|| {
        panic!("node {id} has no NODE_ENTRIES but is engine-run (argv __self__/__gate)")
    });
    let fps = engine_fps();
    let mut groups: Vec<&str> = Vec::new();
    let mut tool_files: Vec<&str> = Vec::new();
    for e in entries {
        if let Some(stem) = e.strip_prefix("tools:") {
            if !tool_files.contains(&stem) {
                tool_files.push(stem);
            }
        } else if !groups.contains(&e) {
            groups.push(e);
        }
    }
    // intra-tools closure; each covered file contributes its declared groups
    let mut i = 0;
    while i < tool_files.len() {
        let stem = tool_files[i];
        if let Some((_, dirs)) = TOOLS_FILE_DEPS.iter().find(|(n, _)| *n == stem) {
            for d in *dirs {
                if !groups.contains(&d) {
                    groups.push(d);
                }
            }
        } else {
            panic!("TOOLS_FILE_DEPS: no entry for tools file {stem}");
        }
        if let Some((_, sibs)) = TOOLS_FILE_INTRA.iter().find(|(n, _)| *n == stem) {
            for s in *sibs {
                if !tool_files.contains(s) {
                    tool_files.push(s);
                }
            }
        }
        i += 1;
    }
    let mut i = 0;
    while i < groups.len() {
        let g = groups[i];
        if let Some((_, deps)) = GROUP_DEPS.iter().find(|(name, _)| *name == g) {
            for d in deps.iter() {
                if !groups.contains(d) {
                    groups.push(d);
                }
            }
        }
        i += 1;
    }
    groups.sort();
    tool_files.sort();
    if !tool_files.is_empty() && !tool_files.contains(&"mod") {
        tool_files.push("mod");
    }
    let mut h = Sha256::new();
    h.update(format!("hull={}", fps["hull"]));
    for g in &groups {
        let hash = fps.get(*g).unwrap_or_else(|| panic!("ENGINE_FPS: no hash for group {g}"));
        h.update(format!(";{g}={hash}"));
    }
    for t in &tool_files {
        let key = format!("tools:{t}");
        let hash = fps
            .get(&key)
            .unwrap_or_else(|| panic!("ENGINE_FPS: no hash for {key}"));
        h.update(format!(";{key}={hash}"));
    }
    hex::encode(h.finalize())
}

/// argv[0] for an engine-run node command: this binary, tagged with the
/// node's own fingerprint (see `node_fp`). The tag rides argv[0] so it folds
/// into the node key without any subcommand's parser seeing it; the runner
/// splits on '@' and spawns the running executable.
///
/// Under `go-mirror` the Go-verbatim shape is mandatory (shard keys are
/// byte-compared against the Go keyer), so the Go binary path goes back in.
pub fn engine_argv0(id: &str) -> String {
    if mirror_mode() {
        "./build/pipeline".to_string()
    } else {
        format!("__self__@{}", node_fp(id))
    }
}

/// Gates whose Go form was `go test -run '^TestX..'`. Self-hosted, each runs
/// the ported gate implementation in this binary via `__gate`.
const GO_TEST_GATES: &[&str] = &[
    "pin",
    "ledger",
    "script-refs",
    "dist-complete",
    "pack",
    "coverage",
    "product-verify",
    "consumer-sim",
    "css-direction",
    "reproducible",
];

/// The ids `__gate <id>` dispatches: the ten go-test-gate ports plus unit.
/// The script-refs gate validates Makefile/package.json `__gate <id>`
/// references against this table.
pub const GATE_IDS: &[&str] = &[
    "pin", "unit", "ledger", "script-refs", "dist-complete", "pack",
    "coverage", "product-verify", "consumer-sim", "css-direction",
    "reproducible",
];

/// Every top-level verb this binary dispatches (main.rs `run`'s public
/// match arms; the hidden __gate/__meta/__oxc-probe are not on this list).
/// The Makefile and package.json drive the binary through
/// `./build/pipeline <verb>`, and the script-refs gate validates those
/// references against this table — main.rs carries a two-way test pinning
/// its dispatch arms to it.
pub const VERBS: &[&str] = &[
    "plan", "list", "status", "inputs", "run", "adopt", "build-js", "build-rtl",
    "product-css", "tw", "example-oracle", "demo", "emit", "convert", "pin",
    "coverage", "ledger", "audit-boundary", "oracle-css", "docs-catalog",
    "docs-upstream-mirror", "ir-diff", "css-direction", "upstream",
    "resolve-skins", "rtl-dict", "docs-consistency", "docs-build",
    "docs-fidelity", "example-fixture", "example-golden", "contract",
    "contracts", "upstream-snapshot", "demo-smoke", "docs-smoke", "overlay",
    "interactivity-sweep", "demo-parity", "style-parity", "path-parity",
];

/// Rewrite one Go-verbatim node into the self-hosted shape.
fn self_host(n: Node) -> Node {
    let id = n.id.clone();
    let id = id.as_str();
    let mut n = n;
    if GO_TEST_GATES.contains(&id) {
        n.run = vec![vec![
            engine_argv0(id),
            "__gate".to_string(),
            n.id.clone(),
        ]];
    } else if id == "unit" {
        // cmd[0] (node tools/unit-check.mjs) tests the product's JS surface
        // and stays; only the go-test half becomes this engine's own tests.
        if let Some(second) = n.run.get_mut(1) {
            *second = vec![
                engine_argv0(id),
                "__gate".to_string(),
                "unit".to_string(),
            ];
        }
    } else {
        for cmd in n.run.iter_mut() {
            if cmd.first().map(String::as_str) == Some("./build/pipeline") {
                cmd[0] = engine_argv0(id);
            }
        }
    }
    // The Go engine is gone: every pipeline/* input was either a data read
    // (all decoupled by 71f467a) or executed implementation (covered by the
    // per-node engine fingerprint). None survive the transform.
    if let Some(inputs) = n.inputs.as_mut() {
        inputs.retain(|p| !p.starts_with("pipeline/"));
    }
    // The Go-authored needs for reproducible name only four producers, but
    // the gate reads the WHOLE generated tree (`git status` over
    // GENERATED_ROOTS) while its tailwind-driven producers scan that tree as
    // an undeclared input — a concurrent dist/docs writer can skew a
    // verdict. Order it after every producer of a generated root. This is a
    // self-hosted-shape decision: the go-mirror table is Go-verbatim
    // history and stays untouched.
    // A node that executes a repo toolchain binary must declare it (see the
    // unit_toolchain_executors_declare_their_binary test): the Go table
    // predates node_modules toolchains and only consumer-sim declared its
    // tailwind path. Keep in sync with the Command::new sites in jsbuild.rs,
    // convert/mod.rs, emit/tw.rs, tools/oracle_css.rs, oracle/oracle_lib.rs
    // and the npx tsc invocation in the typecheck node.
    const EXTRA_INPUTS: &[(&str, &str)] = &[("example-oracle", "overlays/**")];
    if let Some((_, extra)) = EXTRA_INPUTS.iter().find(|(nid, _)| *nid == id) {
        match n.inputs.as_mut() {
            Some(inputs) => {
                inputs.push(extra.to_string());
                inputs.sort();
                inputs.dedup();
            }
            None => n.inputs = Some(vec![extra.to_string()]),
        }
    }
    const TOOLCHAIN_INPUTS: &[(&str, &str)] = &[
        ("build-js", "node_modules/.bin/esbuild"),
        ("convert", "node_modules/.bin/esbuild"),
        ("typecheck", "node_modules/.bin/tsc"),
        ("demo", "node_modules/.bin/tailwindcss"),
        ("product-build", "node_modules/.bin/tailwindcss"),
        ("oracle-css", "node_modules/.bin/tailwindcss"),
        ("example-oracle", "node_modules/.bin/esbuild"),
        ("example-fixture", "node_modules/.bin/esbuild"),
        ("contract-fixture", "node_modules/.bin/esbuild"),
        ("golden-gate", "node_modules/.bin/esbuild"),
    ];
    if let Some((_, bin)) = TOOLCHAIN_INPUTS.iter().find(|(nid, _)| *nid == id) {
        match n.inputs.as_mut() {
            Some(inputs) => {
                inputs.push(bin.to_string());
                inputs.sort();
                inputs.dedup();
            }
            None => n.inputs = Some(vec![bin.to_string()]),
        }
    }
    if id == "reproducible" {
        let mut producers: Vec<String> = all_go()
            .iter()
            .filter(|o| {
                o.id != "reproducible"
                    && o.produces.iter().flatten().any(|p| {
                        p.starts_with("dist/")
                            || p.starts_with("generated/")
                            || p.starts_with("docs/")
                            || p.starts_with("src/kernel")
                    })
            })
            .map(|o| o.id.clone())
            .collect();
        producers.sort();
        producers.dedup();
        let mut needs = std::mem::take(&mut n.needs);
        needs.extend(producers);
        needs.sort();
        needs.dedup();
        n.needs = needs;
    }
    n
}

/// The graph the engine runs: self-hosted by default, `SHADLESS_GRAPH=go-mirror`
/// for the Go-parity acceptance harness.
pub fn all() -> Vec<Node> {
    if mirror_mode() {
        return all_go();
    }
    all_go().into_iter().map(self_host).collect()
}

pub fn mirror_mode() -> bool {
    std::env::var("SHADLESS_GRAPH").as_deref() == Ok("go-mirror")
}

/// (Per-node fingerprints — see `engine_argv0(id)`; fanout shards pass
/// their own `contracts:<name>` id so the shard key carries the oracle
/// group's fingerprint, not the parent's.)

#[cfg(test)]
mod self_host_tests {
    use super::*;

    /// A node that executes a repo toolchain binary must declare it: an
    /// upgraded or deleted esbuild/tailwind/tsc would otherwise leave every
    /// consumer falsely fresh (Rust-side read evidence cannot see it, and
    /// node_modules is excluded from the JS read log by design).
    /// consumer-sim already declared its tailwind path in the Go table; the
    /// self_host table is the policy for the rest.
    #[test]
    fn unit_toolchain_executors_declare_their_binary() {
        let g: Vec<Node> = all_go().into_iter().map(self_host).collect();
        for (id, bin) in [
            ("build-js", "node_modules/.bin/esbuild"),
            ("convert", "node_modules/.bin/esbuild"),
            ("typecheck", "node_modules/.bin/tsc"),
            ("demo", "node_modules/.bin/tailwindcss"),
            ("product-build", "node_modules/.bin/tailwindcss"),
            ("oracle-css", "node_modules/.bin/tailwindcss"),
            ("example-oracle", "node_modules/.bin/esbuild"),
            ("example-fixture", "node_modules/.bin/esbuild"),
            ("contract-fixture", "node_modules/.bin/esbuild"),
            ("golden-gate", "node_modules/.bin/esbuild"),
        ] {
            let n = g.iter().find(|n| n.id == id).unwrap_or_else(|| panic!("node {id} missing"));
            let inputs = n.inputs.as_deref().unwrap_or(&[]);
            assert!(
                inputs.iter().any(|p| p == bin),
                "{id} executes {bin} but does not declare it — upgrading the binary stays falsely fresh"
            );
        }
    }

    #[test]
    fn all_go_is_the_go_verbatim_shape() {
        let g = all_go();
        assert_eq!(g.len(), 41, "nodes.go declares 41 nodes");
        let gates = g.iter().filter(|n| n.kind == "gate").count();
        assert_eq!(gates, 24, "nodes.go declares 24 gates");
        // Spot-checks against the Go source, verbatim (nodes.go:122-148, 518-524):
        let pin = g.iter().find(|n| n.id == "pin").unwrap();
        assert_eq!(
            pin.run,
            vec![vec![
                "go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestPin$", "."
            ]]
        );
        let unit = g.iter().find(|n| n.id == "unit").unwrap();
        assert_eq!(unit.run.len(), 2);
        assert_eq!(unit.run[0], vec!["node", "tools/unit-check.mjs"]);
        assert_eq!(
            unit.run[1],
            vec![
                "go", "test", "-C", "pipeline", "-count=1", "-v", "-run", "^TestUnit", "./..."
            ]
        );
        assert!(unit
            .inputs
            .as_ref()
            .unwrap()
            .contains(&"pipeline/*.go".to_string()));
        let convert = g.iter().find(|n| n.id == "convert").unwrap();
        assert_eq!(
            convert.run,
            vec![
                vec!["./build/pipeline", "resolve-skins"],
                vec!["./build/pipeline", "convert"]
            ]
        );
        let reproducible = g.iter().find(|n| n.id == "reproducible").unwrap();
        assert_eq!(reproducible.inputs, None, "judges state outside the tree");
    }

    #[test]
    fn self_hosted_table_spawns_no_go() {
        let go = all_go();
        let g: Vec<Node> = go.iter().cloned().map(self_host).collect();
        assert_eq!(g.len(), 41);
        assert_eq!(g.iter().filter(|n| n.kind == "gate").count(), 24);
        for n in &g {
            for (i, cmd) in n.run.iter().enumerate() {
                assert!(
                    !cmd.iter().any(|a| a == "go" || a == "./build/pipeline"),
                    "{} still references Go: {:?}",
                    n.id,
                    cmd
                );
                // The only commands not run by this engine are the
                // product-side tools the Go table already ran directly:
                // unit's JS checker, typecheck's tsc, docs-site's zola.
                let product_side = matches!(
                    (n.id.as_str(), cmd.first().map(String::as_str)),
                    ("unit", Some("node")) | ("typecheck", Some("npx")) | ("docs-site", Some("zola"))
                );
                if !product_side {
                    let argv0 = cmd.first().map(String::as_str).unwrap_or("");
                    assert!(
                        argv0.starts_with("__self__@") && argv0.len() > "__self__@".len(),
                        "{} cmd {} not engine-run: {:?}",
                        n.id,
                        i,
                        cmd
                    );
                }
            }
            // Structural fields must survive the transform untouched —
            // except reproducible's needs, which self_host deliberately
            // widens to every producer of a generated root (the gate reads
            // the whole generated tree while tailwind-driven producers scan
            // it as an undeclared input).
            let go_n = go.iter().find(|m| m.id == n.id).unwrap();
            assert_eq!(n.kind, go_n.kind, "{}", n.id);
            assert_eq!(n.tier, go_n.tier, "{}", n.id);
            assert_eq!(n.produces, go_n.produces, "{}", n.id);
            assert_eq!(n.mutations, go_n.mutations, "{}", n.id);
            if n.id == "reproducible" {
                for o in &go {
                    let produced_root = o.produces.iter().flatten().any(|p| {
                        p.starts_with("dist/")
                            || p.starts_with("generated/")
                            || p.starts_with("docs/")
                            || p.starts_with("src/kernel")
                    });
                    if o.id != "reproducible" && produced_root {
                        assert!(
                            n.needs.contains(&o.id),
                            "reproducible must need the generated-root producer {}",
                            o.id
                        );
                    }
                }
            } else {
                assert_eq!(n.needs, go_n.needs, "{}", n.id);
            }
        }
    }

    #[test]
    fn self_hosted_pipeline_inputs_are_exactly_the_keep_list() {
        let g: Vec<Node> = all_go().into_iter().map(self_host).collect();
        let mut surviving: Vec<(String, String)> = Vec::new();
        for n in &g {
            if let Some(inputs) = &n.inputs {
                for p in inputs {
                    if p.starts_with("pipeline/") {
                        surviving.push((n.id.clone(), p.clone()));
                    }
                }
            }
        }
        assert!(
            surviving.is_empty(),
            "no node may keep a pipeline/ input any more — the Go files are gone"
        );
    }

    #[test]
    fn unit_keeps_product_check_and_swaps_only_the_engine_half() {
        let g: Vec<Node> = all_go().into_iter().map(self_host).collect();
        let unit = g.iter().find(|n| n.id == "unit").unwrap();
        assert_eq!(unit.run.len(), 2);
        assert_eq!(unit.run[0], vec!["node", "tools/unit-check.mjs"]);
        assert_eq!(
            unit.run[1],
            vec![engine_argv0("unit").as_str(), "__gate", "unit"]
        );
        // The go-test inputs (pipeline/*.go, pipeline/internal/**) are gone;
        // the product side stays.
        let inputs = unit.inputs.as_ref().unwrap();
        assert!(!inputs.iter().any(|p| p.starts_with("pipeline/")));
        assert!(inputs.contains(&"tools/unit-check.mjs".to_string()));
        assert!(inputs.contains(&"src/**".to_string()));
    }

    #[test]
    fn node_verbs_are_engine_verbs() {
        // The self-hosted graph drives this binary as `__self__@fp <verb>`;
        // every verb it can name must be in VERBS (the table script-refs
        // validates Makefile/package.json against).
        for n in all() {
            for cmd in &n.run {
                if cmd.first().map_or(false, |c| c.starts_with("__self__")) {
                    let verb = cmd.get(1).expect("engine command carries a verb");
                    if verb == "__gate" {
                        continue; // the hidden gate dispatcher: id, not a verb
                    }
                    assert!(
                        VERBS.contains(&verb.as_str()),
                        "{}: verb `{verb}` is not in VERBS",
                        n.id
                    );
                }
            }
        }
    }

    #[test]
    fn gate_ids_are_exactly_the_dispatched_gates() {
        // GATE_IDS is what script-refs validates build-file references
        // against; the __gate dispatcher must accept precisely the same set
        // (an id here but not dispatchable would pass the gate and die at
        // runtime, and vice versa).
        let mut expected: Vec<&str> = GO_TEST_GATES.to_vec();
        expected.push("unit");
        expected.sort();
        let mut ids: Vec<&str> = GATE_IDS.to_vec();
        ids.sort();
        assert_eq!(ids, expected);
    }

    // ---- fingerprint table enforcement ---------------------------------
    //
    // The fp scheme's soundness lives here, not in build.rs (which only
    // hashes directories). These tests over-approximate on purpose: raw
    // text, comments and string literals all count as references.

    /// The engine-run tables in build.rs (GROUPS, HULL_FILES) and here must
    /// stay identical — build.rs hashes by them, the fp composer keys off
    /// the same names.
    #[test]
    fn build_rs_tables_pair_with_nodes_rs_tables() {
        let build_src = include_str!("../build.rs");
        let groups_line = build_src
            .lines()
            .find(|l| l.starts_with("const GROUPS"))
            .expect("build.rs GROUPS");
        for g in ENGINE_GROUPS {
            assert!(
                groups_line.contains(&format!("\"{g}\"")),
                "group {g} missing from build.rs GROUPS"
            );
        }
        let hull_start = build_src.find("const HULL_FILES").expect("build.rs HULL_FILES");
        let hull_end = build_src[hull_start..].find("];").unwrap() + hull_start;
        let hull_block = &build_src[hull_start..hull_end];
        let build_hull: Vec<&str> = hull_block
            .lines()
            .filter_map(|l| l.trim().strip_prefix('"')?.split('"').next())
            .collect();
        let nodes_hull: Vec<&str> = HULL_FILES.to_vec();
        assert_eq!(build_hull, nodes_hull, "HULL_FILES diverged");
    }

    /// Every src root file must be in the hull list (or jsbuild.rs): a file
    /// outside every group and the hull would silently never stale anything.
    /// Covers non-.rs files too — include_str!-ed assets at the root would
    /// otherwise compile into the binary while staling nothing. Also: every
    /// DIRECTORY under src must be a declared group — a new tier dir would
    /// otherwise stale nothing except unit (global).
    #[test]
    fn every_root_file_is_in_the_hull() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let rs_roots: Vec<String> = crate::fsutil::sorted_read_dir(&src)
            .expect("src readable")
            .into_iter()
            .filter(|n| n.ends_with(".rs") && n != "jsbuild.rs")
            .collect();
        for f in &rs_roots {
            let rel = format!("src/{f}");
            assert!(
                HULL_FILES.contains(&rel.as_str()),
                "{rel} is neither in HULL_FILES (build.rs + this file) nor jsbuild.rs — \
                 it would never stale the graph"
            );
        }
        let mut non_rs: Vec<String> = std::fs::read_dir(&src)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map_or(false, |t| t.is_file()))
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| !n.ends_with(".rs"))
            .collect();
        non_rs.sort();
        assert!(
            non_rs.is_empty(),
            "non-.rs files at src root are not covered by any fingerprint group \
             (add them to HULL_FILES in build.rs + this file, or move them into a \
             group dir): {non_rs:?}"
        );
        let mut dirs: Vec<String> = std::fs::read_dir(&src)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map_or(false, |t| t.is_dir()))
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        dirs.sort();
        for d in &dirs {
            assert!(
                ENGINE_GROUPS.contains(&d.as_str()),
                "src/{d}/ is not a declared ENGINE_GROUP — nodes executing it would \
                 never stale on its edits (add it to GROUPS in build.rs + here)"
            );
        }
    }

    /// Every src/tools/*.rs file must be covered by some node's fingerprint
    /// closure (entry file or transitively imported sibling), or be an
    /// explicitly exempted CLI-only verb dispatched from the hull (its code
    /// never executes under any graph node).
    #[test]
    fn every_tools_file_is_covered_by_a_node_fp() {
        // CLI-only verbs: main.rs (hull) dispatches them, no node runs them.
        // `mod` is the tools module root, folded into every tools fp.
        let exempt: Vec<&str> = vec![
            "mod",                   // tools module root, always folded
            "css_direction_update",  // verb css-direction --update
            "ir_diff",               // verb ir-diff
            "oxc_probe",             // verb __oxc-probe
            "upstream",              // verb upstream (the re-pin drill)
            "upstream_snapshot",     // verb upstream-snapshot
        ];
        let mut covered: Vec<&str> = Vec::new();
        for (_, entries) in NODE_ENTRIES {
            let mut files: Vec<&str> = entries
                .iter()
                .filter_map(|e| e.strip_prefix("tools:"))
                .collect();
            let mut i = 0;
            while i < files.len() {
                if let Some((_, sibs)) =
                    TOOLS_FILE_INTRA.iter().find(|(n, _)| *n == files[i])
                {
                    for s in *sibs {
                        if !files.contains(&s) {
                            files.push(s);
                        }
                    }
                }
                i += 1;
            }
            for f in files {
                if !covered.contains(&f) {
                    covered.push(f);
                }
            }
        }
        let tools_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tools");
        let mut stems: Vec<String> = std::fs::read_dir(&tools_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".rs"))
            .map(|n| n.trim_end_matches(".rs").to_string())
            .collect();
        stems.sort();
        for stem in &stems {
            assert!(
                covered.contains(&stem.as_str()) || exempt.contains(&stem.as_str()),
                "src/tools/{stem}.rs is claimed by no node fp and is not in the \
                 CLI-only exemption list — it would never stale anything. Add a \
                 NODE_ENTRIES tools:{stem} entry or exempt it here."
            );
        }
        for stem in &exempt {
            assert!(
                !covered.contains(stem),
                "exempt tools file {stem} is also claimed by a node entry — drop it \
                 from the exemption list"
            );
        }
    }

    /// Every engine-run node needs fp entries, and every entry is a known
    /// group; the set of nodes carrying `__self__` after self_host must be
    /// exactly NODE_ENTRIES.
    #[test]
    fn node_entries_cover_exactly_the_engine_run_nodes() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let go = all_go();
        let self_hosted: Vec<String> = go
            .iter()
            .map(|n| self_host(n.clone()))
            .filter(|n| n.run.iter().any(|cmd| cmd.first().map_or(false, |a| a.starts_with("__self__@"))))
            .map(|n| n.id)
            .collect();
        let listed: Vec<String> = NODE_ENTRIES
            .iter()
            .map(|(id, _)| id.to_string())
            .collect();
        for id in &listed {
            assert!(
                go.iter().any(|n| &n.id == id),
                "NODE_ENTRIES lists {id}, which no node declares"
            );
        }
        let mut a = self_hosted.clone();
        let mut b = listed.clone();
        a.sort();
        b.sort();
        assert_eq!(a, b, "NODE_ENTRIES must list exactly the engine-run nodes");
        for (id, entries) in NODE_ENTRIES {
            assert!(!entries.is_empty(), "{id}: empty entries");
            for e in entries.iter() {
                assert!(
                    *e == "global"
                        || *e == "jsbuild"
                        || e.starts_with("tools:")
                        || ENGINE_GROUPS.contains(e),
                    "{id}: unknown entry group {e}"
                );
                if let Some(stem) = e.strip_prefix("tools:") {
                    assert!(
                        src.join("tools").join(format!("{stem}.rs")).exists(),
                        "{id}: entry {e} names a nonexistent tools file"
                    );
                }
            }
        }
        // fanout shards resolve through the contracts prefix
        assert_eq!(node_entries("contracts:tooltip"), Some(&["oracle"][..]));
        assert_eq!(node_entries("no-such-node"), None);
    }

    /// Raw-text dependency audit: any cross-group module reference in a
    /// group's sources must be declared in GROUP_DEPS. Hull modules are
    /// exempt (present in every fp). Deliberately crude: comments, string
    /// literals and cfg(test) blocks all count — over-stale, never falsely
    /// fresh. Also bans the two idioms this audit cannot see through:
    /// brace-grouped crate imports and module-path renames.
    #[test]
    fn group_deps_declare_every_cross_group_reference() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        // crate-root single-file modules: reference = hull, no declaration needed
        let hull_modules: Vec<String> = HULL_FILES
            .iter()
            .filter(|f| f.starts_with("src/") && f.ends_with(".rs"))
            .map(|f| f.trim_start_matches("src/").trim_end_matches(".rs").to_string())
            .collect();
        // Crate-root FUNCTIONS in lib.rs: lib.rs is a hull file, so its bytes
        // are folded into every group's fp — a reference to one of these can
        // never go falsely fresh. (The head regex only sees module-shaped
        // paths, so crate-root items need this explicit list.)
        const HULL_ROOT_ITEMS: &[&str] = &["crate_adjacent_tree_root", "tree_root"];
        let lib_src = std::fs::read_to_string(src.join("lib.rs")).expect("src/lib.rs readable");
        for item in HULL_ROOT_ITEMS {
            assert!(
                lib_src.contains(&format!("fn {item}")),
                "HULL_ROOT_ITEMS: `{item}` is not defined in src/lib.rs — the \
                 allowlist is only sound for crate-root functions (lib.rs is \
                 hull); move the item or update the list"
            );
        }

        let mut targets: Vec<(String, std::path::PathBuf)> = Vec::new();
        for g in ENGINE_GROUPS {
            if *g == "tools" {
                // audited per FILE below (TOOLS_FILE_DEPS granularity)
                continue;
            }
            targets.push((g.to_string(), src.join(g)));
        }
        targets.push(("jsbuild".to_string(), src.join("jsbuild.rs")));
        let tools_dir = src.join("tools");
        let mut tools_stems: Vec<String> = std::fs::read_dir(&tools_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".rs"))
            .map(|n| n.trim_end_matches(".rs").to_string())
            .collect();
        tools_stems.sort();
        for stem in &tools_stems {
            targets.push((
                format!("tools:{stem}"),
                tools_dir.join(format!("{stem}.rs")),
            ));
        }

        let head_re = regex::Regex::new(r"\b(?:crate|pipeline)::([a-z_][a-z0-9_]*)").unwrap();
        let supersuper_re = regex::Regex::new(r"\bsuper::super::([a-z_][a-z0-9_]*)").unwrap();
        let super_mod_rs_re = regex::Regex::new(r"\bsuper::([a-z_][a-z0-9_]*)").unwrap();
        let crate_tools_re = regex::Regex::new(r"\bcrate::tools::([a-z_][a-z0-9_]*)").unwrap();
        let banned_brace = regex::Regex::new(r"crate::\{").unwrap();
        let banned_alias = regex::Regex::new(r"use (?:crate|pipeline)::[a-z_][a-z0-9_]* as ").unwrap();

        for (group, path) in &targets {
            let is_tools_file = group.starts_with("tools:");
            let files: Vec<std::path::PathBuf> = if path.is_dir() {
                walkdir::WalkDir::new(path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |x| x == "rs"))
                    .map(|e| e.path().to_path_buf())
                    .collect()
            } else {
                vec![path.clone()]
            };
            let mut found: Vec<String> = Vec::new();
            let mut siblings: Vec<String> = Vec::new();
            for f in &files {
                let text = std::fs::read_to_string(f)
                    .unwrap_or_else(|e| panic!("{}: {}", f.display(), e));
                assert!(
                    banned_brace.find(&text).is_none(),
                    "{}: `crate::{{...}}` brace imports are invisible to the dependency \
                     audit — split them into one `use crate::x::y;` per path",
                    f.display()
                );
                assert!(
                    banned_alias.find(&text).is_none(),
                    "{}: `use crate::x as y` renames hide module references from the \
                     dependency audit — import through the full path instead",
                    f.display()
                );
                let is_top_mod_rs = f.file_name().map_or(false, |n| n == "mod.rs")
                    && f
                        .parent()
                        .and_then(|p| p.parent())
                        .map_or(false, |p| p == src);
                for caps in head_re.captures_iter(&text) {
                    found.push(caps[1].to_string());
                }
                for caps in supersuper_re.captures_iter(&text) {
                    found.push(caps[1].to_string());
                }
                if is_top_mod_rs {
                    // `super::` in a top-level dir's mod.rs IS the crate root
                    for caps in super_mod_rs_re.captures_iter(&text) {
                        found.push(caps[1].to_string());
                    }
                }
                if is_tools_file {
                    // `super::x` / `crate::tools::x` in a tools file: a
                    // sibling FILE (needs TOOLS_FILE_INTRA) or an item of
                    // tools/mod.rs (covered by tools:mod, folded into every
                    // tools fp)
                    for caps in super_mod_rs_re.captures_iter(&text).chain(
                        crate_tools_re.captures_iter(&text),
                    ) {
                        let head = &caps[1];
                        if head == "super" {
                            continue; // super::super, handled above
                        }
                        if tools_dir.join(format!("{head}.rs")).exists() {
                            siblings.push(head.to_string());
                        }
                    }
                }
            }
            let declared: &[&str] = if is_tools_file {
                let stem = group.strip_prefix("tools:").expect("tools: prefix");
                TOOLS_FILE_DEPS
                    .iter()
                    .find(|(name, _)| *name == stem)
                    .map(|(_, deps)| *deps)
                    .unwrap_or_else(|| panic!("TOOLS_FILE_DEPS: no entry for {group}"))
            } else {
                GROUP_DEPS
                    .iter()
                    .find(|(name, _)| name == group)
                    .map(|(_, deps)| *deps)
                    .unwrap_or_else(|| panic!("GROUP_DEPS: no entry for group {group}"))
            };
            let stem = group.strip_prefix("tools:");
            let declared_siblings: &[&str] = stem
                .and_then(|s| {
                    TOOLS_FILE_INTRA
                        .iter()
                        .find(|(n, _)| *n == s)
                        .map(|(_, deps)| *deps)
                })
                .unwrap_or(&[]);
            for head in &found {
                if head == group
                    || hull_modules.contains(head)
                    || HULL_ROOT_ITEMS.contains(&head.as_str())
                    || head == "global"
                    || Some(head.as_str()) == stem
                {
                    continue; // intra-group or hull (in every fp already)
                }
                assert!(
                    ENGINE_GROUPS.contains(&head.as_str()) || head == "jsbuild",
                    "{group}: reference to `{head}` is neither a group nor a hull \
                     module — extend the audit's module table"
                );
                assert!(
                    declared.contains(&head.as_str()),
                    "{group}: reference to group `{head}` missing from declared deps — \
                     either declare it or the fp scheme goes falsely fresh"
                );
            }
            for sib in &siblings {
                assert!(
                    declared_siblings.contains(&sib.as_str()),
                    "{group}: import of sibling tools file `{sib}` missing from \
                     TOOLS_FILE_INTRA — its edits would go unseen by this fp"
                );
            }
        }
    }

    /// Remove `#[test]`- and `#[cfg(test)]`-attributed brace blocks: test
    /// code does not compile into the shipped binary, so its includes are
    /// exempt from the fingerprint-coverage audit.
    fn strip_test_attributed_items(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut rest = text;
        loop {
            let Some(idx) = rest.find("#[test]").or_else(|| rest.find("#[cfg(test)]")) else {
                out.push_str(rest);
                break;
            };
            out.push_str(&rest[..idx]);
            rest = &rest[idx..];
            let end = rest.find('{').and_then(|b| {
                let mut depth = 0;
                for (off, ch) in rest[b..].char_indices() {
                    match ch {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                return Some(b + off + 1);
                            }
                        }
                        _ => {}
                    }
                }
                None
            });
            match end {
                Some(end) => {
                    out.push(' ');
                    rest = &rest[end..];
                }
                // attribute without a brace block (e.g. on a use item): keep
                // scanning after the attribute itself
                None => {
                    out.push_str(rest.get(..14).unwrap_or(rest));
                    rest = rest.get(14..).unwrap_or("");
                }
            }
        }
        out
    }

    /// Non-test `include_str!`/`include!`/`include_bytes!` paths must stay
    /// inside `src/`: anything else compiled into the binary (probe assets,
    /// files next to the crate) is invisible to every fingerprint group.
    /// `#[test]`/`#[cfg(test)]` items are stripped first — test code does not
    /// ship (the jsonorder Go-golden include and this file's build.rs pairing
    /// read live there).
    #[test]
    fn product_includes_stay_inside_src() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let src_real = std::fs::canonicalize(&src).unwrap();
        let include_re =
            regex::Regex::new(r#"include(?:_str|_bytes)?!\(\s*"([^"]+)""#).unwrap();
        for entry in walkdir::WalkDir::new(&src)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |x| x == "rs"))
        {
            let path = entry.path();
            let text = std::fs::read_to_string(path)
                .unwrap_or_else(|e| panic!("{}: {}", path.display(), e));
            let text = strip_test_attributed_items(&text);
            for caps in include_re.captures_iter(&text) {
                let found = &caps[1];
                let target = path.parent().unwrap().join(found);
                // canonicalize: lexical `..` after the src prefix would fool a
                // component-wise starts_with. Unresolvable = fail closed.
                let resolved = std::fs::canonicalize(&target);
                assert!(
                    resolved.as_ref().map(|p| p.starts_with(&src_real)).unwrap_or(false),
                    "{}: `{found}` escapes src/ and compiles into the binary while being \
                     hashed by no fingerprint group — move it under src/ (hashed by \
                     its group) or into HULL_FILES",
                    path.display()
                );
            }
        }
    }

    /// fp properties that document the granularity: same-entry nodes share a
    /// fp, different entries differ, and hull edits change everything
    /// (simulated by composing — the real hull-sensitivity is shell-verified
    /// by file-touch checks).
    #[test]
    fn node_fp_granularity() {
        // tools files are hashed individually: peers differ, and the shared
        // helpers propagate (docs-smoke folds docs_transforms via its own
        // imports? no — docs-smoke imports nothing; it differs from
        // docs-fidelity which folds the helper)
        assert_ne!(node_fp("docs-smoke"), node_fp("demo-smoke"), "per-file tools fps");
        assert_ne!(node_fp("docs-smoke"), node_fp("docs-fidelity"));
        assert_ne!(node_fp("docs-smoke"), node_fp("contracts:tooltip"));
        assert_ne!(node_fp("pin"), node_fp("unit"));
        // engine-run nodes always carry a nonempty tag; fp is stable
        assert_eq!(node_fp("docs-smoke"), node_fp("docs-smoke"));
        assert!(engine_argv0("docs-smoke").starts_with("__self__@"));
        assert_eq!(
            engine_argv0("docs-smoke").split('@').next().unwrap(),
            "__self__"
        );
    }
}
