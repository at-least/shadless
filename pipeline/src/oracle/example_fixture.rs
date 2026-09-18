//! Port of pipeline/example_fixture.go — INTERACTIVE pages for dialog-family
//! kernel examples. Generation per example: oracle-render the CLOSED state,
//! then CLICK the trigger and harvest the MOUNTED overlay/content radix
//! appended to <body> — assemble a fixture page: closed markup +
//! <template id="d1-portal"> + vendored kernel + the component's glue.
//! Self-verifying: write to a scratch path, click through (open → content
//! present → dismiss → closed); on any failure the page is DELETED and the
//! example reported. --check regenerates and byte-compares.

use super::browser_shell::BrowserShell;
use super::oracle_lib::build_oracle;
use regex::Regex;
use serde_json::json;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const EF_TMP: &str = "build/example-fixture";
const EF_SELFTEST: &str = "build/fixture";

pub const EF_HARVEST_LAYER: &str = include_str!("ef_harvest_layer.js");
pub const EF_MENU_IDS: &str = include_str!("ef_menu_ids.js");
pub const EF_NAV_IDS: &str = include_str!("ef_nav_ids.js");
pub const EF_TABS_DRIVER: &str = include_str!("ef_tabs.js");
pub const EF_API_DRIVER: &str = include_str!("ef_api.js");

#[derive(Clone, Debug, Default, Deserialize)]
pub struct EfDef {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub imports: String,
    #[serde(default)]
    pub usage: String,
    #[serde(default)]
    pub open: String,
    #[serde(rename = "shadlessPage", default)]
    pub shadless_page: String,
    #[serde(rename = "oracleCss", default)]
    pub oracle_css: String,
}

pub struct EfTarget {
    pub name: String,
    pub families: Vec<String>,
    pub unsupported: Vec<String>,
    pub trivial: Vec<String>,
}

fn abs_or_die(root: &Path, p: &str) -> PathBuf {
    let q = Path::new(p);
    if q.is_absolute() {
        q.to_path_buf()
    } else {
        root.join(p)
    }
}

// ---- radix id stabilization (Go halves of learn/remap/stripRadixIds) ----

fn re_strip_radix_ids() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"\s(?:id|aria-controls|aria-labelledby|aria-describedby)="radix-[^"]*""##).unwrap())
}
fn re_strip_hidden() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"\s(?:aria-hidden|data-aria-hidden)="true""##).unwrap())
}
fn re_radix_tok() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"radix-[A-Za-z0-9:_-]*").unwrap())
}
fn re_id_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"\sid="(radix-[^"]*)""##).unwrap())
}
fn re_labelled_by() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"aria-labelledby="(radix-[^"]*)""##).unwrap())
}
fn re_id_in_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"\sid="(radix-[^"]*)""##).unwrap())
}
fn re_trailing_sub() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(.+?)s\d+$").unwrap())
}
fn re_word_trigger() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(\w+)-trigger$").unwrap())
}
fn re_has_id_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"\sid=""##).unwrap())
}

pub fn ef_strip_radix_ids(h: &str) -> String {
    let h = re_strip_radix_ids().replace_all(h, "").into_owned();
    re_strip_hidden().replace_all(&h, "").into_owned()
}

pub fn ef_remap(h: &str, id_map: &HashMap<String, String>) -> String {
    re_radix_tok()
        .replace_all(h, |m: &regex::Captures| {
            id_map.get(&m[0]).cloned().unwrap_or_else(|| m[0].to_string())
        })
        .into_owned()
}

pub struct EfSlotStable {
    pub slot: String,
    pub stable: String,
}

