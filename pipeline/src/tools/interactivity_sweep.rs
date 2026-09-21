//! Port of pipeline/interactivity_sweep.go — every demo page that OFFERS an
//! interaction must RESPOND to one. Born from the dead-button bug: kernel
//! examples shipped as static oracle snapshots and nobody's gate asked "does
//! the page respond".

use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use std::sync::LazyLock;

static RE_SWEEP_CAND: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
            r#"data-slot="[^"]*-trigger"|aria-expanded=|role="(switch|checkbox|tab)"|data-slot="(carousel-next|carousel-prev)""#,
        )
        .unwrap()
});
static RE_SWEEP_FAM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
            r"^(alert-dialog|navigation-menu|context-menu|dropdown-menu|hover-card|button-group|message-scroller|input-group|native-select|radio-group|scroll-area|toggle-group|carousel|accordion|attachment|avatar|breadcrumb|bubble|collapsible|checkbox|combobox|dialog|drawer|field|input|item|kbd|label|marker|menubar|message|pagination|popover|progress|select|sheet|slider|switch|table|tabs|toggle|tooltip)-",
        )
        .unwrap()
});
static RE_SWEEP_RTL_FAM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-rtl(-|$).*").unwrap());

// static oracle snapshots pending family migration — keyed by family
pub const SWEEP_KNOWN_DEAD: [&str; 1] = ["message-scroller"];

const SWEEP_CANDIDATES_SEL: &str = r#"[data-slot$="-trigger"], [aria-expanded], [role="switch"], [role="checkbox"], [role="tab"], [data-slot="carousel-next"], [data-slot="carousel-prev"]"#;

// per ELEMENT (index), not a sorted multiset: switching tabs keeps the set
// {active, inactive, inactive} identical and read as "nothing responded"
const SWEEP_FINGERPRINT: &str = r#"JSON.stringify({
  states: [...document.querySelectorAll("[data-state]")].map((e, i) => i + ":" + e.getAttribute("data-slot") + ":" + e.getAttribute("data-state")),
  expanded: [...document.querySelectorAll("[aria-expanded]")].map(e => (e.id || e.getAttribute("data-slot")) + ":" + e.getAttribute("aria-expanded")).sort(),
  checked: [...document.querySelectorAll("[aria-checked]")].map(e => (e.id || e.getAttribute("data-slot")) + ":" + e.getAttribute("aria-checked")).sort(),
  disabled: [...document.querySelectorAll("button, input")].map((e, i) => i + ":" + e.disabled),
  kids: document.body.children.length,
})"#;

fn sweep_family_of(name: &str) -> String {
    if let Some(m) = RE_SWEEP_FAM.captures(name) {
        return m[1].to_string();
    }
    RE_SWEEP_RTL_FAM.replace_all(name, "").into_owned()
}

#[derive(Deserialize, Default)]
struct TierEntry {
    #[serde(default)]
    tier: String,
}

