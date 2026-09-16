//! Port of pipeline/demo_parity.go — every shipped demo page, styled by our
//! CSS, computes what the SAME DOM computes under upstream's own stylesheet.
//! Same DOM on both sides, so every difference is the emitted CSS. Cells
//! ratcheted in gates/demo-parity-baseline.json.

use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;

use super::parity_baseline::{
    cell_map, diff_parity_baseline, load_parity_baseline, parity_cell, parity_norm_value,
    show_cell, show_change, write_parity_baseline,
};

const DEMO_PARITY_BASELINE: &str = "gates/demo-parity-baseline.json";
const DEMO_PARITY_COLLECT: &str = include_str!("demo_parity_collect.js");

const DEMO_PARITY_PROPS: [&str; 28] = [
    "color", "background-color", "border-color", "border-top-width", "border-radius", "padding-top",
    "padding-right", "padding-bottom", "padding-left", "margin-top", "margin-left", "width", "height",
    "min-width", "max-width", "font-size", "font-weight", "line-height", "display", "flex-direction",
    "align-items", "justify-content", "gap", "position", "opacity", "box-shadow", "text-align", "overflow",
];

// harness shell pinned on both sides: neither body convention is the component
const DEMO_PARITY_FREEZE: &str = "*,*::before,*::after{transition:none!important;animation:none!important} body{padding:0!important;margin:0;color:var(--foreground);background:var(--background)}";

fn re_body() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)<body([^>]*)>(.*)</body>").unwrap())
}
fn re_script() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)<script.*?</script>").unwrap())
}

#[derive(Deserialize)]
struct Owned {
    #[serde(default)]
    name: String,
    #[serde(default)]
    out: String,
}

