//! Replay of the recorded self-golden matrix through the Rust binary.
//!
//! tests/gen_golden.sh (self-verification mode — no Go anywhere) records
//! this engine's own stdout/stderr/exit code for every plan/list/status/
//! inputs case; this test re-runs each recorded case and compares, so
//! `cargo test` covers the CLI surface too. The goldens are snapshots of
//! the real tree: after changing the graph or the tree itself, re-run
//! tests/gen_golden.sh to refresh them. Keyer drift is covered by the
//! keys golden; the pre-port Go-parity evidence lives at the
//! `go-parity-final` tag (gate_parity.rs, the old opt-in Go cross-check,
//! was deleted when the Go engine was removed).
//!
//! Layout (flat, one slug per case — see gen_golden.sh):
//!   <slug>.cmd        the argv, one line, space-separated
//!   <slug>.rs.out / .rs.err / .rs.code   the recorded verdict
//!   keys.rs.txt       the __keys dump over the real tree

use std::path::PathBuf;
use std::process::Command;

fn shadless_root() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("SHADLESS_ROOT") {
        let p = PathBuf::from(p);
        if p.is_dir() {
            return Some(p);
        }
        return None;
    }
    pipeline::crate_adjacent_tree_root()
}

#[test]
fn golden_matrix_replays() {
    // The goldens record the go-mirror TABLE presentation (authored data in
    // nodes.rs; fingerprint-free, so they are stable across engine edits).
    // Single test fn and no other test in this binary reads the variable,
    // so there is no concurrent access to race with (edition-2024 unsafe).
    unsafe { std::env::set_var("SHADLESS_GRAPH", "go-mirror") };
    let Some(root) = shadless_root() else {
        eprintln!("skip: no shadless tree next to the crate (set SHADLESS_ROOT)");
        return;
    };
    // The recorded goldens were captured on a BUILT tree: several `inputs`
    // cases glob build/ artifacts (build/rtl-langs.json, build/resolved-ui).
    // On a fresh checkout those files are absent and the replay diverges for
    // an environmental reason, not a logic one — say so instead of failing
    // green-looking red.
    if !root.join("build/rtl-langs.json").exists() {
        eprintln!(
            "skip: build/rtl-langs.json missing under {} — the golden matrix was recorded on a built tree; run the build chain (or tests/gen_golden.sh) first",
            root.display()
        );
        return;
    }
    let golden = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden");
    let Ok(entries) = std::fs::read_dir(&golden) else {
        panic!("no golden matrix under tests/golden — run tests/gen_golden.sh first");
    };
    let mut slugs: Vec<String> = Vec::new();
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if let Some(s) = name.strip_suffix(".cmd") {
            slugs.push(s.to_string());
        }
    }
    slugs.sort();
    assert!(
        slugs.len() > 300,
        "expected the full golden matrix (300+ cases), found only {} — run tests/gen_golden.sh",
        slugs.len()
    );
    let mut fails: Vec<String> = Vec::new();
    for slug in &slugs {
        let cmd = std::fs::read_to_string(golden.join(format!("{}.cmd", slug)))
            .unwrap_or_default();
        let line = cmd.trim_end_matches('\n');
        let args: Vec<&str> = if line.is_empty() {
            Vec::new()
        } else {
            line.split(' ').collect()
        };
        let rs_code: i32 = std::fs::read_to_string(golden.join(format!("{}.rs.code", slug)))
            .expect("rs.code")
            .trim()
            .parse()
            .expect("rs.code is a number");
        let rs_out = std::fs::read(golden.join(format!("{}.rs.out", slug))).unwrap_or_default();
        let rs_err = std::fs::read(golden.join(format!("{}.rs.err", slug))).unwrap_or_default();
        let out = Command::new(env!("CARGO_BIN_EXE_pipeline"))
            .args(&args)
            .current_dir(&root)
            .output()
            .expect("rust binary runs");
        if out.stdout != rs_out || out.stderr != rs_err || out.status.code() != Some(rs_code) {
            fails.push(format!("pipeline {}", args.join(" ")));
        }
    }
    assert!(
        fails.is_empty(),
        "{} golden case(s) diverge from the recorded verdicts:\n{}",
        fails.len(),
        fails.join("\n")
    );

    // key folding: the recorded __keys dump must replay byte-for-byte (this
    // is the layer that catches key-folding drift now)
    let keys_golden = std::fs::read(golden.join("keys.rs.txt")).expect("keys.rs.txt golden");
    assert!(
        keys_golden.len() > 1000,
        "keys golden suspiciously small ({} bytes) — re-record with tests/gen_golden.sh",
        keys_golden.len()
    );
    let keys_now = Command::new(env!("CARGO_BIN_EXE_pipeline"))
        .arg("__keys")
        .current_dir(&root)
        .output()
        .expect("rust binary runs");
    assert!(
        keys_now.stdout == keys_golden && keys_now.status.success(),
        "__keys output diverges from the recorded keys golden — key folding drifted"
    );
}