pub fn run_interactivity_sweep(root: &Path) -> i32 {
    let site = "docs/site/static/demos";
    let tiers_b = match std::fs::read_to_string(root.join("src/registry/tiers.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("interactivity-sweep: {}", e);
            return 1;
        }
    };
    let tiers: std::collections::HashMap<String, TierEntry> =
        serde_json::from_str(&tiers_b).unwrap_or_default();
    let mut static_families: Vec<String> = tiers
        .iter()
        .filter(|(_, t)| t.tier == "static")
        .map(|(c, _)| c.clone())
        .collect();
    static_families.sort();
    let hover_families: [&str; 2] = ["tooltip", "hover-card"];

    let ents = match std::fs::read_dir(root.join(site)) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("interactivity-sweep: {}", e);
            return 1;
        }
    };
    let mut pages: Vec<String> = Vec::new();
    let mut candidates: Vec<String> = Vec::new();
    for e in ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if !n.ends_with(".html") || crate::emit::build_rtl::rtl_page_lang(&n).is_some() {
            continue;
        }
        pages.push(n.clone());
        if let Ok(b) = std::fs::read_to_string(root.join(site).join(&n)) {
            if RE_SWEEP_CAND.is_match(&b) {
                candidates.push(n);
            }
        }
    }

    let shell = match crate::oracle::browser_shell::BrowserShell::start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("interactivity-sweep: {}", e);
            return 1;
        }
    };
    if let Err(e) = shell.launch() {
        eprintln!("interactivity-sweep: {}", e);
        return 1;
    }
    let page = match shell.new_page(false) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("interactivity-sweep: {}", e);
            return 1;
        }
    };

    let mut verified = 0usize;
    let mut static_pages = pages.len() - candidates.len();
    let mut failures: Vec<String> = Vec::new();
    let mut dead_count: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    // usable-candidate predicate (runs per element with the static families)
    let usable_expr = r#"(e, statics) => !e.disabled && e.getAttribute("aria-disabled") !== "true" && !e.closest("[hidden]") && e.getClientRects().length > 0 &&
        !(e.getAttribute("role") === "tab" && e.getAttribute("data-state") === "active") &&
        !statics.some((s) => (e.getAttribute("data-slot") || "") === s + "-trigger")"#;
    let own_fam_expr =
        r#"(e) => (e.getAttribute("data-slot") || "").replace(/-(trigger|item|next|prev)$/, "")"#;
    let hover_el_expr =
        r#"(e) => /^(tooltip|hover-card)-trigger$/.test(e.getAttribute("data-slot") || "")"#;
    let ctx_trg_expr = r#"(e) => e.getAttribute("data-slot") === "context-menu-trigger""#;

    for f in &candidates {
        let name = f.trim_end_matches(".html");
        let fam = sweep_family_of(name);
        if SWEEP_KNOWN_DEAD.contains(&fam.as_str()) {
            *dead_count.entry(fam).or_insert(0) += 1;
            continue;
        }
        let abs = root.join(site).join(f);
        if let Err(e) = page.goto_url(&format!("file://{}", abs.to_string_lossy())) {
            failures.push(format!("{}: {}", name, first_line(&e)));
            continue;
        }
        let _ = page.wait_for_timeout(350);
        // evaluate candidates in one pass: usable flags via per-element fn
        let mut n_cand = 0usize;
        if let Ok(v) = page.evaluate(&format!(
            "document.querySelectorAll(`{}`).length",
            SWEEP_CANDIDATES_SEL
        )) {
            n_cand = v.as_i64().unwrap_or(0) as usize;
        }
        // per-element predicates, one locEvalAll each (index-stable)
        let usable_v = match page.loc_eval_all_arg(
            "",
            SWEEP_CANDIDATES_SEL,
            usable_expr,
            serde_json::json!(static_families),
        ) {
            Ok(v) => v,
            Err(e) => {
                failures.push(format!("{}: {}", name, first_line(&e)));
                continue;
            }
        };
        let own_v = page
            .loc_eval_all("", SWEEP_CANDIDATES_SEL, own_fam_expr)
            .unwrap_or(serde_json::Value::Null);
        let hover_v = page
            .loc_eval_all("", SWEEP_CANDIDATES_SEL, hover_el_expr)
            .unwrap_or(serde_json::Value::Null);
        let ctx_v = page
            .loc_eval_all("", SWEEP_CANDIDATES_SEL, ctx_trg_expr)
            .unwrap_or(serde_json::Value::Null);
        let mut usable_idx: Vec<usize> = Vec::new();
        for i in 0..n_cand {
            if let Some(arr) = usable_v.as_array() {
                if i < arr.len() && arr[i] == serde_json::Value::Bool(true) {
                    usable_idx.push(i);
                }
            }
            if usable_idx.len() == 3 {
                break;
            }
        }
        if usable_idx.is_empty() {
            static_pages += 1;
            continue;
        }
        // the element's OWN family decides deadness
        let mut own_fam = String::new();
        if let Some(arr) = own_v.as_array() {
            if usable_idx[0] < arr.len() {
                own_fam = arr[usable_idx[0]].as_str().unwrap_or("").to_string();
            }
        }
        if !own_fam.is_empty() && SWEEP_KNOWN_DEAD.contains(&own_fam.as_str()) && own_fam != fam {
            *dead_count.entry(own_fam).or_insert(0) += 1;
            continue;
        }
        let mut responded = false;
        for idx in &usable_idx {
            let before_v = page
                .evaluate(SWEEP_FINGERPRINT)
                .unwrap_or(serde_json::Value::Null);
            let before = before_v.as_str().unwrap_or("").to_string();
            let mut hover_el = false;
            if let Some(arr) = hover_v.as_array() {
                if *idx < arr.len() {
                    hover_el = arr[*idx] == serde_json::Value::Bool(true);
                }
            }
            let mut is_ctx = false;
            if let Some(arr) = ctx_v.as_array() {
                if *idx < arr.len() {
                    is_ctx = arr[*idx] == serde_json::Value::Bool(true);
                }
            }
            if hover_families.contains(&fam.as_str()) || hover_el {
                if let Ok(Some(b)) = page.loc_box("", SWEEP_CANDIDATES_SEL, *idx as i64) {
                    let _ = page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 5);
                }
            } else if fam == "context-menu" || is_ctx {
                let _ = page.loc_click("", SWEEP_CANDIDATES_SEL, *idx as i64, "right");
            } else {
                let _ = page.loc_click("", SWEEP_CANDIDATES_SEL, *idx as i64, "left");
            }
            // hover families open after radix's delay (tooltip/hover-card
            // 700ms provider default) — wait past it
            if hover_families.contains(&fam.as_str()) || hover_el {
                let _ = page.wait_for_timeout(1100);
            } else {
                let _ = page.wait_for_timeout(600);
            }
            let after_v = page
                .evaluate(SWEEP_FINGERPRINT)
                .unwrap_or(serde_json::Value::Null);
            let after = after_v.as_str().unwrap_or("").to_string();
            if before != after {
                responded = true;
                break;
            }
        }
        if !responded {
            failures.push(format!(
                "{}: interaction offered but nothing responded ({})",
                name, fam
            ));
        } else {
            verified += 1;
        }
    }
    let dead_total: usize = dead_count.values().sum();
    if !failures.is_empty() {
        let n = failures.len().min(12);
        eprint!(
            "FAIL  interactivity-sweep\n  {}",
            failures[..n].join("\n  ")
        );
        if failures.len() > 12 {
            eprint!("\n  … +{} more", failures.len() - 12);
        }
        eprintln!();
        return 1;
    }
    println!(
        "PASS  interactivity-sweep ({} pages responded, {} static-by-design, {} known-dead across {} families pending migration — see EXEMPTIONS)",
        verified,
        static_pages,
        dead_total,
        dead_count.len()
    );
    shell.close();
    0
}

fn first_line(s: &str) -> String {
    match s.find('\n') {
        Some(i) => s[..i].to_string(),
        None => s.to_string(),
    }
}
