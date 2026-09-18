//! Port of pipeline/example_golden.go — hop 1 of the 1:1 gate: local oracle
//! render == upstream live-site snapshot. Both sides are canonicalized in a
//! real browser (canonOf); the snapshot is the committed SSR artifact.
//! Exemptions are explicit and their staleness fails the gate.

use super::browser_shell::BrowserShell;
use super::oracle_lib::{await_oracle, build_oracle, canon_of, oracle_root_html};
use regex::Regex;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::OnceLock;

/// Both spellings a normalised auto-id can have in a diff context:
/// oracle_norm emits radix-a<N>, oracle_canon.js emits radix-<id>.
fn re_golden_auto_id() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"radix-(?:<id>|a\d+)").unwrap())
}

fn re_long_quoted() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#""[^"]{20,}""#).unwrap())
}

/// First-difference window with clamped context on both sides.
fn first_diff_window(a: &str, b: &str, before: usize, after: usize) -> (String, String) {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let mut i = 0;
    while i < ab.len() && i < bb.len() && ab[i] == bb[i] {
        i += 1;
    }
    // the first differing BYTE can sit mid-character (two encodings can
    // share a prefix byte) — clamp every slice edge to a char boundary or
    // the failure report itself panics instead of reporting the diff
    let floor = |mut x: usize, s: &str| -> usize {
        while x > 0 && !s.is_char_boundary(x) {
            x -= 1;
        }
        x
    };
    let ceil = |mut x: usize, s: &str| -> usize {
        while x < s.len() && !s.is_char_boundary(x) {
            x += 1;
        }
        x
    };
    let window = |s: &str| -> String {
        let lo = floor(i.saturating_sub(before), s);
        let hi = ceil((i + after).min(s.len()), s);
        s[lo..hi].to_string()
    };
    (window(a), window(b))
}


/// Truncates to `max` bytes; the report paths only, never worth a panic on
/// multi-byte content.
fn truncate_utf8(s: &mut String, max: usize) {
    if s.len() <= max {
        return;
    }
    let mut cut = max;
    while !s.is_char_boundary(cut) {
        cut -= 1;
    }
    s.truncate(cut);
}

fn file_exists(p: &str) -> bool {
    Path::new(p).exists()
}

pub fn run_example_golden(args: &[String]) -> i32 {
    match run_inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("example-golden: {}", msg);
            1
        }
    }
}

fn run_inner(args: &[String]) -> Result<i32, String> {
    const EXAMPLES_DIR: &str = ".upstream/shadcn-ui/apps/v4/examples/radix";
    const SNAPSHOT_DIR: &str = "src/registry/upstream-snapshot";
    const TMP: &str = "build/example-golden";

    let mut mode = "gate";
    let mut diff_name = String::new();
    let mut diff_page = String::new();
    for i in 0..args.len() {
        match args[i].as_str() {
            "--classify" => mode = "classify",
            "--diff" => {
                mode = "diff";
                if i + 1 < args.len() {
                    diff_name = args[i + 1].clone();
                }
                if i + 2 < args.len() {
                    diff_page = args[i + 2].clone();
                }
            }
            _ => {}
        }
    }

    let shell = BrowserShell::start()?;
    let result = run_inner_shell(&shell, args, mode, &diff_name, &diff_page, EXAMPLES_DIR, SNAPSHOT_DIR, TMP);
    shell.close();
    result
}

/// Loads one golden snapshot file into its previews map. A damaged file is
/// corpus damage, not a per-example diff: the caller fails the gate rather
/// than silently comparing less.
fn load_snapshot(dir: &str, pf: &str) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    let b = std::fs::read_to_string(Path::new(dir).join(pf))
        .map_err(|e| format!("{}: {}", pf, e))?;
    let snap: serde_json::Value = serde_json::from_str(&b).map_err(|e| format!("{}: {}", pf, e))?;
    snap.get("previews")
        .and_then(|p| p.as_object())
        .cloned()
        .ok_or_else(|| format!("{}: no previews object", pf))
}

