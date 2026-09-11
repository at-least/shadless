//! Port of pipeline/resolve_skins.go — upstream generation parity at the
//! SOURCE level. cn-X in the skin's @apply body expands in place, allowlisted
//! names stay, marker-only names styled by no skin are DROPPED, and every
//! class-string literal then passes twMerge. The output tree
//! build/resolved-ui/{ui,ui-rtl} mirrors bases/radix, with ui-rtl additionally
//! RTL-transformed. lib/ and hooks/ copy verbatim.

use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

const RESOLVE_SRC: &str = ".upstream/shadcn-ui/apps/v4/registry/bases/radix";
const RESOLVE_OUT: &str = "build/resolved-ui";

fn cn_token_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(^|[ \t\n\f\r ])cn-[a-z0-9-]+").unwrap())
}
fn rtl_mappable() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(
            r"(^|[ \t\n\f\r ])(ml-|mr-|pl-|pr-|left-|right-|rounded-[tlbr]+-|border-[lr]\b|text-(left|right)|translate-x-|space-x-|divide-x-|float-|clear-|origin-|scroll-[mp][lr]-|inset-[lr]-|cursor-[we]-resize)",
        )
        .unwrap()
    })
}
fn class_attr_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"class="[^"]*""#).unwrap())
}
fn re_cn_prefix() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(^|[ \t\n\f\r ])cn-").unwrap())
}

/// expandClassString: cn-X in the skin's @apply body expands in place,
/// allowlisted names stay, marker-only names styled by no skin are DROPPED.
pub fn expand_class_string(str_: &str) -> String {
    expand_class_string_with(crate::emit::skin_data(), str_)
}

/// The same with an explicit skin table (the Go test swaps skinData in).
pub fn expand_class_string_with(skin: &crate::emit::SkinData, str_: &str) -> String {
    let toks: Vec<&str> = str_.split_whitespace().collect();
    let mut has_cn = false;
    for t in &toks {
        if t.starts_with("cn-") {
            has_cn = true;
            break;
        }
    }
    if !has_cn {
        return str_.to_string();
    }
    let mut expanded: Vec<String> = Vec::new();
    for t in toks {
        if !t.starts_with("cn-") {
            expanded.push(t.to_string());
            continue;
        }
        if skin.allowlist.contains(t) {
            expanded.push(t.to_string());
            continue;
        }
        if let Some(body) = skin.map.get(t) {
            expanded.extend(body.split_whitespace().map(|s| s.to_string()));
        }
        // styled-by-no-skin markers drop out (no entries appended)
    }
    crate::twmerge::merge(&expanded.join(" "))
}

// --- RTL transform (upstream transform-rtl.ts parity) ----------------------

const RTL_MAPPINGS: [(&str, &str); 38] = [
    ("-ml-", "-ms-"), ("-mr-", "-me-"), ("ml-", "ms-"), ("mr-", "me-"),
    ("pl-", "ps-"), ("pr-", "pe-"), ("-left-", "-start-"), ("-right-", "-end-"),
    ("left-", "start-"), ("right-", "end-"), ("inset-l-", "inset-inline-start-"),
    ("inset-r-", "inset-inline-end-"), ("rounded-tl-", "rounded-ss-"),
    ("rounded-tr-", "rounded-se-"), ("rounded-bl-", "rounded-es-"),
    ("rounded-br-", "rounded-ee-"), ("rounded-l-", "rounded-s-"),
    ("rounded-r-", "rounded-e-"), ("border-l-", "border-s-"), ("border-r-", "border-e-"),
    ("border-l", "border-s"), ("border-r", "border-e"), ("text-left", "text-start"),
    ("text-right", "text-end"), ("scroll-ml-", "scroll-ms-"), ("scroll-mr-", "scroll-me-"),
    ("scroll-pl-", "scroll-ps-"), ("scroll-pr-", "scroll-pe-"),
    ("float-left", "float-start"), ("float-right", "float-end"),
    ("clear-left", "clear-start"), ("clear-right", "clear-end"),
    ("origin-top-left", "origin-top-start"), ("origin-top-right", "origin-top-end"),
    ("origin-bottom-left", "origin-bottom-start"), ("origin-bottom-right", "origin-bottom-end"),
    ("origin-left", "origin-start"), ("origin-right", "origin-end"),
];

