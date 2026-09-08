//! Port of pipeline/docs_fidelity_driver.go — mdx↔markdown page fidelity over
//! EVERY page in docs/content-map.json.

use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;

use super::docs_transforms::{
    all_hrefs_of, apply_text_adjustments, compare_page, docs_hrefs_of, guides, md_page_facts,
    mdx_page_facts, rewrite_utility_jsx_fences, text_adjustments,
};

fn re_comp_source() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"/components/radix/").unwrap())
}
fn re_variant_href() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"/components/(base|aria)/|-(base|aria)\.html$").unwrap())
}

pub fn run_docs_fidelity(root: &Path) -> i32 {
    const DEMOS: &str = "docs/site/static/demos";
    let mut guide_by_slug: std::collections::HashMap<&str, &super::docs_transforms::Guide> =
        std::collections::HashMap::new();
    for g in guides() {
        guide_by_slug.insert(g.slug, g);
    }
    let page_path = |name: &str| -> String {
        if guide_by_slug.contains_key(name) {
            format!("docs/site/content/guides/{}.md", name)
        } else {
            format!("docs/site/content/components/{}.md", name)
        }
    };
    if !root.join("docs/site/content/_index.md").exists() {
        eprintln!(
            "FAIL  docs-fidelity: the markdown pages are not built — run the docs chain first (make docs)"
        );
        return 1;
    }

    let cm_b = match std::fs::read_to_string(root.join("docs/content-map.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("docs-fidelity: {}", e);
            return 1;
        }
    };
    // ordered read: pages iteration order must match the JS Object.entries
    let cm_top: serde_json::Value = match serde_json::from_str(&cm_b) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("docs-fidelity: content-map: {}", e);
            return 1;
        }
    };
    let pages_obj = match cm_top.get("pages").and_then(|v| v.as_object()) {
        Some(o) => o,
        None => {
            eprintln!("docs-fidelity: content-map: pages is not an object");
            return 1;
        }
    };

    struct Issue {
        page: String,
        kind: String,
        detail: String,
    }
    let mut issues: Vec<Issue> = Vec::new();
    let mut pages = 0usize;

    for (name, raw) in pages_obj {
        if name == "index" {
            continue;
        }
        #[derive(Deserialize)]
        struct Meta {
            #[serde(default)]
            source: String,
        }
        let Ok(meta) = serde_json::from_value::<Meta>(raw.clone()) else {
            continue;
        };
        let md_path = page_path(name);
        if !root.join(&md_path).exists() {
            issues.push(Issue {
                page: name.clone(),
                kind: "missing-page".to_string(),
                detail: "content-map page has no built markdown".to_string(),
            });
            continue;
        }
        pages += 1;
        let is_component = re_comp_source().is_match(&meta.source);
        let g = guide_by_slug.get(name.as_str()).copied();

        if meta.source.is_empty() || !root.join(&meta.source).exists() {
            issues.push(Issue {
                page: name.clone(),
                kind: "source-missing".to_string(),
                detail: format!("source {} unreadable", meta.source),
            });
            continue;
        }
        let raw_b = match std::fs::read_to_string(root.join(&meta.source)) {
            Ok(b) => b,
            Err(_) => {
                issues.push(Issue {
                    page: name.clone(),
                    kind: "source-missing".to_string(),
                    detail: format!("source {} unreadable", meta.source),
                });
                continue;
            }
        };
        let base = std::path::Path::new(&meta.source)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        let adjusted = match apply_text_adjustments(&base, &raw_b) {
            Ok(a) => a,
            Err(e) => {
                issues.push(Issue {
                    page: name.clone(),
                    kind: "text-adjustment".to_string(),
                    detail: e,
                });
                continue;
            }
        };
        let adjusted = if let Some(g) = g {
            if !g.util.is_empty() {
                match rewrite_utility_jsx_fences(g.slug, &adjusted) {
                    Ok(a) => a,
                    Err(e) => {
                        issues.push(Issue {
                            page: name.clone(),
                            kind: "utility-jsx-fences".to_string(),
                            detail: e,
                        });
                        continue;
                    }
                }
            } else {
                adjusted
            }
        } else {
            adjusted
        };
        let fix_leaked_jsx = g.map(|g| g.util.is_empty()).unwrap_or(true);
        let m = match mdx_page_facts(
            name,
            &adjusted,
            is_component,
            g.map(|g| g.install_section).unwrap_or(false),
            g.map(|g| g.rtl_migrate).unwrap_or(false),
            is_component,
            is_component,
            is_component,
            is_component,
            fix_leaked_jsx,
            root,
        ) {
            Ok(m) => m,
            Err(e) => {
                issues.push(Issue {
                    page: name.clone(),
                    kind: "leaked-jsx".to_string(),
                    detail: e,
                });
                continue;
            }
        };
        let md_b = std::fs::read_to_string(root.join(&md_path)).unwrap_or_default();
        let h = md_page_facts(&md_b);

        // The rewritten Installation must name something specific to THIS
        // component, proving the manual tab was regenerated and not left as
        // upstream's shadcn-CLI text.
        let expected = if is_component && root.join("dist/css").join(format!("{}.css", name)).exists() {
            format!("shadless/{}.css", name)
        } else {
            String::new()
        };
        for d in compare_page(&m, &h, name, is_component, &expected) {
            let (kind, detail) = match d.find(": ") {
                Some(i) => (d[..i].to_string(), d[i + 2..].to_string()),
                None => (d, String::new()),
            };
            issues.push(Issue {
                page: name.clone(),
                kind,
                detail,
            });
        }

        // disk-existence checks: VitePress follows page links itself; the
        // demo iframes are raw html it does not check
        for src in &h.iframes {
            let base_name = std::path::Path::new(src)
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();
            if !root.join(DEMOS).join(&base_name).exists() {
                issues.push(Issue {
                    page: name.clone(),
                    kind: "iframe-404".to_string(),
                    detail: src.clone(),
                });
            }
        }
        let dh = docs_hrefs_of(&md_b);
        if !dh.is_empty() {
            issues.push(Issue {
                page: name.clone(),
                kind: "docs-href".to_string(),
                detail: format!("unrewritten: {}", crate::emit::css::dedup(&dh).join(", ")),
            });
        }

        // variant retirement (2026-08-26): base/aria mirror is GONE
        let mut variant_hrefs: Vec<String> = Vec::new();
        for h in all_hrefs_of(&md_b) {
            if re_variant_href().is_match(&h) {
                variant_hrefs.push(h);
            }
        }
        if !variant_hrefs.is_empty() {
            issues.push(Issue {
                page: name.clone(),
                kind: "variant-href".to_string(),
                detail: format!(
                    "links to retired variant pages: {}",
                    crate::emit::css::dedup(&variant_hrefs).join(", ")
                ),
            });
        }

        // retired prose: declared TEXT_ADJUSTMENTS must not survive
        for adj in text_adjustments() {
            if !adj.files.contains(&base.as_str()) {
                continue;
            }
            for op in adj.ops {
                if h.text.contains(op.find) {
                    let n = op.find.len().min(60);
                    issues.push(Issue {
                        page: name.clone(),
                        kind: "retired-prose".to_string(),
                        detail: format!(
                            "{}: {:?} survives in the built page",
                            adj.id,
                            &op.find[..n]
                        ),
                    });
                }
            }
        }
    }

    // report
    let mut by_kind: std::collections::HashMap<String, Vec<&Issue>> =
        std::collections::HashMap::new();
    let mut kinds: Vec<String> = Vec::new();
    for i in &issues {
        if !by_kind.contains_key(&i.kind) {
            kinds.push(i.kind.clone());
        }
        by_kind.entry(i.kind.clone()).or_default().push(i);
    }
    kinds.sort();
    for k in kinds {
        let list = &by_kind[&k];
        eprintln!("FAIL  {} ({}):", k, list.len());
        let n = list.len().min(8);
        for i in list.iter().take(n) {
            eprintln!("  - [{}] {}", i.page, i.detail);
        }
        if list.len() > 8 {
            eprintln!("  … +{} more", list.len() - 8);
        }
    }
    println!(
        "docs fidelity: {} pages compared against mdx sources — issues: {}",
        pages,
        issues.len()
    );
    if !issues.is_empty() {
        eprintln!("FAIL  docs fidelity (built pages drift from their mdx sources)");
        return 1;
    }
    println!("PASS  docs fidelity (every page matches its mdx source: headings/previews/fences/links)");
    0
}
