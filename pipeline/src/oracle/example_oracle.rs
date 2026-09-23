//! Port of pipeline/example_oracle.go — upstream examples/*.tsx → shadless
//! demo HTML, 1:1 by construction. BUILD renders the real React + registry
//! sources in chromium (the oracle), extracts #root.innerHTML as the page
//! body; --check re-renders and byte-diffs the normalized DOM against the
//! emitted page.
//!
//! All-or-nothing: a render failure writes NOTHING (pages or manifests).

use super::browser_shell::BrowserShell;
use super::oracle_lib::{await_oracle, build_oracle, oracle_norm, oracle_root_html};
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::OnceLock;

const ORA_EXAMPLES_DIR: &str = ".upstream/shadcn-ui/apps/v4/examples/radix";
const ORA_MANIFEST: &str = "docs/example-oracle.json";
const ORA_FIX_TARGETS: &str = "docs/example-fixture-targets.json";

fn fixture_family_sel() -> &'static HashMap<&'static str, &'static str> {
    static M: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    M.get_or_init(|| {
        HashMap::from([
            ("alert-dialog", "[data-slot=\"alert-dialog-trigger\"]"),
            ("dialog", "[data-slot=\"dialog-trigger\"]"),
            ("sheet", "[data-slot=\"sheet-trigger\"]"),
            ("popover", "[data-slot=\"popover-trigger\"]"),
            ("tooltip", "[data-slot=\"tooltip-trigger\"]"),
            ("hover-card", "[data-slot=\"hover-card-trigger\"]"),
            ("dropdown-menu", "[data-slot=\"dropdown-menu-trigger\"]"),
            ("context-menu", "[data-slot=\"context-menu-trigger\"]"),
            ("menubar", "[data-slot=\"menubar\"]"),
            ("select", "[data-slot=\"select-trigger\"]"),
            ("tabs", "[data-slot=\"tabs\"]"),
            ("slider", "[data-slot=\"slider\"]"),
            ("scroll-area", "[data-slot=\"scroll-area\"]"),
            ("carousel", "[data-slot=\"carousel\"]"),
            ("navigation-menu", "[data-slot=\"navigation-menu\"]"),
        ])
    })
}

pub struct OraTarget {
    pub name: String,
    pub out: String,
}

#[derive(Deserialize)]
pub(crate) struct TierEntry {
    #[serde(default)]
    pub(crate) tier: String,
}

/// tiers.json read once for the whole run; an unparseable file is fatal —
/// an empty map used to silently make is_kernel_demo always-false and drop
/// every trivial-js script (the vacuous-verdict shape the sweep fixed).
fn tiers_map() -> Result<HashMap<String, TierEntry>, String> {
    let tiers_b = std::fs::read_to_string("src/registry/tiers.json")
        .map_err(|e| format!("tiers: {}", e))?;
    serde_json::from_str(&tiers_b).map_err(|e| format!("tiers: {}", e))
}

#[derive(Deserialize)]
struct Preview {
    #[serde(default)]
    name: String,
    #[serde(default)]
    status: String,
}

#[derive(Deserialize, Default)]
struct Catalog {
    #[serde(default)]
    previews: Vec<Preview>,
}

#[derive(Deserialize)]
struct Owned {
    #[serde(default)]
    name: String,
    #[serde(default)]
    out: String,
}

/// Every input whose corruption used to silently change what this tool does:
/// the catalog and overlays manifest decide what the stale-page sweep may
/// DELETE, the owned manifest decides what --check verifies (an empty either
/// used to mean mass deletion or a vacuous green). All parsed up front,
/// before the browser — and the sweep — runs.
struct OraInputs {
    tiers: HashMap<String, TierEntry>,
    catalog: Catalog,
    overlay_manifest: serde_json::Value,
    owned: Vec<Owned>,
}

fn ora_inputs(check: bool) -> Result<OraInputs, String> {
    let tiers = tiers_map()?;
    let catalog_b = std::fs::read_to_string("docs/catalog.json")
        .map_err(|e| format!("catalog: {}", e))?;
    let catalog: Catalog =
        serde_json::from_str(&catalog_b).map_err(|e| format!("catalog: {}", e))?;
    let om_b = std::fs::read_to_string("overlays/manifest.json")
        .map_err(|e| format!("overlays manifest: {}", e))?;
    let overlay_manifest =
        serde_json::from_str(&om_b).map_err(|e| format!("overlays manifest: {}", e))?;
    let owned = if check {
        let owned_b = std::fs::read_to_string(ORA_MANIFEST)
            .map_err(|e| format!("owned: {}", e))?;
        serde_json::from_str(&owned_b).map_err(|e| format!("owned: {}", e))?
    } else {
        Vec::new()
    };
    Ok(OraInputs {
        tiers,
        catalog,
        overlay_manifest,
        owned,
    })
}

