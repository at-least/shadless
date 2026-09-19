//! Error-path parity: exit codes and stderr are part of the byte contract
//! too. The happy paths are covered by the golden harness; these pin the
//! missing-skin shapes, which the port tolerated as a panic (exit 101) until
//! 2026-09-10 — Go prints `resolve-skins: skin: <PathError>` and exits 1.
//! The ENOENT/EISDIR/EACCES trio was byte-compared against the Go binary
//! when the deviation was closed; EACCES is not asserted here because a
//! root-run cargo test would read straight through chmod 000.

use std::process::Command;

use pipeline::crate_adjacent_tree_root;

fn run_emit_in(dir: &std::path::Path) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_pipeline"))
        .arg("emit")
        .current_dir(dir)
        .output()
        .expect("spawn pipeline");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

// The skin lookup falls back to the tree adjacent to the crate — the one
// compiled in. When THAT tree is pinned (`.upstream/` present, i.e. the
// normal dev tree after `npm run pin`), the missing-skin scenario cannot
// occur and the Go-shaped errors are only exercisable from an unpinned
// tree (a fresh CI checkout). Both tests skip there rather than fail.
fn adjacent_tree_is_pinned() -> bool {
    crate_adjacent_tree_root()
        .map(|r| {
            r.join(".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css")
                .exists()
        })
        .unwrap_or(false)
}

#[test]
fn missing_skin_is_go_shaped_stderr_exit_1() {
    if adjacent_tree_is_pinned() {
        eprintln!("skip: the adjacent tree is pinned (.upstream present) — no missing-skin scenario here");
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let (code, stdout, stderr) = run_emit_in(tmp.path());
    assert_eq!(code, 1);
    assert_eq!(stdout, "");
    assert_eq!(
        stderr,
        "resolve-skins: skin: open \
         .upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css: \
         no such file or directory\n"
    );
}

#[test]
fn skin_path_directory_reports_read_eisdir() {
    if adjacent_tree_is_pinned() {
        eprintln!("skip: the adjacent tree is pinned (.upstream present) — no missing-skin scenario here");
        return;
    }
    // Go os.ReadFile opens the directory fine on Linux and fails in read:
    // the PathError op is "read", not "open".
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(
        tmp.path()
            .join(".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css"),
    )
    .unwrap();
    let (code, stdout, stderr) = run_emit_in(tmp.path());
    assert_eq!(code, 1);
    assert_eq!(stdout, "");
    assert_eq!(
        stderr,
        "resolve-skins: skin: read \
         .upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css: is a directory\n"
    );
}
