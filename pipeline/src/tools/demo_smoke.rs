//! Port of pipeline/demo_smoke.go — every dist/components/*.html renders with
//! zero real console errors, every data-slot in the page source is a shipped
//! IR slot name, and a page whose IR has slots renders at least one.

use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use std::sync::LazyLock;

static RE_SLOT_ATTR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"data-slot="([0-9A-Za-z_-]+)""#).unwrap());

#[derive(Deserialize, Default)]
struct TierEntry {
    #[serde(default)]
    tier: String,
    #[serde(default)]
    emit: bool,
}

pub fn run_demo_smoke(root: &Path) -> i32 {
    let tiers_b = match std::fs::read_to_string(root.join("src/registry/tiers.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("demo-smoke: {}", e);
            return 1;
        }
    };
    let reg_tiers: std::collections::HashMap<String, TierEntry> =
        match serde_json::from_str(&tiers_b) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("demo-smoke: tiers: {}", e);
                return 1;
            }
        };
    let shipped = |name: &str, tier: &str| -> bool {
        tier == "static"
            || tier == "kernel"
            || tier == "trivial-js"
            || reg_tiers.get(name).map(|t| t.emit).unwrap_or(false)
    };

    let ents = match std::fs::read_dir(root.join("dist/components")) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("demo-smoke: {}", e);
            return 1;
        }
    };
    let mut all_html: Vec<String> = Vec::new();
    for e in ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if n.ends_with(".html") {
            all_html.push(n);
        }
    }
    all_html.sort();
    let mut pages: Vec<String> = Vec::new();
    for f in &all_html {
        if crate::emit::build_rtl::rtl_page_lang(f).is_none() {
            pages.push(f.clone());
        }
    }
    let rtl_variants = all_html.len() - pages.len();
    let mut emitted = 0;
    for (n, t) in &reg_tiers {
        if shipped(n, &t.tier) {
            emitted += 1;
        }
    }
    if pages.len() != emitted + 1 {
        eprintln!(
            "FAIL demo-smoke: expected {} base pages ({} IR + alert-demo), got {} ({} RTL variants skipped)",
            emitted + 1,
            emitted,
            pages.len(),
            rtl_variants
        );
        return 1;
    }

    // global slot vocabulary across all emitted components, plus each IR
    // file's own slot count — collected in the same pass
    let mut all_slots: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut slot_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let ir_ents = match std::fs::read_dir(root.join("generated/ir")) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("demo-smoke: generated/ir: {}", e);
            return 1;
        }
    };
    for e in ir_ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if !n.ends_with(".json") {
            continue;
        }
        let ir = match crate::emit::css::read_ir(&root.join("generated/ir").join(&n)) {
            Ok(ir) => ir,
            Err(e) => {
                eprintln!("demo-smoke: {}", e);
                return 1;
            }
        };
        let mut count = 0;
        for c in &ir.components {
            for el in &c.elements {
                if !el.slot.is_empty() {
                    count += 1;
                }
            }
        }
        slot_counts.insert(n.trim_end_matches(".json").to_string(), count);
        if !shipped(&ir.name, &ir.tier) {
            continue;
        }
        for c in &ir.components {
            for el in &c.elements {
                if !el.slot.is_empty() {
                    all_slots.insert(el.slot.clone());
                }
            }
        }
    }

    let shell = match crate::oracle::browser_shell::BrowserShell::start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("demo-smoke: {}", e);
            return 1;
        }
    };
    if let Err(e) = shell.launch() {
        eprintln!("demo-smoke: {}", e);
        return 1;
    }

    let mut fail = false;
    for f in &pages {
        let name = f.trim_end_matches(".html");
        let html =
            std::fs::read_to_string(root.join("dist/components").join(f)).unwrap_or_default();
        let mut phantom: Vec<String> = Vec::new();
        for m in RE_SLOT_ATTR.captures_iter(&html) {
            let slot = m[1].to_string();
            if !all_slots.contains(&slot) && !phantom.contains(&slot) {
                phantom.push(slot);
            }
        }
        let n_ir_slots = slot_counts.get(name).copied().unwrap_or(0);
        let page = match shell.new_page(true) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("demo-smoke: {}", e);
                return 1;
            }
        };
        let url = format!(
            "file://{}",
            root.join("dist/components").join(f).to_string_lossy()
        );
        if let Err(e) = page.goto_url(&url) {
            eprintln!("demo-smoke: {}", e);
            return 1;
        }
        let slots_v = match page.evaluate(r#"document.querySelectorAll("[data-slot]").length"#) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("demo-smoke: {}", e);
                return 1;
            }
        };
        let errors = page.events().unwrap_or_default();
        page.close();
        let slots = slots_v.as_i64().unwrap_or(0) as usize;
        if !phantom.is_empty() || !errors.is_empty() || (slots == 0 && n_ir_slots > 0) {
            eprintln!(
                "FAIL demo-smoke [{}]: phantom={} slots={} irSlots={} errors={:?}",
                f,
                phantom.join(","),
                slots,
                n_ir_slots,
                errors
            );
            fail = true;
        }
    }
    shell.close();
    if fail {
        println!("FAIL  demo smoke");
        return 1;
    }
    println!(
        "PASS  demo smoke ({} base pages + {} RTL variants, IR-slot fidelity, 0 console errors)",
        pages.len(),
        rtl_variants
    );
    0
}