pub(crate) fn ora_load_targets(
    tiers: &HashMap<String, TierEntry>,
    catalog: &Catalog,
) -> (Vec<OraTarget>, Vec<String>) {
    let mut targets: Vec<OraTarget> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    static DEMO_RE: OnceLock<Regex> = OnceLock::new();
    let demo_re = DEMO_RE.get_or_init(|| Regex::new(r"^(.+)-demo$").unwrap());
    let is_kernel_demo = |demo_name: &str| -> bool {
        match demo_re.captures(demo_name) {
            Some(m) => tiers
                .get(&m[1])
                .map(|t| t.tier == "kernel")
                .unwrap_or(false),
            None => false,
        }
    };
    // alert-demo replaces the retired build-demo hand emitter (dist target)
    targets.push(OraTarget {
        name: "alert-demo".to_string(),
        out: "dist/components/alert-demo.html".to_string(),
    });
    let mut seen: HashSet<String> = HashSet::from(["alert-demo".to_string()]);
    for p in &catalog.previews {
        if !root_path(&format!("{}/{}.tsx", ORA_EXAMPLES_DIR, p.name)).exists() {
            skipped.push(p.name.clone());
            continue;
        }
        if p.status != "authored" && p.status != "existing-dist" {
            continue;
        }
        if seen.contains(&p.name) {
            continue;
        }
        if is_kernel_demo(&p.name) {
            skipped.push(p.name.clone());
            continue;
        }
        targets.push(OraTarget {
            name: p.name.clone(),
            out: format!("docs/demos/{}.html", p.name),
        });
        seen.insert(p.name.clone());
    }
    (targets, skipped)
}

fn root_path(rel: &str) -> std::path::PathBuf {
    Path::new(rel).to_path_buf()
}

/// Trivial-tier families become INTERACTIVE by loading the runtime — scripts
/// deferred in <head> so <body> stays byte-equal to the oracle render.
fn ora_scripts_head(trivial: &[String]) -> String {
    if trivial.is_empty() {
        return String::new();
    }
    let mut s = "\n<script defer src=\"../shadless.js\"></script>".to_string();
    for c in trivial {
        s.push_str(&format!("\n<script defer src=\"../js/{}.js\"></script>", c));
    }
    s
}

/// Matches upstream's ComponentPreviewTabs .preview box — docs-site chrome
/// around the example, re-added at the docs-page-body level.
const ORA_BODY_ATTR: &str = "class=\"flex min-h-72 w-full items-center justify-center p-8\"";

/// shadless protocol attributes that React never renders. Exact-anchor,
/// single occurrence, loud on a miss.
fn ora_protocol_patch(name: &str, dom: &str) -> Result<String, String> {
    if name != "accordion-multiple" {
        return Ok(dom.to_string());
    }
    let find = "<div data-slot=\"accordion\" class=\"flex w-full flex-col max-w-lg\" data-orientation=\"vertical\">";
    let repl = "<div data-slot=\"accordion\" data-type=\"multiple\" class=\"flex w-full flex-col max-w-lg\" data-orientation=\"vertical\">";
    let n = dom.matches(find).count();
    if n != 1 {
        return Err(format!(
            "protocol patch {}: anchor found {} times, want 1 — re-anchor after the re-pin",
            name, n
        ));
    }
    Ok(dom.replacen(find, repl, 1))
}

fn ora_page_html(name: &str, body: &str, trivial: &[String]) -> String {
    format!(
        "<!doctype html>\n<html><head><meta charset=\"utf-8\"><title>shadless {}</title>\n<link rel=\"stylesheet\" href=\"../out.css\">{}{}</head>\n<body {}>\n{}\n</body></html>",
        name,
        super::super::emit::prepaint::inject_pre_paint(""),
        ora_scripts_head(trivial),
        ORA_BODY_ATTR,
        body
    )
}

fn first_line(s: &str) -> String {
    s.lines().next().unwrap_or("").to_string()
}

pub fn run_example_oracle(check: bool) -> i32 {
    match run_inner(check) {
        Ok(()) => 0,
        Err(msg) => {
            eprintln!("example-oracle: {}", msg);
            1
        }
    }
}

