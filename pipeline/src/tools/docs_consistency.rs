//! Port of pipeline/docs_consistency.go. Eight checks that a builder cannot
//! answer for itself:
//!
//!   1. skin residue: shipped HTML carries zero non-allowlist cn-* classes
//!   2. install-import reality: every @import "shadless…" the pages TEACH
//!      must resolve through package.json's exports to a file on disk
//!   3. no built page may teach @/components/ui (React import retirement)
//!   4. every dist/js/<name>.js that has a component page is shown on it
//!   5. no built page carries JSX expression residue in prose ("}>")
//!   6. every dist/… path a table row hands the reader is on disk
//!   7. every data-slot an API Reference table lists exists in shipped markup
//!      or is set by the shipped runtime
//!   8. no shipped demo page carries the same id twice

use regex::Regex;
use serde_json::Value;
use std::path::Path;
use std::sync::OnceLock;

use super::docs_transforms::{fence_shadow, re_data_slot_attr, re_data_slot_set};

fn re_cn() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\bcn-[a-z0-9-]+").unwrap())
}
fn re_import_teach() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"@import[ \t]+(?:"|&quot;)shadless[^"&]*"#).unwrap())
}
fn re_dist_path() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("`(dist/[A-Za-z0-9._/-]+)`").unwrap())
}
fn re_id_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"[ \t]id="([^"]+)""#).unwrap())
}
fn re_slot_table_row() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"^\| `data-slot="([a-z0-9-]+)"` \|$"#).unwrap())
}

struct Problem {
    kind: String,
    file: String,
    detail: String,
}

