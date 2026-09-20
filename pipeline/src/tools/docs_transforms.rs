//! Port of pipeline/docs_transforms.go + docs_frontmatter.go + docs_guides.go
//! + docs_families.go + docs_scripts.go + docs_overrides.go + the pure half of
//! docs_fidelity.go — the SINGLE SOURCE for where each mdx section transform
//! touches the raw source. Two consumers must agree on the span: the builder
//! (docs-build replaces it) and the gate (docs-fidelity drops it).

use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

// ---------------------------------------------------------------- regexes

fn re_fence_open() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("^`{3,}").unwrap())
}
fn re_leading_backticks() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("^`+").unwrap())
}
fn re_h2_installation() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^## Installation$").unwrap())
}
fn re_h2_usage() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^## Usage$").unwrap())
}
fn re_h2_migrating() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^## Migrating existing components$").unwrap())
}
fn re_h2_composition() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^## Composition$").unwrap())
}
fn re_h2_changelog() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^## Changelog$").unwrap())
}
fn re_api_ref_leak() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^[ \t]*\|.*\||^### ").unwrap())
}
fn re_import_from() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"from\s+"[@/][^"]+"\s*;?\s*$"#).unwrap())
}
fn re_tsx_meta() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)```tsx([^\n]*)\n(.*?)```").unwrap())
}
fn re_highlight_list() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\{([0-9,\s-]+)\}").unwrap())
}
fn re_tsx_fence_open() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^```tsx\b.*$").unwrap())
}
fn re_class_name_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\bclassName=").unwrap())
}
fn re_jsx_comment() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\{/\*\s*(.*?)\s*\*/\}").unwrap())
}
fn re_leaked_fence_open() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^```(tsx|jsx|ts)\b.*$").unwrap())
}
fn re_jsx_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"</?([A-Z][A-Za-z0-9]*)\b[^<>]*?(/?)>").unwrap())
}
fn re_icon_fn() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^[A-Z][A-Za-z0-9]*Icon$").unwrap())
}
fn re_camel_word() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"[A-Z][a-z0-9]*").unwrap())
}
fn re_data_slot() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"<([a-zA-Z][0-9A-Za-z_-]*)\b[^>]*\bdata-slot="([^"]+)""#).unwrap())
}
fn re_inline_jsx_mention() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("`</?([A-Z][A-Za-z0-9]*)[^`]*`").unwrap())
}
fn re_jsx_tag_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"([A-Za-z][0-9A-Za-z_:.-]*)(?:="([^"]*)")?"#).unwrap())
}
fn re_frontmatter() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)^---\r?\n(.*?)\r?\n---").unwrap())
}
fn re_top_key() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^([A-Za-z][0-9A-Za-z_-]*):[ \t]*(.*)$").unwrap())
}
fn re_sub_key() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^[ \t]+([A-Za-z][0-9A-Za-z_-]*):[ \t]*(.*)$").unwrap())
}
fn re_int_only() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^-?\d+$").unwrap())
}
fn re_import_named() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(
            r#"(?m)^[ \t]*import[ \t]*(?:\{[\s\S]*?\}[ \t]*|\*(?:[ \t]+as[ \t]+[0-9A-Za-z_$]+)[ \t]*|[0-9A-Za-z_$]+[ \t]*)from[ \t]*["'][^"']+["'];?[ \t]*$"#,
        )
        .unwrap()
    })
}
fn re_import_bare_line() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"(?m)^[ \t]*import[ \t]*["'][^"']+["'];?[ \t]*$"#).unwrap())
}
fn re_fence_line() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("^`{3,}([^`]*)$").unwrap())
}
fn re_entity() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"&(amp|lt|gt|quot);").unwrap())
}
fn re_any_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<[^>]*>").unwrap())
}
fn re_ws() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"[ \t\n\f\r]+").unwrap())
}
fn re_md_heading() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^(#{2,4})[ \t]+(.+)$").unwrap())
}
fn re_h_open() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<h([234])\b[^>]*>").unwrap())
}
fn re_inline_code() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("`([^`\n]+)`").unwrap())
}
fn re_comp_preview() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<ComponentPreview\b([^>]*)>").unwrap())
}
fn re_comp_source() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<ComponentSource\b([^>]*)>").unwrap())
}
fn re_md_h1() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^# (.+)$").unwrap())
}
fn re_frontmatter_block() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)^---\n.*?\n---\n").unwrap())
}
fn re_demo_iframe() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"<iframe class="demo" src="([^"]*)" title="([^"]*)" data-status="([^"]*)""#).unwrap()
    })
}
fn re_demo_missing() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"<div class="demo-missing" data-demo="([^"]*)" data-status="([^"]*)""#).unwrap()
    })
}
fn re_demo_in_order() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"<iframe class="demo"[^>]*title="([^"]*)"|<div class="demo-missing" data-demo="([^"]*)""#)
            .unwrap()
    })
}
fn re_page_links_p() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"(?s)<p class="page-links">(.*?)</p>"#).unwrap())
}
fn re_anchor() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"<a href="([^"]*)"[^>]*>([^<]*)</a>"#).unwrap())
}
fn re_docs_href() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\]\((/docs/[^)]*)\)").unwrap())
}
fn re_all_href() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\]\(([^)]*)\)").unwrap())
}
fn re_react_prop_table() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^\|[ \t]*`?Prop`?[ \t]*\|.*$").unwrap())
}
fn re_slot_open_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"<[a-z][0-9A-Za-z_-]*\b[^<>]*\bdata-slot="[^"]*"[^<>]*>"#).unwrap()
    })
}
fn re_tag_attr_name() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"[ \t]([A-Za-z][0-9A-Za-z_:.-]*)(?:=(?:"[^"]*"|'[^']*'|[^ \t>]*))?"#).unwrap()
    })
}
fn re_comp_route() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^/docs/components/(?:radix/)?([a-z0-9-]+)$").unwrap())
}
fn re_guide_preview_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<ComponentPreview\b([^>]*)>").unwrap())
}
fn re_demo_src_script() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"<script (?:defer )?src="\.\./(js/[0-9A-Za-z_.-]+\.js|shadless\.js)"></script>"#)
            .unwrap()
    })
}
fn re_inline_script() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)<script>(.*?)</script>").unwrap())
}
fn re_fence() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)```.*?```").unwrap())
}
#[allow(dead_code)] // ported from the Go docs transforms; kept for parity
fn re_docs_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<(ComponentPreview|ComponentSource)\b([^>]*)>").unwrap())
}
#[allow(dead_code)] // ported from the Go docs transforms; kept for parity
fn re_preview_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<ComponentPreview\b([^>]*)>").unwrap())
}
#[allow(dead_code)] // ported from the Go docs transforms; kept for parity
fn re_primary() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^([a-z0-9-]+)-demo$").unwrap())
}
#[allow(dead_code)] // ported from the Go docs transforms; kept for parity
fn re_hooks() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(
            r"\b(useState|useEffect|useRef|useContext|useMemo|useCallback|useReducer|useLayoutEffect|useImperativeHandle|useId|useTransition|useDeferredValue|useSyncExternalStore|useInsertionEffect)\b",
        )
        .unwrap()
    })
}

// ---------------------------------------------------------------- fence shadow

/// fenceShadow blanks fenced regions (newlines and offsets preserved) so span
/// searches see only prose/markup. Line-based: 3+ backtick fences, info
/// strings, unclosed fences blank to EOF.
pub fn fence_shadow(text: &str) -> String {
    let mut lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
    let mut open: isize = -1;
    for line in lines.iter_mut() {
        if open < 0 {
            if re_fence_open().is_match(line) {
                open = re_leading_backticks().find(line).map(|m| m.end()).unwrap_or(0) as isize;
                *line = blank_line(line);
            }
        } else {
            let n = re_leading_backticks().find(line).map(|m| m.end()).unwrap_or(0);
            if n as isize >= open {
                open = -1;
            }
            *line = blank_line(line);
        }
    }
    lines.join("\n")
}

fn blank_line(s: &str) -> String {
    " ".repeat(s.len())
}

/// span is a [start, end) byte range into the ORIGINAL text (the caller
/// locates on the shadow and splices on the real string).
#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub fn replace_span(raw: &str, s: Span, replacement: &str) -> String {
    let mut out = String::with_capacity(raw.len() + replacement.len());
    out.push_str(&raw[..s.start]);
    out.push_str(replacement);
    out.push_str(&raw[s.end..]);
    out
}

// ---------------------------------------------------------------- locators

/// locateCodeTabsSpans: every <CodeTabs>…</CodeTabs> outside fences.
pub fn locate_code_tabs_spans(shadow: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    let mut off = 0usize;
    loop {
        let Some(i) = shadow[off..].find("<CodeTabs>") else {
            break;
        };
        let start = off + i;
        let Some(close) = shadow[start..].find("</CodeTabs>") else {
            break;
        };
        let end = start + close + "</CodeTabs>".len();
        spans.push(Span { start, end });
        off = end;
    }
    spans
}

/// locateInstallSection: the `## Installation` … `## Usage` span in utils
/// guides; end = the '#' of "## Usage". None when absent/malformed.
pub fn locate_install_section(shadow: &str) -> Option<Span> {
    let open = re_h2_installation().find(shadow)?;
    let next = re_h2_usage().find(shadow)?;
    if next.start() <= open.start() {
        return None;
    }
    Some(Span {
        start: open.start(),
        end: next.start(),
    })
}

/// locateRtlMigrateSpan: `## Migrating existing components` … </Steps>.
pub fn locate_rtl_migrate_span(shadow: &str) -> Option<Span> {
    let open = re_h2_migrating().find(shadow)?;
    let close = shadow[open.start()..].find("</Steps>")?;
    Some(Span {
        start: open.start(),
        end: open.start() + close + "</Steps>".len(),
    })
}

/// locateUsageSpan: `## Usage` … next `## ` heading (any heading that is not
/// Usage itself). The JS regex /^## (?!Usage$)/m needs a lookahead — scanned
/// by line instead.
pub fn locate_usage_span(shadow: &str) -> Option<Span> {
    let open = re_h2_usage().find(shadow)?;
    let after = &shadow[open.start() + "## Usage".len()..];
    let mut off = 0usize;
    for line in after.split('\n') {
        if line.starts_with("## ") && line.trim() != "## Usage" {
            return Some(Span {
                start: open.start(),
                end: open.start() + "## Usage".len() + off,
            });
        }
        off += line.len() + 1;
    }
    None
}

/// locateCompositionSpan: `## Composition` … next non-Composition `## `.
pub fn locate_composition_span(shadow: &str) -> Option<Span> {
    let open = re_h2_composition().find(shadow)?;
    let after = &shadow[open.start() + "## Composition".len()..];
    let mut off = 0usize;
    for line in after.split('\n') {
        if line.starts_with("## ") && line.trim() != "## Composition" {
            return Some(Span {
                start: open.start(),
                end: open.start() + "## Composition".len() + off,
            });
        }
        off += line.len() + 1;
    }
    None
}

/// locateChangelogSpan: `## Changelog` … EOF.
pub fn locate_changelog_span(shadow: &str) -> Option<Span> {
    let open = re_h2_changelog().find(shadow)?;
    Some(Span {
        start: open.start(),
        end: shadow.len(),
    })
}

