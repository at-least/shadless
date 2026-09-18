//! pipeline — the graph runner (Rust port).
//!
//!   pipeline plan   <tier|node…>   the closure, topologically sorted
//!   pipeline list   <tier|node…>   the closure, annotated, no execution
//!   pipeline status <tier|node…>   fresh / stale, per node
//!
//! Output formats are byte-identical with the Go binary (pipeline/main.go) —
//! that is the M1 acceptance bar; see tests/golden.rs.

use pipeline::glob::files;
use pipeline::graph::{load_graph_at, Graph};
use pipeline::key::{outputs_present, stamp_value, Keyer};
use pipeline::nodes::Node;
use pipeline::runner::Runner;
use pipeline::stamps::{load_stamps, write_stamp};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

fn usage() -> ! {
    // The usage text is part of the presented surface, so it follows the
    // graph shape: go-mirror reproduces the Go binary's text byte-for-byte
    // (golden layer 1 records it); the self-hosted engine describes itself.
    let text = if std::env::var("SHADLESS_GRAPH").as_deref() == Ok("go-mirror") {
        "usage: pipeline <plan|list|status|run|adopt> <fast|full|builds|all|node…>\n       pipeline pin [--check-only]\n       pipeline tw <in> <out> [--minify] [--cwd DIR]\n       pipeline oracle-css\n       pipeline product-css\n       pipeline docs-catalog\n       pipeline ir-diff <git-ref>|<dirA> <dirB> [--json]\n       pipeline css-direction --update\n       pipeline ledger --record|--render|--dissolve\n       pipeline audit-boundary [--strict|discover]\n       pipeline upstream --to=shadcn@X.Y.Z [--fetch] [--no-build]\n       pipeline build-js\n       pipeline resolve-skins [--fixtures]\n       pipeline inputs <node> [--produces]\n\nThe gates are Go tests: go test -C pipeline -count=1 -v [-run '^TestPack$']\n"
    } else {
        "usage: pipeline <plan|list|status|run|adopt> <fast|full|builds|all|node…>\n       pipeline pin [--check-only]\n       pipeline tw <in> <out> [--minify] [--cwd DIR]\n       pipeline oracle-css\n       pipeline product-css\n       pipeline docs-catalog\n       pipeline ir-diff <git-ref>|<dirA> <dirB> [--json]\n       pipeline css-direction --update\n       pipeline ledger --record|--render|--dissolve\n       pipeline audit-boundary [--strict|discover]\n       pipeline upstream --to=shadcn@X.Y.Z [--fetch] [--no-build]\n       pipeline build-js\n       pipeline resolve-skins [--fixtures]\n       pipeline inputs <node> [--produces]\n\nThe engine runs its own gates: every node executes this binary (__gate / subcommands). SHADLESS_GRAPH=go-mirror presents the historical Go-verbatim graph; the Go engine itself lives at the go-engine-final tag.\n"
    };
    eprint!("{}", text);
    std::process::exit(2);
}

fn die(err: impl std::fmt::Display) -> ! {
    eprintln!("pipeline: {}", err);
    std::process::exit(1);
}

fn resolve_targets(g: &Graph, targets: &[String]) -> Result<Vec<Node>, String> {
    if targets.len() == 1 {
        match targets[0].as_str() {
            "fast" | "full" => return g.plan_tier(&targets[0]),
            "builds" => return g.plan_builds(),
            "all" => return g.plan(g.ids()),
            _ => {}
        }
    }
    for a in targets {
        if g.node(a).is_none() {
            return Err(format!(
                "unknown node: {}\nknown: {}",
                a,
                g.ids().join(", ")
            ));
        }
    }
    g.plan(targets)
}

/// Shared flag parsing: --force / --gates-only / --builds-only / --keep-going
/// are flags, everything else is a target.
fn parse_view_args(args: &[String]) -> (bool, bool, bool, bool, Vec<String>) {
    let (mut force, mut gates_only, mut builds_only, mut keep_going) =
        (false, false, false, false);
    let mut targets: Vec<String> = Vec::new();
    for a in args {
        match a.as_str() {
            "--force" => force = true,
            "--gates-only" => gates_only = true,
            "--builds-only" => builds_only = true,
            "--keep-going" => keep_going = true,
            t => targets.push(t.to_string()),
        }
    }
    (force, gates_only, builds_only, keep_going, targets)
}

