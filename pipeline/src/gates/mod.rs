//! Port of pipeline/gate_*.go — the Go gates as Rust functions, each wired to
//! a #[test] where the inputs are pure files (dist/, generated/, docs/).

pub mod audit_boundary;
pub mod consumer_sim;
pub mod coverage;
pub mod ledger;
pub mod mutations;
pub mod pack;
pub mod pin;

use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::OnceLock;

/// gate_coverage_budget.go — the ledger lives at a fixed path; one constant
/// means one file (the coverage budget is stored in it too).
pub const LEDGER_PATH: &str = "gates/ledger.json";

fn slot_selector_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"\[data-slot="[^"]+"\]"#).unwrap())
}

/// gate_dist_complete: the tracked no-build stylesheet must carry every
/// component's slot rules. Every [data-slot="…"] selector declared in
/// dist/css/<name>.css must appear in dist/out.css.
pub fn gate_dist_complete(root: &Path) -> Result<usize, String> {
    let out = std::fs::read_to_string(root.join("dist/out.css"))
        .map_err(|_| "FAIL  dist-complete: dist/out.css missing".to_string())?;
    let names: Vec<String> = crate::fsutil::sorted_read_dir(&root.join("dist/css"))
        .map_err(|e| format!("FAIL  dist-complete: dist/css unreadable: {}", e))?
        .into_iter()
        .filter(|n| n.ends_with(".css"))
        .collect();

    let mut missing: Vec<String> = Vec::new();
    let (mut selectors, mut files) = (0usize, 0usize);
    for name in &names {
        files += 1;
        // an unreadable stylesheet must fail the gate, not shrink the count:
        // a silently skipped file reports fewer slots than dist actually
        // carries and the completeness check passes over a hole
        let src = std::fs::read_to_string(root.join("dist/css").join(name))
            .map_err(|e| format!("FAIL  dist-complete: reading dist/css/{}: {}", name, e))?;
        let mut seen: HashSet<String> = HashSet::new();
        for sel in slot_selector_re().find_iter(&src) {
            let sel = sel.as_str();
            if !seen.insert(sel.to_string()) {
                continue;
            }
            selectors += 1;
            if !out.contains(sel) {
                missing.push(format!(
                    "{}: {}",
                    name.trim_end_matches(".css"),
                    sel
                ));
            }
        }
    }

    if !missing.is_empty() {
        let mut comps: Vec<String> = Vec::new();
        for m in &missing {
            let c = m.splitn(2, ':').next().unwrap_or("").to_string();
            if !comps.contains(&c) {
                comps.push(c);
            }
        }
        let mut head = comps.clone();
        head.truncate(6);
        let ell = if comps.len() > 6 { ", …" } else { "" };
        let mut shown = missing.clone();
        let more = if shown.len() > 8 {
            shown.truncate(8);
            "\n  …"
        } else {
            ""
        };
        return Err(format!(
            "FAIL  dist-complete: dist/out.css lacks {} slot selectors from {} components ({}{})\n  {}{}\n  out.css was built from a partial dist/components — run the full `npm run demo` and commit its out.css",
            missing.len(),
            comps.len(),
            head.join(", "),
            ell,
            shown.join("\n  "),
            more
        ));
    }
    println!(
        "PASS  dist-complete ({} slot selectors from {} component sources all present in dist/out.css)",
        selectors, files
    );
    Ok(selectors)
}

/// gate_reproducible: the committed generated trees must equal what the
/// pipeline just produced. `git status --porcelain` over the generated roots.
///
/// The parity baselines are covered deliberately: a deleted baseline
/// re-records itself green on the next run, so the committed file is what
/// pins the ratchet.

/// Does this libtest output show at least one passing test? libtest exits 0
/// for a zero-match filter, so the unit gate parses the count — a substring
/// check for "0 passed" went red whenever the count merely ENDED in 0
/// ("120 passed" contains "0 passed"). Lives in the lib (not main.rs) so
/// `cargo test --lib -- unit_` — the very gate that uses it — covers it.
pub fn libtest_has_passing_tests(stdout: &str) -> bool {
    for line in stdout.lines() {
        let Some(rest) = line.strip_prefix("test result: ") else {
            continue;
        };
        // "ok. 173 passed; 0 failed; ..." — the count is field 1
        if let Some(count) = rest.split_whitespace().nth(1) {
            if count.parse::<usize>().map(|n| n > 0).unwrap_or(false) {
                return true;
            }
        }
    }
    false
}

