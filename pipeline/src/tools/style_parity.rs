//! Port of pipeline/style_parity.go — COMPUTED STYLE parity between the React
//! oracle and the shadless fixture, per contract component. Pair by data-slot
//! sequence, compare getComputedStyle over a property list; deterministic,
//! computed values rounded to 2dp.

use serde::Deserialize;
use std::path::Path;

use super::parity_baseline::{
    cell_map, diff_parity_baseline, load_parity_baseline, parity_cell, parity_norm_value,
    show_cell, show_change, write_parity_baseline,
};

const STYLE_PARITY_DIR: &str = "tools/contracts/out";
const STYLE_PARITY_BASELINE: &str = "gates/style-parity-baseline.json";

const STYLE_PARITY_PROPS: [&str; 36] = [
    "color", "background-color", "border-color", "border-top-width", "border-radius",
    "display", "position", "flex-direction", "align-items", "justify-content",
    "gap", "padding-top", "padding-right", "padding-bottom", "padding-left",
    "margin-top", "margin-right", "margin-bottom", "margin-left",
    "width", "height", "min-width", "max-width", "font-size", "font-weight",
    "line-height", "text-align", "opacity", "z-index", "overflow", "visibility",
    "box-shadow", "transform", "inset", "flex-wrap", "grid-template-columns",
];

// runtime-measured / animation noise both sides carry differently
const STYLE_PARITY_SKIP: [&str; 4] = ["transform", "transition", "transition-duration", "animation"];

// Measurements were being taken MID-ANIMATION: freeze both sides first.
const STYLE_PARITY_FREEZE: &str = r#"*, *::before, *::after {
  transition: none !important; animation: none !important;
  animation-duration: 0s !important; transition-duration: 0s !important;
}"#;

// Harness shell, identical on both sides (neither convention is the component).
const STYLE_PARITY_SHELL: &str = "padding:0;margin:0;color:var(--foreground);background:var(--background)";

const STYLE_PARITY_COLLECT: &str = r#"(props) => {
  const out = []
  const walk = (el) => {
    if (el.tagName === "SCRIPT" || el.tagName === "TEMPLATE") return
    if (el.hasAttribute("data-slot")) {
      const cs = getComputedStyle(el)
      const style = {}
      for (const p of props) style[p] = cs.getPropertyValue(p)
      out.push({ slot: el.getAttribute("data-slot"), tag: el.tagName, style })
    }
    for (const c of el.children) walk(c)
  }
  for (const root of document.body.children) walk(root)
  return out
}"#;

#[derive(Deserialize, Default)]
struct SpDef {
    #[serde(default)]
    open: String,
    #[serde(rename = "openShadless", default)]
    open_shadless: String,
    #[serde(rename = "styleIgnore", default)]
    style_ignore: Vec<String>,
}

struct SpCellEnt {
    component: String,
    key: String,
    prop: String,
    oracle: String,
    shadless: String,
}

