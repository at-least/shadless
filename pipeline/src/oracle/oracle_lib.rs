//! Port of pipeline/oracle_lib.go — the React oracle render environment
//! shared by example-golden / example-oracle / example-fixture / contracts.
//! Render env = upstream's provider stack (TooltipProvider +
//! DirectionProvider for -rtl demos); the entry template closes the crash
//! holes: flushSync propagates render exceptions into __err, and an empty
//! #root is itself an error.
//!
//! Bundle cache: node_modules/.cache/shadless/oracle. The key hashes the
//! process-invariant inputs (pin commit, lockfile, skin.mjs, every stub)
//! plus the per-example tsx. Until the Go engine's removal (2026-09) the two
//! .go implementation files were hashed too, byte-identically to Go's key so
//! both engines could share one cache; the Rust implementation files are now
//! covered by the engine fingerprint instead, so the invariant no longer
//! reads any Go source (the key changed once at that cut-over — every warm
//! oracle bundle re-bundled exactly once).

use super::browser_shell::BPage;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub const ORACLE_CANON: &str = include_str!("oracle_canon.js");

pub(crate) fn oracle_cache_dir() -> PathBuf {
    if let Ok(d) = std::env::var("SHADLESS_CACHE") {
        return PathBuf::from(d);
    }
    PathBuf::from("node_modules/.cache/shadless/oracle")
}

/// For callers that join against the repo root themselves.
pub(crate) fn oracle_cache_dir_relative() -> PathBuf {
    oracle_cache_dir()
}

fn oracle_invariant_once() -> &'static Result<Vec<u8>, String> {
    static INV: OnceLock<Result<Vec<u8>, String>> = OnceLock::new();
    INV.get_or_init(|| {
        let mut h = Sha256::new();
        let pin_b = std::fs::read_to_string("src/registry/pin.json").map_err(|e| e.to_string())?;
        let pin: serde_json::Value = serde_json::from_str(&pin_b).map_err(|e| e.to_string())?;
        let commit = pin["shadcn_ui"]["commit"]
            .as_str()
            .ok_or("pin.json: missing shadcn_ui.commit")?;
        h.update((commit.to_string() + "\n").as_bytes());
        // The oracle group's engine fingerprint: with the Go implementation
        // files gone from this hash, this is what invalidates warm bundles
        // built by older oracle code when the alias table, entry template or
        // bundler glue changes. (The esbuild/rolldown choice is separately
        // isolated by per-bundler key files below.)
        let oracle_fp = env!("ENGINE_FPS")
            .split(';')
            .find_map(|p| p.strip_prefix("oracle="))
            .ok_or("ENGINE_FPS: no oracle entry")?;
        h.update(oracle_fp.as_bytes());
        for f in ["package-lock.json", "src/emitter/skin.mjs"] {
            let b = std::fs::read(f).map_err(|e| e.to_string())?;
            h.update(&b);
        }
        let mut entries: Vec<String> = Vec::new();
        for e in std::fs::read_dir("tools/contracts/stubs").map_err(|e| e.to_string())? {
            let e = e.map_err(|e| e.to_string())?;
            entries.push(e.file_name().to_string_lossy().into_owned());
        }
        entries.sort();
        for n in &entries {
            let b = std::fs::read(Path::new("tools/contracts/stubs").join(n))
                .map_err(|e| e.to_string())?;
            h.update(n.as_bytes());
            h.update(&b);
        }
        // The bundles inline the RESOLVED registry tree (alias
        // @/registry/bases/radix/ui → build/resolved-ui): hashing the pin
        // commit string alone let a warm bundle outlive the sources it
        // inlined whenever the resolved bytes moved without a re-pin (a
        // tiers.json reclassification, a skins or kernel-page change that
        // convert folds in). Content-hash the tree, sorted for determinism.
        let mut resolved: Vec<PathBuf> = walkdir::WalkDir::new("build/resolved-ui")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .collect();
        resolved.sort();
        if resolved.is_empty() {
            return Err("build/resolved-ui is empty or missing — run the convert node first".to_string());
        }
        for f in &resolved {
            let b = std::fs::read(f).map_err(|e| format!("{}: {}", f.display(), e))?;
            h.update(f.to_string_lossy().as_bytes());
            h.update([0u8]);
            h.update(&b);
            h.update([0u8]);
        }
        Ok(h.finalize().to_vec())
    })
}