fn reproducible_roots() -> &'static [&'static str] {
    &[
        "dist", "docs/catalog.json", "docs/demos", "docs/example-oracle.json",
        "docs/site/content/components", "docs/site/content/guides",
        "docs/site/content/_index.md", "docs/site/content/components/_index.md",
        "docs/site/content/guides/_index.md", "docs/content-map.json",
        "generated/ir", "generated/docs-upstream", "src/kernel/*.html",
        "gates/demo-parity-baseline.json",
        "gates/path-parity-baseline.json",
        "gates/style-parity-baseline.json",
    ]
}

pub fn gate_reproducible(root: &Path) -> Result<usize, String> {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("status").arg("--porcelain").arg("--untracked-files=all").arg("--");
    for r in reproducible_roots() {
        cmd.arg(r);
    }
    cmd.current_dir(root);
    let out = cmd.output().map_err(|e| format!("FAIL  reproducible (git status failed: {})", e))?;
    if !out.status.success() {
        return Err(format!(
            "FAIL  reproducible (git status failed: {})",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let lines: Vec<&str> = stdout
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();
    if !lines.is_empty() {
        let mut shown: Vec<&str> = lines.clone();
        let more = if shown.len() > 40 {
            let n = shown.len() - 40;
            shown.truncate(40);
            format!("\n  … +{} more", n)
        } else {
            String::new()
        };
        return Err(format!(
            "FAIL  reproducible ({} generated paths differ from the committed tree)\n  {}{}",
            lines.len(),
            shown.join("\n  "),
            more
        ));
    }
    println!("PASS  reproducible ({} generated roots match the committed tree)", reproducible_roots().len());
    Ok(lines.len())
}

/// gate_product_verify: slot rules survive the product compile in both
/// chains (the logic lives in emit::product_css::verify_product).
pub fn gate_product_verify(root: &Path) -> Result<(), String> {
    let read = |p: &str| -> Result<String, String> {
        std::fs::read_to_string(root.join(p)).map_err(|e| format!("{}: {}", p, e))
    };
    let full = read("dist/shadless.full.css")?;
    let out = read("dist/out.css")?;
    let names: Vec<String> = crate::fsutil::sorted_read_dir(&root.join("dist/css"))?
        .into_iter()
        .filter(|n| n.ends_with(".css") && n != "shadless.css")
        .collect();
    let mut parts: Vec<String> = Vec::new();
    for n in &names {
        parts.push(read(&format!("dist/css/{}", n))?);
    }
    let parts_css = parts.join("\n");
    let product_source = read("dist/shadless.product.css")?;
    let r = crate::emit::product_css::verify_product(&full, &out, &parts_css, &product_source);
    let mut problems: Vec<String> = Vec::new();
    let mut add = |label: &str, xs: &[String]| {
        if !xs.is_empty() {
            problems.push(format!("{}{}", label, xs.join(", ")));
        }
    };
    add("slot rules missing from product build: ", &r.missing);
    add("slot rules missing from DEMO build (both chains disagree): ", &r.demo_dropped);
    add("docs chrome leaked into product build: ", &r.chrome);
    add("tokens missing from product build: ", &r.tokens);
    add("standalone classes with no origin in product source (content-scan leak?): ", &r.stray);
    if !problems.is_empty() {
        return Err(format!(
            "FAIL  product-css --verify\n  {}",
            problems.join("\n  ")
        ));
    }
    println!("PASS  product-css --verify");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    /// "120 passed" CONTAINS "0 passed" — a substring check for "0 passed"
    /// went red whenever the passing count merely ended in 0.
    #[test]
    fn unit_libtest_count_parse_is_not_a_substring_check() {
        assert!(libtest_has_passing_tests(
            "test result: ok. 120 passed; 0 failed; 0 ignored"
        ));
        assert!(libtest_has_passing_tests(
            "test result: ok. 173 passed; 0 failed; 2 filtered out"
        ));
        assert!(!libtest_has_passing_tests(
            "test result: ok. 0 passed; 0 failed; 2 filtered out"
        ));
        assert!(!libtest_has_passing_tests("no summary at all"));
    }

    /// A deleted parity baseline re-records itself green — reproducible is
    /// what pins the committed file, so the roots must cover all three.
    #[test]
    fn unit_reproducible_covers_the_parity_baselines() {
        for b in [
            "gates/demo-parity-baseline.json",
            "gates/path-parity-baseline.json",
            "gates/style-parity-baseline.json",
        ] {
            assert!(
                reproducible_roots().contains(&b),
                "reproducible must cover {b} — the ratchet resets silently otherwise"
            );
        }
    }

    #[test]
    fn gate_product_verify_on_real_tree() {
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => std::path::PathBuf::from(r),
            Err(_) => {
                crate::crate_adjacent_tree_root()
                    .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
            }
        };
        for p in ["dist/shadless.full.css", "dist/out.css", "dist/shadless.product.css"] {
            if !root.join(p).exists() {
                eprintln!("skip: {} missing (run the demo chain first)", p);
                return;
            }
        }
        gate_product_verify(&root).expect("product-verify gate must pass");
    }

    #[test]
    fn gate_reproducible_on_real_tree() {
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => std::path::PathBuf::from(r),
            Err(_) => {
                crate::crate_adjacent_tree_root()
                    .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
            }
        };
        if !root.join("generated/ir").exists() {
            eprintln!("skip: no shadless tree");
            return;
        }
        gate_reproducible(&root).expect("reproducible gate must pass on a clean tree");
    }

    #[test]
    fn gate_dist_complete_on_real_tree() {
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => std::path::PathBuf::from(r),
            Err(_) => {
                crate::crate_adjacent_tree_root()
                    .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
            }
        };
        let root = root.canonicalize().unwrap_or(root);
        if !root.join("dist/out.css").exists() {
            eprintln!("skip: no shadless tree");
            return;
        }
        gate_dist_complete(&root).expect("dist-complete gate must pass");
    }
}

