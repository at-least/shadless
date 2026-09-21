//! Port of the emit chain: pipeline/{emit,emitter_css,emitter_html,prepaint,
//! default_content,tags}.go — IR (tier=static) → semantic usage HTML +
//! slot-keyed CSS, with the static gates.

pub mod build_rtl;
pub mod css;
pub mod demo;
pub mod product_css;
pub mod tw;

use std::collections::{HashMap, HashSet};

static SKIN: OnceLock<SkinData> = OnceLock::new();
use regex::Regex;
use std::sync::{LazyLock, OnceLock};
pub mod default_content;
pub mod htmlutil;
pub mod prepaint;
pub mod tags;

// ------------------------------------------------------- tree build/render

/// The resolved element tree buildTree produces.
#[derive(Clone, Debug, Default)]
pub struct TreeNode {
    pub tag: String,
    pub slot: String,
    pub anchor: String,
    pub anchor_m: Vec<String>,
    pub kids: Vec<TreeNode>,
}

static RE_SKETCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^<([^ >]+)(?: slot=([^ >]+))?").unwrap());

struct BuildTreeOpts {
    claimed: HashSet<usize>,
    anchors: HashMap<String, String>,
    anchor_markers: HashMap<String, Vec<String>>,
}

/// Resolves a fn's element tree from the flat walk-order elements + sketches.
/// Every tag (root AND children) goes through normalize_tag.
use crate::twmerge;
use css::{CssIrComponent, IrEl, IrFn, component_css, dedup, split_markers, wrap_component_css};
use markup5ever_rcdom::{Handle, NodeData};
use tags::void_tags;
use tags::{native_tags, normalize_tag};

fn build_tree(ir: &CssIrComponent, f: &IrFn, o: &mut BuildTreeOpts) -> Result<TreeNode, String> {
    fn resolve(
        ir: &CssIrComponent,
        f: &IrFn,
        o: &mut BuildTreeOpts,
        el: &IrEl,
        i: usize,
    ) -> Result<TreeNode, String> {
        let mut kids: Vec<TreeNode> = Vec::new();
        for sk in &el.children {
            let Some(m) = RE_SKETCH.captures(sk) else {
                continue; // text/{children}/OPT?/expr → nothing structural
            };
            let raw_tag = &m[1];
            let sketch_slot = m.get(2).map(|s| s.as_str()).unwrap_or("");
            // raw-tag match on both sides (sketch carries the raw JSX name)
            let mut hit: Option<usize> = None;
            for (idx, cand) in f.elements.iter().enumerate() {
                if o.claimed.contains(&idx) {
                    continue;
                }
                if !sketch_slot.is_empty() {
                    if cand.slot == sketch_slot && cand.tag == raw_tag {
                        hit = Some(idx);
                        break;
                    }
                } else if cand.tag == raw_tag {
                    hit = Some(idx);
                    break;
                }
            }
            if let Some(hit_i) = hit {
                o.claimed.insert(hit_i);
                let kid = resolve(ir, f, o, &f.elements[hit_i], hit_i)?;
                kids.push(kid);
                continue;
            }
            // unresolvable sketch: icon → svg; native → bare; else skip
            let Some(tag) = normalize_tag(raw_tag, &ir.tag_hints) else {
                continue_with_fallback(raw_tag, &mut kids, sketch_slot);
                continue;
            };
            let _ = tag;
        }
        let Some(tag) = normalize_tag(&el.tag, &ir.tag_hints) else {
            return Err(format!(
                "[{}] unresolvable tag in {}: {}",
                ir.name, f.fn_, el.tag
            ));
        };
        Ok(TreeNode {
            tag,
            slot: el.slot.clone(),
            anchor: o
                .anchors
                .get(&format!("{}#{}", f.fn_, i))
                .cloned()
                .unwrap_or_default(),
            anchor_m: o
                .anchor_markers
                .get(&format!("{}#{}", f.fn_, i))
                .cloned()
                .unwrap_or_default(),
            kids,
        })
    }
    // local helper mirroring Go's native-tags bare-kid branch
    fn continue_with_fallback(raw_tag: &str, kids: &mut Vec<TreeNode>, sketch_slot: &str) {
        if native_tags().contains(raw_tag) {
            kids.push(TreeNode {
                tag: raw_tag.to_string(),
                slot: sketch_slot.to_string(),
                ..Default::default()
            });
        }
    }
    let _ = &continue_with_fallback;
    if f.elements.is_empty() {
        return Err(format!("[{}] fn {} has no elements", ir.name, f.fn_));
    }
    o.claimed.insert(0);
    resolve(ir, f, o, &f.elements[0], 0)
}