/// locateHeadingRangeSpan: `## fromHeading` … `## toHeading` (exclusive of
/// the closing heading).
pub fn locate_heading_range_span(shadow: &str, from_heading: &str, to_heading: &str) -> Option<Span> {
    let open_re = Regex::new(&format!(r"(?m)^## {}$", regex::escape(from_heading))).unwrap();
    let close_re = Regex::new(&format!(r"(?m)^## {}$", regex::escape(to_heading))).unwrap();
    let open = open_re.find(shadow)?;
    let close = close_re.find(&shadow[open.end()..])?;
    Some(Span {
        start: open.start(),
        end: open.end() + close.start(),
    })
}

pub fn locate_rtl_framework_span(shadow: &str) -> Option<Span> {
    locate_heading_range_span(shadow, "Get Started", "Animations")
}

pub fn rtl_framework_note() -> String {
    "## How RTL works here\n\n\
There is no CLI to opt into and no `components.json`. shadless emits a mechanical conversion of the pinned shadcn registry, so where upstream uses a physical utility (`pl-*`, `right-*`, `rounded-l-*`) the shipped stylesheet carries it too — converting them would be a divergence from the oracle the whole port is checked against. The `css-direction` gate holds that inventory to a committed baseline, so a re-pin that moves the RTL story is a visible decision instead of a silent regression.\n\n\
What you get instead: upstream authors a separate RTL example for the components that need one, and shadless ships those as their own pages — `<name>-rtl` beside `<name>-demo`, plus the `-he` / `-fa` / `-en` language variants `build-rtl` emits. Compare `button-group-demo` (`rounded-l-none`) with `button-group-rtl` (`rounded-s-none`) to see the difference an RTL page actually makes.\n\n\
So: set `dir=\"rtl\"` on the page, start from this page's RTL examples rather than the LTR ones, and check the components you use — the ones whose stylesheet is already logical need nothing more. To flip an individual icon, give it the `rtl:rotate-180` utility class.\n\n\
## Font Recommendations\n\n\
Use a font with proper support for your target language. [Noto](https://fonts.google.com/noto) is a good family for this and pairs well with Inter and Geist. shadless ships no fonts and no font configuration — load them the way your own build already loads fonts.\n\n"
        .to_string()
}

pub fn locate_message_scroller_js_span(shadow: &str) -> Option<Span> {
    locate_heading_range_span(shadow, "Core Concepts", "Accessibility")
}

pub fn message_scroller_js_note() -> String {
    "## Core Concepts\n\nshadless's `message-scroller` is pure static markup and CSS — `generated/ir/message-scroller.json` reports tier `static`, and there is no `dist/js/message-scroller.js`. Upstream's virtualization, scroll anchoring, `useMessageScrollerVisibility`, and `scrollToMessage` are all React-only behavior this port does not include; build them yourself on top of the shipped markup if you need them.\n\n".to_string()
}

/// locateApiReferenceSpan: `## API Reference` … next `## ` heading or EOF —
/// ONLY where the section actually contains leaked React content
/// (reApiRefLeak).
pub fn locate_api_reference_span(_comp: &str, shadow: &str) -> Option<Span> {
    let open = Regex::new(r"(?m)^## API Reference[ \t]*\n").unwrap().find(shadow)?;
    let after = &shadow[open.end()..];
    let mut off = 0usize;
    let mut end = shadow.len();
    for line in after.split('\n') {
        if line.starts_with("## ") {
            end = open.end() + off;
            break;
        }
        off += line.len() + 1;
    }
    if !re_api_ref_leak().is_match(&shadow[open.end()..end]) {
        return None;
    }
    Some(Span {
        start: open.start(),
        end,
    })
}

// ---------------------------------------------------------------- import fences

/// dropReactImportFences removes pure React-import fences (content is ONLY
/// import statements from @/components/ui/*).
pub fn drop_react_import_fences(raw: &str) -> String {
    let lines: Vec<&str> = raw.split('\n').collect();
    let mut out: Vec<&str> = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        if !lines[i].starts_with("```tsx") {
            out.push(lines[i]);
            i += 1;
            continue;
        }
        // Match a pure import fence: one or more import statements, each
        // ending on a from "@/…" (or from "/…") line, then IMMEDIATELY the
        // closing fence.
        let mut j = i + 1;
        let mut statements = 0usize;
        let mut failed = false;
        'stmt: while j < lines.len() {
            if !lines[j].starts_with("import") {
                break;
            }
            statements += 1;
            // consume through the from-line, spanning multi-line imports
            while j < lines.len() && !re_import_from().is_match(lines[j]) {
                if lines[j].contains("```") {
                    failed = true;
                    break 'stmt;
                }
                j += 1;
            }
            if j >= lines.len() {
                failed = true;
                break;
            }
            j += 1; // the from-line itself
        }
        if !failed && statements > 0 && j < lines.len() && lines[j].starts_with("```") {
            // the pattern's trailing \n? consumes exactly ONE newline after
            // the closing fence — in line terms: skip the closing line only,
            // never a following blank line.
            i = j + 1;
            continue;
        }
        out.push(lines[i]);
        i += 1;
    }
    out.join("\n")
}

/// stripImportsFromMixedFences strips import statements from ```tsx fences
/// that MIX imports with a JSX example, and renumbers shiki {1,4-6} highlight
/// refs past the removed lines. Imports-only fences drop entirely.
pub fn strip_imports_from_mixed_fences(raw: &str) -> String {
    re_tsx_meta()
        .replace_all(raw, |caps: &regex::Captures| {
            let whole = caps.get(0).unwrap().as_str();
            let meta = caps.get(1).unwrap().as_str();
            let body = caps.get(2).unwrap().as_str();
            let lines: Vec<&str> = body.split('\n').collect();
            let mut removed = 0usize;
            let mut i = 0usize;
            while i < lines.len() {
                let l = lines[i];
                if l.starts_with("import") {
                    removed += 1;
                    i += 1;
                    // NOTE: the JS loop is GREEDY here — for a single-line
                    // import the inner while keeps consuming subsequent lines
                    // until a from-line or EOF. That is load-bearing
                    // (bubble.mdx's mixed fence loses its whole body to it);
                    // do not "fix" it.
                    while i < lines.len() && !re_import_from().is_match(lines[i]) {
                        removed += 1;
                        i += 1;
                    }
                    if i < lines.len() {
                        removed += 1;
                        i += 1;
                    }
                    continue;
                }
                if l.trim().is_empty() && removed > 0 {
                    removed += 1;
                    i += 1;
                    continue;
                }
                break;
            }
            if removed == 0 {
                return whole.to_string();
            }
            let mut kept: Vec<&str> = lines[removed..].to_vec();
            if removed > 0 && removed <= lines.len() && lines[removed - 1].trim().is_empty() {
                kept = lines[removed - 1..].to_vec();
            }
            if kept.join("").trim().is_empty() {
                return String::new(); // fence was imports-only
            }
            let meta_out = shift_highlight_refs(meta, removed);
            format!(
                "```tsx{}\n{}```",
                meta_out.trim_end_matches([' ', '\t']),
                kept.join("\n")
            )
        })
        .into_owned()
}

/// shiftHighlightRefs maps {1,4-6} → refs with `removed` subtracted, dropping
/// refs into the removed range (the JS callback's semantics).
pub fn shift_highlight_refs(meta: &str, removed: usize) -> String {
    re_highlight_list()
        .replace_all(meta, |caps: &regex::Captures| {
            let list = caps.get(1).unwrap().as_str();
            let mut shifted: Vec<String> = Vec::new();
            for part in list.split(',') {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }
                let lohi: Vec<&str> = part.splitn(2, '-').collect();
                let lo = atoi_safe(lohi[0]);
                let hi = if lohi.len() == 2 { atoi_safe(lohi[1]) } else { None };
                // JS semantics: a NaN endpoint keeps the part verbatim; a part
                // whose low end is inside the removed range drops; otherwise
                // both endpoints shift by -removed.
                if lo.is_none() || (lohi.len() == 2 && hi.is_none()) {
                    shifted.push(part.to_string());
                    continue;
                }
                let lo = lo.unwrap();
                if lo <= removed {
                    continue; // range[0] <= removed → null (dropped)
                }
                if let Some(hi) = hi {
                    shifted.push(format!("{}-{}", lo - removed, hi - removed));
                } else {
                    shifted.push(format!("{}", lo - removed));
                }
            }
            if shifted.is_empty() {
                return String::new();
            }
            format!("{{{}}}", shifted.join(","))
        })
        .into_owned()
}

fn atoi_safe(s: &str) -> Option<usize> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let mut n = 0usize;
    for c in s.chars() {
        if !c.is_ascii_digit() {
            return None;
        }
        n = n * 10 + (c as usize - '0' as usize);
    }
    Some(n)
}

// ---------------------------------------------------------------- utility-guide JSX fences

const SHIMMER_MARKER_JSX: &str = "```tsx\n<Marker role=\"status\">\n  <MarkerIcon>\n    <Spinner />\n  </MarkerIcon>\n  <MarkerContent className=\"shimmer\">Thinking&hellip;</MarkerContent>\n</Marker>\n```";
const SHIMMER_MARKER_HTML: &str = "```html\n<div data-slot=\"marker\" role=\"status\">\n  <span data-slot=\"marker-icon\"><!-- Spinner markup — see /components/spinner --></span>\n  <span data-slot=\"marker-content\" class=\"shimmer\">Thinking&hellip;</span>\n</div>\n```";

/// rewriteUtilityJsxFences converts those specific fences to plain HTML: fence
/// lang tsx -> html, className= -> class=, and the JSX comment placeholder
/// {/* ... */} -> an HTML comment.
pub fn rewrite_utility_jsx_fences(slug: &str, raw: &str) -> Result<String, String> {
    let mut raw = raw.to_string();
    if slug == "shimmer" {
        if !raw.contains(SHIMMER_MARKER_JSX) {
            return Err(
                "shimmer guide: the Marker+Spinner fence text moved — re-anchor rewriteUtilityJsxFences"
                    .to_string(),
            );
        }
        raw = raw.replacen(SHIMMER_MARKER_JSX, SHIMMER_MARKER_HTML, 1);
    }
    let mut lines: Vec<String> = raw.split('\n').map(|s| s.to_string()).collect();
    let mut in_fence = false;
    for line in lines.iter_mut() {
        if !in_fence {
            if re_tsx_fence_open().is_match(line) {
                *line = "```html".to_string();
                in_fence = true;
            }
            continue;
        }
        if line.starts_with("```") {
            in_fence = false;
            continue;
        }
        *line = re_class_name_attr().replace_all(line, "class=").into_owned();
        *line = re_jsx_comment()
            .replace_all(line, |caps: &regex::Captures| {
                format!("<!-- {} -->", caps.get(1).unwrap().as_str())
            })
            .into_owned();
    }
    Ok(lines.join("\n"))
}

// ---------------------------------------------------------------- leaked upstream JSX fences