// ------------------------------------------------- css-direction gate

fn physical_patterns() -> &'static [Regex] {
    static P: OnceLock<Vec<Regex>> = OnceLock::new();
    P.get_or_init(|| {
        [
            r"^m[lr]-",
            r"^p[lr]-",
            r"^s[lr]-",
            r"^(left|right)-",
            r"^text-(left|right)$",
            r"^border-[lr]-\d",
            r"^rounded-[lr]-",
            r"^(space|divide)-x",
        ]
        .iter()
        .map(|p| Regex::new(p).unwrap())
        .collect()
    })
}

/// Utilities that LOOK physical but are sanctioned (a11y semantics). Written
/// as concatenated pairs so a content scan never sees literal utility syntax.
fn sanctioned_utilities() -> &'static HashSet<&'static str> {
    static S: OnceLock<HashSet<&'static str>> = OnceLock::new();
    S.get_or_init(|| HashSet::from(["sr-only"]))
}

/// Keeps only the segment after the LAST ':'.
pub fn utility_segment(token: &str) -> &str {
    match token.rfind(':') {
        Some(i) => &token[i + 1..],
        None => token,
    }
}

pub fn is_physical_utility(token: &str) -> bool {
    let u = utility_segment(token);
    if sanctioned_utilities().contains(u) {
        return false;
    }
    physical_patterns().iter().any(|re| re.is_match(u))
}

fn apply_block_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"@apply([^;]+);").unwrap())
}

pub fn extract_apply_tokens(css: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    for m in apply_block_re().captures_iter(css) {
        tokens.extend(m[1].split_whitespace().map(|s| s.to_string()));
    }
    tokens
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirEntry {
    pub token: String,
    pub n: usize,
}

/// Counts the physical subset, sorted bytewise for stable baselines.
pub fn scan_directions(css: &str) -> Vec<DirEntry> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for tok in extract_apply_tokens(css) {
        if !is_physical_utility(&tok) {
            continue;
        }
        *counts.entry(utility_segment(&tok).to_string()).or_default() += 1;
    }
    let mut out: Vec<DirEntry> = counts
        .into_iter()
        .map(|(token, n)| DirEntry { token, n })
        .collect();
    out.sort_by(|a, b| a.token.cmp(&b.token));
    out
}

