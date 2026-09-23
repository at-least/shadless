//! shadless-rs — Rust port of shadless/pipeline (the graph runner + converters).
//! The port's acceptance bar was byte-identity with the Go binary
//! (go-parity-final tag); the live bar is the self-recorded golden harness
//! (tests/gen_golden.sh). See PORT.md.

pub mod convert;
pub mod engine;
pub mod fanout;
pub mod fsutil;
pub mod glob;
pub mod emit;
pub mod gates;
pub mod graph;
pub mod jsbuild;
pub mod jsonorder;
pub mod key;
pub mod nodes;
pub mod oracle;
pub mod produces;
pub mod runner;
pub mod stamps;
pub mod tools;
pub mod tsx;
pub mod twmerge;
pub mod verify;

/// Repo-root candidates adjacent to the crate, nearest-first: the crate's
/// parent (correct once the engine lives at pipeline/ inside the product
/// repo) and the sibling checkout (correct before that move). A candidate is
/// a product tree when it holds package.json with a pipeline/ directory —
/// the same marker the verb-side find_repo_root walks for. Tests resolve the
/// real tree through here so the unit gate works with or without
/// SHADLESS_ROOT, before and after the move.
pub fn crate_adjacent_tree_root() -> Option<std::path::PathBuf> {
    for rel in ["..", "../shadless"] {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
        if p.join("package.json").exists() && p.join("pipeline").is_dir() {
            return Some(p.canonicalize().unwrap_or(p));
        }
    }
    None
}

/// SHADLESS_ROOT, else the crate-adjacent product tree (crate parent as the
/// last resort) — the one true-tree resolution the tree-backed tests share.
pub fn tree_root() -> std::path::PathBuf {
    match std::env::var("SHADLESS_ROOT") {
        Ok(r) => std::path::PathBuf::from(r),
        Err(_) => crate::crate_adjacent_tree_root()
            .unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..")),
    }
}
