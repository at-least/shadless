//! oracle-css byte-identity check: the RS binary's output must equal the Go
//! binary's, modulo the tailwind CLI's own `Done in Xms` timing line (proven
//! non-deterministic Go-vs-Go: 205/219/206ms across three runs).

use std::path::PathBuf;
use std::process::Command;

fn shadless_root() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("SHADLESS_ROOT") {
        let p = PathBuf::from(p);
        if p.is_dir() {
            return Some(p);
        }
        return None;
    }
    pipeline::crate_adjacent_tree_root()
}

fn strip_timing(s: &str) -> String {
    s.lines()
        .map(|l| {
            if l.starts_with("Done in ") && l.ends_with("ms") {
                "Done in Xms".to_string()
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn oracle_css_matches_go_modulo_timing() {
    let Some(root) = shadless_root() else {
        eprintln!("skip: no shadless tree next to the crate (set SHADLESS_ROOT)");
        return;
    };
    let go_bin = root.join("pipeline/pipeline");
    if !go_bin.exists() {
        eprintln!("skip: no Go pipeline binary under {}", root.display());
        return;
    }
    if !root.join(".upstream/shadcn-ui/apps/v4/app/globals.css").exists() {
        eprintln!("skip: no pinned upstream checkout");
        return;
    }
    let css_path = root.join("build/gates/oracle.css");
    let entry_path = root.join("build/gates/oracle.entry.css");
    let css_before = std::fs::read(&css_path).ok();
    let entry_before = std::fs::read(&entry_path).ok();

    let go = Command::new(&go_bin)
        .arg("oracle-css")
        .current_dir(&root)
        .output()
        .expect("go binary runs");
    let go_css = std::fs::read(&css_path).expect("oracle.css after go run");
    let go_entry = std::fs::read(&entry_path).expect("oracle.entry.css after go run");

    let rs = Command::new(env!("CARGO_BIN_EXE_pipeline"))
        .arg("oracle-css")
        .current_dir(&root)
        .output()
        .expect("rust binary runs");
    let rs_css = std::fs::read(&css_path).expect("oracle.css after rs run");
    let rs_entry = std::fs::read(&entry_path).expect("oracle.entry.css after rs run");

    // restore whatever the tree had before (build/gates is gitignored, but
    // the test must not leave a different state than it found)
    match css_before {
        Some(b) => std::fs::write(&css_path, b).ok(),
        None => std::fs::remove_file(&css_path).ok(),
    };
    match entry_before {
        Some(b) => std::fs::write(&entry_path, b).ok(),
        None => std::fs::remove_file(&entry_path).ok(),
    };

    assert_eq!(go.status.code(), rs.status.code(), "exit codes differ");
    assert_eq!(
        strip_timing(&String::from_utf8_lossy(&go.stdout)),
        strip_timing(&String::from_utf8_lossy(&rs.stdout)),
        "stdout differs beyond the tailwind timing line"
    );
    assert_eq!(
        strip_timing(&String::from_utf8_lossy(&go.stderr)),
        strip_timing(&String::from_utf8_lossy(&rs.stderr)),
        "stderr differs beyond the tailwind timing line"
    );
    assert_eq!(go_css, rs_css, "oracle.css differs");
    assert_eq!(go_entry, rs_entry, "oracle.entry.css differs");
}
