//! Port of pipeline/product_css.go — derive the npm-consumable product
//! surface from the pipeline's demo-oriented artifacts.
//!
//!   dist/shadless-core.css        theme vars + @theme + custom variants +
//!                                 @utility helpers + keyframes (NO docs-site
//!                                 chrome, NO demo @source, NO demo body pad)
//!   dist/shadless.product.css     tokens + fixes + all parts (tailwind input)
//!
//! SHADLESS_CSS_FIXES is lifted out of src/docs/theme-prepaint.mjs by regex —
//! as TEXT, the way its JS predecessor did: a read, not a second
//! implementation.

use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

static RE_IMPORT_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^@import .+;$").unwrap());
static RE_DARK_VARIANT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^@custom-variant dark [^\n]+$").unwrap());
static RE_BLOCK_OPEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"@theme[^{]*\{|(^|\n)[ \t]*(:root|\.dark)\s*\{").unwrap());
static RE_BORDER_RESET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\n[ \t]*\*[ \t]*\{[^}]*@apply border-border[^}]*\}").unwrap());
static RE_STRAY_IMPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^@import ("[^"]+"|url\([^)]*\));?$"#).unwrap());
static RE_SLOT_SELECTOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\[data-slot="([^"]+)"\]"#).unwrap());
static RE_STANDALONE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^  \.([^ \t\n\f\r]+) \{$").unwrap());
static RE_CSS_ESCAPE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\(.)").unwrap());
static RE_DATA_ATTR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[data-[^\]]*\]").unwrap());
static RE_CSS_FIXES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("export const SHADLESS_CSS_FIXES = `([^`]*)`").unwrap());

const SHADCN_MARKER_END: &str = "/* === end inlined shadcn/tailwind.css === */";

/// Pulls a balanced block (at-rule or selector) starting at the index of its
/// opening brace.
fn take_block(text: &str, open_idx: usize) -> Result<String, String> {
    let b = text.as_bytes();
    let mut depth = 0i32;
    for i in open_idx..b.len() {
        match b[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(text[open_idx..i + 1].to_string());
                }
            }
            _ => {}
        }
    }
    Err("unbalanced block".to_string())
}

/// Returns the at-rule/selector text from line start up to the brace.
fn header_of(text: &str, brace_idx: usize) -> String {
    let start = text[..brace_idx].rfind('\n').map(|i| i + 1).unwrap_or(0);
    text[start..brace_idx].trim().to_string()
}

/// The keep-list: the narrow product-relevant subset of a globals.css that
/// mixes library config with oracle-site chrome.
fn extract_tokens(globals: &str) -> Result<String, String> {
    let mut keep: Vec<String> = Vec::new();
    // 1. @import lines (tailwindcss + tw-animate-css)
    keep.extend(
        RE_IMPORT_LINE
            .find_iter(globals)
            .map(|m| m.as_str().to_string()),
    );
    // 2. the inlined shadcn library tailwind.css (marker-bounded)
    let begin = globals.find("/* === begin inlined shadcn/tailwind.css === */");
    let end = globals.find(SHADCN_MARKER_END);
    let (Some(begin), Some(end)) = (begin, end) else {
        return Err("inlined shadcn/tailwind.css markers missing".to_string());
    };
    keep.push(globals[begin..end + SHADCN_MARKER_END.len()].to_string());
    // 3. dark-mode custom variant (line-scoped exact grab)
    if let Some(dark) = RE_DARK_VARIANT.find(globals) {
        keep.push(dark.as_str().to_string());
    }
    // 4. walk top-level-ish at-rule/selector blocks with balanced braces;
    //    keep only the product-relevant ones
    for loc in RE_BLOCK_OPEN.find_iter(globals) {
        let brace_idx = loc.end() - 1;
        let header = header_of(globals, brace_idx);
        let body = take_block(globals, brace_idx)?;
        if header.starts_with("@theme") && body.contains("--color-background:") {
            keep.push(format!("{}{}", header, body));
        } else if (header == ":root" || header == ".dark") && body.contains("--background:") {
            keep.push(format!("{} {}", header, body));
        }
    }
    // 5. the base border/outline reset rule (indented inside @layer base)
    if let Some(star) = RE_BORDER_RESET.find(globals) {
        keep.push(format!("@layer base {{{}\n}}", star.as_str().trim()));
    }
    Ok(format!("{}\n", keep.join("\n\n")))
}

fn build_product_entry(tokens_css: &str, fixes_css: &str, parts_css: &str) -> String {
    format!("{}\n{}\n{}\n", tokens_css, fixes_css, parts_css)
}