/// The process-invariant hash above, plus the one per-example input that
/// varies with `name`.
pub fn oracle_bundle_cache_key(name: &str) -> Result<String, String> {
    let inv = oracle_invariant_once().clone()?;
    let tsx = std::fs::read(Path::new(".upstream/shadcn-ui/apps/v4/examples/radix").join(format!("{}.tsx", name)))
        .map_err(|e| e.to_string())?;
    let mut h = Sha256::new();
    h.update(inv);
    h.update(&tsx);
    Ok(hex::encode(h.finalize()))
}

/// Examples import GENERATED style dirs @/styles/<flavor>-<skin>/ui[-rtl]/*;
/// the resolved tree is their tracked equivalent.
pub fn oracle_aliases() -> Result<HashMap<String, String>, String> {
    let skins = ["nova", "vega", "lyra", "maia", "mira", "luma", "sera", "rhea"];
    let abs = |p: &str| -> String {
        std::fs::canonicalize(p)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| {
                // filepath.Abs does not require existence; fall back to the
                // joined path against the cwd
                if Path::new(p).is_absolute() {
                    p.to_string()
                } else {
                    std::env::current_dir()
                        .map(|d| d.join(p).to_string_lossy().into_owned())
                        .unwrap_or_else(|_| p.to_string())
                }
            })
    };
    let resolved = abs("build/resolved-ui");
    let up = abs(".upstream/shadcn-ui/apps/v4");
    let mut a: HashMap<String, String> = HashMap::from([
        ("@".to_string(), up.clone()),
        ("@/registry/bases/radix/ui".to_string(), format!("{}/ui", resolved)),
        ("@/registry/bases/radix/lib".to_string(), format!("{}/lib", resolved)),
        ("@/registry/bases/radix/hooks".to_string(), format!("{}/hooks", resolved)),
        (
            "@/components/language-selector".to_string(),
            abs("tools/contracts/stubs/app-components.jsx"),
        ),
        (
            "@/components/markdown".to_string(),
            abs("tools/contracts/stubs/app-components.jsx"),
        ),
        (
            "@/components/message-animated".to_string(),
            abs("tools/contracts/stubs/message-animated.jsx"),
        ),
        // route-group indirection + subtree cut: ui components import the
        // demo-app icon switcher, which pulls next/navigation + nuqs into the
        // oracle bundle — stub it
        (
            "@/app/(create)/components/icon-placeholder".to_string(),
            abs("tools/contracts/stubs/icon-placeholder.jsx"),
        ),
        ("next/image".to_string(), abs("tools/contracts/stubs/next-image.jsx")),
        ("next/link".to_string(), abs("tools/contracts/stubs/next-link.jsx")),
        ("date-fns".to_string(), abs("tools/contracts/stubs/date-fns.mjs")),
        ("sonner".to_string(), abs("tools/contracts/stubs/sonner.jsx")),
        (
            "embla-carousel-autoplay".to_string(),
            abs("tools/contracts/stubs/embla-autoplay.mjs"),
        ),
        (
            "react-textarea-autosize".to_string(),
            abs("tools/contracts/stubs/textarea-autosize.jsx"),
        ),
    ]);
    for flavor in ["radix", "base", "aria"] {
        for s in skins {
            a.insert(format!("@/styles/{}-{}/ui", flavor, s), format!("{}/ui", resolved));
            a.insert(
                format!("@/styles/{}-{}/ui-rtl", flavor, s),
                format!("{}/ui-rtl", resolved),
            );
        }
    }
    Ok(a)
}

/// The oracle-bundle bundler choice. Default: rolldown when the binary was
/// built with the `oxc` feature (the no-byte-contract point runs the pure
/// Rust toolchain), esbuild otherwise. `SHADLESS_ORACLE_BUNDLER=esbuild|oxc`
/// overrides either way. The gate lives inside the oracle group on purpose —
/// a cargo feature folds into the hull fingerprint and would stale the whole
/// graph on every toggle.
///
/// History (2026-09-17): the rolldown path shipped bundling TWO react
/// instances — example_oracle passes root = Path::new("."), the entry
/// reached rolldown relative ("./node_modules/…"), and react imported from
/// that relative graph resolved as a second instance beside the one the
/// absolutely-aliased .upstream imports see; 332/343 oracle renders failed
/// with "Invalid hook call" → empty #root. bundle_oracle_oxc canonicalizes
/// the entry now; regression-tested by
/// oxc_bundle::tests::build_oracle_via_relative_root_bundles_one_react.
fn oracle_bundler_use_oxc() -> Result<bool, String> {
    match std::env::var("SHADLESS_ORACLE_BUNDLER").as_deref() {
        Ok("oxc") => {
            if cfg!(feature = "oxc") {
                Ok(true)
            } else {
                Err(
                    "SHADLESS_ORACLE_BUNDLER=oxc needs a binary built with --features oxc"
                        .to_string(),
                )
            }
        }
        Ok("esbuild") => Ok(false),
        Ok(other) => Err(format!(
            "SHADLESS_ORACLE_BUNDLER: unknown value {other:?} (expected esbuild|oxc)"
        )),
        Err(std::env::VarError::NotPresent) => Ok(cfg!(feature = "oxc")),
        Err(e) => Err(format!("SHADLESS_ORACLE_BUNDLER: {e}")),
    }
}