fn run_inner_shell(
    shell: &BrowserShell,
    _args: &[String],
    mode: &str,
    diff_name: &str,
    diff_page: &str,
    examples_dir: &str,
    snapshot_dir: &str,
    tmp: &str,
) -> Result<i32, String> {
    shell.launch()?;
    let page = shell.new_page(false).ok();
    let page_ref = page.as_ref();

    // keep avatar-style examples in their INITIAL render state: a loaded
    // image flips radix Avatar to the img branch and the trees diverge on
    // structure, not styling
    if let Some(p) = page_ref {
        p.route_abort_external()?;
        p.goto_url("about:blank")?;
    }

    let oracle_canon = |name: &str| -> Result<String, String> {
        let page = page_ref.ok_or("no page")?;
        let html_file = build_oracle(Path::new("."), name, Path::new(tmp))?;
        await_oracle(page, &html_file)?;
        let root_html = oracle_root_html(page)?;
        canon_of(page, &root_html)
    };

    if mode == "diff" {
        let mut page_name = diff_page.to_string();
        if page_name.is_empty() {
            page_name = diff_name.splitn(2, '-').next().unwrap_or("").to_string();
        }
        let snap_b = std::fs::read_to_string(Path::new(snapshot_dir).join(format!("{}.json", page_name)))
            .map_err(|e| format!("example-golden: {}", e))?;
        let snap: serde_json::Value = serde_json::from_str(&snap_b).unwrap_or(serde_json::Value::Null);
        let upstream_html = snap["previews"][diff_name].as_str().unwrap_or("").to_string();
        if upstream_html.is_empty() {
            return Err(format!("no snapshot preview {:?} in {}.json", diff_name, page_name));
        }
        let a = oracle_canon(diff_name)?;
        let b = canon_of(page_ref.ok_or("no page")?, &upstream_html)?;
        if a == b {
            println!("EQUAL");
            return Ok(0);
        }
        let (wa, wb) = first_diff_window(&a, &b, 80, 120);
        println!("ORACLE  : {}", wa);
        println!("UPSTREAM: {}", wb);
        return Ok(1);
    }

    #[derive(Deserialize, Default, Clone)]
    struct Exemption {
        #[serde(default)]
        stale: bool,
        #[serde(default)]
        reason: String,
    }
    #[derive(Deserialize, Default)]
    struct Exemptions {
        #[serde(default)]
        examples: HashMap<String, Exemption>,
    }
    let exemptions: Exemptions = std::fs::read_to_string(format!("{}/exemptions.json", snapshot_dir))
        .ok()
        .and_then(|eb| serde_json::from_str(&eb).ok())
        .unwrap_or_default();

    let sig = |a: &str, b: &str| -> String {
        let (wa, wb) = first_diff_window(a, b, 60, 60);
        let mut ctx = format!("{} ||| {}", wa, wb);
        ctx = re_golden_auto_id().replace_all(&ctx, "#").into_owned();
        truncate_utf8(&mut ctx, 200);
        ctx
    };

    let mut pass = 0;
    let mut fail = 0;
    let mut exempt = 0;
    let mut corrupt = 0usize;
    let mut stale_exemptions: Vec<String> = Vec::new();
    let mut buckets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    struct FailureRec {
        name: String,
        page: String,
        kind: String,
        signature: String,
        error: String,
    }
    let mut failures: Vec<FailureRec> = Vec::new();
    let bucket_of = |key: &str, name: &str, buckets: &mut BTreeMap<String, Vec<String>>| {
        let mut k = re_long_quoted().replace_all(key, "\"…\"").into_owned();
        truncate_utf8(&mut k, 110);
        buckets.entry(k).or_default().push(name.to_string());
    };

    let mut pages: Vec<String> = Vec::new();
    for e in std::fs::read_dir(snapshot_dir).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        let name = e.file_name().to_string_lossy().into_owned();
        if name.ends_with(".json") && name != "exemptions.json" {
            pages.push(name);
        }
    }
    pages.sort();
    for pf in &pages {
        let previews = match load_snapshot(snapshot_dir, pf) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "FAIL example-golden: unreadable snapshot ({}); the golden corpus is damaged — failing instead of shrinking coverage",
                    e
                );
                corrupt += 1;
                continue;
            }
        };
        for (name, preview) in &previews {
            let Some(upstream_html) = preview.as_str() else {
                eprintln!(
                    "FAIL example-golden: snapshot {} preview {} is not a string — the golden corpus is damaged",
                    pf, name
                );
                corrupt += 1;
                continue;
            };
            let ex = exemptions.examples.get(name).cloned().unwrap_or_default();
            if !ex.reason.is_empty() && !ex.stale {
                exempt += 1;
                continue;
            }
            let tsx = format!("{}/{}.tsx", examples_dir, name);
            if !file_exists(&tsx) {
                if !ex.reason.is_empty() {
                    exempt += 1;
                    continue;
                }
                eprintln!(
                    "FAIL [{}]: snapshot demo has no example tsx and no exemption",
                    name
                );
                fail += 1;
                continue;
            }
            let sa = match oracle_canon(name) {
                Ok(v) => v,
                Err(e) => {
                    if !ex.reason.is_empty() {
                        exempt += 1;
                        continue;
                    }
                    let msg = e.splitn(2, '\n').next().unwrap_or("").to_string();
                    if mode == "classify" {
                        bucket_of(&format!("RENDER-FAIL {}", msg), name, &mut buckets);
                        failures.push(FailureRec {
                            name: name.clone(),
                            page: pf.trim_end_matches(".json").to_string(),
                            kind: "render".to_string(),
                            signature: String::new(),
                            error: msg,
                        });
                    } else {
                        eprintln!(
                            "FAIL [{}]: oracle build/render failed — {} (add an exemption with a reason if unfixable)",
                            name, msg
                        );
                    }
                    fail += 1;
                    continue;
                }
            };
            let sb = canon_of(page_ref.ok_or("no page")?, upstream_html)?;
            if sa == sb {
                pass += 1;
            } else {
                if mode == "classify" {
                    let s = sig(&sa, &sb);
                    bucket_of(&s, name, &mut buckets);
                    failures.push(FailureRec {
                        name: name.clone(),
                        page: pf.trim_end_matches(".json").to_string(),
                        kind: "diff".to_string(),
                        signature: s,
                        error: String::new(),
                    });
                } else {
                    eprintln!("FAIL [{}]: oracle != upstream snapshot", name);
                }
                fail += 1;
            }
            if !ex.reason.is_empty() {
                stale_exemptions.push(name.clone());
            }
        }
    }
    let mut exit = 0;
    if corrupt > 0 {
        eprintln!(
            "FAIL  example-golden: {} snapshot file(s) unreadable — the golden gate compared less than the corpus promises",
            corrupt
        );
        exit = 1;
    }
    if !stale_exemptions.is_empty() {
        eprintln!(
            "FAIL  example-golden: stale exemptions (rendered fine, remove them): {}",
            stale_exemptions.join(", ")
        );
        exit = 1;
    }
    if mode == "classify" {
        let mut out = String::from("[");
        for (i, f) in failures.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(&format!(
                "\n {{\n  \"name\": {},\n  \"page\": {},\n  \"kind\": {},{}\n  \"error\": {}\n }}",
                crate::jsonorder::json_string(&f.name),
                crate::jsonorder::json_string(&f.page),
                crate::jsonorder::json_string(&f.kind),
                if f.signature.is_empty() {
                    String::new()
                } else {
                    format!("  \"signature\": {},", crate::jsonorder::json_string(&f.signature))
                },
                crate::jsonorder::json_string(&f.error)
            ));
        }
        out.push_str("\n]\n");
        std::fs::create_dir_all(tmp).map_err(|e| e.to_string())?;
        std::fs::write(format!("{}/failures.json", tmp), out).map_err(|e| e.to_string())?;
        println!(
            "classify: {} pass, {} fail, {} exempt ({} recorded to {}/failures.json)",
            pass,
            fail,
            exempt,
            failures.len(),
            tmp
        );
        let mut keys: Vec<&String> = buckets.keys().collect();
        keys.sort_by(|a, b| buckets[*b].len().cmp(&buckets[*a].len()));
        for k in keys {
            println!(
                "\n{:>4}×  {}\n     {}",
                buckets[k].len(),
                k,
                buckets[k].join(", ")
            );
        }
        if fail > 0 {
            exit = 1;
        }
    } else if fail > 0 {
        eprintln!(
            "FAIL  example-golden ({} failed, {} passed, {} exempt)",
            fail, pass, exempt
        );
        exit = 1;
    } else if exit == 0 {
        println!(
            "PASS  example-golden ({} == upstream snapshot, {} exempt)",
            pass, exempt
        );
    }
    Ok(exit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn load_snapshot_rejects_a_damaged_corpus_file() {
        // A truncated/invalid snapshot used to be skipped with a bare
        // `continue` — the gate compared less and still printed PASS. The
        // loader must surface the damage so the gate can fail loudly.
        let t = std::env::temp_dir().join(format!("shadless-golden-{}", std::process::id()));
        fs::create_dir_all(&t).unwrap();
        fs::write(t.join("ok.json"), r#"{"previews": {"a": "<div></div>"}}"#).unwrap();
        fs::write(t.join("truncated.json"), r#"{"previews": {"a""#).unwrap();
        fs::write(t.join("wrongshape.json"), r#"{"pages": []}"#).unwrap();

        let ok = load_snapshot(t.to_str().unwrap(), "ok.json").unwrap();
        assert_eq!(ok.get("a").and_then(|v| v.as_str()), Some("<div></div>"));

        let err = load_snapshot(t.to_str().unwrap(), "truncated.json").unwrap_err();
        assert!(err.contains("truncated.json"), "{}", err);
        let err = load_snapshot(t.to_str().unwrap(), "wrongshape.json").unwrap_err();
        assert!(err.contains("no previews object"), "{}", err);
        let err = load_snapshot(t.to_str().unwrap(), "missing.json").unwrap_err();
        assert!(err.contains("missing.json"), "{}", err);
        let _ = fs::remove_dir_all(&t);
    }


    /// The two spellings the canon actually produces: oracle_norm emits
    /// radix-a<N> and oracle_canon.js emits radix-<id>. The old pattern
    /// carried a double-escaped \\d that matched neither, so the id masker
    /// was inert and classify-mode failures that differed only in id
    /// spelling never bucketed together.
    #[test]
    fn unit_golden_auto_id_mask_matches_both_spellings() {
        let re = re_golden_auto_id();
        assert!(re.is_match("radix-a1"), "oracle_norm spelling");
        assert!(re.is_match("radix-a12"), "oracle_norm multi-digit");
        assert!(re.is_match("radix-<id>"), "canon.js spelling");
        assert_eq!(
            re.replace_all("x radix-a1 y radix-<id> z", "#"),
            "x # y # z"
        );
    }

    /// é = C3 A9 vs ê = C3 EA: the first differing BYTE sits mid-character
    /// in both strings, and the old byte-index slicing panicked instead of
    /// reporting the diff.
    #[test]
    fn unit_diff_window_survives_a_mid_character_divergence() {
        let a = "\u{e9}".repeat(41);
        let b = format!("{}\u{ea}", "\u{e9}".repeat(40));
        let (wa, wb) = first_diff_window(&a, &b, 60, 60);
        assert!(!wa.is_empty() && !wb.is_empty(), "window must not panic");
        assert_ne!(wa, wb);
    }

    #[test]
    fn unit_truncate_utf8_never_splits_a_character() {
        let mut s = "\u{e9}".repeat(150); // 300 bytes; byte 201 is mid-character
        truncate_utf8(&mut s, 201);
        assert!(s.len() <= 200);
        assert!(s.chars().all(|c| c == '\u{e9}'), "no split character");
    }
}