pub fn run_docs_consistency(root: &Path) -> i32 {
    crate::emit::load_skin();

    for dir in ["docs/site/content/components", "docs/site/content/guides"] {
        if !root.join(dir).exists() {
            eprintln!(
                "FAIL  docs-consistency: the markdown pages are not built — run the docs chain first (make docs)"
            );
            return 1;
        }
    }

    let mut page_files: Vec<String> = Vec::new();
    for d in ["docs/site/content/components", "docs/site/content/guides"] {
        if let Ok(ents) = std::fs::read_dir(root.join(d)) {
            for e in ents.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if n.ends_with(".md") {
                    page_files.push(format!("{}/{}", d, n));
                }
            }
        }
    }

    let mut problems: Vec<Problem> = Vec::new();
    let add_problem = |kind: &str, file: &str, detail: String, problems: &mut Vec<Problem>| {
        problems.push(Problem {
            kind: kind.to_string(),
            file: file.to_string(),
            detail,
        });
    };

    // 1. skin residue
    let mut skin_scanned = 0usize;
    for tree in ["dist/components", "docs/demos"] {
        let Ok(ents) = std::fs::read_dir(root.join(tree)) else {
            continue;
        };
        for e in ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if !n.ends_with(".html") {
                continue;
            }
            skin_scanned += 1;
            let Ok(b) = std::fs::read_to_string(root.join(tree).join(&n)) else {
                continue;
            };
            let mut bad: Vec<String> = Vec::new();
            let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
            for m in re_cn().find_iter(&b) {
                let tok = m.as_str().to_string();
                if !seen.insert(tok.clone()) {
                    continue;
                }
                if !crate::emit::skin_data().allowlist.contains(&tok) {
                    bad.push(tok);
                }
            }
            if !bad.is_empty() {
                add_problem(
                    "cn-residue",
                    &format!("{}/{}", tree, n),
                    bad.join(" "),
                    &mut problems,
                );
            }
        }
    }

    // 2. install-import reality
    let pkg_b = match std::fs::read_to_string(root.join("package.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("docs-consistency: {}", e);
            return 1;
        }
    };
    let pkg: Value = serde_json::from_str(&pkg_b).unwrap_or(Value::Null);
    let exports = pkg
        .get("exports")
        .cloned()
        .unwrap_or(Value::Object(Default::default()));
    let mut imports_checked = 0usize;
    let mut seen_import: std::collections::HashSet<String> = std::collections::HashSet::new();
    for f in &page_files {
        let Ok(b) = std::fs::read_to_string(root.join(f)) else {
            continue;
        };
        for spec in re_import_teach().find_iter(&b) {
            let spec = spec.as_str();
            let key = Regex::new(r#"@import[ \t]+|"|&quot;"#)
                .unwrap()
                .replace_all(spec, "")
                .into_owned();
            if !seen_import.insert(key.clone()) {
                continue;
            }
            imports_checked += 1;
            let target = resolve_import(&exports, &key);
            if target.is_empty() {
                add_problem(
                    "install-import",
                    f,
                    format!("{}: no exports rule resolves it", key),
                    &mut problems,
                );
                continue;
            }
            if !root.join(&target).exists() {
                add_problem(
                    "install-import",
                    f,
                    format!("{} → {} does not exist on disk", key, target),
                    &mut problems,
                );
            }
        }
    }

    // 3. React-import retirement
    for f in &page_files {
        let Ok(b) = std::fs::read_to_string(root.join(f)) else {
            continue;
        };
        if b.contains("@/components/ui") {
            add_problem(
                "react-import",
                f,
                "built page still teaches a React import".to_string(),
                &mut problems,
            );
        }
    }

    // 4. shipped JS is documented. Artifact-only on purpose.
    let mut behavior_checked = 0usize;
    if let Ok(js_ents) = std::fs::read_dir(root.join("dist/js")) {
        for e in js_ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if e.file_type().map(|t| t.is_dir()).unwrap_or(false) || !n.ends_with(".js") {
                continue;
            }
            let name = n.trim_end_matches(".js");
            let page = format!("docs/site/content/components/{}.md", name);
            let Ok(b) = std::fs::read_to_string(root.join(&page)) else {
                continue; // not every dist/js entry has a component page
            };
            behavior_checked += 1;
            if !b.contains("js,name=behavior") {
                add_problem(
                    "behavior-tab-missing",
                    &page,
                    format!("dist/js/{} ships but the page shows no [behavior] tab", n),
                    &mut problems,
                );
            }
        }
    }

    // 5. no JSX expression residue in prose. Artifact-only.
    for f in &page_files {
        let Ok(b) = std::fs::read_to_string(root.join(f)) else {
            continue;
        };
        for (i, line) in fence_shadow(&b).split('\n').enumerate() {
            let t = line.trim();
            if t.starts_with("}>") || t.starts_with("/>}") {
                add_problem(
                    "jsx-residue",
                    f,
                    format!(
                        "line {}: {:?} — an unclosed JSX attribute expression leaked into prose",
                        i + 1,
                        t
                    ),
                    &mut problems,
                );
            }
        }
    }

    // 6. every dist/… path a table row hands the reader exists. Scoped to
    // table rows on purpose.
    let mut dist_refs = 0usize;
    for f in &page_files {
        let Ok(b) = std::fs::read_to_string(root.join(f)) else {
            continue;
        };
        for (i, line) in b.split('\n').enumerate() {
            if !line.trim_start().starts_with('|') {
                continue;
            }
            for m in re_dist_path().captures_iter(line) {
                dist_refs += 1;
                if !root.join(&m[1]).exists() {
                    add_problem(
                        "dist-ref",
                        f,
                        format!(
                            "line {}: table row names {}, which is not on disk",
                            i + 1,
                            &m[1]
                        ),
                        &mut problems,
                    );
                }
            }
        }
    }

    // 7. every slot a page's table lists actually exists.
    let mut real: std::collections::HashSet<String> = std::collections::HashSet::new();
    for dir in ["dist/components", "docs/demos"] {
        if let Ok(ents) = std::fs::read_dir(root.join(dir)) {
            for e in ents.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if !n.ends_with(".html") {
                    continue;
                }
                let Ok(b) = std::fs::read_to_string(root.join(dir).join(&n)) else {
                    continue;
                };
                for m in re_data_slot_attr().captures_iter(&b) {
                    real.insert(m[1].to_string());
                }
            }
        }
    }
    let mut js_files: Vec<String> = vec!["dist/shadless.js".to_string()];
    if let Ok(ents) = std::fs::read_dir(root.join("dist/js")) {
        for e in ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.ends_with(".js") {
                js_files.push(format!("dist/js/{}", n));
            }
        }
    }
    for f in &js_files {
        let Ok(b) = std::fs::read_to_string(root.join(f)) else {
            continue;
        };
        for m in re_data_slot_set().captures_iter(&b) {
            real.insert(m[1].to_string());
        }
    }
    let mut slot_rows = 0usize;
    for f in &page_files {
        let Ok(b) = std::fs::read_to_string(root.join(f)) else {
            continue;
        };
        for (i, line) in b.split('\n').enumerate() {
            let Some(m) = re_slot_table_row().captures(line) else {
                continue;
            };
            slot_rows += 1;
            if !real.contains(&m[1]) {
                add_problem(
                    "phantom-slot",
                    f,
                    format!(
                        "line {}: the table lists data-slot={:?}, which is in no shipped page and set by no shipped script",
                        i + 1,
                        &m[1]
                    ),
                    &mut problems,
                );
            }
        }
    }

    // 8. no duplicate `id` inside a shipped demo page.
    let dup_exempt: std::collections::HashMap<&str, std::collections::HashMap<&str, &str>> = [
        (
            "collapsible-settings",
            [("radius", "upstream tsx repeats id=radius")]
                .into_iter()
                .collect::<std::collections::HashMap<_, _>>(),
        ),
    ]
    .into_iter()
    .collect();
    let mut demo_id_pages = 0usize;
    if let Ok(ents) = std::fs::read_dir(root.join("docs/demos")) {
        for e in ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if !n.ends_with(".html") {
                continue;
            }
            let Ok(b) = std::fs::read_to_string(root.join("docs/demos").join(&n)) else {
                continue;
            };
            demo_id_pages += 1;
            let page = n.trim_end_matches(".html");
            let mut count: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for m in re_id_attr().captures_iter(&b) {
                *count.entry(m[1].to_string()).or_insert(0) += 1;
            }
            let mut ids: Vec<&String> = count.keys().collect();
            ids.sort();
            for id in ids {
                if count[id] < 2
                    || dup_exempt
                        .get(page)
                        .and_then(|m| m.get(id.as_str()))
                        .is_some()
                {
                    continue;
                }
                add_problem(
                    "duplicate-id",
                    &format!("docs/demos/{}", n),
                    format!("id={:?} appears {} times", id, count[id]),
                    &mut problems,
                );
            }
            // an exemption is a claim about the page; when the duplicate it
            // names is gone the claim is stale and the gate says so
            let mut ex_ids: Vec<&str> = dup_exempt
                .get(page)
                .map(|m| m.keys().map(|s| *s).collect())
                .unwrap_or_default();
            ex_ids.sort();
            for id in ex_ids {
                if count.get(id).copied().unwrap_or(0) < 2 {
                    add_problem(
                        "stale-exemption",
                        &format!("docs/demos/{}", n),
                        format!(
                            "exemption names id={:?} ({}) but it appears {} time(s) — remove the exemption",
                            id,
                            dup_exempt[page][id],
                            count.get(id).copied().unwrap_or(0)
                        ),
                        &mut problems,
                    );
                }
            }
        }
    }

    // report
    let mut by_kind: std::collections::HashMap<String, Vec<&Problem>> =
        std::collections::HashMap::new();
    for p in &problems {
        by_kind.entry(p.kind.clone()).or_default().push(p);
    }
    let mut ks: Vec<&String> = by_kind.keys().collect();
    ks.sort();
    for k in ks {
        let list = &by_kind[k];
        eprintln!("FAIL  {} ({}):", k, list.len());
        let n = list.len().min(10);
        for p in list.iter().take(n) {
            eprintln!("  - {}: {}", p.file, p.detail);
        }
        if list.len() > 10 {
            eprintln!("  … +{} more", list.len() - 10);
        }
    }
    println!(
        "docs consistency: {} shipped pages scanned for skin residue, {} taught @imports resolved, {} built pages checked for React imports, {} shipped js files checked for a behavior tab, {} table dist refs resolved, {} slot rows checked, {} demo pages checked for duplicate ids — problems: {}",
        skin_scanned,
        imports_checked,
        page_files.len(),
        behavior_checked,
        dist_refs,
        slot_rows,
        demo_id_pages,
        problems.len()
    );
    if !problems.is_empty() {
        return 1;
    }
    0
}