fn run_view(cmd: &str, args: &[String]) -> Result<(), String> {
    let (_force, gates_only, builds_only, _keep_going, targets) = parse_view_args(args);
    if gates_only && builds_only {
        return Err("--gates-only and --builds-only are mutually exclusive".to_string());
    }
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let g = load_graph_at(&root)?;
    let mut plan = resolve_targets(&g, &targets)?;
    // --gates-only assumes the artifacts are already fresh; --builds-only is
    // the mutation harness's prelude, which runs the gates itself.
    if gates_only {
        plan.retain(|n| n.kind == "gate");
    }
    if builds_only {
        plan.retain(|n| n.kind == "build");
    }
    match cmd {
        "plan" => {
            for n in &plan {
                println!("{}", n.id);
            }
        }
        "list" => print_list(&g, &plan),
        "status" => print_status(&root, &g, &plan)?,
        _ => unreachable!(),
    }
    Ok(())
}

fn print_list(g: &Graph, plan: &[Node]) {
    let width = plan.iter().map(|n| n.id.len()).max().unwrap_or(0);
    let mut ngates = 0;
    for n in plan {
        let (kind, is_gate) = if n.kind == "gate" { ("GATE ", true) } else { ("build", false) };
        if is_gate {
            ngates += 1;
        }
        // the EFFECTIVE tier is what decides whether a tier run picks the node
        // up; show the declared one only when they disagree
        let eff = g.effective_tier(&n.id);
        let self_note = if eff != n.tier {
            format!(" (self {})", n.tier)
        } else {
            String::new()
        };
        let needs = if n.needs.is_empty() {
            String::new()
        } else {
            format!("  needs: {}", n.needs.join(", "))
        };
        println!(
            "{} {:<width$}  [{}{}]{needs}",
            kind,
            n.id,
            eff,
            self_note,
            width = width
        );
    }
    println!();
    println!("{} nodes ({} gates)", plan.len(), ngates);
}

fn print_status(root: &Path, g: &Graph, plan: &[Node]) -> Result<(), String> {
    let mut k = Keyer::new(root, g);
    let rec = load_stamps(root);
    for n in plan {
        let key = k.key(&n.id)?;
        match key {
            None => println!("{:<22} NEVER-FRESH", n.id),
            Some(key) => {
                if rec.get(&n.id).map(String::as_str) == Some(stamp_value(root, n, &key).as_str()) {
                    match outputs_present(root, n) {
                        Err(missing) => {
                            println!("{:<22} STALE (output missing: {})", n.id, missing)
                        }
                        Ok(()) => println!("{:<22} fresh", n.id),
                    }
                } else {
                    println!("{:<22} STALE", n.id);
                }
            }
        }
    }
    Ok(())
}

fn run_inputs(args: &[String]) -> i32 {
    let mut produces = false;
    let mut id = String::new();
    for a in args {
        if a == "--produces" {
            produces = true;
            continue;
        }
        id = a.clone();
    }
    if id.is_empty() {
        eprintln!("usage: pipeline inputs <node> [--produces]");
        return 2;
    }
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    let g = match load_graph_at(&root) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    let Some(n) = g.node(&id) else {
        eprintln!("unknown node: {}", id);
        return 2;
    };
    let Some(patterns) = (if produces { n.produces.as_ref() } else { n.inputs.as_ref() }) else {
        return 0; // never-fresh or no declared set at all
    };
    let list = match files(&root, patterns) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    for f in list {
        writeln!(out, "{}", f).ok();
    }
    0
}