/// Renders an element tree. defaultInner REPLACES the root's content;
/// defaultBySlot fills EMPTY leaves.
fn render_tree(
    node: &TreeNode,
    markers: &HashMap<String, Vec<String>>,
    default_inner: &str,
    default_by_slot: &HashMap<String, String>,
    is_root: bool,
) -> String {
    let mut classes: Vec<String> = Vec::new();
    if !node.slot.is_empty() {
        if let Some(ms) = markers.get(&node.slot) {
            if !ms.is_empty() {
                classes.extend(dedup(ms));
            }
        }
    }
    if !node.anchor.is_empty() {
        classes.push(node.anchor.clone());
    }
    classes.extend(node.anchor_m.iter().cloned());
    let cls = if classes.is_empty() {
        String::new()
    } else {
        format!(" class=\"{}\"", classes.join(" "))
    };
    let slot = if node.slot.is_empty() {
        String::new()
    } else {
        format!(" data-slot=\"{}\"", node.slot)
    };
    let open = format!("<{}{}{}>", node.tag, slot, cls);
    if void_tags().contains(node.tag.as_str()) {
        return open;
    }
    let inner = if is_root && !default_inner.is_empty() {
        default_inner.to_string()
    } else if !node.kids.is_empty() {
        node.kids
            .iter()
            .map(|k| render_tree(k, markers, "", default_by_slot, false))
            .collect::<Vec<_>>()
            .join("")
    } else {
        default_by_slot.get(&node.slot).cloned().unwrap_or_default()
    };
    format!("{}{}</{}>", open, inner, node.tag)
}

/// Stray table-parts get dropped by HTML parsers at body level — wrap
/// ancestors.
fn table_wrap(tag: &str) -> Option<&'static str> {
    match tag {
        "thead" | "tbody" | "tfoot" | "caption" | "colgroup" | "tr" | "th" | "td" => Some("table"),
        _ => None,
    }
}

fn render_fn(
    tree: &TreeNode,
    markers: &HashMap<String, Vec<String>>,
    default_inner: &str,
    default_by_slot: &HashMap<String, String>,
) -> String {
    let mut h = render_tree(tree, markers, default_inner, default_by_slot, true);
    let mut tag = tree.tag.clone();
    while let Some(w) = table_wrap(&tag) {
        h = format!("<{}>{}</{}>", w, h, w);
        tag = w.to_string();
    }
    h
}

/// string → escaped text; {Inner,Attrs,Children} composed.
fn resolve_default(
    ir: &CssIrComponent,
    f: &IrFn,
) -> Option<(String, Vec<(String, String)>, HashMap<String, String>)> {
    let content = default_content::default_content();
    let comp = content.get(ir.name.as_str())?;
    let entry = comp.get(f.fn_.as_str())?;
    if !entry.set && entry.inner.is_empty() && entry.attrs.is_empty() && entry.children.is_empty() {
        // present-and-null → explicitly no default
        return None;
    }
    let has = !entry.inner.is_empty() || !entry.attrs.is_empty() || !entry.children.is_empty();
    if !has {
        return None;
    }
    Some((
        entry.inner.clone(),
        entry.attrs.clone(),
        entry.children.iter().cloned().collect(),
    ))
}

/// Applies extra attrs to the root open tag — quote-aware scan for the end of
/// the first open tag.
fn merge_root_attrs(h: &str, attrs: &[(String, String)]) -> String {
    if attrs.is_empty() {
        return h.to_string();
    }
    let parts: Vec<String> = attrs
        .iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, htmlutil::esc_html(v)))
        .collect();
    let extra = parts.join(" ");
    static TAG_RE: OnceLock<Regex> = OnceLock::new();
    let re = TAG_RE.get_or_init(|| Regex::new(r"^<([a-zA-Z][A-Za-z0-9_-]*)").unwrap());
    let Some(m) = re.captures(h) else {
        return h.to_string();
    };
    let mut i = m.get(0).unwrap().end();
    let b = h.as_bytes();
    let mut quote = 0u8;
    while i < b.len() {
        let ch = b[i];
        if quote != 0 {
            if ch == quote {
                quote = 0;
            }
        } else if ch == b'"' || ch == b'\'' {
            quote = ch;
        } else if ch == b'>' {
            break;
        }
        i += 1;
    }
    format!("{} {}{}", &h[..i], extra, &h[i..])
}

