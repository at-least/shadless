//! Port of pipeline/docs_build.go: upstream mdx → Zola markdown (built by the
//! shadcn theme — see docs/site/). The text transform chain, the four MDX
//! shapes, the grey-list cross-check, content-map emission, index + section
//! pages. The ONE node dependency left is prettier's html printer for the
//! demo markup shown under each preview (tools/prettier-batch.mjs — one
//! subprocess per build).

use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{LazyLock, OnceLock};

use super::docs_transforms::{
    Guide, RE_API_REF_HEADING, RE_DATA_SLOT_ATTR, RE_DATA_SLOT_SET, api_reference_mdx,
    apply_jsx_overrides, apply_text_adjustments, cva_axis_rows, drop_react_import_fences,
    extract_demo_scripts, fence_shadow, fm_string, grey_components, guides,
    locate_api_reference_span, locate_changelog_span, locate_code_tabs_spans,
    locate_composition_span, locate_install_section, locate_message_scroller_js_span,
    locate_rtl_framework_span, locate_rtl_migrate_span, locate_usage_span,
    message_scroller_js_note, parse_frontmatter, protocol_mdx, read_demo_scripts, replace_span,
    resolve_docs_route, rewrite_inline_jsx_mentions, rewrite_leaked_jsx_fences,
    rewrite_utility_jsx_fences, rtl_framework_note, scan_guide_previews, ships_stylesheet,
    strip_imports, strip_imports_from_mixed_fences, trivial_mdx,
};
use crate::jsonorder::{Json, JsonObj, json_string};

const DOCS_RADIX_DIR: &str = "generated/docs-upstream/components/radix";
#[allow(dead_code)] // ported path constants; kept for parity with the Go docs build
const DOCS_ROOT: &str = "docs";
#[allow(dead_code)]
const SITE_ROOT: &str = "docs/site";
const CONTENT_ROOT: &str = "docs/site/content";
const STATIC_ROOT: &str = "docs/site/static";

static RE_FENCE_SPLIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)(```.*?```)").unwrap());
static RE_INLINE_CODE_SPAN: LazyLock<Regex> = LazyLock::new(|| Regex::new("`+[^`\n]*`+").unwrap());
static RE_JSX_COMPONENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<([A-Z][0-9A-Za-z_]*)").unwrap());
static RE_COMP_PREVIEW_ALL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<ComponentPreview\b.*?/>").unwrap());
static RE_COMP_SOURCE_ALL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<ComponentSource\b.*?/>").unwrap());
static RE_STEPS_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"</?Steps\b[^>]*>").unwrap());
static RE_STEP_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<Step>(.*?)</Step>").unwrap());
static RE_KBD_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"</?Kbd>").unwrap());
static RE_LINKED_CARD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<LinkedCard\b[^>]*>|</LinkedCard>").unwrap());
static RE_CLASS_NAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bclassName=").unwrap());
static RE_MD_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]*)\]\((/[^)\s]*)\)").unwrap());
static RE_JSX_ATTR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"([A-Za-z][0-9A-Za-z_-]*)=(?:"([^"]*)"|\{((?:[^{}]|\{[^}]*\})*)\})"#).unwrap()
});
static RE_BODY_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<body[^>]*>(.*?)</body>").unwrap());
static RE_SCRIPT_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<script.*?</script>").unwrap());
static RE_FM_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)^---\n.*?\n---\n").unwrap());
static RE_INIT_ALL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"shadless\.initAll\(\)").unwrap());
static RE_RADIX_LEGACY_PATH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https://www\.radix-ui\.com/docs/primitives/").unwrap());
static RE_CALLOUT_START: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<Callout\b").unwrap());

fn inline_code_shadow(text: &str) -> String {
    RE_INLINE_CODE_SPAN
        .replace_all(text, |caps: &regex::Captures| {
            " ".repeat(caps.get(0).unwrap().as_str().len())
        })
        .into_owned()
}

fn markup_shadow(text: &str) -> String {
    inline_code_shadow(&fence_shadow(text))
}

/// replaceMarkup runs re over the markup shadow and splices replacements into
/// the REAL text at the same offsets, right to left.
fn replace_markup(text: &str, re: &Regex, f: impl Fn(&str, &[String]) -> String) -> String {
    let shadow = markup_shadow(text);
    let locs: Vec<(usize, usize, Vec<String>)> = re
        .captures_iter(&shadow)
        .map(|caps| {
            let m = caps.get(0).unwrap();
            let groups: Vec<String> = caps
                .iter()
                .skip(1)
                .filter_map(|g| g.map(|g| text[g.start()..g.end()].to_string()))
                .collect();
            (m.start(), m.end(), groups)
        })
        .collect();
    let mut out = text.to_string();
    for (start, end, groups) in locs.into_iter().rev() {
        let whole = &text[start..end];
        let repl = f(whole, &groups);
        out = format!("{}{}{}", &out[..start], repl, &out[end..]);
    }
    out
}

/// parseAttrs: JSX attributes name="value" (expressions dropped — every one
/// upstream is a React icon element).
fn parse_attrs(tag: &str) -> HashMap<String, String> {
    let mut attrs: HashMap<String, String> = HashMap::new();
    for caps in RE_JSX_ATTR.captures_iter(tag) {
        // group 2 is the quoted value; group 3 the {expr} — only the quoted
        // form is kept (Go's m[2] is "" when the group did not participate)
        let val = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        if !val.is_empty() || caps[0].contains("=\"") {
            attrs.insert(caps[1].to_string(), val.to_string());
        }
    }
    attrs
}

/// stripImportsOutsideFences: split on fence blocks, strip only outside them.
fn strip_imports_outside_fences(src: &str) -> String {
    let segs: Vec<&str> = RE_FENCE_SPLIT.split(src).collect();
    let fences: Vec<String> = RE_FENCE_SPLIT
        .captures_iter(src)
        .map(|c| c[1].to_string())
        .collect();
    let mut b = String::new();
    for (i, seg) in segs.iter().enumerate() {
        b.push_str(&strip_imports(seg));
        if i < fences.len() {
            b.push_str(&fences[i]);
        }
    }
    b
}

// ---- the four MDX shapes --------------------------------------------------------

fn callout_kind(variant: &str) -> &'static str {
    match variant {
        "info" => "tip",
        "warning" => "warning",
        "danger" => "danger",
        _ => "tip",
    }
}

/// findCalloutOpen returns the [start, end) of a <Callout …> opening tag.
/// It is a scanner rather than `<Callout\b[^>]*>` because a JSX attribute value
/// is an expression that can contain ">".
fn find_callout_open(s: &str) -> (isize, isize) {
    let Some(m) = RE_CALLOUT_START.find(s) else {
        return (-1, -1);
    };
    let mut depth = 0isize;
    let mut quote: u8 = 0;
    for (i, c) in s[m.end()..].char_indices() {
        let i = m.end() + i;
        let cb = c as u32;
        if quote != 0 {
            if cb == quote as u32 {
                quote = 0;
            }
        } else if c == '"' || c == '\'' {
            quote = c as u8;
        } else if c == '{' {
            depth += 1;
        } else if c == '}' {
            if depth > 0 {
                depth -= 1;
            }
        } else if c == '>' && depth == 0 {
            return (m.start() as isize, (i + 1) as isize);
        }
    }
    (-1, -1) // an unterminated tag is not an opening tag
}

fn convert_callouts(text: &str, page: &str) -> Result<String, String> {
    let mut out = text.to_string();
    loop {
        let shadow = markup_shadow(&out);
        let (os_, oe) = find_callout_open(&shadow);
        if os_ < 0 {
            break;
        }
        let (open_s, open_e) = (os_ as usize, oe as usize);
        let Some(close) = shadow[open_e..].find("</Callout>") else {
            return Err(format!("{}: <Callout> without a closing tag", page));
        };
        let close = open_e + close;
        let attrs = parse_attrs(&out[open_s..open_e]);
        let kind = callout_kind(attrs.get("variant").map(String::as_str).unwrap_or("info"));
        let body = &out[open_e..close];
        let ls: Vec<String> = body.split('\n').map(strip_up_to_3_spaces).collect();
        let trimmed = ls.join("\n").trim().to_string();
        // the theme's tip component; `title` must be listed on a block call
        // (empty falls back to the kind's default title).
        let title = attrs.get("title").cloned().unwrap_or_default();
        if title.contains('"') || title.contains('\\') {
            return Err(format!(
                "{}: <Callout> title with a quote/backslash is not expressible: {:?}",
                page, title
            ));
        }
        let block = format!(
            "{{% <tip kind=\"{}\" title=\"{}\" no_title={{false}}> %}}",
            kind, title
        );
        let block = format!("{}\n{}\n{{% </tip> %}}", block, trimmed);
        out = format!(
            "{}{}{}",
            &out[..open_s],
            block,
            &out[close + "</Callout>".len()..]
        );
    }
    Ok(out)
}

/// stripUpTo3Spaces removes 1–3 leading spaces before a non-space char.
fn strip_up_to_3_spaces(l: &str) -> String {
    let mut n = 0usize;
    while n < 3 && n < l.len() && l.as_bytes()[n] == b' ' {
        n += 1;
    }
    if n > 0 && n < l.len() {
        l[n..].to_string()
    } else {
        l.to_string()
    }
}

/// convertDetails rewrites the VitePress `::: details <summary>` container
/// into the theme's details component.
fn convert_details(text: &str, page: &str) -> Result<String, String> {
    let mut out: Vec<String> = Vec::new();
    let mut in_fence = false;
    let mut open = false;
    for l in text.split('\n') {
        let trimmed = l.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            out.push(l.to_string());
            continue;
        }
        if in_fence {
            out.push(l.to_string());
            continue;
        }
        if !open && l.starts_with("::: details ") {
            let sum = l
                .trim_start_matches("::: details ")
                .trim()
                .replace('"', "&quot;");
            out.push(format!(
                "{{% <details summary=\"{}\" open={{false}}> %}}",
                sum
            ));
            open = true;
            continue;
        }
        if open && l == ":::" {
            out.push("{% </details> %}".to_string());
            open = false;
            continue;
        }
        out.push(l.to_string());
    }
    if open {
        return Err(format!("{}: ::: details without a closing :::", page));
    }
    Ok(out.join("\n"))
}

fn convert_steps(text: &str) -> String {
    let out = replace_markup(text, &RE_STEPS_TAG, |_, _| String::new());
    replace_markup(&out, &RE_STEP_BLOCK, |_, m| format!("**{}**", m[0].trim()))
}

fn convert_kbd(text: &str) -> String {
    replace_markup(text, &RE_KBD_TAG, |w, _| {
        if w.starts_with("</") {
            "</kbd>".to_string()
        } else {
            "<kbd>".to_string()
        }
    })
}

fn convert_linked_cards(text: &str) -> String {
    replace_markup(text, &RE_LINKED_CARD, |w, _| {
        if w.starts_with("</") {
            "</div>".to_string()
        } else {
            r#"<div class="linked-card">"#.to_string()
        }
    })
}

fn convert_class_name(text: &str) -> String {
    replace_markup(text, &RE_CLASS_NAME, |_, _| "class=".to_string())
}

fn rewrite_links(text: &str, site_members: &std::collections::HashSet<String>) -> String {
    let text = RE_RADIX_LEGACY_PATH
        .replace_all(text, "https://www.radix-ui.com/primitives/docs/")
        .into_owned();
    replace_markup(&text, &RE_MD_LINK, |whole, m| {
        let Some(route) = resolve_docs_route(&m[1], site_members) else {
            return whole.to_string();
        };
        if route.grey {
            return m[0].clone();
        }
        let slug = route.file.trim_end_matches(".html");
        let target = if site_members.contains(slug) {
            format!("/components/{}", slug)
        } else {
            format!("/guides/{}", slug)
        };
        let target = if route.frag.is_empty() {
            target
        } else {
            format!("{}#{}", target, route.frag)
        };
        format!("[{}]({})", m[0], target)
    })
}

/// assertNoJsx: any JSX reaching here is a shape this mapping has never seen.
fn assert_no_jsx(page: &str, text: &str) -> Result<(), String> {
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut left: Vec<String> = Vec::new();
    for caps in RE_JSX_COMPONENT.captures_iter(text) {
        let name = caps[1].to_string();
        if seen.insert(name.clone()) {
            left.push(name);
        }
    }
    if !left.is_empty() {
        return Err(format!(
            "{}: unmapped JSX components: {}",
            page,
            left.join(", ")
        ));
    }
    Ok(())
}

// ---- section transforms ----------------------------------------------------------

struct DocsBuildCtx {
    catalog: super::docs_transforms::DocsCatalog,
    rtl_langs: HashMap<String, Vec<String>>,
    markup: HashMap<String, String>, // demo file → prettier-formatted markup
    sections: HashMap<String, Vec<String>>,
    real_slots: Option<HashMap<String, bool>>,
    ir_cache: HashMap<String, Option<crate::emit::css::CssIrComponent>>,
}

impl DocsBuildCtx {
    /// componentIR reads+parses generated/ir/<name>.json once per name.
    /// None when the component has no IR file; one that does not decode
    /// panics, as the projection's reads do — it used to read as None too,
    /// and the page shipped without its slot table.
    fn component_ir(&mut self, name: &str) -> Option<&crate::emit::css::CssIrComponent> {
        if !self.ir_cache.contains_key(name) {
            let path = std::path::PathBuf::from(format!("generated/ir/{}.json", name));
            let ir = path.exists().then(|| {
                crate::emit::css::read_ir(&path).unwrap_or_else(|e| panic!("docs-build: {e}"))
            });
            self.ir_cache.insert(name.to_string(), ir);
        }
        self.ir_cache.get(name).and_then(|o| o.as_ref())
    }