fn run_command(args: &[String]) -> Result<(), String> {
    let (force, gates_only, builds_only, keep_going, targets) = parse_view_args(args);
    if gates_only && builds_only {
        return Err("--gates-only and --builds-only are mutually exclusive".to_string());
    }
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let g = Arc::new(load_graph_at(&root)?);
    let mut plan = resolve_targets(&g, &targets)?;
    if gates_only {
        plan.retain(|n| n.kind == "gate");
    }
    if builds_only {
        plan.retain(|n| n.kind == "build");
    }

    let mut jobs = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    if let Ok(v) = std::env::var("PIPELINE_PARALLEL") {
        if let Ok(n) = v.parse::<usize>() {
            if n > 0 {
                jobs = n;
            }
        }
    }
    // Each browser node runs its own Chromium (~0.5–1 GB, bursty CPU during
    // render). A small cap keeps them off the contention cliff; the main
    // semaphore bounds everything, so a browser cap above -j is a lie.
    let mut browser_jobs = jobs.min(4);
    if let Ok(v) = std::env::var("PIPELINE_BROWSER_JOBS") {
        if let Ok(n) = v.parse::<usize>() {
            if n > 0 {
                browser_jobs = n;
            }
        }
    }
    browser_jobs = browser_jobs.min(jobs);
    let runner = Arc::new(Runner::new(
        root.clone(),
        Arc::clone(&g),
        jobs,
        browser_jobs,
        force,
        keep_going || std::env::var("PIPELINE_FAILURES").map(|v| v == "continue").unwrap_or(false),
        load_stamps(&root),
    ));
    let start = std::time::Instant::now();
    let counts = runner.run(&plan);
    if keep_going {
        // the re-pin drill reads this to classify each red gate
        if let Err(e) = runner.write_report() {
            eprintln!("pipeline: writing run report: {}", e);
        }
    }
    println!(
        "ran {}, skipped {} in {:.1}s (-j{}, browsers ≤{})",
        counts.ran,
        counts.skipped,
        start.elapsed().as_secs_f64(),
        jobs,
        browser_jobs
    );
    if jobs > 1 && counts.ran > 0 {
        println!("note: the undeclared-write check only runs at -j1 (PIPELINE_PARALLEL=1)");
    }
    if counts.violations > 0 {
        eprintln!(
            "\n{} undeclared write(s): a node is driving the graph's freshness {}",
            counts.violations,
            "through a file it does not admit to producing. Fix `produces`, or stop writing there."
        );
    }
    if counts.bad_reads > 0 {
        eprintln!(
            "\n{} undeclared file access(es): a node opened a file it declares in {}",
            counts.bad_reads,
            "neither `inputs` nor `produces`. If it reads the file, it is not in the node's key and a \
             change to it leaves the node falsely fresh — add it to `inputs`, and if the file is another \
             node's output add that node to `needs` too. If it writes the file, add it to `produces`."
        );
    }
    if counts.violations > 0 || counts.bad_reads > 0 {
        std::process::exit(1);
    }
    if counts.failed > 0 {
        std::process::exit(1);
    }
    Ok(())
}

fn adopt_command(args: &[String]) -> Result<(), String> {
    let (_force, gates_only, builds_only, _keep_going, targets) = parse_view_args(args);
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let g = load_graph_at(&root)?;
    let mut plan = resolve_targets(&g, &targets)?;
    if gates_only {
        plan.retain(|n| n.kind == "gate");
    }
    if builds_only {
        plan.retain(|n| n.kind == "build");
    }
    // Record every node's current key WITHOUT running anything. This asserts
    // that the tree already is what the pipeline would produce — only valid
    // straight after a full green run.
    let mut k = Keyer::new(&root, &g);
    let mut n0 = 0;
    for n in &plan {
        let Some(key) = k.key(&n.id)? else {
            continue;
        };
        write_stamp(&root, &n.id, &stamp_value(&root, n, &key)).map_err(|e| e.to_string())?;
        n0 += 1;
    }
    println!("adopted {} nodes as fresh (assumes the tree is a green full run)", n0);
    Ok(())
}

fn run_keys() -> Result<(), String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let g = load_graph_at(&root)?;
    let mut k = Keyer::new(&root, &g);
    for id in g.ids() {
        match k.key(id)? {
            Some(key) => println!(
                "{}\t1\t{}",
                id,
                stamp_value(&root, g.node(id).expect("ids are graph nodes"), &key)
            ),
            None => println!("{}\t0\t", id),
        }
    }
    Ok(())
}


fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage();
    }
    let cmd = args[0].clone();
    let rest = args[1..].to_vec();
    // < 2, not < 3: the single-word subcommands (docs-catalog, oracle-css,
    // product-css, build-js) take no argument.
    if rest.is_empty() && matches!(cmd.as_str(), "plan" | "list" | "status" | "run" | "adopt") {
        eprintln!("usage: pipeline {} <fast|full|builds|all|node…>", cmd);
        std::process::exit(2);
    }
    match cmd.as_str() {
        "plan" | "list" | "status" => {
            if let Err(e) = run_view(&cmd, &rest) {
                die(e);
            }
        }
        "inputs" => std::process::exit(run_inputs(&rest)),
        "__keys" => {
            // hidden: key-parity oracle for tests/gen_golden.sh — prints
            // "<id>\t<skippable>\t<stamp value>" per node in ids() order.
            if let Err(e) = run_keys() {
                die(e);
            }
        }
        "build-js" => std::process::exit(pipeline::jsbuild::run_build_js()),
        "build-rtl" => std::process::exit(pipeline::emit::build_rtl::run_build_rtl()),
        "product-css" => std::process::exit(pipeline::emit::product_css::run_product_css()),
        "tw" => std::process::exit(pipeline::emit::tw::run_tw(&rest)),
        "example-oracle" => std::process::exit(pipeline::oracle::example_oracle::run_example_oracle(
            has_flag(&rest, "--check"),
        )),
        "demo" => {
            if let Err(e) = pipeline::emit::demo::run_demo() {
                die(e);
            }
        }
        "emit" => {
            if let Err(e) = pipeline::emit::run_emit() {
                die(e);
            }
        }
        "convert" => {
            if let Err(e) = pipeline::convert::run_convert() {
                die(e);
            }
        }
        "run" => {
            if let Err(e) = run_command(&rest) {
                die(e);
            }
        }
        "adopt" => {
            if let Err(e) = adopt_command(&rest) {
                die(e);
            }
        }
        "pin" => std::process::exit(pipeline::gates::pin::run_pin(
            &std::env::current_dir().unwrap_or_default(),
            has_flag(&rest, "--check-only"),
            has_flag(&rest, "--force"),
        )),
        "coverage" => {
            if !has_flag(&rest, "--record") {
                eprintln!(
                    "the coverage GATE is a #[test]: cargo test coverage\nthis subcommand only re-records the ledger budget: pipeline coverage --record"
                );
                std::process::exit(2);
            }
            std::process::exit(
                match pipeline::gates::coverage::gate_coverage(
                    &std::env::current_dir().unwrap_or_default(),
                    &["--record".to_string()],
                ) {
                    Ok(()) => 0,
                    Err(e) => {
                        eprintln!("{}", e);
                        1
                    }
                },
            );
        }
        "ledger" => std::process::exit(pipeline::gates::ledger::run_ledger(&rest)),
        "audit-boundary" => {
            std::process::exit(pipeline::gates::audit_boundary::run_audit_boundary(&rest))
        }
        "oracle-css" => std::process::exit(pipeline::tools::oracle_css::run_oracle_css()),
        "docs-catalog" => std::process::exit(pipeline::tools::docs_catalog::run_docs_catalog(
            &std::env::current_dir().unwrap_or_default(),
        )),
        "docs-upstream-mirror" => std::process::exit(
            pipeline::tools::docs_upstream_mirror::run_docs_upstream_mirror(),
        ),
        "ir-diff" => std::process::exit(pipeline::tools::ir_diff::run_ir_diff(&rest)),
        "css-direction" => {
            if !has_flag(&rest, "--update") {
                eprintln!(
                    "the css-direction GATE is a #[test]: cargo test css_direction\nthis subcommand only re-records the baseline: pipeline css-direction --update"
                );
                std::process::exit(2);
            }
            std::process::exit(pipeline::tools::css_direction_update::run_css_direction_update(
                &std::env::current_dir().unwrap_or_default(),
            ));
        }
        "upstream" => std::process::exit(pipeline::tools::upstream::run_upstream(
            &std::env::current_dir().unwrap_or_default(),
            &rest,
        )),
        "resolve-skins" => std::process::exit(pipeline::tools::resolve_skins::run_resolve_skins(
            &std::env::current_dir().unwrap_or_default(),
            &rest,
        )),
        "rtl-dict" => std::process::exit(pipeline::tools::rtl_dict::run_rtl_dict()),
        "docs-consistency" => std::process::exit(
            pipeline::tools::docs_consistency::run_docs_consistency(
                &std::env::current_dir().unwrap_or_default(),
            ),
        ),
        "docs-build" => std::process::exit(pipeline::tools::docs_build::run_docs_build(
            &std::env::current_dir().unwrap_or_default(),
        )),
        "docs-fidelity" => std::process::exit(pipeline::tools::docs_fidelity::run_docs_fidelity(
            &std::env::current_dir().unwrap_or_default(),
        )),
        "example-fixture" => std::process::exit(pipeline::oracle::example_fixture::run_example_fixture(&rest)),
        "example-golden" => std::process::exit(pipeline::oracle::example_golden::run_example_golden(&rest)),
        "contract" => {
            if rest.is_empty() {
                die("contract: need a component name".to_string());
            }
            std::process::exit(pipeline::oracle::contract::run_contract(&rest[0]));
        }
        "contracts" => std::process::exit(pipeline::oracle::contract::run_contracts_all()),
        "upstream-snapshot" => std::process::exit(
            pipeline::tools::upstream_snapshot::run_upstream_snapshot(&rest),
        ),
        "demo-smoke" => std::process::exit(pipeline::tools::demo_smoke::run_demo_smoke(
            &std::env::current_dir().unwrap_or_default(),
        )),
        "docs-smoke" => std::process::exit(pipeline::tools::docs_smoke::run_docs_smoke(
            &std::env::current_dir().unwrap_or_default(),
            has_flag(&rest, "--all"),
        )),
        "overlay" => std::process::exit(pipeline::tools::overlay::run_overlay(
            &std::env::current_dir().unwrap_or_default(),
            &rest,
        )),
        "interactivity-sweep" => std::process::exit(
            pipeline::tools::interactivity_sweep::run_interactivity_sweep(
                &std::env::current_dir().unwrap_or_default(),
            ),
        ),
        "demo-parity" => std::process::exit(pipeline::tools::demo_parity::run_demo_parity(
            &std::env::current_dir().unwrap_or_default(),
            has_flag(&rest, "--record"),
            has_flag(&rest, "--details"),
        )),
        "style-parity" => std::process::exit(pipeline::tools::style_parity::run_style_parity(
            &std::env::current_dir().unwrap_or_default(),
            has_flag(&rest, "--strict"),
            has_flag(&rest, "--record"),
        )),
        "path-parity" => std::process::exit(pipeline::tools::path_parity::run_path_parity(
            &std::env::current_dir().unwrap_or_default(),
            has_flag(&rest, "--record"),
            has_flag(&rest, "--details"),
        )),
        // hidden: the self-hosted gate dispatcher — the graph's gate nodes
        // call this instead of `go test`. Not part of the public surface.
        "__gate" => std::process::exit(run_hidden_gate(&rest)),
        // hidden: the mutation harness (Go's opt-in TestMeta), driven from
        // this binary so the gates it runs resolve __self__ correctly. An
        // optional argument narrows the run to one gate's mutations.
        "__meta" => std::process::exit(run_hidden_meta(&rest)),
        // hidden: the Oxc A/B probe (PLAN.md「Oxc 替換」). Compiled only
        // under --features oxc; never part of the graph.
        #[cfg(feature = "oxc")]
        "__oxc-probe" => match pipeline::tools::oxc_probe::run_oxc_probe(
            &std::env::current_dir().unwrap_or_default(),
            &rest,
        ) {
            Ok(code) => std::process::exit(code),
            Err(e) => {
                eprintln!("__oxc-probe: {e}");
                std::process::exit(1);
            }
        },
        #[cfg(not(feature = "oxc"))]
        "__oxc-probe" => {
            eprintln!("__oxc-probe: binary built without --features oxc");
            std::process::exit(2);
        }
        other => {
            eprintln!("unknown command: {}", other);
            std::process::exit(2);
        }
    }
}

