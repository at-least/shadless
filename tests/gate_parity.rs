//! Gate verdict parity — the self-hosting acceptance harness.
//!
//! For every gate in the authored table, both engines must AGREE:
//!
//!   clean tree   same verdict (green+green expected; both-red is a
//!                tree-state warning — the gates themselves are the tree's
//!                authority, this harness is the engines')
//!   mutated      both engines' mutation harnesses reach the same verdict;
//!                both must CATCH (a not-caught on both is printed loudly
//!                as a warning — parity holds, the mutation is suspect)
//!
//! This is what replaces "the gates are byte-identical" once the gates run
//! Rust: verdicts must match, including the red side — an engine gate that
//! disagrees with its Go twin (or stays green under a break the Go gate
//! catches) fails here. Real instance: example-golden printed Go's FAIL line
//! but exited 0, so the runner saw green; parity caught it.
//!
//! Heavy by design (real browsers on the full tier): opt in with
//! SHADLESS_GATE_PARITY=1, narrow with GATE_ONLY=id1,id2. Requires the
//! shadless tree (SHADLESS_ROOT or ../shadless) and its Go toolchain. The
//! tree is pre-built first via the engine's own runner (`run all
//! --builds-only`), so build-node outputs (docs demos, rtl, product css)
//! exist for the gates that read them.

use std::path::{Path, PathBuf};
use std::process::Command;

fn shadless_root() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("SHADLESS_ROOT") {
        let p = PathBuf::from(p);
        return if p.is_dir() { Some(p) } else { None };
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../shadless")
        .canonicalize()
        .ok()
}

/// Child env for gate processes: this harness's own knobs must not leak.
/// Real instance: SHADLESS_ROOT short-circuits Go's findRepoRoot, making
/// TestUnitFindRepoRootMarker fail in a temp tree.
fn child_env(mut cmd: Command) -> Command {
    cmd.env_remove("SHADLESS_ROOT")
        .env_remove("SHADLESS_GRAPH")
        .env_remove("SHADLESS_GATE_PARITY")
        .env_remove("GATE_ONLY");
    cmd
}

/// Run one node's command list in order; true iff every command exits 0.
fn run_cmds(root: &Path, cmds: &[Vec<String>], argv0: &Path) -> bool {
    for argv in cmds {
        let exe = if argv[0].starts_with("__self__") {
            argv0.to_path_buf()
        } else {
            PathBuf::from(&argv[0])
        };
        let status = child_env(Command::new(exe))
            .args(&argv[1..])
            .current_dir(root)
            .status();
        match status {
            Ok(s) if s.success() => continue,
            _ => return false,
        }
    }
    true
}