/// resolveImport mirrors the JS gate's check, which reads package.json's
/// exports the way node resolution does: exact keys first, then the "./*"
/// wildcard (./* → ./dist/css/*.css per the exports map).
fn resolve_import(exports: &Value, spec: &str) -> String {
    if spec == "shadless" || spec == "shadless/" {
        if let Some(s) = exports.get(".").and_then(|v| v.as_str()) {
            return s.strip_prefix("./").unwrap_or(s).to_string();
        }
        return String::new();
    }
    // exact key: "shadless/accordion.css" → "./accordion.css"
    let sub = format!(".{}", spec.strip_prefix("shadless").unwrap_or(spec)); // spec carries "/" already
    if let Some(v) = exports.get(&sub) {
        match v {
            Value::String(s) => return s.strip_prefix("./").unwrap_or(s).to_string(),
            Value::Object(o) => {
                if let Some(imp) = o.get("import").and_then(|v| v.as_str()) {
                    return imp.strip_prefix("./").unwrap_or(imp).to_string();
                }
                if let Some(d) = o.get("default").and_then(|v| v.as_str()) {
                    return d.strip_prefix("./").unwrap_or(d).to_string();
                }
            }
            _ => {}
        }
    }
    // wildcard: "./*" → "./dist/css/*.css"; the wildcard already supplies the
    // ".css" suffix — do not add one.
    if let Some(w) = exports.get("./*").and_then(|v| v.as_str()) {
        return w
            .replacen('*', sub.strip_prefix("./").unwrap_or(&sub), 1)
            .strip_prefix("./")
            .unwrap_or("")
            .to_string();
    }
    String::new()
}
