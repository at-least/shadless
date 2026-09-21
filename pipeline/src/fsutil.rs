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
