//! Shared filesystem helpers. The invariant that matters anywhere a walk
//! reaches committed bytes or a cache key: readdir order is
//! filesystem-dependent (btrfs hashes, ext4/tmpfs insert), so a directory
//! walk must be sorted before it is consumed.

/// Directory entry file names, sorted. IO errors surface as strings, like
/// the rest of the port.
pub fn sorted_read_dir(dir: &std::path::Path) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        out.push(e.file_name().to_string_lossy().into_owned());
    }
    out.sort();
    Ok(out)
}

/// Created, unique throwaway directory under the system temp dir, the one
/// the test fixtures share: it derefs to its path and is deleted when it
/// drops, so hold it for as long as the test uses the path.
#[cfg(test)]
pub fn temp_root(purpose: &str) -> TempRoot {
    TempRoot(
        tempfile::Builder::new()
            .prefix(&format!("shadless-{}-", purpose))
            .tempdir()
            .unwrap(),
    )
}

#[cfg(test)]
pub struct TempRoot(tempfile::TempDir);

#[cfg(test)]
impl AsRef<std::path::Path> for TempRoot {
    fn as_ref(&self) -> &std::path::Path {
        self.0.path()
    }
}

#[cfg(test)]
impl std::ops::Deref for TempRoot {
    type Target = std::path::Path;
    fn deref(&self) -> &std::path::Path {
        self.0.path()
    }
}

/// The emitter exemption sets — src/registry/emitter-exemptions.json, the
/// one source the JS emitters, this port and the overlay audit read.
/// Resolved from the working directory first (engine verbs run at the repo
/// root), then the crate-adjacent tree (in-process cargo tests run at the
/// crate).
pub fn registry_exemptions() -> &'static (Vec<String>, Vec<String>) {
    static E: std::sync::OnceLock<(Vec<String>, Vec<String>)> = std::sync::OnceLock::new();
    E.get_or_init(|| {
        #[derive(serde::Deserialize)]
        struct Ex {
            #[serde(rename = "deadUtilities")]
            dead_utilities: Vec<String>,
            #[serde(rename = "skinAllowlist")]
            skin_allowlist: Vec<String>,
        }
        let mut candidates = vec![std::path::PathBuf::from(
            "src/registry/emitter-exemptions.json",
        )];
        if let Some(tree) = crate::crate_adjacent_tree_root() {
            candidates.push(tree.join("src/registry/emitter-exemptions.json"));
        }
        for c in &candidates {
            if let Ok(b) = std::fs::read_to_string(c) {
                let ex: Ex = serde_json::from_str(&b)
                    .unwrap_or_else(|e| panic!("{}: {}", c.display(), e));
                return (ex.dead_utilities, ex.skin_allowlist);
            }
        }
        panic!(
            "emitter-exemptions.json not found under any of: {:?}",
            candidates
        );
    })
}

/// Go's lowercase strerror for an errno — Go's own table
/// (syscall/zerrors_linux_amd64.go), not libc's. One table for every tool
/// that renders io::Errors Go-shaped: the filesystem entries serve all of
/// them, ETIMEDOUT/ECONNREFUSED serve the network path.
pub fn go_strerror(code: i32) -> Option<&'static str> {
    Some(match code {
        1 => "operation not permitted",
        2 => "no such file or directory",
        5 => "input/output error",
        6 => "no such device or address",
        12 => "cannot allocate memory",
        13 => "permission denied",
        17 => "file exists",
        20 => "not a directory",
        21 => "is a directory",
        22 => "invalid argument",
        24 => "too many open files",
        26 => "text file busy",
        28 => "no space left on device",
        30 => "read-only file system",
        36 => "file name too long",
        39 => "directory not empty",
        40 => "too many levels of symbolic links",
        75 => "value too large for defined data type",
        110 => "connection timed out",
        111 => "connection refused",
        _ => return None,
    })
}

/// Renders an io::Error the way Go's *PathError does:
/// `open <path>: <strerror>`. Go never drops the `op <path>:` prefix, so
/// errnos outside the table fall back to the prefixed Rust display.
pub fn go_path_err(op: &str, path: impl AsRef<std::path::Path>, e: &std::io::Error) -> String {
    match e.raw_os_error().and_then(go_strerror) {
        Some(msg) => format!("{} {}: {}", op, path.as_ref().display(), msg),
        None => format!("{} {}: {}", op, path.as_ref().display(), e),
    }
}

#[cfg(test)]
mod tests {
    /// Every test fixture root used to survive its test: a cargo test run
    /// left ~20 shadless-* dirs in the temp dir (2006 shadless-mut-test
    /// ones when measured).
    #[test]
    fn unit_temp_root_is_deleted_when_it_drops() {
        let root = super::temp_root("temp-root-test");
        let path = root.to_path_buf();
        assert!(path.is_dir(), "the root exists while it is held");
        drop(root);
        assert!(!path.exists(), "{} survived its test", path.display());
    }

    /// A fixture dir named off the temp dir and removed by a trailing
    /// remove_dir_all leaks whenever an assert before it fails — and under
    /// nextest every test is its own process, so each failure leaves its
    /// pid's dir behind for good (shadless-rs-fanout-* from 2026-10-18 were
    /// still in /tmp). temp_root's guard deletes on unwind too.
    /// Any std::env::temp_dir call counts, not only `.join` on it: a
    /// dir held in a variable and joined later, or the shared temp dir
    /// itself used as a test's cwd, passed the first version of this guard.
    #[test]
    fn unit_test_fixtures_take_their_dir_from_temp_root() {
        let needle = concat!("temp_dir", "()");
        let mut hits: Vec<String> = Vec::new();
        let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut dirs = vec![crate_dir.join("src"), crate_dir.join("tests")];
        while let Some(dir) = dirs.pop() {
            for name in super::sorted_read_dir(&dir).unwrap() {
                let p = dir.join(&name);
                if p.is_dir() {
                    dirs.push(p);
                } else if name.ends_with(".rs") {
                    let text = std::fs::read_to_string(&p).unwrap();
                    for (i, line) in text.lines().enumerate() {
                        if line.contains(needle) {
                            hits.push(format!("{}:{}", p.display(), i + 1));
                        }
                    }
                }
            }
        }
        assert!(
            hits.is_empty(),
            "fixture dirs outside fsutil::temp_root (removed only if the test passes):\n  {}",
            hits.join("\n  ")
        );
    }
}