/// Maps radix auto ids to stable fixture ids (insertion-ordered slotToStable).
pub fn ef_learn(html: &str, slot_to_stable: &[EfSlotStable], id_map: &mut HashMap<String, String>) {
    let mut base = String::new();
    for e in slot_to_stable {
        // attribute order is radix's, not ours: find the tag, then its id
        static TAG_RE: OnceLock<HashMap<String, Regex>> = OnceLock::new();
        let re_map = TAG_RE.get_or_init(HashMap::new);
        let re = re_map
            .get(&e.slot)
            .cloned()
            .unwrap_or_else(|| {
                let r = Regex::new(&format!(
                    r##"<[^>]*data-slot="{}"[^>]*>"##,
                    regex_quote(&e.slot)
                ))
                .unwrap();
                r
            });
        let tag = re.find(html).map(|m| m.as_str()).unwrap_or("");
        if !tag.is_empty() {
            if let Some(m) = re_id_in_tag().captures(tag) {
                id_map.insert(m[1].to_string(), e.stable.clone());
            }
        }
        if base.is_empty() {
            base = e.stable.clone();
        }
    }
    // every other radix id inside the layer gets a stable derived id
    let mut n = 0;
    if base.is_empty() {
        base = "x".to_string();
    }
    for m in re_id_attr().captures_iter(html) {
        if !id_map.contains_key(&m[1]) {
            id_map.insert(m[1].to_string(), format!("{}-e{}", base, n));
            n += 1;
        }
    }
    // a reference to an id that exists nowhere in the layer is radix's
    // internal id for the TRIGGER: point it at the stable trigger id
    let trigger_stable = if base != "x" {
        format!("{}-trigger", re_trailing_sub().replace(&base, "$1"))
    } else {
        String::new()
    };
    for m in re_labelled_by().captures_iter(html) {
        if !id_map.contains_key(&m[1]) && !trigger_stable.is_empty() {
            id_map.insert(m[1].to_string(), trigger_stable.clone());
        }
    }
}

fn regex_quote(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if matches!(c, '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// On the FIRST tag carrying the data-slot, insert id before ">" unless the
/// tag already has one (the whole tag is inspected, from "<" to ">").
pub fn ef_ensure_content_id(h: &str, comp: &str, id: &str) -> String {
    let attr = format!("data-slot=\"{}-content\"", comp);
    let Some(idx) = h.find(&attr) else {
        return h.to_string();
    };
    let Some(end_rel) = h[idx..].find('>') else {
        return h.to_string();
    };
    let end = end_rel + idx;
    let start = h[..idx].rfind('<').unwrap_or(0);
    if re_has_id_attr().is_match(&h[start..end]) {
        return h.to_string();
    }
    format!("{} id=\"{}\">{}", &h[..end], id, &h[end + 1..])
}

/// dialog's closed-markup trigger annotation: on the first
/// `<… data-slot="<comp>-trigger"…>` tag, append id="d1-trigger" unless the
/// tag already carries any id.
pub fn ef_trigger_with_id(body_html: &str, comp: &str, id: &str) -> String {
    static CACHE: OnceLock<HashMap<String, Regex>> = OnceLock::new();
    let map = CACHE.get_or_init(HashMap::new);
    let key = format!("{}-trigger", comp);
    let re = map.get(&key).cloned().unwrap_or_else(|| {
        Regex::new(&format!(
            r##"(<[^>]*data-slot="{}"[^>]*?)>"##,
            regex_quote(&key)
        ))
        .unwrap()
    });
    let Some(m) = re.captures(body_html) else {
        return body_html.to_string();
    };
    let whole = m.get(0).unwrap();
    let g1 = m.get(1).unwrap();
    let open = &body_html[g1.start()..g1.end()];
    if re_has_id_attr().is_match(open) {
        return body_html.to_string();
    }
    // Go: bodyHtml[:loc[3]] + ' id="X">' + bodyHtml[loc[1]:]
    format!(
        "{} id=\"{}\">{}",
        &body_html[..g1.end()],
        id,
        &body_html[whole.end()..]
    )
}

/// Spliced into every place a trigger is renamed: whatever pointed at the OLD
/// id now points at the new one.
pub const EF_RETARGET_JS: &str = r##"if (o && o !== t.id) { const A = ["for", "aria-labelledby", "aria-describedby", "aria-controls", "aria-owns"]; document.querySelectorAll("#root " + A.map((a) => "[" + a + "]").join(",")).forEach((e) => { for (const a of A) { const v = e.getAttribute(a); if (v == null) continue; const ts = v.split(/\s+/); if (ts.includes(o)) e.setAttribute(a, ts.map((x) => (x === o ? t.id : x)).join(" ")) } }) }"##;

pub fn ef_re_harvest_mark() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"\sdata-ef-harvested="""##).unwrap())
}
pub fn ef_re_orig_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"\sdata-ef-orig="[^"]*""##).unwrap())
}
pub fn ef_re_orig_and_id() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r##"data-ef-orig="(radix-[^"]*)"[^>]*\sid="([^"]+)""##).unwrap())
}