/// `__meta [gate]`: apply each selected mutation, run its gate through the
/// presented graph's own commands, restore — exit 0 iff every one is caught.
fn run_hidden_meta(rest: &[String]) -> i32 {
    let root = std::env::current_dir().unwrap_or_default();
    let g = match pipeline::graph::authored() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    let selected: Vec<&pipeline::gates::mutations::Mutation> =
        match rest.first() {
            Some(gate) => {
                let hits: Vec<_> = pipeline::gates::mutations::MUTATIONS
                    .iter()
                    .filter(|m| m.gate == gate.as_str())
                    .collect();
                if hits.is_empty() {
                    eprintln!("pipeline __meta: no mutations for gate: {}", gate);
                    return 2;
                }
                hits
            }
            None => pipeline::gates::mutations::MUTATIONS.iter().collect(),
        };
    println!("meta: {} mutations", selected.len());
    let mut failures: Vec<String> = Vec::new();
    for m in &selected {
        match pipeline::gates::mutations::run_mutation(&root, &g, m) {
            Err(e) => {
                // a tree left mutated is worse than any missed mutation
                eprintln!("{}: {}", m.id, e);
                return 1;
            }
            Ok((res, Some(restore_err))) => {
                eprintln!("{}: {}", res.id, restore_err);
                return 1;
            }
            Ok((res, None)) => {
                if res.caught {
                    println!("  {:<34} -> {:<20} CAUGHT", res.id, res.gate);
                } else {
                    let note = if res.note.is_empty() {
                        String::new()
                    } else {
                        format!(" ({})", res.note)
                    };
                    println!(
                        "  {:<34} -> {:<20} NOT CAUGHT{}",
                        res.id, res.gate, note
                    );
                    failures.push(format!(
                        "{}: {} stayed green under \"{}\"{}",
                        res.id, res.gate, m.why, note
                    ));
                }
            }
        }
    }
    if !failures.is_empty() {
        eprintln!(
            "FAIL  meta ({}/{} mutations not caught)\n  {}\n\n  A gate that cannot fail is not a gate. Fix the gate, not the mutation.",
            failures.len(),
            selected.len(),
            failures.join("\n  ")
        );
        return 1;
    }
    println!(
        "PASS  meta ({} mutations, every one caught by its gate)",
        selected.len()
    );
    0
}