/// The [data-slot="…"] names in first-occurrence order, which is what the JS
/// Set iteration produced and what the failure messages list.
fn slot_set(css: &str) -> (Vec<String>, HashMap<String, bool>) {
    let mut order: Vec<String> = Vec::new();
    let mut seen: HashMap<String, bool> = HashMap::new();
    for m in RE_SLOT_SELECTOR.captures_iter(css) {
        if !seen.contains_key(&m[1]) {
            seen.insert(m[1].to_string(), true);
            order.push(m[1].to_string());
        }
    }
    (order, seen)
}

pub struct ProductReport {
    pub missing: Vec<String>,
    pub demo_dropped: Vec<String>,
    pub chrome: Vec<String>,
    pub tokens: Vec<String>,
    pub stray: Vec<String>,
}

/// Every [data-slot="…"] selector in the per-component PARTS must survive
/// compilation in BOTH chains. out.css legitimately carries extra docs-site
/// slots — those must NOT appear in the product build (chrome check). The
/// stray-class check enforces hermeticity: content scanning must not leak.
pub fn verify_product(
    full_css: &str,
    out_css: &str,
    parts_css: &str,
    product_source: &str,
) -> ProductReport {
    let (expected, _) = slot_set(parts_css);
    let (_, full_slots) = slot_set(full_css);
    let (_, out_slots) = slot_set(out_css);
    let mut missing = Vec::new();
    let mut demo_dropped = Vec::new();
    for s in &expected {
        if !full_slots.contains_key(s) {
            missing.push(s.clone());
        }
        if !out_slots.contains_key(s) {
            demo_dropped.push(s.clone());
        }
    }
    let mut chrome = Vec::new();
    for needle in [
        "rehype",
        "typeset",
        "dialog-ring",
        "style-vega",
        "data-wrapper",
        "[data-slot=\"docs\"]",
        "[data-slot=\"layout\"]",
        "[data-slot=\"copy-button\"]",
    ] {
        if full_css.contains(needle) {
            chrome.push(needle.to_string());
        }
    }
    // runtime vars the compiled output must carry. NOTE: --color-* aliases are
    // compile-time only under `@theme inline` — never literal in a correct
    // build.
    let mut tokens = Vec::new();
    for tok in ["--background:", "--radius:"] {
        if !full_css.contains(tok) {
            tokens.push(tok.to_string());
        }
    }
    // compiled @apply of VARIANT-qualified utilities emits the variant into
    // the selector — compare the base class too before calling it stray
    let mut stray = Vec::new();
    for m in RE_STANDALONE.captures_iter(full_css) {
        let cls = RE_CSS_ESCAPE_RE.replace_all(&m[1], "$1").into_owned();
        let base = RE_DATA_ATTR.replace_all(&cls, "").into_owned();
        // token-boundary: `focus` inside the legitimate `focus-visible`
        // must not vouch for a leaked standalone class
        if !super::css_contains_token(product_source, &cls)
            && !super::css_contains_token(product_source, &base)
        {
            stray.push(cls);
        }
    }
    ProductReport {
        missing,
        demo_dropped,
        chrome,
        tokens,
        stray,
    }
}

/// Lists dist/css/*.css minus the aggregate, sorted.
fn part_files(root: &Path) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    for name in crate::fsutil::sorted_read_dir(&root.join("dist/css"))? {
        if name.ends_with(".css") && name != "shadless.css" {
            out.push(name);
        }
    }
    Ok(out)
}