fn run_inner(check: bool) -> Result<(), String> {
    // NO_ORACLE: examples whose golden exemption reason starts "external dep"
    let mut no_oracle: HashSet<String> = HashSet::new();
    if let Ok(eb) =
        std::fs::read_to_string("src/registry/upstream-snapshot/exemptions.json")
    {
        #[derive(Deserialize)]
        struct ExEntry {
            #[serde(default)]
            reason: String,
        }
        #[derive(Deserialize)]
        struct Exemptions {
            #[serde(default)]
            examples: HashMap<String, ExEntry>,
        }
        if let Ok(ex) = serde_json::from_str::<Exemptions>(&eb) {
            for (name, e) in ex.examples {
                if e.reason.starts_with("external dep ") {
                    no_oracle.insert(name);
                }
            }
        }
    }

    let inputs = ora_inputs(check)?;
    let shell = BrowserShell::start().map_err(|e| format!("example-oracle: {}", e))?;
    let result = run_with_shell(&shell, check, &no_oracle, &inputs);
    shell.close();
    result
}

fn run_with_shell(
    shell: &BrowserShell,
    check: bool,
    no_oracle: &HashSet<String>,
    inputs: &OraInputs,
) -> Result<(), String> {
    shell.launch().map_err(|e| format!("example-oracle: {}", e))?;
    let page =
        shell.new_page(false).map_err(|e| format!("example-oracle: new page: {}", e))?;

    let (targets, skipped) = ora_load_targets(&inputs.tiers, &inputs.catalog);
    if !skipped.is_empty() {
        println!(
            "oracle: {} authored demos have no upstream example (kept hand-authored): {}",
            skipped.len(),
            skipped.join(", ")
        );
    }

    if check {
        let owned = &inputs.owned;
        let mut drift = 0;
        for t in owned {
            let html_file = match build_oracle(Path::new("."), &t.name, Path::new("build/example-oracle")) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("DRIFT [{}]: oracle render failed ({})", t.name, first_line(&e));
                    drift += 1;
                    continue;
                }
            };
            if let Err(e) = await_oracle(&page, &html_file) {
                eprintln!("DRIFT [{}]: oracle render failed ({})", t.name, first_line(&e));
                drift += 1;
                let _ = std::fs::remove_file(&html_file);
                continue;
            }
            let dom = match oracle_root_html(&page) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("DRIFT [{}]: {}", t.name, e);
                    drift += 1;
                    continue;
                }
            };
            let dom = match ora_protocol_patch(&t.name, &dom) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("DRIFT [{}]: {}", t.name, e);
                    drift += 1;
                    continue;
                }
            };
            let _ = std::fs::remove_file(&html_file);
            let current = std::fs::read_to_string(&t.out).unwrap_or_default();
            static BODY_RE: OnceLock<Regex> = OnceLock::new();
            let body_re = BODY_RE.get_or_init(|| Regex::new(r"(?s)<body[^>]*>\n(.*)\n</body>").unwrap());
            let body = body_re
                .captures(&current)
                .map(|m| m[1].to_string())
                .unwrap_or_default();
            if oracle_norm(body.trim()) != dom.trim() {
                eprintln!("DRIFT [{}]: {} != oracle render", t.name, t.out);
                drift += 1;
            }
        }
        if drift > 0 {
            eprintln!("FAIL  example-oracle check ({}/{} drifted)", drift, owned.len());
            return Err("example-oracle check drifted".to_string());
        }
        println!("PASS  example-oracle check ({} pages == oracle render)", owned.len());
        return Ok(());
    }

    // trivial-js components with a behavior file; selector per component
    let mut trivial_js: Vec<String> = Vec::new();
    let rt_ents = std::fs::read_dir("src/runtime/components")
        .map_err(|e| format!("runtime components: {}", e))?;
    for e in rt_ents.flatten() {
        let c = e.file_name().to_string_lossy().into_owned();
        let c = c.trim_end_matches(".js").to_string();
        if inputs.tiers.get(&c).map(|t| t.tier == "trivial-js").unwrap_or(false) {
            trivial_js.push(c);
        }
    }
    let mut trivial_sel: HashMap<String, String> = HashMap::new();
    for c in &trivial_js {
        trivial_sel.insert(c.clone(), format!("[data-slot=\"{}\"]", c));
    }

    #[derive(Deserialize)]
    struct FixTarget {
        name: String,
        families: Vec<String>,
        trivial: Vec<String>,
    }
    let mut fixture_targets: Vec<FixTarget> = Vec::new();
    struct RenderedEnt {
        t: OraTarget,
        dom: String,
        trivial: Vec<String>,
    }
    let mut rendered: Vec<RenderedEnt> = Vec::new();
    let mut failures: Vec<String> = Vec::new();
    let mut exempt: Vec<String> = Vec::new();

    // Phase 1 — render everything, write nothing (all-or-nothing)
    for t in &targets {
        let tsx = format!("{}/{}.tsx", ORA_EXAMPLES_DIR, t.name);
        if !Path::new(&tsx).exists() {
            failures.push(format!("{}: {} missing", t.name, tsx));
            continue;
        }
        let html_file = match build_oracle(Path::new("."), &t.name, Path::new("build/example-oracle")) {
            Ok(h) => h,
            Err(e) => {
                if no_oracle.contains(&t.name) {
                    exempt.push(t.name.clone());
                    continue;
                }
                failures.push(format!("{}: oracle render failed ({})", t.name, first_line(&e)));
                continue;
            }
        };
        if let Err(e) = await_oracle(&page, &html_file) {
            let _ = std::fs::remove_file(&html_file);
            if no_oracle.contains(&t.name) {
                exempt.push(t.name.clone());
                continue;
            }
            failures.push(format!("{}: oracle render failed ({})", t.name, first_line(&e)));
            continue;
        }
        let mut dom = match oracle_root_html(&page) {
            Ok(d) => d,
            Err(e) => {
                let _ = std::fs::remove_file(&html_file);
                failures.push(format!("{}: {}", t.name, first_line(&e)));
                continue;
            }
        };
        dom = match ora_protocol_patch(&t.name, &dom) {
            Ok(d) => d,
            Err(e) => {
                let _ = std::fs::remove_file(&html_file);
                failures.push(format!("{}: {}", t.name, e));
                continue;
            }
        };
        // family + trivial detection: evaluate once with both maps
        let detect = r##"(spec) => {
      const root = document.querySelector("#root")
      const fams = [], triv = []
      for (const [f, sel] of Object.entries(spec.families))
        if (root.querySelector(sel)) fams.push(f)
      for (const [c, sel] of Object.entries(spec.trivial))
        if (root.querySelector(sel)) triv.push(c)
      return { fams, triv }
    }"##;
        let spec = serde_json::json!({
            "families": fixture_family_sel(),
            "trivial": trivial_sel,
        });
        let res = page.evaluate_fn_arg(detect, spec);
        let _ = std::fs::remove_file(&html_file);
        let Ok(res) = res else {
            failures.push(format!("{}: detection failed", t.name));
            continue;
        };
        let mut families: Vec<String> = Vec::new();
        let mut trivial: Vec<String> = Vec::new();
        if let Some(fams) = res.get("fams").and_then(|v| v.as_array()) {
            for v in fams {
                if let Some(s) = v.as_str() {
                    families.push(s.to_string());
                }
            }
        }
        if let Some(triv) = res.get("triv").and_then(|v| v.as_array()) {
            for v in triv {
                if let Some(s) = v.as_str() {
                    trivial.push(s.to_string());
                }
            }
        }
        families.sort();
        trivial.sort();
        if !families.is_empty() && t.out.starts_with("docs/demos/") {
            fixture_targets.push(FixTarget {
                name: t.name.clone(),
                families,
                trivial,
            });
            continue;
        }
        rendered.push(RenderedEnt {
            t: OraTarget {
                name: t.name.clone(),
                out: t.out.clone(),
            },
            dom: dom.trim().to_string(),
            trivial,
        });
    }
    if !exempt.is_empty() {
        println!(
            "example-oracle: {} examples cannot be bundled (external deps, recorded in src/registry/upstream-snapshot/exemptions.json) — pages stay hand-authored: {}",
            exempt.len(),
            exempt.join(", ")
        );
    }
    if !failures.is_empty() {
        for f in &failures {
            if let Some(i) = f.find(": ") {
                eprintln!("FAIL [{}]: {}", &f[..i], &f[i + 2..]);
            }
        }
        eprintln!(
            "FAIL  example-oracle ({}/{} examples did not render) — nothing written; {} and {} keep their previous contents",
            failures.len(),
            targets.len(),
            ORA_MANIFEST,
            ORA_FIX_TARGETS
        );
        return Err("examples did not render".to_string());
    }

    // Phase 2 — commit pages and manifests together
    for r in &rendered {
        std::fs::write(&r.t.out, ora_page_html(&r.t.name, &r.dom, &r.trivial))
            .map_err(|e| format!("example-oracle: {}", e))?;
    }
    // stale non-RTL pages would otherwise survive forever, and WORSE: the
    // catalog derives "authored" from page EXISTENCE, so a page whose
    // upstream example retired re-authorized itself every run and nothing
    // downstream could tell. This node is the only writer of docs/demos
    // non-RTL pages (example-fixture overwrites a subset after it), so the
    // produced set plus the fixture's own targets plus the two known
    // non-producer exceptions is the truth; anything else is stale by
    // construction. RTL variants belong to build-rtl and are swept there.
    let mut legit: HashSet<String> = rendered
        .iter()
        .filter_map(|r| r.t.out.strip_prefix("docs/demos/"))
        .map(|n| n.trim_end_matches(".html").to_string())
        .collect();
    legit.extend(fixture_targets.iter().map(|f| f.name.clone()));
    // overlay-authored demo pages (e.g. the message-scroller units) are
    // written by `make overlay`, not by this node — they are legit
    if let Some(units) = inputs.overlay_manifest.get("units").and_then(|u| u.as_object()) {
        for (uid, u) in units {
            if let Some(file) = u.get("file").and_then(|f| f.as_str()) {
                if let Some(stem) = file
                    .strip_prefix("docs/demos/")
                    .and_then(|f| f.strip_suffix(".html"))
                {
                    legit.insert(stem.to_string());
                }
            }
            let _ = uid;
        }
    }
    legit.insert("alert-demo".to_string());
    // FT8 guide preview: host page dark-mode; its example lives in the
    // guides tree, not examples/radix
    legit.insert("mode-toggle".to_string());
    let Ok(ents) = std::fs::read_dir("docs/demos") else {
        return Err("example-oracle: docs/demos unreadable".to_string());
    };
    for e in ents.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let stem = name.strip_suffix(".html");
        let Some(stem) = stem else { continue };
        if name.contains("-rtl-") || legit.contains(stem) {
            continue;
        }
        let path = e.path();
        std::fs::remove_file(&path)
            .map_err(|err| format!("example-oracle: removing stale {}: {}", path.display(), err))?;
        println!("example-oracle: removed stale docs/demos/{}", name);
    }
    let mut manifest_b = String::from("[");
    for (i, r) in rendered.iter().enumerate() {
        if i > 0 {
            manifest_b.push(',');
        }
        manifest_b.push_str(&format!(
            "\n {{\n  \"name\": {},\n  \"out\": {}\n }}",
            crate::jsonorder::json_string(&r.t.name),
            crate::jsonorder::json_string(&r.t.out)
        ));
    }
    manifest_b.push_str("\n]\n");
    std::fs::write(ORA_MANIFEST, manifest_b.as_bytes()).map_err(|e| e.to_string())?;

    fixture_targets.sort_by(|a, b| a.name.cmp(&b.name));
    let mut ft_b = String::from("[");
    for (i, ft) in fixture_targets.iter().enumerate() {
        if i > 0 {
            ft_b.push(',');
        }
        // JSON.stringify(_, null, 1) layout: array elements each on their
        // own line, one indent deeper than the key
        let arr = |items: &[String]| -> String {
            if items.is_empty() {
                return "[]".to_string();
            }
            let parts: Vec<String> = items
                .iter()
                .map(|f| crate::jsonorder::json_string(f))
                .collect();
            format!("[\n   {}\n  ]", parts.join(",\n   "))
        };
        ft_b.push_str(&format!(
            "\n {{\n  \"name\": {},\n  \"families\": {},\n  \"trivial\": {}\n }}",
            crate::jsonorder::json_string(&ft.name),
            arr(&ft.families),
            arr(&ft.trivial)
        ));
    }
    ft_b.push_str("\n]\n");
    std::fs::write(ORA_FIX_TARGETS, ft_b.as_bytes()).map_err(|e| e.to_string())?;

    println!(
        "example-oracle: {} pages carry kernel families — handed to example-fixture ({})",
        fixture_targets.len(),
        ORA_FIX_TARGETS
    );
    println!(
        "example-oracle: {} pages emitted from React oracle ({} targets, {} to example-fixture, {} exempt, 0 failures)",
        rendered.len(),
        targets.len(),
        fixture_targets.len(),
        exempt.len()
    );
    Ok(())
}
