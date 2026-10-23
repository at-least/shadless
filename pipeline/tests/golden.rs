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
//! The replay compares what the tracked files decide (`git ls-files`: their
//! working-tree content, so an uncommitted edit to one still reddens it, as
//! it should). A case or key row that folds an untracked file —
//! docs/site/public from a local zola build, build/, tools/contracts/out,
//! node_modules, the nested .upstream checkout — is local state: `inputs`
//! lists are compared without untracked paths, and key rows whose node (or
//! a node it needs) declares an untracked input, or produces an untracked
//! file, are not compared. A zola rebuild reddened this test with no
//! committed change (0319f1f re-recorded it). Dependency-key folding is
//! covered by key.rs's tests: none of the rows compared today has needs.
//!
//! `status` cases read the untracked stamp store too, but the stamps a
//! normal (self-hosted) run writes never carry a go-mirror key — every
//! node's key differs between the two tables (measured 2026-10-21: 0 of
//! 70 equal) — so these cases replay as recorded.
//!
//! Layout (flat, one slug per case — see gen_golden.sh):
//!   <slug>.cmd        the argv, one line, space-separated
//!   <slug>.rs.out / .rs.err / .rs.code   the recorded verdict
//!   keys.rs.txt       the __keys dump over the real tree

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

/// The tracked files (what a fresh clone of the tree holds).
fn tracked_paths(root: &Path) -> HashSet<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z"])
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git ls-files in {}: {}",
        root.display(),
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
        .split(|b| *b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| String::from_utf8_lossy(p).into_owned())
        .collect()
}

/// Nodes whose `__keys` row folds an untracked file — through its own
/// declared inputs or those of a node it needs (dependency keys fold up), or
/// through its produces (the row's outputs digest) — each with the file.
fn local_state_rows(root: &Path, tracked: &HashSet<String>) -> BTreeMap<String, String> {
    let g = pipeline::graph::load_graph_at(root).expect("graph loads");
    let untracked = |patterns: Option<&[String]>| {
        pipeline::glob::files(root, patterns.unwrap_or(&[]))
            .expect("patterns expand")
            .into_iter()
            .find(|f| !tracked.contains(f))
    };
    let key_hit: BTreeMap<&String, String> = g
        .ids()
        .iter()
        .filter_map(|id| Some((id, untracked(g.node(id).unwrap().inputs.as_deref())?)))
        .collect();
    g.ids()
        .iter()
        .filter_map(|id| {
            let why = untracked(g.node(id).unwrap().produces.as_deref()).or_else(|| {
                g.plan(std::slice::from_ref(id))
                    .unwrap()
                    .iter()
                    .find_map(|n| key_hit.get(&n.id).cloned())
            })?;
            Some((id.clone(), why))
        })
        .collect()
}

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
        // a CI misconfiguration must not turn the whole golden matrix into a
        // vacuous pass — the lib gates fail loudly on CI for the same reason
        if std::env::var_os("CI").is_some() {
            panic!("CI: required tree input missing (skip: no shadless tree)");
        }
        eprintln!("skip: no shadless tree next to the crate (set SHADLESS_ROOT)");
        return;
    };
    // The recorded goldens were captured on a BUILT tree: several `inputs`
    // cases glob build/ artifacts (build/rtl-langs.json, build/resolved-ui).
    // On a fresh checkout those files are absent and the replay diverges for
    // an environmental reason, not a logic one — say so instead of failing
    // green-looking red.
    // build/emit is produced, not committed: like build/rtl-langs.json, a
    // fresh checkout without it would red the produces-goldens on runtime
    // state the tree legitimately does not have yet.
    if !root.join("build/rtl-langs.json").exists() || !root.join("build/emit").exists() {
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
    let tracked = tracked_paths(&root);
    // an `inputs` listing without the files a fresh clone does not have
    let committed = |out: &[u8]| -> Vec<u8> {
        let text = String::from_utf8_lossy(out);
        text.lines()
            .filter(|l| tracked.contains(*l))
            .flat_map(|l| format!("{l}\n").into_bytes())
            .collect()
    };
    let mut fails: Vec<String> = Vec::new();
    for slug in &slugs {
        let cmd = std::fs::read_to_string(golden.join(format!("{}.cmd", slug))).unwrap_or_default();
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
        let (out_now, out_rec) = if args.first() == Some(&"inputs") {
            (committed(&out.stdout), committed(&rs_out))
        } else {
            (out.stdout, rs_out)
        };
        if out_now != out_rec || out.stderr != rs_err || out.status.code() != Some(rs_code) {
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
    assert!(keys_now.status.success(), "__keys failed");
    let local = local_state_rows(&root, &tracked);
    let committed_rows = |dump: &[u8]| -> Vec<String> {
        String::from_utf8_lossy(dump)
            .lines()
            .filter(|l| !local.contains_key(l.split('\t').next().unwrap()))
            .map(str::to_string)
            .collect()
    };
    let (rec, now) = (
        committed_rows(&keys_golden),
        committed_rows(&keys_now.stdout),
    );
    eprintln!(
        "keys: {} tracked-tree rows compared, {} local-state rows skipped",
        rec.len(),
        local.len()
    );
    // build-js, ledger, script-refs and typecheck qualify today; with none
    // left the layer would compare nothing
    assert!(
        !rec.is_empty(),
        "every key row folds an untracked file, so none is compared:\n  {}",
        local
            .iter()
            .map(|(id, f)| format!("{id}: {f}"))
            .collect::<Vec<_>>()
            .join("\n  ")
    );
    assert!(
        now == rec,
        "__keys output diverges from the recorded keys golden on the tracked-tree rows — key folding drifted"
    );
}