/// Stale keys previously survived silently.
fn validate_default_content(statics: &[CssIrComponent]) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();
    let names: HashSet<&str> = statics.iter().map(|ir| ir.name.as_str()).collect();
    let content = default_content::default_content();
    let mut comps: Vec<&str> = content.keys().copied().collect();
    comps.sort_unstable();
    for comp in comps {
        if !names.contains(comp) {
            errs.push(format!("unknown component key: {}", comp));
            continue;
        }
        let ir = statics.iter().find(|ir| ir.name == comp).unwrap();
        let mut fns: Vec<&str> = content[comp].keys().copied().collect();
        fns.sort_unstable();
        for f in fns {
            let exported = ir.components.iter().any(|c| c.export && c.fn_ == f);
            if !exported {
                errs.push(format!("[{}] unknown fn key: {}", comp, f));
            }
        }
    }
    errs
}

// -------------------------------------------------- skin (from resolve_skins)

pub struct SkinData {
    pub map: HashMap<String, String>,
    pub allowlist: HashSet<String>,
}

pub fn skin_data() -> &'static SkinData {
    load_skin();
    SKIN.get().expect("load_skin initialized skin data")
}

const SKIN_PATH: &str = ".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css";

/// Parses style-nova.css into skinData: a plain flat-block scan identical to
/// skin.mjs's parseSkinMap; anything other than flat pure-@apply blocks fails
/// loudly.
pub fn load_skin() {
    static ONCE: OnceLock<()> = OnceLock::new();
    if ONCE.set(()).is_ok() {
        // the pipeline runs from the repo root; tests run from the crate —
        // fall back to the adjacent product tree so the pinned skin is
        // reachable from both (same bytes either way; errors still report
        // the Go-parity relative path)
        let skin = match std::path::Path::new(SKIN_PATH).exists() {
            true => std::path::PathBuf::from(SKIN_PATH),
            false => crate::crate_adjacent_tree_root()
                .map(|r| r.join(SKIN_PATH))
                .unwrap_or_else(|| std::path::PathBuf::from(SKIN_PATH)),
        };
        let b = match std::fs::read_to_string(&skin) {
            Ok(b) => b,
            Err(e) => {
                // Go os.ReadFile: open-phase errors report "open"; a directory
                // opens fine on Linux and fails in read with EISDIR.
                let op = if e.raw_os_error() == Some(21) {
                    "read"
                } else {
                    "open"
                };
                eprintln!("resolve-skins: skin: {}", go_err(op, SKIN_PATH, &e));
                std::process::exit(1);
            }
        };
        let mut map: HashMap<String, String> = HashMap::new();
        let mut allowlist: HashSet<String> = HashSet::new();
        for t in [
            "cn-menu-target",
            "cn-menu-translucent",
            "cn-rtl-flip",
            "cn-font-heading",
        ] {
            allowlist.insert(t.to_string());
        }
        parse_skin_map(&b, &mut map);
        SKIN.set(SkinData {
            map: map.clone(),
            allowlist,
        })
        .ok();
        let _ = map;
    }
}