pub fn run_demo_parity(root: &Path, record: bool, details: bool) -> i32 {
    let ob = match std::fs::read_to_string(root.join("docs/example-oracle.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("demo-parity: {}", e);
            return 1;
        }
    };
    let owned: Vec<Owned> = serde_json::from_str(&ob).unwrap_or_default();
    let oracle_css = std::fs::read_to_string(root.join("build/gates/oracle.css"))
        .unwrap_or_default();
    let out_css = std::fs::read_to_string(root.join("dist/out.css")).unwrap_or_default();

    let shell = match crate::oracle::browser_shell::BrowserShell::start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("demo-parity: {}", e);
            return 1;
        }
    };
    if let Err(e) = shell.launch() {
        eprintln!("demo-parity: {}", e);
        return 1;
    }
    let page = match shell.new_page(false) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("demo-parity: {}", e);
            return 1;
        }
    };
    let _ = page.route_abort_external();

    let mut cells: Vec<parity_cell> = Vec::new();
    let (mut pages, mut compared) = (0usize, 0usize);
    for t in &owned {
        if !t.out.starts_with("docs/demos/") {
            continue;
        }
        let Ok(html) = std::fs::read_to_string(root.join(&t.out)) else {
            continue;
        };
        let Some(m) = re_body().captures(&html) else {
            continue;
        };
        let bare = re_script().replace_all(&m[2], "");
        let doc = |css: &str, root_class: &str| -> String {
            format!(
                "<!doctype html><html class=\"{}\"><head><style>{}</style><style>{}</style></head><body{}>{}</body></html>",
                root_class, css, DEMO_PARITY_FREEZE, &m[1], bare
            )
        };
        let collect_once = || -> std::collections::HashMap<String, std::collections::HashMap<String, String>> {
            let Ok(v) = page.evaluate_fn_arg(DEMO_PARITY_COLLECT, serde_json::json!(DEMO_PARITY_PROPS)) else {
                return Default::default();
            };
            let mut out = std::collections::HashMap::new();
            if let Some(obj) = v.as_object() {
                for (k, sv) in obj {
                    if let Some(sm) = sv.as_object() {
                        let mut cellm = std::collections::HashMap::new();
                        for (p, val) in sm {
                            if let Some(s) = val.as_str() {
                                cellm.insert(p.clone(), s.to_string());
                            }
                        }
                        out.insert(k.clone(), cellm);
                    }
                }
            }
            out
        };
        let _ = page.evaluate_fn_arg(
            r#"(html) => { document.open(); document.write(html); document.close(); return true }"#,
            serde_json::json!(doc(&out_css, "")),
        );
        let _ = page.wait_for_timeout(30);
        let ours = collect_once();
        let _ = page.evaluate_fn_arg(
            r#"(html) => { document.open(); document.write(html); document.close(); return true }"#,
            serde_json::json!(doc(&oracle_css, "style-nova")),
        );
        let _ = page.wait_for_timeout(30);
        let theirs = collect_once();
        pages += 1;
        for (k, ref_) in &theirs {
            let Some(got) = ours.get(k) else { continue };
            compared += 1;
            let parts: Vec<&str> = k.split('@').collect();
            let (slot_key, theme, dir) = (parts[0], parts[1], parts[2]);
            for p in DEMO_PARITY_PROPS {
                let a = parity_norm_value(ref_.get(p).map(String::as_str).unwrap_or(""), false);
                let b = parity_norm_value(got.get(p).map(String::as_str).unwrap_or(""), false);
                if a != b {
                    cells.push(parity_cell {
                        id: format!("{}/{}/{}@{}@{}", t.name, slot_key, p, theme, dir),
                        oracle: a,
                        shadless: b,
                    });
                }
            }
        }
    }

    let (actual, order) = cell_map(&cells);
    if details {
        for id in &order {
            if id.ends_with("@light@ltr") {
                println!("{}: {}", id, show_cell(&actual[id]));
            }
        }
    }
    if record || !root.join(DEMO_PARITY_BASELINE).exists() {
        if let Err(e) = write_parity_baseline(
            root,
            DEMO_PARITY_BASELINE,
            "shipped demo DOM under our css vs the same DOM under upstream css; may only shrink, and a recorded cell's VALUES are pinned too",
            &[],
            &actual,
        ) {
            eprintln!("demo-parity: {}", e);
            return 1;
        }
        println!(
            "demo-parity: baseline recorded ({} cells over {} pages, {} element×theme×dir comparisons)",
            actual.len(),
            pages,
            compared
        );
        return 0;
    }
    let recorded = match load_parity_baseline(root, DEMO_PARITY_BASELINE) {
        Ok(Some((_, r))) => r,
        Ok(None) => Default::default(),
        Err(e) => {
            eprintln!("demo-parity: {}", e);
            return 1;
        }
    };
    let d = diff_parity_baseline(&recorded, &actual, &order);
    if !d.appeared.is_empty() {
        let n = d.appeared.len().min(40);
        let parts: Vec<String> = d.appeared[..n]
            .iter()
            .map(|id| format!("{}: {}", id, show_cell(&actual[id])))
            .collect();
        eprintln!(
            "FAIL  demo-parity ({} NEW cells where a shipped demo ≠ upstream css)\n  {}\n",
            d.appeared.len(),
            parts.join("\n  ")
        );
        return 1;
    }
    if !d.changed.is_empty() {
        let n = d.changed.len().min(20);
        let parts: Vec<String> = d.changed[..n].iter().map(show_change).collect();
        eprintln!(
            "FAIL  demo-parity ({} recorded cells still differ, but by a DIFFERENT amount — re-look, then re-record: ./build/pipeline demo-parity --record)\n  {}\n",
            d.changed.len(),
            parts.join("\n  ")
        );
        return 1;
    }
    if !d.fixed.is_empty() {
        let n = d.fixed.len().min(20);
        eprintln!(
            "FAIL  demo-parity ({} recorded cells no longer differ — record the win: ./build/pipeline demo-parity --record && ./build/pipeline ledger --record)\n  {}\n",
            d.fixed.len(),
            d.fixed[..n].join("\n  ")
        );
        return 1;
    }
    println!(
        "PASS  demo-parity ({} pages, {} comparisons, {} cells at the recorded baseline incl. their values; --strict is the end state)",
        pages,
        compared,
        actual.len()
    );
    shell.close();
    0
}