    /// shippedSlots: every data-slot that EXISTS — present in shipped markup,
    /// or written by the shipped runtime.
    fn shipped_slots(&mut self, root: &Path) -> HashMap<String, bool> {
        if let Some(s) = &self.real_slots {
            return s.clone();
        }
        let mut out: HashMap<String, bool> = HashMap::new();
        for dir in ["dist/components", "docs/demos"] {
            let Ok(ents) = std::fs::read_dir(root.join(dir)) else {
                continue;
            };
            for e in ents.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if !n.ends_with(".html") {
                    continue;
                }
                let Ok(b) = std::fs::read_to_string(root.join(dir).join(&n)) else {
                    continue;
                };
                for m in RE_DATA_SLOT_ATTR.captures_iter(&b) {
                    out.insert(m[1].to_string(), true);
                }
            }
        }
        let mut js_files: Vec<String> = vec!["dist/shadless.js".to_string()];
        if let Ok(ents) = std::fs::read_dir(root.join("dist/js")) {
            for e in ents.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if n.ends_with(".js") {
                    js_files.push(format!("dist/js/{}", n));
                }
            }
        }
        for f in &js_files {
            let Ok(b) = std::fs::read_to_string(root.join(f)) else {
                continue;
            };
            for m in RE_DATA_SLOT_SET.captures_iter(&b) {
                out.insert(m[1].to_string(), true);
            }
        }
        self.real_slots = Some(out.clone());
        out
    }

    fn install_steps_mdx(&self, root: &Path, name: &str) -> String {
        let demo =
            std::fs::read_to_string(root.join("dist/components").join(format!("{}.html", name)))
                .unwrap_or_default();
        let init_all = RE_INIT_ALL.is_match(&demo);
        let scripts = extract_demo_scripts(&demo);
        let inline_init = !scripts.inline_scripts.is_empty();
        let has_own_css = root.join("dist/css").join(format!("{}.css", name)).exists();
        let mut rows: Vec<String> = vec![
            "| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build (>= 4.3) |".to_string(),
        ];
        if has_own_css {
            rows.push(format!(
                "| `dist/css/{}.css` | this component's slot styles (`@apply` source — your build compiles it) |",
                name
            ));
        }
        let mut load_lines: Vec<String> = Vec::new();
        let mut has_shadless_js = false;
        for s in &scripts.src_scripts {
            if s == "shadless.js" {
                has_shadless_js = true;
                break;
            }
        }
        if has_shadless_js {
            rows.push("| `dist/shadless.js` | behavior base — engine, registry, theme; initialises on DOMContentLoaded (`shadless.init(root)` for content added later) |".to_string());
            load_lines.push("<script src=\"shadless.js\"></script>".to_string());
        } else if scripts.src_scripts.is_empty() {
            rows.push("| — | no JavaScript: this component is markup + CSS |".to_string());
        }
        for s in &scripts.src_scripts {
            if s == "shadless.js" {
                continue;
            }
            let label = if s.starts_with("js/") {
                "this component's behavior — registers with the base".to_string()
            } else {
                format!("vendored runtime ({})", s)
            };
            rows.push(format!("| `dist/{}` | {} |", s, label));
            load_lines.push(format!("<script src=\"{}\"></script>", s));
        }
        let inline_note = if !init_all && !scripts.src_scripts.is_empty() && inline_init {
            " (including the inline init script at the bottom of the demo page)"
        } else {
            ""
        };
        let js_step = if load_lines.is_empty() {
            String::new()
        } else {
            format!(
                "\n\n<Step>Load the behavior files in your page:</Step>\n\n```html\n{}\n```",
                load_lines.join("\n")
            )
        };
        let css_imports = if has_own_css {
            format!("\n@import \"shadless/{}.css\";", name)
        } else {
            String::new()
        };
        let no_css_note = if has_own_css {
            String::new()
        } else {
            "\nThis component has no stylesheet of its own — its styling rides the core theme and utilities in `shadless`.\n".to_string()
        };
        let and_comp = if has_own_css {
            " and this component"
        } else {
            ""
        };
        let step_tail = if inline_note.is_empty() {
            "Copy the markup"
        } else {
            "Copy the markup and init"
        };
        let inline_tail = if inline_init && !init_all {
            inline_note
        } else {
            ""
        };
        format!(
            "<Steps>\n\n<Step>Add shadless{} to your Tailwind v4 (>= 4.3) entry:</Step>\n\n```css\n@import \"shadless\";{}\n```\n{}\nThe files this component needs:\n\n| File | Purpose |\n| --- | --- |\n{}\n{}\n\n<Step>{} from any example on this page (the code tab under its preview) into your page and adapt it{} — the inline utilities are picked up by your build's content scan.</Step>\n{}{}\nNo Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.\n\n</Steps>",
            and_comp,
            css_imports,
            no_css_note,
            rows.join("\n"),
            js_step,
            step_tail,
            inline_tail,
            protocol_mdx(name),
            trivial_mdx(name)
        )
    }

    /// usageMdx: nothing. Upstream's `## Usage` is an import + a JSX
    /// composition; the shadless replacement for it was one sentence that
    /// Installation's own last step already says ten lines earlier.
    fn usage_mdx(&self, _name: &str) -> String {
        String::new()
    }

    fn composition_transform(&mut self, name: &str, raw: &str, seen: &mut Vec<String>) -> String {
        let Some(s) = locate_composition_span(&fence_shadow(raw)) else {
            return raw.to_string();
        };
        seen.push("composition".to_string());
        let section = &raw[s.start..s.end];
        let mut tree = String::new();
        if let Some(m) = Regex::new(r"(?s)```text\n(.*?)```")
            .unwrap()
            .captures(section)
        {
            tree = m[1].trim_end_matches([' ', '\t', '\n']).to_string();
        }
        let mut mapped = String::new();
        if !tree.is_empty() {
            let mut name_to_slot: HashMap<String, String> = HashMap::new();
            if let Some(ir) = self.component_ir(name) {
                for c in &ir.components {
                    for e in &c.elements {
                        if !e.slot.is_empty() {
                            name_to_slot.insert(c.fn_.clone(), e.slot.clone());
                            break;
                        }
                    }
                }
            }
            mapped = Regex::new(r"[A-Z][A-Za-z0-9]+")
                .unwrap()
                .replace_all(&tree, |caps: &regex::Captures| {
                    name_to_slot
                        .get(caps.get(0).unwrap().as_str())
                        .cloned()
                        .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
                })
                .into_owned();
        }
        let body = if mapped.is_empty() {
            "See the demos for real compositions — every slot is a `data-slot` attribute in the shipped markup.\n".to_string()
        } else {
            format!(
                "The slot tree — every node is a `data-slot` attribute in the shipped markup:\n\n```text\n{}\n```\n",
                mapped
            )
        };
        replace_span(raw, s, &format!("## Composition\n\n{}\n", body))
    }

    fn api_reference_transform(
        &mut self,
        root: &Path,
        name: &str,
        raw: &str,
        seen: &mut Vec<String>,
    ) -> String {
        // An upstream page without the section (input, pagination, skeleton,
        // spinner, table, textarea) still gets one when the component ships a
        // stylesheet: the embed is the only place the page shows it. It
        // closes the page, where every upstream-carried section sits;
        // mdx_page_facts mirrors this for docs-fidelity.
        let heading = RE_API_REF_HEADING.find(raw);
        if heading.is_none() && !ships_stylesheet(root, name) {
            return raw.to_string();
        }
        seen.push("api-reference".to_string());
        let mut slots: Vec<String> = Vec::new();
        let mut axes: Vec<super::docs_transforms::CvaAxisRow> = Vec::new();
        let mut tier = String::new();
        let mut slot_seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        let real = self.shipped_slots(root);
        if let Some(ir) = self.component_ir(name) {
            for c in &ir.components {
                for e in &c.elements {
                    if !e.slot.is_empty()
                        && !slot_seen.contains(&e.slot)
                        && real.contains_key(&e.slot)
                    {
                        slot_seen.insert(e.slot.clone());
                        slots.push(e.slot.clone());
                    }
                }
            }
            axes = cva_axis_rows(ir);
            tier = ir.tier.clone();
        }
        let s = locate_api_reference_span(name, &fence_shadow(raw));
        let extra = api_reference_mdx(name, &slots, &axes, &tier, s.is_some(), root);
        if extra.is_empty() {
            return raw.to_string();
        }
        if let Some(s) = s {
            return format!(
                "{}{}{}",
                &raw[..s.start],
                format!("## API Reference\n\n{}\n", extra),
                &raw[s.end..]
            );
        }
        match heading {
            Some(m) => format!("{}\n{}{}", &raw[..m.end()], extra, &raw[m.end()..]),
            None => format!(
                "{}\n\n## API Reference\n\n{}\n",
                raw.trim_end_matches('\n'),
                extra
            ),
        }
    }

    fn component_transform(
        &mut self,
        root: &Path,
        name: &str,
        raw: &str,
    ) -> Result<String, String> {
        let mut seen: Vec<String> = Vec::new();
        let mut raw = apply_jsx_overrides(name, raw)?;
        raw = strip_imports_from_mixed_fences(&drop_react_import_fences(&raw));
        let spans = locate_code_tabs_spans(&fence_shadow(&raw));
        if spans.len() != 1 {
            return Err(format!(
                "install code-tabs: fence-shadowed count {}, expected 1",
                spans.len()
            ));
        }
        seen.push("installation".to_string());
        let mut out = replace_span(&raw, spans[0], &self.install_steps_mdx(root, name));
        if let Some(u) = locate_usage_span(&fence_shadow(&out)) {
            seen.push("usage".to_string());
            out = replace_span(&out, u, &self.usage_mdx(name));
        }
        out = self.composition_transform(name, &out, &mut seen);
        // before api_reference_transform: the Changelog span runs to the end
        // of the file and would swallow a section appended there
        if let Some(s) = locate_changelog_span(&fence_shadow(&out)) {
            out = replace_span(&out, s, "");
        }
        out = self.api_reference_transform(root, name, &out, &mut seen);
        if let Some(s) = locate_message_scroller_js_span(&fence_shadow(&out)) {
            out = replace_span(&out, s, &message_scroller_js_note());
        }
        out = rewrite_leaked_jsx_fences(name, &out, root)?;
        out = rewrite_inline_jsx_mentions(name, &out, root)?;
        seen.sort();
        self.sections.insert(name.to_string(), seen);
        apply_text_adjustments(&format!("{}.mdx", name), &out)
    }
}

// utilsInstallMdx used to say the utilities "ship precompiled inside
// dist/shadless-core.css" — neither is true.
fn utils_install_mdx(util: &str) -> String {
    format!(
        "## Installation\n\nThe `{}` utilities are declared as Tailwind `@utility` rules in\n`dist/shadless-core.css` (npm: bare `shadless`) — the same file every shadless\ncomponent already needs — so on the Tailwind path there is nothing extra to\ninstall or import: write the class and your build emits it.\n\nThey are not in the no-build stylesheet. `dist/shadless.full.min.css` is\ncompiled ahead of time from shadless's own demo markup, and Tailwind emits a\nutility only where it saw the class, so a class you have not used yet is not in\nthere. These utilities need Tailwind running over your own markup (see the\n[Installation](/docs/installation) guide).",
        util
    )
}

fn rtl_migrate_mdx() -> String {
    "shadless components ship the pinned registry's classes as-is, and that cuts both ways: many slots are already logical (start/end-aware) and need nothing but `dir=\"rtl\"` on the page, while others still carry the physical utilities upstream wrote (`pl-*`, `right-*`, `rounded-l-*`) — `css-direction` keeps a committed inventory of exactly which. There is no migration command to run; check the components you actually use, and prefer this page's `-rtl` examples over the LTR ones where upstream authored a pair. To flip an individual icon, give it the `rtl:rotate-180` utility class.".to_string()
}

fn guide_transform(root: &Path, g: &Guide, raw: &str) -> Result<String, String> {
    let mut raw = apply_jsx_overrides(g.slug, raw)?;
    if g.rtl_migrate {
        let Some(s) = locate_rtl_migrate_span(&fence_shadow(&raw)) else {
            return Err(format!("rtl migrate section: not found in {}", g.source));
        };
        raw = replace_span(&raw, s, &rtl_migrate_mdx());
        // The framework/CLI run (Get Started … Supported Styles) goes with it.
        let Some(f) = locate_rtl_framework_span(&fence_shadow(&raw)) else {
            return Err(format!("rtl framework section: not found in {}", g.source));
        };
        raw = replace_span(&raw, f, &rtl_framework_note());
    }
    if g.install_section {
        let Some(s) = locate_install_section(&fence_shadow(&raw)) else {
            return Err(
                "utils Installation section: not found (or no following ## Usage)".to_string(),
            );
        };
        raw = replace_span(&raw, s, &format!("{}\n\n", utils_install_mdx(g.util)));
    }
    if !g.util.is_empty() {
        return rewrite_utility_jsx_fences(g.slug, &raw);
    }
    raw = rewrite_leaked_jsx_fences(g.slug, &raw, root)?;
    raw = rewrite_inline_jsx_mentions(g.slug, &raw, root)?;
    Ok(raw)
}

// ---- page assembly ---------------------------------------------------------------

fn yaml_scalar(s: &str) -> String {
    json_string(s)
}

fn preview_markdown(
    ctx: &DocsBuildCtx,
    attrs: &HashMap<String, String>,
    page: &str,
) -> Result<String, String> {
    let name = attrs.get("name").cloned().unwrap_or_default();
    if name.is_empty() {
        return Err(format!("{}: <ComponentPreview> without a name", page));
    }
    let mut status = "to-author".to_string();
    let mut demo_path = String::new();
    for p in &ctx.catalog.previews {
        if p.name == name {
            status = p.status.clone();
            demo_path = p.demo_path.clone().unwrap_or_default();
            break;
        }
    }
    if status != "existing-dist" && status != "authored" {
        let note = match status.as_str() {
            "unavailable" => "demo not available in shadless (base-style demo)",
            "tombstoned" => "demo not available in shadless (component greyed)",
            _ => "demo not yet available",
        };
        return Ok(format!(
            "<div class=\"demo-missing\" data-demo=\"{}\" data-status=\"{}\">{} — <code>{}</code></div>",
            name, status, note, name
        ));
    }
    let mut file = std::path::Path::new(&format!("docs/demos/{}.html", name))
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    if status == "existing-dist" {
        file = std::path::Path::new(&demo_path)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
    }
    let mut others: Vec<String> = Vec::new();
    if attrs.get("direction").map(String::as_str) == Some("rtl") {
        for l in ctx.rtl_langs.get(&name).cloned().unwrap_or_default() {
            if l == "ar" {
                continue;
            }
            others.push(format!(
                "<a href=\"/demos/{}-{}.html\">{}</a>",
                name,
                l,
                l.to_uppercase()
            ));
        }
    }
    let others_s = if others.is_empty() {
        String::new()
    } else {
        format!(" · {}", others.join(" · "))
    };
    // the site's demo component (docs/site/templates/components.html): the card
    // wrapping the preview iframe and the demo's source. Block calls list
    // every parameter.
    Ok(format!(
        "{{% <demo name=\"{}\" status=\"{}\"> %}}\n<iframe class=\"demo\" src=\"/demos/{}\" title=\"{}\" data-status=\"{}\" loading=\"lazy\"></iframe>\n\n<p class=\"demo-langs\"><a href=\"/demos/{}\">Open the demo page</a>{}</p>\n{}\n{{% </demo> %}}\n",
        name,
        status,
        file,
        name,
        status,
        file,
        others_s,
        demo_source(ctx, &file)
    ))
}

// ---- the slot-only code-tab projection -------------------------------------
//
// The tab under every non-RTL preview used to carry the demo's full
// inline-utility DOM — what React's cn() renders (~16 tokens per slotted
// element), duplicating what shadless/<name>.css already applies through
// [data-slot] selectors. The projection rewrites each slotted element to
// SLOT-ONLY markup: same DOM, same attributes, but the class list keeps
// only the tokens the component stylesheet does NOT already supply (the
// demo author's own layout/size choices, group/peer markers, anything the
// CSS cannot reproduce). docs-code-parity proves the projected tab renders
// exactly like the original markup, in a consumer build as well as under
// the precompiled CSS.

/// One IR element registered under a data-slot name. Slot names are
/// mostly component-namespaced; the measured collisions (7 names, all
/// intra-IR) are resolved per element, in walk order.
struct SlotEntry {
    ir: usize, // index into SlotIndex::irs
    fn_: String,
    el: usize, // index of the element inside IrFn::elements
}

/// What the component CSS supplies to a slot's `[data-slot]` rule. For a
/// non-cva slot carried by MORE than one IR element, css.rs only puts the
/// tokens ALL those elements share under `[data-slot=…]`; each element's
/// rest lands behind an anchor class (`.accordion-trigger-icon-2`) that
/// exists only in the static-tier markup — the demo DOM (and therefore the
/// projected tab) never carries it. Dropping an anchor-keyed token from the
/// tab loses it: the accordion's second trigger icon stopped rendering
/// `hidden` at rest and docs-code-parity went red. A slot ABSENT from the
/// map (every cva slot, whose base+value rules carry the full set) means
/// "everything supplied".
///
/// `own_rule` is, per IR, each slot's exact `[data-slot="x"] { @apply …; }`
/// rule as dist/css ships it: tokens in ONE @apply are emitted in the
/// utilities layer's own sort order (layer closure below), so the closure
/// needs to know which supplied tokens share that rule.
struct SlotIndex {
    irs: Vec<crate::emit::css::CssIrComponent>,
    cva_slots: Vec<HashMap<String, crate::emit::css::CvaTable>>,
    slots: HashMap<String, Vec<SlotEntry>>,
    supply: Vec<HashMap<String, std::collections::HashSet<String>>>,
    own_rule: Vec<HashMap<String, std::collections::HashSet<String>>>,
}

/// A slot's own rule in dist/css/<name>.css: the bare `[data-slot="x"]`
/// selector, one @apply on one line (the css.rs emit shape).
static RE_OWN_SLOT_RULE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)^\s*\[data-slot="([^"]+)"\] \{ @apply ([^;]*); \}\s*$"#).unwrap()
});

