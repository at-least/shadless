//! docs-code-parity — the code tab is the copy source, so the tab must
//! RENDER like the preview it sits under. After the slot-only projection,
//! every non-RTL tab keeps only the classes the component stylesheet does
//! not already supply; this gate proves with ZERO diffs (no baseline file,
//! no ledger budget) that the committed tab text renders element-for-element
//! identically to the raw demo markup it was projected from. RTL tabs keep
//! every class by design and are swept too — their fences carry the same
//! guarantee against their `-rtl` demo sources.
//!
//! Two environments, both must be zero-diff:
//! - P (preview): both sides under the precompiled `dist/out.css`.
//! - C (consumer): each side under a CSS compiled the way the docs teach a
//!   consumer — a scratch consumer with a real COPY of package.json +
//!   shadless-core.css + every dist/css file, entry = `@import "shadless"`
//!   and every component css, tailwind's content scan restricted to that
//!   side's texts only. The consumer environment exists because the
//!   precompiled out.css cannot see a utility dropped from a dist/css
//!   `@apply`: only a compile from the @apply sources goes red on it, which
//!   is exactly the drift a consumer hits. A control compile (same entry
//!   without the `@source` lines) proves nothing else is scanned.

use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

use crate::oracle::browser_shell::BrowserShell;

const COLLECT: &str = include_str!("docs_code_parity_collect.js");

/// Same prop set demo-parity compares — they exist precisely to detect
/// slot-rule loss.
const PROPS: [&str; 28] = [
    "color",
    "background-color",
    "border-color",
    "border-top-width",
    "border-radius",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "margin-top",
    "margin-left",
    "width",
    "height",
    "min-width",
    "max-width",
    "font-size",
    "font-weight",
    "line-height",
    "display",
    "flex-direction",
    "align-items",
    "justify-content",
    "gap",
    "position",
    "opacity",
    "box-shadow",
    "text-align",
    "overflow",
];

/// Harness shell pinned on both sides: neither body convention is the
/// component (demo-parity's freeze).
const FREEZE: &str = "*,*::before,*::after{transition:none!important;animation:none!important} body{padding:0!important;margin:0;color:var(--foreground);background:var(--background)}";

static RE_BODY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<body([^>]*)>(.*)</body>").unwrap());
static RE_SCRIPT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<script.*?</script>").unwrap());
static RE_HTML_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<html([^>]*)>").unwrap());
static RE_CLASS_ATTR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"class="([^"]*)""#).unwrap());

#[derive(Debug)]
struct Fence {
    page: String,
    file: String,
    text: String,
}

struct Job {
    page: String,
    file: String,
    r_text: String,
    t_text: String,
}

struct Diff {
    id: String,
    r: String,
    t: String,
}

/// Every ```text,name=<file> fence in the built markdown, in page order.
fn collect_fences(root: &Path) -> Result<Vec<Fence>, String> {
    let mut pages: Vec<String> = Vec::new();
    for dir in ["docs/site/content/components", "docs/site/content/guides"] {
        let full = root.join(dir);
        if !full.exists() {
            return Err(format!(
                "docs-code-parity: {} is not built — run the docs chain first",
                dir
            ));
        }
        let mut here: Vec<String> = std::fs::read_dir(&full)
            .map_err(|e| e.to_string())?
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".md"))
            .map(|n| format!("{dir}/{n}"))
            .collect();
        here.sort();
        pages.extend(here);
    }
    let mut out = Vec::new();
    for p in &pages {
        let b = std::fs::read_to_string(root.join(p)).map_err(|e| e.to_string())?;
        let mut in_tab = false;
        let mut file = String::new();
        let mut lines: Vec<String> = Vec::new();
        for l in b.split('\n') {
            if !in_tab && l.starts_with("```text,name=") {
                in_tab = true;
                file = l["```text,name=".len()..].trim().to_string();
                lines.clear();
                continue;
            }
            if in_tab {
                if l.starts_with("```") {
                    out.push(Fence {
                        page: p.clone(),
                        file: std::mem::take(&mut file),
                        text: lines.join("\n"),
                    });
                    in_tab = false;
                    continue;
                }
                lines.push(l.to_string());
            }
        }
        if in_tab {
            return Err(format!("docs-code-parity: {p}: unterminated text fence"));
        }
    }
    Ok(out)
}