#[test]
fn gate_verdicts_parity() {
    if std::env::var_os("SHADLESS_GATE_PARITY").is_none() {
        eprintln!("skip: set SHADLESS_GATE_PARITY=1 (heavy: runs every gate on both engines, browsers included)");
        return;
    }
    let Some(root) = shadless_root() else {
        panic!("SHADLESS_GATE_PARITY=1 but no shadless tree (set SHADLESS_ROOT)");
    };
    let go_bin = root.join("build/pipeline");
    if !go_bin.exists() {
        panic!("Go binary missing at {} — build it first", go_bin.display());
    }
    let rs_bin = PathBuf::from(env!("CARGO_BIN_EXE_pipeline"));

    // Pre-build through the engine's own runner: every build node, no gates.
    // The gates below read build-node outputs (docs demos, rtl, product css);
    // a tree that was never built would red them on BOTH engines and say
    // nothing about parity.
    let pre = child_env(Command::new(&rs_bin))
        .args(["run", "all", "--builds-only"])
        .current_dir(&root)
        .status()
        .expect("pre-build run all --builds-only");
    assert!(pre.success(), "pre-build (run all --builds-only) failed");

    let only: Vec<String> = std::env::var("GATE_ONLY")
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
        .unwrap_or_default();
    // Gates expected BOTH-red for known tree-state reasons must be declared,
    // or the both-red WARN is a failure: a shared blind spot must be an
    // explicit decision, not an accident of two engines reading one tree.
    // (Real instance: reproducible — upstream's committed dist/out.css is
    // stale; see PROGRESS.md.)
    let expect_red: Vec<String> = std::env::var("PARITY_EXPECT_RED")
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
        .unwrap_or_default();
    if !only.is_empty() {
        for want in &only {
            assert!(
                pipeline::nodes::all_go()
                    .iter()
                    .any(|n| n.kind == "gate" && &n.id == want),
                "GATE_ONLY={}: no such gate",
                want
            );
        }
    }

    // The authored table gives both shapes: all_go() is Go's gate commands
    // verbatim; all() (default env) is this engine's.
    let go_gates: Vec<_> = pipeline::nodes::all_go()
        .into_iter()
        .filter(|n| n.kind == "gate")
        .collect();
    assert_eq!(go_gates.len(), 24);
    let rs_gates: Vec<_> = pipeline::nodes::all()
        .into_iter()
        .filter(|n| n.kind == "gate")
        .collect();

    let mut failures: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    for go_n in &go_gates {
        if !only.is_empty() && !only.iter().any(|w| w == &go_n.id) {
            continue;
        }
        let rs_n = rs_gates
            .iter()
            .find(|n| n.id == go_n.id)
            .unwrap_or_else(|| panic!("gate {} missing from the self-hosted table", go_n.id));

        // clean tree: verdicts must agree
        let go_green = run_cmds(&root, &go_n.run, &go_bin);
        let rs_green = run_cmds(&root, &rs_n.run, &rs_bin);
        println!(
            "  {:<22} clean: go={} rs={}",
            go_n.id,
            if go_green { "green" } else { "RED" },
            if rs_green { "green" } else { "RED" }
        );
        if go_green != rs_green {
            failures.push(format!(
                "{}: clean-tree verdicts disagree (go={} rs={})",
                go_n.id,
                if go_green { "green" } else { "RED" },
                if rs_green { "green" } else { "RED" }
            ));
        } else if !go_green {
            if expect_red.iter().any(|w| w == &go_n.id) {
                warnings.push(format!(
                    "{}: both engines red on the clean tree (declared: PARITY_EXPECT_RED)",
                    go_n.id
                ));
            } else {
                failures.push(format!(
                    "{}: both engines red on the clean tree — declare it via PARITY_EXPECT_RED if this is a known tree-state red, or investigate",
                    go_n.id
                ));
            }
        }

        // mutated: each engine's own mutation harness; verdicts must agree,
        // and agreement on CATCH is the red-proof.
        let mut ids: Vec<&str> = pipeline::gates::mutations::MUTATIONS
            .iter()
            .filter(|m| m.gate == go_n.id.as_str())
            .map(|m| m.id)
            .collect();
        if ids.is_empty() {
            failures.push(format!("{}: no mutations prove this gate", go_n.id));
            continue;
        }
        ids.sort_unstable();
        let go_caught = child_env(Command::new("go"))
            .args(["test", "-C", "pipeline", "-count=1", "-run", "^TestMeta$", "."])
            .current_dir(&root)
            .env("SHADLESS_META", "1")
            .env("META_ONLY", ids.join(","))
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        let rs_caught = child_env(Command::new(&rs_bin))
            .args(["__meta", &go_n.id])
            .current_dir(&root)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        println!(
            "  {:<22} meta:   go={} rs={}",
            go_n.id,
            if go_caught { "caught" } else { "NOT caught" },
            if rs_caught { "caught" } else { "NOT caught" }
        );
        if go_caught != rs_caught {
            failures.push(format!(
                "{}: mutation verdicts disagree (go={} rs={})",
                go_n.id,
                if go_caught { "caught" } else { "not caught" },
                if rs_caught { "caught" } else { "not caught" }
            ));
        } else if !go_caught {
            warnings.push(format!(
                "{}: neither engine's gate went red under its mutation — parity holds, the mutation is suspect",
                go_n.id
            ));
        }
    }

    for w in &warnings {
        eprintln!("WARN  {}", w);
    }
    assert!(
        failures.is_empty(),
        "gate verdict parity failed:\n  {}",
        failures.join("\n  ")
    );
}
