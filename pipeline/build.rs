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
//! - GROUPS — the tier directories (convert/emit/gates/oracle/twmerge/tsx)
//!   and the lone jsbuild.rs, hashed per directory. A node's fp covers its
//!   entry group plus the groups its group depends on (the hand-written DAG
//!   in nodes.rs GROUP_DEPS, enforced by a raw-text grep test).
//! - tools/ — hashed per FILE (`tools:<stem>`; plus `tools:mod` for the
//!   module root, folded into every tools node). The 20 tools are
//!   independent commands: editing docs_smoke.rs must not stale demo-parity.
//!   Per-file deps live in nodes.rs TOOLS_FILE_DEPS/TOOLS_FILE_INTRA,
//!   enforced by the same raw-text grep test.
//!
//! build.rs deliberately does NO parsing (no import extraction): soundness
//! lives in the enforcement test, which over-approximates (comments and
//! string literals count as references).

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
    // tools/ hashes per FILE (plus its module root): the 20 tools are
    // independent commands, so editing docs_smoke.rs must not stale
    // demo-parity. The dir-level `tools` hash stays emitted but unused —
    // nodes.rs composes tools nodes from the per-file entries. Stems must be
    // unique: a subdir with a colliding stem would silently alias an
    // existing file's hash (walk is recursive, coverage test is not).
    let mut tool_stems: Vec<String> = Vec::new();
    for f in walk_files("src/tools") {
        let stem = f.file_stem().unwrap().to_string_lossy().into_owned();
        assert!(
            !tool_stems.contains(&stem),
            "duplicate tools file stem {stem:?} — per-file fingerprints key on \
             stems and would alias two files into one hash"
        );
        tool_stems.push(stem.clone());
        out.push_str(&format!(
            ";tools:{}={}",
            stem,
            hash_files(&mut vec![f])
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