/// The tab source a fence resolves to: the same file the preview iframe
/// shows (docs_build's demo_source resolution).
fn resolve_source(root: &Path, file: &str) -> Option<String> {
    for base in ["docs/demos", "dist/components"] {
        let p = root.join(base).join(file);
        if p.exists() {
            return Some(p.to_string_lossy().into_owned());
        }
    }
    None
}

/// The source page's raw <body> inner, scripts stripped — the bytes the
/// preview iframe renders — serialized through the code-tab writer with the
/// projection OFF, so both sides carry the same attribute escaping and the
/// consumer's scan reads raw `&` on both sides alike.
fn decoded_original(source: &str) -> Result<String, String> {
    let html = std::fs::read_to_string(source).map_err(|e| e.to_string())?;
    let m = RE_BODY
        .captures(&html)
        .ok_or_else(|| format!("{source}: no <body>"))?;
    let bare = RE_SCRIPT.replace_all(&m[2], "").trim().to_string();
    let full = format!("<!doctype html><html><body>{bare}</body></html>");
    use html5ever::tendril::TendrilSink;
    use markup5ever_rcdom::NodeData;
    let dom: markup5ever_rcdom::RcDom = html5ever::parse_document(
        markup5ever_rcdom::RcDom::default(),
        html5ever::ParseOpts::default(),
    )
    .one(full.as_str());
    let html_el = dom
        .document
        .children
        .borrow()
        .iter()
        .find(|c| {
            matches!(
                &c.data,
                NodeData::Element { name, .. } if name.local == html5ever::local_name!("html")
            )
        })
        .ok_or("decoded original: no <html>")?
        .clone();
    let body = html_el
        .children
        .borrow()
        .iter()
        .find(|c| {
            matches!(
                &c.data,
                NodeData::Element { name, .. } if name.local == html5ever::local_name!("body")
            )
        })
        .ok_or("decoded original: no <body>")?
        .clone();
    Ok(crate::emit::writer::serialize_children(&body))
}

fn shell_attrs(html: &str) -> (String, String) {
    let html_attrs = RE_HTML_TAG
        .captures(html)
        .map(|c| c[1].to_string())
        .unwrap_or_default();
    let body_attrs = RE_BODY
        .captures(html)
        .map(|c| c[1].to_string())
        .unwrap_or_default();
    (html_attrs, body_attrs)
}

/// A scratch consumer laid out like consumer-sim's: `node_modules/shadless`
/// holds a real COPY of the package files a CSS consumer's imports reach
/// (copy, not symlink — tailwind resolves imports through the real path),
/// `node_modules/tailwindcss` points at the repo's engine. The temp dir is
/// held by the caller and removed on drop.
fn install_consumer(root: &Path, sim: &Path) -> Result<(), String> {
    let pkg = sim.join("node_modules/shadless");
    std::fs::create_dir_all(pkg.join("dist/css")).map_err(|e| e.to_string())?;
    let mut files = vec![
        "package.json".to_string(),
        "dist/shadless-core.css".to_string(),
    ];
    for e in std::fs::read_dir(root.join("dist/css")).map_err(|e| e.to_string())? {
        let n = e
            .map_err(|e| e.to_string())?
            .file_name()
            .to_string_lossy()
            .into_owned();
        if n.ends_with(".css") {
            files.push(format!("dist/css/{n}"));
        }
    }
    for f in &files {
        std::fs::copy(root.join(f), pkg.join(f))
            .map_err(|e| format!("copying {f} into the consumer: {e}"))?;
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(
            root.join("node_modules/tailwindcss"),
            sim.join("node_modules/tailwindcss"),
        )
        .map_err(|e| e.to_string())?;
    }
    #[cfg(not(unix))]
    {
        let _ = (root, sim);
        return Err("non-unix hosts have no symlink support".to_string());
    }
    Ok(())
}

