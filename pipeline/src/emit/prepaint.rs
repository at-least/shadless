//! Port of pipeline/prepaint.go — the constants lifted out of
//! src/docs/theme-prepaint.mjs (the Go-side single source of truth; a lint in
//! the emitter's test asserts the JS file carries the same constants verbatim).

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