pub fn run_product_css() -> i32 {
    let wd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("product-css: {}", e);
            return 1;
        }
    };
    let root = match crate::emit::tw::find_repo_root(&wd) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("product-css: {}", e);
            return 1;
        }
    };
    let fail = |err: String| -> i32 {
        eprintln!("product-css: {}", err);
        1
    };
    let read = |p: &str| -> Result<String, String> {
        std::fs::read_to_string(root.join(p)).map_err(|e| e.to_string())
    };

    // dist/globals.css (composed by the demo chain) when present; the probe
    // capture alone predates the skin and cannot supply the @utility cn-*
    // defs the product slot rules @apply.
    let mut globals_path = "dist/globals.css".to_string();
    if !root.join(&globals_path).exists() {
        globals_path = "probes/h4/globals.css".to_string();
    }
    let mut globals = match read(&globals_path) {
        Ok(g) => g,
        Err(e) => return fail(e),
    };
    // the demo entry turns tailwind's automatic content detection off; the
    // CONSUMER's build must keep detection on
    globals = globals.replacen(
        "@import \"tailwindcss\" source(none);",
        "@import \"tailwindcss\";",
        1,
    );

    if let Err(e) = std::fs::create_dir_all(root.join("dist/css")) {
        return fail(e.to_string());
    }
    // tw-animate-css is INLINED into the product surface: the consumer story
    // is "two @imports + your tailwind build"
    let animate = match read("node_modules/tw-animate-css/dist/tw-animate.css") {
        Ok(a) => a,
        Err(e) => return fail(e),
    };
    let mut tokens = match extract_tokens(&globals) {
        Ok(t) => t,
        Err(e) => return fail(e),
    };
    let inline = "/* === begin inlined tw-animate-css (self-contained product surface) === */\n"
        .to_string()
        + animate.trim()
        + "\n/* === end inlined tw-animate-css === */";
    tokens = tokens.replacen("@import \"tw-animate-css\";", &inline, 1);
    if tokens.contains("@import \"tw-animate-css\";") {
        return fail("tw-animate-css import not replaced".to_string());
    }
    let mut stray_imports: Vec<String> = Vec::new();
    for m in RE_STRAY_IMPORT.captures_iter(&tokens) {
        if &m[1] != "\"tailwindcss\"" {
            stray_imports.push(m[1].to_string());
        }
    }
    if !stray_imports.is_empty() {
        return fail(format!(
            "shadless.css not self-contained — unresolved @import(s): {}",
            stray_imports.join(", ")
        ));
    }
    let core = "/* shadless theme — extracted from probes/h4/globals.css by pipeline/src/emit/product_css.rs.\n   Product surface ONLY: theme vars, @theme, custom variants, @utility helpers,\n   keyframes. Deliberately excluded: docs-site chrome (prose/steps/packs),\n   demo @source and demo body padding. The only @import left is \"tailwindcss\"\n   itself — the animate layer is inlined so consumers need nothing else. */\n".to_string() + &tokens;
    if let Err(e) = std::fs::write(root.join("dist/shadless-core.css"), core.clone()) {
        return fail(e.to_string());
    }

    // product entry = tokens + fixes + per-component parts (written by demo)
    let prepaint = match read("src/docs/theme-prepaint.mjs") {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    let Some(m) = RE_CSS_FIXES.captures(&prepaint) else {
        return fail("SHADLESS_CSS_FIXES not found in src/docs/theme-prepaint.mjs".to_string());
    };
    let names = match part_files(&root) {
        Ok(n) => n,
        Err(e) => return fail(e),
    };
    let mut parts: Vec<String> = Vec::new();
    for n in &names {
        match read(&format!("dist/css/{}", n)) {
            Ok(s) => parts.push(s.trim().to_string()),
            Err(e) => return fail(e),
        }
    }
    if parts.is_empty() {
        return fail("dist/css has no per-component files — run the demo chain first".to_string());
    }
    let entry = build_product_entry(&core, &m[1], &parts.join("\n\n"));
    if let Err(e) = std::fs::write(root.join("dist/shadless.product.css"), entry) {
        return fail(e.to_string());
    }
    println!(
        "product-css: shadless-core.css + shadless.product.css ({} component parts)",
        parts.len()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// part_files order reaches committed bytes (dist/shadless.product.css is
    /// the joined parts). readdir order is filesystem-dependent — btrfs
    /// hashes, ext4 inserts, tmpfs inserts — so the list must be sorted
    /// explicitly, never walked.
    #[test]
    fn unit_part_files_is_sorted_and_skips_the_aggregate() {
        let dir = tempfile::Builder::new()
            .prefix("shadless-part-files-")
            .tempdir()
            .unwrap();
        let css = dir.path().join("dist/css");
        std::fs::create_dir_all(&css).unwrap();
        for n in [
            "button.css",
            "alert.css",
            "alert-dialog.css",
            "badge.css",
            "card.css",
            "input.css",
            "separator.css",
            "table.css",
            "tooltip.css",
            "avatar.css",
        ] {
            std::fs::write(css.join(n), "/* part */\n").unwrap();
        }
        std::fs::write(css.join("shadless.css"), "/* aggregate */\n").unwrap();
        let names = part_files(dir.path()).unwrap();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(
            names, sorted,
            "part_files must return sorted names (readdir order is filesystem-dependent)"
        );
        assert!(names.iter().all(|n| n != "shadless.css"));
    }
}
