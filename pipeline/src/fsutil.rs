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