const RTL_TRANSLATE_X: [(&str, &str); 2] = [("-translate-x-", "translate-x-"), ("translate-x-", "-translate-x-")];
const RTL_REVERSE: [(&str, &str); 2] = [("space-x-", "space-x-reverse"), ("divide-x-", "divide-x-reverse")];
const RTL_SWAP: [(&str, &str); 2] = [("cursor-w-resize", "cursor-e-resize"), ("cursor-e-resize", "cursor-w-resize")];

const RTL_LOGICAL_SLIDE: [(&str, &str, &str); 4] = [
    ("data-[side=inline-start]", "slide-in-from-right", "slide-in-from-end"),
    ("data-[side=inline-start]", "slide-out-to-right", "slide-out-to-end"),
    ("data-[side=inline-end]", "slide-in-from-left", "slide-in-from-start"),
    ("data-[side=inline-end]", "slide-out-to-left", "slide-out-to-start"),
];

const POSITIONING_PREFIXES: [&str; 4] = ["-left-", "-right-", "left-", "right-"];

/// splitClassName mirrors upstream splitClassName: bracket-aware last-colon
/// split for the variant, then first '/' outside brackets separates alpha.
pub fn split_class_name(cls: &str) -> (Option<String>, Option<String>, Option<String>) {
    let mut last_colon: isize = -1;
    let mut depth = 0isize;
    for (i, c) in cls.char_indices().rev() {
        match c {
            ']' => depth += 1,
            '[' => depth -= 1,
            ':' => {
                if depth == 0 {
                    last_colon = i as isize;
                    break;
                }
            }
            _ => {}
        }
    }
    let rest = if last_colon != -1 {
        let variant = cls[..last_colon as usize].to_string();
        let rest = &cls[last_colon as usize + 1..];
        (Some(variant), rest)
    } else {
        (None, cls)
    };
    let (variant, rest) = rest;
    let mut slash: isize = -1;
    let mut depth = 0isize;
    for (i, c) in rest.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            '/' => {
                if depth == 0 {
                    slash = i as isize;
                    break;
                }
            }
            _ => {}
        }
    }
    if slash == -1 {
        return (variant, Some(rest.to_string()), None);
    }
    (
        variant,
        Some(rest[..slash as usize].to_string()),
        Some(rest[slash as usize + 1..].to_string()),
    )
}

/// variantPrefix builds "variant:value", or "rtl:variant:value" when rtl ==
/// true.
pub fn variant_prefixed(variant: &Option<String>, value: &str, rtl: bool) -> String {
    match variant {
        None => {
            if rtl {
                format!("rtl:{}", value)
            } else {
                value.to_string()
            }
        }
        Some(v) => {
            if rtl {
                format!("rtl:{}:{}", v, value)
            } else {
                format!("{}:{}", v, value)
            }
        }
    }
}