/// Bundles the pinned example and writes the oracle page. Returns the
/// htmlFile path — goto it, then await_oracle. The esbuild Build API call is
/// realized through the pinned CLI with one repeated --alias:from=to flag per
/// alias (same engine; the bundle is browser-executed, so byte-identity of
/// the bundle itself is not the bar — the rendered DOM is).
pub fn build_oracle(root: &Path, name: &str, tmp: &Path) -> Result<PathBuf, String> {
    let dir = if name.ends_with("-rtl") { "rtl" } else { "ltr" };
    // Go worked with root-relative paths and filepath.Rel; mirror that by
    // making both sides absolute against root and computing the relative
    // script path from the tmp dir.
    let tmp_abs = if tmp.is_absolute() {
        tmp.to_path_buf()
    } else {
        root.join(tmp)
    };
    let cache = if oracle_cache_dir().is_absolute() {
        oracle_cache_dir()
    } else {
        root.join(oracle_cache_dir())
    };
    for d in [&tmp_abs, &cache] {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let entry = cache.join(format!(".entry-{}.mjs", name));
    let entry_src = format!(
        r##"
import * as React from "react"
import {{ createRoot }} from "react-dom/client"
import {{ flushSync }} from "react-dom"
import {{ Direction }} from "radix-ui"
import * as Mod from "@/examples/radix/{name}"
import {{ TooltipProvider }} from "@/registry/bases/radix/ui/tooltip"
const Demo = Mod.default ?? Object.values(Mod).find((v) => typeof v === "function")
try {{
  const root = createRoot(document.getElementById("root"))
  flushSync(() => root.render(
    React.createElement(TooltipProvider, {{ delayDuration: 0 }},
      React.createElement(Direction.Provider, {{ dir: "{dir}" }}, React.createElement(Demo)))
  ))
  if (!document.querySelector("#root").hasChildNodes()) throw new Error("empty #root after render")
  window.__done = true
}} catch (e) {{ window.__err = String(e?.message ?? e) }}
"##
    );
    std::fs::write(&entry, &entry_src).map_err(|e| e.to_string())?;
    // The oracle cache is shared between the Go and Rust engines ("one cache
    // without ever reusing a bundle the other side would reject"), so the
    // cache key must stay byte-identical with Go's. The experiment-only
    // rolldown path therefore keeps its OWN outfile and key file (prefixed
    // `oxc-`, so no `bundle-*.js` glob can ever sweep them up) and never
    // touches the shared ones.
    let use_oxc = oracle_bundler_use_oxc()?;
    let (outfile, key_file) = if use_oxc {
        (
            cache.join(format!("oxc-bundle-{}.js", name)),
            cache.join(format!(".oxc-key-{}", name)),
        )
    } else {
        (
            cache.join(format!("bundle-{}.js", name)),
            cache.join(format!(".key-{}", name)),
        )
    };
    let key = oracle_bundle_cache_key(name)?;
    let old_key = std::fs::read_to_string(&key_file).unwrap_or_default();
    if old_key != key {
        // stage + rename: golden-gate, example-oracle and example-fixture
        // have no graph edge between them and rebundle the same example
        // names cold — esbuild writing the live outfile directly let a
        // racing goto execute a torn bundle (or a torn bundle land under a
        // matching key file, poisoning the cache). Rename is atomic on the
        // cache filesystem: readers always see a whole file.
        let staging = outfile.with_extension(format!("js.tmp-{}", std::process::id()));
        let aliases = oracle_aliases()?;
        if use_oxc {
            #[cfg(feature = "oxc")]
            crate::oracle::oxc_bundle::bundle_oracle_oxc(root, &entry, &aliases, &staging)?;
            #[cfg(not(feature = "oxc"))]
            unreachable!("gated above");
        } else {
            let mut argv: Vec<String> = vec![
                entry.to_string_lossy().into_owned(),
                "--bundle".into(),
                "--format=iife".into(),
                format!("--outfile={}", staging.to_string_lossy()),
                "--log-level=error".into(),
                "--loader:.tsx=tsx".into(),
                "--jsx=automatic".into(),
            ];
            let mut alias_keys: Vec<&String> = aliases.keys().collect();
            alias_keys.sort();
            for k in alias_keys {
                argv.push(format!("--alias:{}={}", k, aliases[k]));
            }
            let out = std::process::Command::new(root.join("node_modules/.bin/esbuild"))
                .args(&argv)
                .current_dir(root)
                .output()
                .map_err(|e| format!("esbuild: {}", e))?;
            if !out.status.success() {
                let text = String::from_utf8_lossy(&out.stderr);
                let first = text.lines().next().unwrap_or("").to_string();
                return Err(format!("esbuild: {}", first));
            }
        }
        std::fs::rename(&staging, &outfile).map_err(|e| e.to_string())?;
        std::fs::write(&key_file, &key).map_err(|e| e.to_string())?;
    }
    let html_file = tmp_abs.join(format!("oracle-{}.html", name));
    let rel = rel_path(&tmp_abs, &outfile)?;
    let dir_attr = if dir == "rtl" { " dir=\"rtl\"" } else { "" };
    let html = format!(
        "<!doctype html><html{}><head><meta charset=\"utf-8\"></head>\n<body><div id=\"root\"></div><script src=\"{}\"></script></body></html>",
        dir_attr, rel
    );
    std::fs::write(&html_file, html).map_err(|e| e.to_string())?;
    Ok(html_file)
}

/// Waits for the oracle page to settle; the render error surfaces as an error.
pub fn await_oracle(p: &BPage<'_>, html_file: &Path) -> Result<(), String> {
    let abs = std::fs::canonicalize(html_file).map_err(|e| e.to_string())?;
    p.goto_url(&format!("file://{}", abs.to_string_lossy()))?;
    p.wait_for_function("window.__done === true || window.__err !== undefined", 5000)?;
    let err_v = p.evaluate("window.__err")?;
    if let Some(s) = err_v.as_str() {
        if !s.is_empty() {
            return Err(s.to_string());
        }
    }
    Ok(())
}

/// Every spelling React's useId has had, as radix prefixes it — React 18
/// `:r1:`, 19.0 `«r1»`, 19.1+ CSR `_r_1_` and SSR `_R_1H2_`.
fn re_radix_auto_id() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"radix-(?::r[a-z0-9]*:?|«r[a-z0-9]*»|_[rR]_[A-Za-z0-9-]*_?)").unwrap()
    })
}

