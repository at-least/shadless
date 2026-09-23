//! Port of pipeline/docs_upstream_mirror.go; the contract was proven
//! byte-identical during the port (go-parity-final tag).
//!
//! Copies the slice of the pinned upstream docs tree that docs-build/
//! docs-fidelity actually read (components/radix, and the upstream-sourced
//! guides.source entries) from the gitignored .upstream/ into the tracked
//! generated/docs-upstream/, so a re-pin changes it visibly instead of
//! silently.

use std::fs;
use std::path::{Path, PathBuf};

const UPSTREAM_DOCS_DIR: &str = ".upstream/shadcn-ui/apps/v4/content/docs";
const DOCS_UPSTREAM_MIRROR: &str = "generated/docs-upstream";

/// One entry per upstream-sourced guides[].source (docs_guides.go) — kept as
/// a literal list, not derived from `guides`, so a newly added upstream-
/// sourced guide fails loud here (file not found in the mirror) rather than
/// silently reading straight from .upstream/ again.
const DOCS_UPSTREAM_FILES: [&str; 3] = [
    "rtl/index.mdx",
    "utils/shimmer.mdx",
    "utils/scroll-fade.mdx",
];

/// copyTree copies src over dst, replacing it (pipeline/upstream.go:491-516).
/// Go's error strings are `open <path>: <strerror>` / `mkdir <path>: ...` /
/// `remove <path>: ...`; Rust's io::Error Display is not, so the op and path
/// are wrapped here with Go's exact shapes.
fn copy_tree(src: &Path, dst: &Path) -> Result<(), String> {
    if dst.exists() {
        fs::remove_dir_all(dst).map_err(|e| crate::fsutil::go_path_err("remove", dst, &e))?;
    }
    let mut stack: Vec<PathBuf> = vec![src.to_path_buf()];
    while let Some(p) = stack.pop() {
        // WalkDir lstats every entry (it never follows symlinks), so a
        // missing root reports `lstat <path>: ...` — not `open`.
        let meta =
            fs::symlink_metadata(&p).map_err(|e| crate::fsutil::go_path_err("lstat", &p, &e))?;
        let rel = p.strip_prefix(src).map_err(|e| e.to_string())?;
        let target = dst.join(rel);
        if meta.file_type().is_symlink() {
            return Err(format!(
                "copy_tree: {}: symlink refused — the pinned tree must not carry symlinks",
                p.display()
            ));
        }
        if meta.is_dir() {
            fs::create_dir_all(&target)
                .map_err(|e| crate::fsutil::go_path_err("mkdir", &target, &e))?;
            let mut entries: Vec<PathBuf> = fs::read_dir(&p)
                .map_err(|e| crate::fsutil::go_path_err("open", &p, &e))?
                .map(|e| {
                    e.map(|e| e.path())
                        .map_err(|e| crate::fsutil::go_path_err("open", &p, &e))
                })
                .collect::<Result<_, _>>()?;
            entries.sort();
            for e in entries.into_iter().rev() {
                stack.push(e);
            }
        } else {
            let b = fs::read(&p).map_err(|e| crate::fsutil::go_path_err("open", &p, &e))?;
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| crate::fsutil::go_path_err("mkdir", parent, &e))?;
            }
            fs::write(&target, b).map_err(|e| crate::fsutil::go_path_err("open", &target, &e))?;
        }
    }
    Ok(())
}

pub fn run_docs_upstream_mirror() -> i32 {
    let radix_src = Path::new(UPSTREAM_DOCS_DIR).join("components/radix");
    let radix_dst = Path::new(DOCS_UPSTREAM_MIRROR).join("components/radix");
    if let Err(e) = copy_tree(&radix_src, &radix_dst) {
        eprintln!("docs-upstream-mirror: components/radix: {}", e);
        return 1;
    }
    let mut n = 0;
    for rel in DOCS_UPSTREAM_FILES {
        let src = Path::new(UPSTREAM_DOCS_DIR).join(rel);
        let b = match fs::read(&src) {
            Ok(b) => b,
            Err(e) => {
                eprintln!(
                    "docs-upstream-mirror: {}",
                    crate::fsutil::go_path_err("open", &src, &e)
                );
                return 1;
            }
        };
        let dst = Path::new(DOCS_UPSTREAM_MIRROR).join(rel);
        if let Some(dir) = dst.parent() {
            if let Err(e) = fs::create_dir_all(dir) {
                eprintln!(
                    "docs-upstream-mirror: {}",
                    crate::fsutil::go_path_err("mkdir", dir, &e)
                );
                return 1;
            }
        }
        if let Err(e) = fs::write(&dst, &b) {
            eprintln!(
                "docs-upstream-mirror: {}",
                crate::fsutil::go_path_err("open", &dst, &e)
            );
            return 1;
        }
        n += 1;
    }
    let mut radix_count = 0;
    if let Ok(ents) = fs::read_dir(Path::new(DOCS_UPSTREAM_MIRROR).join("components/radix")) {
        radix_count = ents.count();
    }
    println!(
        "docs-upstream-mirror: {} components/radix files + {} guide files -> {}",
        radix_count, n, DOCS_UPSTREAM_MIRROR
    );
    0
}