/// htmlAttrs: plain HTML attributes a JSX tag may legitimately carry through
/// to the rewritten markup unchanged.
pub fn html_attrs() -> &'static std::collections::HashSet<&'static str> {
    static S: OnceLock<std::collections::HashSet<&'static str>> = OnceLock::new();
    S.get_or_init(|| {
        [
            "class", "id", "href", "src", "srcset", "alt", "title", "role", "type", "name",
            "value", "placeholder", "disabled", "checked", "hidden", "target", "rel", "style",
            "lang", "dir", "width", "height", "for", "tabindex", "colspan", "rowspan", "action",
            "method", "autocomplete", "required", "readonly", "multiple", "selected", "min", "max",
            "step", "rows", "cols", "loading", "open", "controls", "poster", "download",
            "autofocus", "maxlength",
        ]
        .iter()
        .copied()
        .collect()
    })
}

#[derive(Clone, Default)]
pub struct JsxTagInfo {
    pub tag: String,
    pub slot: String,
    // axes: the cva axis names declared by the IR file this fn comes from,
    // each mapped to its declared values.
    pub axes: HashMap<String, Vec<String>>,
}

/// loadJsxTagIndex maps a JSX fn name (as upstream mdx spells it, e.g.
/// "CarouselItem") to the real tag + data-slot the shipped markup uses for
/// that slot. Built once from generated/ir (fn → slot) and dist/components
/// (slot → tag, the majority tag across every page that emits the slot).
pub fn load_jsx_tag_index(root: &Path) -> Result<&'static HashMap<String, JsxTagInfo>, String> {
    static IDX: OnceLock<HashMap<String, JsxTagInfo>> = OnceLock::new();
    if let Some(idx) = IDX.get() {
        return Ok(idx);
    }
    let mut fn_slot: HashMap<String, String> = HashMap::new();
    let mut fn_axes: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let ents = std::fs::read_dir(root.join("generated/ir")).map_err(|e| e.to_string())?;
    for e in ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if !n.ends_with(".json") {
            continue;
        }
        let Ok(b) = std::fs::read_to_string(root.join("generated/ir").join(&n)) else {
            continue;
        };
        // Go json.Unmarshal tolerates `"slot": null`; serde does not.
        let mut v: serde_json::Value = match serde_json::from_str(&b) {
            Ok(v) => v,
            Err(_) => continue,
        };
        crate::emit::css::drop_nulls(&mut v);
        let Ok(ir) = serde_json::from_value::<crate::emit::css::CssIrComponent>(v) else {
            continue;
        };
        // The axis union of this IR file.
        let mut axes: HashMap<String, Vec<String>> = HashMap::new();
        for key in ir.cva.keys() {
            if let Some(table) = ir.cva.table(&key) {
                for ax in table.axis_order() {
                    let vals: Vec<String> = table.values(&ax).keys().cloned().collect();
                    axes.entry(ax).or_default().extend(vals);
                }
            }
        }
        for c in &ir.components {
            if c.fn_.is_empty() || fn_slot.contains_key(&c.fn_) {
                continue;
            }
            for el in &c.elements {
                if !el.slot.is_empty() {
                    fn_slot.insert(c.fn_.clone(), el.slot.clone());
                    if !axes.is_empty() {
                        fn_axes.insert(c.fn_.clone(), axes.clone());
                    }
                    break;
                }
            }
        }
    }
    let mut slot_tag_count: HashMap<String, HashMap<String, usize>> = HashMap::new();
    if let Ok(dents) = std::fs::read_dir(root.join("dist/components")) {
        for e in dents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if !n.ends_with(".html") {
                continue;
            }
            let Ok(b) = std::fs::read_to_string(root.join("dist/components").join(&n)) else {
                continue;
            };
            for m in re_data_slot().captures_iter(&b) {
                let (tag, slot) = (m[1].to_string(), m[2].to_string());
                *slot_tag_count
                    .entry(slot)
                    .or_default()
                    .entry(tag)
                    .or_insert(0) += 1;
            }
        }
    }
    let mut idx: HashMap<String, JsxTagInfo> = HashMap::new();
    for (fn_, slot) in fn_slot {
        let Some(counts) = slot_tag_count.get(&slot) else {
            continue;
        };
        if counts.is_empty() {
            continue;
        }
        let mut tags: Vec<&String> = counts.keys().collect();
        tags.sort();
        let mut best_tag = String::new();
        let mut best_n: isize = -1;
        for t in tags {
            if counts[t] as isize > best_n {
                best_tag = t.clone();
                best_n = counts[t] as isize;
            }
        }
        idx.insert(
            fn_.clone(),
            JsxTagInfo {
                tag: best_tag,
                slot,
                axes: fn_axes.get(&fn_).cloned().unwrap_or_default(),
            },
        );
    }
    Ok(IDX.get_or_init(|| idx))
}

/// kebabIconName: "ChevronLeftIcon" -> "chevron-left" — matches lucide's own
/// kebab-case icon ids, which is what dist markup's `lucide-<id>` class uses.
pub fn kebab_icon_name(fn_: &str) -> String {
    let base = fn_.trim_end_matches("Icon");
    let words: Vec<String> = re_camel_word()
        .find_iter(base)
        .map(|m| m.as_str().to_lowercase())
        .collect();
    words.join("-")
}

/// rewriteJsxAttrs ports one JSX tag's attribute text to markup.
pub fn rewrite_jsx_attrs(fn_: &str, attrs: &str, info: &JsxTagInfo) -> Result<String, String> {
    let rest = attrs.trim();
    if rest.is_empty() {
        return Ok(String::new());
    }
    let mut out: Vec<String> = Vec::new();
    for m in re_jsx_tag_attr().captures_iter(rest) {
        let name = m[1].to_string();
        let val = m.get(2).map(|v| v.as_str()).unwrap_or("").to_string();
        let has_val = m[0].contains('=');
        let lower = name.to_lowercase();
        // React's two renames of real HTML attributes. className is already
        // handled a step earlier (reClassNameAttr); htmlFor has no other form.
        let (name, lower, whole) = if name == "htmlFor" {
            (
                "for".to_string(),
                "for".to_string(),
                format!("for=\"{}\"", val),
            )
        } else {
            (name.clone(), lower, m[0].to_string())
        };
        if lower.starts_with("data-") || lower.starts_with("aria-") {
            out.push(whole);
        } else if html_attrs().contains(lower.as_str()) {
            out.push(whole);
        } else if info.axes.contains_key(&name) {
            if !has_val {
                return Err(format!("JSX prop {} on <{}> has no value", name, fn_));
            }
            let declared = &info.axes[&name];
            if !declared.contains(&val) {
                return Err(format!(
                    "JSX prop {}={:?} on <{}> is not a declared value ({})",
                    name,
                    val,
                    fn_,
                    declared.join(", ")
                ));
            }
            out.push(format!("data-{}=\"{}\"", name, val));
        } else {
            return Err(format!("JSX prop {} on <{}> has no markup form", name, fn_));
        }
    }
    if out.is_empty() {
        return Ok(String::new());
    }
    Ok(format!(" {}", out.join(" ")))
}

/// rewriteJsxTagsInLine mechanically rewrites JSX tags on one fence line.
pub fn rewrite_jsx_tags_in_line(
    line: &str,
    idx: &HashMap<String, JsxTagInfo>,
) -> Result<String, String> {
    let mut out_err: Option<String> = None;
    let out = re_jsx_tag()
        .replace_all(line, |caps: &regex::Captures| {
            if out_err.is_some() {
                return caps.get(0).unwrap().as_str().to_string();
            }
            let m = caps.get(0).unwrap().as_str();
            let fn_ = caps.get(1).unwrap().as_str();
            let self_close = caps.get(2).map(|v| v.as_str() == "/").unwrap_or(false);
            let closing = m.starts_with("</");
            if re_icon_fn().is_match(fn_) {
                return format!("<!-- lucide \"{}\" icon -->", kebab_icon_name(fn_));
            }
            let Some(info) = idx.get(fn_) else {
                out_err = Some(format!("unmapped JSX tag <{}>", fn_));
                return m.to_string();
            };
            if closing {
                return format!("</{}>", info.tag);
            }
            let mut attrs = m[format!("<{}", fn_).len()..].to_string();
            if self_close {
                attrs = attrs.trim_end_matches("/>").to_string();
            } else {
                attrs = attrs.trim_end_matches('>').to_string();
            }
            attrs = attrs.trim_end_matches(' ').to_string();
            match rewrite_jsx_attrs(fn_, &attrs, info) {
                Ok(a) => {
                    let open = format!(
                        "<{} data-slot=\"{}\"{}",
                        info.tag, info.slot, a
                    );
                    if self_close {
                        format!("{} />", open)
                    } else {
                        format!("{}>", open)
                    }
                }
                Err(e) => {
                    out_err = Some(e);
                    m.to_string()
                }
            }
        })
        .into_owned();
    if let Some(e) = out_err {
        return Err(e);
    }
    Ok(out)
}

/// rewriteLeakedJsxFences runs after the four handled sections and any hand
/// overrides. Every remaining ```tsx/```jsx/```ts fence is upstream
/// illustrating a prop or composition with real React JSX.
pub fn rewrite_leaked_jsx_fences(page: &str, raw: &str, root: &Path) -> Result<String, String> {
    let idx = load_jsx_tag_index(root)?;
    let lines: Vec<&str> = raw.split('\n').collect();
    let mut out: Vec<String> = Vec::new();
    let mut in_fence = false;
    let mut fence_start = 0usize;
    for (i, line) in lines.iter().enumerate() {
        if !in_fence {
            if re_leaked_fence_open().is_match(line) {
                in_fence = true;
                fence_start = i;
                out.push("```html".to_string());
                continue;
            }
            out.push(line.to_string());
            continue;
        }
        if line.starts_with("```") {
            in_fence = false;
            out.push(line.to_string());
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with("import ") || trimmed.starts_with("import{") {
            return Err(format!(
                "{}: leaked fence at line {} has a React import — needs a docs_overrides.go entry: {:?}",
                page,
                fence_start + 1,
                trimmed
            ));
        }
        let mut line = re_class_name_attr().replace_all(line, "class=").into_owned();
        line = re_jsx_comment()
            .replace_all(&line, |caps: &regex::Captures| {
                format!("<!-- {} -->", caps.get(1).unwrap().as_str())
            })
            .into_owned();
        let rewritten = rewrite_jsx_tags_in_line(&line, idx).map_err(|e| {
            format!(
                "{}: leaked fence at line {}: {} — needs a docs_overrides.go entry: {:?}",
                page,
                fence_start + 1,
                e,
                trimmed
            )
        })?;
        if rewritten.contains('{') && rewritten.contains('}') {
            return Err(format!(
                "{}: leaked fence at line {} still has a JS expression — needs a docs_overrides.go entry: {:?}",
                page,
                fence_start + 1,
                trimmed
            ));
        }
        out.push(rewritten);
    }
    Ok(out.join("\n"))
}

/// rewriteInlineJsxMentions handles the prose form ("the `<CarouselItem />`").
pub fn rewrite_inline_jsx_mentions(page: &str, raw: &str, root: &Path) -> Result<String, String> {
    let idx = load_jsx_tag_index(root)?;
    let mut out_err: Option<String> = None;
    let out = re_inline_jsx_mention()
        .replace_all(raw, |caps: &regex::Captures| {
            if out_err.is_some() {
                return caps.get(0).unwrap().as_str().to_string();
            }
            let m = caps.get(0).unwrap().as_str();
            let fn_ = caps.get(1).unwrap().as_str();
            if re_icon_fn().is_match(fn_) {
                return format!("`<!-- lucide \"{}\" icon -->`", kebab_icon_name(fn_));
            }
            let Some(info) = idx.get(fn_) else {
                out_err = Some(format!(
                    "{}: unmapped inline JSX mention {} — needs a docs_overrides.go entry",
                    page, m
                ));
                return m.to_string();
            };
            format!("`data-slot=\"{}\"`", info.slot)
        })
        .into_owned();
    if let Some(e) = out_err {
        return Err(e);
    }
    Ok(out)
}

// ---------------------------------------------------------------- declared prose adjustments

pub struct TextOp {
    pub find: &'static str,
    pub replace: &'static str,
}

pub struct TextAdjustment {
    pub id: &'static str,
    pub files: &'static [&'static str],
    pub ops: &'static [TextOp],
}