/// applyRtlMapping mirrors applyRtlMapping in tools/resolve-skins.mjs branch
/// for branch.
pub fn apply_rtl_mapping(input: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    for cls in input.split_whitespace() {
        if cls.starts_with("rtl:") || cls.starts_with("ltr:") {
            out.push(cls.to_string());
            continue;
        }
        if cls == "cn-rtl-flip" {
            out.push("rtl:rotate-180".to_string());
            continue;
        }
        let (variant, value_p, alpha) = split_class_name(cls);
        let Some(value) = value_p else {
            out.push(cls.to_string());
            continue;
        };
        let m = |v: &str| -> String {
            if let Some(a) = &alpha {
                format!("{}/{}", v, a)
            } else {
                v.to_string()
            }
        };
        let mut done = false;
        for p in RTL_TRANSLATE_X {
            if value.starts_with(p.0) {
                let rv = value.replacen(p.0, p.1, 1);
                out.push(cls.to_string());
                out.push(variant_prefixed(&variant, &m(&rv), true));
                done = true;
                break;
            }
        }
        if !done {
            for p in RTL_REVERSE {
                if value.starts_with(p.0) {
                    out.push(cls.to_string());
                    out.push(variant_prefixed(&variant, p.1, true));
                    done = true;
                    break;
                }
            }
        }
        if !done {
            for p in RTL_SWAP {
                if value == p.0 {
                    out.push(cls.to_string());
                    out.push(variant_prefixed(&variant, p.1, true));
                    done = true;
                    break;
                }
            }
        }
        if !done {
            for p in RTL_LOGICAL_SLIDE {
                if let Some(v) = &variant {
                    if v.contains(p.0) && value.starts_with(p.1) {
                        let mapped = value.replacen(p.1, p.2, 1);
                        out.push(variant_prefixed(&variant, &m(&mapped), false));
                        done = true;
                        break;
                    }
                }
            }
        }
        if done {
            continue;
        }
        let is_phys_side = variant.as_ref().map(|v| {
            v.contains("data-[side=left]") || v.contains("data-[side=right]")
        }).unwrap_or(false);
        let mut mapped = value.clone();
        for p in RTL_MAPPINGS {
            if is_phys_side && has_any_prefix(p.0, &POSITIONING_PREFIXES) {
                continue;
            }
            if value.starts_with(p.0) {
                if !p.0.ends_with('-') && value != p.0 {
                    continue;
                }
                mapped = value.replacen(p.0, p.1, 1);
                break;
            }
        }
        out.push(variant_prefixed(&variant, &m(&mapped), false));
    }
    out.join(" ")
}

fn has_any_prefix(s: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|p| s.starts_with(p))
}

struct ResolveEdit {
    start: usize,
    end: usize,
    value: String,
}

/// resolveSource transforms one tsx file: every StringLiteral carrying a
/// cn-* token (or, when rtl, an RTL-mappable class pattern) gets replaced,
/// spliced by source offset in reverse order.
pub fn resolve_source(src: &str, rtl: bool) -> (String, usize) {
    let mut edits: Vec<ResolveEdit> = Vec::new();
    for sp in crate::tsx::string_literals(src) {
        if sp.template {
            continue; // only plain strings; templates with interpolation are code
        }
        let content = sp.content(src);
        let mut next = String::new();
        if cn_token_re().is_match(&content) {
            next = expand_class_string(&content);
            if rtl {
                next = apply_rtl_mapping(&next);
            }
        } else if rtl && rtl_mappable().is_match(&content) {
            next = apply_rtl_mapping(&content);
        } else {
            continue;
        }
        if next != content {
            // jsonString == JSON.stringify semantics (escapes control chars,
            // passes non-ASCII through raw)
            edits.push(ResolveEdit {
                start: sp.start,
                end: sp.end,
                value: crate::jsonorder::json_string(&next),
            });
        }
    }
    let n = edits.len();
    let mut out = src.to_string();
    edits.sort_by(|a, b| b.start.cmp(&a.start));
    for e in edits {
        out = format!(
            "{}{}{}",
            &out[..e.start - 1],
            e.value,
            &out[e.end + 1..]
        );
    }
    (out, n)
}

/// resolveFixtureHtml expands cn-* in kernel fixtures' class attributes.
/// Idempotent.
pub fn resolve_fixture_html(html: &str) -> String {
    class_attr_re()
        .replace_all(html, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap().as_str();
            let inner = &m["class=\"".len()..m.len() - 1];
            if !re_cn_prefix().is_match(inner) {
                return m.to_string();
            }
            let next = expand_class_string(inner);
            format!("class=\"{}\"", next)
        })
        .into_owned()
}