/// Renders an io::Error the way Go's *PathError does: `open <path>: <strerror>`
/// (Go's own lowercase strerror table, not libc's). The shared entries match
/// the three go_err copies in tools/ (oracle_css, docs_upstream_mirror,
/// upstream_snapshot) verbatim; the rest of Go's table
/// (syscall/zerrors_linux_amd64.go) is added here because the skin path is
/// user-environment-reachable (ELOOP via a symlinked checkout, ENOSPC/EROFS
/// on a full disk) and the fallback must keep the `op <path>:` prefix Go
/// never drops. op comes from errno, not phase: EISDIR is the only in-table
/// errno Go can raise during Read — an EIO-in-read would print "open" here.
fn go_err(op: &str, path: &str, e: &std::io::Error) -> String {
    let msg = match e.raw_os_error() {
        Some(1) => "operation not permitted",
        Some(2) => "no such file or directory",
        Some(5) => "input/output error",
        Some(6) => "no such device or address",
        Some(12) => "cannot allocate memory",
        Some(13) => "permission denied",
        Some(17) => "file exists",
        Some(20) => "not a directory",
        Some(21) => "is a directory",
        Some(22) => "invalid argument",
        Some(24) => "too many open files",
        Some(26) => "text file busy",
        Some(28) => "no space left on device",
        Some(30) => "read-only file system",
        Some(36) => "file name too long",
        Some(39) => "directory not empty",
        Some(40) => "too many levels of symbolic links",
        Some(75) => "value too large for defined data type",
        _ => return format!("{} {}: {}", op, path, e),
    };
    format!("{} {}: {}", op, path, msg)
}

fn parse_skin_map(css: &str, map: &mut HashMap<String, String>) {
    static SKIN_BLOCK: OnceLock<Regex> = OnceLock::new();
    static STYLE_NOVA: OnceLock<Regex> = OnceLock::new();
    static APPLY_STMT: OnceLock<Regex> = OnceLock::new();
    let skin_block =
        SKIN_BLOCK.get_or_init(|| Regex::new(r"(?s)\.([\w-]+)\s*\{([^{}]*)\}").unwrap());
    let style_nova = STYLE_NOVA.get_or_init(|| Regex::new(r"^\s*\.style-nova\s*\{").unwrap());
    let apply_stmt = APPLY_STMT.get_or_init(|| Regex::new(r"^\s*@apply\s+([^;]+);\s*$").unwrap());
    let start = css.find('{');
    let end = css.rfind('}');
    if start.is_none() || end.is_none() || !style_nova.is_match(css) {
        eprintln!("skin: expected a single top-level .style-nova block");
        std::process::exit(1);
    }
    let body = &css[start.expect("checked") + 1..end.expect("checked")];
    for m in skin_block.captures_iter(body) {
        let name = &m[1];
        let decls = m[2].trim();
        let Some(sub) = apply_stmt.captures(decls) else {
            eprintln!(
                "skin: cn-{} is not a flat pure-@apply block: {:.60}",
                name, decls
            );
            std::process::exit(1);
        };
        map.insert(name.to_string(), sub[1].to_string());
    }
    if map.is_empty() {
        eprintln!("skin: no cn-* blocks found");
        std::process::exit(1);
    }
}

// ------------------------------------------------------------------ runEmit

