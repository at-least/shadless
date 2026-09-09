//! Self-hosting plumbing: how a node command refers to "this engine".
//!
//! Nodes authored in the Go-verbatim table execute the Go binary
//! (`./build/pipeline`) or `go test`; the self-hosted shape (nodes::self_host)
//! rewrites those argv[0]s to `__self__@<engine fingerprint>`. The runner,
//! the meta harness and the upstream drill resolve that token here.

use std::path::PathBuf;

/// Resolve a command's argv[0]. `__self__` (optionally `__self__@<fp>` — the
/// fingerprint is key material, not a path component) means "whatever binary
/// is running right now"; everything else passes through untouched.
pub fn resolve_argv0(argv0: &str) -> std::io::Result<PathBuf> {
    let base = argv0.split('@').next().unwrap_or(argv0);
    if base == "__self__" {
        std::env::current_exe()
    } else {
        Ok(PathBuf::from(argv0))
    }
}