/// runResolveSkins is the `pipeline resolve-skins [--fixtures]` entry.
pub fn run_resolve_skins(root: &Path, args: &[String]) -> i32 {
    crate::emit::load_skin();
    // Go os.RemoveAll tolerates a missing target AND unlinks a regular
    // file/symlink sitting at the path; Rust's remove_dir_all errors on
    // both (NotFound / NotADirectory — the file case surfaced by the same
    // probe). Match the Go semantics exactly.
    match std::fs::remove_dir_all(root.join(RESOLVE_OUT)) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotADirectory => {
            if let Err(e2) = std::fs::remove_file(root.join(RESOLVE_OUT)) {
                eprintln!("resolve-skins: {}", e2);
                return 1;
            }
        }
        Err(e) => {
            eprintln!("resolve-skins: {}", e);
            return 1;
        }
    }
    if let Err(e) = std::fs::create_dir_all(root.join(RESOLVE_OUT)) {
        eprintln!("resolve-skins: {}", e);
        return 1;
    }
    let (mut files, mut edits) = (0usize, 0usize);
    let copy_tree = |dir: &str,
                     out_dir: &str,
                     transform: Option<fn(&str) -> (String, usize)>,
                     files: &mut usize,
                     edits: &mut usize|
     -> Result<(), String> {
        let walk_root = root.join(RESOLVE_SRC).join(dir);
        for e in walkdir::WalkDir::new(&walk_root) {
            let e = e.map_err(|e| e.to_string())?;
            let rel = e
                .path()
                .strip_prefix(&walk_root)
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .into_owned();
            let o = root.join(RESOLVE_OUT).join(out_dir).join(&rel);
            if e.file_type().is_dir() {
                std::fs::create_dir_all(&o).map_err(|e| e.to_string())?;
                continue;
            }
            let src = std::fs::read_to_string(e.path()).map_err(|e| e.to_string())?;
            if let Some(transform) = transform {
                if e.path().extension().map(|x| x == "tsx").unwrap_or(false) {
                    let (out, n) = transform(&src);
                    *files += 1;
                    *edits += n;
                    std::fs::write(&o, out).map_err(|e| e.to_string())?;
                    continue;
                }
            }
            std::fs::write(&o, src).map_err(|e| e.to_string())?;
        }
        Ok(())
    };
    if let Err(e) = copy_tree("ui", "ui", Some(|s| resolve_source(s, false)), &mut files, &mut edits) {
        eprintln!("resolve-skins: {}", e);
        return 1;
    }
    if let Err(e) = copy_tree("ui", "ui-rtl", Some(|s| resolve_source(s, true)), &mut files, &mut edits) {
        eprintln!("resolve-skins: {}", e);
        return 1;
    }
    if let Err(e) = copy_tree("lib", "lib", None, &mut files, &mut edits) {
        eprintln!("resolve-skins: {}", e);
        return 1;
    }
    if let Err(e) = copy_tree("hooks", "hooks", None, &mut files, &mut edits) {
        eprintln!("resolve-skins: {}", e);
        return 1;
    }

    let fixtures = args.iter().any(|a| a == "--fixtures");
    if fixtures {
        let mut fx = 0usize;
        if let Ok(ents) = std::fs::read_dir(root.join("src/kernel")) {
            for e in ents.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if !n.ends_with(".html") {
                    continue;
                }
                let p = root.join("src/kernel").join(&n);
                let Ok(html) = std::fs::read_to_string(&p) else {
                    continue;
                };
                let next = resolve_fixture_html(&html);
                if next != html {
                    if let Err(e) = std::fs::write(&p, next) {
                        eprintln!("resolve-skins --fixtures: {}", e);
                        return 1;
                    }
                    fx += 1;
                }
            }
        }
        println!("resolve-skins: fixtures rewritten: {}", fx);
    }
    println!(
        "resolve-skins: {} ui files resolved ({} class strings), tree at {}",
        files, edits, RESOLVE_OUT
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Go TestUnitApplyRtlMapping — cases lifted from upstream's own
    /// transform-rtl.test.ts (packages/shadcn/src/utils/transformers), the
    /// independent oracle for this port.
    #[test]
    fn unit_apply_rtl_mapping() {
        let cases: [(&str, &str, &str); 39] = [
            ("margin", "ml-2 mr-4 -ml-2 -mr-4", "ms-2 me-4 -ms-2 -me-4"),
            ("padding", "pl-2 pr-4", "ps-2 pe-4"),
            ("positioning", "left-0 right-0 -left-2 -right-2", "start-0 end-0 -start-2 -end-2"),
            ("inset", "inset-l-0 inset-r-0", "inset-inline-start-0 inset-inline-end-0"),
            ("border", "border-l border-r border-l-2 border-r-2", "border-s border-e border-s-2 border-e-2"),
            ("rounded corners", "rounded-l-md rounded-tl-md rounded-br-md", "rounded-s-md rounded-ss-md rounded-ee-md"),
            ("text align", "text-left text-right", "text-start text-end"),
            ("scroll margin/padding", "scroll-ml-2 scroll-pr-2", "scroll-ms-2 scroll-pe-2"),
            ("float", "float-left float-right", "float-start float-end"),
            ("clear", "clear-left clear-right", "clear-start clear-end"),
            ("origin", "origin-left origin-top-right", "origin-start origin-top-end"),
            ("variant prefix preserved", "hover:ml-2 sm:md:ml-2", "hover:ms-2 sm:md:ms-2"),
            ("named group selector with data attr", "sm:group-data-[size=default]/alert-dialog-content:text-left",
                "sm:group-data-[size=default]/alert-dialog-content:text-start"),
            ("arbitrary values", "ml-[10px] left-[50%]", "ms-[10px] start-[50%]"),
            ("alpha modifier reattached", "ml-2/50", "ms-2/50"),
            ("unrelated classes untouched", "bg-red-500 flex mx-auto px-4", "bg-red-500 flex mx-auto px-4"),
            ("partial-match guard: suffix after the mapped prefix blocks it",
                "border-ring border-ring/50 border-lime-500 scroll-m-4",
                "border-ring border-ring/50 border-lime-500 scroll-m-4"),
            ("translate-x gains a mirrored rtl: variant", "-translate-x-1/2", "-translate-x-1/2 rtl:translate-x-1/2"),
            ("translate-x positive form", "translate-x-full", "translate-x-full rtl:-translate-x-full"),
            ("translate-x with a variant prefix", "after:-translate-x-1/2", "after:-translate-x-1/2 rtl:after:translate-x-1/2"),
            ("translate-y is not translate-x", "-translate-y-1/2 translate-y-full", "-translate-y-1/2 translate-y-full"),
            ("space-x/divide-x gain rtl:-reverse", "space-x-4 divide-x-2", "space-x-4 rtl:space-x-reverse divide-x-2 rtl:divide-x-reverse"),
            ("space-x with a variant prefix", "md:space-x-4", "md:space-x-4 rtl:md:space-x-reverse"),
            ("space-y/divide-y untouched", "space-y-4 divide-y-2", "space-y-4 divide-y-2"),
            ("cursor resize swaps direction", "cursor-w-resize cursor-e-resize", "cursor-w-resize rtl:cursor-e-resize cursor-e-resize rtl:cursor-w-resize"),
            ("cursor resize with a variant prefix", "hover:cursor-w-resize", "hover:cursor-w-resize rtl:hover:cursor-e-resize"),
            ("cn-rtl-flip marker, alone", "cn-rtl-flip", "rtl:rotate-180"),
            ("cn-rtl-flip marker, leading", "cn-rtl-flip size-4", "rtl:rotate-180 size-4"),
            ("cn-rtl-flip marker, trailing", "size-4 cn-rtl-flip", "size-4 rtl:rotate-180"),
            ("cn-rtl-flip combined with a real mapping", "cn-rtl-flip ml-2", "rtl:rotate-180 ms-2"),
            ("logical slide inside a logical side variant", "data-[side=inline-start]:slide-in-from-right-2",
                "data-[side=inline-start]:slide-in-from-end-2"),
            ("logical slide, the other side/direction", "data-[side=inline-end]:slide-out-to-left-2",
                "data-[side=inline-end]:slide-out-to-start-2"),
            ("slide inside a PHYSICAL side variant is untouched", "data-[side=left]:slide-in-from-right-2",
                "data-[side=left]:slide-in-from-right-2"),
            ("positioning inside a physical side variant is excluded", "data-[side=left]:-right-1 data-[side=right]:-left-1 data-[side=left]:right-0",
                "data-[side=left]:-right-1 data-[side=right]:-left-1 data-[side=left]:right-0"),
            ("non-positioning classes still map inside a physical side variant",
                "data-[side=left]:ml-2 data-[side=right]:pl-4 data-[side=left]:text-left",
                "data-[side=left]:ms-2 data-[side=right]:ps-4 data-[side=left]:text-start"),
            ("already rtl:-prefixed classes pass through untouched", "rtl:ml-2 rtl:text-right rtl:space-x-reverse",
                "rtl:ml-2 rtl:text-right rtl:space-x-reverse"),
            ("already ltr:-prefixed classes pass through untouched", "ltr:ml-2 ltr:text-left", "ltr:ml-2 ltr:text-left"),
            ("rtl:/ltr: classes skipped, others in the same string still map",
                "ml-2 rtl:mr-2", "ms-2 rtl:mr-2"),
            ("hand-written ltr:/rtl: translate pair left alone (both already prefixed)",
                "ltr:-translate-x-1/2 rtl:-translate-x-1/2", "ltr:-translate-x-1/2 rtl:-translate-x-1/2"),
        ];
        for (name, input, want) in cases {
            let got = apply_rtl_mapping(input);
            assert_eq!(got, want, "applyRtlMapping({:?})", name);
        }
    }

    /// Go TestUnitSplitClassName.
    #[test]
    fn unit_split_class_name() {
        let cases: [(&str, &str, Option<&str>, Option<&str>, Option<&str>); 7] = [
            ("bare utility, no variant, no alpha", "ml-2", None, Some("ml-2"), None),
            ("one variant", "hover:ml-2", Some("hover"), Some("ml-2"), None),
            ("stacked variants split at the LAST colon", "sm:md:ml-2", Some("sm:md"), Some("ml-2"), None),
            ("alpha modifier", "bg-red-500/50", None, Some("bg-red-500"), Some("50")),
            ("variant + alpha", "hover:bg-red-500/50", Some("hover"), Some("bg-red-500"), Some("50")),
            ("bracketed variant with an internal ':' does not split there",
                "data-[state=open]:bg-red-500/50", Some("data-[state=open]"), Some("bg-red-500"), Some("50")),
            ("a '/' inside brackets is not the alpha divider",
                "data-[state=open]:w-1/2", Some("data-[state=open]"), Some("w-1"), Some("2")),
        ];
        for (name, input, want_v, want_val, want_a) in cases {
            let (v, val, a) = split_class_name(input);
            assert_eq!(v.as_deref(), want_v, "{}: variant", name);
            assert_eq!(val.as_deref(), want_val, "{}: value", name);
            assert_eq!(a.as_deref(), want_a, "{}: alpha", name);
        }
    }

    /// Go TestUnitExpandClassString — synthetic skin table, as the Go test
    /// swaps skinData in.
    #[test]
    fn unit_expand_class_string() {
        let mut map = std::collections::HashMap::new();
        map.insert("cn-btn".to_string(), "bg-blue-500 text-white".to_string());
        let mut allowlist = std::collections::HashSet::new();
        allowlist.insert("cn-keep".to_string());
        let skin = crate::emit::SkinData { map, allowlist };
        let cases: [(&str, &str, &str); 5] = [
            ("no cn- token: returned verbatim, whitespace untouched", "flex  items-center", "flex  items-center"),
            ("allowlisted cn- name stays literal", "flex cn-keep", "flex cn-keep"),
            ("mapped cn- name expands to its @apply body", "cn-btn", "bg-blue-500 text-white"),
            ("marker cn- name (styled by no skin) drops out", "flex cn-ghost", "flex"),
            ("allowlist + map + marker together", "cn-keep cn-btn cn-ghost flex", "cn-keep bg-blue-500 text-white flex"),
        ];
        for (name, input, want) in cases {
            let got = expand_class_string_with(&skin, input);
            assert_eq!(got, want, "expandClassString({:?})", name);
        }
    }
}