static TEXT_ADJUSTMENTS: &[TextAdjustment] = &[
    TextAdjustment {
        id: "carousel-embla-doc-urls",
        files: &["carousel.mdx"],
        ops: &[
            TextOp {
                find: "See the [Embla Carousel docs](https://www.embla-carousel.com/api/events/) for more information on using events.",
                replace: "See the [Embla Carousel docs](https://www.embla-carousel.com/docs/api/events) for more information on using events.",
            },
            TextOp {
                find: "See the [Embla Carousel docs](https://www.embla-carousel.com/api/) for more information on props and plugins.",
                replace: "See the [Embla Carousel docs](https://www.embla-carousel.com/docs/api) for more information on props and plugins.",
            },
        ],
    },
    TextAdjustment {
        id: "button-pointer-cli-prose",
        files: &["button.mdx"],
        ops: &[TextOp {
            find: "You can also enable this during project setup with `npx shadcn@latest init --pointer`.",
            replace: "shadless has no CLI, so the rule below is the whole of it — there is no flag that adds it for you.",
        }],
    },
];

/// The declared prose adjustments (public for docs-fidelity's retired-prose
/// check).
pub fn text_adjustments() -> &'static [TextAdjustment] {
    TEXT_ADJUSTMENTS
}

/// docsHrefsOf: every /docs/… href outside fences.
pub fn docs_hrefs_of(md: &str) -> Vec<String> {
    re_docs_href()
        .captures_iter(&fence_shadow(md))
        .map(|m| m[1].to_string())
        .collect()
}

/// allHrefsOf: every markdown link target outside fences.
pub fn all_hrefs_of(md: &str) -> Vec<String> {
    re_all_href()
        .captures_iter(&fence_shadow(md))
        .map(|m| m[1].to_string())
        .collect()
}