/// The recorded physical-utility inventory of the current pin (concatenated
/// pairs — constant expressions, invisible to a tailwind content scan).
fn direction_baseline() -> &'static HashMap<String, usize> {
    static M: OnceLock<HashMap<String, usize>> = OnceLock::new();
    M.get_or_init(|| {
        HashMap::from([
            ("border-l-0".to_string(), 1usize), ("left-3".to_string(), 1), ("ml-1".to_string(), 1),
            ("ml-[-0.15rem]".to_string(), 1), ("ml-[-0.3rem]".to_string(), 1), ("mr-1".to_string(), 1),
            ("mr-[-0.15rem]".to_string(), 1), ("mr-[-0.3rem]".to_string(), 1), ("pl-1.5".to_string(), 4),
            ("pl-1.5!".to_string(), 1), ("pl-2".to_string(), 4), ("pl-2.5".to_string(), 1),
            ("pr-0".to_string(), 2), ("pr-1.5".to_string(), 4), ("pr-1.5!".to_string(), 1),
            ("pr-18".to_string(), 1), ("pr-2".to_string(), 4), ("pr-8".to_string(), 1),
            ("right-2".to_string(), 1), ("right-2.5".to_string(), 1), ("right-3".to_string(), 2),
            ("rounded-l-none".to_string(), 1), ("rounded-r-lg".to_string(), 1),
            ("rounded-r-lg!".to_string(), 1), ("rounded-r-none".to_string(), 1),
            ("text-left".to_string(), 5),
        ])
    })
}

/// gateCSSDirection: emitted physical utilities must match the baseline.
pub fn gate_css_direction(root: &Path) -> Result<usize, String> {
    let css = std::fs::read_to_string(root.join("dist/shadless.css"))
        .map_err(|e| format!("FAIL  css-direction: {}", e))?;
    let entries = scan_directions(&css);

    let mut seen: HashSet<String> = HashSet::new();
    let mut diffs: Vec<String> = Vec::new();
    let baseline = direction_baseline();
    for e in &entries {
        seen.insert(e.token.clone());
        match baseline.get(&e.token) {
            None => diffs.push(format!("  + {} ×{} (new)", e.token, e.n)),
            Some(want) if *want != e.n => {
                diffs.push(format!("  ~ {}: ×{} was ×{}", e.token, e.n, want))
            }
            _ => {}
        }
    }
    let mut gone: Vec<(&String, &usize)> = baseline.iter().filter(|(tok, _)| !seen.contains(*tok)).collect();
    gone.sort();
    for (tok, n) in gone {
        diffs.push(format!("  - {} ×{} (gone)", tok, n));
    }

    if !diffs.is_empty() {
        return Err(format!(
            "FAIL  css-direction-gate: emitted physical utilities drifted from baseline\n{}\n\nIf intended (upstream re-pin / reviewed change): re-record with ./build/pipeline css-direction --update",
            diffs.join("\n")
        ));
    }
    println!("PASS  css-direction-gate ({} physical utilities match baseline)", entries.len());
    Ok(entries.len())
}

#[cfg(test)]
mod css_direction_tests {
    use super::*;

    #[test]
    fn css_direction_matches_baseline_on_real_tree() {
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => std::path::PathBuf::from(r),
            Err(_) => {
                crate::crate_adjacent_tree_root()
                    .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
            }
        };
        if !root.join("dist/shadless.css").exists() {
            eprintln!("skip: no shadless tree");
            return;
        }
        gate_css_direction(&root).expect("css-direction gate must pass");
    }

    #[test]
    fn physical_utility_classification() {
        assert!(is_physical_utility("ml-1"));
        assert!(is_physical_utility("hover:pr-2"));
        assert!(is_physical_utility("text-left"));
        assert!(!is_physical_utility("ms-1")); // logical twin
        assert!(!is_physical_utility("text-sm")); // not direction-related
        assert!(!is_physical_utility("sr-only")); // sanctioned a11y
        assert_eq!(utility_segment("hover:pr-2"), "pr-2");
        assert_eq!(utility_segment("dark:hover:text-left"), "text-left");
    }
}

// ------------------------------------------------- script-refs gate

fn script_refs_res() -> &'static ScriptRefsRes {
    static R: OnceLock<ScriptRefsRes> = OnceLock::new();
    R.get_or_init(|| ScriptRefsRes {
        re_node_call: Regex::new(r#"\bnode\s+([^\s&|;"']+.mjs)"#).unwrap(),
        re_pipeline_cmd: Regex::new(r"\./(?:build/pipeline|\$\(PIPELINE\))\s+([a-zA-Z_][a-zA-Z0-9_-]*)(?:\s+([a-zA-Z_][a-zA-Z0-9_-]*))?").unwrap(),
        re_go_invocation: Regex::new(r"\bgo\s+(test|build|run|generate)\b").unwrap(),
        re_go_mod: Regex::new(r"\bgo\.mod\b").unwrap(),
    })
}

