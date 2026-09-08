//! Port of pipeline/docs_build.go: upstream mdx → Zola markdown (built by the
//! vitezola theme — see docs/site/). The text transform chain, the four MDX
//! shapes, the grey-list cross-check, content-map emission, index + section
//! pages. The ONE node dependency left is prettier's html printer for the
//! demo markup shown under each preview (tools/prettier-batch.mjs — one
//! subprocess per build).

use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

use super::docs_transforms::{
    api_reference_mdx, apply_jsx_overrides, apply_text_adjustments, attr_of, cva_axis_rows,
    drop_react_import_fences, extract_demo_scripts, fence_shadow, fm_string, grey_components,
    guides, locate_api_reference_span, locate_changelog_span, locate_code_tabs_spans,
    locate_composition_span, locate_install_section, locate_message_scroller_js_span,
    locate_rtl_framework_span, locate_rtl_migrate_span, locate_usage_span, message_scroller_js_note,
    parse_frontmatter, protocol_mdx, read_demo_scripts, replace_span, resolve_docs_route,
    rewrite_inline_jsx_mentions, rewrite_leaked_jsx_fences, rewrite_utility_jsx_fences,
    rtl_framework_note, scan_guide_previews, strip_fences, strip_imports, strip_imports_from_mixed_fences,
    trivial_mdx, Guide,
};
use crate::jsonorder::{json_string, Json, JsonObj};

const DOCS_RADIX_DIR: &str = "generated/docs-upstream/components/radix";
const DOCS_ROOT: &str = "docs";
const SITE_ROOT: &str = "docs/site";
const CONTENT_ROOT: &str = "docs/site/content";
const STATIC_ROOT: &str = "docs/site/static";

fn re_fence_split() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)(```.*?```)").unwrap())
}
fn re_inline_code_span() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("`+[^`\n]*`+").unwrap())
}
fn re_jsx_component() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<([A-Z][0-9A-Za-z_]*)").unwrap())
}
fn re_comp_preview_all() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)<ComponentPreview\b.*?/>").unwrap())
}
fn re_comp_source_all() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)<ComponentSource\b.*?/>").unwrap())
}
fn re_steps_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"</?Steps\b[^>]*>").unwrap())
}
fn re_step_block() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)<Step>(.*?)</Step>").unwrap())
}
fn re_kbd_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"</?Kbd>").unwrap())
}
fn re_linked_card() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<LinkedCard\b[^>]*>|</LinkedCard>").unwrap())
}
fn re_class_name() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\bclassName=").unwrap())
}
fn re_md_link() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\[([^\]]*)\]\((/[^)\s]*)\)").unwrap())
}
fn re_jsx_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"([A-Za-z][0-9A-Za-z_-]*)=(?:"([^"]*)"|\{((?:[^{}]|\{[^}]*\})*)\})"#).unwrap()
    })
}
fn re_body_tag() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)<body[^>]*>(.*?)</body>").unwrap())
}
fn re_script_block() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)<script.*?</script>").unwrap())
}
fn re_fm_block() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?s)^---\n.*?\n---\n").unwrap())
}
fn re_api_ref_heading() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^## API Reference[ \t]*\n").unwrap())
}
fn re_init_all() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"shadless\.initAll\(\)").unwrap())
}
fn re_radix_legacy_path() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"https://www\.radix-ui\.com/docs/primitives/").unwrap())
}
fn re_callout_start() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<Callout\b").unwrap())
}
fn re_data_slot_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"data-slot="([a-z0-9-]+)""#).unwrap())
}
fn re_data_slot_set() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"setAttribute\([ \t]*"data-slot"[ \t]*,[ \t]*"([a-z0-9-]+)""#).unwrap()
    })
}

