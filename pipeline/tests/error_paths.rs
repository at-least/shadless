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

fn run_tool_in(dir: &std::path::Path, verb: &str) -> (i32, String, String) {
    run_tool_args_in(dir, verb, &[])
}

fn run_tool_args_in(dir: &std::path::Path, verb: &str, args: &[&str]) -> (i32, String, String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pipeline"));
    cmd.arg(verb)
        .args(args)
        .current_dir(dir);
    let out = cmd.output().expect("spawn pipeline");
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

// A corrupt tiers.json must fail the tool that reads it, by naming tiers —
// not by coincidence downstream. Both tools used to parse it with
// unwrap_or_default: the sweep ran with no static families excluded (a
// verdict over a matrix it did not mean to test) and emit reported
// "expected 0 static" nonsense instead of the parse error. 2026-09 review.
#[test]
fn corrupt_tiers_json_fails_the_sweep_naming_tiers() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("src/registry")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/site/static/demos")).unwrap();
    std::fs::write(tmp.path().join("src/registry/tiers.json"), "{ not json").unwrap();
    let (code, _stdout, stderr) = run_tool_in(tmp.path(), "interactivity-sweep");
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("interactivity-sweep: tiers:"),
        "must name the corrupt file, got: {}",
        stderr
    );
}

#[test]
fn corrupt_tiers_json_fails_emit_naming_tiers() {
    // emit loads the pinned skin before reading tiers: the fallback to the
    // crate-adjacent tree needs THAT tree pinned, or the run dies on the
    // missing skin before it ever reaches the corrupt file
    if !adjacent_tree_is_pinned() {
        eprintln!("skip: the adjacent tree is unpinned — emit dies on the missing skin before tiers");
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("src/registry")).unwrap();
    std::fs::create_dir_all(tmp.path().join("generated/ir")).unwrap();
    std::fs::write(tmp.path().join("src/registry/tiers.json"), "{ not json").unwrap();
    let (code, _stdout, stderr) = run_tool_in(tmp.path(), "emit");
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("emit: tiers:"),
        "must name the corrupt file, got: {}",
        stderr
    );
}

// example-oracle parsed tiers with unwrap_or_default too (both halves of
// the run): an empty tier map silently made is_kernel_demo always-false —
// the same vacuous-verdict shape as the sweep. Review follow-up.
#[test]
fn corrupt_tiers_json_fails_example_oracle_naming_tiers() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("src/registry")).unwrap();
    std::fs::write(tmp.path().join("src/registry/tiers.json"), "{ not json").unwrap();
    let (code, _stdout, stderr) = run_tool_in(tmp.path(), "example-oracle");
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("example-oracle: tiers:"),
        "must name the corrupt file, got: {}",
        stderr
    );
}

// docs-smoke's iframe check must not pass vacuously when the built pages
// carry no preview iframes at all (template drift would re-open the hole
// the mutation pins). Review follow-up.
#[test]
fn docs_smoke_without_iframes_fails_not_vacuously_passes() {
    let tmp = tempfile::tempdir().unwrap();
    let public = tmp.path().join("docs/site/public");
    std::fs::create_dir_all(&public).unwrap();
    std::fs::write(public.join("index.html"), "<html><body><article>x</article></body></html>").unwrap();
    let (code, _stdout, stderr) = run_tool_in(tmp.path(), "docs-smoke");
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("no preview iframes"),
        "must name the missing iframes, got: {}",
        stderr
    );
}

// example-oracle decides what its stale-page sweep may DELETE from
// docs/catalog.json and overlays/manifest.json, and what --check verifies
// from docs/example-oracle.json. All three used to parse with silent
// empty fallbacks: a corrupt catalog made the sweep's legit set empty (it
// would delete every non-overlay demo page and exit 0), a corrupt owned
// manifest made the check vacuously green. All three are now fatal at
// preflight, before the browser (and the sweep) runs. 2026-09 review r2.
#[test]
fn corrupt_catalog_fails_example_oracle_before_any_work() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("src/registry")).unwrap();
    std::fs::write(tmp.path().join("src/registry/tiers.json"), "{}").unwrap();
    std::fs::create_dir_all(tmp.path().join("docs")).unwrap();
    std::fs::write(tmp.path().join("docs/catalog.json"), "{ not json").unwrap();
    let (code, _stdout, stderr) = run_tool_in(tmp.path(), "example-oracle");
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("example-oracle: catalog:"),
        "must name the corrupt file, got: {}",
        stderr
    );
}

