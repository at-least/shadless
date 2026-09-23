//! The theme pre-paint script and CSS-fix layer, mirrored from
//! src/docs/theme-prepaint.mjs — the single authored source (the emitter's
//! JS twin imports it directly). The mirror is pinned byte-for-byte by the
//! unit test at the end of this file; product_css.rs additionally lifts
//! SHADLESS_CSS_FIXES from the mjs by regex at build time, so demo pages and
//! the product stylesheet meet the same bytes.

pub const THEME_PREPAINT_SCRIPT: &str = r#"<script>(function(){try{var k="shadless-docs-theme";var apply=function(d){document.documentElement.classList.toggle("dark",!!d)};var s=localStorage.getItem(k);var d=s?s==="dark":matchMedia("(prefers-color-scheme: dark)").matches;apply(d);addEventListener("storage",function(e){if(e.key===k)apply(e.newValue==="dark")});}catch(e){}})();</script>"#;
pub const THEME_PREPAINT_SIG: &str = r#"<script>(function(){try{var k="shadless-docs-theme""#;
pub const SHADLESS_CSS_FIXES: &str = "";

/// Prepends to </head> when present, otherwise into the opening <head …>,
/// otherwise at the very front. Idempotent on the SIG prefix.
pub fn inject_pre_paint(html: &str) -> String {
    if html.contains(THEME_PREPAINT_SIG) {
        return html.to_string();
    }
    if html.contains("</head>") {
        return html.replacen(
            "</head>",
            &format!("{}{}", THEME_PREPAINT_SCRIPT, "</head>"),
            1,
        );
    }
    static HEAD_RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let head_re = HEAD_RE.get_or_init(|| Regex::new(r"(?i)<head[^>]*>").unwrap());
    if head_re.is_match(html) {
        return head_re
            .replace_all(html, |m: &regex::Captures| {
                format!("{}{}", &m[0], THEME_PREPAINT_SCRIPT)
            })
            .into_owned();
    }
    format!("{}{}", THEME_PREPAINT_SCRIPT, html)
}

use regex::Regex;

#[cfg(test)]
mod tests {
    /// The pin the header used to claim as a lint: prepaint.rs's copies and
    /// the mjs source of truth must stay byte-identical, or a fix reaches
    /// one consumer (product stylesheet) and not the other (demo pages).
    #[test]
    fn unit_prepaint_constants_match_the_mjs_source_of_truth() {
        let mjs = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../src/docs/theme-prepaint.mjs"),
        )
        .expect("read theme-prepaint.mjs");
        let script = regex::Regex::new("export const THEME_PREPAINT_SCRIPT = `([^`]*)`")
            .unwrap()
            .captures(&mjs)
            .expect("THEME_PREPAINT_SCRIPT export");
        assert_eq!(&script[1], super::THEME_PREPAINT_SCRIPT);
        let fixes = regex::Regex::new("export const SHADLESS_CSS_FIXES = `([^`]*)`")
            .unwrap()
            .captures(&mjs)
            .expect("SHADLESS_CSS_FIXES export");
        assert_eq!(&fixes[1], super::SHADLESS_CSS_FIXES);
    }
}