fn consumer_entry(imports: &[String], source_file: Option<&str>) -> String {
    // source(none) turns tailwind's automatic content detection off: the
    // only scanned inputs are the @source lines below, and the control
    // compile (same entry, no @source) must emit none of their utilities.
    let mut s = String::from("@import \"tailwindcss\" source(none);\n@import \"shadless\";\n");
    for i in imports {
        s.push_str(&format!("@import \"shadless/{i}\";\n"));
    }
    if let Some(f) = source_file {
        s.push_str(&format!("@source \"./{f}\";\n"));
    }
    s
}

fn component_imports(root: &Path) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = std::fs::read_dir(root.join("dist/css"))
        .map_err(|e| e.to_string())?
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".css") && n != "shadless.css")
        .map(|n| n.trim_end_matches(".css").to_string())
        .collect();
    out.sort();
    Ok(out)
}

/// A simple utility that occurs ONLY in the given texts: proves a compile
/// saw them (its rule appears) and that nothing else was scanned (the rule
/// is absent from the control). Picked from the text itself so the proof
/// tracks the corpus instead of a hardcoded token.
fn scan_proof_token(
    texts: &str,
    compiled: &str,
    control: &str,
    dist_css: &str,
    what: &str,
) -> Result<(), String> {
    let mut candidates: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for caps in RE_CLASS_ATTR.captures_iter(texts) {
        for t in caps[1].split_whitespace() {
            let simple = !t.is_empty()
                && t.chars().next().unwrap().is_ascii_lowercase()
                && t.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
            if simple && seen.insert(t.to_string()) {
                candidates.push(t.to_string());
            }
        }
    }
    candidates.sort();
    for t in &candidates {
        let sel = format!(".{t}");
        let in_dist = crate::emit::css_contains_token(dist_css, &sel);
        if !in_dist
            && crate::emit::css_contains_token(compiled, &sel)
            && !crate::emit::css_contains_token(control, &sel)
        {
            return Ok(());
        }
    }
    Err(format!(
        "docs-code-parity: no {what}-only utility proved the content scan (checked {} simple tokens) — \
         either the texts carry no utilities outside dist/css or the scan restriction broke",
        candidates.len()
    ))
}

/// "{page}/{file}/{idx}{slot}/{prop}/{env}@{theme}@{dir}"; slot is
/// " [data-slot=x]" or empty.
#[allow(clippy::too_many_arguments)] // one id field per argument
fn diff_id(
    page: &str,
    file: &str,
    idx: usize,
    slot: &str,
    prop: &str,
    env: &str,
    theme: &str,
    dir: &str,
) -> String {
    let slot_s = if slot.is_empty() {
        String::new()
    } else {
        format!(" [data-slot={slot}]")
    };
    format!("{page}/{file}/{idx}{slot_s}/{prop}/{env}@{theme}@{dir}")
}

/// diff_id → family
/// "{page}/{file}/{idx}{slot}/{prop}" (env×theme×dir are states).
fn diff_family(id: &str) -> String {
    let no_state = id.split('@').next().unwrap_or(id);
    no_state
        .rsplit_once('/')
        .map(|(f, _)| f.to_string())
        .unwrap_or_else(|| no_state.to_string())
}