#[test]
fn corrupt_overlays_manifest_fails_example_oracle() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("src/registry")).unwrap();
    std::fs::write(tmp.path().join("src/registry/tiers.json"), "{}").unwrap();
    std::fs::create_dir_all(tmp.path().join("docs")).unwrap();
    std::fs::write(tmp.path().join("docs/catalog.json"), "{\"previews\": []}").unwrap();
    std::fs::create_dir_all(tmp.path().join("overlays")).unwrap();
    std::fs::write(tmp.path().join("overlays/manifest.json"), "{ not json").unwrap();
    let (code, _stdout, stderr) = run_tool_in(tmp.path(), "example-oracle");
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("example-oracle: overlays manifest:"),
        "must name the corrupt file, got: {}",
        stderr
    );
}

#[test]
fn corrupt_owned_manifest_fails_example_oracle_check() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("src/registry")).unwrap();
    std::fs::write(tmp.path().join("src/registry/tiers.json"), "{}").unwrap();
    std::fs::create_dir_all(tmp.path().join("overlays")).unwrap();
    std::fs::write(tmp.path().join("overlays/manifest.json"), "{\"units\": {}}").unwrap();
    std::fs::create_dir_all(tmp.path().join("docs")).unwrap();
    std::fs::write(tmp.path().join("docs/catalog.json"), "{\"previews\": []}").unwrap();
    std::fs::write(tmp.path().join("docs/example-oracle.json"), "{ not json").unwrap();
    let (code, _stdout, stderr) = run_tool_args_in(tmp.path(), "example-oracle", &["--check"]);
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("example-oracle: owned:"),
        "must name the corrupt file, got: {}",
        stderr
    );
}

// An IR name becomes an output path (dist/components/{name}.html) and a CSS
// comment header: a name with path separators writes outside the output
// tree. Round-2 review: no loader validated the charset.
#[test]
fn traversal_ir_name_fails_emit() {
    if !adjacent_tree_is_pinned() {
        eprintln!("skip: the adjacent tree is unpinned — emit dies on the missing skin before the IR");
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    // a real IR file, only the name mutated
    let ir_src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../generated/ir/button.json"),
    )
    .unwrap();
    let mut ir: serde_json::Value = serde_json::from_str(&ir_src).unwrap();
    ir["name"] = serde_json::json!("../../evil");
    std::fs::create_dir_all(tmp.path().join("generated/ir")).unwrap();
    std::fs::create_dir_all(tmp.path().join("src/registry")).unwrap();
    std::fs::write(
        tmp.path().join("generated/ir/evil.json"),
        serde_json::to_string(&ir).unwrap(),
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("src/registry/tiers.json"),
        "{\"button\": {\"tier\": \"static\"}}",
    )
    .unwrap();
    let (code, _stdout, stderr) = run_tool_in(tmp.path(), "emit");
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("kebab component name"),
        "must refuse the traversal name, got: {}",
        stderr
    );
    assert!(
        !tmp.path().join("evil.html").exists(),
        "the traversal write must not happen"
    );
}

// a manifest that parses but lacks a units object would silently empty the
// sweep's legit set — the mass-deletion shape preflight exists to stop.
#[test]
fn unitsless_overlays_manifest_fails_example_oracle() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("src/registry")).unwrap();
    std::fs::write(tmp.path().join("src/registry/tiers.json"), "{}").unwrap();
    std::fs::create_dir_all(tmp.path().join("docs")).unwrap();
    std::fs::write(tmp.path().join("docs/catalog.json"), "{\"previews\": []}").unwrap();
    std::fs::create_dir_all(tmp.path().join("overlays")).unwrap();
    std::fs::write(tmp.path().join("overlays/manifest.json"), "{}").unwrap();
    let (code, _stdout, stderr) = run_tool_in(tmp.path(), "example-oracle");
    assert_eq!(code, 1, "stderr: {}", stderr);
    assert!(
        stderr.contains("no units object"),
        "must name the missing units, got: {}",
        stderr
    );
}
