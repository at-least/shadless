//! Replay of the recorded golden matrix through the Rust binary.
//!
//! tests/gen_golden.sh records the GO binary's stdout/stderr/exit code for
//! every plan/list/status/inputs case (and asserts the Rust port matches
//! live); this test re-runs each recorded case through the Rust binary so
//! `cargo test` covers the surface too. The goldens are snapshots of the
//! real tree: after changing the graph or the tree itself, re-run
//! tests/gen_golden.sh to refresh them.
//!
//! Layout (flat, one slug per case — see gen_golden.sh):
//!   <slug>.cmd        the argv, one line, space-separated
//!   <slug>.go.out / .go.err / .go.code   the recorded Go verdict

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
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../shadless")
        .canonicalize()
        .ok()
}

#[test]
fn golden_matrix_replays() {
    let Some(root) = shadless_root() else {
        eprintln!("skip: no shadless tree next to the crate (set SHADLESS_ROOT)");
        return;
    };
    // The recorded goldens were captured on a BUILT tree: several `inputs`
    // cases glob build/ artifacts (build/rtl-langs.json, build/resolved-ui).
    // On a fresh checkout those files are absent and the replay diverges for
    // an environmental reason, not a porting one — say so instead of failing
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
        let go_code: i32 = std::fs::read_to_string(golden.join(format!("{}.go.code", slug)))
            .expect("go.code")
            .trim()
            .parse()
            .expect("go.code is a number");
        let go_out = std::fs::read(golden.join(format!("{}.go.out", slug))).unwrap_or_default();
        let go_err = std::fs::read(golden.join(format!("{}.go.err", slug))).unwrap_or_default();
        let out = Command::new(env!("CARGO_BIN_EXE_pipeline"))
            .args(&args)
            .current_dir(&root)
            .output()
            .expect("rust binary runs");
        if out.stdout != go_out || out.stderr != go_err || out.status.code() != Some(go_code) {
            fails.push(format!("pipeline {}", args.join(" ")));
        }
    }
    assert!(
        fails.is_empty(),
        "{} golden case(s) diverge from the recorded Go verdicts:\n{}",
        fails.len(),
        fails.join("\n")
    );
}
