//! Port of pipeline/build_rtl.go — multi-language RTL demo emission. Reads
//! src/registry/rtl-translations.json, substitutes each ar value with the
//! target language's, patches lang/dir attributes, and injects the theme
//! pre-paint script. Nothing is written on any failure.

use super::prepaint::inject_pre_paint;
use regex::Regex;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::sync::{LazyLock, OnceLock};

static RE_HTML_HAS_LANG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<html[^>]*[\t\n\f\r ]lang=""#).unwrap());
static RE_HTML_LANG_ATTR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(<html[^>]*[\t\n\f\r ]lang=")[^"]*(")"#).unwrap());
static RE_HTML_OPEN_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<html(\s[^>]*)?>").unwrap());
static RE_DIR_ATTR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"([\s"'])dir="(rtl|ltr)""#).unwrap());

#[derive(Deserialize, Clone, Default)]
pub struct Translation {
    #[serde(default)]
    pub dir: String,
    #[serde(default)]
    pub values: HashMap<String, String>,
}

/// Mirrors tools/rtl-lib.mjs. Order of the two phases is load-bearing:
/// ar→X substitution first (longest-value ordering avoids a "Loading" /
/// "Loading…" prefix collision), then the lang/dir attribute patch.
pub fn substitute_and_patch(
    arabic_html: &str,
    translations: &HashMap<String, Translation>,
    from_lang: &str,
    to_lang: &str,
    to_values: &HashMap<String, String>,
    to_dir_override: &str,
) -> String {
    let empty = HashMap::new();
    let from = translations
        .get(from_lang)
        .map(|t| &t.values)
        .unwrap_or(&empty);
    // keys sorted by descending from-value length so longer strings match
    // first (longest-prefix problem); missing from-side sorts last
    let mut keys: Vec<&String> = to_values.keys().collect();
    keys.sort_by(|a, b| {
        let la = from.get(*a).map(|s| s.len()).unwrap_or(0);
        let lb = from.get(*b).map(|s| s.len()).unwrap_or(0);
        // tiebreak on the key: equal-length duplicate ar values in
        // insertion order are HashMap-order random per process, which made
        // which twin "won" (and the unmatched-warning list) nondeterministic
        lb.cmp(&la).then_with(|| a.cmp(b))
    });
    let mut unmatched: Vec<String> = Vec::new();
    let mut out = arabic_html.to_string();
    for key in keys {
        let from_val = from.get(key.as_str()).map(String::as_str).unwrap_or("");
        let to_val = to_values
            .get(key.as_str())
            .map(String::as_str)
            .unwrap_or("");
        if !from_val.is_empty() && !to_val.is_empty() && from_val != to_val {
            if !out.contains(from_val) {
                unmatched.push(key.clone());
                continue;
            }
            out = out.replace(from_val, to_val);
        } else if !to_val.is_empty() && from_val.is_empty() {
            unmatched.push(format!("{}(no {} source)", key, from_lang));
        }
    }
    if !unmatched.is_empty() {
        eprintln!(
            "warn {}: translation keys not found in HTML (left as {}): {}",
            to_lang,
            from_lang,
            unmatched.join(", ")
        );
    }
    let mut lang_dir = to_dir_override.to_string();
    if lang_dir.is_empty() {
        lang_dir = translations
            .get(to_lang)
            .map(|t| t.dir.clone())
            .unwrap_or_default();
    }
    if lang_dir.is_empty() {
        lang_dir = "ltr".to_string();
    }
    // <html lang>: replace or inject
    if RE_HTML_HAS_LANG.is_match(&out) {
        out = RE_HTML_LANG_ATTR
            .replace_all(&out, format!("${{1}}{}${{2}}", to_lang))
            .into_owned();
    } else {
        out = RE_HTML_OPEN_TAG
            .replace_all(&out, |m: &regex::Captures| {
                let m0 = m.get(0).unwrap().as_str();
                if !m0.ends_with('>') {
                    m0.to_string()
                } else {
                    format!("{} lang=\"{}\">", &m0[..m0.len() - 1], to_lang)
                }
            })
            .into_owned();
    }
    // every dir attribute (attribute-boundary anchored: a bare global would
    // also rewrite data-dir="ltr")
    out = RE_DIR_ATTR
        .replace_all(&out, format!("${{1}}dir=\"{}\"", lang_dir))
        .into_owned();
    out
}

/// The RTL demo filename convention this module writes: `-rtl-<lang>.html`
/// with lang in {"he", "en"} for every demo plus "fa" for alert. Gates and
/// sweeps classify pages through here so the set lives in one place.
pub fn rtl_page_lang(name: &str) -> Option<&'static str> {
    static R: OnceLock<Regex> = OnceLock::new();
    let re = R.get_or_init(|| Regex::new(r"-rtl-(en|he|fa)\.html$").unwrap());
    re.captures(name)
        .map(|c| match c.get(1).map(|m| m.as_str()) {
            Some("he") => "he",
            Some("fa") => "fa",
            _ => "en",
        })
}

