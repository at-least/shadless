//! Port of pipeline/demo.go — unified slot-keyed CSS for all emitted
//! components + a browsable demo page per component in dist/components/.
//! Static pages come from the emitter; kernel pages reuse the verified
//! src/kernel fixtures; trivial-js pages reuse probes/t7; carousel from
//! probes/t8; menubar/navigation-menu from src/kernel; field is inlined.

use super::css::{CssIrComponent, component_css, wrap_component_css};
use super::prepaint::{SHADLESS_CSS_FIXES, inject_pre_paint};
use super::{load_skin, skin_data};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{LazyLock, OnceLock};

/// rewritePaths — ported from tools/demo-lib.mjs. The bare `out.css` form is
/// a SEPARATE replace from the `[^"]*-out\.css` form: the dash is mandatory
/// in the first pattern, so it can never match plain "out.css".
fn rewrite_paths(html: &str) -> String {
    static LINK_COMP: OnceLock<Regex> = OnceLock::new();
    static LINK_BARE: OnceLock<Regex> = OnceLock::new();
    static LINK_DIST: OnceLock<Regex> = OnceLock::new();
    static SCRIPT_BASE: OnceLock<Regex> = OnceLock::new();
    static SCRIPT_COMP: OnceLock<Regex> = OnceLock::new();
    let link_comp =
        LINK_COMP.get_or_init(|| Regex::new(r#"(<link[^>]*href=")[^"]*-out\.css(")"#).unwrap());
    let link_bare =
        LINK_BARE.get_or_init(|| Regex::new(r#"(<link[^>]*href=")out\.css(")"#).unwrap());
    let link_dist = LINK_DIST
        .get_or_init(|| Regex::new(r#"(<link[^>]*href=")\.\./\.\./dist/out\.css(")"#).unwrap());
    let script_base = SCRIPT_BASE.get_or_init(|| {
        Regex::new(r#"(<script[^>]*src=")\.\./\.\./dist/shadless\.js(")"#).unwrap()
    });
    let script_comp = SCRIPT_COMP.get_or_init(|| {
        Regex::new(r#"(<script[^>]*src=")\.\./\.\./dist/js/([\w-]+\.js)(")"#).unwrap()
    });
    let out = link_comp.replace_all(html, "${1}../out.css${2}");
    let out = link_bare.replace_all(&out, "${1}../out.css${2}");
    let out = link_dist.replace_all(&out, "${1}../out.css${2}");
    let out = script_base.replace_all(&out, "${1}../shadless.js${2}");
    let out = script_comp.replace_all(&out, "${1}../js/${2}${3}");
    out.into_owned()
}

static RE_HAS_OUT_CSS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<link[^>]*out\.css"#).unwrap());

/// ensureLink: t7 fixtures ship without a stylesheet link.
fn ensure_link(html: &str) -> String {
    if RE_HAS_OUT_CSS.is_match(html) {
        return html.to_string();
    }
    html.replacen(
        "<head>",
        "<head>\n<link rel=\"stylesheet\" href=\"../out.css\">",
        1,
    )
}

const KERNEL_T6: &[&str] = &[
    "alert-dialog",
    "context-menu",
    "dropdown-menu",
    "hover-card",
    "popover",
    "scroll-area",
    "select",
    "sheet",
    "slider",
    "tabs",
    "tooltip",
];

const TRIVIAL_T7: &[&str] = &[
    "accordion",
    "aspect-ratio",
    "avatar",
    "checkbox",
    "collapsible",
    "label",
    "progress",
    "radio-group",
    "separator",
    "switch",
    "toggle",
    "toggle-group",
];

/// out.css's content scan is EXPLICIT (source(none)); this list mirrors the
/// dist-facing inputs of the `demo-css` node in the authored nodes table
/// (nodes.rs; the Go-verbatim table spells them `dist/**`) — keep them in
/// step, the reproducible gate is what catches drift.
const DEMO_SOURCES: &[&str] = &[
    "./components",
    "./js",
    "../docs/demos",
    "../docs/content",
    "../src/kernel",
    "../tools/contracts/out",
    "../generated/ir",
    "../probes/t7",
    "../probes/t8",
];

fn field_demo_html() -> &'static str {
    r##"<!doctype html>
<html><head><meta charset="utf-8"><title>shadless field</title>
<link rel="stylesheet" href="../out.css"></head>
<body>
  <fieldset data-slot="field-set">
    <legend data-slot="field-legend" data-variant="legend">Login</legend>

    <div data-slot="field-group">
      <div data-slot="field" class="group/field" data-orientation="vertical">
        <div data-slot="field-content" class="group/field-content">
          <label data-slot="field-label" class="group/field-label peer/field-label" for="email">Email</label>
          <input data-slot="input" id="email" type="email" placeholder="m@example.com">
          <p data-slot="field-description">We'll never share your email.</p>
        </div>
      </div>

      <div data-slot="field" class="group/field" data-orientation="vertical" data-invalid="true">
        <div data-slot="field-content" class="group/field-content">
          <label data-slot="field-label" class="group/field-label peer/field-label" for="pw">Password</label>
          <input data-slot="input" id="pw" type="password" aria-invalid="true">
          <div data-slot="field-error">Password must be at least 8 characters.</div>
        </div>
      </div>

      <div data-slot="field-separator" class="group/field-group">
        <div data-slot="separator" class="absolute inset-0 top-1/2"></div>
        <span data-slot="field-separator-content">or</span>
      </div>
    </div>
  </fieldset>
</body></html>"##
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegTier {
    #[serde(default)]
    tier: String,
    #[serde(default)]
    emit: bool,
}

fn contains_tok(ss: &[&str], s: &str) -> bool {
    ss.iter().any(|x| *x == s)
}

pub fn run_demo() -> Result<(), String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let root_s = root.to_string_lossy().into_owned();
    let _ = &root_s;
    load_skin();
    std::fs::create_dir_all(root.join("dist/components")).map_err(|e| format!("demo: {}", e))?;

    // tiers + shipped set (dispatch is tier-based; emit:true covers the
    // exceptions)
    let tiers_b = std::fs::read_to_string(root.join("src/registry/tiers.json"))
        .map_err(|e| format!("demo: {}", e))?;
    let reg_tiers: HashMap<String, RegTier> =
        serde_json::from_str(&tiers_b).map_err(|e| format!("demo: tiers: {}", e))?;
    let shipped_tier = |tier: &str| tier == "static" || tier == "kernel" || tier == "trivial-js";

    let ir_dir = root.join("generated/ir");
    let mut file_order: Vec<String> = Vec::new(); // ReadDir order: sorted filenames
    let mut ir_all: HashMap<String, CssIrComponent> = HashMap::new();
    let mut ents: Vec<String> = Vec::new();
    for e in std::fs::read_dir(&ir_dir).map_err(|e| format!("demo: {}", e))? {
        let e = e.map_err(|e| e.to_string())?;
        ents.push(e.file_name().to_string_lossy().into_owned());
    }
    ents.sort();
    for n in &ents {
        if !n.ends_with(".json") {
            continue;
        }
        let b = std::fs::read_to_string(ir_dir.join(n)).map_err(|e| e.to_string())?;
        let mut v: serde_json::Value =
            serde_json::from_str(&b).map_err(|e| format!("demo: ir: {} {}", n, e))?;
        super::css::drop_nulls(&mut v);
        let ir: CssIrComponent =
            serde_json::from_value(v).map_err(|e| format!("demo: ir: {} {}", n, e))?;
        if shipped_tier(&ir.tier) || reg_tiers.get(&ir.name).map(|t| t.emit).unwrap_or(false) {
            file_order.push(ir.name.clone());
            ir_all.insert(ir.name.clone(), ir);
        }
    }
    let mut names = file_order.clone();
    names.sort();
    // count assertion derives from the same predicate, so the two cannot disagree
    let expected = reg_tiers
        .values()
        .filter(|t| shipped_tier(&t.tier) || t.emit)
        .count();
    if ir_all.len() != expected {
        return Err(format!(
            "expected {} emitted components (static/kernel/trivial-js + tiers.json emit:true), got {}",
            expected,
            ir_all.len()
        ));
    }

    // ---- 1. unified globals.css ----
    let base_b = std::fs::read_to_string(root.join("probes/h4/globals.css"))
        .map_err(|e| format!("demo: {}", e))?;
    let base = base_b.replacen("@source \"./demo.html\";\n", "", 1);
    std::fs::create_dir_all(root.join("dist/css")).map_err(|e| format!("demo: {}", e))?;
    let mut css_parts: Vec<String> = Vec::new();
    let mut css_files: HashMap<String, bool> = HashMap::new();
    for name in &file_order {
        // JS iterated irAll = ReadDir order
        let css = match component_css(&ir_all[name]) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("FAIL css[{}]: {}", name, e);
                return Err(e);
            }
        };
        if css.rules.is_empty() {
            continue;
        }
        let part = wrap_component_css(name, &css);
        std::fs::write(
            root.join(format!("dist/css/{}.css", name)),
            format!("{}\n", part),
        )
        .map_err(|e| format!("demo: {}", e))?;
        css_files.insert(format!("{}.css", name), true);
        css_parts.push(part);
    }
    // a component leaving the emitted set takes its file with it — an orphan
    // keeps shipping through shadless.product.css (the form.json bug class)
    if let Ok(ents) = std::fs::read_dir(root.join("dist/css")) {
        for f in ents.flatten() {
            let name = f.file_name().to_string_lossy().into_owned();
            if name.ends_with(".css") && !css_files.contains_key(&name) {
                let _ = std::fs::remove_file(f.path());
                println!(
                    "demo: removed orphaned dist/css/{} (no longer an emitted component)",
                    name
                );
            }
        }
    }
    let srcs: Vec<String> = DEMO_SOURCES
        .iter()
        .map(|d| format!("@source \"{}\";", d))
        .collect();
    let globals = base.replacen(
        "@import \"tailwindcss\";",
        "@import \"tailwindcss\" source(none);",
        1,
    ) + "\n"
        + &srcs.join("\n")
        + "\n\n"
        + SHADLESS_CSS_FIXES
        + "\n\n"
        + &css_parts.join("\n\n")
        + "\n@layer base { body { @apply bg-background text-foreground p-8; } }\n";
    std::fs::write(root.join("dist/globals.css"), globals).map_err(|e| format!("demo: {}", e))?;

    // ---- 3. per-component demo pages ----
    let mut emitted = 0usize;
    let read_fixture = |p: &str| -> Option<String> { std::fs::read_to_string(root.join(p)).ok() };
    for name in &names {
        let ir = &ir_all[name];
        let html: String;
        match ir.tier.as_str() {
            "static" => {
                if !root.join(format!("dist/components/{}.html", name)).exists() {
                    eprintln!(
                        "demo: static page missing: {} (run the emit step first)",
                        name
                    );
                    return Err("static page missing".to_string());
                }
                emitted += 1;
                continue;
            }
            "kernel" => {
                if name == "dialog" || contains_tok(KERNEL_T6, name) {
                    let Some(src) = read_fixture(&format!("src/kernel/{}.html", name)) else {
                        eprintln!("demo: no kernel fixture for {}", name);
                        return Err("no kernel fixture".to_string());
                    };
                    html = rewrite_paths(&src);
                } else {
                    eprintln!("demo: no kernel fixture for {}", name);
                    return Err("no kernel fixture".to_string());
                }
            }
            "medium" => {
                if name == "menubar" || name == "navigation-menu" {
                    let Some(src) = read_fixture(&format!("src/kernel/{}.html", name)) else {
                        eprintln!("demo: no medium fixture for {}", name);
                        return Err("no medium fixture".to_string());
                    };
                    html = ensure_link(&rewrite_paths(&src));
                } else {
                    eprintln!("demo: no medium fixture for {}", name);
                    return Err("no medium fixture".to_string());
                }
            }
            "trivial-js" => {
                if !contains_tok(TRIVIAL_T7, name) {
                    eprintln!("demo: no trivial fixture for {}", name);
                    return Err("no trivial fixture".to_string());
                }
                let Some(src) = read_fixture(&format!("probes/t7/{}.html", name)) else {
                    eprintln!("demo: no trivial fixture for {}", name);
                    return Err("no trivial fixture".to_string());
                };
                html = ensure_link(&rewrite_paths(&src));
            }
            "logic" => {
                if name == "field" {
                    html = field_demo_html().to_string();
                } else {
                    eprintln!("demo: no presentational fixture for {}", name);
                    return Err("no presentational fixture".to_string());
                }
            }
            "external" => {
                if name == "carousel" {
                    let Some(src) = read_fixture("probes/t8/carousel.html") else {
                        eprintln!("demo: no carousel fixture");
                        return Err("no carousel fixture".to_string());
                    };
                    html = ensure_link(&rewrite_paths(&src));
                } else {
                    eprintln!("demo: no external fixture for {}", name);
                    return Err("no external fixture".to_string());
                }
            }
            other => {
                eprintln!("demo: unhandled tier {}", other);
                return Err("unhandled tier".to_string());
            }
        }
        std::fs::write(
            root.join(format!("dist/components/{}.html", name)),
            inject_pre_paint(&html),
        )
        .map_err(|e| format!("demo: {}", e))?;
        emitted += 1;
    }
    if emitted != expected {
        return Err(format!("emitted {}, expected {}", emitted, expected));
    }

    // ---- 4. demo index ----
    let groups: [(&str, &str); 5] = [
        ("static", "Static (markup + CSS)"),
        ("kernel", "Kernel (base + per-component behavior)"),
        ("trivial-js", "Trivial (shadless runtime)"),
        ("logic", "Presentational logic (markup + CSS)"),
        ("external", "External (vanilla port)"),
    ];
    let mut idx = String::from(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>shadless demo</title>\n<link rel=\"stylesheet\" href=\"out.css\"></head>\n<body>\n<h1>shadless demo</h1>\n",
    );
    for (g_tier, g_label) in &groups {
        let ns: Vec<&String> = names
            .iter()
            .filter(|name| {
                ir_all[*name].tier == *g_tier
                    || (*g_tier == "kernel" && ir_all[*name].tier == "medium")
            })
            .collect();
        idx.push_str(&format!(
            "<h2>{} <small>({})</small></h2><ul>",
            g_label,
            ns.len()
        ));
        for n in &ns {
            idx.push_str(&format!(
                "<li><a href=\"components/{}.html\">{}</a></li>",
                n, n
            ));
        }
        idx.push_str("</ul>\n");
    }
    idx.push_str("</body></html>");
    std::fs::write(root.join("dist/demo-index.html"), idx).map_err(|e| format!("demo: {}", e))?;
    println!("demo: {} pages, globals.css + assets written", emitted);
    let _ = skin_data();
    Ok(())
}