pub fn run_emit() -> Result<(), String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    load_skin();
    std::fs::create_dir_all(root.join("dist/components")).map_err(|e| format!("emit: {}", e))?;
    std::fs::create_dir_all(root.join("build/emit")).map_err(|e| format!("emit: {}", e))?;

    // statics in sorted-filename order (os.ReadDir sorts)
    let mut statics: Vec<CssIrComponent> = Vec::new();
    let ir_dir = root.join("generated/ir");
    let names: Vec<String> =
        crate::fsutil::sorted_read_dir(&ir_dir).map_err(|e| format!("emit: {}", e))?;
    for n in &names {
        if !n.ends_with(".json") {
            continue;
        }
        let b = std::fs::read_to_string(ir_dir.join(n)).map_err(|e| e.to_string())?;
        let mut v: serde_json::Value =
            serde_json::from_str(&b).map_err(|e| format!("emit: ir: {} {}", n, e))?;
        css::drop_nulls(&mut v);
        let ir: CssIrComponent =
            serde_json::from_value(v).map_err(|e| format!("emit: ir: {} {}", n, e))?;
        if ir.tier == "static" {
            statics.push(ir);
        }
    }
    let tiers_b = std::fs::read_to_string(root.join("src/registry/tiers.json")).unwrap_or_default();
    #[derive(serde::Deserialize)]
    struct TierEntry {
        #[serde(default)]
        tier: String,
    }
    let tiers: HashMap<String, TierEntry> = serde_json::from_str(&tiers_b).unwrap_or_default();
    let want_static = tiers.values().filter(|t| t.tier == "static").count();
    let mut fail = false;
    if statics.len() != want_static {
        eprintln!(
            "FAIL expected {} static (from tiers.json), got {}",
            want_static,
            statics.len()
        );
        return Err("static count mismatch".to_string());
    }
    for e in validate_default_content(&statics) {
        eprintln!("FAIL defaults: {}", e);
        fail = true;
    }

    let mut css_parts: Vec<String> = Vec::new();
    let mut all_anchors: HashSet<String> = HashSet::new();
    struct IrTrees {
        ir: CssIrComponent,
        trees: Vec<TreeNode>,
    }
    let mut trees_by_ir: Vec<IrTrees> = Vec::new();
    let mut pages: HashMap<String, String> = HashMap::new();
    let mut total_slots = 0usize;
    for ir in &statics {
        let css = match component_css(ir) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("FAIL css[{}]: {}", ir.name, e);
                fail = true;
                continue;
            }
        };
        for t in css.anchors.values() {
            all_anchors.insert(t.clone());
        }
        let mut trees: Vec<TreeNode> = Vec::new();
        let mut bodies: Vec<String> = Vec::new();
        for c in &ir.components {
            if !c.export {
                continue;
            }
            let tree = match build_tree(
                ir,
                c,
                &mut BuildTreeOpts {
                    claimed: HashSet::new(),
                    anchors: css.anchors.clone(),
                    anchor_markers: css.anchor_markers.clone(),
                },
            ) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("FAIL emit[{}.{}]: {}", ir.name, c.fn_, e);
                    fail = true;
                    bodies.push(String::new());
                    continue;
                }
            };
            let def = resolve_default(ir, c);
            let (inner, attrs, children) = match def {
                Some((i, a, ch)) => (i, a, ch),
                None => (String::new(), Vec::new(), HashMap::new()),
            };
            let mut h = render_fn(&tree, &css.markers, &inner, &children);
            trees.push(tree);
            if !attrs.is_empty() {
                h = merge_root_attrs(&h, &attrs);
            }
            bodies.push(h);
        }
        trees_by_ir.push(IrTrees {
            ir: ir.clone(),
            trees,
        });
        let page = format!(
            "<!doctype html>\n<html><head><meta charset=\"utf-8\"><title>shadless {}</title>\n<link rel=\"stylesheet\" href=\"../out.css\">{}</head>\n<body>\n{}\n</body></html>",
            ir.name,
            prepaint::THEME_PREPAINT_SCRIPT,
            bodies.join("\n")
        );
        std::fs::write(
            root.join(format!("dist/components/{}.html", ir.name)),
            &page,
        )
        .map_err(|e| format!("emit: {}", e))?;
        pages.insert(ir.name.clone(), page.clone());
        css_parts.push(wrap_component_css(&ir.name, &css));
        total_slots += page.matches("data-slot=\"").count();
    }
    std::fs::write(root.join("dist/shadless.css"), css_parts.join("\n\n"))
        .map_err(|e| format!("emit: {}", e))?;

    // CSS completeness gate: every class TOKEN must appear in some emitted
    // rule (token-level; same-slot complementary elements legitimately split
    // tokens across rules)
    {
        let all_css = css_parts.join("\n");
        for ir in &statics {
            for c in &ir.components {
                for el in &c.elements {
                    let apply_toks: Vec<String> =
                        el.classes.iter().map(|x| split_markers(x).apply).collect();
                    let kept: HashSet<String> = twmerge::merge(&apply_toks.join(" "))
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    for cs in &el.classes {
                        let applied = split_markers(cs).apply;
                        let missing: Vec<&str> = applied
                            .split_whitespace()
                            .filter(|t| {
                                !t.is_empty()
                                    && kept.contains(*t)
                                    && !css_contains_token(&all_css, t)
                            })
                            .collect();
                        if !missing.is_empty() {
                            let n = missing.len().min(6);
                            eprintln!(
                                "FAIL css[{}]: class tokens not in CSS: {:?}…",
                                ir.name,
                                &missing[..n]
                            );
                            fail = true;
                        }
                    }
                }
            }
        }
    }

    // globals (build/emit, NOT dist — the demo chain owns dist/globals.css)
    {
        let gb = std::fs::read_to_string(root.join("probes/h4/globals.css"))
            .map_err(|e| format!("emit: {}", e))?;
        let g = gb.replacen("@source \"./demo.html\";\n", "", 1);
        let out = format!(
            "{}\n{}\n{}",
            g,
            prepaint::SHADLESS_CSS_FIXES,
            css_parts.join("\n\n")
        );
        std::fs::write(root.join("build/emit/globals.css"), out)
            .map_err(|e| format!("emit: {}", e))?;
        let mut li = String::from(
            "<!doctype html><html><head><meta charset=\"utf-8\">\n<link rel=\"stylesheet\" href=\"out.css\"></head><body>\n<ul>",
        );
        for ir in &statics {
            li.push_str(&format!(
                "<li><a href=\"components/{}.html\">{}</a></li>",
                ir.name, ir.name
            ));
        }
        li.push_str("</ul>\n</body></html>");
        std::fs::write(root.join("build/emit/demo-index.html"), li)
            .map_err(|e| format!("emit: {}", e))?;
    }
    println!(
        "emit: {} pages, {} slots, shadless.css",
        statics.len(),
        total_slots
    );

    // ---- gates ----
    static RE_CLASS: OnceLock<Regex> = OnceLock::new();
    let re_class = RE_CLASS.get_or_init(|| Regex::new(r#"class="([^"]*)""#).unwrap());
    static RE_PASCAL: OnceLock<Regex> = OnceLock::new();
    let re_pascal = RE_PASCAL.get_or_init(|| {
        Regex::new(r"</?([A-Z][A-Za-z0-9]*(?:\.[A-Z][A-Za-z0-9]*)?)[\s>]").unwrap()
    });

    // gate: no class= beyond markers/anchors/allowlist
    for ir in &statics {
        let h = &pages[&ir.name];
        for m in re_class.captures_iter(h) {
            let mut bad = false;
            for t in m[1].split_whitespace() {
                if t.is_empty()
                    || css::MARKER_RE.is_match(t)
                    || skin_data().allowlist.contains(t)
                    || all_anchors.contains(t)
                {
                    continue;
                }
                bad = true;
                break;
            }
            if bad {
                eprintln!("FAIL [{}]: non-anchor class= in HTML: {}", ir.name, &m[1]);
                fail = true;
            }
        }
    }

    // gate: literal PascalCase / ternary tags
    for ir in &statics {
        let h = &pages[&ir.name];
        if let Some(m) = re_pascal.captures(h) {
            eprintln!(
                "FAIL [{}]: literal component tag in HTML: {}",
                ir.name, &m[1]
            );
            fail = true;
        }
    }

    // gate: slot-tree vs IR (exact tags + nesting) — the HTML5 parse is the
    // jsdom twin: same lowercased tags, same foster-parenting rules.
    use markup5ever_rcdom::{Handle, NodeData, RcDom};
    fn slot_of(e: &Handle) -> String {
        if let NodeData::Element { attrs, .. } = &e.data {
            for a in attrs.borrow().iter() {
                if &*a.name.local == "data-slot" {
                    return a.value.to_string();
                }
            }
        }
        String::new()
    }
    fn walk_slotted(n: &Handle, parent_slot: &str, visit: &mut impl FnMut(&Handle, &str)) {
        for c in n.children.borrow().iter() {
            let mut ps = parent_slot.to_string();
            if let NodeData::Element { .. } = &c.data {
                let s = slot_of(c);
                if !s.is_empty() {
                    visit(c, parent_slot);
                    ps = s;
                }
            }
            walk_slotted(c, &ps, visit);
        }
    }
    fn parse_html(h: &str) -> Result<RcDom, String> {
        use html5ever::parse_document;
        use html5ever::tendril::TendrilSink;
        let opts = html5ever::ParseOpts::default();
        let dom = parse_document(RcDom::default(), opts).one(h);
        Ok(dom)
    }
    for it in &trees_by_ir {
        let h = &pages[&it.ir.name];
        let doc = match parse_html(h) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("FAIL [{}]: parse: {}", it.ir.name, e);
                fail = true;
                continue;
            }
        };
        let mut tree_pairs: HashSet<String> = HashSet::new();
        let mut tree_edges: HashSet<String> = HashSet::new();
        fn collect(
            n: &TreeNode,
            parent_slot: &str,
            pairs: &mut HashSet<String>,
            edges: &mut HashSet<String>,
        ) {
            if !n.slot.is_empty() {
                pairs.insert(format!("{}@{}", n.tag, n.slot));
                if !parent_slot.is_empty() {
                    edges.insert(format!("{}>{}", parent_slot, n.slot));
                }
            }
            let ps = if !n.slot.is_empty() {
                &n.slot
            } else {
                parent_slot
            };
            for k in &n.kids {
                collect(k, ps, pairs, edges);
            }
        }
        for t in &it.trees {
            collect(t, "", &mut tree_pairs, &mut tree_edges);
        }

        // DEFAULT_CONTENT html chunks are sanctioned sources of pairs+edges
        let mut def_pairs: HashSet<String> = HashSet::new();
        let mut def_edges: HashSet<String> = HashSet::new();
        for c in &it.ir.components {
            if !c.export {
                continue;
            }
            let Some((inner, _, _)) = resolve_default(&it.ir, c) else {
                continue;
            };
            if inner.is_empty() {
                continue;
            }
            let Some(root_el) = c.elements.first() else {
                continue;
            };
            let root_tag =
                normalize_tag(&root_el.tag, &it.ir.tag_hints).unwrap_or_else(|| "div".to_string());
            let scope = if table_wrap(&root_tag).is_some() {
                "<table>"
            } else {
                ""
            };
            let mut wrap_open = scope.to_string();
            if !root_el.slot.is_empty() {
                wrap_open.push_str(&format!("<{} data-slot=\"{}\">", root_tag, root_el.slot));
            } else {
                wrap_open.push_str(&format!("<{}>", root_tag));
            }
            let close_tag = format!(
                "</{}>{}",
                root_tag,
                if scope.is_empty() { "" } else { "</table>" }
            );
            if let Ok(frag) = parse_html(&format!("{}{}{}", wrap_open, inner, close_tag)) {
                walk_slotted(&frag.document, "", &mut |e: &Handle, parent_slot: &str| {
                    def_pairs.insert(format!("{}@{}", node_name(e), slot_of(e)));
                    if !parent_slot.is_empty() {
                        def_edges.insert(format!("{}>{}", parent_slot, slot_of(e)));
                    }
                });
            }
        }

        // IR-side pairs
        let mut ir_pairs: HashSet<String> = HashSet::new();
        for c in &it.ir.components {
            for el in &c.elements {
                if el.slot.is_empty() {
                    continue;
                }
                let tag =
                    normalize_tag(&el.tag, &it.ir.tag_hints).unwrap_or_else(|| "?".to_string());
                ir_pairs.insert(format!("{}@{}", tag, el.slot));
            }
        }

        let mut dom_nodes: Vec<String> = Vec::new();
        let mut dom_set: HashSet<String> = HashSet::new();
        let mut dom_edges: HashSet<String> = HashSet::new();
        walk_slotted(&doc.document, "", &mut |e: &Handle, parent_slot: &str| {
            let p = format!("{}@{}", node_name(e), slot_of(e));
            dom_nodes.push(p.clone());
            dom_set.insert(p);
            if !parent_slot.is_empty() {
                dom_edges.insert(format!("{}>{}", parent_slot, slot_of(e)));
            }
        });

        let mut sanctioned_pairs: HashSet<String> = HashSet::new();
        for k in tree_pairs.iter().cloned() {
            sanctioned_pairs.insert(k);
        }
        for k in ir_pairs.iter().cloned() {
            sanctioned_pairs.insert(k);
        }
        for k in def_pairs.iter().cloned() {
            sanctioned_pairs.insert(k);
        }
        let mut sanctioned_edges: HashSet<String> = HashSet::new();
        for k in tree_edges.iter().cloned() {
            sanctioned_edges.insert(k);
        }
        for k in def_edges.iter().cloned() {
            sanctioned_edges.insert(k);
        }

        // IR slot missing in DOM: any dom pair with the same slot suffix
        for p in ir_pairs.union(&tree_pairs) {
            let slot = &p[p.find('@').unwrap_or(0) + 1..];
            let found = dom_set.iter().any(|d| d.ends_with(&format!("@{}", slot)));
            if !found {
                eprintln!("FAIL [{}]: IR slot missing in DOM: {}", it.ir.name, p);
                fail = true;
            }
        }
        for d in &dom_nodes {
            if !sanctioned_pairs.contains(d) {
                eprintln!(
                    "FAIL [{}]: DOM slot not sanctioned (tree/IR/default): {}",
                    it.ir.name, d
                );
                fail = true;
            }
        }
        for e in &dom_edges {
            if !sanctioned_edges.contains(e) {
                eprintln!("FAIL [{}]: DOM nesting not sanctioned: {}", it.ir.name, e);
                fail = true;
            }
        }
        if (!ir_pairs.is_empty() || !tree_pairs.is_empty()) && dom_nodes.is_empty() {
            eprintln!("FAIL [{}]: no slots rendered", it.ir.name);
            fail = true;
        }
    }

    if fail {
        println!("FAIL  emit gates");
        return Err("emit gates failed".to_string());
    }
    println!(
        "PASS  emit static gates ({} files, 0 class=, exact slot-tree, {} anchors)",
        statics.len(),
        all_anchors.len()
    );
    Ok(())
}