impl SlotIndex {
    /// Every generated/ir component that SHIPS CSS (the same filter
    /// path-parity applies: a tab is only rewritten when the component's
    /// stylesheet exists for the consumer to import). IRs that fail to
    /// parse fail the build — the projection refusing one silently would
    /// leave component tokens inline across a whole component's tabs.
    fn load(root: &Path) -> SlotIndex {
        let mut names: Vec<String> = Vec::new();
        if let Ok(ents) = std::fs::read_dir(root.join("generated/ir")) {
            for e in ents.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if n.ends_with(".json") {
                    names.push(n);
                }
            }
        }
        names.sort();
        let mut out = SlotIndex {
            irs: Vec::new(),
            cva_slots: Vec::new(),
            slots: HashMap::new(),
            supply: Vec::new(),
            own_rule: Vec::new(),
        };
        for f in &names {
            let name = f.trim_end_matches(".json");
            let css_path = root.join("dist/css").join(format!("{}.css", name));
            if !css_path.exists() {
                continue;
            }
            let css = std::fs::read_to_string(&css_path)
                .unwrap_or_else(|e| panic!("projection: reading dist/css/{name}.css: {e}"));
            let mut own: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
            for caps in RE_OWN_SLOT_RULE.captures_iter(&css) {
                own.insert(
                    caps[1].to_string(),
                    caps[2].split_whitespace().map(|t| t.to_string()).collect(),
                );
            }
            let b = std::fs::read_to_string(root.join("generated/ir").join(f))
                .unwrap_or_else(|e| panic!("projection: reading generated/ir/{f}: {e}"));
            let mut v: serde_json::Value = serde_json::from_str(&b)
                .unwrap_or_else(|e| panic!("projection: parsing generated/ir/{f}: {e}"));
            crate::emit::css::drop_nulls(&mut v);
            let ir: crate::emit::css::CssIrComponent = serde_json::from_value(v)
                .unwrap_or_else(|e| panic!("projection: decoding generated/ir/{f}: {e}"));
            let cva = crate::emit::css::cva_slot(&ir);
            let mut cva_slots: HashMap<String, crate::emit::css::CvaTable> = HashMap::new();
            for (table, slot) in cva.values() {
                if !slot.is_empty() {
                    cva_slots.insert(slot.clone(), table.clone());
                }
            }
            let ir_idx = out.irs.len();
            for c in &ir.components {
                for (ei, el) in c.elements.iter().enumerate() {
                    if el.slot.is_empty() {
                        continue;
                    }
                    out.slots
                        .entry(el.slot.clone())
                        .or_default()
                        .push(SlotEntry {
                            ir: ir_idx,
                            fn_: c.fn_.clone(),
                            el: ei,
                        });
                }
            }
            // The [data-slot] supply per slot, mirroring css.rs pass 1:
            // cva slots are fully covered by their base+value rules; other
            // slots supply the INTERSECTION of their elements' applied
            // tokens (empty-class elements never enter a rule at all).
            let mut acc: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
            for c in &ir.components {
                for el in &c.elements {
                    if el.classes.is_empty() || el.slot.is_empty() {
                        continue;
                    }
                    if cva_slots.contains_key(&el.slot) {
                        continue; // cva slot: everything supplied (absent from the supply map)
                    }
                    let applied: std::collections::HashSet<String> =
                        crate::emit::css::split_markers(&el.classes.join(" "))
                            .apply
                            .split_whitespace()
                            .map(|t| t.to_string())
                            .collect();
                    acc.entry(el.slot.clone())
                        .and_modify(|prev| prev.retain(|t| applied.contains(t)))
                        .or_insert(applied);
                }
            }
            out.supply.push(acc);
            out.own_rule.push(own);
            out.irs.push(ir);
            out.cva_slots.push(cva_slots);
        }
        out
    }
}

#[derive(Default)]
struct ProjectMetrics {
    files_projected: usize,
    files_rtl_exempt: usize,
    slotted: usize,        // elements on an indexed slot
    unindexed_slot: usize, // data-slot with no index entry — kept verbatim
    tokens_before: usize,  // on indexed slotted elements
    tokens_after: usize,
    zero_class_after: usize, // slotted elements left with no class at all
    fallback_inconsistent: usize, // no consistent candidate — all of D kept
    collision_ties: usize,   // multi-candidate slots resolved by walk-order tie
    closure_kept_tokens: usize, // tokens moved back into K by the layer closure
}

/// The RTL demo family, derived from what the pipeline actually defines:
/// build-rtl's own manifest (build/rtl-langs.json: `<name>-rtl` → the
/// languages it emitted; the ar base file is `<name>-rtl.html`, the others
/// `<name>-rtl-<lang>.html`) plus every `-rtl` demoPath the catalog hands a
/// preview (the oracle-rendered `-rtl` pages, e.g. menubar-rtl, that
/// predate build-rtl's family). A demo file whose stem contains `-rtl` but
/// is NOT in the derived set fails the build — an unexpected family shape
/// must stop the pipeline, not silently pick a treatment.
fn rtl_exempt_stems(root: &Path, demo_stems: &[String]) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    let mb = std::fs::read_to_string(root.join("build/rtl-langs.json"))
        .unwrap_or_else(|e| panic!("projection: reading build/rtl-langs.json: {e}"));
    let manifest: HashMap<String, Vec<String>> = serde_json::from_str(&mb)
        .unwrap_or_else(|e| panic!("projection: parsing build/rtl-langs.json: {e}"));
    for (key, langs) in &manifest {
        out.insert(key.clone());
        for l in langs {
            out.insert(format!("{key}-{l}"));
        }
    }
    let cb = std::fs::read_to_string(root.join("docs/catalog.json"))
        .unwrap_or_else(|e| panic!("projection: reading docs/catalog.json: {e}"));
    #[derive(serde::Deserialize)]
    struct Catalog {
        #[serde(default)]
        previews: Vec<CatalogPreview>,
    }
    #[derive(serde::Deserialize)]
    struct CatalogPreview {
        #[serde(default, rename = "demoPath")]
        demo_path: Option<String>,
    }
    let catalog: Catalog = serde_json::from_str(&cb)
        .unwrap_or_else(|e| panic!("projection: parsing docs/catalog.json: {e}"));
    for p in &catalog.previews {
        if let Some(path) = &p.demo_path
            && path.contains("-rtl")
        {
            let stem = std::path::Path::new(path)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();
            out.insert(stem);
        }
    }
    for stem in demo_stems {
        if stem.contains("-rtl") && !out.contains(stem) {
            panic!(
                "projection: demo file {stem}.html matches -rtl but neither build/rtl-langs.json \
                 nor docs/catalog.json defines it — the RTL exemption set is out of date"
            );
        }
    }
    out
}

/// An element's attribute lookup over the parsed DOM.
fn attr_of(e: &markup5ever_rcdom::Handle, name: &str) -> Option<String> {
    if let markup5ever_rcdom::NodeData::Element { attrs, .. } = &e.data {
        for a in attrs.borrow().iter() {
            if &*a.name.local == name {
                return Some(a.value.to_string());
            }
        }
    }
    None
}

/// The context stack: every ancestor's `data-<axis>=<value>` attributes,
/// innermost last. Only cvaRef (context) axes may look up it — a
/// generalized ancestor fallback mis-styleS a button-group inside a
/// vertical Field (measured in the scout prototype).
type CtxStack = Vec<(String, String)>;

fn push_ctx(e: &markup5ever_rcdom::Handle, ctx: &mut CtxStack) -> usize {
    let mut pushed = 0usize;
    if let markup5ever_rcdom::NodeData::Element { attrs, .. } = &e.data {
        for a in attrs.borrow().iter() {
            let name = a.name.local.to_string();
            if let Some(axis) = name.strip_prefix("data-") {
                if axis == "slot" {
                    continue;
                }
                ctx.push((axis.to_string(), a.value.to_string()));
                pushed += 1;
            }
        }
    }
    pushed
}

/// sel for one candidate: the element's own `data-<axis>` attributes; an
/// own-table axis without its attribute stays UNSET (classes_of then uses
/// the table default, mirroring the CSS `:not([data-axis])` arm); only
/// cvaRef axes may fall back to the nearest ancestor carrying the axis.
fn build_sel(
    ir: &crate::emit::css::CssIrComponent,
    cva_slots: &HashMap<String, crate::emit::css::CvaTable>,
    slot: &str,
    e: &markup5ever_rcdom::Handle,
    ctx: &CtxStack,
) -> HashMap<String, String> {
    let mut sel: HashMap<String, String> = HashMap::new();
    let mut own_axes: Vec<String> = Vec::new();
    if let Some(table) = cva_slots.get(slot) {
        own_axes.extend(table.axis_order());
    }
    let mut ctx_axes: Vec<String> = Vec::new();
    for r in &ir.cva_refs {
        if r.slot == slot {
            ctx_axes.extend(r.dyn_axes.iter().cloned());
        }
    }
    for ax in &own_axes {
        if let Some(v) = attr_of(e, &format!("data-{ax}")) {
            sel.insert(ax.clone(), v);
        }
    }
    for ax in &ctx_axes {
        if sel.contains_key(ax) {
            continue;
        }
        if let Some(v) = attr_of(e, &format!("data-{ax}")) {
            sel.insert(ax.clone(), v);
            continue;
        }
        if let Some((_, v)) = ctx.iter().rev().find(|(a, _)| a == ax) {
            sel.insert(ax.clone(), v.clone());
        }
    }
    sel
}

/// The candidate's M: what React rendered for this slot under sel.
fn candidate_classes(
    index: &SlotIndex,
    entry: &SlotEntry,
    slot: &str,
    e: &markup5ever_rcdom::Handle,
    ctx: &CtxStack,
) -> String {
    let ir = &index.irs[entry.ir];
    let sel = build_sel(ir, &index.cva_slots[entry.ir], slot, e, ctx);
    let c = ir
        .components
        .iter()
        .find(|c| c.fn_ == entry.fn_)
        .unwrap_or_else(|| panic!("projection: ir fn {} vanished", entry.fn_));
    let el = &c.elements[entry.el];
    crate::emit::compose::classes_of(ir, &index.cva_slots[entry.ir], c, el, &sel)
}

/// x is consistent with D iff every applied token x is either in D or
/// overridden by D (twMerge(x + " " + D) no longer contains x — React's
/// cn(base, className) order).
fn consistent_with(d_tokens: &[String], x: &str) -> bool {
    if d_tokens.iter().any(|t| t == x) {
        return true;
    }
    let merged = crate::twmerge::merge(&format!("{} {}", x, d_tokens.join(" ")));
    !merged.split_whitespace().any(|t| t == x)
}

/// Strip variant prefixes and the important modifier: split on top-level
/// `:` outside `[]`/`()`, take the last segment, drop a leading/trailing
/// `!`. The layer closure compares bases because ANY utilities-layer token
/// beats a components-layer token on the same element and property,
/// regardless of variants.
fn base_token(t: &str) -> &str {
    let bytes = t.as_bytes();
    let mut depth = 0i32;
    let mut last_colon = None;
    for (i, &c) in bytes.iter().enumerate() {
        match c {
            b'[' | b'(' => depth += 1,
            b']' | b')' => depth -= 1,
            b':' if depth == 0 => last_colon = Some(i),
            _ => {}
        }
    }
    let seg = match last_colon {
        Some(i) => &t[i + 1..],
        None => t,
    };
    let seg = seg.strip_prefix('!').unwrap_or(seg);
    seg.strip_suffix('!').unwrap_or(seg)
}

/// Does this token's selector TARGET DESCENDANTS? `[&_svg]`, `[&>input]`,
/// `*:data-[slot=…]`, `**:` do — the only kept tokens whose utilities-layer
/// win can flip a descendant's dropped token, or whose own loss to a
/// descendant's kept utility the closure must restore. A plain utility on
/// an ancestor (`w-full`, `flex`) styles ITS OWN element only: an ancestor
/// carrying w-full never competes with a descendant's w-fit, so it is not
/// a conflict.
fn targets_descendants(t: &str) -> bool {
    t.contains("[&") || t.starts_with('*')
}

/// Would the projected element render `x` and `k` differently from the
/// original, where both were inline utilities? Two tokens CONFLICT iff a
/// twMerge in either direction drops one of them — same property,
/// different value, so which one wins is order-dependent. Equal bases
/// never conflict: same utility, same declarations, order irrelevant.
/// Variants are stripped first (base_token) because ANY utilities-layer
/// token beats a components-layer one regardless of variants once the
/// layers differ.
fn conflicts(x: &str, k: &str) -> bool {
    let (xb, kb) = (base_token(x), base_token(k));
    let m1 = crate::twmerge::merge(&format!("{xb} {kb}"));
    let m2 = crate::twmerge::merge(&format!("{kb} {xb}"));
    let has = |m: &str, t: &str| m.split_whitespace().any(|s| s == t);
    !has(&m1, xb) || !has(&m1, kb) || !has(&m2, xb) || !has(&m2, kb)
}

/// Does the token carry a variant (a top-level `:` outside []/())?
fn has_variant(t: &str) -> bool {
    let bare = t.strip_prefix('!').unwrap_or(t);
    let bare = bare.strip_suffix('!').unwrap_or(bare);
    base_token(bare) != bare
}

/// A kept utility k cannot flip a dropped x when x is unvarianted and k is
/// varianted: the utilities layer sorts every unvarianted utility before
/// every varianted one (measured on tailwind 4.3.3), so inline k already
/// won exactly when its variant matched — which it still does from the
/// utilities layer over x in @layer components. `!` reverses layer order
/// for important declarations, so any `!` keeps the pair conservative.
fn variant_cannot_flip(x: &str, k: &str) -> bool {
    !has_variant(x) && has_variant(k) && !x.contains('!') && !k.contains('!')
}

