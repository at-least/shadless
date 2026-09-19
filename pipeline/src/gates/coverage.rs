//! Port of pipeline/gate_coverage.go + gate_coverage_budget.go — the product
//! surface as a matrix (component x path x theme x dir x state), and which
//! gate covers each cell. The UNCOVERED count is budgeted in
//! gates/ledger.json (see ledger.rs).

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

pub const COVERAGE_KEY: &str = "coverage.uncovered-cells";

fn re_state_named() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    // Go \w is ASCII; Rust is Unicode — spell it out.
    R.get_or_init(|| {
        Regex::new(
            r"(^|[\t\n\f\r ]|:|-)(data-(open|closed|checked|unchecked|active|selected|disabled|horizontal|vertical|inset|highlighted|empty|pressed)|aria-(expanded|invalid|checked|disabled|pressed|selected|current)|aria-\[[0-9A-Za-z_-]+=[0-9A-Za-z_-]+\]):",
        )
        .unwrap()
    })
}
fn re_state_data() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"(^|[\t\n\f\r ]|:|-)data-\[([0-9A-Za-z_-]+)(=[0-9A-Za-z_-]+)?\]:").unwrap()
    })
}
fn re_ext_dep_gate() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^external dep ").unwrap())
}

/// The state-token test, kept in the same shape as path-parity's
/// stateConfigs. RE2 has no negative lookahead, so the `data-[...]` arm drops
/// the exclusion and it is applied to the capture instead. The `-` anchor is
/// load-bearing: Tailwind compound variants (`group-data-[...]:`) put a
/// literal `-` directly in front of `data-`/`aria-`.
pub fn has_state_token(all: &str) -> bool {
    if re_state_named().is_match(all) {
        return true;
    }
    for m in re_state_data().captures_iter(all) {
        let name = m.get(2).map(|x| x.as_str()).unwrap_or("");
        let value = m.get(3).map(|x| x.as_str()).unwrap_or("");
        if !value.is_empty() && (name == "slot" || name == "variant" || name == "size") {
            continue;
        }
        return true;
    }
    false
}

#[derive(Deserialize)]
struct TiersEntry {
    #[serde(default)]
    tier: String,
}

#[derive(Deserialize, Default)]
struct IrElement {
    #[serde(default)]
    classes: Vec<String>,
}
#[derive(Deserialize, Default)]
struct IrComponent {
    #[serde(default)]
    elements: Vec<IrElement>,
}
#[derive(Deserialize, Default)]
struct IrCvaEntry {
    #[serde(default)]
    base: String,
    #[serde(default)]
    variants: HashMap<String, HashMap<String, String>>,
}
#[derive(Deserialize, Default)]
struct IrFile {
    #[serde(default)]
    components: Vec<IrComponent>,
    #[serde(default)]
    cva: HashMap<String, IrCvaEntry>,
}

#[derive(Clone)]
struct CovCell {
    component: String,
    path: String,
    theme: String,
    dir: String,
    state: String,
    covered_by: Vec<String>,
    shallow: Vec<String>,
}

