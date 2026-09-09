//! Engine fingerprint: one sha256 over the engine's own sources.
//!
//! Self-hosted nodes carry `__self__@<fp>` as argv[0], so the fingerprint
//! lands inside every node key via the ordinary run-argv fold — editing any
//! engine source invalidates the whole graph (coarse but never falsely
//! fresh; per-module granularity is future work, see PLAN.md).

use sha2::{Digest, Sha256};
use std::path::PathBuf;

fn main() {
    let mut files: Vec<PathBuf> = walkdir::WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |x| x == "rs"))
        .map(|e| e.path().to_path_buf())
        .collect();
    files.sort();
    for name in ["Cargo.toml", "Cargo.lock", "build.rs", "rust-toolchain.toml"] {
        files.push(PathBuf::from(name));
    }

    let mut h = Sha256::new();
    for f in &files {
        let bytes = std::fs::read(f)
            .unwrap_or_else(|e| panic!("engine fingerprint: {}: {}", f.display(), e));
        h.update(f.to_string_lossy().as_bytes());
        h.update([0u8]);
        h.update(&bytes);
        h.update([0u8]);
    }
    println!("cargo:rustc-env=ENGINE_FP={}", hex::encode(h.finalize()));
    for name in ["Cargo.toml", "Cargo.lock", "build.rs", "rust-toolchain.toml"] {
        println!("cargo:rerun-if-changed={}", name);
    }
    println!("cargo:rerun-if-changed=src");
}
