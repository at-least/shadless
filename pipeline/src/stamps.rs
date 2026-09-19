//! Port of the stamp store in shadless/pipeline/main.go: one file per node
//! under pipeline/stamps/, holding the stamp value that produced the current
//! outputs. Node ids carry ":" after a fan-out (contracts:dialog); it is
//! legal in a POSIX filename but not on Windows, so it is escaped in the path.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const STAMP_DIR: &str = "pipeline/stamps";

pub fn stamp_file(id: &str) -> String {
    // percent-style escaping, '%' first so it is injective: the old
    // ':'→"__" map was lossy — an id containing "__" loaded back as a
    // different id and its stamp never matched. ':' stays out of the
    // filename for Windows.
    id.replace('%', "%25").replace(':', "%3A")
}

fn stamp_id(name: &str) -> String {
    name.replace("%3A", ":").replace("%25", "%")
}

pub fn load_stamps(root: &Path) -> HashMap<String, String> {
    let mut s = HashMap::new();
    let Ok(entries) = fs::read_dir(root.join(STAMP_DIR)) else {
        return s;
    };
    for e in entries.flatten() {
        if e.file_type().map(|t| t.is_dir()).unwrap_or(true) {
            continue;
        }
        let name = e.file_name().to_string_lossy().into_owned();
        if let Ok(b) = fs::read(e.path()) {
            s.insert(stamp_id(&name), String::from_utf8_lossy(&b).trim().to_string());
        }
    }
    s
}

pub fn write_stamp(root: &Path, id: &str, key: &str) -> std::io::Result<()> {
    fs::create_dir_all(root.join(STAMP_DIR))?;
    fs::write(root.join(STAMP_DIR).join(stamp_file(id)), format!("{}\n", key))
}

pub fn remove_stamp(root: &Path, id: &str) {
    let _ = fs::remove_file(root.join(STAMP_DIR).join(stamp_file(id)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamp_filename_escapes_colons() {
        assert_eq!(stamp_file("contracts:dialog"), "contracts%3Adialog");
        assert_eq!(stamp_id("contracts%3Adialog"), "contracts:dialog");
        assert_eq!(stamp_file("pin"), "pin");
    }

    /// the escape must be injective: the old ':'→"__" map silently confused
    /// a future id containing "__" with a fan-out id (stamp_id turned
    /// "a__b" into "a:b", so the stamp never matched and the node rebuilt
    /// forever), and '%' must be escaped too for the inverse to be exact
    #[test]
    fn stamp_filename_is_injective_and_round_trips() {
        for id in ["pin", "contracts:dialog", "under_score", "a__b", "100%:weird"] {
            assert_eq!(stamp_id(&stamp_file(id)), id, "round trip: {}", id);
        }
        assert_ne!(stamp_file("a:b"), stamp_file("a__b"), "the old map collided here");
    }

    #[test]
    fn load_missing_dir_is_empty_and_roundtrip_works() {
        let tmp = std::env::temp_dir().join(format!("shadless-rs-stamps-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        assert!(load_stamps(&tmp).is_empty());
        write_stamp(&tmp, "pin", "abc").unwrap();
        write_stamp(&tmp, "contracts:dialog", "def").unwrap();
        let stamps = load_stamps(&tmp);
        assert_eq!(stamps.get("pin").map(String::as_str), Some("abc"));
        assert_eq!(stamps.get("contracts:dialog").map(String::as_str), Some("def"));
        remove_stamp(&tmp, "pin");
        assert!(!load_stamps(&tmp).contains_key("pin"));
        let _ = fs::remove_dir_all(&tmp);
    }
}