pub fn gate_coverage(root: &Path, argv: &[String]) -> Result<(), String> {
    let flag = |f: &str| argv.iter().any(|a| a == f);
    let read = |p: &str| std::fs::read_to_string(root.join(p)).map_err(|e| format!("{}: {}", p, e));

    let tiers_raw = read("src/registry/tiers.json").map_err(|e| format!("FAIL  coverage: {}", e))?;
    let tiers: HashMap<String, TiersEntry> = serde_json::from_str(&tiers_raw)
        .map_err(|e| format!("FAIL  coverage: tiers.json: {}", e))?;

    let mut components: Vec<String> = Vec::new();
    for (n, t) in &tiers {
        if t.tier == "external" || t.tier == "logic" {
            continue;
        }
        if root.join("generated/ir").join(format!("{}.json", n)).exists() {
            components.push(n.clone());
        }
    }
    components.sort();

    // A missing or malformed IR file must fail the gate, not silently
    // downgrade the component to "no css": the uncovered count would not
    // move and the budget would stay green over a matrix that quietly lost
    // real cells.
    let ir_of = |n: &str| -> Result<IrFile, String> {
        let path = root.join("generated/ir").join(format!("{}.json", n));
        let b = std::fs::read_to_string(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
        serde_json::from_str(&b).map_err(|e| format!("{}: {}", path.display(), e))
    };

    let mut contract_defs: HashMap<String, bool> = HashMap::new();
    if let Ok(ents) = std::fs::read_dir(root.join("tools/contracts/components")) {
        for e in ents.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if let Some(stem) = name.strip_suffix(".mjs") {
                let key = stem.strip_suffix("-multiple").unwrap_or(stem);
                contract_defs.insert(key.to_string(), true);
            }
        }
    }

    let oracle_targets: Vec<Value> = std::fs::read_to_string(root.join("docs/example-oracle.json"))
        .ok()
        .and_then(|b| serde_json::from_str(&b).ok())
        .unwrap_or_default();
    let oracle_demo_of = |n: &str| -> bool {
        oracle_targets.iter().any(|t| {
            t.get("name")
                .and_then(|v| v.as_str())
                .map(|name| name == format!("{}-demo", n) || name.starts_with(&format!("{}-", n)))
                .unwrap_or(false)
        })
    };
    let rtl_demo_of = |n: &str| -> bool {
        root.join("docs/demos").join(format!("{}-rtl.html", n)).exists()
    };
    let has_behavior = |n: &str| -> bool {
        tiers.get(n).map(|t| t.tier != "static").unwrap_or(true)
    };

    // noCSS[n]: the component's IR carries zero classes and zero cva entries
    // at all (upstream ships it unstyled) — open is trivially
    // indistinguishable from closed for the css paths.
    let mut state_tokens: HashMap<String, bool> = HashMap::new();
    let mut no_css: HashMap<String, bool> = HashMap::new();
    for n in &components {
        let j = ir_of(n).map_err(|e| format!("FAIL  coverage: {}", e))?;
        let mut parts: Vec<String> = Vec::new();
        for c in &j.components {
            for e in &c.elements {
                parts.extend(e.classes.iter().cloned());
            }
        }
        let mut cva_keys: Vec<&String> = j.cva.keys().collect();
        cva_keys.sort();
        for k in cva_keys {
            let t = &j.cva[k];
            parts.push(t.base.clone());
            let mut v_keys: Vec<&String> = t.variants.keys().collect();
            v_keys.sort();
            for vk in v_keys {
                let vv = &t.variants[vk];
                let mut i_keys: Vec<&String> = vv.keys().collect();
                i_keys.sort();
                for ik in i_keys {
                    parts.push(vv[ik].clone());
                }
            }
        }
        let joined = parts.join(" ");
        if has_state_token(&joined) {
            state_tokens.insert(n.clone(), true);
        }
        if joined.trim().is_empty() {
            no_css.insert(n.clone(), true);
        }
    }

    // oracleExempt[n]: n's canonical example is recorded in exemptions.json
    // with an "external dep" reason.
    let mut oracle_exempt: HashMap<String, bool> = HashMap::new();
    if let Ok(g) = super::ledger::read_golden_exemptions(root) {
        for name in &g.order {
            let reason = match g.reasons.get(name) {
                Some(r) => r,
                None => continue,
            };
            if !re_ext_dep_gate().is_match(reason) {
                continue;
            }
            for n in &components {
                if name == &format!("{}-demo", n) || name.starts_with(&format!("{}-", n)) {
                    oracle_exempt.insert(n.clone(), true);
                }
            }
        }
    }

    // noStateAxis: components whose upstream source has NO CSS keyed on any
    // state attribute — verified by hand against upstream.
    let no_state_axis: &[&str] = &["avatar", "progress"];

    // The known-dead families, read from the sweep gate itself.
    let known_dead: &[&str] = &["message-scroller"];

    let paths = ["demo-inline", "css-import", "full-css"];
    let themes = ["light", "dark"];
    let dirs = ["ltr", "rtl"];

    let mut cells: Vec<CovCell> = Vec::new();
    for c in &components {
        for path in paths {
            for theme in themes {
                for dir in dirs {
                    let states: Vec<&str> = if has_behavior(c) {
                        vec!["closed", "open"]
                    } else {
                        vec!["closed"]
                    };
                    for state in states {
                        let mut by: Vec<String> = Vec::new();
                        let mut shallow: Vec<String> = Vec::new();
                        if path == "demo-inline" {
                            if theme == "light" && dir == "ltr" {
                                if oracle_demo_of(c) {
                                    by.push("example-gate".into());
                                    by.push("golden-gate".into());
                                }
                                if contract_defs.contains_key(c) {
                                    by.push("contracts".into());
                                }
                                if state == "open"
                                    && has_behavior(c)
                                    && !known_dead.contains(&c.as_str())
                                    && oracle_demo_of(c)
                                {
                                    by.push("interactivity-sweep".into());
                                }
                                if state == "closed" {
                                    shallow.push("demo-smoke".into());
                                }
                            }
                            if contract_defs.contains_key(c) {
                                by.push("style-parity".into());
                            }
                            if state == "closed" && oracle_demo_of(c) {
                                by.push("demo-parity".into());
                            }
                        }
                        if path == "demo-inline"
                            && theme == "light"
                            && dir == "rtl"
                            && state == "closed"
                            && rtl_demo_of(c)
                        {
                            shallow.push("docs-smoke".into());
                            shallow.push("css-direction".into());
                        }
                        if path == "demo-inline"
                            && state == "closed"
                            && by.is_empty()
                            && shallow.is_empty()
                            && oracle_exempt.contains_key(c)
                        {
                            shallow.push("example-oracle".into());
                        }
                        // A noCSS component's isolated page carries no
                        // theme/dir-conditional markup at all — the canonical
                        // light/ltr crawl demo-smoke already did IS the check
                        // for the other three.
                        if path == "demo-inline"
                            && state == "closed"
                            && by.is_empty()
                            && shallow.is_empty()
                            && no_css.contains_key(c)
                        {
                            shallow.push("demo-smoke".into());
                        }
                        if path == "css-import" || path == "full-css" {
                            if no_css.contains_key(c) {
                                // path-parity SKIPS a component with no
                                // dist/css/<name>.css — crediting it here
                                // claimed an assertion that provably never
                                // ran. Recorded as shallow with its reason.
                                shallow.push("no-stylesheet".into());
                            } else if state == "closed"
                                || state_tokens.contains_key(c)
                                || no_state_axis.contains(&c.as_str())
                            {
                                by.push("path-parity".into());
                            }
                        }
                        cells.push(CovCell {
                            component: c.clone(),
                            path: path.to_string(),
                            theme: theme.to_string(),
                            dir: dir.to_string(),
                            state: state.to_string(),
                            covered_by: by,
                            shallow,
                        });
                    }
                }
            }
        }
    }

    let mut covered: Vec<&CovCell> = Vec::new();
    let mut shallow_only: Vec<&CovCell> = Vec::new();
    let mut uncovered: Vec<&CovCell> = Vec::new();
    for x in &cells {
        if !x.covered_by.is_empty() {
            covered.push(x);
        } else if !x.shallow.is_empty() {
            shallow_only.push(x);
        } else {
            uncovered.push(x);
        }
    }
    let by_dim = |get: &dyn Fn(&CovCell) -> &str| -> String {
        let mut m: HashMap<String, i64> = HashMap::new();
        let mut order: Vec<String> = Vec::new();
        for x in &uncovered {
            let k = get(x).to_string();
            if !m.contains_key(&k) {
                order.push(k.clone());
            }
            *m.entry(k).or_insert(0) += 1;
        }
        // sort.SliceStable by count desc — stable on insertion order
        order.sort_by(|a, b| m[b].cmp(&m[a]));
        order
            .iter()
            .map(|k| format!("{}={}", k, m[k]))
            .collect::<Vec<_>>()
            .join("  ")
    };

    std::fs::create_dir_all(root.join("build/gates")).map_err(|e| e.to_string())?;
    // Go json.MarshalIndent(report, "", " ") — struct field order. Built as a
    // jsonorder tree and rendered with step " ": for these plain
    // string/number values that is byte-identical to MarshalIndent.
    // Values here are component/gate names — they never carry < > &, so the
    // jsonorder (JSON.stringify) escaper and Go's MarshalIndent HTML escaper
    // produce identical bytes for them.
    use crate::jsonorder::{Json, JsonObj};
    let cells_json: Vec<Json> = cells
        .iter()
        .map(|x| {
            Json::Obj(
                JsonObj::new()
                    .add("component", Json::Str(x.component.clone()))
                    .add("path", Json::Str(x.path.clone()))
                    .add("theme", Json::Str(x.theme.clone()))
                    .add("dir", Json::Str(x.dir.clone()))
                    .add("state", Json::Str(x.state.clone()))
                    .add(
                        "covered_by",
                        Json::Arr(x.covered_by.iter().map(|v| Json::Str(v.clone())).collect()),
                    )
                    .add(
                        "shallow",
                        Json::Arr(x.shallow.iter().map(|v| Json::Str(v.clone())).collect()),
                    )
                    .into_pairs(),
            )
        })
        .collect();
    let report = crate::jsonorder::marshal_js_step(
        &Json::Obj(
            JsonObj::new()
                .add("total", Json::Int(cells.len() as i64))
                .add("covered", Json::Int(covered.len() as i64))
                .add("shallow", Json::Int(shallow_only.len() as i64))
                .add("uncovered", Json::Int(uncovered.len() as i64))
                .add("cells", Json::Arr(cells_json))
                .into_pairs(),
        ),
        "",
        " ",
    );
    std::fs::write(root.join("build/gates/coverage.json"), report)
        .map_err(|e| e.to_string())?;

    if flag("--cells") {
        for x in &uncovered {
            println!(
                "{} {} {} {} {}",
                x.component, x.path, x.theme, x.dir, x.state
            );
        }
    }
    println!(
        "coverage: {} cells over {} components — {} covered (computed/behavioral), {} shallow (presence only), {} UNCOVERED",
        cells.len(),
        components.len(),
        covered.len(),
        shallow_only.len(),
        uncovered.len()
    );
    println!("  uncovered by path:  {}", by_dim(&|x: &CovCell| &x.path));
    println!("  uncovered by theme: {}", by_dim(&|x: &CovCell| &x.theme));
    println!("  uncovered by dir:   {}", by_dim(&|x: &CovCell| &x.dir));
    println!("  uncovered by state: {}", by_dim(&|x: &CovCell| &x.state));
    println!("  detail: build/gates/coverage.json  (--cells lists them)");

    coverage_budget(
        root,
        uncovered.len(),
        covered.len(),
        flag("--record"),
        flag("--check"),
    )
}


