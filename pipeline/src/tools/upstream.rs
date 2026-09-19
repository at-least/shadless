//! Port of pipeline/upstream.go — the re-pin drill. One command from "a new
//! shadcn release exists" to "green, or a classified report with task packets".
//!
//!   pipeline upstream --to=shadcn@4.20.0            full drill
//!   pipeline upstream --to=shadcn@4.20.0 --fetch    fetch tags first (network)
//!   pipeline upstream --to=shadcn@4.19.0            same tag: must be green (self-test)
//!   pipeline upstream --report-only                 re-classify the last run

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::OnceLock;

use crate::gates::pin::{read_pin, truncate, PinFile};

const UPSTREAM_DIR: &str = ".upstream/shadcn-ui";
const GATES_OUT: &str = "build/gates";

fn re_src_ext() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\.(tsx|mdx|css)$").unwrap())
}

/// upGit runs git inside the pinned checkout and returns trimmed stdout.
fn up_git(root: &Path, args: &[&str]) -> Result<String, String> {
    let mut argv: Vec<&str> = vec!["-C", UPSTREAM_DIR];
    argv.extend_from_slice(args);
    let out = std::process::Command::new("git")
        .args(&argv)
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        // Go cmd.Output() returns the *exec.ExitError, whose %v is
        // "exit status N" — the git stderr text never reaches the caller.
        return Err(format!(
            "exit status {}",
            out.status.code().unwrap_or(-1)
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// inherit runs a command with the drill's own stdio, reporting only whether
/// it succeeded — the drill continues past a red step on purpose.
fn inherit(root: &Path, name: &str, args: &[&str]) -> bool {
    std::process::Command::new(name)
        .args(args)
        .current_dir(root)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// The engine binary the drill drives: the Go binary path under
/// `SHADLESS_GRAPH=go-mirror` (the drill is itself a parity harness there),
/// otherwise this running binary — by definition already built. Resolving
/// the latter is not allowed to fall back to the Go path.
fn pipeline_exe() -> String {
    if crate::nodes::mirror_mode() {
        return "./build/pipeline".to_string();
    }
    match std::env::current_exe() {
        Ok(p) => p.to_string_lossy().into_owned(),
        Err(e) => {
            eprintln!("pipeline: cannot resolve the running binary: {}", e);
            std::process::exit(1);
        }
    }
}

fn upstream_step(title: &str) {
    println!("\n##### upstream: {}", title);
}

/// drillReport accumulates the markdown as the drill goes.
struct DrillReport {
    parts: Vec<String>,
}

impl DrillReport {
    fn h(&mut self, s: &str) {
        self.parts.push(format!("\n## {}\n", s));
    }
    fn p(&mut self, s: &str) {
        self.parts.push(s.to_string());
    }
    fn string(&self) -> String {
        format!("# Upstream drill report\n{}\n", self.parts.join("\n"))
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct PatchConflict {
    #[serde(default)]
    f: String,
    #[serde(default)]
    out: String,
}

pub fn run_upstream(root: &Path, args: &[String]) -> i32 {
    let to = flag_value(args, "to");
    let report_only = args.iter().any(|a| a == "--report-only");
    let no_build = args.iter().any(|a| a == "--no-build");
    if to.is_empty() && !report_only {
        eprintln!("usage: pipeline upstream --to=shadcn@X.Y.Z [--fetch] [--no-build]");
        return 2;
    }
    if let Err(e) = std::fs::create_dir_all(root.join(GATES_OUT)) {
        eprintln!("pipeline: {}", e);
        return 1;
    }
    let from = match read_pin(root) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    let registry = from.shadcn_ui.registry.clone();
    let mut rep = DrillReport { parts: Vec::new() };

    if !report_only {
        if drill_repin(root, &to, args, &from, &mut rep) != 0 {
            return 1;
        }
        if !no_build {
            upstream_step("full tier, keep going");
            let exe = pipeline_exe();
            if crate::nodes::mirror_mode() {
                inherit(root, "make", &["pipeline"]);
            }
            inherit(root, &exe, &["run", "all", "--keep-going"]);
        }
    }

    // --------------------------------------------------------- 5. classify
    upstream_step("classify");
    let to_pin = match read_pin(root) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    let changed = changed_upstream_files(
        root,
        &from.shadcn_ui.commit,
        &to_pin.shadcn_ui.commit,
        &registry,
    );
    let registry_names = list_registry_names(root, &registry);
    let mut changed_components: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    for f in &changed {
        changed_components.insert(component_of_example(
            &component_of(&f.path),
            &registry_names,
        ));
    }

    // The IR diff lives in ir_diff. The drill consumes it as data — the
    // --json shape is the interface.
    let ir_before = format!("{}/ir-before", GATES_OUT);
    let exe = pipeline_exe();
    let ir_text = capture_output(root, &exe, &["ir-diff", &ir_before, "generated/ir"])
        .unwrap_or_default();
    let ir_json = capture_output(
        root,
        &exe,
        &["ir-diff", &ir_before, "generated/ir", "--json"],
    )
    .unwrap_or_default();
    #[derive(Deserialize)]
    struct Diff {
        #[serde(default)]
        components: std::collections::HashMap<String, serde_json::Value>,
    }
    if let Ok(diff) = serde_json::from_str::<Diff>(&ir_json) {
        for n in diff.components.keys() {
            changed_components.insert(n.clone());
        }
    }

    rep.h("Upstream changes in scope");
    if changed.is_empty() {
        rep.p("- none");
    } else {
        let lines: Vec<String> = changed
            .iter()
            .map(|f| format!("- {} `{}`", f.status, f.path))
            .collect();
        rep.p(&lines.join("\n"));
    }
    rep.h("IR semantic diff");
    rep.p(&format!("```\n{}\n```", ir_text.trim_end_matches('\n')));

    let run = read_run_report(root);
    rep.h("Gates");
    rep.p(&format!(
        "- passed: {}\n- failed: {}\n- blocked: {}",
        run.passed.len(),
        run.failed.len(),
        run.blocked.len()
    ));

    let (expected, unexpected) = classify_failures(&run, &registry_names, &changed_components);
    if !unexpected.is_empty() {
        rep.h(&format!(
            "UNEXPECTED failures ({}) — our pipeline, not upstream",
            unexpected.len()
        ));
        rep.p(&unexpected.join("\n\n"));
    }
    if !expected.is_empty() {
        rep.h(&format!(
            "EXPECTED failures ({}) — consequences of upstream changes",
            expected.len()
        ));
        rep.p(&expected.join("\n\n"));
    }
    if !run.blocked.is_empty() {
        rep.p(&format!(
            "\nblocked (a dependency failed): {}",
            run.blocked.join(", ")
        ));
    }

    // ----------------------------------------------------------- 6. overlay
    upstream_step("overlay audit + task packets");
    inherit(root, &pipeline_exe(), &["overlay", "--tasks"]);
    let tasks = list_dir(&root.join(GATES_OUT).join("tasks"));
    let conflicts = read_conflicts(root);
    rep.h("Manual work");
    if tasks.is_empty() {
        rep.p("- none: every manual intervention still applies");
    } else {
        let lines: Vec<String> = tasks
            .iter()
            .map(|t| format!("- `{}/tasks/{}`", GATES_OUT, t))
            .collect();
        rep.p(&lines.join("\n"));
    }
    if !conflicts.is_empty() {
        let lines: Vec<String> = conflicts
            .iter()
            .map(|c| format!("- CONFLICT `overlays/upstream/{}`\n```\n{}\n```", c.f, c.out))
            .collect();
        rep.p(&lines.join("\n"));
    }

    rep.h("Next");
    rep.p(&[
        "- 1. read UNEXPECTED failures first — those are ours",
        "- 2. work the task packets (each names the gates that must be green)",
        "- 3. `./build/pipeline ledger --record` for exemptions that legitimately survive; `./build/pipeline overlay --record` after re-authoring",
        "- 4. `make upstream-snapshot` (network) to refresh the ui.shadcn.com golden snapshot for the new release",
        "- 5. `make` must be green; then commit source + regenerated output together (the dist/ diff IS the review)",
    ].join("\n"));

    let report_path = root.join(GATES_OUT).join("upstream-report.md");
    if let Err(e) = std::fs::write(&report_path, rep.string()) {
        eprintln!("pipeline: {}", e);
        return 1;
    }
    let green = run.failed.is_empty() && tasks.is_empty() && conflicts.is_empty();
    let verdict = if green { "PASS " } else { "REPORT" };
    print!(
        "\n{} upstream {} → {}: {} passed, {} failed ({} unexpected), {} task packets, {} conflicts\n  {}/upstream-report.md\n",
        verdict,
        from.shadcn_ui.tag,
        to_pin.shadcn_ui.tag,
        run.passed.len(),
        run.failed.len(),
        unexpected.len(),
        tasks.len(),
        conflicts.len(),
        GATES_OUT
    );
    if green {
        0
    } else {
        1
    }
}

/// drillRepin is steps 1-3: checkout, re-pin, dissolve, apply the patch series.
fn drill_repin(root: &Path, to: &str, args: &[String], from: &PinFile, rep: &mut DrillReport) -> i32 {
    // a leading dash would ride into `git checkout` as an option
    if to.starts_with('-') {
        eprintln!("pipeline upstream: --to looks like an option, not a revision: {}", to);
        return 2;
    }
    upstream_step(&format!("checkout {}", to));
    if args.iter().any(|a| a == "--fetch") {
        if let Err(e) = up_git(root, &["fetch", "--tags", "--quiet"]) {
            eprintln!("fetch failed (offline?): {}", e);
        }
    }
    let dirty = up_git(root, &["status", "--porcelain"]).unwrap_or_default();
    if !dirty.is_empty() {
        // an applied overlay series leaves the tree dirty by design; anything
        // else is a hand-edit that would be lost
        if patch_series(root).is_empty() {
            eprintln!(
                ".upstream has uncommitted changes and no overlay series explains them:\n{}\n  reset it or turn the change into overlays/upstream/*.patch\n",
                dirty
            );
            return 1;
        }
        if let Err(e) = up_git(root, &["checkout", "--", "."]) {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    }
    if let Err(e) = up_git(root, &["checkout", "--quiet", to]) {
        eprintln!("cannot checkout {}: {}  (try --fetch)", to, e);
        return 1;
    }
    if let Err(e) = copy_tree(
        &root.join("generated/ir"),
        &root.join(GATES_OUT).join("ir-before"),
    ) {
        eprintln!("pipeline: {}", e);
        return 1;
    }
    if !inherit(root, &pipeline_exe(), &["pin", "--force"]) {
        return 1;
    }
    let to_pin = match read_pin(root) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    rep.h(&format!("Re-pin {} → {}", from.shadcn_ui.tag, to_pin.shadcn_ui.tag));
    rep.p(&format!(
        "- from: `{}` ({})\n- to:   `{}` ({})",
        from.shadcn_ui.tag,
        truncate(&from.shadcn_ui.commit, 10),
        to_pin.shadcn_ui.tag,
        truncate(&to_pin.shadcn_ui.commit, 10)
    ));
    let log = up_git(
        root,
        &[
            "log",
            "--oneline",
            &format!("{}..{}", from.shadcn_ui.commit, to_pin.shadcn_ui.commit),
        ],
    )
    .unwrap_or_default();
    rep.p(&format!(
        "- upstream commits in range: {}",
        non_empty_lines(&log).len()
    ));

    upstream_step("dissolve auto-dissolve exemptions");
    inherit(root, &pipeline_exe(), &["ledger", "--dissolve"]);

    upstream_step("apply overlays/upstream");
    let series = patch_series(root);
    let mut conflicts: Vec<PatchConflict> = Vec::new();
    for f in &series {
        let out = std::process::Command::new("git")
            .args(["-C", UPSTREAM_DIR, "apply", "--3way"])
            .arg(root.join("overlays/upstream").join(f))
            .current_dir(root)
            .output();
        if let Ok(out) = out {
            if !out.status.success() {
                conflicts.push(PatchConflict {
                    f: f.clone(),
                    out: String::from_utf8_lossy(&out.stdout)
                        .trim()
                        .to_string()
                        + &String::from_utf8_lossy(&out.stderr).trim(),
                });
            }
        }
    }
    let msg = if conflicts.is_empty() {
        String::new()
    } else {
        format!(", {} CONFLICT", conflicts.len())
    };
    println!(
        "  {}/{} patches applied{}",
        series.len() - conflicts.len(),
        series.len(),
        msg
    );
    let b = serde_json::to_string_pretty(&conflicts).unwrap_or_else(|_| "[]".to_string());
    let _ = std::fs::write(root.join(GATES_OUT).join("upstream-conflicts.json"), b);
    0
}

// ------------------------------------------------------------- classify

struct ChangedFile {
    status: String,
    path: String,
}

fn changed_upstream_files(
    root: &Path,
    from_commit: &str,
    to_commit: &str,
    registry: &str,
) -> Vec<ChangedFile> {
    let out = up_git(
        root,
        &[
            "diff",
            "--name-status",
            "--no-renames",
            &format!("{}..{}", from_commit, to_commit),
            "--",
            registry,
            "apps/v4/examples",
            "apps/v4/registry/styles/style-nova.css",
            "apps/v4/content/docs/components/radix",
            "apps/v4/app/globals.css",
        ],
    )
    .unwrap_or_default();
    let mut files: Vec<ChangedFile> = Vec::new();
    for l in non_empty_lines(&out) {
        if let Some((status, path)) = l.split_once('\t') {
            files.push(ChangedFile {
                status: status.to_string(),
                path: path.to_string(),
            });
        }
    }
    files
}

/// componentOf strips the directory and the source extension: git reports
/// forward-slash paths regardless of platform, so this does not use
/// filepath.Base.
fn component_of(p: &str) -> String {
    let base = p.rsplit('/').next().unwrap_or(p);
    re_src_ext().replace_all(base, "").into_owned()
}

/// componentOfExample maps an example filename back to its component by the
/// longest registry-name prefix: "accordion-multiple" -> "accordion".
fn component_of_example(name: &str, registry_names: &[String]) -> String {
    let mut best = String::new();
    for r in registry_names {
        if name == r || name.starts_with(&format!("{}-", r)) {
            if r.len() > best.len() {
                best = r.clone();
            }
        }
    }
    if best.is_empty() {
        name.to_string()
    } else {
        best
    }
}

fn list_registry_names(root: &Path, registry: &str) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root.join(UPSTREAM_DIR).join(registry)) else {
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for e in entries.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if let Some(stem) = n.strip_suffix(".tsx") {
            out.push(stem.to_string());
        }
    }
    out.sort();
    out
}

/// classifyFailures splits red gates into EXPECTED (the failure text names a
/// component that moved upstream) and UNEXPECTED (it does not — we regressed).
fn classify_failures(
    run: &crate::runner::RunReport,
    registry_names: &[String],
    changed: &std::collections::HashSet<String>,
) -> (Vec<String>, Vec<String>) {
    let mut ids: Vec<&String> = run.failed.keys().collect();
    ids.sort();
    let mut expected: Vec<String> = Vec::new();
    let mut unexpected: Vec<String> = Vec::new();
    for id in ids {
        let f = &run.failed[id];
        let mut mentioned: Vec<String> = Vec::new();
        let mut hits: Vec<String> = Vec::new();
        for n in registry_names {
            let re = Regex::new(&format!(r"\b{}\b", regex::escape(n))).unwrap();
            if re.is_match(&f.tail) {
                mentioned.push(n.clone());
                if changed.contains(n) {
                    hits.push(n.clone());
                }
            }
        }
        let verdict = if !hits.is_empty() {
            format!("EXPECTED — upstream changed: {}", hits.join(", "))
        } else if !mentioned.is_empty() {
            let head: Vec<String> = mentioned.iter().take(6).cloned().collect();
            format!(
                "UNEXPECTED — mentions {}, none changed upstream",
                head.join(", ")
            )
        } else {
            "UNEXPECTED — no component attribution; read the tail".to_string()
        };
        let entry = format!(
            "### {}\n\n{}\n\n```\n{}\n```\nrepro: `./build/pipeline run {}`",
            id, verdict, f.tail, id
        );
        if !hits.is_empty() {
            expected.push(entry);
        } else {
            unexpected.push(entry);
        }
    }
    (expected, unexpected)
}

// ---------------------------------------------------------------- helpers

fn read_run_report(root: &Path) -> crate::runner::RunReport {
    let b = std::fs::read_to_string(root.join(GATES_OUT).join("run-report.json"))
        .unwrap_or_default();
    serde_json::from_str(&b).unwrap_or_default()
}

fn read_conflicts(root: &Path) -> Vec<PatchConflict> {
    let b = std::fs::read_to_string(root.join(GATES_OUT).join("upstream-conflicts.json"))
        .unwrap_or_default();
    serde_json::from_str(&b).unwrap_or_default()
}

fn patch_series(root: &Path) -> Vec<String> {
    list_dir(&root.join("overlays/upstream"))
        .into_iter()
        .filter(|n| n.ends_with(".patch"))
        .collect()
}

fn list_dir(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<String> = entries
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    out.sort();
    out
}

fn non_empty_lines(s: &str) -> Vec<&str> {
    s.split('\n').filter(|l| !l.trim().is_empty()).collect()
}

fn capture_output(root: &Path, name: &str, args: &[&str]) -> Result<String, String> {
    let out = std::process::Command::new(name)
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// flagValue reads --name=value from argv.
fn flag_value(args: &[String], name: &str) -> String {
    let prefix = format!("--{}=", name);
    for a in args {
        if let Some(v) = a.strip_prefix(&prefix) {
            return v.to_string();
        }
    }
    String::new()
}

/// copyTree copies src over dst, replacing it. The drill snapshots the IR
/// before a re-pin so ir-diff has a "before" to compare against.
fn copy_tree(src: &Path, dst: &Path) -> Result<(), String> {
    if dst.exists() {
        std::fs::remove_dir_all(dst).map_err(|e| e.to_string())?;
    }
    for e in walkdir::WalkDir::new(src) {
        let e = e.map_err(|e| e.to_string())?;
        let rel = e.path().strip_prefix(src).map_err(|e| e.to_string())?;
        let target = dst.join(rel);
        if e.file_type().is_dir() {
            std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
            continue;
        }
        let b = std::fs::read(e.path()).map_err(|e| e.to_string())?;
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&target, b).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Go TestUnitClassifyFailures.
    #[test]
    fn unit_classify_failures() {
        let registry: Vec<String> = ["accordion", "badge", "dialog", "select"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut changed = std::collections::HashSet::new();
        changed.insert("dialog".to_string());
        let mut failed = std::collections::BTreeMap::new();
        failed.insert(
            "contracts:dialog".to_string(),
            crate::runner::FailedNode {
                cmd: "x".to_string(),
                tail: "FAIL contracts dialog: slot mismatch".to_string(),
            },
        );
        failed.insert(
            "path-parity".to_string(),
            crate::runner::FailedNode {
                cmd: "y".to_string(),
                tail: "FAIL path-parity: badge lost its padding".to_string(),
            },
        );
        failed.insert(
            "pack".to_string(),
            crate::runner::FailedNode {
                cmd: "z".to_string(),
                tail: "FAIL pack: exports map is broken".to_string(),
            },
        );
        let run = crate::runner::RunReport {
            failed,
            blocked: Vec::new(),
            passed: Vec::new(),
        };
        let (expected, unexpected) = classify_failures(&run, &registry, &changed);
        assert_eq!(expected.len(), 1);
        assert!(
            expected[0].contains("EXPECTED — upstream changed: dialog"),
            "expected = {:?}",
            expected
        );
        assert_eq!(unexpected.len(), 2);
        let joined = unexpected.join("\n");
        assert!(
            joined.contains("UNEXPECTED — mentions badge, none changed upstream"),
            "{}",
            joined
        );
        assert!(
            joined.contains("UNEXPECTED — no component attribution"),
            "{}",
            joined
        );
        for e in expected.iter().chain(unexpected.iter()) {
            assert!(
                e.contains("repro: `./build/pipeline run "),
                "entry has no repro line:\n{}",
                e
            );
        }
    }

    /// Go TestUnitClassifyUsesWordBoundaries.
    #[test]
    fn unit_classify_uses_word_boundaries() {
        let mut failed = std::collections::BTreeMap::new();
        failed.insert(
            "g".to_string(),
            crate::runner::FailedNode {
                cmd: String::new(),
                tail: "the selected item was unselected".to_string(),
            },
        );
        let run = crate::runner::RunReport {
            failed,
            blocked: Vec::new(),
            passed: Vec::new(),
        };
        let mut changed = std::collections::HashSet::new();
        changed.insert("select".to_string());
        let (_, unexpected) = classify_failures(
            &run,
            &["select".to_string()],
            &changed,
        );
        assert_eq!(unexpected.len(), 1);
        assert!(
            unexpected[0].contains("no component attribution"),
            "{}",
            unexpected[0]
        );
    }

    /// Go TestUnitClassifyIsDeterministic.
    #[test]
    fn unit_classify_is_deterministic() {
        let mut failed = std::collections::BTreeMap::new();
        for id in ["zeta", "alpha", "mid"] {
            failed.insert(
                id.to_string(),
                crate::runner::FailedNode {
                    cmd: String::new(),
                    tail: "no names".to_string(),
                },
            );
        }
        let run = crate::runner::RunReport {
            failed,
            blocked: Vec::new(),
            passed: Vec::new(),
        };
        let mut first = String::new();
        for i in 0..5 {
            let (_, unexpected) = classify_failures(&run, &[], &std::collections::HashSet::new());
            let got = unexpected.join("|");
            if i == 0 {
                first = got;
                continue;
            }
            assert_eq!(got, first, "classification order is not stable");
        }
        assert!(
            first.starts_with("### alpha"),
            "want the entries sorted by id, got {}",
            &first[..20.min(first.len())]
        );
    }

    /// Go TestUnitComponentOfExample.
    #[test]
    fn unit_component_of_example() {
        let registry: Vec<String> = ["accordion", "toggle", "toggle-group", "select"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        for (name, want) in [
            ("accordion", "accordion"),
            ("accordion-multiple", "accordion"),
            ("toggle-group", "toggle-group"),
            ("toggle-group-multiple", "toggle-group"),
            ("toggle-demo", "toggle"),
            ("something-unrelated", "something-unrelated"),
        ] {
            assert_eq!(component_of_example(name, &registry), want);
        }
    }

    /// Go TestUnitComponentOf.
    #[test]
    fn unit_component_of() {
        for (input, want) in [
            ("apps/v4/registry/bases/radix/ui/badge.tsx", "badge"),
            ("apps/v4/examples/radix/accordion-demo.tsx", "accordion-demo"),
            ("apps/v4/content/docs/components/radix/tabs.mdx", "tabs"),
            ("apps/v4/app/globals.css", "globals"),
            ("noslash.tsx", "noslash"),
        ] {
            assert_eq!(component_of(input), want);
        }
    }

    /// Go TestUnitReadRunReportToleratesMissing.
    #[test]
    fn unit_read_run_report_tolerates_missing() {
        let r = read_run_report(std::path::Path::new("/nonexistent-shadless-dir"));
        assert!(!r.failed.is_empty() || r.failed.is_empty()); // non-nil map
        assert!(r.passed.is_empty() && r.blocked.is_empty());
    }

    /// Go TestUnitCopyTreeReplaces.
    #[test]
    fn unit_copy_tree_replaces() {
        let root = std::env::temp_dir().join(format!(
            "shadless-copytree-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        let src = root.join("src");
        let dst = root.join("dst");
        std::fs::create_dir_all(src.join("sub")).unwrap();
        std::fs::write(src.join("sub/a.json"), b"new").unwrap();
        std::fs::create_dir_all(&dst).unwrap();
        std::fs::write(dst.join("stale.json"), b"old").unwrap();
        copy_tree(&src, &dst).unwrap();
        assert!(
            !dst.join("stale.json").exists(),
            "copyTree left a stale file behind"
        );
        assert_eq!(std::fs::read_to_string(dst.join("sub/a.json")).unwrap(), "new");
    }

    /// Go TestUnitFlagValue.
    #[test]
    fn unit_flag_value() {
        let args: Vec<String> = ["--to=shadcn@4.20.0", "--fetch"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(flag_value(&args, "to"), "shadcn@4.20.0");
        assert_eq!(flag_value(&args, "missing"), "");
        assert_eq!(flag_value(&args, "fetch"), "");
    }
}