/// applyTextAdjustments rewrites the declared ops for the named file. Errors
/// if a declared find string is missing — the adjustment must be re-anchored,
/// never silently skipped.
pub fn apply_text_adjustments(basename: &str, raw: &str) -> Result<String, String> {
    let mut out = raw.to_string();
    for adj in TEXT_ADJUSTMENTS {
        if !adj.files.contains(&basename) {
            continue;
        }
        for op in adj.ops {
            let i = fence_shadow(&out)
                .find(op.find)
                .ok_or_else(|| {
                    format!(
                        "text adjustment {}: find string not present in {} — re-anchor against the new upstream prose",
                        adj.id, basename
                    )
                })?;
            out = format!(
                "{}{}{}",
                &out[..i],
                op.replace,
                &out[i + op.find.len()..]
            );
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------- jsx overrides

/// Hand-authored fixes for leaked upstream React JSX that the mechanical
/// rewrite can't resolve. Extracted verbatim from pipeline/docs_overrides.go.
pub fn jsx_overrides() -> &'static [(&'static str, &'static str, &'static str)] {
    static O: OnceLock<Vec<(&'static str, &'static str, &'static str)>> = OnceLock::new();
    O.get_or_init(|| include!("jsx_overrides.inc"))
}

pub fn apply_jsx_overrides(page: &str, raw: &str) -> Result<String, String> {
    let mut out = raw.to_string();
    for (o_page, find, repl) in jsx_overrides() {
        if *o_page != page {
            continue;
        }
        if !out.contains(find) {
            return Err(format!(
                "{}: jsx override anchor not found — re-anchor against the new upstream content: {:?}",
                page, find
            ));
        }
        out = out.replacen(find, repl, 1);
    }
    Ok(out)
}

// ---------------------------------------------------------------- frontmatter

pub type Frontmatter = HashMap<String, serde_json::Value>;

/// parseFrontmatter tolerates CRLF and coerces true/false/integers; quotes are
/// stripped only as a matched pair (so `years'` keeps its apostrophe).
pub fn parse_frontmatter(src: &str) -> Frontmatter {
    let Some(m) = re_frontmatter().captures(src) else {
        return Frontmatter::new();
    };
    let mut out: Frontmatter = Frontmatter::new();
    let mut cur: Option<HashMap<String, serde_json::Value>> = None;
    for line in m[1].split('\n') {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        if let Some(top) = re_top_key().captures(line) {
            if !line.starts_with([' ', '\t']) {
                if top[2].is_empty() {
                    let inner = HashMap::new();
                    out.insert(top[1].to_string(), serde_json::json!(inner));
                    cur = Some(inner);
                } else {
                    out.insert(top[1].to_string(), coerce_scalar(&top[2]));
                    cur = None;
                }
                continue;
            }
        }
        if let Some(sub) = re_sub_key().captures(line) {
            if let Some(cur) = cur.as_mut() {
                cur.insert(sub[1].to_string(), coerce_scalar(&sub[2]));
            }
        }
    }
    out
}

fn coerce_scalar(v: &str) -> serde_json::Value {
    if v == "true" {
        return serde_json::Value::Bool(true);
    }
    if v == "false" {
        return serde_json::Value::Bool(false);
    }
    if re_int_only().is_match(v) {
        let mut n: i64 = 0;
        let mut neg = false;
        for (i, c) in v.chars().enumerate() {
            if i == 0 && c == '-' {
                neg = true;
                continue;
            }
            n = n * 10 + (c as i64 - '0' as i64);
        }
        return serde_json::Value::from(if neg { -n } else { n });
    }
    let b = v.as_bytes();
    if b.len() >= 2 && b[0] == b[b.len() - 1] && (b[0] == b'"' || b[0] == b'\'') {
        return serde_json::Value::String(v[1..v.len() - 1].to_string());
    }
    serde_json::Value::String(v.to_string())
}

/// fmString reads a scalar as a string ("" when absent/not a string).
pub fn fm_string(fm: &Frontmatter, key: &str) -> String {
    fm.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// stripImports removes ESM import statements outside fences — the
/// fence-agnostic line-anchored strip.
pub fn strip_imports(src: &str) -> String {
    let out = re_import_named().replace_all(src, "");
    re_import_bare_line().replace_all(&out, "").into_owned()
}

// ---------------------------------------------------------------- fidelity facts

#[derive(Clone, Debug)]
pub struct FenceInfo {
    pub lang: String,
    pub content: String,
}

/// scanFences: one entry per fenced block; lang is the info string up to the
/// first space; content is verbatim. Unclosed fences run to EOF.
pub fn scan_fences(src: &str) -> Vec<FenceInfo> {
    let mut out: Vec<FenceInfo> = Vec::new();
    let mut open: isize = -1;
    let mut lang = String::new();
    let mut content = String::new();
    for line in src.split('\n') {
        if open < 0 {
            if let Some(m) = re_fence_line().captures(line) {
                if line.trim_start().starts_with("```") {
                    open = 3;
                    lang = m[1].splitn(2, ' ').next().unwrap_or("").to_string();
                    content = String::new();
                    continue;
                }
            }
        } else {
            if line.trim_start().starts_with("```") {
                out.push(FenceInfo {
                    lang: lang.clone(),
                    content: content.clone(),
                });
                open = -1;
                continue;
            }
            content += line;
            content.push('\n');
        }
    }
    if open >= 0 {
        out.push(FenceInfo { lang, content });
    }
    out
}

pub fn decode_entities(s: &str) -> String {
    re_entity()
        .replace_all(s, |caps: &regex::Captures| match &caps[1] {
            "amp" => "&",
            "lt" => "<",
            "gt" => ">",
            "quot" => "\"",
            _ => unreachable!(),
        })
        .into_owned()
}

pub fn html_text(s: &str) -> String {
    decode_entities(&re_ws().replace_all(&re_any_tag().replace_all(s, ""), " "))
}

pub fn md_text(s: &str) -> String {
    re_ws().replace_all(s, " ").into_owned()
}

pub fn strip_inline_code(s: &str) -> String {
    re_inline_code().replace_all(s, "$1").into_owned()
}

#[derive(Clone, Debug)]
pub struct HeadingEnt {
    pub depth: usize,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct PreviewEnt {
    pub name: String,
    pub style_name: String,
    pub direction: String,
}

#[derive(Clone, Debug)]
pub struct SourceEnt {
    pub name: String,
    pub src: String,
}

#[derive(Clone, Debug, Default)]
pub struct PageFacts {
    pub frontmatter: Frontmatter,
    pub headings: Vec<HeadingEnt>,
    pub previews: Vec<PreviewEnt>,
    pub sources: Vec<SourceEnt>,
    pub fences: Vec<FenceInfo>,
}

/// mdxPageFacts with the drop* predicates mirroring what the builder applied.
#[allow(clippy::too_many_arguments)]
pub fn mdx_page_facts(
    name: &str,
    raw: &str,
    drop_code_tabs: bool,
    drop_install_section: bool,
    drop_rtl_migrate: bool,
    drop_usage_section: bool,
    drop_composition_section: bool,
    drop_api_reference_section: bool,
    drop_changelog_section: bool,
    fix_leaked_jsx: bool,
    root: &Path,
) -> Result<PageFacts, String> {
    let mut src = raw.to_string();
    if fix_leaked_jsx {
        src = apply_jsx_overrides(name, &src)?;
    }
    if drop_code_tabs {
        src = without_code_tabs(&src);
    }
    if drop_install_section {
        src = without_install_section(&src);
    }
    if drop_rtl_migrate {
        src = without_rtl_migrate(&src);
    }
    if drop_usage_section {
        src = without_usage_section(&src);
    }
    if drop_code_tabs {
        src = strip_imports_from_mixed_fences(&drop_react_import_fences(&src));
    }
    if drop_composition_section {
        src = without_composition_section(&src);
    }
    if drop_api_reference_section {
        src = without_api_reference_section(name, &src);
    }
    if drop_changelog_section {
        src = without_changelog_section(&src);
    }
    if let Some(s) = locate_message_scroller_js_span(&fence_shadow(&src)) {
        src = replace_span(&src, s, &message_scroller_js_note());
    }
    if fix_leaked_jsx {
        src = rewrite_leaked_jsx_fences(name, &src, root)?;
        src = rewrite_inline_jsx_mentions(name, &src, root)?;
    }
    let body = fence_shadow(&src);
    let no_inline_code = re_inline_code()
        .replace_all(&body, |caps: &regex::Captures| {
            " ".repeat(caps.get(0).unwrap().as_str().len())
        })
        .into_owned();
    let mut heads: Vec<(usize, usize, String)> = Vec::new();
    for caps in re_md_heading().captures_iter(&body) {
        let m = caps.get(0).unwrap();
        let depth = caps.get(1).unwrap().as_str().len();
        let text = strip_inline_code(&md_text(caps.get(2).unwrap().as_str()));
        heads.push((m.start(), depth, text));
    }
    for h in find_all_raw_headings(&no_inline_code) {
        heads.push((h.at, h.depth, html_text(&h.text)));
    }
    heads.sort_by_key(|h| h.0);
    let mut f = PageFacts {
        frontmatter: parse_frontmatter(raw),
        ..PageFacts::default()
    };
    for h in heads {
        f.headings.push(HeadingEnt {
            depth: h.1,
            text: h.2,
        });
    }
    for caps in re_comp_preview().captures_iter(&body) {
        f.previews.push(PreviewEnt {
            name: attr_of_js(&caps[1], "name"),
            style_name: attr_of_js(&caps[1], "styleName"),
            direction: attr_of_js(&caps[1], "direction"),
        });
    }
    for caps in re_comp_source().captures_iter(&body) {
        f.sources.push(SourceEnt {
            name: attr_of_js(&caps[1], "name"),
            src: attr_of_js(&caps[1], "src"),
        });
    }
    f.fences = scan_fences(&src);
    Ok(f)
}

pub fn without_code_tabs(raw: &str) -> String {
    let mut out = raw.to_string();
    // reverse order to keep offsets valid
    let spans = locate_code_tabs_spans(&fence_shadow(raw));
    for s in spans.iter().rev() {
        out = format!("{}\n{}", &out[..s.start], &out[s.end..]);
    }
    out
}

pub fn without_install_section(raw: &str) -> String {
    let Some(s) = locate_install_section(&fence_shadow(raw)) else {
        return raw.to_string();
    };
    replace_span(raw, s, "## Installation\n\n")
}

pub fn without_rtl_migrate(raw: &str) -> String {
    let Some(s) = locate_rtl_migrate_span(&fence_shadow(raw)) else {
        return raw.to_string();
    };
    let mut out = replace_span(raw, s, "\n");
    // Both halves of the same builder branch (guideTransform's g.rtlMigrate):
    // the migrate section AND the framework/CLI run are replaced together.
    if let Some(f) = locate_rtl_framework_span(&fence_shadow(&out)) {
        out = replace_span(&out, f, &rtl_framework_note());
    }
    out
}

pub fn without_usage_section(raw: &str) -> String {
    let Some(s) = locate_usage_span(&fence_shadow(raw)) else {
        return raw.to_string();
    };
    replace_span(raw, s, "")
}

pub fn without_composition_section(raw: &str) -> String {
    let Some(s) = locate_composition_span(&fence_shadow(raw)) else {
        return raw.to_string();
    };
    replace_span(raw, s, "## Composition\n")
}

pub fn without_api_reference_section(comp: &str, raw: &str) -> String {
    let Some(s) = locate_api_reference_span(comp, &fence_shadow(raw)) else {
        return raw.to_string();
    };
    replace_span(raw, s, "## API Reference\n\n")
}

pub fn without_changelog_section(raw: &str) -> String {
    let Some(s) = locate_changelog_span(&fence_shadow(raw)) else {
        return raw.to_string();
    };
    replace_span(raw, s, "")
}

/// mdPageFacts reads the facts the CONTENT transform is responsible for out of
/// the built markdown (the TOC/pager/breadcrumb are VitePress's now).
#[derive(Clone, Debug, Default)]
pub struct MdFacts {
    pub text: String,
    pub h1: String,
    pub lead: String,
    pub headings: Vec<HeadingEnt>,
    pub previews: Vec<(String, String, String)>, // name, status, src
    pub fences: Vec<FenceInfo>,
    pub chips: Vec<(String, String)>,
    pub iframes: Vec<String>,
}

pub fn md_page_facts(md: &str) -> MdFacts {
    let front = parse_frontmatter(md);
    let body = re_frontmatter_block().replace_all(md, "");
    let h1 = re_md_h1()
        .captures(&body)
        .map(|m| m[1].to_string())
        .unwrap_or_default();
    let after_h1 = match body.find(&format!("# {}", h1)) {
        Some(i) => body[i + h1.len() + 3..].to_string(),
        None => body.to_string(),
    };
    let shadow = fence_shadow(&after_h1);
    let no_inline_code = re_inline_code()
        .replace_all(&shadow, |caps: &regex::Captures| {
            " ".repeat(caps.get(0).unwrap().as_str().len())
        })
        .into_owned();
    let mut heads: Vec<(usize, usize, String)> = Vec::new();
    for caps in re_md_heading().captures_iter(&shadow) {
        let m = caps.get(0).unwrap();
        let depth = caps.get(1).unwrap().as_str().len();
        let text = strip_inline_code(&md_text(caps.get(2).unwrap().as_str()));
        heads.push((m.start(), depth, text));
    }
    for h in find_all_raw_headings(&no_inline_code) {
        heads.push((h.at, h.depth, html_text(&h.text)));
    }
    heads.sort_by_key(|h| h.0);
    let mut f = MdFacts {
        text: body.to_string(),
        h1: md_text(&h1),
        lead: md_text(&fm_string(&front, "description")),
        ..MdFacts::default()
    };
    for h in heads {
        f.headings.push(HeadingEnt {
            depth: h.1,
            text: h.2,
        });
    }
    let mut by_name: HashMap<String, (String, String, String)> = HashMap::new();
    for caps in re_demo_iframe().captures_iter(&after_h1) {
        by_name.insert(
            caps[2].to_string(),
            (caps[2].to_string(), caps[3].to_string(), caps[1].to_string()),
        );
    }
    for caps in re_demo_missing().captures_iter(&after_h1) {
        by_name.insert(
            caps[1].to_string(),
            (caps[1].to_string(), caps[2].to_string(), String::new()),
        );
    }
    for caps in re_demo_in_order().captures_iter(&after_h1) {
        let n = if !caps.get(1).map(|m| m.as_str()).unwrap_or("").is_empty() {
            caps[1].to_string()
        } else {
            caps[2].to_string()
        };
        if let Some(e) = by_name.get(&n) {
            f.previews.push(e.clone());
        }
    }
    f.fences = scan_fences(&after_h1);
    if let Some(m) = re_page_links_p().captures(&after_h1) {
        for a in re_anchor().captures_iter(&m[1]) {
            f.chips.push((a[2].to_string(), a[1].to_string()));
        }
    }
    for caps in re_demo_iframe().captures_iter(&after_h1) {
        f.iframes.push(caps[1].to_string());
    }
    f
}

// ---------------------------------------------------------------- comparison

fn known_statuses() -> &'static std::collections::HashSet<&'static str> {
    static S: OnceLock<std::collections::HashSet<&'static str>> = OnceLock::new();
    S.get_or_init(|| {
        [
            "existing-dist",
            "authored",
            "unavailable",
            "to-author",
            "tombstoned",
            "unknown",
        ]
        .iter()
        .copied()
        .collect()
    })
}

/// comparePage returns the fidelity issues for one page.
pub fn compare_page(
    m: &PageFacts,
    h: &MdFacts,
    page_name: &str,
    is_component_page: bool,
    expected_manual_ref: &str,
) -> Vec<String> {
    let mut issues: Vec<String> = Vec::new();
    let mut issue = |kind: &str, detail: String| issues.push(format!("{}: {}", kind, detail));

    let mut want_title = fm_string(&m.frontmatter, "title");
    if want_title.is_empty() {
        want_title = page_name.to_string();
    }
    if h.h1 != md_text(&want_title) {
        issue("h1", format!("built={:?} mdx={:?}", h.h1, md_text(&want_title)));
    }
    let want_lead = fm_string(&m.frontmatter, "description");
    if h.lead != md_text(&want_lead) {
        issue(
            "lead",
            format!("built={:?} mdx={:?}", trunc70(&h.lead), trunc70(&md_text(&want_lead))),
        );
    }
    let want_heads = format!("{:?}", m.headings);
    let got_heads = format!("{:?}", h.headings);
    if want_heads != got_heads {
        issue(
            "headings",
            format!("mdx {:?} != built {:?}", trunc140(&want_heads), trunc140(&got_heads)),
        );
    }
    let want_prev: Vec<String> = m.previews.iter().map(|p| p.name.clone()).collect();
    let got_prev: Vec<String> = h.previews.iter().map(|p| p.0.clone()).collect();
    if format!("{:?}", want_prev) != format!("{:?}", got_prev) {
        issue(
            "previews",
            format!(
                "mdx {:?} != built {:?}",
                trunc120(&format!("{:?}", want_prev)),
                trunc120(&format!("{:?}", got_prev))
            ),
        );
    }
    for p in &h.previews {
        if !known_statuses().contains(p.1.as_str()) {
            issue(
                "preview-status",
                format!("{}: status {:?} not emitted by the catalog", p.0, p.1),
            );
        }
        if p.1 == "unknown" && is_component_page {
            issue(
                "preview-status",
                format!(
                    "{}: unknown status on a component page (radix catalog must be complete)",
                    p.0
                ),
            );
        }
    }
    for f in &m.fences {
        let want = md_text(&f.content);
        if want.is_empty() {
            continue;
        }
        let mut found = false;
        for p in &h.fences {
            if md_text(&p.content).contains(&want) {
                found = true;
                break;
            }
        }
        if !found {
            issue(
                "fence",
                format!(
                    "[{}] {:?} has no matching fence in the built page",
                    f.lang,
                    trunc80(&want)
                ),
            );
        }
    }
    // The mdx frontmatter `links:` chips are a DECLARED drop, not a mirror.
    if !h.chips.is_empty() {
        let mut got: Vec<String> = h.chips.iter().map(|c| format!("{}→{}", c.0, c.1)).collect();
        got.sort();
        issue(
            "chips",
            format!("built page still carries page-links chips: {}", got.join(" | ")),
        );
    }
    if !expected_manual_ref.is_empty() && !h.text.contains(expected_manual_ref) {
        issue(
            "manual-tab",
            format!("rewritten manual tab never mentions {}", expected_manual_ref),
        );
    }
    // Two assertions that read ONLY the built page.
    for f in &h.fences {
        // html fences only: those are the hand-written and rewritten ones.
        if f.lang != "html" {
            continue;
        }
        for prop in react_props_in_markup(&f.content) {
            issue(
                "react-prop",
                format!(
                    "html fence carries the JSX prop {} on a data-slot element — no stylesheet selects it; it must be a data attribute or be dropped",
                    prop
                ),
            );
        }
    }
    if let Some(m) = re_react_prop_table().find(&h.text) {
        issue(
            "react-prop-table",
            format!(
                "built page keeps an upstream React props table: {}",
                m.as_str().trim()
            ),
        );
    }
    issues
}

fn trunc70(s: &str) -> String {
    if s.len() > 70 {
        format!("{}…", &s[..70])
    } else {
        s.to_string()
    }
}
fn trunc80(s: &str) -> String {
    if s.len() > 80 {
        format!("{}…", &s[..80])
    } else {
        s.to_string()
    }
}
fn trunc120(s: &str) -> String {
    if s.len() > 120 {
        format!("{}…", &s[..120])
    } else {
        s.to_string()
    }
}
fn trunc140(s: &str) -> String {
    if s.len() > 140 {
        format!("{}…", &s[..140])
    } else {
        s.to_string()
    }
}

/// reactPropsInMarkup names every attribute on a data-slot element that is
/// neither a plain HTML attribute nor data-*/aria-*.
pub fn react_props_in_markup(fence: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    for tag in re_slot_open_tag().find_iter(fence) {
        let tag = tag.as_str();
        let mut body = tag;
        if let Some(i) = body.find([' ', '\t']) {
            body = &body[i..];
        }
        body = body.trim().trim_end_matches('>').trim_end_matches('/');
        for m in re_tag_attr_name().captures_iter(&format!(" {}", body)) {
            let name = m[1].to_string();
            let lower = name.to_lowercase();
            if lower.starts_with("data-")
                || lower.starts_with("aria-")
                || html_attrs().contains(lower.as_str())
            {
                continue;
            }
            if seen.insert(name.clone()) {
                out.push(name);
            }
        }
    }
    out.sort();
    out
}

/// rawHeading is one <hN>…</hN> hit (JS used a \1 backreference RE2 lacks).
pub struct RawHeading {
    at: usize,
    depth: usize,
    text: String,
}

pub fn find_all_raw_headings(s: &str) -> Vec<RawHeading> {
    let mut out: Vec<RawHeading> = Vec::new();
    let mut off = 0usize;
    loop {
        let Some(i) = re_h_open().find(&s[off..]) else {
            break;
        };
        let depth = (s.as_bytes()[off + i.start() + 2] - b'0') as usize;
        let open = off + i.end();
        let close = format!("</h{}>", depth);
        let Some(j) = s[open..].find(&close) else {
            break;
        };
        out.push(RawHeading {
            at: off + i.start(),
            depth,
            text: s[open..open + j].to_string(),
        });
        off = open + j + close.len();
    }
    out
}

// ---------------------------------------------------------------- guides

pub const GUIDES_UP: &str = ".upstream/shadcn-ui/apps/v4/content/docs";

#[derive(Clone, Debug)]
pub struct Guide {
    pub slug: &'static str,
    pub route: &'static str,
    pub title: &'static str,
    pub source: &'static str,
    pub disposition: &'static str,
    pub notes: &'static str,
    pub install_section: bool,
    pub util: &'static str,
    pub rtl_migrate: bool,
    pub pinned: bool,
}

pub fn guides() -> &'static [Guide] {
    static G: OnceLock<Vec<Guide>> = OnceLock::new();
    G.get_or_init(|| {
        vec![
            Guide { slug: "introduction", route: "/docs/introduction", title: "Introduction", source: "docs/content/introduction.mdx", disposition: "adapted",
                notes: "FT8: vanilla rewrite of upstream (root)/index.mdx; pure prose + 1 Accordion FAQ — was wrongly pruned as \"framework content\" but contains no React/CLI specifics",
                install_section: false, util: "", rtl_migrate: false, pinned: true },
            Guide { slug: "installation", route: "/docs/installation", title: "Installation", source: "docs/content/installation.mdx", disposition: "adapted",
                notes: "authored vanilla rewrite; replaces upstream installation/index.mdx (CLI/create/framework cards — React toolchain) and installation/manual.mdx (Tailwind+aliases+components.json setup); copy-files story over dist/ artifacts",
                install_section: false, util: "", rtl_migrate: false, pinned: false },
            Guide { slug: "dark-mode", route: "/docs/dark-mode", title: "Dark Mode", source: "docs/content/dark-mode.mdx", disposition: "adapted",
                notes: "authored rewrite; upstream dark-mode/index.mdx is framework cards only (next/vite/astro/remix/tanstack-start pruned); .dark theme variables ship precompiled in dist/out.css; mode-toggle preview to-author (FT7)",
                install_section: false, util: "", rtl_migrate: false, pinned: false },
            Guide { slug: "rtl", route: "/docs/rtl", title: "RTL", source: "generated/docs-upstream/rtl/index.mdx", disposition: "mirror",
                notes: "load-bearing (56 radix pages link /docs/rtl); framework sub-pages (rtl/next|vite|start) pruned → their LinkedCard links greyed; card-rtl preview to-author (FT7); shadcn-CLI migrate section rewritten (rtlMigrate)",
                install_section: false, util: "", rtl_migrate: true, pinned: false },
            Guide { slug: "shimmer", route: "/docs/utils/shimmer", title: "shimmer", source: "generated/docs-upstream/utils/shimmer.mdx", disposition: "adapted",
                notes: "mirrored; Installation section replaced (utilities ship precompiled in dist/out.css, no npm install); all 9 previews base-style (base-rhea) → unavailable",
                install_section: true, util: "shimmer", rtl_migrate: false, pinned: false },
            Guide { slug: "scroll-fade", route: "/docs/utils/scroll-fade", title: "scroll-fade", source: "generated/docs-upstream/utils/scroll-fade.mdx", disposition: "adapted",
                notes: "mirrored; Installation section replaced (utilities ship precompiled in dist/out.css); all 7 previews base-style (6 base-rhea + 1 base-nova) → unavailable",
                install_section: true, util: "scroll-fade", rtl_migrate: false, pinned: false },
            Guide { slug: "typography", route: "/docs/typography", title: "Typography", source: "docs/content/typography.mdx", disposition: "adapted",
                notes: "FT8: vanilla rewrite — upstream typography.mdx demos a <Typography> component shadless does not (and should not) ship; this guide maps the same typographic roles to plain Tailwind utilities already in dist/out.css",
                install_section: false, util: "", rtl_migrate: false, pinned: false },
        ]
    })
}

pub fn pruned_guides() -> &'static [(&'static str, &'static str, &'static str)] {
    static P: OnceLock<Vec<(&'static str, &'static str, &'static str)>> = OnceLock::new();
    P.get_or_init(|| {
        vec![
            ("forms", ".upstream/shadcn-ui/apps/v4/content/docs/forms/", "React-only (react-hook-form / tanstack-form / formisch guides); field.mdx links ×4 → greyed spans"),
            ("react", ".upstream/shadcn-ui/apps/v4/content/docs/react/", "React-only component recipes (message-scroller, questionnaire); radix links ×6 → greyed spans"),
            ("registry", ".upstream/shadcn-ui/apps/v4/content/docs/registry/", "shadcn CLI registry system (json schema, MCP, namespaces) — no vanilla equivalent"),
            ("changelog", ".upstream/shadcn-ui/apps/v4/content/docs/changelog/", "shadcn release notes — not shadless content"),
            ("(root)", ".upstream/shadcn-ui/apps/v4/content/docs/(root)/", "shadcn-site root pages (theming, cli, components.json…) — React/CLI specific"),
            ("helpers", ".upstream/shadcn-ui/apps/v4/content/docs/helpers/", "React helper packages (@shadcn/helpers/ai-sdk, /tanstack-ai) — useChat/JSX end to end, no vanilla port and no shadless artifact to document"),
            ("framework sub-pages", ".upstream/shadcn-ui/apps/v4/content/docs/{installation,dark-mode,rtl}/* (non-index)", "per-React-framework setup guides (next/vite/astro/remix/tanstack/laravel/gatsby…) — installation/dark-mode/rtl index pages kept instead"),
        ]
    })
}

/// routeTarget is resolveDocsRoute's result: routable (file+frag) or grey.
pub struct RouteTarget {
    pub file: String,
    pub frag: String,
    pub grey: bool,
}

pub fn resolve_docs_route(href: &str, members: &std::collections::HashSet<String>) -> Option<RouteTarget> {
    if !href.starts_with('/') || href.starts_with("//") {
        return None;
    }
    let (path, frag) = match href.find('#') {
        Some(i) => (&href[..i], &href[i + 1..]),
        None => (href, ""),
    };
    if let Some(m) = re_comp_route().captures(path) {
        if members.contains(&m[1]) {
            return Some(RouteTarget {
                file: format!("{}.html", &m[1]),
                frag: frag.to_string(),
                grey: false,
            });
        }
        return Some(RouteTarget {
            file: String::new(),
            frag: String::new(),
            grey: true,
        });
    }
    for g in guides() {
        if g.route == path {
            return Some(RouteTarget {
                file: format!("{}.html", g.slug),
                frag: frag.to_string(),
                grey: false,
            });
        }
    }
    Some(RouteTarget {
        file: String::new(),
        frag: String::new(),
        grey: true,
    })
}

#[derive(Clone, Debug)]
pub struct GuidePreviewInfo {
    pub host_pages: Vec<String>,
    pub style_name: String,
    pub disposition: String,
    pub reason: String,
}

/// scanGuidePreviews enumerates guide ComponentPreview names with the same
/// fence-stripped tag-scoped discipline as the catalog.
pub fn scan_guide_previews(
    root: &Path,
    catalog_preview_status: &HashMap<String, String>,
) -> (HashMap<String, GuidePreviewInfo>, Vec<String>) {
    let mut out: HashMap<String, GuidePreviewInfo> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for g in guides() {
        let Ok(b) = std::fs::read_to_string(root.join(g.source)) else {
            continue;
        };
        let text = strip_fences(&b);
        for caps in re_guide_preview_tag().captures_iter(&text) {
            let name = attr_of_js(&caps[1], "name");
            if name.is_empty() {
                continue;
            }
            if let Some(e) = out.get_mut(&name) {
                e.host_pages.push(g.slug.to_string());
                continue;
            }
            let style_name = attr_of_js(&caps[1], "styleName");
            let (disposition, reason) = if let Some(st) = catalog_preview_status.get(&name) {
                (st.clone(), "already cataloged in the radix set".to_string())
            } else if style_name.starts_with("base-") {
                (
                    "unavailable".to_string(),
                    format!(
                        "base-line demo ({}) — shadless implements the radix line only",
                        style_name
                    ),
                )
            } else {
                ("to-author".to_string(), "authored in an FT7 wave".to_string())
            };
            out.insert(
                name.clone(),
                GuidePreviewInfo {
                    host_pages: vec![g.slug.to_string()],
                    style_name,
                    disposition,
                    reason,
                },
            );
            order.push(name);
        }
    }
    (out, order)
}

/// attrOfJS: attrOf's (string, bool) collapsed to a bare string, for callers
/// that only need the value.
pub fn attr_of_js(attrs: &str, name: &str) -> String {
    attr_of(attrs, name).0
}

// ---------------------------------------------------------------- families

#[derive(Clone, Copy)]
pub struct FamilyEnt {
    pub kind: &'static str,
    pub open: &'static str,
    pub attr: &'static str,
    pub js: &'static str,
}

pub fn family(comp: &str) -> Option<FamilyEnt> {
    Some(match comp {
        "alert-dialog" => FamilyEnt { kind: "dialog", open: "", attr: "", js: "alert-dialog" },
        "dialog" => FamilyEnt { kind: "dialog", open: "", attr: "", js: "dialog" },
        "sheet" => FamilyEnt { kind: "dialog", open: "", attr: "", js: "sheet" },
        "popover" => FamilyEnt { kind: "portal", open: "click", attr: "", js: "popover" },
        "tooltip" => FamilyEnt { kind: "portal", open: "hover", attr: "", js: "tooltip" },
        "hover-card" => FamilyEnt { kind: "portal", open: "hover", attr: "", js: "hover-card" },
        "tabs" => FamilyEnt { kind: "inline", open: "", attr: "", js: "tabs" },
        "slider" => FamilyEnt { kind: "none", open: "", attr: "", js: "slider" },
        "scroll-area" => FamilyEnt { kind: "none", open: "", attr: "", js: "scroll-area" },
        "dropdown-menu" => FamilyEnt { kind: "menu", open: "click", attr: "data-radixuigo-menu-trigger", js: "dropdown-menu" },
        "context-menu" => FamilyEnt { kind: "menu", open: "contextmenu", attr: "data-radixuigo-context-trigger", js: "context-menu" },
        "menubar" => FamilyEnt { kind: "menu", open: "click", attr: "data-radixuigo-menu-trigger", js: "menubar" },
        "select" => FamilyEnt { kind: "select", open: "", attr: "", js: "select" },
        "carousel" => FamilyEnt { kind: "none", open: "", attr: "", js: "carousel" },
        "navigation-menu" => FamilyEnt { kind: "nav", open: "", attr: "", js: "navigation-menu" },
        _ => return None,
    })
}

fn gesture_of(open: &str) -> &'static str {
    match open {
        "hover" => "hovering",
        "contextmenu" => "right-clicking",
        _ => "clicking",
    }
}

pub fn protocol_mdx(comp: &str) -> String {
    let Some(f) = family(comp) else {
        return String::new();
    };
    let (t, c) = (format!("{}-trigger", comp), format!("{}-content", comp));
    let gesture = gesture_of(f.open);
    let mut rows: Vec<(String, String)> = Vec::new();
    match f.kind {
        "dialog" | "portal" => {
            rows.push((
                format!("`<… data-slot=\"{}\" id=\"<k>-trigger\">`", t),
                format!("opens on {}; `<k>` names the instance", gesture),
            ));
            rows.push((
                format!("`<template id=\"<k>-portal\">`"),
                format!(
                    "holds the overlay/content subtree (`data-slot=\"{}\"` …) that the glue mounts into `<body>` while open",
                    c
                ),
            ));
        }
        "menu" => {
            rows.push((
                format!(
                    "`<… data-slot=\"{}\" id=\"<k>-trigger\" {}=\"<k>\">`",
                    t, f.attr
                ),
                format!("opens on {}", gesture),
            ));
            rows.push((
                "`<template id=\"<k>-tpl\">`".to_string(),
                format!("holds the `data-slot=\"{}\"` subtree", c),
            ));
            rows.push((
                format!(
                    "`<… data-slot=\"{}-sub-trigger\" id=\"<k>s0-trigger\" data-radixuigo-menu-subtrigger=\"<k>s0\">`",
                    comp
                ),
                "a sub menu inside a layer; its own `<template id=\"<k>s0-tpl\">`".to_string(),
            ));
        }
        "select" => {
            rows.push((
                format!("`<button data-slot=\"{}\" id=\"<k>-trigger\">`", t),
                "opens on click / Enter / Space / arrows; the `data-slot=\"select-value\"` child shows the selection".to_string(),
            ));
            rows.push((
                "`<template id=\"<k>-tpl\">`".to_string(),
                format!("holds the `data-slot=\"{}\"` listbox subtree", c),
            ));
        }
        "nav" => {
            rows.push((
                format!(
                    "`<… data-slot=\"{}\" id=\"<k>-trigger\" data-radixuigo-nav-trigger=\"<k>\">`",
                    t
                ),
                "opens on click".to_string(),
            ));
            rows.push((
                "`<template id=\"<k>-content-tpl\">`".to_string(),
                format!(
                    "holds the `data-slot=\"{}\"` subtree; the glue creates the shared viewport inside the root",
                    c
                ),
            ));
        }
        "inline" => {
            rows.push((
                "`<div data-slot=\"tabs\">` with `data-slot=\"tabs-trigger\" aria-controls=\"<panel-id>\"` and `data-slot=\"tabs-content\" id=\"<panel-id>\"`".to_string(),
                "no template: every panel is in the markup, inactive ones `hidden`; the glue wires every root it finds".to_string(),
            ));
        }
        "none" => {
            rows.push((
                format!("`<… data-slot=\"{}\">`", comp),
                "no ids, no templates: the glue wires every root it finds".to_string(),
            ));
        }
        _ => {}
    }
    #[allow(unused_assignments)] // the String::new() init is the fallback branch's value
    let mut api = String::new();
    if f.kind == "inline" {
        api = "`shadless.get(rootEl)` → `activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`)".to_string();
    } else if f.kind == "none" && comp == "slider" {
        api = "`shadless.get(rootEl)` → `values()`, `setValue(value, index)`; the root dispatches `shadless:change` (`detail: { values }`, live) and `shadless:commit` (once per gesture)".to_string();
    } else if f.kind == "none" && comp == "carousel" {
        api = "`shadless.get(rootEl)` → the embla api (`scrollNext()`, `scrollTo(i)`, `on(\"select\", …)`)".to_string();
    } else if f.kind == "none" {
        api = String::new();
    } else {
        api = "`shadless.get(\"#<k>-trigger\")` → `open()`, `close()`, `toggle()`, `isOpen()`".to_string();
        if f.kind == "select" {
            api += ", `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger also dispatches `shadless:change` (`detail: { value, label, item }`). An option's value is its `value` / `data-value` attribute or id — React's value prop never reaches the DOM, so add `data-value` to options whose value differs from their label";
        }
    }
    let mut b = String::new();
    b.push_str("\n**Behavior protocol**\n\n");
    b.push_str("The glue is data-driven: it scans the page for the markup shape below and wires every instance it finds — several per page, any ids you like. The demos on this page were generated from the React original by the same rules.\n\n");
    b.push_str("| Markup | Meaning |\n| --- | --- |\n");
    for r in rows {
        b.push_str(&format!("| {} | {} |\n", r.0, r.1));
    }
    // Only the kinds whose table above actually has a <template> row.
    if f.kind != "inline" && f.kind != "none" {
        b.push_str("\nContent that React would render inside the component's portal lives in the `<template>`; the glue clones it into `<body>` while open and removes it on close, exactly as radix mounts and unmounts.\n");
    }
    if !api.is_empty() {
        b.push_str(&format!("\n**From code:** {}. `shadless.get` accepts an element or a selector and walks up from any element inside the instance.", api));
        if f.kind != "inline" && f.kind != "none" {
            b.push_str(" The trigger dispatches `shadless:open` / `shadless:close` (bubbling, `detail: { component, api }`) on every open and close, however it was caused.");
        }
        b.push('\n');
    }
    b
}

#[derive(Clone, Copy)]
pub struct TrivialEnt {
    pub state: &'static str,
    pub events: &'static str,
    pub form: &'static str,
    pub keys: &'static str,
}

pub fn trivial(comp: &str) -> Option<TrivialEnt> {
    Some(match comp {
        "checkbox" => TrivialEnt {
            state: "`role=\"checkbox\"` root with `aria-checked` + `data-state=\"checked|unchecked\"`; the `checkbox-indicator` mounts from a `<template data-for=\"checkbox-indicator\">` while checked (radix Presence)",
            events: "the root dispatches `shadless:change` (`detail: { checked }`)",
            form: "a `name` attribute submits its `value` (default `on`) while checked",
            keys: "Space / click toggles",
        },
        "switch" => TrivialEnt {
            state: "`role=\"switch\"` root with `aria-checked` + `data-state`; the `switch-thumb` mirrors `data-state`",
            events: "the root dispatches `shadless:change` (`detail: { checked }`)",
            form: "a `name` attribute submits its `value` (default `on`) while checked",
            keys: "Space / click toggles",
        },
        "toggle" => TrivialEnt {
            state: "`aria-pressed` + `data-state=\"on|off\"` on the root",
            events: "the root dispatches `shadless:change` (`detail: { pressed }`)",
            form: "",
            keys: "Space / click toggles",
        },
        "radio-group" => TrivialEnt {
            state: "`role=\"radiogroup\"` root; items are `role=\"radio\"` with `aria-checked` + `data-state`, the checked one carries the `radio-group-indicator` mounted from `<template data-for=\"radio-group-indicator\">`; an item's value is its `value` / `data-value` attribute or id",
            events: "the root dispatches `shadless:change` (`detail: { value, item }`)",
            form: "a `name` attribute on the root submits the checked item's value",
            keys: "arrows / Home / End move focus over enabled items (rtl-aware); with nothing checked an arrow also checks its target (radix)",
        },
        "toggle-group" => TrivialEnt {
            state: "`role=\"group\"` root; single mode items are `role=\"radio\"` with `aria-checked`, multiple mode items carry `aria-pressed`; `data-state=\"on|off\"` in both",
            events: "the root dispatches `shadless:change` (`detail: { value, item }` — `value` is the on item's value, or an array in multiple mode)",
            form: "",
            keys: "arrows / Home / End move focus over enabled items (rtl-aware); Space / Enter / click selects",
        },
        "accordion" => TrivialEnt {
            state: "root `data-type=\"single|multiple\"`; each `accordion-trigger` carries `aria-expanded` + `data-state=\"open|closed\"` and `aria-controls` → its `accordion-content`, which is `hidden` while closed (kept in the DOM — radix unmounts it)",
            events: "each trigger dispatches `shadless:open` / `shadless:close` (a sibling closed by single mode gets its own `close`)",
            form: "",
            keys: "arrows / Home / End move focus between triggers (rtl-aware); Enter / Space / click toggles",
        },
        "collapsible" => TrivialEnt {
            state: "`collapsible-trigger` carries `aria-expanded` + `data-state` and `aria-controls` → the `collapsible-content`, `hidden` while closed",
            events: "the trigger dispatches `shadless:open` / `shadless:close`",
            form: "",
            keys: "Enter / Space / click toggles",
        },
        "avatar" => TrivialEnt {
            state: "`avatar-image` is shown once loaded; on error or while loading the `avatar-fallback` stays (radix Presence) — settled at init and by `shadless.refresh(el)` for injected markup",
            events: "",
            form: "",
            keys: "",
        },
        _ => return None,
    })
}

fn upper_first(s: &str) -> String {
    if s.is_empty() {
        return s.to_string();
    }
    let mut c = s.chars();
    c.next().unwrap().to_uppercase().collect::<String>() + c.as_str()
}

pub fn trivial_mdx(comp: &str) -> String {
    let Some(t) = trivial(comp) else {
        return String::new();
    };
    let mut b = String::new();
    b.push_str("\n**Behavior**\n\n");
    b.push_str(&format!(
        "Delegated from the base — no handle (`shadless.get(el)` is `null`): the state is the attribute radix renders and `el.click()` drives it. {}.",
        t.state
    ));
    if !t.keys.is_empty() {
        b.push_str(&format!(" Keys: {}.", t.keys));
    }
    b.push('\n');
    if !t.events.is_empty() {
        b.push_str(&format!(
            "\n{}, bubbling, after the state change, whichever path caused it.",
            upper_first(t.events)
        ));
    }
    if !t.form.is_empty() {
        b.push_str(&format!(
            "\n\nForms: {}; `form.reset()` restores the initial state.",
            t.form
        ));
    }
    b.push('\n');
    b
}

#[derive(Clone, Debug)]
pub struct CvaAxisRow {
    pub slot: String,
    pub attr: String,
    pub values: Vec<String>,
    pub def: String,
}

/// cvaAxisRows: cva variant axes flattened to (slot, attribute, values,
/// default) rows, in the IR's declaration order.
pub fn cva_axis_rows(ir: &crate::emit::css::CssIrComponent) -> Vec<CvaAxisRow> {
    let by_slot = crate::emit::css::cva_slot(ir);
    let mut out: Vec<CvaAxisRow> = Vec::new();
    for var_name in ir.cva.keys() {
        let Some((table, slot)) = by_slot.get(&var_name) else {
            continue;
        };
        if slot.is_empty() {
            continue;
        }
        for axis in table.axis_order() {
            out.push(CvaAxisRow {
                slot: slot.clone(),
                attr: axis.clone(),
                values: table.values(&axis).keys().cloned().collect(),
                def: table.defaults.get(&axis).cloned().unwrap_or_default(),
            });
        }
    }
    out
}

/// cvaAxisTableMdx: the one rendering of a component's cva axes, sourced from
/// the IR's own value order and defaults.
pub fn cva_axis_table_mdx(comp: &str, axes: &[CvaAxisRow]) -> String {
    let mut b = String::new();
    b.push_str(&format!(
        "Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — check `dist/css/{}.css` for any other `data-*` selector on these slots.\n\n",
        comp
    ));
    b.push_str("| Slot | Attribute | Values | Default |\n| --- | --- | --- | --- |\n");
    for a in axes {
        let vals: Vec<String> = a.values.iter().map(|v| format!("`{}`", v)).collect();
        b.push_str(&format!(
            "| `{}` | `data-{}` | {} | `{}` |\n",
            a.slot,
            a.attr,
            vals.join(", "),
            a.def
        ));
    }
    b
}

pub fn api_reference_mdx(
    comp: &str,
    slots: &[String],
    axes: &[CvaAxisRow],
    tier: &str,
    leaked: bool,
    root: &Path,
) -> String {
    let t = trivial(comp);
    let f = family(comp);
    let mut rows: Vec<String> = Vec::new();
    for s in slots {
        rows.push(format!("| `data-slot=\"{}\"` |", s));
    }
    let slot_table = if rows.is_empty() {
        String::new()
    } else {
        format!("\n| Slot |\n| --- |\n{}\n", rows.join("\n"))
    };
    let mut runtime = String::new();
    if let Some(t) = t {
        runtime = format!("\n**Runtime:** {}.", t.state);
        if !t.keys.is_empty() {
            runtime += &format!(" Keys: {}.", t.keys);
        }
        if !t.events.is_empty() {
            runtime += &format!(" {}.", upper_first(t.events));
        }
        if !t.form.is_empty() {
            runtime += &format!(" Forms: {}.", t.form);
        }
        runtime += " No handle — `shadless.get(el)` returns `null`; `el.click()` is the driver.\n";
    } else if let Some(f) = f {
        let api = match f.kind {
            "inline" => "`activate(i)`, `active()`; the root dispatches `shadless:change` (`detail: { index, trigger }`)".to_string(),
            _ if comp == "slider" => "`values()`, `setValue(value, index)`; the root dispatches `shadless:change` (live) and `shadless:commit` (once per gesture) with `detail: { values }`; a `name` attribute submits one input per thumb".to_string(),
            _ if comp == "carousel" => "the embla api (`scrollNext()`, `scrollTo(i)`, `on(\"select\", …)`)".to_string(),
            _ if comp == "scroll-area" => String::new(),
            _ => {
                let mut a = "`open()`, `close()`, `toggle()`, `isOpen()`".to_string();
                if f.kind == "select" {
                    a += ", `select(optionEl)`, `value()`, `label()`, `selected()`; the trigger dispatches `shadless:change` (`detail: { value, label, item }`); a `name` attribute submits the selected value";
                }
                a += "; the trigger dispatches `shadless:open` / `shadless:close`";
                a
            }
        };
        if !api.is_empty() {
            runtime = format!(
                "\n**Runtime:** `shadless.get(el)` (element or selector, any element inside the instance) → {}. Markup protocol: see Installation → Behavior protocol.\n",
                api
            );
        } else {
            runtime = "\n**Runtime:** wired from `data-slot` alone — no handle, no events; see Installation → Behavior protocol.\n".to_string();
        }
    } else if leaked {
        let mut b = String::new();
        b.push_str("\n**Runtime:** ");
        if tier == "static" {
            b.push_str("no JavaScript — this is markup + CSS. ");
        }
        if !axes.is_empty() {
            b.push_str(&cva_axis_table_mdx(comp, axes));
        } else if root.join("dist/css").join(format!("{}.css", comp)).exists() {
            b.push_str(&format!(
                "No `cva`-declared variants. Check `dist/css/{}.css` for any `data-*` attribute this slot's styling depends on.\n",
                comp
            ));
        } else {
            // aspect-ratio, collapsible and direction ship no stylesheet of
            // their own; the page's own Installation section says exactly that
            // two hundred lines earlier.
            b.push_str("No `cva`-declared variants, and no stylesheet of its own — the styling rides the core theme and utilities in `shadless`.\n");
        }
        // Tier is not the question the sentence asks.
        if root.join("dist/js").join(format!("{}.js", comp)).exists() {
            b.push_str("See Installation → Files this component needs for the JavaScript this component requires.\n");
        }
        runtime = b;
    }
    // The `leaked` branch already wrote it; the other two never did.
    if !leaked && !axes.is_empty() {
        runtime += &format!("\n{}", cva_axis_table_mdx(comp, axes));
    }
    if slot_table.is_empty() && runtime.is_empty() {
        return String::new();
    }
    let preamble = if leaked {
        "\n**shadless surface** — every node is a `data-slot` attribute in the shipped markup.\n"
    } else {
        "\n**shadless surface** — every node is a `data-slot` attribute in the shipped markup; state lives in the attributes radix renders (`data-state`, `aria-*`), never in classes.\n"
    };
    format!("{}{}{}", preamble, slot_table, runtime)
}

// ---------------------------------------------------------------- demo scripts

const PREPAINT_SIG: &str = "var k=\"shadless-docs-theme\"";

#[derive(Default)]
pub struct DemoScripts {
    pub src_scripts: Vec<String>, // local script srcs (js/<name>.js + shadless.js)
    pub inline_scripts: Vec<String>,
}

pub fn extract_demo_scripts(html: &str) -> DemoScripts {
    let mut out = DemoScripts::default();
    for caps in re_demo_src_script().captures_iter(html) {
        out.src_scripts.push(caps[1].to_string()); // JS pushed every match, no dedupe
    }
    for caps in re_inline_script().captures_iter(html) {
        let body = caps[1].trim();
        if body.is_empty() {
            continue;
        }
        if body.contains(PREPAINT_SIG) {
            continue; // theme persistence, not demo JS
        }
        out.inline_scripts.push(dedent_script(&caps[1]));
    }
    out
}

/// readDemoScripts is the file-backed helper docs-build uses.
pub fn read_demo_scripts(path: &Path) -> DemoScripts {
    match std::fs::read_to_string(path) {
        Ok(b) => extract_demo_scripts(&b),
        Err(_) => DemoScripts::default(),
    }
}

/// dedentScript removes the indentation the demo page's HTML gives an inline
/// <script>.
pub fn dedent_script(body: &str) -> String {
    let lines: Vec<&str> = body.trim_matches('\n').split('\n').collect();
    let mut indent: isize = -1;
    for l in &lines {
        if l.trim().is_empty() {
            continue;
        }
        let n = (l.len() - l.trim_start_matches([' ', '\t']).len()) as isize;
        if indent < 0 || n < indent {
            indent = n;
        }
    }
    let mut out: Vec<String> = Vec::new();
    if indent > 0 {
        for l in &lines {
            if l.len() >= indent as usize {
                out.push(l[indent as usize..].to_string());
            } else {
                out.push(l.trim_start_matches([' ', '\t']).to_string());
            }
        }
    } else {
        out = lines.iter().map(|s| s.to_string()).collect();
    }
    out.join("\n").trim_end_matches([' ', '\t', '\n']).to_string()
}

// ---------------------------------------------------------------- catalog helpers

/// stripFences blanks code fences: they are prose, not rendered tags. Replaced
/// with spaces (not removed) so offsets and line numbers stay stable.
pub fn strip_fences(text: &str) -> String {
    re_fence()
        .replace_all(text, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap().as_str();
            m.chars()
                .map(|c| if c == '\n' { '\n' } else { ' ' })
                .collect::<String>()
        })
        .into_owned()
}