/// Does the token style elements OTHER than the one carrying it? A
/// variant in its chain selects children, descendants or siblings: `*:`,
/// `**:`, or an arbitrary `[&…]` variant whose `&` is followed by a
/// combinator (`_` = descendant space, `>`, `+`, `~`). `[&:hover]`,
/// `has-[>svg]:`, `group-*:` and `[.border-b]:` all style the element
/// itself.
fn targets_other_elements(t: &str) -> bool {
    let t = t.strip_prefix('!').unwrap_or(t);
    let mut depth = 0i32;
    let mut start = 0usize;
    let mut variants: Vec<&str> = Vec::new();
    for (i, c) in t.char_indices() {
        match c {
            '[' | '(' => depth += 1,
            ']' | ')' => depth -= 1,
            ':' if depth == 0 => {
                variants.push(&t[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    variants.iter().any(|v| {
        *v == "*"
            || *v == "**"
            || (v.starts_with('[')
                && v.match_indices('&').any(|(i, _)| {
                    matches!(v.as_bytes().get(i + 1), Some(b'_' | b'>' | b'+' | b'~'))
                }))
    })
}

/// Can x and y land on the same element? Not when exactly one of them
/// targets other elements: `[&_svg]:size-4` sizes the button's svg, h-8
/// the button itself. Two other-targeting tokens may meet on one
/// descendant, so they count.
fn same_target(x: &str, y: &str) -> bool {
    targets_other_elements(x) == targets_other_elements(y)
}

struct Projected {
    indexed: bool, // counted in ProjectMetrics::tokens_before
    kept: Vec<String>,
    dropped: Vec<String>,
    supplied: Vec<String>,
    rules: Vec<std::collections::HashSet<String>>,
    markers: Vec<String>,
}

impl Projected {
    /// D kept whole: an unindexed slot, or no consistent candidate.
    fn verbatim(d_tokens: &[String], indexed: bool) -> Projected {
        Projected {
            indexed,
            kept: d_tokens.to_vec(),
            dropped: Vec::new(),
            supplied: Vec::new(),
            rules: Vec::new(),
            markers: Vec::new(),
        }
    }
}

fn project_element(
    e: &markup5ever_rcdom::Handle,
    slot: &str,
    d_tokens: &[String],
    index: &SlotIndex,
    ctx: &CtxStack,
    m: &mut ProjectMetrics,
) -> Projected {
    let candidates = match index.slots.get(slot) {
        Some(c) => c,
        None => {
            m.unindexed_slot += 1;
            return Projected::verbatim(d_tokens, false);
        }
    };
    m.slotted += 1;
    m.tokens_before += d_tokens.len();

    let dset: std::collections::HashSet<&String> = d_tokens.iter().collect();
    let d_markers = crate::emit::css::split_markers(&d_tokens.join(" ")).markers;

    // CONSISTENCY: a candidate whose full applied set M the demo DOM
    // plausibly rendered. No consistent candidate means our M is NOT what
    // React rendered (a non-default conditional branch, an unexpected sel)
    // — dropping anything would be a guess, so all of D stays.
    let mut best: Option<(&SlotEntry, Vec<String>, usize)> = None; // (entry, A, overlap)
    let mut best_tie = false;
    for entry in candidates {
        let m_cls = candidate_classes(index, entry, slot, e, ctx);
        let a_toks = crate::emit::css::split_markers(&m_cls).apply;
        let a: Vec<String> = a_toks.split_whitespace().map(|s| s.to_string()).collect();
        if !a.iter().all(|x| consistent_with(d_tokens, x)) {
            continue;
        }
        let overlap = a.iter().filter(|x| dset.contains(*x)).count();
        match &best {
            None => best = Some((entry, a, overlap)),
            Some((_, _, bo)) => {
                if overlap > *bo {
                    best = Some((entry, a, overlap));
                    best_tie = false;
                } else if overlap == *bo {
                    best_tie = true;
                }
            }
        }
    }
    let Some((entry, a_toks, _)) = best else {
        m.fallback_inconsistent += 1;
        return Projected::verbatim(d_tokens, true);
    };
    if best_tie {
        m.collision_ties += 1;
    }
    // What the CSS actually supplies to the projected markup: the slot-rule
    // intersection, not the full composed set (SlotIndex::supply).
    let supplied = index.supply[entry.ir].get(slot);
    let a: std::collections::HashSet<&String> = a_toks
        .iter()
        .filter(|t| supplied.is_none_or(|s| s.contains(*t)))
        .collect();

    // K = markers of D ∪ (D ∖ A), in D's order, first occurrence wins.
    let mut kept: Vec<String> = Vec::new();
    let mut seen: std::collections::HashSet<&String> = std::collections::HashSet::new();
    let mut dropped: Vec<String> = Vec::new();
    for t in d_tokens {
        if !seen.insert(t) {
            continue;
        }
        if d_markers.iter().any(|x| x == t) || !a.contains(t) {
            kept.push(t.clone());
        } else {
            dropped.push(t.clone());
        }
    }

    // The same-element layer closure runs in project_body (close_element),
    // together with the cross-element one, to one shared fixpoint.
    let mut supplied: Vec<String> = a.iter().map(|t| t.to_string()).collect();
    supplied.sort();
    // The single-@apply rules that style this element: the slot's own
    // [data-slot] rule and, for a cva slot, the one value rule per axis
    // its data-* selects (classes_of's key choice; every registered table's
    // value class sits whole inside an emitted value rule, measured over
    // dist/css).
    let mut rules: Vec<std::collections::HashSet<String>> = Vec::new();
    if let Some(r) = index.own_rule[entry.ir].get(slot) {
        rules.push(r.clone());
    }
    if let Some(table) = index.cva_slots[entry.ir].get(slot) {
        let sel = build_sel(
            &index.irs[entry.ir],
            &index.cva_slots[entry.ir],
            slot,
            e,
            ctx,
        );
        for axis in table.axis_order() {
            let mut key = sel.get(&axis).cloned().unwrap_or_default();
            if key.is_empty() {
                key = table.defaults.get(&axis).cloned().unwrap_or_default();
            }
            if let Some(v) = table.class_of(&axis, &key)
                && !v.is_empty()
            {
                rules.push(
                    crate::emit::css::split_markers(&v)
                        .apply
                        .split_whitespace()
                        .map(|t| t.to_string())
                        .collect(),
                );
            }
        }
    }
    Projected {
        indexed: true,
        kept,
        dropped,
        supplied,
        rules,
        markers: d_markers,
    }
}

/// The same-element half of the LAYER CLOSURE, to a fixpoint on one
/// element; returns whether a token moved. Every dropped token x arrives
/// from the component CSS in @layer components after the projection, while
/// in the original markup it sat inline in the utilities layer. x moves
/// back into K (over-keeping is always safe) when that layer change could
/// flip a winner:
///
/// Only pairs that can style the same element count (same_target).
///
/// - a KEPT non-marker token k conflicts with x: k now always beats x —
///   except when variant_cannot_flip(x, k);
/// - ANOTHER SUPPLIED token y conflicts with x: inline, x beat every
///   component token; now x and y compete inside @layer components
///   (measured: item-image's item-title carries line-clamp-1 but not flex
///   — twMerge removed it — yet the [data-slot] rule applies both, and
///   flex wins there). The one pair that cannot flip: x and y BOTH
///   rendered inline in the original (y in D) and BOTH in one of the
///   element's single-@apply rules (the slot's own [data-slot] rule or the
///   cva value rule its data-* selects). @apply emits its candidates in the utilities
///   layer's own sort order whatever the written order (measured on
///   tailwind 4.3.3: `@apply flex line-clamp-1` and `@apply line-clamp-1
///   flex` compile identically, in the order the utilities layer emits
///   .line-clamp-1 and .flex), so the pair resolves as it did inline.
///   Pairs across rules (base vs a cva value rule) are kept: rule order,
///   not utility order, decides there.
fn close_element(st: &mut ElemState, m: &mut ProjectMetrics) -> bool {
    let mut moved = false;
    loop {
        let mut changed = false;
        for x in st.dropped.clone() {
            let kept_conflict = st
                .kept
                .iter()
                .filter(|k| !st.markers.contains(*k))
                .any(|k| same_target(&x, k) && conflicts(&x, k) && !variant_cannot_flip(&x, k));
            let same_rule_inline = |y: &String| {
                st.d.contains(y)
                    && st.rules.iter().any(|r| r.contains(&x) && r.contains(y))
                    && !x.contains('!')
                    && !y.contains('!')
            };
            let css_conflict = st
                .supplied
                .iter()
                .filter(|y| **y != x)
                .any(|y| same_target(&x, y) && conflicts(&x, y) && !same_rule_inline(y));
            if kept_conflict || css_conflict {
                st.kept.push(x.clone());
                st.dropped.retain(|t| t != &x);
                m.closure_kept_tokens += 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
        moved = true;
    }
    moved
}

/// Parse a demo body fragment: a full-document parse, then the <body>
/// element. The RcDom must outlive the Handle, so both come back together.
fn parse_body_fragment(
    body_text: &str,
) -> Result<(markup5ever_rcdom::RcDom, markup5ever_rcdom::Handle), String> {
    use html5ever::tendril::TendrilSink;
    use markup5ever_rcdom::NodeData;
    let full = format!("<!doctype html><html><body>{body_text}</body></html>");
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
        .find(|c| matches!(&c.data, NodeData::Element { name, .. } if name.local == html5ever::local_name!("html")))
        .ok_or("projection: parsed document has no <html>")?
        .clone();
    let body = html_el
        .children
        .borrow()
        .iter()
        .find(|c| matches!(&c.data, NodeData::Element { name, .. } if name.local == html5ever::local_name!("body")))
        .ok_or("projection: parsed document has no <body>")?
        .clone();
    Ok((dom, body))
}

/// Walk the parsed body: project every element's class attribute, then
/// serialize the (mutated) children through the code-tab writer.
fn project_body(
    body_text: &str,
    index: &SlotIndex,
    m: &mut ProjectMetrics,
) -> Result<String, String> {
    use markup5ever_rcdom::NodeData;
    let (_dom, body) = parse_body_fragment(body_text)?;

    // pass 1: compute every element's kept/dropped sets, in document order
    let mut states: Vec<ElemState> = Vec::new();
    {
        fn walk1(
            n: &markup5ever_rcdom::Handle,
            index: &SlotIndex,
            ctx: &mut CtxStack,
            m: &mut ProjectMetrics,
            states: &mut Vec<ElemState>,
        ) {
            let mut pushed = 0usize;
            if let NodeData::Element { .. } = &n.data {
                pushed = push_ctx(n, ctx);
                let slot = attr_of(n, "data-slot").unwrap_or_default();
                let class_value = attr_of(n, "class");
                if !slot.is_empty() {
                    let d: Vec<String> = class_value
                        .as_deref()
                        .unwrap_or("")
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    // indexed slots are rewritten; unindexed slots and
                    // non-slotted elements keep their classes verbatim
                    let p = project_element(n, &slot, &d, index, ctx, m);
                    states.push(ElemState {
                        el: n.clone(),
                        d,
                        indexed: p.indexed,
                        kept: p.kept,
                        dropped: p.dropped,
                        supplied: p.supplied,
                        rules: p.rules,
                        markers: p.markers,
                    });
                }
            }
            for c in n.children.borrow().iter() {
                walk1(c, index, ctx, m, states);
            }
            if let NodeData::Element {
                template_contents, ..
            } = &n.data
                && let Some(tc) = template_contents.borrow().as_ref()
            {
                for c in tc.children.borrow().iter() {
                    walk1(c, index, ctx, m, states);
                }
            }
            for _ in 0..pushed {
                ctx.pop();
            }
        }
        let mut ctx: CtxStack = Vec::new();
        walk1(&body, index, &mut ctx, m, &mut states);
    }

    // pass 2: the layer closure, same-element (close_element) and across
    // elements, to one shared fixpoint — a token the cross-element sweep
    // moves into K can flip a same-element pair, and back. A kept token on
    // one element can defeat a dropped one on another (a button's kept
    // `[&_svg:not([class*='size-'])]:size-3.5` matching a spinner's dropped
    // `size-4`), and a dropped token on an ancestor can be beaten by a
    // descendant's kept one once the layers differ (the input-group's
    // supplied `has-[>…]:[&>input]:pl-1.5` against the control's kept
    // `px-2.5`). In the original markup BOTH sides sat inline in the
    // utilities layer; after the projection only the kept side does, and
    // the layer order flips the winner. When BOTH sides drop, each comes
    // from its own component rule and those rules' specificity and order
    // decide instead of the utility sort (measured: field's `*:w-full` lost
    // to field-label's w-fit, whose `:is(label)` rule is more specific
    // though it comes first; a button's dropped `[&_svg:not(
    // [class*='size-'])]:size-3.5` took the spinner once its size-4 left the
    // class attribute the selector reads). So every conflicting pair of an
    // ancestor's descendant-targeting token and a descendant's token ends
    // with BOTH sides in K — over-keeping is always safe — until nothing
    // moves.
    fn settle(
        states: &mut [ElemState],
        ancestors: &[usize],
        b: usize,
        m: &mut ProjectMetrics,
    ) -> bool {
        fn keep(st: &mut ElemState, t: &str, m: &mut ProjectMetrics) -> bool {
            if !st.dropped.iter().any(|d| d == t) {
                return false;
            }
            st.dropped.retain(|d| d != t);
            st.kept.push(t.to_string());
            m.closure_kept_tokens += 1;
            true
        }
        let mut moved = false;
        for &a in ancestors {
            let from_a: Vec<String> = states[a]
                .kept
                .iter()
                .chain(states[a].dropped.iter())
                .filter(|x| targets_descendants(x))
                .cloned()
                .collect();
            for x in from_a {
                let hits: Vec<String> = states[b]
                    .kept
                    .iter()
                    .chain(states[b].dropped.iter())
                    .filter(|y| conflicts(&x, y))
                    .cloned()
                    .collect();
                if hits.is_empty() {
                    continue;
                }
                moved |= keep(&mut states[a], &x, m);
                for y in &hits {
                    moved |= keep(&mut states[b], y, m);
                }
            }
        }
        moved
    }
    fn walk2(
        n: &markup5ever_rcdom::Handle,
        ancestors: &mut Vec<usize>,
        states: &mut Vec<ElemState>,
        m: &mut ProjectMetrics,
    ) -> bool {
        let mut moved = false;
        let mut here: Option<usize> = None;
        if let NodeData::Element { .. } = &n.data
            && let Some(i) = states.iter().position(|st| std::rc::Rc::ptr_eq(&st.el, n))
        {
            here = Some(i);
            moved |= settle(states, ancestors, i, m);
        }
        if let Some(i) = here {
            ancestors.push(i);
        }
        for c in n.children.borrow().iter() {
            moved |= walk2(c, ancestors, states, m);
        }
        if let NodeData::Element {
            template_contents, ..
        } = &n.data
            && let Some(tc) = template_contents.borrow().as_ref()
        {
            for c in tc.children.borrow().iter() {
                moved |= walk2(c, ancestors, states, m);
            }
        }
        if here.is_some() {
            ancestors.pop();
        }
        moved
    }
    {
        // moves are monotone (tokens only travel dropped -> kept), so the
        // sweep terminates; the bound just caps the walk
        let mut ancestors: Vec<usize> = Vec::new();
        for _ in 0..64 {
            let mut moved = false;
            for st in states.iter_mut() {
                moved |= close_element(st, m);
            }
            moved |= walk2(&body, &mut ancestors, &mut states, m);
            if !moved {
                break;
            }
        }
    }

    // pass 3: write the settled kept sets back and serialize
    for st in &states {
        if let NodeData::Element { attrs, .. } = &st.el.data {
            let mut ab = attrs.borrow_mut();
            if let Some(ci) = ab.iter().position(|a| &*a.name.local == "class") {
                if st.kept.is_empty() {
                    ab.remove(ci);
                } else {
                    ab[ci].value = st.ordered_kept().join(" ").into();
                }
            }
        }
    }
    for st in states.iter().filter(|st| st.indexed) {
        m.tokens_after += st.kept.len();
        if st.kept.is_empty() {
            m.zero_class_after += 1;
        }
    }
    Ok(crate::emit::writer::serialize_children(&body))
}

struct ElemState {
    el: markup5ever_rcdom::Handle,
    d: Vec<String>,
    indexed: bool,
    kept: Vec<String>,
    dropped: Vec<String>,
    supplied: Vec<String>,
    rules: Vec<std::collections::HashSet<String>>,
    markers: Vec<String>,
}

impl ElemState {
    /// The final kept set in D's order. The closure appends to `kept`; the
    /// class attribute keeps every token where the demo author wrote it.
    /// Every kept token is in D by construction (markers and D∖A come from
    /// D, the closure moves dropped tokens of D back), so a leftover would
    /// mean a token silently vanishing from the tab — that is an invariant
    /// break and fails loudly.
    fn ordered_kept(&self) -> Vec<String> {
        if self.kept == self.d {
            return self.d.clone(); // fallback paths: D verbatim, duplicates included
        }
        let kset: std::collections::HashSet<&String> = self.kept.iter().collect();
        let mut seen = std::collections::HashSet::new();
        let mut out: Vec<String> = Vec::new();
        for t in &self.d {
            if kset.contains(t) && seen.insert(t) {
                out.push(t.clone());
            }
        }
        let lost: Vec<&String> = self.kept.iter().filter(|k| !self.d.contains(*k)).collect();
        assert!(
            lost.is_empty() && out.len() == self.kept.len(),
            "projection: kept token(s) {lost:?} outside D — closure invariant broken"
        );
        out
    }
}

// ---- whitespace-sensitive subtrees survive prettier verbatim ---------------
//
// The tab is prettier-formatted text, but prettier's html printer cannot
// see what a class list does to white-space: it reflows text and inserts
// element-separating whitespace, and inside any white-space:pre* subtree
// that output renders EXTRA line boxes and grows (bubble-collapsible's
// pre-line bubble rendered 300.25px tall against the demo's 209.25px — the one
// docs-code-parity family no prettier option removes). Prettier prints
// <pre> (and textarea) content VERBATIM, so the formatting runs in three
// steps: every at-rest pre-like element is renamed to <pre> with its real
// tag in a marker attribute (stamp_pre_like), prettier formats the
// renamed fragment, and the tags are restored on prettier's output
// (strip_pre_like). The shipped tab carries no trace; the subtree's bytes
// survive formatting exactly.

/// Marker attribute carrying a stamped element's real tag name. Never
/// ships: strip_pre_like removes it from prettier's output.
const PRE_WS_MARK: &str = "data-shadless-tab-pre";

/// Does this class list put the element in a pre-like white-space AT REST?
/// Only unvarianted tokens count (`hover:whitespace-pre-line` never applies
/// at rest), and only the ones mapping to a `pre*` value:
/// `whitespace-pre[-line|-wrap]` and the `[white-space:pre…]` arbitrary
/// form. `whitespace-nowrap` collapses a formatter's newlines to single
/// spaces exactly like `normal` does, so it is not pre-like.
fn is_at_rest_pre_like(class_value: &str) -> bool {
    for t in class_value.split_whitespace() {
        let t = t.strip_prefix('!').unwrap_or(t);
        let t = t.strip_suffix('!').unwrap_or(t);
        // variant-scoped: a top-level `:` outside []/() (colons INSIDE an
        // arbitrary value are part of the value)
        if base_token(t) != t {
            continue;
        }
        if matches!(
            t,
            "whitespace-pre" | "whitespace-pre-line" | "whitespace-pre-wrap"
        ) {
            return true;
        }
        if let Some(v) = t
            .strip_prefix("[white-space:")
            .and_then(|s| s.strip_suffix(']'))
        {
            let v = v.replace('_', " ");
            if v == "pre" || v.starts_with("pre-line") || v.starts_with("pre-wrap") {
                return true;
            }
        }
    }
    false
}

/// rcdom element names are plain fields behind a shared Rc — a tag rename
/// is a rebuild-and-swap, not a mutation. Builds a copy of `src` with
/// `new_local` as its tag and `attrs` as its attributes, moves the
/// children over (parent pointers re-pointed; the old child list cleared),
/// and swaps the copy into `src`'s parent. `src` stays alive for the
/// caller (both stamp and strip still read it).
fn element_retagged(
    src: &markup5ever_rcdom::Handle,
    new_local: html5ever::LocalName,
    attrs: Vec<html5ever::Attribute>,
) -> markup5ever_rcdom::Handle {
    use markup5ever_rcdom::{Node, NodeData};
    let NodeData::Element {
        name,
        template_contents,
        mathml_annotation_xml_integration_point,
        ..
    } = &src.data
    else {
        panic!("retag: not an element");
    };
    let nn = Node::new(NodeData::Element {
        name: html5ever::QualName::new(None, name.ns.clone(), new_local),
        attrs: std::cell::RefCell::new(attrs),
        template_contents: template_contents.clone(),
        mathml_annotation_xml_integration_point: *mathml_annotation_xml_integration_point,
    });
    let moved: Vec<markup5ever_rcdom::Handle> = src.children.borrow().clone();
    src.children.borrow_mut().clear();
    for c in moved {
        c.parent.set(Some(std::rc::Rc::downgrade(&nn)));
        nn.children.borrow_mut().push(c);
    }
    let parent = src
        .parent
        .take()
        .and_then(|w| w.upgrade())
        .expect("retag: element without a parent");
    let mut ch = parent.children.borrow_mut();
    let pos = ch
        .iter()
        .position(|c| std::rc::Rc::ptr_eq(c, src))
        .expect("retag: element not in its parent's child list");
    nn.parent.set(Some(std::rc::Rc::downgrade(&parent)));
    ch[pos] = nn.clone();
    nn
}

/// Rename every at-rest pre-like element (see is_at_rest_pre_like) to
/// `<pre>`, recording the real tag in the marker attribute. Returns how
/// many elements were stamped — 0 means the item can go to prettier as-is.
fn stamp_pre_like(body: &markup5ever_rcdom::Handle) -> usize {
    use markup5ever_rcdom::NodeData;
    // Sites are collected before any swap: the Handles stay valid (the
    // children are shared Rcs, only the parent element is replaced).
    fn collect(n: &markup5ever_rcdom::Handle, sites: &mut Vec<markup5ever_rcdom::Handle>) {
        for c in n.children.borrow().iter() {
            if let NodeData::Element { attrs, .. } = &c.data {
                let pre_like = attrs
                    .borrow()
                    .iter()
                    .find(|a| &*a.name.local == "class")
                    .is_some_and(|a| is_at_rest_pre_like(&a.value));
                if pre_like {
                    sites.push(c.clone());
                }
            }
            collect(c, sites);
            if let NodeData::Element {
                template_contents, ..
            } = &c.data
                && let Some(tc) = template_contents.borrow().as_ref()
            {
                collect(tc, sites);
            }
        }
    }
    let mut sites = Vec::new();
    collect(body, &mut sites);
    for s in &sites {
        let NodeData::Element { name, attrs, .. } = &s.data else {
            unreachable!("collect only pushes elements");
        };
        let mut ab: Vec<html5ever::Attribute> = attrs.borrow().clone();
        ab.push(html5ever::Attribute {
            name: html5ever::QualName::new(
                None,
                html5ever::ns!(),
                html5ever::LocalName::from(PRE_WS_MARK),
            ),
            value: name.local.to_string().into(),
        });
        element_retagged(s, html5ever::LocalName::from("pre"), ab);
    }
    sites.len()
}

/// Undo stamp_pre_like on prettier's output. The formatted text is parsed
/// and walked IN LOCKSTEP with the stamped tree (a formatter reformats, it
/// does not restructure — a different element count or tag is a formatter
/// bug and fails loudly): stamped elements go back to their real tag, the
/// marker is dropped, and a leading newline the HTML parser discards right
/// after `<pre>` (the spec's ignore-first-newline rule) is put back from
/// the stamped tree so the shipped tab keeps the demo's text node.
fn strip_pre_like(
    stamped_body: &markup5ever_rcdom::Handle,
    formatted_text: &str,
) -> Result<String, String> {
    use markup5ever_rcdom::NodeData;
    let (_dom, fbody) = parse_body_fragment(formatted_text)?;
    fn elements(n: &markup5ever_rcdom::Handle, out: &mut Vec<markup5ever_rcdom::Handle>) {
        for c in n.children.borrow().iter() {
            if let NodeData::Element { .. } = &c.data {
                out.push(c.clone());
            }
            elements(c, out);
            if let NodeData::Element {
                template_contents, ..
            } = &c.data
                && let Some(tc) = template_contents.borrow().as_ref()
            {
                elements(tc, out);
            }
        }
    }
    let (mut sa, mut fa) = (Vec::new(), Vec::new());
    elements(stamped_body, &mut sa);
    elements(&fbody, &mut fa);
    if sa.len() != fa.len() {
        return Err(format!(
            "pre-like strip: the formatter restructured the DOM ({} stamped elements vs {} formatted)",
            sa.len(),
            fa.len()
        ));
    }
    let local = |h: &markup5ever_rcdom::Handle| match &h.data {
        NodeData::Element { name, .. } => name.local.to_string(),
        _ => String::new(),
    };
    for (s, f) in sa.iter().zip(fa.iter()) {
        // On the stamped tree the site IS the renamed <pre> (the original
        // element was replaced); its marker holds the real tag. The marker
        // must survive formatting on both sides or the sites disagree.
        let (sm, fm) = (attr_of(s, PRE_WS_MARK), attr_of(f, PRE_WS_MARK));
        match (&sm, &fm) {
            (Some(orig), Some(forig)) => {
                if orig != forig {
                    return Err(format!(
                        "pre-like strip: stamped <{orig}> came back marked <{forig}>"
                    ));
                }
                let kept: Vec<html5ever::Attribute> = match &f.data {
                    NodeData::Element { attrs, .. } => attrs
                        .borrow()
                        .iter()
                        .filter(|a| &*a.name.local != PRE_WS_MARK)
                        .cloned()
                        .collect(),
                    _ => unreachable!("attr_of only matches elements"),
                };
                // The parser dropped the newline right after <pre>; the
                // stamped tree still has it, so put it back — BEFORE the
                // retag moves f's children to the replacement.
                let sc = s.children.borrow();
                if let Some(first) = sc.first()
                    && let NodeData::Text { contents } = &first.data
                    && contents.borrow().starts_with('\n')
                {
                    drop(sc);
                    let fc = f.children.borrow();
                    match fc.first() {
                        Some(ff) => match &ff.data {
                            NodeData::Text { contents } => {
                                if !contents.borrow().starts_with('\n') {
                                    let restored = format!("\n{}", contents.borrow());
                                    *contents.borrow_mut() = restored.as_str().into();
                                }
                            }
                            _ => panic!(
                                "pre-like strip: the formatter replaced the element's first text child"
                            ),
                        },
                        None => panic!(
                            "pre-like strip: the formatter dropped the element's first text child"
                        ),
                    }
                }
                element_retagged(f, html5ever::LocalName::from(orig.as_str()), kept);
            }
            (None, None) => {
                let (sname, fname) = (local(s), local(f));
                if sname != fname {
                    return Err(format!(
                        "pre-like strip: element <{sname}> came back as <{fname}>"
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "pre-like strip: stamp marker lost or gained on <{}>",
                    local(f)
                ));
            }
        }
    }
    Ok(crate::emit::writer::serialize_children(&fbody))
}

fn demo_source(ctx: &DocsBuildCtx, file: &str) -> String {
    let path = format!("{}/demos/{}", STATIC_ROOT, file);
    let Some(markup) = ctx.markup.get(file) else {
        return String::new();
    };
    let scripts = read_demo_scripts(std::path::Path::new(&path));
    let mut js: Vec<String> = Vec::new();
    for s in &scripts.src_scripts {
        if s == "shadless.js" {
            js.push(
                "// <script src=\"shadless.js\"></script>  — the shared runtime (see Installation)"
                    .to_string(),
            );
            break;
        }
    }
    for s in &scripts.src_scripts {
        if s == "shadless.js" {
            continue;
        }
        let b = std::fs::read_to_string(format!("dist/{}", s)).unwrap_or_default();
        js.push(format!("// {}\n{}", s, b.trim()));
    }
    js.extend(scripts.inline_scripts.clone());
    let js_text = js.join("\n\n").trim().to_string();
    // the theme's codegroup: one fenced block per tab, the tab label from the
    // name= annotation. Markup rides the `text` language (highlighting off —
    // it is code a reader copies verbatim out of the shipped demo page).
    if js_text.is_empty() {
        return format!(
            "\n{{% <codegroup> %}}\n```text,name={}\n{}\n```\n{{% </codegroup> %}}\n",
            file, markup
        );
    }
    format!(
        "\n{{% <codegroup> %}}\n```text,name={}\n{}\n```\n\n```js,name=behavior\n{}\n```\n{{% </codegroup> %}}\n",
        file, markup, js_text
    )
}

fn build_page(
    ctx: &mut DocsBuildCtx,
    root: &Path,
    name: &str,
    source: &str,
    weight: usize,
    transform: impl Fn(&mut DocsBuildCtx, &Path, &str) -> Result<String, String>,
    skip_jsx_check: bool,
) -> Result<Vec<u8>, String> {
    let raw = std::fs::read_to_string(root.join(source)).map_err(|e| e.to_string())?;
    let fm = parse_frontmatter(&raw);
    let mut body = transform(ctx, root, &raw)?;
    body = RE_FM_BLOCK.replace_all(&body, "").into_owned();
    body = strip_imports_outside_fences(&body);
    body = replace_markup(&body, &RE_COMP_SOURCE_ALL, |_, _| String::new());
    body = convert_callouts(&body, name)?;
    body = convert_details(&body, name)?;
    body = convert_steps(&body);
    body = convert_kbd(&body);
    body = convert_linked_cards(&body);
    body = convert_class_name(&body);
    let site_members = ctx.site_members();
    body = rewrite_links(&body, &site_members);
    body = replace_markup(&body, &RE_COMP_PREVIEW_ALL, |whole, _| {
        let attrs = parse_attrs(whole);
        match preview_markdown(ctx, &attrs, name) {
            Ok(md) => md,
            Err(e) => format!("%%ERROR:{}%%", e),
        }
    });
    if let Some(i) = body.find("%%ERROR:") {
        // search AFTER the marker
        let start = i + "%%ERROR:".len();
        let mut end = body.len();
        if let Some(j) = body[start..].find("%%") {
            end = start + j;
        }
        return Err(body[start..end].to_string());
    }
    if !skip_jsx_check {
        assert_no_jsx(name, &body)?;
    }
    let mut title = fm_string(&fm, "title");
    if title.is_empty() {
        title = name.to_string();
    }
    // Upstream's frontmatter `links:` (the "doc · api" chips) are NOT emitted.
    let mut front = format!("---\ntitle: {}\n", yaml_scalar(&title));
    let d = fm_string(&fm, "description");
    if !d.is_empty() {
        front += &format!("description: {}\n", yaml_scalar(&d));
    }
    // sidebar order on the Zola site: sections sort their pages by weight
    front += &format!("weight: {}\n", weight);
    front += "---";
    let lead = if d.is_empty() {
        String::new()
    } else {
        format!("{}\n\n", d)
    };
    Ok(format!(
        "{}\n\n# {}\n\n{}{}\n",
        front,
        title,
        lead,
        normalize_blank_lines(body.trim())
    )
    .into_bytes())
}

/// normalizeBlankLines tidies the seams the section transforms leave behind.
/// Whitespace only, and only outside fences.
fn normalize_blank_lines(body: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut in_fence = false;
    for line in body.split('\n') {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            out.push(line.to_string());
            continue;
        }
        if in_fence {
            out.push(line.to_string());
            continue;
        }
        if line.trim().is_empty() {
            // at most one blank line in a row
            if out.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
                continue;
            }
            out.push(String::new());
            continue;
        }
        // a heading always gets a blank line above it
        if line.starts_with('#') && out.last().map(|l| !l.trim().is_empty()).unwrap_or(false) {
            out.push(String::new());
        }
        out.push(line.to_string());
    }
    out.join("\n")
}

impl DocsBuildCtx {
    fn site_members(&self) -> std::collections::HashSet<String> {
        let mut m = std::collections::HashSet::new();
        for s in mirror_set_cache() {
            m.insert(s.to_string());
        }
        m
    }
}

static MIRROR_SET_CACHE: OnceLock<Vec<String>> = OnceLock::new();

fn mirror_set_cache() -> &'static [String] {
    MIRROR_SET_CACHE.get_or_init(Vec::new)
}

fn set_mirror_set_cache(v: Vec<String>) {
    let _ = MIRROR_SET_CACHE.set(v);
}

// ---- the build --------------------------------------------------------------------

#[derive(Deserialize, Default)]
#[allow(dead_code)] // unused: run_docs_build parses rtl-langs.json as a bare HashMap
struct RtlLangsFile(HashMap<String, Vec<String>>);

pub fn run_docs_build(root: &Path) -> i32 {
    let catalog_b = match std::fs::read_to_string(root.join("docs/catalog.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("docs-build: {}", e);
            return 1;
        }
    };
    let catalog: super::docs_transforms::DocsCatalog = match serde_json::from_str(&catalog_b) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("docs-build: catalog: {}", e);
            return 1;
        }
    };
    let meta_b = match std::fs::read_to_string(root.join(DOCS_RADIX_DIR).join("meta.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("docs-build: {}", e);
            return 1;
        }
    };
    let meta: super::docs_transforms::DocsMeta = match serde_json::from_str(&meta_b) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("docs-build: meta: {}", e);
            return 1;
        }
    };
    let mut rtl_langs: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(rb) = std::fs::read_to_string(root.join("build/rtl-langs.json")) {
        if let Ok(m) = serde_json::from_str::<HashMap<String, Vec<String>>>(&rb) {
            rtl_langs = m;
        }
    }

    let mut component_pages: Vec<(String, String)> = Vec::new();
    for s in &catalog.sources {
        if s.status != "existing-dist" {
            continue;
        }
        let p = format!("{}/{}.mdx", DOCS_RADIX_DIR, s.name);
        if !root.join(&p).exists() {
            continue;
        }
        component_pages.push((s.name.clone(), p));
    }
    component_pages.sort();
    let mut mirror_set: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (name, _) in &component_pages {
        mirror_set.insert(name.clone());
    }
    let mut mirror_set_cache: Vec<String> = Vec::new();
    for n in &meta.pages {
        if mirror_set.contains(n) {
            mirror_set_cache.push(n.clone());
        }
    }
    mirror_set_cache.sort();
    set_mirror_set_cache(mirror_set_cache.clone());
    let mirror_total = component_pages.len() + guides().len();
    println!(
        "mirror set: {} components = {} pages + {} guides",
        mirror_set_cache.len(),
        component_pages.len(),
        guides().len()
    );

    let mut grey_set: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for g in grey_components() {
        grey_set.insert(g);
    }
    {
        let mut not_sub: Vec<String> = Vec::new();
        let mut overlap: Vec<String> = Vec::new();
        let mut uncovered: Vec<String> = Vec::new();
        let mut orphan: Vec<String> = Vec::new();
        for s in &catalog.sources {
            if s.status == "no-dist" && !grey_set.contains(s.name.as_str()) {
                not_sub.push(s.name.clone());
            }
        }
        for n in &mirror_set {
            if grey_set.contains(n.as_str()) {
                overlap.push(n.clone());
            }
        }
        let mut accounted: std::collections::HashSet<String> = std::collections::HashSet::new();
        for n in &mirror_set {
            accounted.insert(n.clone());
        }
        for g in grey_components() {
            accounted.insert(g.to_string());
        }
        for p in &meta.pages {
            if !accounted.contains(p) {
                uncovered.push(p.clone());
            }
        }
        for n in accounted.iter() {
            if !meta.pages.contains(n) {
                orphan.push(n.clone());
            }
        }
        if !not_sub.is_empty()
            || !overlap.is_empty()
            || !uncovered.is_empty()
            || !orphan.is_empty()
            || accounted.len() != meta.pages.len()
        {
            eprintln!(
                "FAIL grey-list cross-check: noDist-not-grey={:?} built∩grey={:?} meta-uncovered={:?} grey-not-in-meta={:?}",
                not_sub, overlap, uncovered, orphan
            );
            return 1;
        }
    }
    // docs/site's content/ and static/ are generated in full; only the theme
    // (themes/shadcn), the site shell (config.toml, templates/, sass/) and
    // this pipeline's own inputs are tracked
    for d in [
        format!("{}/components", CONTENT_ROOT),
        format!("{}/guides", CONTENT_ROOT),
        format!("{}/demos", STATIC_ROOT),
        format!("{}/js", STATIC_ROOT),
    ] {
        // Go os.RemoveAll succeeds on a missing path; Rust remove_dir_all
        // errors — NotFound is the same no-op.
        match std::fs::remove_dir_all(root.join(&d)) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
                eprintln!("docs-build: {}", e);
                return 1;
            }
            _ => {}
        }
        if let Err(e) = std::fs::create_dir_all(root.join(&d)) {
            eprintln!("docs-build: {}", e);
            return 1;
        }
    }
    // demos + assets into the served tree
    let mut copied = 0usize;
    for tree in ["dist/components", "docs/demos"] {
        let Ok(ents) = std::fs::read_dir(root.join(tree)) else {
            continue;
        };
        for e in ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if !n.ends_with(".html") {
                continue;
            }
            let Ok(b) = std::fs::read_to_string(root.join(tree).join(&n)) else {
                continue;
            };
            if let Err(e) = std::fs::write(root.join(STATIC_ROOT).join("demos").join(&n), b) {
                eprintln!("docs-build: {}", e);
                return 1;
            }
            copied += 1;
        }
    }
    for asset in ["out.css", "shadless.js"] {
        let Ok(b) = std::fs::read_to_string(root.join("dist").join(asset)) else {
            continue;
        };
        if let Err(e) = std::fs::write(root.join(STATIC_ROOT).join(asset), b) {
            eprintln!("docs-build: {}", e);
            return 1;
        }
    }
    let mut glue = 0usize;
    if let Ok(ents) = std::fs::read_dir(root.join("dist/js")) {
        for e in ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            let Ok(b) = std::fs::read_to_string(root.join("dist/js").join(&n)) else {
                continue;
            };
            if let Err(e) = std::fs::write(root.join(STATIC_ROOT).join("js").join(&n), b) {
                eprintln!("docs-build: {}", e);
                return 1;
            }
            glue += 1;
        }
    }
    println!(
        "demos copied: {} pages, {} behavior files + out.css/shadless.js",
        copied, glue
    );

    // markup pretty-printed ONCE per file via the prettier shell
    let mut items: Vec<(String, String)> = Vec::new();
    if let Ok(demo_ents) = std::fs::read_dir(root.join(STATIC_ROOT).join("demos")) {
        for e in demo_ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if !n.ends_with(".html") {
                continue;
            }
            let Ok(raw) = std::fs::read_to_string(root.join(STATIC_ROOT).join("demos").join(&n))
            else {
                continue;
            };
            let mut body = raw.clone();
            if let Some(m) = RE_BODY_TAG.captures(&raw) {
                body = m[1].to_string();
            }
            body = RE_SCRIPT_BLOCK.replace_all(&body, "").trim().to_string();
            items.push((n, body));
        }
    }
    // The code tab is SLOT-ONLY markup: every tab source is parsed, each
    // slotted element keeps only the classes the component stylesheet does
    // not already supply, and the result is serialized raw (the writer
    // keeps `&` unescaped wherever it cannot start a character reference —
    // a consumer's Tailwind scan must read `[&_a]`, not `&amp;_a`; prettier
    // preserves entities verbatim). RTL demos are exempt: their tab keeps
    // every class, because build-rtl rewrote their utilities into logical
    // forms (pe-8, ps-1.5, end-2, …) that dist/css does not carry.
    crate::emit::load_skin();
    let slot_index = SlotIndex::load(root);
    let demo_stems: Vec<String> = items
        .iter()
        .map(|(n, _)| n.trim_end_matches(".html").to_string())
        .collect();
    let rtl_stems = rtl_exempt_stems(root, &demo_stems);
    let mut metrics = ProjectMetrics::default();
    let mut projected_items: Vec<(String, String)> = Vec::new();
    for (n, body) in &items {
        let stem = n.trim_end_matches(".html");
        if rtl_stems.contains(stem) {
            metrics.files_rtl_exempt += 1;
            // The exemption is real, not emergent: the RTL tab keeps EVERY
            // class. build-rtl rewrote their utilities into logical forms
            // (pe-8, ps-1.5, end-2, …) that dist/css does not carry, so the
            // projection has nothing honest to subtract — it is serialized
            // through the code-tab writer (same attribute escaping as every
            // tab) with the projection off, and no metrics are recorded.
            match parse_body_fragment(body) {
                Ok((_dom, b)) => {
                    projected_items.push((n.clone(), crate::emit::writer::serialize_children(&b)))
                }
                Err(e) => {
                    eprintln!("docs-build: {}: {}", n, e);
                    return 1;
                }
            }
        } else {
            metrics.files_projected += 1;
            match project_body(body, &slot_index, &mut metrics) {
                Ok(p) => projected_items.push((n.clone(), p)),
                Err(e) => {
                    eprintln!("docs-build: {}: {}", n, e);
                    return 1;
                }
            }
        }
    }
    println!(
        "code-tab projection: {} files projected, {} RTL files exempt (every class kept); {} indexed slotted elements: {} → {} class tokens ({} now classless, {:.1}% no class), {} unindexed slots kept verbatim, {} inconsistent-fallback elements kept whole, {} collision ties, {} closure-kept tokens",
        metrics.files_projected,
        metrics.files_rtl_exempt,
        metrics.slotted,
        metrics.tokens_before,
        metrics.tokens_after,
        metrics.zero_class_after,
        if metrics.slotted > 0 {
            100.0 * metrics.zero_class_after as f64 / metrics.slotted as f64
        } else {
            0.0
        },
        metrics.unindexed_slot,
        metrics.fallback_inconsistent,
        metrics.collision_ties,
        metrics.closure_kept_tokens,
    );

    // Prettier cannot see white-space:pre* utility classes; inside such a
    // subtree its reflow renders extra line boxes (see the stamp_pre_like
    // comment above). Items carrying one take the stamp -> prettier ->
    // strip detour; every other item goes through byte-identically to
    // before.
    let mut stamped: HashMap<String, (markup5ever_rcdom::RcDom, markup5ever_rcdom::Handle)> =
        HashMap::new();
    let mut prettier_items: Vec<(String, String)> = Vec::new();
    for (n, text) in &projected_items {
        if text.contains("whitespace-pre") {
            match parse_body_fragment(text) {
                Ok((dom, b)) => {
                    if stamp_pre_like(&b) > 0 {
                        let t = crate::emit::writer::serialize_children(&b);
                        stamped.insert(n.clone(), (dom, b));
                        prettier_items.push((n.clone(), t));
                        continue;
                    }
                }
                Err(e) => {
                    eprintln!("docs-build: {}: {}", n, e);
                    return 1;
                }
            }
        }
        prettier_items.push((n.clone(), text.clone()));
    }
    let pretty = match prettier_batch(&prettier_items) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("docs-build: prettier: {}", e);
            return 1;
        }
    };
    let mut markup: HashMap<String, String> = HashMap::new();
    for (n, formatted) in &pretty {
        match stamped.get(n) {
            Some((_, b)) => match strip_pre_like(b, formatted) {
                Ok(t) => {
                    markup.insert(n.clone(), t);
                }
                Err(e) => {
                    eprintln!("docs-build: {}: {}", n, e);
                    return 1;
                }
            },
            None => {
                markup.insert(n.clone(), formatted.clone());
            }
        }
    }

    let mut ctx = DocsBuildCtx {
        catalog,
        rtl_langs,
        markup,
        sections: HashMap::new(),
        real_slots: None,
        ir_cache: HashMap::new(),
    };

    let mut all_pages: Vec<(String, String, String, usize, bool, bool)> = Vec::new();
    for (i, (name, source)) in component_pages.iter().enumerate() {
        all_pages.push((
            name.clone(),
            source.clone(),
            format!("{}/components", CONTENT_ROOT),
            i + 1,
            false,
            false,
        ));
    }
    for (i, g) in guides().iter().enumerate() {
        all_pages.push((
            g.slug.to_string(),
            g.source.to_string(),
            format!("{}/guides", CONTENT_ROOT),
            i + 1,
            false,
            true,
        ));
    }
    let mut built = 0usize;
    let mut errors_: Vec<String> = Vec::new();
    for (name, source, dir, weight, skip_jsx_check, is_guide) in &all_pages {
        let out = if *is_guide {
            let g = guides().iter().find(|g| g.slug == *name).unwrap();
            build_page(
                &mut ctx,
                root,
                name,
                source,
                *weight,
                |_ctx, root, raw| guide_transform(root, g, raw),
                *skip_jsx_check,
            )
        } else {
            build_page(
                &mut ctx,
                root,
                name,
                source,
                *weight,
                |ctx, root, raw| ctx.component_transform(root, name, raw),
                *skip_jsx_check,
            )
        };
        match out {
            Ok(out) => {
                if let Err(e) = std::fs::write(root.join(dir).join(format!("{}.md", name)), out) {
                    errors_.push(format!("{}: {}", name, e));
                    continue;
                }
                built += 1;
            }
            Err(e) => errors_.push(format!("{}: {}", name, e)),
        }
    }

    // content map with the per-page section sets the transforms recorded
    let cp_pages: Vec<(String, String)> = component_pages.clone();
    if let Err(e) = write_content_map(root, &cp_pages, &ctx.sections) {
        eprintln!("docs-build: {}", e);
        return 1;
    }

    // home + section pages. The Zola sidebar is derived per top-level section
    // from the content tree (weight order set above), so there is no sidebar
    // artifact to emit.
    let mut guide_by_slug: HashMap<&str, &Guide> = HashMap::new();
    for g in guides() {
        guide_by_slug.insert(g.slug, g);
    }
    let mut comp_index = format!(
        "---\ntitle: \"Components\"\nsort_by: \"weight\"\n---\n\n# Components\n\n{} components ported · {} not ported (they need React, or upstream removed them) · {} guides.\n\nNew here? Read the [Introduction](/guides/introduction) to learn what shadless is and why it exists.\n\n",
        mirror_set_cache.len(),
        grey_components().len(),
        guides().len()
    );
    for n in &meta.pages {
        if grey_set.contains(n.as_str()) {
            let mut line = format!("- {} <span class=\"unavailable\">not available</span>", n);
            // A name can be greyed as a component and still have a guide —
            // typography is CSS, not a component, so it ships as one.
            if let Some(g) = guide_by_slug.get(n.as_str()) {
                line += &format!(" — see the [{}](/guides/{}) guide", g.title, g.slug);
            }
            comp_index += &format!("{}\n", line);
        } else {
            comp_index += &format!("- [{}](/components/{})\n", n, n);
        }
    }
    comp_index += "\n## Guides\n\n";
    for g in guides() {
        comp_index += &format!("- [{}](/guides/{})\n", g.title, g.slug);
    }
    if let Err(e) = std::fs::create_dir_all(root.join(CONTENT_ROOT).join("components")) {
        eprintln!("docs-build: {}", e);
        return 1;
    }
    if let Err(e) = std::fs::write(
        root.join(CONTENT_ROOT).join("components").join("_index.md"),
        comp_index,
    ) {
        eprintln!("docs-build: {}", e);
        return 1;
    }
    let guides_index = format!(
        "---\ntitle: \"Guides\"\nsort_by: \"weight\"\n---\n\n# Guides\n\n{} guides — ported where they make sense for a component library that ships static HTML.\n",
        guides().len()
    );
    if let Err(e) = std::fs::write(
        root.join(CONTENT_ROOT).join("guides").join("_index.md"),
        guides_index,
    ) {
        eprintln!("docs-build: {}", e);
        return 1;
    }
    // the hero home — the theme's index.html renders vitepress_home (plus a
    // non-placeholder body), so the component listing lives on /components/ above
    // TOML frontmatter: the vitepress_home tables are TOML, and a YAML block
    // cannot carry them (the pages keep YAML — Zola accepts either per file)
    let home = format!(
        "+++\ntemplate = \"index.html\"\n[extra.vitepress_home]\ntext = \"shadless\"\ntagline = \"shadcn/ui as static HTML and a vanilla runtime — no React. Copy the markup, load the stylesheets, done.\"\n[[extra.vitepress_home.actions]]\ntext = \"Get Started\"\ntheme = \"brand\"\nlink = \"/guides/introduction/\"\n\n[[extra.vitepress_home.actions]]\ntext = \"Components\"\ntheme = \"alt\"\nlink = \"/components/\"\n\n[[extra.vitepress_home.features]]\ntitle = \"Static HTML\"\ndetails = \"Every component is markup + CSS you copy — data-slot attributes plus only the utilities you add; the stylesheet supplies the rest. No framework.\"\n\n[[extra.vitepress_home.features]]\ntitle = \"Vanilla runtime\"\ndetails = \"A dependency-free JS base wires the interactive pieces; shadless.init(root) for content added later.\"\n\n[[extra.vitepress_home.features]]\ntitle = \"shadcn/ui, ported\"\ndetails = \"{} components mirrored from the pinned upstream registry, plus the guides that still apply.\"\n+++\n",
        mirror_set_cache.len()
    );
    if let Err(e) = std::fs::write(root.join(CONTENT_ROOT).join("_index.md"), home) {
        eprintln!("docs-build: {}", e);
        return 1;
    }

    if !errors_.is_empty() {
        for e in &errors_ {
            eprintln!("  - {}", e);
        }
        eprintln!("FAIL  docs build ({} pages failed)", errors_.len());
        return 1;
    }
    if built != mirror_total {
        eprintln!(
            "FAIL  docs build (built {}, expected {})",
            built, mirror_total
        );
        return 1;
    }
    println!(
        "PASS  docs build ({}/{} pages: {} components + {} guides → markdown)",
        built,
        mirror_total,
        mirror_set_cache.len(),
        guides().len()
    );
    0
}