/// gate_coverage_budget.go — the ratchet, both ways.
fn coverage_budget(
    root: &Path,
    uncovered: usize,
    covered: usize,
    record: bool,
    check: bool,
) -> Result<(), String> {
    if !record && !check {
        return Ok(());
    }
    let mut l = super::ledger::read_ledger(root).map_err(|e| format!("FAIL  coverage: {}", e))?;

    if record {
        if !l.budgets.contains_key(COVERAGE_KEY) {
            l.budget_order.push(COVERAGE_KEY.to_string());
        }
        l.budgets.insert(
            COVERAGE_KEY.to_string(),
            super::ledger::LedgerBudget {
                max: uncovered as i64,
                target: 0,
                class: "debt".to_string(),
                reason: "cells of the product matrix (component x path x theme x dir x state) no gate makes a computed-style or behavioral assertion about; see `./build/pipeline coverage`".to_string(),
            },
        );
        l.write(root)?;
        println!("coverage: budget {} recorded = {}", COVERAGE_KEY, uncovered);
        return Ok(());
    }

    let budget = l
        .budgets
        .get(COVERAGE_KEY)
        .ok_or_else(|| {
            format!(
                "FAIL  coverage: no budget {} in {} — run ./build/pipeline coverage --record",
                COVERAGE_KEY,
                super::LEDGER_PATH
            )
        })?;
    if uncovered as i64 > budget.max {
        return Err(format!(
            "FAIL  coverage: {} uncovered cells > budget {} — a gate or a contract def was lost, or a new component landed unverified",
            uncovered, budget.max
        ));
    }
    if (uncovered as i64) < budget.max {
        return Err(format!(
            "FAIL  coverage: {} uncovered cells < budget {} — coverage improved; record it: ./build/pipeline coverage --record",
            uncovered, budget.max
        ));
    }
    println!(
        "PASS  coverage ({} uncovered cells, at budget; {} covered)",
        uncovered, covered
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real_root() -> std::path::PathBuf {
        match std::env::var("SHADLESS_ROOT") {
            Ok(r) => std::path::PathBuf::from(r),
            Err(_) => {
                let m = crate::crate_adjacent_tree_root()
                    .unwrap_or(Path::new(env!("CARGO_MANIFEST_DIR")).join(".."));
                m
            }
        }
    }

    /// Go TestCoverage: gate(t, func(root) { return gateCoverage(root, []string{"--check"}) }).
    #[test]
    fn coverage_on_real_tree() {
        let root = real_root();
        if !root.join("generated/ir").exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: no shadless tree)");
            }
            eprintln!("skip: no shadless tree");
            return;
        }
        gate_coverage(&root, &["--check".to_string()]).expect("coverage gate must pass");
    }

    /// Go TestUnitHasStateToken.
    #[test]
    fn unit_has_state_token() {
        let t = |parts: &[&str]| parts.concat();
        let true_cases = [
            t(&["data-[state=open]:bg", "-primary"]),
            t(&["aria-expanded:rotate", "-180"]),
            t(&["data-open:opacity", "-100"]),
            t(&["aria-[foo=bar]:opacity", "-50"]),
            t(&["group-data-[disabled=true]:opacity", "-50"]),
            t(&["hover:data-[state=open]:opacity", "-50"]),
            t(&["data-[slot]:opacity", "-50"]),
            t(&["data-[slot=trigger]:x ", "data-[state=open]:opacity", "-50"]),
        ];
        for tok in &true_cases {
            assert!(has_state_token(tok), "{:?} should carry a state token", tok);
        }
        let false_cases = [
            t(&["data-[slot=trigger]:opacity", "-50"]),
            t(&["data-[variant=outline]:opacity", "-50"]),
            t(&["data-[size=lg]:opacity", "-50"]),
            t(&["xdata-[state=open]:opacity", "-50"]),
            "data-[state=open]".to_string(),
            t(&["bg", "-primary", " ", "text", "-sm"]),
        ];
        for tok in &false_cases {
            assert!(!has_state_token(tok), "{:?} should NOT carry a state token", tok);
        }
    }
}