pub fn run_docs_code_parity(root: &Path) -> i32 {
    let fences = match collect_fences(root) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("docs-code-parity: {}", e);
            return 1;
        }
    };
    let mut jobs: Vec<Job> = Vec::new();
    let mut rtl_fences = 0usize;
    for f in &fences {
        // RTL tabs keep every class by design; they are swept like the
        // rest — their sources exist (docs_build's rtl_exempt_stems
        // guarantees every -rtl demo stem is a real RTL-family member).
        if f.file.contains("-rtl") {
            rtl_fences += 1;
        }
        let Some(source) = resolve_source(root, &f.file) else {
            eprintln!(
                "FAIL  docs-code-parity: {} fences {f:?} but no docs/demos/{}.html or dist/components/{}.html exists",
                f.page, f.file, f.file
            );
            return 1;
        };
        let r_text = match decoded_original(&source) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("FAIL  docs-code-parity: {}: {}", source, e);
                return 1;
            }
        };
        jobs.push(Job {
            page: f.page.clone(),
            file: f.file.clone(),
            r_text,
            t_text: f.text.clone(),
        });
    }
    if jobs.is_empty() {
        eprintln!("FAIL  docs-code-parity: no tab sources found");
        return 1;
    }

    // ---- the consumer environment (one compile per side + a control) ----
    let imports = match component_imports(root) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("docs-code-parity: {}", e);
            return 1;
        }
    };
    let sim_r = tempfile::Builder::new()
        .prefix("shadless-docs-code-parity-r-")
        .tempdir();
    let sim_t = tempfile::Builder::new()
        .prefix("shadless-docs-code-parity-t-")
        .tempdir();
    let (sim_r, sim_t) = match (sim_r, sim_t) {
        (Ok(r), Ok(t)) => (r, t),
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("docs-code-parity: temp consumer: {}", e);
            return 1;
        }
    };
    if let Err(e) =
        install_consumer(root, sim_r.path()).and_then(|_| install_consumer(root, sim_t.path()))
    {
        eprintln!("docs-code-parity: {}", e);
        return 1;
    }
    let r_texts: String = jobs
        .iter()
        .map(|j| j.r_text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let t_texts: String = jobs
        .iter()
        .map(|j| j.t_text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    if let Err(e) = std::fs::write(sim_r.path().join("texts.html"), &r_texts)
        .and_then(|_| std::fs::write(sim_t.path().join("texts.html"), &t_texts))
    {
        eprintln!("docs-code-parity: writing scan texts: {}", e);
        return 1;
    }
    // entry bodies: the imports are identical; only the @source line differs
    // (and the control carries none).
    let write_compile =
        |sim: &Path, source: Option<&str>, entry: &str, out: &str| -> Result<String, String> {
            let body = consumer_entry(&imports, source);
            let entry_p = sim.join(entry);
            let out_p = sim.join(out);
            std::fs::write(&entry_p, body).map_err(|e| e.to_string())?;
            crate::emit::tw::tw_compile_cli(
                &crate::emit::tw::TwCli::dev(),
                root,
                &entry_p.to_string_lossy(),
                &out_p.to_string_lossy(),
                &sim.to_string_lossy(),
                false,
                true,
            )
            .map_err(|e| format!("consumer compile {entry} failed: {e}"))?;
            std::fs::read_to_string(&out_p).map_err(|e| e.to_string())
        };
    let out_r_css = match write_compile(sim_r.path(), Some("texts.html"), "entry.css", "out.css") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("docs-code-parity: {}", e);
            return 1;
        }
    };
    let out_t_css = match write_compile(sim_t.path(), Some("texts.html"), "entry.css", "out.css") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("docs-code-parity: {}", e);
            return 1;
        }
    };
    let control_css =
        match write_compile(sim_t.path(), None, "entry-control.css", "out-control.css") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("docs-code-parity: {}", e);
                return 1;
            }
        };
    let dist_css: String = match component_imports(root) {
        Ok(names) => names
            .iter()
            .filter_map(|n| {
                std::fs::read_to_string(root.join("dist/css").join(format!("{n}.css"))).ok()
            })
            .collect::<Vec<String>>()
            .join("\n"),
        Err(_) => String::new(),
    };
    if let Err(e) = scan_proof_token(&t_texts, &out_t_css, &control_css, &dist_css, "tab-text") {
        eprintln!("{}", e);
        return 1;
    }
    if let Err(e) = scan_proof_token(&r_texts, &out_r_css, &control_css, &dist_css, "demo-text") {
        eprintln!("{}", e);
        return 1;
    }

    // ---- the browser sweep ----
    let out_css = match std::fs::read_to_string(root.join("dist/out.css")) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("docs-code-parity: dist/out.css: {}", e);
            return 1;
        }
    };
    let shell = match BrowserShell::start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("docs-code-parity: {}", e);
            return 1;
        }
    };
    if let Err(e) = shell.launch() {
        eprintln!("docs-code-parity: {}", e);
        return 1;
    }
    let page = match shell.new_page(false) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("docs-code-parity: {}", e);
            return 1;
        }
    };
    let _ = page.route_abort_external();

    let collect_once = || -> Result<serde_json::Value, String> {
        page.evaluate_fn_arg(COLLECT, serde_json::json!({ "props": PROPS.to_vec() }))
            .map_err(|e| e.to_string())
    };

    let mut diffs: Vec<Diff> = Vec::new();
    let mut structural: Vec<String> = Vec::new();
    let (mut compared, mut fences_checked, mut elements_checked, mut template_elements) =
        (0usize, 0usize, 0usize, 0usize);
    for job in &jobs {
        let source = resolve_source(root, &job.file).expect("resolved above");
        let html = std::fs::read_to_string(&source).unwrap_or_default();
        let (html_attrs, body_attrs) = shell_attrs(&html);
        let doc = |css: &str, content: &str| {
            format!(
                "<!doctype html><html{html_attrs}><head><meta charset=\"utf-8\"><style>{css}</style><style>{FREEZE}</style></head><body{body_attrs}>{content}</body></html>"
            )
        };
        let mut sides: Vec<(String, serde_json::Value)> = Vec::new();
        for (env, css_r, css_t) in [("P", &out_css, &out_css), ("C", &out_r_css, &out_t_css)] {
            if let Err(e) = page.set_content(&doc(css_r, &job.r_text)) {
                eprintln!("docs-code-parity: {}: {}", job.file, e);
                return 1;
            }
            let _ = page.wait_for_timeout(30);
            let r = match collect_once() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("docs-code-parity: {}: collect R: {}", job.file, e);
                    return 1;
                }
            };
            if let Err(e) = page.set_content(&doc(css_t, &job.t_text)) {
                eprintln!("docs-code-parity: {}: {}", job.file, e);
                return 1;
            }
            let _ = page.wait_for_timeout(30);
            let t = match collect_once() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("docs-code-parity: {}: collect T: {}", job.file, e);
                    return 1;
                }
            };
            sides.push((env.to_string(), serde_json::json!({ "r": r, "t": t })));
        }
        fences_checked += 1;
        for (env, pair) in &sides {
            let (r, t) = (&pair["r"], &pair["t"]);
            let rt = r["tags"].as_array().cloned().unwrap_or_default();
            let tt = t["tags"].as_array().cloned().unwrap_or_default();
            if rt != tt {
                let at = rt
                    .iter()
                    .zip(tt.iter())
                    .position(|(a, b)| a != b)
                    .unwrap_or(rt.len().min(tt.len()));
                structural.push(format!(
                    "{}: {} element mismatch at #{}: R <{}> vs T <{}> (R {} elements, T {} elements) — the tab text does not reproduce the demo DOM",
                    job.file, env, at,
                    rt.get(at).map(|v| v.as_str().unwrap_or("")).unwrap_or("—"),
                    tt.get(at).map(|v| v.as_str().unwrap_or("")).unwrap_or("—"),
                    rt.len(), tt.len()
                ));
                continue;
            }
            let slots: Vec<String> = r["slots"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|s| s.as_str().unwrap_or("").to_string())
                        .collect()
                })
                .unwrap_or_default();
            // elements inside <template> contents are covered by the tag
            // assert above; they do not render, so there is no computed
            // style to compare
            let rendered: Vec<bool> = r["rendered"]
                .as_array()
                .map(|a| a.iter().map(|v| v.as_bool().unwrap_or(true)).collect())
                .unwrap_or_default();
            if rendered.len() != slots.len() {
                structural.push(format!(
                    "{}: {} collector returned {} rendered flags for {} elements",
                    job.file,
                    env,
                    rendered.len(),
                    slots.len()
                ));
                continue;
            }
            let ro = r["out"].as_object().cloned().unwrap_or_default();
            let to = t["out"].as_object().cloned().unwrap_or_default();
            elements_checked += rendered.iter().filter(|r| **r).count();
            template_elements += rendered.iter().filter(|r| !**r).count();
            for (i, is_rendered) in rendered.iter().enumerate() {
                if !is_rendered {
                    continue;
                }
                for theme in ["light", "dark"] {
                    for dir in ["ltr", "rtl"] {
                        let key = format!("{i}@{theme}@{dir}");
                        let (Some(rs), Some(ts)) = (ro.get(&key), to.get(&key)) else {
                            structural
                                .push(format!("{}: {} missing collect entry {key}", job.file, env));
                            continue;
                        };
                        for p in PROPS {
                            let rv = rs.get(p).and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let tv = ts.get(p).and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let rn = super::parity_baseline::parity_norm_value(&rv, false);
                            let tn = super::parity_baseline::parity_norm_value(&tv, false);
                            compared += 1;
                            if rn != tn {
                                let slot = slots.get(i).map(|s| s.as_str()).unwrap_or("");
                                diffs.push(Diff {
                                    id: diff_id(&job.page, &job.file, i, slot, p, env, theme, dir),
                                    r: rn,
                                    t: tn,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "docs-code-parity: {} fences swept ({} RTL), {} elements compared ({} more in template contents, structural assert only), {} comparisons per environment-pair, across preview + consumer environments",
        fences_checked, rtl_fences, elements_checked, template_elements, compared
    );
    if !structural.is_empty() {
        let n = structural.len().min(10);
        eprintln!(
            "FAIL  docs-code-parity ({} structural mismatches — a projected tab does not reproduce the demo DOM)\n  {}",
            structural.len(),
            structural[..n].join("\n  ")
        );
        return 1;
    }
    if !diffs.is_empty() {
        // group by fence + prop: one root cause shows up as one family
        let mut by_family: std::collections::BTreeMap<String, usize> = Default::default();
        for d in &diffs {
            let family = diff_family(&d.id);
            *by_family.entry(family).or_default() += 1;
        }
        let mut fams: Vec<(String, usize)> = by_family.into_iter().collect();
        fams.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let total_fams = fams.len();
        // a failing run names every cell when DCP_DUMP points at a file —
        // diagnosis surface only, never a baseline
        if let Ok(path) = std::env::var("DCP_DUMP") {
            let mut b = String::new();
            for d in &diffs {
                b.push_str(&format!("{}: R={} T={}\n", d.id, d.r, d.t));
            }
            let _ = std::fs::write(&path, b);
        }
        let n = diffs.len().min(40);
        let parts: Vec<String> = diffs[..n]
            .iter()
            .map(|d| format!("{}: R={} T={}", d.id, d.r, d.t))
            .collect();
        let fam_parts: Vec<String> = fams
            .iter()
            .take(30)
            .map(|(f, c)| {
                format!(
                    "{} ({c} cells)",
                    f.rsplit_once('/')
                        .map(|(_, r)| r.to_string())
                        .unwrap_or_else(|| f.clone())
                )
            })
            .collect();
        eprintln!(
            "FAIL  docs-code-parity ({} cells differ between the committed tab and the raw demo markup — no baseline, every cell must be zero; {} families)\n  families:\n    {}\n  first cells:\n  {}\n",
            diffs.len(),
            total_fams,
            fam_parts.join("\n    "),
            parts.join("\n  ")
        );
        return 1;
    }
    println!(
        "PASS  docs-code-parity ({} fences, {} elements, {} comparisons across preview + consumer environments, 0 diffs; {} RTL fences keep every class and are swept too)",
        fences_checked, elements_checked, compared, rtl_fences
    );
    shell.close();
    drop(sim_r);
    drop(sim_t);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_diff_family_keeps_slot_and_prop_apart() {
        let id = diff_id(
            "button", "demo", 3, "button", "color", "preview", "dark", "rtl",
        );
        assert_eq!(
            id,
            "button/demo/3 [data-slot=button]/color/preview@dark@rtl"
        );
        assert_eq!(diff_family(&id), "button/demo/3 [data-slot=button]/color");
    }
}