/// The engine's own Persian dictionary (the Go source this used to live in
/// is gone); overlay's rtl:persian-dictionary audit enumerates these keys.
pub fn persian() -> HashMap<String, String> {
    [
        ("paymentTitle", "پرداخت موفق"),
        (
            "paymentDescription",
            "پرداخت حضرت به مبلغ ۲۹.۹۹ دلار با موفقیت انجام شد. رسید نیز به نشانی پست الکترونیکی شما ارسال گردید.",
        ),
        ("featureTitle", "ویژگی جدید موجود است"),
        (
            "featureDescription",
            "ما پشتیبانی از حالت تیره را به سیستم افزوده‌ایم. می‌توانید این قابلیت را از بخش تنظیمات حساب کاربری خود فعال نمایید.",
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
}

pub fn run_build_rtl() -> i32 {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("build-rtl: {}", e);
            return 1;
        }
    };
    let dict_b = match std::fs::read_to_string(root.join("src/registry/rtl-translations.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("build-rtl: {}", e);
            return 1;
        }
    };
    let dict: HashMap<String, HashMap<String, Translation>> = match serde_json::from_str(&dict_b) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("build-rtl: dict: {}", e);
            return 1;
        }
    };
    let tiers_b = match std::fs::read_to_string(root.join("src/registry/tiers.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("build-rtl: {}", e);
            return 1;
        }
    };
    #[derive(Deserialize)]
    struct TierEntry {
        #[serde(default)]
        tier: String,
        #[serde(default)]
        emit: bool,
    }
    // fail loud: an unparsable tiers.json degrading to an empty map made
    // every preview silently classify as skipped and the run exit 0
    let tiers: HashMap<String, TierEntry> = match serde_json::from_str(&tiers_b) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("build-rtl: tiers.json: {}", e);
            return 1;
        }
    };
    let shipped = |name: &str| -> bool {
        match tiers.get(name.trim_end_matches("-rtl")) {
            Some(t) => t.emit || t.tier == "static" || t.tier == "kernel" || t.tier == "trivial-js",
            None => false,
        }
    };

    for d in ["docs/demos", "dist/components", "build"] {
        if let Err(e) = std::fs::create_dir_all(root.join(d)) {
            eprintln!("build-rtl: {}", e);
            return 1;
        }
    }

    let mut names: Vec<String> = dict.keys().cloned().collect();
    names.sort();

    struct Pending {
        path: std::path::PathBuf,
        html: String,
    }
    let mut pending_writes: Vec<Pending> = Vec::new();
    let mut failures: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    let mut manifest: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut emitted = 0usize;

    for name in &names {
        let existing = format!("docs/demos/{}.html", name);
        let b = match std::fs::read_to_string(root.join(&existing)) {
            Ok(b) => b,
            Err(_) => {
                if shipped(name) {
                    failures.push(format!(
                        "{}: no {} to translate from, but the component ships",
                        name, existing
                    ));
                } else {
                    skipped.push(name.clone());
                }
                continue;
            }
        };
        let empty = HashMap::new();
        let ar = dict.get(name).unwrap_or(&empty);

        let mut langs = vec!["ar".to_string()];
        let mut emit_lang = |lang: &str, values: &HashMap<String, String>, dir_override: &str| {
            let html = inject_pre_paint(&substitute_and_patch(
                &b,
                ar,
                "ar",
                lang,
                values,
                dir_override,
            ));
            for dst in [
                root.join(format!("docs/demos/{}-{}.html", name, lang)),
                root.join(format!("dist/components/{}-{}.html", name, lang)),
            ] {
                pending_writes.push(Pending {
                    path: dst,
                    html: html.clone(),
                });
            }
            emitted += 1;
            langs.push(lang.to_string());
        };
        for lang in ["he", "en"] {
            if let Some(entry) = ar.get(lang) {
                emit_lang(lang, &entry.values, "");
            }
        }
        if name == "alert-rtl" {
            emit_lang("fa", &persian(), "rtl");
        }
        manifest.insert(name.clone(), langs);
    }

    if !failures.is_empty() {
        for f in &failures {
            if let Some(i) = f.find(": ") {
                eprintln!("FAIL [{}]: {}", &f[..i], &f[i + 2..]);
            }
        }
        eprintln!(
            "FAIL  build-rtl ({} previews could not be built) — nothing written",
            failures.len()
        );
        return 1;
    }
    for p in &pending_writes {
        if let Err(e) = std::fs::write(&p.path, &p.html) {
            eprintln!("build-rtl: {}", e);
            return 1;
        }
    }
    // stale RTL pages would otherwise survive silently: nothing else sweeps
    // docs/demos (reproducible cannot see a COMMITTED file that stopped
    // regenerating, and demo-smoke excludes the RTL variants from its
    // count), while a dictionary typo turns a component into a "skipped"
    // entry. Every dictionary component regenerates here, so anything
    // -rtl- shaped that this run did not produce is stale by construction.
    let produced: std::collections::HashSet<std::path::PathBuf> =
        pending_writes.iter().map(|p| p.path.clone()).collect();
    // only this run's languages may name a deletable file: a bare `-rtl-`
    // match is wider than what build_rtl writes (-rtl-<lang>)
    let mut langs: Vec<String> = Vec::new();
    for ls in manifest.values() {
        for l in ls {
            // "ar" is the SOURCE language — no *-rtl-ar file is ever written
            if l != "ar" && !langs.contains(l) {
                langs.push(l.clone());
            }
        }
    }
    langs.sort();
    for dir in ["docs/demos", "dist/components"] {
        let Ok(ents) = std::fs::read_dir(root.join(dir)) else {
            continue;
        };
        for e in ents.flatten() {
            let p = e.path();
            let name = e.file_name().to_string_lossy().into_owned();
            let this_run_lang = langs
                .iter()
                .any(|l| name.ends_with(&format!("-{}.html", l)));
            if !name.ends_with(".html")
                || !name.contains("-rtl-")
                || !this_run_lang
                || produced.contains(&p)
            {
                continue;
            }
            if let Err(e) = std::fs::remove_file(&p) {
                eprintln!("build-rtl: removing stale {}: {}", p.display(), e);
                return 1;
            }
            println!("build-rtl: removed stale {}", p.display());
        }
    }
    let mb = match serde_json::to_string_pretty(&manifest) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("build-rtl: {}", e);
            return 1;
        }
    };
    if let Err(e) = std::fs::write(root.join("build/rtl-langs.json"), mb) {
        eprintln!("build-rtl: {}", e);
        return 1;
    }
    print!(
        "build-rtl: {} language variants emitted (excluding ar default) + manifest for {} previews",
        emitted,
        manifest.len()
    );
    if !skipped.is_empty() {
        print!(
            " ({} dictionaries have no shipped page: {})",
            skipped.len(),
            skipped.join(", ")
        );
    }
    println!();
    0
}