/// writeContentMap emits docs/content-map.json byte-compatibly with the JS
/// writer (JSON.stringify(_, null, 2) + "\n"; key order = insertion order).
fn write_content_map(
    root: &Path,
    component_pages: &[(String, String)],
    sections: &HashMap<String, Vec<String>>,
) -> Result<(), String> {
    let catalog_b =
        std::fs::read_to_string(root.join("docs/catalog.json")).map_err(|e| e.to_string())?;
    #[derive(Deserialize)]
    struct Catalog {
        #[serde(default)]
        previews: Vec<CatalogPreview>,
    }
    #[derive(Deserialize)]
    struct CatalogPreview {
        #[serde(default)]
        name: String,
        #[serde(default)]
        status: String,
    }
    let catalog: Catalog = serde_json::from_str(&catalog_b).map_err(|e| e.to_string())?;
    let mut status: HashMap<String, String> = HashMap::new();
    for p in catalog.previews {
        status.insert(p.name, p.status);
    }
    let (gp, gp_order) = scan_guide_previews(root, &status);

    // pages: components (sorted), then guides, then index — the JS writer's
    // insertion order. sections land LAST inside each entry.
    let mut pages_kv: Vec<(String, Json)> = Vec::new();
    for (name, source) in component_pages {
        let mut e: Vec<(String, Json)> = vec![
            ("source".to_string(), Json::Str(source.clone())),
            ("disposition".to_string(), Json::Str("adapted".to_string())),
            (
                "notes".to_string(),
                Json::Str("radix mirror; installation Manual tab rewritten to the vanilla copy-files path (build-time transform over dist/ artifacts)".to_string()),
            ),
        ];
        if let Some(sec) = sections.get(name) {
            if !sec.is_empty() {
                e.push((
                    "sections".to_string(),
                    Json::Arr(sec.iter().map(|x| Json::Str(x.clone())).collect()),
                ));
            }
        }
        pages_kv.push((name.clone(), Json::Obj(e)));
    }
    for g in guides() {
        let mut e: Vec<(String, Json)> = vec![
            ("source".to_string(), Json::Str(g.source.to_string())),
            (
                "disposition".to_string(),
                Json::Str(g.disposition.to_string()),
            ),
            ("notes".to_string(), Json::Str(g.notes.to_string())),
        ];
        if let Some(sec) = sections.get(g.slug) {
            if !sec.is_empty() {
                e.push((
                    "sections".to_string(),
                    Json::Arr(sec.iter().map(|x| Json::Str(x.clone())).collect()),
                ));
            }
        }
        pages_kv.push((g.slug.to_string(), Json::Obj(e)));
    }
    pages_kv.push((
        "index".to_string(),
        Json::Obj(vec![
            ("source".to_string(), Json::Str("(generated)".to_string())),
            (
                "disposition".to_string(),
                Json::Str("generated".to_string()),
            ),
            (
                "notes".to_string(),
                Json::Str("components + guides index page".to_string()),
            ),
        ]),
    ));

    // pruned in the recorded order
    let pkeys = [
        "forms",
        "react",
        "registry",
        "changelog",
        "(root)",
        "helpers",
        "framework sub-pages",
    ];
    let mut pruned_kv: Vec<(String, Json)> = Vec::new();
    for k in pkeys {
        let p = super::docs_transforms::pruned_guides()
            .iter()
            .find(|(name, _, _)| *name == k)
            .unwrap();
        pruned_kv.push((
            k.to_string(),
            Json::Obj(vec![
                ("source".to_string(), Json::Str(p.1.to_string())),
                ("reason".to_string(), Json::Str(p.2.to_string())),
            ]),
        ));
    }

    // guidePreviews in first-seen order
    let mut gp_kv: Vec<(String, Json)> = Vec::new();
    for name in &gp_order {
        let g = &gp[name];
        let style: Json = if g.style_name.is_empty() {
            Json::Null
        } else {
            Json::Str(g.style_name.clone())
        };
        gp_kv.push((
            name.clone(),
            Json::Obj(vec![
                (
                    "hostPages".to_string(),
                    Json::Arr(g.host_pages.iter().map(|h| Json::Str(h.clone())).collect()),
                ),
                ("styleName".to_string(), style),
                ("disposition".to_string(), Json::Str(g.disposition.clone())),
                ("reason".to_string(), Json::Str(g.reason.clone())),
            ]),
        ));
    }

    let root_obj = JsonObj::new()
        .add("version", Json::Int(1))
        .add(
            "generatedBy",
            Json::Str("pipeline/src/tools/docs_build.rs".to_string()),
        )
        .add("pages", Json::Obj(pages_kv))
        .add("pruned", Json::Obj(pruned_kv))
        .add("guidePreviews", Json::Obj(gp_kv));
    std::fs::create_dir_all(root.join("docs")).map_err(|e| e.to_string())?;
    std::fs::write(
        root.join("docs/content-map.json"),
        format!(
            "{}\n",
            crate::jsonorder::marshal_js(&Json::from_obj(root_obj))
        ),
    )
    .map_err(|e| e.to_string())
}

