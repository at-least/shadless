//! Port of pipeline/gate_pack.go — the npm surface, machine-checked.
//!
//! Checks (no network — `npm pack --dry-run --json` lists the tarball):
//!   1. dependencies is empty (a React-free library must not install React);
//!   2. every export target exists on disk AND is in the tarball;
//!   3. every `shadless…` specifier the README documents resolves through the
//!      exports map;
//!   4. the tarball carries nothing outside the product surface.

use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

static RE_EXPORT_STMT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bexport[\t\n\f\r {]").unwrap());
static RE_SPEC_BACKTICK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("`(shadless(?:/[^`\\s]*)?)`").unwrap());
static RE_SPEC_IMPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("(?:@import|from|import)[\\t\\n\\f\\r ]+\"(shadless(?:/[^\"]*)?)\"").unwrap()
});
static RE_ALLOWED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
            r"^(package\.json|README\.md|CHANGELOG\.md|LICENSE|dist/(css|js|esm)/[^/]+|dist/shadless(-core|\.full(\.min)?)?\.(css|js)|dist/shadless\.min\.js)$",
        )
        .unwrap()
});

fn npm_pack_files(root: &Path) -> Result<BTreeMap<String, bool>, String> {
    let out = std::process::Command::new("npm")
        .args(["pack", "--dry-run", "--json", "--ignore-scripts"])
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("npm pack failed".to_string());
    }
    let res: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    let arr = res.as_array().ok_or("unexpected npm pack output")?;
    let first = arr.first().ok_or("unexpected npm pack output")?;
    let mut packed = BTreeMap::new();
    for f in first
        .get("files")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
    {
        if let Some(p) = f.get("path").and_then(|v| v.as_str()) {
            packed.insert(p.to_string(), true);
        }
    }
    Ok(packed)
}

/// addTarget, ported recursively: key + condition label → files.
fn add_target(
    root: &Path,
    key: &str,
    cond: &str,
    value: &Value,
    targets: &mut BTreeMap<String, Vec<String>>,
    fail: &mut Vec<String>,
) {
    match value {
        Value::String(v) => {
            let label = format!("{} [{}]", key, cond);
            if v.contains('*') {
                let (pre, post) = match v.split_once('*') {
                    Some(x) => x,
                    None => (v.as_str(), ""),
                };
                let trimmed = pre.strip_prefix("./").unwrap_or(pre);
                let (dir, stem) = match trimmed.rfind('/') {
                    Some(i) => (&trimmed[..i], &trimmed[i + 1..]),
                    None => ("", trimmed),
                };
                let mut files: Vec<String> = Vec::new();
                if let Ok(names) = crate::fsutil::sorted_read_dir(&root.join(dir)) {
                    for name in names {
                        if name.starts_with(stem) && name.ends_with(post) {
                            files.push(format!("{}/{}", dir, name));
                        }
                    }
                }
                if files.is_empty() {
                    fail.push(format!(
                        "export {} [{}] → {}: pattern matches nothing",
                        key, cond, v
                    ));
                }
                targets.insert(label, files);
            } else {
                targets.insert(label, vec![v.strip_prefix("./").unwrap_or(v).to_string()]);
            }
        }
        Value::Object(obj) => {
            let mut keys: Vec<&String> = obj.keys().collect();
            keys.sort();
            for c in keys {
                add_target(root, key, c, &obj[c], targets, fail);
            }
        }
        _ => {}
    }
}