struct ScriptRefsRes {
    re_node_call: Regex,
    re_pipeline_cmd: Regex,
    re_go_invocation: Regex,
    re_go_mod: Regex,
}

/// script-refs: every pipeline invocation named in Makefile and package.json
/// must resolve to a verb this engine dispatches — and the build files must
/// stay Go-free. v1 checked references against pipeline/main.go verbs and
/// pipeline/*_test.go test names; the Go engine's removal (its files are
/// gone) turned the gate into the enforcement side of that contract: a
/// `go test -C pipeline` line returning to the Makefile or package.json is
/// itself a failure.
pub fn gate_script_refs(root: &Path) -> Result<(usize, usize), String> {
    let r = script_refs_res();
    let mut fail: Vec<String> = Vec::new();
    let verbs: std::collections::HashSet<&str> = crate::nodes::VERBS.iter().copied().collect();

    let check = |source: &str, label: &str, fail: &mut Vec<String>| {
        for m in r.re_node_call.captures_iter(source) {
            let p = m[1].to_string();
            if !root.join(&p).exists() {
                fail.push(format!("{}: `node {}` — file does not exist", label, p));
            }
        }
        for m in r.re_pipeline_cmd.captures_iter(source) {
            let v = &m[1];
            if v == "__gate" {
                match m.get(2).map(|g| g.as_str()) {
                    Some(g) if !crate::nodes::GATE_IDS.contains(&g) => {
                        fail.push(format!(
                            "{}: `pipeline __gate {g}` — not a gate id this engine dispatches",
                            label
                        ));
                    }
                    Some(_) => {}
                    None => fail.push(format!(
                        "{}: `pipeline __gate` without an id — name the gate",
                        label
                    )),
                }
                continue;
            }
            if v == "__meta" {
                if let Some(g) = m.get(2) {
                    if !crate::nodes::GATE_IDS.contains(&g.as_str()) {
                        fail.push(format!(
                            "{}: `pipeline __meta {}` — not a gate id this engine dispatches",
                            label, g.as_str()
                        ));
                    }
                }
                continue;
            }
            if !verbs.contains(v) {
                fail.push(format!(
                    "{}: `pipeline {}` — not a verb this engine dispatches",
                    label, v
                ));
            }
        }
        for m in r.re_go_invocation.captures_iter(source) {
            fail.push(format!(
                "{}: `go {}` — the Go engine was removed; build files must not invoke it",
                label, &m[0]
            ));
        }
        if r.re_go_mod.is_match(source) {
            fail.push(format!(
                "{}: go.mod referenced — the Go engine was removed; build files must not depend on it",
                label
            ));
        }
    };

    let pkg_b = std::fs::read_to_string(root.join("package.json"))
        .map_err(|e| format!("FAIL  script-refs (package.json unreadable: {})", e))?;
    #[derive(Deserialize)]
    struct Pkg {
        #[serde(default)]
        scripts: HashMap<String, String>,
    }
    let pkg: Pkg = serde_json::from_str(&pkg_b)
        .map_err(|e| format!("FAIL  script-refs (package.json is not valid JSON: {})", e))?;
    let mut script_names: Vec<&String> = pkg.scripts.keys().collect();
    script_names.sort();
    for n in &script_names {
        check(
            &pkg.scripts[n.as_str()],
            &format!("package.json script {:?}", n),
            &mut fail,
        );
    }

    let makefile = std::fs::read_to_string(root.join("Makefile"))
        .map_err(|e| format!("FAIL  script-refs (Makefile unreadable: {})", e))?;
    check(&makefile, "Makefile", &mut fail);

    if !fail.is_empty() {
        fail.sort();
        return Err(format!(
            "FAIL  script-refs ({} problems)\n  {}",
            fail.len(),
            fail.join("\n  ")
        ));
    }
    println!(
        "PASS  script-refs ({} package.json scripts + Makefile — every node/pipeline call resolves; no Go references)",
        script_names.len()
    );
    Ok((script_names.len(), 0))
}

#[cfg(test)]
mod script_refs_tests {
    use super::*;

    #[test]
    fn script_refs_resolve_on_real_tree() {
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => std::path::PathBuf::from(r),
            Err(_) => {
                crate::crate_adjacent_tree_root()
                    .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
            }
        };
        if !root.join("Makefile").exists() {
            eprintln!("skip: no shadless tree");
            return;
        }
        gate_script_refs(&root).expect("script-refs gate must pass");
    }
}