/// prettierBatch runs the one-shot node shell over all items.
fn prettier_batch(items: &[(String, String)]) -> Result<HashMap<String, String>, String> {
    if items.is_empty() {
        return Ok(HashMap::new());
    }
    // Go json.Marshal([]prettierItem{File, Body}) — an array of OBJECTS
    let stdin = serde_json::to_string(
        &items
            .iter()
            .map(|(f, b)| serde_json::json!({"file": f, "body": b}))
            .collect::<Vec<_>>(),
    )
    .map_err(|e| e.to_string())?;
    let mut child = std::process::Command::new("node")
        .arg("tools/prettier-batch.mjs")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    use std::io::Write;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stdin.as_bytes())
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "{}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::{base_token, conflicts};

    #[test]
    fn unit_base_token_strips_variants_and_important() {
        assert_eq!(base_token("px-2"), "px-2");
        assert_eq!(base_token("hover:px-2"), "px-2");
        assert_eq!(base_token("dark:sm:hover:px-2"), "px-2");
        assert_eq!(base_token("!px-2"), "px-2");
        assert_eq!(base_token("px-2!"), "px-2");
        assert_eq!(base_token("hover:!px-2"), "px-2");
        // colons inside arbitrary values are not variant separators
        assert_eq!(
            base_token("bg-[color-mix(in_oklch,var(--a),var(--b)_5%)]"),
            "bg-[color-mix(in_oklch,var(--a),var(--b)_5%)]"
        );
        assert_eq!(
            base_token("md:bg-[url(https://x/y)]"),
            "bg-[url(https://x/y)]"
        );
        assert_eq!(base_token("[&_a]:underline"), "underline");
    }

    #[test]
    fn unit_supply_limits_what_a_multi_element_slot_drops() {
        // css.rs pass 1: for a non-cva slot with two differently-classed
        // elements, only the COMMON tokens go under [data-slot]; the rest
        // is anchor-keyed and must stay inline in the projected tab — the
        // demo DOM carries no anchor classes.
        use std::collections::{HashMap, HashSet};

        use super::{SlotEntry, SlotIndex};
        use crate::emit::css::{CssIrComponent, IrEl, IrFn};
        let mk_el = |slot: &str, classes: &[&str]| IrEl {
            tag: "svg".to_string(),
            slot: slot.to_string(),
            classes: classes.iter().map(|c| c.to_string()).collect(),
            ..Default::default()
        };
        let ir = CssIrComponent {
            name: "accordion".to_string(),
            components: vec![IrFn {
                fn_: "AccordionTrigger".to_string(),
                export: true,
                elements: vec![
                    mk_el(
                        "accordion-trigger-icon",
                        &[
                            "pointer-events-none",
                            "shrink-0",
                            "group-aria-expanded/accordion-trigger:hidden",
                        ],
                    ),
                    mk_el(
                        "accordion-trigger-icon",
                        &[
                            "pointer-events-none",
                            "hidden",
                            "shrink-0",
                            "group-aria-expanded/accordion-trigger:inline",
                        ],
                    ),
                ],
            }],
            ..Default::default()
        };
        let mut index = SlotIndex {
            irs: vec![ir],
            cva_slots: vec![HashMap::new()],
            slots: HashMap::new(),
            supply: Vec::new(),
            own_rule: Vec::new(),
        };
        index.slots.insert(
            "accordion-trigger-icon".to_string(),
            vec![
                SlotEntry {
                    ir: 0,
                    fn_: "AccordionTrigger".to_string(),
                    el: 0,
                },
                SlotEntry {
                    ir: 0,
                    fn_: "AccordionTrigger".to_string(),
                    el: 1,
                },
            ],
        );
        let mut sup: HashMap<String, HashSet<String>> = HashMap::new();
        sup.insert(
            "accordion-trigger-icon".to_string(),
            ["pointer-events-none".to_string(), "shrink-0".to_string()]
                .into_iter()
                .collect(),
        );
        index.own_rule.push(sup.clone());
        index.supply.push(sup);

        // the second icon: `hidden` is anchor-keyed — it must SURVIVE the
        // projection even though the candidate's full M contains it
        let html = r#"<div><svg class="lucide lucide-chevron-up pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline" data-slot="accordion-trigger-icon"></svg></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        // `hidden` and the :inline variant are anchor-keyed — they must
        // survive the projection even though the candidate's full M carries
        // them
        assert!(
            out.contains(
                "lucide lucide-chevron-up hidden group-aria-expanded/accordion-trigger:inline"
            ),
            "anchor-keyed tokens must stay inline: {out}"
        );
        // the shared tokens drop (the [data-slot] rule supplies them)
        assert!(
            !out.contains("pointer-events-none") && !out.contains("shrink-0"),
            "slot-supplied tokens still drop: {out}"
        );
    }

    #[test]
    fn unit_conflicts_catches_layer_flips() {
        // same property, different value: a twMerge in either direction
        // drops one, so which wins is order-dependent
        assert!(conflicts("px-(--card-spacing)", "p-6"));
        assert!(conflicts("p-6", "px-(--card-spacing)"));
        // the order-dependent pair the css-order closure exists for
        assert!(conflicts("line-clamp-1", "flex"));
        assert!(conflicts("flex", "line-clamp-1"));
        // equal bases are the SAME declarations — order can never matter
        assert!(!conflicts("hover:bg-muted", "bg-muted"));
        assert!(!conflicts("p-6", "p-6"));
        // unrelated properties never conflict
        assert!(!conflicts("rounded-xl", "gap-4"));
    }

    /// One-slot index whose candidate M is exactly `m_classes` (no cva, no
    /// axes — classes_of returns the element's classes) with the given
    /// [data-slot] supply.
    fn single_slot_index(slot: &str, m_classes: &[&str], supply: &[&str]) -> super::SlotIndex {
        use crate::emit::css::{CssIrComponent, IrEl, IrFn};
        use std::collections::{HashMap, HashSet};
        let ir = CssIrComponent {
            name: "test".to_string(),
            components: vec![IrFn {
                fn_: "Test".to_string(),
                export: true,
                elements: vec![IrEl {
                    tag: "div".to_string(),
                    slot: slot.to_string(),
                    classes: m_classes.iter().map(|c| c.to_string()).collect(),
                    ..Default::default()
                }],
            }],
            ..Default::default()
        };
        let mut index = super::SlotIndex {
            irs: vec![ir],
            cva_slots: vec![HashMap::new()],
            slots: HashMap::new(),
            supply: Vec::new(),
            own_rule: Vec::new(),
        };
        index.slots.insert(
            slot.to_string(),
            vec![super::SlotEntry {
                ir: 0,
                fn_: "Test".to_string(),
                el: 0,
            }],
        );
        let mut sup: HashMap<String, HashSet<String>> = HashMap::new();
        sup.insert(
            slot.to_string(),
            supply.iter().map(|c| c.to_string()).collect(),
        );
        index.own_rule.push(sup.clone());
        index.supply.push(sup);
        index
    }

    #[test]
    fn unit_consistency_fallback_keeps_unrendered_candidates() {
        // The candidate's M carries `shrink-0`, but the demo DOM never
        // rendered it (D has gap-2 instead and nothing overrides it): our M
        // is not what React rendered for this element, so dropping anything
        // would be a guess — all of D stays. Deleting the consistency rule
        // makes the projection subtract shrink-0-supplied tokens anyway and
        // this test fails.
        let index = single_slot_index(
            "test-icon",
            &["pointer-events-none", "shrink-0"],
            &["pointer-events-none", "shrink-0"],
        );
        let html =
            r#"<div><span data-slot="test-icon" class="pointer-events-none gap-2"></span></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert_eq!(m.fallback_inconsistent, 1, "{out}");
        // a fallback element counts its tokens once on each side
        assert_eq!((m.tokens_before, m.tokens_after), (2, 2), "{out}");
        assert!(
            out.contains(r#"class="pointer-events-none gap-2""#),
            "unrendered candidate must keep all of D: {out}"
        );
    }

    #[test]
    fn unit_metrics_count_indexed_elements_only() {
        // An unindexed slot keeps its classes verbatim and stays out of the
        // before/after token counts on both sides; counting its kept tokens
        // after but not before inflated the printed total.
        let index = single_slot_index("test-icon", &["shrink-0"], &["shrink-0"]);
        let html = r#"<div data-slot="test-hook" class="prose p-4"><span data-slot="test-icon" class="shrink-0"></span></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert_eq!(m.unindexed_slot, 1, "{out}");
        assert_eq!((m.tokens_before, m.tokens_after), (1, 0), "{out}");
        assert_eq!(m.zero_class_after, 1, "{out}");
        assert!(out.contains(r#"class="prose p-4""#), "{out}");
    }

    #[test]
    fn unit_closure_keeps_dropped_token_conflicting_with_a_kept_one() {
        // `sm:w-px` is supplied by the [data-slot] rule and dropped from
        // the tab — but the demo author's own `w-full` stays inline. In the
        // original both sat in the utilities layer, where the varianted
        // sm:w-px sorts after w-full and wins at sm. After the projection
        // sm:w-px would come from @layer components, which the kept
        // utilities-layer w-full beats — so the closure moves sm:w-px back
        // into K. Deleting the same-element closure makes this test fail.
        let index = single_slot_index(
            "test-trigger",
            &["sm:w-px", "rounded-md"],
            &["sm:w-px", "rounded-md"],
        );
        let html = r#"<div><button data-slot="test-trigger" class="sm:w-px w-full rounded-md"></button></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert!(
            out.contains(r#"class="sm:w-px w-full""#),
            "sm:w-px conflicts with the kept w-full and must stay: {out}"
        );
        // rounded-md conflicts with nothing kept or supplied — it still drops
        assert!(!out.contains("rounded-md"), "{out}");
    }

    #[test]
    fn unit_closure_drops_unvarianted_token_under_a_kept_variant() {
        // The mirror image: the supplied token is unvarianted (w-px), the
        // kept one varianted (sm:w-full). Inline, sm:w-full sorted after
        // w-px and won exactly at sm; from the utilities layer over @layer
        // components it still wins exactly at sm — no flip, w-px drops.
        // Deleting variant_cannot_flip keeps w-px and this test fails.
        let index = single_slot_index(
            "test-trigger",
            &["w-px", "rounded-md"],
            &["w-px", "rounded-md"],
        );
        let html = r#"<div><button data-slot="test-trigger" class="w-px sm:w-full rounded-md"></button></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert!(out.contains(r#"class="sm:w-full""#), "{out}");
    }

    #[test]
    fn unit_closure_drops_a_same_rule_pair_both_rendered_inline() {
        // item-title's default shape: D carries line-clamp-1 AND flex, and
        // the slot's own [data-slot] rule applies both. @apply emits them in
        // the utilities layer's sort order, so the pair resolves exactly as
        // it did inline — both drop and the class attribute goes. Deleting
        // the same-rule exemption keeps both and this test fails.
        let index = single_slot_index(
            "test-title",
            &["line-clamp-1", "flex"],
            &["line-clamp-1", "flex"],
        );
        let html = r#"<div><span data-slot="test-title" class="line-clamp-1 flex"></span></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert!(
            out.contains(r#"<span data-slot="test-title"></span>"#),
            "{out}"
        );
        assert_eq!((m.tokens_before, m.tokens_after), (2, 0));
    }

    #[test]
    fn unit_closure_drops_a_pair_inside_the_selected_cva_value_rule() {
        // The button's default size rule applies px-2.5 AND
        // has-data-[icon=inline-end]:pr-2 in one @apply, and the demo DOM
        // rendered both inline: the pair cannot flip, both drop with the
        // base. Without the value rule in the element's rule set the pair
        // stays inline — and then a copied tab would pin the default size's
        // padding against a later data-size change.
        use crate::emit::css::{CssIrComponent, CvaTable, IrEl, IrFn};
        use std::collections::{HashMap, HashSet};
        let table: CvaTable = serde_json::from_value(serde_json::json!({
            "base": "rounded-lg",
            "variants": {"size": {
                "default": "px-2.5 has-data-[icon=inline-end]:pr-2",
                "sm": "px-2 has-data-[icon=inline-end]:pr-1.5"
            }},
            "defaults": {"size": "default"}
        }))
        .expect("table");
        let ir = CssIrComponent {
            name: "test".to_string(),
            components: vec![IrFn {
                fn_: "Test".to_string(),
                export: true,
                elements: vec![IrEl {
                    tag: "button".to_string(),
                    slot: "test-button".to_string(),
                    classes: vec![
                        "rounded-lg".to_string(),
                        "px-2.5 has-data-[icon=inline-end]:pr-2".to_string(),
                    ],
                    ..Default::default()
                }],
            }],
            ..Default::default()
        };
        let mut cva: HashMap<String, CvaTable> = HashMap::new();
        cva.insert("test-button".to_string(), table);
        let mut index = super::SlotIndex {
            irs: vec![ir],
            cva_slots: vec![cva],
            slots: HashMap::new(),
            supply: vec![HashMap::new()], // cva slot: everything supplied
            own_rule: Vec::new(),
        };
        index.slots.insert(
            "test-button".to_string(),
            vec![super::SlotEntry {
                ir: 0,
                fn_: "Test".to_string(),
                el: 0,
            }],
        );
        let mut own: HashMap<String, HashSet<String>> = HashMap::new();
        own.insert(
            "test-button".to_string(),
            ["rounded-lg".to_string()].into_iter().collect(),
        );
        index.own_rule.push(own);
        let html = r#"<div><button data-slot="test-button" data-size="default" class="rounded-lg px-2.5 has-data-[icon=inline-end]:pr-2"></button></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert!(
            out.contains(r#"<button data-slot="test-button" data-size="default"></button>"#),
            "{out}"
        );
    }

    #[test]
    fn unit_targets_other_elements_shape() {
        use super::{same_target, targets_other_elements};
        assert!(targets_other_elements(
            "[&_svg:not([class*='size-'])]:size-4"
        ));
        assert!(targets_other_elements("[&>input]:pl-1.5"));
        assert!(targets_other_elements("has-[>svg]:[&>input]:pl-1.5"));
        assert!(targets_other_elements("*:data-[slot=button]:rounded-none"));
        assert!(targets_other_elements("**:data-[slot=x]:p-0"));
        assert!(targets_other_elements("[&+div]:mt-2"));
        // the element itself
        assert!(!targets_other_elements("[&:hover]:bg-muted"));
        assert!(!targets_other_elements("has-[>svg]:px-2"));
        assert!(!targets_other_elements("group-hover/card:p-2"));
        assert!(!targets_other_elements("[.border-b]:pb-4"));
        assert!(!targets_other_elements("h-8"));
        // an ancestor condition (`&` at the end) still styles the element
        assert!(!targets_other_elements("[[data-x]_&]:p-1"));
        assert!(!same_target("h-8", "[&_svg]:size-4"));
        assert!(same_target("h-8", "hover:h-9"));
        assert!(same_target("[&_svg]:size-3", "[&_svg]:size-4"));
    }

    #[test]
    fn unit_closure_ignores_a_pair_on_different_elements() {
        // h-8 sizes the button, `[&_svg]:size-4` its svg: they never meet
        // on one element, so the stripped-variant conflict (size-4 vs h-8)
        // is not one. The supplied `[&_svg]:size-4` is not rendered (D
        // overrides it with size-3), which would otherwise pin h-8 inline.
        // Deleting same_target keeps h-8 and this test fails.
        let index = single_slot_index(
            "test-button",
            &["[&_svg]:size-4", "h-8"],
            &["[&_svg]:size-4", "h-8"],
        );
        let html =
            r#"<div><button data-slot="test-button" class="h-8 [&_svg]:size-3"></button></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert!(out.contains(r#"class="[&_svg]:size-3""#), "{out}");
    }

    #[test]
    fn unit_closure_keeps_dropped_token_conflicting_with_another_supplied_one() {
        // The item-image shape: the [data-slot] rule applies line-clamp-1
        // AND flex in one @apply, where flex wins (line-clamp-1 sets
        // display: -webkit-box, flex sets display: flex). The demo DOM
        // carries line-clamp-1 but NOT flex — React's twMerge removed it —
        // so inline, line-clamp-1 beat the rule's flex from the utilities
        // layer; dropping it hands the win to flex, so the closure keeps it.
        // Deleting the css-conflict arm empties K entirely and this test
        // fails.
        let index = single_slot_index(
            "test-title",
            &["line-clamp-1", "flex"],
            &["line-clamp-1", "flex"],
        );
        let html = r#"<div><span data-slot="test-title" class="line-clamp-1"></span></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert!(
            out.contains(r#"class="line-clamp-1""#),
            "line-clamp-1 conflicts with the rule's own flex and must stay: {out}"
        );
    }

    #[test]
    fn unit_closure_cross_element_keeps_descendant_targeted_token() {
        // The spinner shape: the trigger's supplied token `[&_svg]:size-4`
        // targets descendants; the icon's own `size-3` stays inline. In the
        // original both sat inline (specificity gave the trigger's token the
        // icon); after the projection the trigger's token would come from
        // @layer components, which the icon's inline size-3 beats — so the
        // closure moves the trigger's token back.
        // Deleting the cross-element settle makes this test fail.
        use crate::emit::css::{CssIrComponent, IrEl, IrFn};
        use std::collections::{HashMap, HashSet};
        let ir = CssIrComponent {
            name: "test".to_string(),
            components: vec![IrFn {
                fn_: "Test".to_string(),
                export: true,
                elements: vec![
                    IrEl {
                        tag: "button".to_string(),
                        slot: "test-trigger".to_string(),
                        classes: vec!["[&_svg]:size-4".to_string(), "inline-flex".to_string()],
                        ..Default::default()
                    },
                    IrEl {
                        tag: "svg".to_string(),
                        slot: "test-icon".to_string(),
                        classes: vec!["size-4".to_string()],
                        ..Default::default()
                    },
                ],
            }],
            ..Default::default()
        };
        let mut index = super::SlotIndex {
            irs: vec![ir],
            cva_slots: vec![HashMap::new()],
            slots: HashMap::new(),
            supply: Vec::new(),
            own_rule: Vec::new(),
        };
        index.slots.insert(
            "test-trigger".to_string(),
            vec![super::SlotEntry {
                ir: 0,
                fn_: "Test".to_string(),
                el: 0,
            }],
        );
        index.slots.insert(
            "test-icon".to_string(),
            vec![super::SlotEntry {
                ir: 0,
                fn_: "Test".to_string(),
                el: 1,
            }],
        );
        let mut sup: HashMap<String, HashSet<String>> = HashMap::new();
        sup.insert(
            "test-trigger".to_string(),
            ["[&_svg]:size-4".to_string(), "inline-flex".to_string()]
                .into_iter()
                .collect(),
        );
        sup.insert(
            "test-icon".to_string(),
            ["size-4".to_string()].into_iter().collect(),
        );
        index.own_rule.push(sup.clone());
        index.supply.push(sup);

        let html = r#"<div><button data-slot="test-trigger" class="[&_svg]:size-4 inline-flex"><svg data-slot="test-icon" class="size-3"></svg></button></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        // inline-flex is supplied by the [data-slot] rule and drops;
        // the descendant-targeting token survives the closure
        assert!(
            out.contains("[&_svg]:size-4") && !out.contains("inline-flex"),
            "the descendant-targeting token must survive the cross-element closure: {out}"
        );
        assert!(
            out.contains(r#"class="size-3""#),
            "the icon's own override stays: {out}"
        );
    }

    #[test]
    fn unit_closure_reruns_same_element_after_a_cross_element_move() {
        // The label's px-3 and pr-1 are both rendered inline and both in
        // its own rule, so the same-element closure lets them drop. Then
        // the parent's kept `[&_span]:pl-2` (it beat px-3's left padding
        // inline, by specificity) pulls px-3 back into K. Now px-3 sits in
        // the utilities layer over pr-1 in @layer components — inline,
        // pr-1 sorted after px-3 and won the right padding — so pr-1 must
        // follow. Running the same-element closure only once, before the
        // cross-element sweep, leaves pr-1 dropped and this test fails.
        use crate::emit::css::{CssIrComponent, IrEl, IrFn};
        use std::collections::{HashMap, HashSet};
        let ir = CssIrComponent {
            name: "test".to_string(),
            components: vec![IrFn {
                fn_: "Test".to_string(),
                export: true,
                elements: vec![
                    IrEl {
                        tag: "div".to_string(),
                        slot: "test-group".to_string(),
                        classes: vec!["inline-flex".to_string()],
                        ..Default::default()
                    },
                    IrEl {
                        tag: "span".to_string(),
                        slot: "test-label".to_string(),
                        classes: vec!["px-3".to_string(), "pr-1".to_string()],
                        ..Default::default()
                    },
                ],
            }],
            ..Default::default()
        };
        let mut index = super::SlotIndex {
            irs: vec![ir],
            cva_slots: vec![HashMap::new()],
            slots: HashMap::new(),
            supply: Vec::new(),
            own_rule: Vec::new(),
        };
        for (slot, el) in [("test-group", 0), ("test-label", 1)] {
            index.slots.insert(
                slot.to_string(),
                vec![super::SlotEntry {
                    ir: 0,
                    fn_: "Test".to_string(),
                    el,
                }],
            );
        }
        let mut sup: HashMap<String, HashSet<String>> = HashMap::new();
        sup.insert(
            "test-group".to_string(),
            ["inline-flex".to_string()].into_iter().collect(),
        );
        sup.insert(
            "test-label".to_string(),
            ["px-3".to_string(), "pr-1".to_string()]
                .into_iter()
                .collect(),
        );
        index.own_rule.push(sup.clone());
        index.supply.push(sup);

        let html = r#"<div><div data-slot="test-group" class="[&_span]:pl-2 inline-flex"><span data-slot="test-label" class="px-3 pr-1"></span></div></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert!(
            out.contains(r#"<span data-slot="test-label" class="px-3 pr-1">"#),
            "pr-1 must follow px-3 back into K: {out}"
        );
        assert!(
            out.contains(r#"class="[&_span]:pl-2""#),
            "inline-flex still drops from the group: {out}"
        );
    }

    #[test]
    fn unit_closure_keeps_a_cross_element_pair_when_both_sides_drop() {
        // The field shape: the parent's `*:w-full` and the label's w-fit
        // are both supplied, so both would drop — and then the two
        // component rules' stylesheet order decides instead of the utility
        // sort that let *:w-full win inline. Both stay. Without the
        // both-dropped arm of settle neither keeps a class and this test
        // fails.
        use crate::emit::css::{CssIrComponent, IrEl, IrFn};
        use std::collections::{HashMap, HashSet};
        let el = |tag: &str, slot: &str, cls: &str| IrEl {
            tag: tag.to_string(),
            slot: slot.to_string(),
            classes: vec![cls.to_string()],
            ..Default::default()
        };
        let ir = CssIrComponent {
            name: "test".to_string(),
            components: vec![IrFn {
                fn_: "Test".to_string(),
                export: true,
                elements: vec![
                    el("div", "test-field", "*:w-full flex"),
                    el("label", "test-label", "w-fit"),
                ],
            }],
            ..Default::default()
        };
        let mut index = super::SlotIndex {
            irs: vec![ir],
            cva_slots: vec![HashMap::new()],
            slots: HashMap::new(),
            supply: Vec::new(),
            own_rule: Vec::new(),
        };
        for (slot, el) in [("test-field", 0), ("test-label", 1)] {
            index.slots.insert(
                slot.to_string(),
                vec![super::SlotEntry {
                    ir: 0,
                    fn_: "Test".to_string(),
                    el,
                }],
            );
        }
        let set = |v: &[&str]| v.iter().map(|t| t.to_string()).collect::<HashSet<String>>();
        let mut sup: HashMap<String, HashSet<String>> = HashMap::new();
        sup.insert("test-field".to_string(), set(&["*:w-full", "flex"]));
        sup.insert("test-label".to_string(), set(&["w-fit"]));
        index.own_rule.push(sup.clone());
        index.supply.push(sup);

        let html = r#"<div><div data-slot="test-field" class="*:w-full flex"><label data-slot="test-label" class="w-fit"></label></div></div>"#;
        let mut m = super::ProjectMetrics::default();
        let out = super::project_body(html, &index, &mut m).expect("projects");
        assert!(
            out.contains(r#"<div data-slot="test-field" class="*:w-full">"#),
            "{out}"
        );
        assert!(
            out.contains(r#"<label data-slot="test-label" class="w-fit">"#),
            "{out}"
        );
    }

    #[test]
    fn unit_at_rest_pre_like_shape() {
        use super::is_at_rest_pre_like;
        assert!(is_at_rest_pre_like("whitespace-pre"));
        assert!(is_at_rest_pre_like("whitespace-pre-line"));
        assert!(is_at_rest_pre_like("whitespace-pre-wrap"));
        assert!(is_at_rest_pre_like("rounded-xl whitespace-pre-line px-3"));
        assert!(is_at_rest_pre_like("[white-space:pre]"));
        assert!(is_at_rest_pre_like("[white-space:pre-line]"));
        // variant-scoped utilities never apply at rest
        assert!(!is_at_rest_pre_like("hover:whitespace-pre-line"));
        // nowrap collapses newlines to spaces exactly like normal
        assert!(!is_at_rest_pre_like("whitespace-nowrap"));
        assert!(!is_at_rest_pre_like("whitespace-normal"));
    }

    #[test]
    fn unit_pre_ws_stamp_strip_roundtrip() {
        use super::{PRE_WS_MARK, parse_body_fragment, stamp_pre_like, strip_pre_like};
        let html = "<div><div data-slot=\"bubble-content\" class=\"rounded-xl whitespace-pre-line px-3\"><div>one.\ntwo.</div><button type=\"button\">more</button></div></div>";
        let (dom, body) = parse_body_fragment(html).expect("parses");
        let _ = &dom; // alive for the whole call
        assert_eq!(stamp_pre_like(&body), 1);
        let stamped = crate::emit::writer::serialize_children(&body);
        assert!(
            stamped.starts_with("<div><pre ") && stamped.contains("data-shadless-tab-pre=\"div\""),
            "the element is formatted as pre with its real tag in the marker: {stamped}"
        );
        // a formatter that changes nothing round-trips byte-for-byte
        let out = strip_pre_like(&body, &stamped).expect("strips");
        assert_eq!(out, html);
        assert!(!out.contains(PRE_WS_MARK) && !out.contains("<pre"), "{out}");
    }

    #[test]
    fn unit_pre_ws_restores_leading_newline() {
        use super::{parse_body_fragment, stamp_pre_like, strip_pre_like};
        // The HTML parser drops the newline immediately after a <pre> start
        // tag; prettier prints one back so ITS output reparses to the tree
        // it got — but the strip reparses with the ORIGINAL tag pending, so
        // the newline must be restored from the stamped tree or the
        // shipped tab loses the demo's leading blank line.
        let html = "<div><p class=\"whitespace-pre-line\">\nfirst\nsecond</p></div>";
        let (dom, body) = parse_body_fragment(html).expect("parses");
        let _ = &dom;
        assert_eq!(stamp_pre_like(&body), 1);
        let stamped = crate::emit::writer::serialize_children(&body);
        // prettier's parse drops it, its printer adds one back after the
        // open tag — simulate exactly that output
        let dropped = stamped.replacen(">\nfirst", ">first", 1);
        assert_ne!(dropped, stamped, "fixture must carry a leading newline");
        let start = dropped.find("<pre").expect("stamped element");
        let open_end = start + dropped[start..].find('>').expect("open tag end") + 1;
        let prettier_out = format!("{}\n{}", &dropped[..open_end], &dropped[open_end..]);
        let out = strip_pre_like(&body, &prettier_out).expect("strips");
        assert_eq!(out, html, "the leading newline must come back");
    }
}