pub fn gate_pack(root: &Path) -> Result<(), String> {
    let raw = std::fs::read_to_string(root.join("package.json"))
        .map_err(|e| format!("FAIL  pack (package.json unreadable: {})", e))?;
    let pkg: Value = serde_json::from_str(&raw)
        .map_err(|e| format!("FAIL  pack (package.json is not valid JSON: {})", e))?;
    let empty_map = Value::Object(Default::default());
    let dependencies = pkg.get("dependencies").unwrap_or(&empty_map);
    let exports = pkg.get("exports").unwrap_or(&empty_map);
    let mut fail: Vec<String> = Vec::new();

    // 1. dependencies
    if let Some(deps) = dependencies.as_object() {
        if !deps.is_empty() {
            let mut names: Vec<&String> = deps.keys().collect();
            names.sort();
            fail.push(format!(
                "dependencies must be empty (React-free means installing nothing): {}",
                names
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    // 2. exports -> files
    let mut targets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut export_keys: Vec<String> = exports
        .as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    export_keys.sort();
    for k in &export_keys {
        add_target(
            root,
            k,
            "default",
            &exports.as_object().unwrap()[k],
            &mut targets,
            &mut fail,
        );
    }
    if let Some(style) = pkg.get("style").and_then(|v| v.as_str()) {
        targets.insert(
            "style".to_string(),
            vec![style.strip_prefix("./").unwrap_or(style).to_string()],
        );
    }
    // the `import` condition must resolve to a real ES module
    for k in &export_keys {
        let Some(obj) = exports.as_object().unwrap()[k].as_object() else {
            continue;
        };
        let Some(imp) = obj.get("import").and_then(|v| v.as_str()) else {
            continue;
        };
        if imp.contains('*') {
            continue;
        }
        let src = std::fs::read_to_string(root.join(imp)).unwrap_or_default();
        if !RE_EXPORT_STMT.is_match(&src) {
            fail.push(format!(
                "export {} [import] → {} has no export statement (an IIFE under the import condition yields undefined)",
                k, imp
            ));
        }
    }

    // ONE ES-module base. Every dist/esm/<name>.mjs opens with
    // `import "./shadless.mjs"` and then registers on the global, so a second
    // base module cannot be shared with them. Read off the artifacts — a base
    // is any dist/esm/*.mjs that does not import the base.
    let mut bases: Vec<String> = Vec::new();
    for n in crate::fsutil::sorted_read_dir(&root.join("dist/esm")).unwrap_or_default() {
        if !n.ends_with(".mjs") {
            continue;
        }
        let Ok(src) = std::fs::read_to_string(root.join("dist/esm").join(&n)) else {
            continue;
        };
        if !src.contains("import \"./shadless.mjs\"") {
            bases.push(format!("dist/esm/{}", n));
        }
    }
    if bases.len() > 1 {
        fail.push(format!(
            "{} ES-module bases in dist/esm ({}) — component modules import ./shadless.mjs by path, so any other base is an instance they never register on",
            bases.len(),
            bases.join(", ")
        ));
    }

    let packed =
        npm_pack_files(root).map_err(|e| format!("FAIL  pack (npm pack failed: {})", e))?;
    for (label, files) in &targets {
        for f in files {
            if !root.join(f).exists() {
                fail.push(format!("export {} → {}: file does not exist", label, f));
            } else if !packed.contains_key(f) {
                fail.push(format!(
                    "export {} → {}: not in the tarball (package.json \"files\")",
                    label, f
                ));
            }
        }
    }

    // 3. README specifiers resolve
    let readme = std::fs::read_to_string(root.join("README.md")).unwrap_or_default();
    let mut spec_set: BTreeMap<String, bool> = BTreeMap::new();
    for m in RE_SPEC_BACKTICK.captures_iter(&readme) {
        spec_set.insert(m[1].to_string(), true);
    }
    for m in RE_SPEC_IMPORT.captures_iter(&readme) {
        spec_set.insert(m[1].to_string(), true);
    }
    let specs: Vec<String> = spec_set.keys().cloned().collect();
    let resolves = |spec: &str| -> bool {
        let sub = if spec == "shadless" {
            ".".to_string()
        } else {
            format!(".{}", spec.strip_prefix("shadless").unwrap_or(spec))
        };
        if exports
            .as_object()
            .map(|o| o.contains_key(&sub))
            .unwrap_or(false)
        {
            return true;
        }
        for k in &export_keys {
            if !k.contains('*') {
                continue;
            }
            let Some((pre, post)) = k.split_once('*') else {
                continue;
            };
            if sub.starts_with(pre) && sub.ends_with(post) && sub.len() > pre.len() + post.len() {
                return true;
            }
        }
        false
    };
    for s in &specs {
        // the README's <name> placeholder
        if !resolves(&s.replace("<name>", "button")) {
            fail.push(format!(
                "README documents {:?} but package.json exports do not resolve it",
                s
            ));
        }
    }

    // 3b. README's "Not included" prose is grey_components(), not a drifted
    // copy of it: the comma list between the heading and the em-dash must
    // set-equal the registry's grey list (the two-places rule).
    if let Some(head) = readme.find("## Not included") {
        let tail = &readme[head..];
        // drop the heading line itself — the list is the prose after it
        let after_head = &tail[tail.find('\n').unwrap_or(tail.len())..];
        let body = match after_head.find('—') {
            Some(d) => &after_head[..d],
            None => after_head,
        };
        let listed: BTreeMap<&str, bool> = body
            .split([',', ' ', '\n', '\t'])
            .filter(|t| {
                !t.is_empty()
                    && t.chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            })
            .map(|t| (t, true))
            .collect();
        for t in listed.keys() {
            if !crate::tools::docs_transforms::grey_components().contains(t) {
                fail.push(format!(
                    "README \"Not included\" lists {:?} but grey_components() does not",
                    t
                ));
            }
        }
        for g in crate::tools::docs_transforms::grey_components() {
            if !listed.contains_key(g) {
                fail.push(format!(
                    "grey_components() lists {:?} but README \"Not included\" does not",
                    g
                ));
            }
        }
    } else {
        fail.push("README has no \"## Not included\" section to cross-check against grey_components()".to_string());
    }

    // 4. nothing outside the product surface
    for f in packed.keys() {
        if !RE_ALLOWED.is_match(f) {
            fail.push(format!(
                "tarball carries {} — outside the product surface",
                f
            ));
        }
    }

    if !fail.is_empty() {
        return Err(format!(
            "FAIL  pack ({} problems)\n  {}",
            fail.len(),
            fail.join("\n  ")
        ));
    }
    println!(
        "PASS  pack ({} export targets in a {}-file tarball; {} README specifiers resolve; dependencies empty)",
        targets.len(),
        packed.len(),
        specs.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Go TestPack: gate(t, gatePack).
    #[test]
    fn pack_on_real_tree() {
        let root = crate::tree_root();
        if !root.join("package.json").exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: no shadless tree)");
            }
            eprintln!("skip: no shadless tree");
            return;
        }
        gate_pack(&root).expect("pack gate must pass");
    }
}