/// attrOf reads one attribute out of a tag's attribute text. The (^|\s) anchor
/// is deliberate: a bare \b would let data-name= match name=.
pub fn attr_of(attrs: &str, name: &str) -> (String, bool) {
    let re = Regex::new(&format!(
        "(?:^|[ \\t]){}=\"([^\"]*)\"",
        regex::escape(name)
    ))
    .unwrap();
    match re.captures(attrs) {
        Some(m) => (m[1].to_string(), true),
        None => (String::new(), false),
    }
}

/// optStr models a JS optional: a key may be absent entirely (Present false)
/// or present with a null value.
#[derive(Clone, Debug, Default)]
pub struct OptStr {
    pub present: bool,
    pub null: bool,
    pub val: String,
}

pub fn attr_or_null(attrs: &str, name: &str) -> OptStr {
    let (v, ok) = attr_of(attrs, name);
    if !ok || v.is_empty() {
        return OptStr {
            present: true,
            null: true,
            val: String::new(),
        };
    }
    OptStr {
        present: true,
        null: false,
        val: v,
    }
}

pub fn attr_or_absent(attrs: &str, name: &str) -> OptStr {
    let (v, ok) = attr_of(attrs, name);
    if !ok {
        return OptStr::default();
    }
    OptStr {
        present: true,
        null: false,
        val: v,
    }
}