/// Token-boundary containment for CSS class tokens: `p-2` must not match
/// inside `gap-2` or `p-2.5`. A class token is `[\w-]+`, so an occurrence
/// is a real token only when neither neighbour is `[\w-]` and the follower
/// is neither `.` nor `\` (compiled CSS writes the escaped decimal
/// `.p-2\.5`); a variant prefix (`hover:p-2`) still counts — the variant
/// colon is not a word character. The completeness gates on both emit twins
/// check every `@apply` token this way: a substring check let a dropped
/// rule pass because some longer token elsewhere happened to contain it.
pub(crate) fn css_contains_token(hay: &str, tok: &str) -> bool {
    if tok.is_empty() {
        return false;
    }
    let bad = |c: u8| c.is_ascii_alphanumeric() || c == b'_' || c == b'-';
    let b = hay.as_bytes();
    let mut from = 0;
    while let Some(pos) = hay[from..].find(tok) {
        let start = from + pos;
        let end = start + tok.len();
        let before_ok = start == 0 || !bad(b[start - 1]);
        let after_ok = end >= b.len() || (!bad(b[end]) && b[end] != b'.' && b[end] != b'\\');
        if before_ok && after_ok {
            return true;
        }
        from = start + 1;
    }
    false
}

fn node_name(e: &Handle) -> String {
    match &e.data {
        NodeData::Element { name, .. } => name.local.to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Characterization for the Go-shaped PathError text the error_paths
    /// integration tests pin: they can only execute from an unpinned adjacent
    /// tree (see their skip), so the contract lives here too, at the
    /// formatting function itself.
    #[test]
    fn unit_go_err_shapes_go_path_errors() {
        let skin = ".upstream/shadcn-ui/apps/v4/registry/styles/style-nova.css";
        let enoent = std::io::Error::from_raw_os_error(2);
        assert_eq!(
            go_err("open", skin, &enoent),
            format!("open {}: no such file or directory", skin)
        );
        let eisdir = std::io::Error::from_raw_os_error(21);
        assert_eq!(
            go_err("read", skin, &eisdir),
            format!("read {}: is a directory", skin)
        );
    }

    /// The substring check this replaced matched `p-2` inside `gap-2`, so a
    /// dropped rule passed whenever a longer token elsewhere contained it.
    #[test]
    fn unit_css_contains_token_is_boundary_aware() {
        let hay = ".gap-2 { @apply gap-2; }\n.p-2 { @apply p-2; }";
        assert!(hay.contains("p-2"), "documents the old false positive");
        assert!(css_contains_token(hay, "p-2"));
        assert!(css_contains_token(hay, "gap-2"));
        assert!(!css_contains_token("only gap-2 here", "p-2"));
        assert!(!css_contains_token(".p-2.5 { }", "p-2"));
        // compiled selectors escape the decimal dot: p-2\.5
        assert!(!css_contains_token(".p-2\\.5 { }", "p-2"));
        assert!(!css_contains_token(".sp-2 { }", "p-2"));
        assert!(css_contains_token(".hover\\:p-2 { }", "p-2"));
        assert!(!css_contains_token("", "p-2"));
        assert!(!css_contains_token(".p-2 { }", ""));
    }
}