fn inline_code_shadow(text: &str) -> String {
    re_inline_code_span()
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
fn replace_markup(
    text: &str,
    re: &Regex,
    f: impl Fn(&str, &[String]) -> String,
) -> String {
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
    for caps in re_jsx_attr().captures_iter(tag) {
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
    let segs: Vec<&str> = re_fence_split().split(src).collect();
    let fences: Vec<String> = re_fence_split()
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
    let Some(m) = re_callout_start().find(s) else {
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
        let ls: Vec<String> = body
            .split('\n')
            .map(strip_up_to_3_spaces)
            .collect();
        let trimmed = ls.join("\n").trim().to_string();
        // vitezola's tip component; `title` must be listed on a block call
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
/// into vitezola's details component.
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
            let sum = l.trim_start_matches("::: details ").trim().replace('"', "&quot;");
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
    let out = replace_markup(text, re_steps_tag(), |_, _| String::new());
    replace_markup(&out, re_step_block(), |_, m| {
        format!("**{}**", m[0].trim())
    })
}

fn convert_kbd(text: &str) -> String {
    replace_markup(text, re_kbd_tag(), |w, _| {
        if w.starts_with("</") {
            "</kbd>".to_string()
        } else {
            "<kbd>".to_string()
        }
    })
}

fn convert_linked_cards(text: &str) -> String {
    replace_markup(text, re_linked_card(), |w, _| {
        if w.starts_with("</") {
            "</div>".to_string()
        } else {
            r#"<div class="linked-card">"#.to_string()
        }
    })
}

fn convert_class_name(text: &str) -> String {
    replace_markup(text, re_class_name(), |_, _| "class=".to_string())
}

fn rewrite_links(text: &str, site_members: &std::collections::HashSet<String>) -> String {
    let text = re_radix_legacy_path()
        .replace_all(text, "https://www.radix-ui.com/primitives/docs/")
        .into_owned();
    replace_markup(&text, re_md_link(), |whole, m| {
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
    for caps in re_jsx_component().captures_iter(text) {
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
    /// nil (on a missing file or a parse error) is a valid cached result.
    fn component_ir(&mut self, name: &str) -> Option<&crate::emit::css::CssIrComponent> {
        if !self.ir_cache.contains_key(name) {
            let ir = std::fs::read_to_string(format!("generated/ir/{}.json", name))
                .ok()
                .and_then(|b| {
                    // Go json.Unmarshal tolerates `"slot": null`; serde does not.
                    let mut v: serde_json::Value = serde_json::from_str(&b).ok()?;
                    crate::emit::css::drop_nulls(&mut v);
                    serde_json::from_value(v).ok()
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
                for m in re_data_slot_attr().captures_iter(&b) {
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
            for m in re_data_slot_set().captures_iter(&b) {
                out.insert(m[1].to_string(), true);
            }
        }
        self.real_slots = Some(out.clone());
        out
    }

    fn install_steps_mdx(&self, root: &Path, name: &str) -> String {
        let demo = std::fs::read_to_string(root.join("dist/components").join(format!("{}.html", name)))
            .unwrap_or_default();
        let init_all = re_init_all().is_match(&demo);
        let scripts = extract_demo_scripts(&demo);
        let inline_init = !scripts.inline_scripts.is_empty();
        let has_own_css = root.join("dist/css").join(format!("{}.css", name)).exists();
        let mut rows: Vec<String> = vec![
            "| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |".to_string(),
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
        let and_comp = if has_own_css { " and this component" } else { "" };
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
            "<Steps>\n\n<Step>Add shadless{} to your Tailwind v4 entry:</Step>\n\n```css\n@import \"shadless\";{}\n```\n{}\nThe files this component needs:\n\n| File | Purpose |\n| --- | --- |\n{}\n{}\n\n<Step>{} from any example on this page (the code tab under its preview) into your page and adapt it{} — the inline utilities are picked up by your build's content scan.</Step>\n{}{}\nNo Tailwind build? Use the precompiled `dist/shadless.full.min.css` (npm: `shadless/full.min.css`, every component) as a single stylesheet instead of the imports above.\n\n</Steps>",
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
        let Some(m) = re_api_ref_heading().find(raw) else {
            return raw.to_string();
        };
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
        let at = m.end();
        format!("{}\n{}{}", &raw[..at], extra, &raw[at..])
    }

    fn component_transform(&mut self, root: &Path, name: &str, raw: &str) -> Result<String, String> {
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
        out = self.api_reference_transform(root, name, &out, &mut seen);
        if let Some(s) = locate_changelog_span(&fence_shadow(&out)) {
            out = replace_span(&out, s, "");
        }
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
            return Err("utils Installation section: not found (or no following ## Usage)".to_string());
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
    // vitezola's demo component (docs/site/templates/demo.html): the card
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

fn demo_source(ctx: &DocsBuildCtx, file: &str) -> String {
    let path = format!("{}/demos/{}", STATIC_ROOT, file);
    let Some(markup) = ctx.markup.get(file) else {
        return String::new();
    };
    let scripts = read_demo_scripts(std::path::Path::new(&path));
    let mut js: Vec<String> = Vec::new();
    for s in &scripts.src_scripts {
        if s == "shadless.js" {
            js.push("// <script src=\"shadless.js\"></script>  — the shared runtime (see Installation)".to_string());
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
    // vitezola's codegroup: one fenced block per tab, the tab label from the
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
    body = re_fm_block().replace_all(&body, "").into_owned();
    body = strip_imports_outside_fences(&body);
    body = replace_markup(&body, re_comp_source_all(), |_, _| String::new());
    body = convert_callouts(&body, name)?;
    body = convert_details(&body, name)?;
    body = convert_steps(&body);
    body = convert_kbd(&body);
    body = convert_linked_cards(&body);
    body = convert_class_name(&body);
    let site_members = ctx.site_members();
    body = rewrite_links(&body, &site_members);
    body = replace_markup(&body, re_comp_preview_all(), |whole, _| {
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
    // (themes/vitezola), the site shell (config.toml, templates/, sass/) and
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
            let Ok(raw) = std::fs::read_to_string(root.join(STATIC_ROOT).join("demos").join(&n)) else {
                continue;
            };
            let mut body = raw.clone();
            if let Some(m) = re_body_tag().captures(&raw) {
                body = m[1].to_string();
            }
            body = re_script_block().replace_all(&body, "").trim().to_string();
            items.push((n, body));
        }
    }
    let markup = match prettier_batch(&items) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("docs-build: prettier: {}", e);
            return 1;
        }
    };

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
                |ctx, root, raw| guide_transform(root, g, raw),
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
    // the hero home — the theme's index.html renders vitepress_home and
    // nothing else, so the component listing lives on /components/ above
    // TOML frontmatter: the vitepress_home tables are TOML, and a YAML block
    // cannot carry them (the pages keep YAML — Zola accepts either per file)
    let home = format!(
        "+++\ntemplate = \"index.html\"\n[extra.vitepress_home]\ntext = \"shadless\"\ntagline = \"shadcn/ui as static HTML and a vanilla runtime — no React. Copy the markup, load the stylesheets, done.\"\n[[extra.vitepress_home.actions]]\ntext = \"Get Started\"\ntheme = \"brand\"\nlink = \"/guides/introduction/\"\n\n[[extra.vitepress_home.actions]]\ntext = \"Components\"\ntheme = \"alt\"\nlink = \"/components/\"\n\n[[extra.vitepress_home.features]]\ntitle = \"Static HTML\"\ndetails = \"Every component is markup + CSS you copy — data-slot attributes, Tailwind utilities, no framework.\"\n\n[[extra.vitepress_home.features]]\ntitle = \"Vanilla runtime\"\ndetails = \"A dependency-free JS base wires the interactive pieces; shadless.init(root) for content added later.\"\n\n[[extra.vitepress_home.features]]\ntitle = \"shadcn/ui, ported\"\ndetails = \"{} components mirrored from the pinned upstream registry, plus the guides that still apply.\"\n+++\n",
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
    let catalog_b = std::fs::read_to_string(root.join("docs/catalog.json"))
        .map_err(|e| e.to_string())?;
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
            ("disposition".to_string(), Json::Str(g.disposition.to_string())),
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
            ("disposition".to_string(), Json::Str("generated".to_string())),
            (
                "notes".to_string(),
                Json::Str("components + guides index page".to_string()),
            ),
        ]),
    ));

    // pruned in the recorded order
    let pkeys = ["forms", "react", "registry", "changelog", "(root)", "helpers", "framework sub-pages"];
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
        .add("generatedBy", Json::Str("tools/docs-guides.mjs (FT4)".to_string()))
        .add("pages", Json::Obj(pages_kv))
        .add("pruned", Json::Obj(pruned_kv))
        .add("guidePreviews", Json::Obj(gp_kv));
    std::fs::create_dir_all(root.join("docs")).map_err(|e| e.to_string())?;
    std::fs::write(
        root.join("docs/content-map.json"),
        format!("{}\n", crate::jsonorder::marshal_js(&Json::from_obj(root_obj))),
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