/// `__gate <id>`: run the ported gate implementation in this engine, mapping
/// Go's per-gate `go test -run '^TestX..'` semantics to an exit code.
/// The ids the `__gate` match below dispatches, kept adjacent so a new arm
/// must update it (the ARMS pattern from the verb surface). Two-way-tested
/// against nodes::GATE_IDS — the set script-refs validates build-file
/// references against — and the usage line is rendered from it, so there is
/// exactly one place a gate id can be added or forgotten.
const DISPATCHED_GATES: &[&str] = &[
    "pin",
    "unit",
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

fn run_hidden_gate(rest: &[String]) -> i32 {
    let Some(gate) = rest.first() else {
        eprintln!(
            "usage: pipeline __gate <{}>",
            DISPATCHED_GATES.join("|")
        );
        return 2;
    };
    let root = std::env::current_dir().unwrap_or_default();
    let checked: Result<(), String> = match gate.as_str() {
        "pin" => {
            let code = pipeline::gates::pin::run_pin(&root, true, false);
            return code;
        }
        "pack" => pipeline::gates::pack::gate_pack(&root),
        "consumer-sim" => pipeline::gates::consumer_sim::gate_consumer_sim(&root),
        "coverage" => {
            pipeline::gates::coverage::gate_coverage(&root, &["--check".to_string()])
        }
        "ledger" => pipeline::gates::ledger::gate_ledger(&root),
        "script-refs" => pipeline::gates::gate_script_refs(&root).map(|_| ()),
        "dist-complete" => pipeline::gates::gate_dist_complete(&root).map(|_| ()),
        "reproducible" => pipeline::gates::gate_reproducible(&root).map(|_| ()),
        "product-verify" => pipeline::gates::gate_product_verify(&root),
        "css-direction" => pipeline::gates::gate_css_direction(&root).map(|_| ()),
        // Go's `-run '^TestUnit ./...'` runs the pipeline's own TestUnit*
        // functions; the port named their Rust twins with the same `unit_`
        // prefix (57 of them — Go's 188 includes internal/ subpackage tests
        // the port consolidated). The WHOLE lib suite would sweep in other
        // gates' real-tree tests — gate_parity round 2 caught exactly that:
        // a stale committed artifact reds the reproducible test, and unit
        // must not care, because reproducible is its own gate on both
        // engines. --lib keeps the Go-parity harness tests (tests/) out.
        "unit" => {
            // The runner injects the JS fs-recorder into node children; the
            // nested cargo test must not inherit that (or its own node
            // children would write into the gate's scratch js.log).
            let out = std::process::Command::new("cargo")
                .args(["test", "--release", "--lib", "--bins", "--", "unit_"])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .env_remove("NODE_OPTIONS")
                .env_remove("SHADLESS_FSLOG")
                .output();
            return match out {
                Err(e) => {
                    eprintln!("pipeline __gate unit: cargo test: {}", e);
                    1
                }
                Ok(o) if !o.status.success() => o.status.code().unwrap_or(1),
                Ok(o) => {
                    // libtest exits 0 for a zero-match filter: renaming the
                    // unit_ tests would silently empty this gate — and with
                    // it meta_wiring, the only enforcement that every gate
                    // has a mutation. A vacuous green is a failure.
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    if pipeline::gates::libtest_has_passing_tests(&stdout) {
                        0
                    } else {
                        eprintln!(
                            "FAIL  __gate unit: the unit_ filter matched no tests — the gate proved nothing"
                        );
                        1
                    }
                }
            };
        }
        other => {
            eprintln!("pipeline __gate: unknown gate: {}", other);
            return 2;
        }
    };
    match checked {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}



#[cfg(test)]
mod verb_surface {
    /// The public dispatch arms of run()'s match — kept adjacent so a new
    /// arm must update it. Two-way-checked against nodes::VERBS, the table
    /// the script-refs gate validates Makefile/package.json against: a verb
    /// reachable from the build files but not dispatchable (or vice versa)
    /// fails here.
    const ARMS: &[&str] = &[
        "plan", "list", "status", "inputs", "run", "adopt", "build-js",
        "build-rtl", "product-css", "tw", "example-oracle", "demo", "emit",
        "convert", "pin", "coverage", "ledger", "audit-boundary", "oracle-css",
        "docs-catalog", "docs-upstream-mirror", "ir-diff", "css-direction",
        "upstream", "resolve-skins", "rtl-dict", "docs-consistency",
        "docs-build", "docs-fidelity", "example-fixture", "example-golden",
        "contract", "contracts", "upstream-snapshot", "demo-smoke", "docs-smoke",
        "overlay", "interactivity-sweep", "demo-parity", "style-parity",
        "path-parity",
    ];

    // unit_-prefixed and --bins-covered: these are bin-target tests, and
    // the unit gate runs `cargo test --lib --bins -- unit_` — a plain name
    // here silently never executed anywhere (found 2026-09-18: both tests
    // below had never run in any gate).
    #[test]
    fn unit_dispatch_arms_match_public_verbs() {
        let mut a: Vec<&str> = ARMS.to_vec();
        a.sort();
        let mut v: Vec<&str> = pipeline::nodes::VERBS.to_vec();
        v.sort();
        assert_eq!(a, v, "main.rs dispatch arms and nodes::VERBS drifted apart");
    }

    #[test]
    fn unit_dispatched_gates_match_gate_ids() {
        let mut d: Vec<&str> = super::DISPATCHED_GATES.to_vec();
        d.sort();
        let mut g: Vec<&str> = pipeline::nodes::GATE_IDS.to_vec();
        g.sort();
        assert_eq!(d, g, "the __gate match and nodes::GATE_IDS drifted apart");
    }
}
