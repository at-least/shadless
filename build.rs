//! Engine fingerprints, one per implementation group.
//!
//! Self-hosted nodes carry `__self__@<fp>` as argv[0]; the fingerprint folds
//! into every node key via the ordinary run-argv fold. Granularity is the
//! implementation group (advisor-checked design, 2026-09-09):
//!
//! - HULL — every src root file (lib, main, the graph/runner/key layer) plus
//!   Cargo.toml/lock, build.rs and rust-toolchain.toml. This is the engine
//!   every node executes through (dispatch, key folding, stamp semantics),
//!   so a hull edit invalidates every fp: coarser is never falsely fresh.
//! - GROUPS — the tier directories (convert/emit/gates/oracle/tools/twmerge/
//!   tsx) and the lone jsbuild.rs, hashed per directory. A node's fp covers
//!   its entry group plus the groups its group depends on (the hand-written
//!   DAG in nodes.rs GROUP_DEPS, enforced by a raw-text grep test).
//!
//! build.rs deliberately does NO parsing (no import extraction): soundness
//! lives in the enforcement test, which over-approximates (comments and
//! string literals count as references). Per-file granularity inside tools/
//! would need real parsing and is future work.

use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// Same list as nodes.rs::ENGINE_GROUPS (order matters: the env string is
/// positional-free "name=hex" pairs, but keep both tables side-by-side
/// identical so a group added in one place fails the pairing test here).
const GROUPS: &[&str] = &["convert", "emit", "gates", "oracle", "tools", "twmerge", "tsx"];

/// src root files — everything NOT under a group directory and not jsbuild.rs.
/// nodes.rs::HULL_FILES must match; a root file missing here silently leaves
/// the graph (covered by the pairing test).
const HULL_FILES: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "build.rs",
    "rust-toolchain.toml",
    "src/lib.rs",
    "src/main.rs",
    "src/engine.rs",
    "src/fanout.rs",
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

fn hash_files(files: &mut Vec<PathBuf>) -> String {
    files.sort();
    let mut h = Sha256::new();
    for f in files.iter() {
        let bytes = std::fs::read(f)
            .unwrap_or_else(|e| panic!("engine fingerprint: {}: {}", f.display(), e));
        h.update(f.to_string_lossy().as_bytes());
        h.update([0u8]);
        h.update(&bytes);
        h.update([0u8]);
    }
    hex::encode(h.finalize())
}

fn walk_files(dir: &str) -> Vec<PathBuf> {
    // EVERY file, not just .rs: group dirs carry include_str!-ed assets
    // (oracle_canon.js, ef_*.js, twmerge's config/snapshot.json,
    // jsx_overrides.inc) that compile into this binary — editing one must
    // stale the nodes that execute it (reviewer finding, 2026-09-09).
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn main() {
    let mut out = format!("hull={}", hash_files(&mut HULL_FILES.iter().map(PathBuf::from).collect()));

    for g in GROUPS {
        out.push_str(&format!(
            ";{}={}",
            g,
            hash_files(&mut walk_files(&format!("src/{g}")))
        ));
    }
    out.push_str(&format!(
        ";jsbuild={}",
        hash_files(&mut vec![PathBuf::from("src/jsbuild.rs")])
    ));

    // walk_files("src") already includes jsbuild.rs and every group dir
    out.push_str(&format!(";global={}", hash_files(&mut walk_files("src"))));

    println!("cargo:rustc-env=ENGINE_FPS={out}");
}