// ---------------------------------------------------------------- grey list

/// FT5: canonical grey list — the radix meta.json entries with NO shadless
/// implementation. Cross-checked against docs/catalog.json at every build.
pub fn grey_components() -> &'static [&'static str] {
    &[
        // 10 tombstones (Wave D/E externals; form has no mdx page; the medium
        // pair menubar/navigation-menu emitted since — contract-tested glue)
        "calendar", "chart", "combobox", "command", "drawer", "form", "input-otp",
        "resizable", "sidebar", "sonner",
        // 5 FT6 grey dispositions (0 implement / 5 grey)
        "data-table", "date-picker", "questionnaire", "toast", "typography",
    ]
}

#[derive(Deserialize, Default)]
pub struct DocsCatalog {
    #[serde(default)]
    pub previews: Vec<DocsCatalogPreview>,
    #[serde(default)]
    pub sources: Vec<DocsCatalogSource>,
}

#[derive(Deserialize, Default)]
pub struct DocsCatalogPreview {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    // Go tolerates `"demoPath": null` (tombstoned/unavailable previews);
    // serde does not — Option with None read as "".
    #[serde(default, rename = "demoPath")]
    pub demo_path: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct DocsCatalogSource {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Deserialize, Default)]
pub struct DocsMeta {
    #[serde(default)]
    pub pages: Vec<String>,
}