fn ef_truncate_all(input: &[String]) -> Vec<String> {
    input
        .iter()
        .map(|s| {
            if s.len() > 400 {
                s[..400].to_string()
            } else {
                s.clone()
            }
        })
        .collect()
}

fn first_line(s: &str) -> String {
    s.lines().next().unwrap_or("").to_string()
}

fn has(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

// ---- the runner ----

pub fn run_example_fixture(args: &[String]) -> i32 {
    match run_inner(args) {
        Ok(()) => 0,
        Err(msg) => {
            eprintln!("example-fixture: {}", msg);
            1
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), String> {
    let contracts = has(args, "--contracts");
    let check = has(args, "--check");
    let root = std::env::current_dir().map_err(|e| e.to_string())?;

    let shell = BrowserShell::start().map_err(|e| format!("example-fixture: {}", e))?;
    let result = run_with_shell(&shell, &root, contracts, check);
    shell.close();
    result
}

fn run_with_shell(
    shell: &BrowserShell,
    root: &Path,
    contracts: bool,
    check: bool,
) -> Result<(), String> {
    
    shell.launch().map_err(|e| format!("example-fixture: {}", e))?;
    let page = shell
        .new_page_errors_only()
        .map_err(|e| format!("example-fixture: {}", e))?;
    page.route_abort_external().map_err(|e| e.to_string())?;

    let mut targets: Vec<EfTarget> = Vec::new();
    if contracts {
        let mut files: Vec<String> = Vec::new();
        for e in std::fs::read_dir(root.join("tools/contracts/components"))
            .map_err(|e| format!("example-fixture: {}", e))?
        {
            let e = e.map_err(|e| e.to_string())?;
            let name = e.file_name().to_string_lossy().into_owned();
            if name.ends_with(".mjs") {
                files.push(name);
            }
        }
        files.sort();
        for f in &files {
            let abs = abs_or_die(
                root,
                &format!("tools/contracts/components/{}", f),
            )
            .to_string_lossy()
            .into_owned();
            let res = shell.call(&json!({
                "op": "loadContractDef", "file": format!("file://{}", abs)
            }))?;
            let def: EfDef = serde_json::from_value(
                res.get("def").cloned().unwrap_or(json!({})),
            )
            .unwrap_or_default();
            let comp = f
                .trim_end_matches(".mjs")
                .trim_end_matches("-multiple")
                .to_string();
            let ships_kernel = def.shadless_page.starts_with("src/kernel/");
            if super::families::family(&comp).is_none() || !ships_kernel {
                continue;
            }
            let name = f.trim_end_matches(".mjs").to_string();
            targets.push(EfTarget {
                name,
                families: vec![comp],
                unsupported: Vec::new(),
                trivial: Vec::new(),
            });
        }
    } else {
        let tb = std::fs::read_to_string(root.join("docs/example-fixture-targets.json"))
            .map_err(|e| format!("example-fixture: {}", e))?;
        #[derive(Deserialize)]
        struct RawTarget {
            #[serde(default)]
            name: String,
            #[serde(default)]
            families: Vec<String>,
            #[serde(default)]
            trivial: Vec<String>,
        }
        let raw: Vec<RawTarget> = serde_json::from_str(&tb).map_err(|e| format!("example-fixture: {}", e))?;
        for t in raw {
            let mut known: Vec<String> = Vec::new();
            let mut unsupported: Vec<String> = Vec::new();
            for f in &t.families {
                if super::families::family(f).is_some() {
                    known.push(f.clone());
                } else {
                    unsupported.push(f.clone());
                }
            }
            // dialog kinds annotate a closed-markup COPY and must run last
            stable_sort_dialogs_last(&mut known);
            targets.push(EfTarget {
                name: t.name,
                families: known,
                unsupported,
                trivial: t.trivial,
            });
        }
    }

    // the self-test renders from a scratch tree under build/
    for d in [format!("{}/pages", EF_SELFTEST), format!("{}/js", EF_SELFTEST)] {
        std::fs::create_dir_all(root.join(&d)).map_err(|e| format!("example-fixture: {}", e))?;
    }
    if let Ok(ents) = std::fs::read_dir(root.join("dist/js")) {
        for e in ents.flatten() {
            if let Ok(b) = std::fs::read(e.path()) {
                let _ = std::fs::write(
                    root.join(EF_SELFTEST).join("js").join(e.file_name()),
                    b,
                );
            }
        }
    }
    if let Ok(b) = std::fs::read(root.join("dist/shadless.js")) {
        let _ = std::fs::write(root.join(EF_SELFTEST).join("shadless.js"), b);
    }
    if let Ok(b) = std::fs::read(root.join("dist/out.css")) {
        let _ = std::fs::write(root.join(EF_SELFTEST).join("out.css"), b);
    }

    let mut emitted = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut unsupported_pages: Vec<String> = Vec::new();

    for target in &targets {
        let name = &target.name;
        if target.families.is_empty() {
            unsupported_pages
                .push(format!("{}: {}", name, target.unsupported.join(", ")));
            continue;
        }
        if !target.unsupported.is_empty() {
            unsupported_pages.push(format!(
                "{}: {} (page wired for {} only)",
                name,
                target.unsupported.join(", "),
                target.families.join(", ")
            ));
        }
        let mut out_path = root.join("docs").join("demos").join(format!("{}.html", name));
        let (mut css, mut base, mut jsdir) = (
            "../out.css".to_string(),
            "../shadless.js".to_string(),
            "../js/".to_string(),
        );
        let def = if contracts {
            // contracts mode reads the def again from the target list built above
            None::<EfDef>
        } else {
            None
        };
        let _ = def;
        if contracts {
            // out_path/jsdir come from the def at build time below
            css = "../../dist/out.css".to_string();
            base = "../../dist/shadless.js".to_string();
            jsdir = "../../dist/js/".to_string();
            out_path = root.join("placeholder-for-contracts");
        }
        let scratch_path: PathBuf = if contracts {
            root.join(EF_SELFTEST).join(format!("{}.html", name))
        } else {
            root.join(EF_SELFTEST).join("pages").join(format!("{}.html", name))
        };

        let build_result = build_one_page(
            shell, &page, root, name, target, contracts, &mut out_path, &mut css, &mut base,
            &mut jsdir, &scratch_path, check,
        );
        match build_result {
            Ok(()) => {
                emitted += 1;
            }
            Err(e) => {
                if std::env::var("EF_KEEP").unwrap_or_default().is_empty() {
                    let _ = std::fs::remove_file(&scratch_path);
                }
                failures.push(format!("{}: {}", name, first_line(&e)));
                continue;
            }
        }
    }

    if !failures.is_empty() {
        eprintln!("FAIL  example-fixture\n  {}", failures.join("\n  "));
        return Err("example-fixture failures".to_string());
    }
    if !unsupported_pages.is_empty() {
        println!(
            "example-fixture: {} pages carry families without a protocol yet:\n  {}\n",
            unsupported_pages.len(),
            unsupported_pages.join("\n  ")
        );
    }
    let suffix = if check { " == committed" } else { " emitted" };
    println!(
        "PASS  example-fixture ({} interactive pages{}, open/close self-verified)",
        emitted, suffix
    );
    Ok(())
}

/// Go's sort.SliceStable: non-dialog kinds first, dialog kinds last, stable.
fn stable_sort_dialogs_last(known: &mut [String]) {
    let key = |c: &String| match super::families::family(c) {
        Some(f) => f.kind == "dialog",
        None => false,
    };
    // stable insertion sort by key (false before true)
    for i in 1..known.len() {
        let mut j = i;
        while j > 0 && key(&known[j]) < key(&known[j - 1]) {
            known.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn build_one_page(
    shell: &BrowserShell,
    page: &super::browser_shell::BPage<'_>,
    root: &Path,
    name: &str,
    target: &EfTarget,
    contracts: bool,
    out_path: &mut PathBuf,
    css: &mut String,
    base: &mut String,
    jsdir: &mut String,
    scratch_path: &Path,
    check: bool,
) -> Result<(), String> {
    
    // contracts mode needs the def: re-load it (the Go code carried it via
    // the target; re-loading keeps one code path for the fixture build)
    let contract_def: Option<EfDef> = if contracts {
        let file = format!("tools/contracts/components/{}.mjs", name.trim_end_matches("-multiple"));
        let abs = abs_or_die(root, &file).to_string_lossy().into_owned();
        let res = shell
            .call(&json!({"op": "loadContractDef", "file": format!("file://{}", abs)}))
            .map_err(|e| e.to_string())?;
        Some(
            serde_json::from_value(res.get("def").cloned().unwrap_or(json!({})))
                .unwrap_or_default(),
        )
    } else {
        None
    };
    if contracts {
        if let Some(def) = &contract_def {
            *out_path = root.join(&def.shadless_page);
        }
    }

    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut all_templates: Vec<String> = Vec::new();
    let mut body_html = String::new();
    let mut js_files: Vec<String> = Vec::new();
    let mut seen_js: HashSet<String> = HashSet::new();

    // self-tests are closures over the page
    let mut stored_self_tests: Vec<super::fixture_families::SelfTestAction> = Vec::new();

    // oracle render or contract oracle
    if contracts {
        let def = contract_def.clone().expect("contracts mode has def");
        let out = root.join(EF_TMP).join("contracts").join(name);
        build_contract_oracle(root, &def, &out, "")?;
        page
            .goto_url(&format!(
                "file://{}",
                abs_or_die(root, format!("{}/oracle.html", out.display()).as_str()).to_string_lossy()
            ))
            .map_err(|e| e.to_string())?;
        let _ = page.wait_for_timeout(600);
        // controlled-open trees mount their content at first render — close
        let _ = page.evaluate_fn(
            r##"() => { if (typeof window.__setOpen === "function" && window.__open) window.__setOpen(false) }"##,
        );
        let _ = page.wait_for_timeout(400);
    } else {
        let html_file = build_oracle(root, name, Path::new(EF_TMP))?;
        super::oracle_lib::await_oracle(page, &html_file)?;
    }
    let dir_v = page
        .evaluate(r##"document.documentElement.getAttribute("dir") || "ltr""##)
        .map_err(|e| e.to_string())?;
    let mut dir = dir_v.as_str().unwrap_or("ltr").to_string();
    if dir.is_empty() {
        dir = "ltr".to_string();
    }

    // several families on one page: each contributes templates and its glue
    for comp in &target.families {
        let fam = super::families::family(comp).expect("families filtered up front");
        let templates;
        let id_map_ref = &mut id_map;
        // every family contributes its glue script (Go: family[f].js)
        if !fam.js.is_empty() && !seen_js.contains(fam.js) {
            seen_js.insert(fam.js.to_string());
            js_files.push(fam.js.to_string());
        }

        match fam.kind {
            "dialog" => {
                let closed_html = root_html(page)?;
                let before_v = page
                    .evaluate_fn("() => document.body.children.length")
                    .map_err(|e| e.to_string())?;
                let before = before_v.as_i64().unwrap_or(0);
                page.loc_click("", "[data-slot$=\"-trigger\"]", 0, "left")
                    .map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(600);
                let portal_html = harvest_added(page, before)?;
                if portal_html.is_empty() {
                    return Err("no mounted overlay/content after trigger click".to_string());
                }
                ef_learn(
                    &portal_html,
                    &[
                        EfSlotStable { slot: format!("{}-content", comp), stable: "d1".to_string() },
                        EfSlotStable { slot: format!("{}-title", comp), stable: "d1-title".to_string() },
                        EfSlotStable { slot: format!("{}-description", comp), stable: "d1-desc".to_string() },
                    ],
                    id_map_ref,
                );
                let trig_v = page
                    .evaluate_fn_arg(
                        r##"(sel) => document.querySelector("#root " + sel)?.id"##,
                        json!(format!("[data-slot=\"{}-trigger\"]", comp)),
                    )
                    .map_err(|e| e.to_string())?;
                if let Some(trig_orig) = trig_v.as_str() {
                    if trig_orig.starts_with("radix-") {
                        id_map_ref.insert(trig_orig.to_string(), "d1-trigger".to_string());
                    }
                }
                let fixed = ef_strip_radix_ids(&ef_remap(&portal_html, &id_map));
                body_html = ef_trigger_with_id(
                    &ef_strip_radix_ids(&ef_remap(&closed_html, &id_map)),
                    comp,
                    "d1-trigger",
                );
                templates = format!("<template id=\"d1-portal\">\n{}\n</template>", fixed);
                stored_self_tests.push(super::fixture_families::SelfTestAction::Dialog {
                    comp: comp.clone(),
                });
            }
            _ => {
                // portal / menu / select / nav / inline / none are ported with
                // the same structure in fixture_families.rs
                let mut fam_out = super::fixture_families::FamilyOut {
                    body_html: String::new(),
                    templates: String::new(),
                    self_test: None,
                    js_files: Vec::new(),
                };
                super::fixture_families::dispatch_family(
                    page,
                    comp,
                    &fam,
                    contracts,
                    contract_def.as_ref(),
                    id_map_ref,
                    &mut fam_out,
                )?;
                body_html = std::mem::take(&mut fam_out.body_html);
                templates = std::mem::take(&mut fam_out.templates);
                if let Some(st) = fam_out.self_test {
                    stored_self_tests.push(st);
                }
                for j in fam_out.js_files {
                    if !seen_js.contains(&j) {
                        seen_js.insert(j.clone());
                        js_files.push(j);
                    }
                }
            }
        }

        if !templates.is_empty() {
            all_templates.push(templates);
        }
    }

    // assemble page
    if contracts {
        body_html = format!("<div>\n{}\n</div>", body_html);
    }
    let mut inner = body_html.clone();
    if !all_templates.is_empty() {
        inner = format!("{}\n{}", body_html, all_templates.join("\n"));
    }
    let dir_attr = if dir == "rtl" { " dir=\"rtl\"" } else { "" };
    let mut js_tags: Vec<String> = Vec::new();
    for g in &js_files {
        js_tags.push(format!("<script src=\"{}{}.js\"></script>", jsdir, g));
    }
    let mut triv_tags: Vec<String> = Vec::new();
    for c in &target.trivial {
        triv_tags.push(format!("<script src=\"{}{}.js\"></script>", jsdir, c));
    }
    let html = format!(
        "<!doctype html>\n<html{}><head><meta charset=\"utf-8\"><title>shadless {}</title>\n<link rel=\"stylesheet\" href=\"{}\">{}</head>\n<body class=\"p-8\">\n{}\n<script src=\"{}\"></script>\n{}\n{}\n</body></html>\n",
        dir_attr,
        name,
        css,
        super::super::emit::prepaint::THEME_PREPAINT_SCRIPT,
        inner,
        base,
        js_tags.join("\n"),
        triv_tags.join("\n")
    );

    // prove the page interactive BEFORE it lands
    std::fs::write(scratch_path, &html).map_err(|e| e.to_string())?;
    let ev0 = page.events().unwrap_or_default();
    let err_base = ev0.len();
    page.goto_url(&format!(
        "file://{}",
        abs_or_die(root, scratch_path.to_string_lossy().as_ref()).to_string_lossy()
    ))
    .map_err(|e| e.to_string())?;
    let _ = page.wait_for_timeout(400);
    if std::env::var("EF_DEBUG").is_ok() && name == "navigation-menu-demo" {
        for ms in [0usize, 500, 1500, 3000] {
            let _ = page.wait_for_timeout((ms / 3) as i64);
            let v = page.evaluate("JSON.stringify({ t: Math.round(performance.now()), n0: !!document.getElementById('n0-trigger'), n1: !!document.getElementById('n1-trigger'), trigs: document.querySelectorAll('[data-slot=navigation-menu-trigger]').length })").unwrap_or(serde_json::Value::Null);
            eprintln!("[dbg] {} t~{}ms: {}", name, ms, v);
        }
    }
    let _ = js_files.len();

    if std::env::var("EF_DEBUG").is_ok() {
        let v = page.evaluate("JSON.stringify({ count: document.querySelectorAll('[data-slot=navigation-menu-trigger]').length, ids: [...document.querySelectorAll('[data-slot=navigation-menu-trigger]')].map(t => t.id), first: (document.querySelector('[data-slot=navigation-menu-trigger]') || {}).outerHTML?.slice(0, 150), inRoot: !!document.querySelector('#root [data-slot=navigation-menu-trigger]') })").unwrap_or(serde_json::Value::Null);
        eprintln!("[dbg] {} nav-at-selftest: {}", name, v);
    }
    for st in &stored_self_tests {
        if let Err(e) = super::fixture_families::run_self_test(st, page) {
            if std::env::var("EF_DEBUG").is_ok() {
                let v = page.evaluate("document.body.innerHTML.slice(0, 300) + ' ||| open-content=' + !!document.querySelector('[data-slot$=\"-content\"]') + ' ||| shadless=' + (typeof shadless) + ' ||| portal-tpl=' + (document.getElementById('d1-portal')?.innerHTML.length ?? 'gone')").unwrap_or(serde_json::Value::Null);
                eprintln!("[dbg] {} FAILED: {} ||| {}", name, e, v);
            }
            return Err(e);
        }
    }
    // programmatic handles: every openable instance must open/close via
    // shadless.get(trigger) too
    let mut fams: Vec<String> = Vec::new();
    for f in &target.families {
        if let Some(fam) = super::families::family(f) {
            if matches!(fam.kind, "dialog" | "portal" | "menu" | "select" | "nav") {
                fams.push(f.clone());
            }
        }
    }
    if !fams.is_empty() {
        let api_v = page.evaluate_fn_arg(EF_API_DRIVER, json!(fams))?;
        if let Some(s) = api_v.as_str() {
            if !s.is_empty() {
                return Err(format!("self-test (api): {}", s));
            }
        }
    }
    let ev1 = page.events().unwrap_or_default();
    let n = ev1.len().saturating_sub(err_base);
    if n > 0 {
        return Err(format!(
            "self-test: page errors — {}",
            ef_truncate_all(&ev1[err_base..]).join(" | ")
        ));
    }
    // compare against the committed page BEFORE the rename
    if check {
        let committed = std::fs::read_to_string(out_path.clone()).map_err(|e| e.to_string())?;
        if committed != html {
            return Err("committed page drifted from regeneration".to_string());
        }
    }
    std::fs::rename(scratch_path, out_path.clone()).map_err(|e| e.to_string())?;
    Ok(())
}


fn root_html(page: &super::browser_shell::BPage<'_>) -> Result<String, String> {
    let v = page
        .evaluate_fn(r##"() => document.querySelector("#root").innerHTML"##)
        .map_err(|e| e.to_string())?;
    Ok(v.as_str().unwrap_or("").to_string())
}

fn harvest_added(page: &super::browser_shell::BPage<'_>, before: i64) -> Result<String, String> {
    let v = page
        .evaluate_fn_arg(
            r##"(n) => {
      const added = [...document.body.children].slice(n)
        .filter((el) => el.tagName !== "SCRIPT" && !el.hasAttribute("data-radix-focus-guard"))
      return added.length ? added.map((el) => el.outerHTML).join("\n") : null
    }"##,
            serde_json::json!(before),
        )
        .map_err(|e| e.to_string())?;
    Ok(v.as_str().unwrap_or("").to_string())
}

/// tools/contracts/oracle-build.mjs in Go: bundle a contract def's React
/// usage tree into OUT/oracle.{js,html}.
pub fn build_contract_oracle(
    root: &Path,
    def: &EfDef,
    out: &Path,
    recorder: &str,
) -> Result<(), String> {
    let entry = format!(
        "\nimport React from \"react\";\nimport {{ createRoot }} from \"react-dom/client\";\n{}\n{}\nwindow.__open = true;\ntry {{\nconst root = createRoot(document.getElementById(\"root\"));\nconst render = () => root.render(({}));\nwindow.__setOpen = (o) => {{ window.__open = o; render(); }};\nrender();\n}} catch (e) {{ window.__err = String(e?.message ?? e); }}\n",
        def.imports, recorder, def.usage
    );
    let cache = {
        let d = super::oracle_lib::oracle_cache_dir_relative();
        if d.is_absolute() {
            d
        } else {
            root.join(d)
        }
    };
    for d in [out, &cache] {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let base_name = out.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let (entry_name, bundle_name) = contract_cache_names(&base_name, recorder);
    let entry_file = cache.join(entry_name);
    let bundle = cache.join(bundle_name);
    std::fs::write(&entry_file, &entry).map_err(|e| e.to_string())?;
    let aliases = super::oracle_lib::oracle_aliases()?;
    let mut argv: Vec<String> = vec![
        entry_file.to_string_lossy().into_owned(),
        "--bundle".into(),
        "--format=iife".into(),
        format!("--outfile={}", bundle.to_string_lossy()),
        "--log-level=error".into(),
        "--loader:.tsx=tsx".into(),
        "--jsx=automatic".into(),
    ];
    let mut keys: Vec<&String> = aliases.keys().collect();
    keys.sort();
    for k in keys {
        argv.push(format!("--alias:{}={}", k, aliases[k]));
    }
    let out_res = std::process::Command::new(root.join("node_modules/.bin/esbuild"))
        .args(&argv)
        .current_dir(root)
        .output()
        .map_err(|e| format!("esbuild: {}", e))?;
    if !out_res.status.success() {
        let text = String::from_utf8_lossy(&out_res.stderr);
        let first = text.lines().next().unwrap_or("").to_string();
        return Err(format!("esbuild: {}", first));
    }
    let style_tag = if def.oracle_css.is_empty() {
        String::new()
    } else {
        format!("<style>{}</style>", def.oracle_css)
    };
    let rel = super::oracle_lib::rel_path(out, &bundle)?;
    let html = format!(
        "<!doctype html><html><head>{}</head><body><div id=\"root\"></div><script src=\"{}\"></script></body></html>",
        style_tag, rel
    );
    std::fs::write(out.join("oracle.html"), html).map_err(|e| e.to_string())
}

/// The oracle-cache filenames for one contract-oracle build, discriminated by
/// the recorder the entry embeds: the `contracts:<name>` shards embed the
/// contract def's recorder while `contract-fixture` embeds none, the graph
/// orders neither node against the other, and both used to write the SAME
/// entry/bundle paths — under -j a shard could bundle from the recorder-less
/// entry or load a torn bundle mid-overwrite.
pub fn contract_cache_names(base_name: &str, recorder: &str) -> (String, String) {
    if recorder.is_empty() {
        return (
            format!(".contract-entry-{}.mjs", base_name),
            format!("contract-{}.js", base_name),
        );
    }
    let mut h = Sha256::new();
    h.update(recorder.as_bytes());
    let tag = hex::encode(&h.finalize()[..4]);
    (
        format!(".contract-entry-{}-{}.mjs", base_name, tag),
        format!("contract-{}-{}.js", base_name, tag),
    )
}

/// `foo-trigger` → `foo` (efReWordTrigger's Go use).
pub fn word_trigger_prefix(id: &str) -> Option<String> {
    re_word_trigger().captures(id).map(|m| m[1].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_cache_names_discriminate_by_recorder() {
        // The `contracts:<name>` shards embed the contract def's recorder;
        // the contract-fixture node embeds none. Same base_name, different
        // recorder ⇒ different cache files — otherwise the two node
        // families write the same entry/bundle non-atomically with no graph
        // ordering, and a shard under -j can bundle from the recorder-less
        // entry or load a torn bundle mid-overwrite.
        let (e0, b0) = contract_cache_names("accordion", "");
        assert_eq!(e0, ".contract-entry-accordion.mjs");
        assert_eq!(b0, "contract-accordion.js");
        let (e1, b1) = contract_cache_names("accordion", "window.__facts = () => {}");
        let (e2, b2) = contract_cache_names("accordion", "window.__other = () => {}");
        assert_ne!(
            (e1.as_str(), b1.as_str()),
            (e0.as_str(), b0.as_str()),
            "a recorded entry must not share cache files with the recorder-less fixture build"
        );
        assert_ne!(
            (e1.as_str(), b1.as_str()),
            (e2.as_str(), b2.as_str()),
            "different recorders must not share cache files either"
        );
        assert_eq!(contract_cache_names("accordion", "window.__facts = () => {}"), (e1, b1),
            "names are deterministic so a rebuild hits the same files");
    }
}