/// Makes radix auto-ids STABLE without making them EQUAL: each distinct id
/// becomes radix-a1, radix-a2, … in order of first appearance.
pub fn oracle_norm(html: &str) -> String {
    let seen: std::sync::Mutex<HashMap<String, String>> = std::sync::Mutex::new(HashMap::new());
    re_radix_auto_id()
        .replace_all(html, |m: &regex::Captures| {
            let id = m[0].to_string();
            let mut seen = seen.lock().unwrap();
            if let Some(t) = seen.get(&id) {
                return t.clone();
            }
            let t = format!("radix-a{}", seen.len() + 1);
            seen.insert(id, t.clone());
            t
        })
        .into_owned()
}

/// Canons a DOM fragment through the real browser.
pub fn canon_of(p: &BPage<'_>, html: &str) -> Result<String, String> {
    let arg = serde_json::json!([html]);
    let v = p.evaluate_fn_arg(ORACLE_CANON, arg)?;
    serde_json::to_string(&v).map_err(|e| e.to_string())
}

/// The rendered #root innerHTML, normalized.
pub fn oracle_root_html(p: &BPage<'_>) -> Result<String, String> {
    let v = p.evaluate_fn("() => document.querySelector(\"#root\").innerHTML")?;
    let s = v.as_str().unwrap_or("").to_string();
    Ok(oracle_norm(&s))
}

/// Relative path from directory `from` to `to` (both absolute), mirroring
/// filepath.Rel for this use.
pub fn rel_path(from_dir: &Path, to: &Path) -> Result<String, String> {
    let from_comps: Vec<_> = from_dir.components().collect();
    let to_comps: Vec<_> = to.components().collect();
    let mut common = 0;
    while common < from_comps.len() && common < to_comps.len() && from_comps[common] == to_comps[common] {
        common += 1;
    }
    let mut parts: Vec<String> = Vec::new();
    for _ in common..from_comps.len() {
        parts.push("..".to_string());
    }
    for c in &to_comps[common..] {
        parts.push(c.as_os_str().to_string_lossy().into_owned());
    }
    Ok(parts.join("/"))
}

/// absOrDie: absolute path against root, passthrough when already absolute.
pub fn abs_or_die_path(root: &Path, p: &str) -> PathBuf {
    let q = Path::new(p);
    if q.is_absolute() {
        q.to_path_buf()
    } else {
        root.join(q)
    }
}
