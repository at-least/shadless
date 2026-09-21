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

/// Created, unique throwaway directory under the system temp dir — the
/// collision-safe recipe (pid + subsec nanos) the test fixtures share.
#[cfg(test)]
pub fn temp_root(purpose: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "shadless-{}-{}-{}",
        purpose,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos()
    ));
    std::fs::create_dir_all(&root).unwrap();
    root
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