pub fn run_style_parity(root: &Path, strict: bool, record: bool) -> i32 {
    let shell = match crate::oracle::browser_shell::BrowserShell::start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("style-parity: {}", e);
            return 1;
        }
    };
    if let Err(e) = shell.launch() {
        eprintln!("style-parity: {}", e);
        return 1;
    }
    let page = match shell.new_page(false) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("style-parity: {}", e);
            return 1;
        }
    };
    let _ = page.route_abort_external();

    let mut cells: Vec<SpCellEnt> = Vec::new();
    let mut harness_errors: Vec<String> = Vec::new();

    let mut names: Vec<String> = Vec::new();
    if let Ok(ents) = std::fs::read_dir(root.join("tools/contracts/components")) {
        for e in ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if let Some(stem) = n.strip_suffix(".mjs") {
                names.push(stem.to_string());
            }
        }
    }
    names.sort();

    let (mut compared, mut components) = (0usize, 0usize);
    let variants: [(&str, bool, &str); 4] = [
        ("light@ltr", false, "ltr"),
        ("dark@ltr", true, "ltr"),
        ("light@rtl", false, "rtl"),
        ("dark@rtl", true, "rtl"),
    ];

    let collect_matrix = || -> std::collections::HashMap<String, Vec<serde_json::Value>> {
        let mut out: std::collections::HashMap<String, Vec<serde_json::Value>> =
            std::collections::HashMap::new();
        for v in variants {
            let _ = page.evaluate_fn_arg(
                r#"(v) => {
        document.documentElement.classList.toggle("dark", v.dark)
        document.documentElement.setAttribute("dir", v.dir)
      }"#,
                serde_json::json!({"dark": v.1, "dir": v.2}),
            );
            page.wait_for_timeout(60);
            let Ok(res) = page.evaluate_fn_arg(STYLE_PARITY_COLLECT, serde_json::json!(STYLE_PARITY_PROPS.to_vec())) else {
                return out;
            };
            if let Some(arr) = res.as_array() {
                out.insert(v.0.to_string(), arr.clone());
            }
        }
        let _ = page.evaluate(
            r#"(function(){ document.documentElement.classList.remove("dark"); document.documentElement.setAttribute("dir","ltr") })()"#,
        );
        out
    };
    let freeze = || {
        let _ = page.evaluate_fn_arg(
            r#"(shell) => { document.body.style.cssText += ";" + shell }"#,
            serde_json::json!(STYLE_PARITY_SHELL),
        );
        let _ = page.add_style_tag(STYLE_PARITY_FREEZE);
        let _ = page.evaluate(
            r#"(function(){ document.getAnimations?.().forEach((a) => a.finish()) })()"#,
        );
        page.wait_for_timeout(120);
    };

    for name in &names {
        let dir = root.join(STYLE_PARITY_DIR).join(name);
        if !dir.join("oracle.html").exists() || !dir.join("shadless.html").exists() {
            continue;
        }
        let def_res = match shell.call(&serde_json::json!({
            "op": "loadContractDef",
            "file": format!(
                "file://{}",
                root.join("tools/contracts/components")
                    .join(format!("{}.mjs", name))
                    .to_string_lossy()
            ),
        })) {
            Ok(r) => r,
            Err(e) => {
                harness_errors.push(format!("{}: harness error — {}", name, first_line(&e)));
                continue;
            }
        };
        let mut def_v = def_res.get("def").cloned().unwrap_or(serde_json::Value::Null);
        crate::emit::css::drop_nulls(&mut def_v);
        let def: SpDef = match serde_json::from_value(def_v) {
            Ok(d) => d,
            Err(e) => {
                harness_errors.push(format!("{}: harness error — {}", name, first_line(&e.to_string())));
                continue;
            }
        };

        let mut style_ignore: std::collections::HashSet<String> = std::collections::HashSet::new();
        for p in &def.style_ignore {
            style_ignore.insert(p.clone());
        }
        for p in STYLE_PARITY_SKIP {
            style_ignore.insert(p.to_string());
        }
        let props: Vec<&str> = STYLE_PARITY_PROPS
            .iter()
            .filter(|p| !style_ignore.contains(**p))
            .copied()
            .collect();

        let mut run_one = || -> Result<(), String> {
            // oracle side (styled by upstream's own oracle.css)
            let abs_o = dir.join("oracle.html");
            page.goto_url(&format!("file://{}", abs_o.to_string_lossy()))?;
            page.wait_for_timeout(400);
            if !def.open.is_empty() {
                page.driver(&def.open)?;
                page.wait_for_timeout(400);
            }
            page.add_style_tag_path(&root.join("build/gates/oracle.css").to_string_lossy())?;
            let _ = page.evaluate(r#"document.documentElement.classList.add("style-nova")"#);
            freeze();
            let oracle_sides = collect_matrix();

            // shadless side (loads its own out.css via relative link)
            let abs_s = dir.join("shadless.html");
            page.goto_url(&format!("file://{}", abs_s.to_string_lossy()))?;
            page.add_style_tag_path(&root.join("dist/out.css").to_string_lossy())?;
            page.wait_for_timeout(400);
            let mut open = def.open_shadless.clone();
            if open.is_empty() {
                open = def.open.clone();
            }
            if !open.is_empty() {
                page.driver(&open)?;
                page.wait_for_timeout(400);
            }
            freeze();
            let shadless_sides = collect_matrix();

            let slot_of = |e: &serde_json::Value| -> String {
                e.get("slot")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            };
            for variant in variants {
                let mut seen_a: std::collections::HashMap<String, usize> =
                    std::collections::HashMap::new();
                let mut seen_b: std::collections::HashMap<String, usize> =
                    std::collections::HashMap::new();
                let key_of = |slot: &str, seen: &mut std::collections::HashMap<String, usize>| {
                    let n = seen.entry(slot.to_string()).or_insert(0);
                    *n += 1;
                    format!("{}#{}", slot, n)
                };
                let mut map_a: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
                    std::collections::HashMap::new();
                let mut order_a: Vec<String> = Vec::new();
                for e in oracle_sides.get(variant.0).into_iter().flatten() {
                    let k = key_of(&slot_of(e), &mut seen_a);
                    let mut sm = std::collections::HashMap::new();
                    if let Some(style) = e.get("style").and_then(|v| v.as_object()) {
                        for (p, v) in style {
                            if let Some(s) = v.as_str() {
                                sm.insert(p.clone(), s.to_string());
                            }
                        }
                    }
                    map_a.insert(k.clone(), sm);
                    order_a.push(k);
                }
                let mut map_b: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
                    std::collections::HashMap::new();
                let mut order_b: Vec<String> = Vec::new();
                for e in shadless_sides.get(variant.0).into_iter().flatten() {
                    let k = key_of(&slot_of(e), &mut seen_b);
                    let mut sm = std::collections::HashMap::new();
                    if let Some(style) = e.get("style").and_then(|v| v.as_object()) {
                        for (p, v) in style {
                            if let Some(s) = v.as_str() {
                                sm.insert(p.clone(), s.to_string());
                            }
                        }
                    }
                    map_b.insert(k.clone(), sm);
                    order_b.push(k);
                }
                let mut push = |k: &str, prop: &str, a: &str, b: &str| {
                    let key = if variant.0 != "light@ltr" {
                        format!("{}@{}", k, variant.0)
                    } else {
                        k.to_string()
                    };
                    cells.push(SpCellEnt {
                        component: name.clone(),
                        key,
                        prop: prop.to_string(),
                        oracle: a.to_string(),
                        shadless: b.to_string(),
                    });
                };
                for k in &order_a {
                    let a = &map_a[k];
                    match map_b.get(k) {
                        None => push(k, "<presence>", "present", "missing"),
                        Some(b) => {
                            for p in &props {
                                let va = parity_norm_value(a.get(*p).map(String::as_str).unwrap_or(""), true);
                                let vb = parity_norm_value(b.get(*p).map(String::as_str).unwrap_or(""), true);
                                if va != vb {
                                    push(k, p, &va, &vb);
                                }
                            }
                        }
                    }
                }
                for k in &order_b {
                    if !map_a.contains_key(k) {
                        push(k, "<presence>", "missing", "present");
                    }
                }
                compared += map_a.len();
            }
            components += 1;
            Ok(())
        };
        if let Err(e) = run_one() {
            harness_errors.push(format!("{}: harness error — {}", name, first_line(&e)));
        }
    }

    if !harness_errors.is_empty() {
        eprintln!("FAIL  style-parity (harness)\n  {}\n", harness_errors.join("\n  "));
        return 1;
    }

    // `flaky` from the raw file (record must work pre-value format too)
    let mut flaky: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(fb) = std::fs::read_to_string(root.join(STYLE_PARITY_BASELINE)) {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(default)]
            flaky: Vec<String>,
        }
        if let Ok(raw) = serde_json::from_str::<Raw>(&fb) {
            for f in raw.flaky {
                flaky.insert(f);
            }
        }
    }

    let mut ratcheted: Vec<SpCellEnt> = Vec::new();
    for c in &cells {
        if !flaky.contains(&c.component) {
            ratcheted.push(SpCellEnt {
                component: c.component.clone(),
                key: c.key.clone(),
                prop: c.prop.clone(),
                oracle: c.oracle.clone(),
                shadless: c.shadless.clone(),
            });
        }
    }
    let flaky_cells = cells.len() - ratcheted.len();
    let mut actual_cells: Vec<parity_cell> = Vec::new();
    for c in &ratcheted {
        actual_cells.push(parity_cell {
            id: format!("{}/{}/{}", c.component, c.key, c.prop),
            oracle: c.oracle.clone(),
            shadless: c.shadless.clone(),
        });
    }
    let (actual, order) = cell_map(&actual_cells);

    if record || !root.join(STYLE_PARITY_BASELINE).exists() {
        let fl: Vec<String> = flaky.iter().cloned().collect();
        if let Err(e) = write_parity_baseline(
            root,
            STYLE_PARITY_BASELINE,
            "Cells where the shadless fixture's computed style differs from the React oracle, \
with the two values as recorded. This list may only shrink and the values are pinned; \
see the ledger budget style-parity.dirty-cells.",
            &fl,
            &actual,
        ) {
            eprintln!("style-parity: {}", e);
            return 1;
        }
        let comp_set: std::collections::HashSet<&String> =
            ratcheted.iter().map(|c| &c.component).collect();
        println!(
            "style-parity: baseline recorded ({} cells across {} components, {} flaky components excluded)",
            actual.len(),
            comp_set.len(),
            flaky.len()
        );
        return 0;
    }

    if strict && !cells.is_empty() {
        let n = cells.len().min(10);
        let parts: Vec<String> = cells[..n]
            .iter()
            .map(|c| {
                format!(
                    "{}/{}/{}: oracle={} shadless={}",
                    c.component,
                    c.key,
                    c.prop,
                    trunc60(&c.oracle),
                    trunc60(&c.shadless)
                )
            })
            .collect();
        eprintln!(
            "FAIL  style-parity --strict ({} differing cells)\n  {}\n",
            cells.len(),
            parts.join("\n  ")
        );
        return 1;
    }

    let recorded = match load_parity_baseline(root, STYLE_PARITY_BASELINE) {
        Ok(Some((_, r))) => r,
        Ok(None) => Default::default(),
        Err(e) => {
            eprintln!("style-parity: {}", e);
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
        let tail = if d.appeared.len() > 40 {
            format!("\n  … +{} more", d.appeared.len() - 40)
        } else {
            String::new()
        };
        eprintln!(
            "FAIL  style-parity ({} NEW differing cells vs the React oracle)\n  {}{}\n",
            d.appeared.len(),
            parts.join("\n  "),
            tail
        );
        return 1;
    }
    if !d.changed.is_empty() {
        let n = d.changed.len().min(20);
        let parts: Vec<String> = d.changed[..n].iter().map(show_change).collect();
        let tail = if d.changed.len() > 20 {
            format!("\n  … +{} more", d.changed.len() - 20)
        } else {
            String::new()
        };
        eprintln!(
            "FAIL  style-parity ({} recorded cells still differ from the oracle, but by a DIFFERENT amount than what was recorded — look at them again, then re-record)\n  {}{}\n\n  ./build/pipeline style-parity --record\n",
            d.changed.len(),
            parts.join("\n  "),
            tail
        );
        return 1;
    }
    if !d.fixed.is_empty() {
        let n = d.fixed.len().min(12);
        let tail = if d.fixed.len() > 12 {
            format!("\n  … +{} more", d.fixed.len() - 12)
        } else {
            String::new()
        };
        eprintln!(
            "FAIL  style-parity ({} recorded cells no longer differ — record the win so the slack cannot be re-spent)\n  {}{}\n\n  ./build/pipeline style-parity --record && ./build/pipeline ledger --record\n",
            d.fixed.len(),
            d.fixed[..n].join("\n  "),
            tail
        );
        return 1;
    }
    println!(
        "PASS  style-parity ({} components, {} elements compared, {} cells at the recorded baseline incl. their values, {} cells in {} flaky components excluded; --strict is the end state)",
        components,
        compared,
        actual.len(),
        flaky_cells,
        flaky.len()
    );
    shell.close();
    0
}

fn trunc60(s: &str) -> String {
    if s.len() > 60 {
        format!("{}…", &s[..60])
    } else {
        s.to_string()
    }
}

fn first_line(s: &str) -> String {
    match s.find('\n') {
        Some(i) => s[..i].to_string(),
        None => s.to_string(),
    }
}
