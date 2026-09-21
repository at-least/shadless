//! Port of pipeline/tags.go — tag normalization shared by converter/emitter/css.

use regex::Regex;
use std::collections::HashSet;
use std::sync::{LazyLock, OnceLock};

pub fn native_tags() -> &'static HashSet<&'static str> {
    static S: OnceLock<HashSet<&'static str>> = OnceLock::new();
    S.get_or_init(|| {
        "div span p a button h1 h2 h3 h4 h5 h6
ul ol li nav table thead tbody tfoot tr th td caption
input select option optgroup textarea label form img svg path
circle line rect polygon polyline ellipse g defs use
section header footer main article aside small strong em kbd
dl dt dd fieldset legend output datalist meter progress
details summary picture time mark sub sup i b u s
abbr address hgroup dialog search blockquote code pre
template style script title head body html
figure figcaption"
            .split_whitespace()
            .collect()
    })
}

/// HTML5 void elements — never emit a closing tag.
pub fn void_tags() -> &'static HashSet<&'static str> {
    static S: OnceLock<HashSet<&'static str>> = OnceLock::new();
    S.get_or_init(|| {
        "br hr img input meta link area base col embed source track wbr"
            .split_whitespace()
            .collect()
    })
}

fn known_member_tags(tag: &str) -> Option<&'static str> {
    match tag {
        "LabelPrimitive.Root" => Some("label"),
        _ => None,
    }
}

pub fn external_member_tag(tag: &str) -> String {
    if let Some(v) = known_member_tags(tag) {
        return v.to_string();
    }
    let suffix = &tag[tag.rfind('.').map(|i| i + 1).unwrap_or(0)..];
    match suffix {
        "Button" | "Trigger" | "Link" => "button".to_string(),
        _ => "div".to_string(),
    }
}

static KEBAB_BOUNDARY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([a-z0-9])([A-Z])").unwrap());
static KEBAB_SPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\s.]+").unwrap());

pub fn kebab(s: &str) -> String {
    let stepped = KEBAB_BOUNDARY.replace_all(s, "${1}-${2}");
    KEBAB_SPACE.replace_all(&stepped, "-").to_lowercase()
}

pub static TERNARY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^<ternary:([^/]+)/(.+)>$").unwrap());

/// Resolves a raw IR tag to a native tag. Returns None when the tag is
/// unresolvable — the caller must fail loudly.
pub fn normalize_tag(tag: &str, hints: &HashMap<String, String>) -> Option<String> {
    if tag.is_empty() {
        return None;
    }
    if native_tags().contains(tag) {
        return Some(tag.to_string());
    }
    if let Some(m) = TERNARY_RE.captures(tag) {
        return normalize_tag(&m[2], hints);
    }
    if let Some(h) = hints.get(tag) {
        if native_tags().contains(h.as_str()) {
            return Some(h.clone());
        }
        return None;
    }
    None
}

use std::collections::HashMap;
