//! Port of pipeline/convert.go — registry .tsx → versioned IR (generated/ir).
//!
//! Route: esbuild Transform downgrades TSX to classic React.createElement
//! calls (spawned from the pinned node_modules binary — byte-identity proven
//! in probe/m0), the plain-JS output is scanned, and the original source's
//! JSX children kinds are zipped in by position. Order is the interface:
//! everything serializes through jsonorder (insertion-ordered, JS.stringify
//! semantics) and IR files are written with a ONE-space indent step.

pub mod cva;
pub mod jsx;
pub mod scan;
pub mod topscan;

use crate::emit::tags::{TERNARY_RE, external_member_tag, native_tags};
use crate::jsonorder::{Json, JsonObj, marshal_js_step};
use crate::tsx;
use cva::{CvReg, CvTable, CvTables};
use regex::Regex;
use scan::cv_prop_colon;
use scan::*;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::io::Write as _;
use std::path::Path;
use std::sync::{LazyLock, Mutex, OnceLock};

const CV_UI: &str = "build/resolved-ui/ui";
const CV_OUT: &str = "generated/ir";
const CV_PIN_FILE: &str = "src/registry/pin.json";
const CV_TIERS: &str = "src/registry/tiers.json";

// ------------------------------------------------------------------ tier set

pub(crate) fn cv_tier_sets() -> &'static [(&'static str, &'static [&'static str])] {
    &[
        (
            "kernel",
            &[
                "alert-dialog",
                "context-menu",
                "dialog",
                "dropdown-menu",
                "hover-card",
                "popover",
                "select",
                "slider",
                "scroll-area",
                "sheet",
                "tabs",
                "tooltip",
            ],
        ),
        (
            "trivial-js",
            &[
                "accordion",
                "aspect-ratio",
                "avatar",
                "checkbox",
                "collapsible",
                "label",
                "progress",
                "radio-group",
                "separator",
                "switch",
                "toggle",
                "toggle-group",
            ],
        ),
        ("medium", &["menubar", "navigation-menu"]),
        ("logic", &["combobox", "field", "sidebar"]),
        // bases/radix addition: questionnaire is a foreign-runtime wrapper
        ("external", &["questionnaire"]),
    ]
}

pub(crate) fn cv_known_icons() -> &'static [&'static str] {
    &[
        "ChevronRight",
        "ChevronDown",
        "MoreHorizontal",
        "Check",
        "X",
        "Plus",
        "Minus",
        "Search",
    ]
}

fn cv_tier_index() -> &'static HashMap<&'static str, &'static str> {
    static M: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        for (tier, names) in cv_tier_sets() {
            for n in *names {
                m.insert(*n, *tier);
            }
        }
        m
    })
}

fn tier_of(name: &str, imports: &[String]) -> String {
    if let Some(t) = cv_tier_index().get(name) {
        return t.to_string();
    }
    for i in imports {
        if cv_foreign_import(i) {
            return "external".to_string();
        }
    }
    "static".to_string()
}

fn cv_foreign_import(i: &str) -> bool {
    for p in ["radix-ui", "@radix-ui", "@shadcn/", "@/", "next/"] {
        if i.starts_with(p) {
            return false;
        }
    }
    if matches!(
        i,
        "react"
            | "react-dom"
            | "lucide-react"
            | "class-variance-authority"
            | "clsx"
            | "tailwind-merge"
    ) {
        return false;
    }
    if i.starts_with('.') {
        return false;
    }
    true
}

fn cv_known_icons_list() -> &'static [&'static str] {
    cv_known_icons()
}

// ------------------------------------------------------------------- esbuild

/// esbuildTsx via the pinned node_modules binary (byte-identity with the Go
/// in-process api.Transform was proven in probe/m0; flags mirror the
/// TransformOptions one-to-one).
pub fn esbuild_tsx(root: &Path, src: &str) -> Result<String, String> {
    let esbuild = root.join("node_modules/.bin/esbuild");
    let mut cmd = std::process::Command::new(esbuild);
    cmd.args([
        "--loader=tsx",
        "--jsx=transform",
        "--jsx-factory=React.createElement",
        "--jsx-fragment=React.Fragment",
        "--format=esm",
        "--charset=utf8",
    ])
    .current_dir(root)
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("esbuild: {}", e))?;
    child
        .stdin
        .as_mut()
        .expect("piped")
        .write_all(src.as_bytes())
        .map_err(|e| format!("esbuild: {}", e))?;
    let out = child
        .wait_with_output()
        .map_err(|e| format!("esbuild: {}", e))?;
    if !out.status.success() {
        return Err(format!(
            "esbuild: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

// ------------------------------------------------------------ convertFile

#[derive(Default, Clone)]
pub struct CvFile {
    pub pin_commit: Option<String>,
    pub name: String,
    pub tier: String,
    pub imports: Vec<String>,
    pub icons: Vec<String>,
    pub cva: CvTables,
    pub components: Vec<CvComponent>,
    pub conditionals: Vec<Vec<(String, Json)>>,
    pub cva_refs: Vec<Vec<(String, Json)>>,
    pub tag_hints: Vec<(String, Json)>,
    pub meta_module_of: HashMap<String, String>,
    pub meta_tag_vars: HashMap<String, String>,
    pub meta_import: HashMap<String, String>,
}

#[derive(Default, Clone)]
pub struct CvComponent {
    pub fn_name: String,
    pub is_export: bool,
    pub elements: Vec<Vec<(String, Json)>>,
}

/// resolve context (fileCtx + extractFn's ctx in one)
pub struct CvCtx<'a> {
    pub file: &'a mut CvFile,
    pub reg: &'a CvReg,
    pub src: &'a str,
    pub js: &'a str,
    pub param_defaults: HashMap<String, String>,
    pub fn_name: String,
    pub body_start: usize,
    pub body_end: usize,
    pub ref_keys: &'a mut HashSet<String>,
}

#[derive(Clone)]
pub struct CvElCtx {
    pub props: Vec<CvProp>,
}

#[derive(Clone)]
pub struct CvCallOcc {
    pos: usize,
    args: Vec<String>,
    arg_pos: Vec<usize>,   // absolute position of each arg's first non-space byte
    child_arg: Vec<usize>, // arg indexes of direct element-call children
    child_pos: Vec<usize>, // absolute positions of those calls
    tag: String,
    tag_ok: bool,
}

struct CvCallPair {
    call: CvCallOcc,
    rec: jsx::CvJsxRec,
    fn_index: isize, // index into the pushed-fn list, -1 = dropped
}

pub fn cv_find_calls(js: &str) -> Result<Vec<CvCallOcc>, String> {
    let mut out: Vec<CvCallOcc> = Vec::new();
    let b = js.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let e = cv_mask_end(js, i);
        if e > 0 {
            i = e;
            continue;
        }
        if b[i] == b'R' && js[i..].starts_with("React.createElement(") {
            // callee must be exactly React.createElement (not a member tail)
            if i > 0 {
                let p = b[i - 1];
                if p == b'.'
                    || p.is_ascii_lowercase()
                    || p.is_ascii_uppercase()
                    || p.is_ascii_digit()
                    || p == b'_'
                    || p == b'$'
                {
                    i += 1;
                    continue;
                }
            }
            let occ = cv_parse_call(js, i)?;
            // a fragment is not an element; skip the record but keep scanning
            if !occ.tag_ok && occ.args[0] == "React.Fragment" {
                i = occ.pos + 1;
                continue;
            }
            i = occ.pos + 1; // resume inside the args; nested calls come later
            out.push(occ);
            continue;
        }
        i += 1;
    }
    Ok(out)
}

fn cv_parse_call(js: &str, i: usize) -> Result<CvCallOcc, String> {
    let mut occ = CvCallOcc {
        pos: i,
        args: Vec::new(),
        arg_pos: Vec::new(),
        child_arg: Vec::new(),
        child_pos: Vec::new(),
        tag: String::new(),
        tag_ok: false,
    };
    let open = i + "React.createElement".len();
    let close = cv_match_bracket(js, open, b'(', b')')
        .ok_or_else(|| format!("createElement: unbalanced call at {}", open))?;
    let mut offset = open + 1;
    for a in cv_split_top(&js[open + 1..close]) {
        let trimmed = a.trim_start_matches([' ', '\t', '\n', '\r']);
        occ.arg_pos.push(offset + (a.len() - trimmed.len()));
        occ.args.push(trimmed.to_string());
        offset += a.len() + 1;
    }
    if occ.args.len() < 2 {
        return Err(format!(
            "createElement: {} args at {}",
            occ.args.len(),
            open
        ));
    }
    let tag = cv_parse_expr(&occ.args[0]);
    if tag.kind == "str" {
        occ.tag = tag.str.clone();
        occ.tag_ok = true;
    } else if occ.args[0] == "React.Fragment" {
        // a fragment is not an element; its children are visited via the scan
    } else if tag.kind == "ident" {
        occ.tag = tag.ident.clone();
        occ.tag_ok = true;
    } else if CV_MEMBER_RE.is_match(&occ.args[0]) {
        occ.tag = occ.args[0].clone();
        occ.tag_ok = true;
    }
    for k in 2..occ.args.len() {
        let mut t = occ.args[k].clone();
        if let Some(rest) = t.strip_prefix("/* @__PURE__ */") {
            t = rest.trim_start_matches([' ', '\t', '\n', '\r']).to_string();
        }
        if t.starts_with("React.createElement(")
            && t != "React.createElement(React.Fragment"
            && !t.starts_with("React.createElement(React.Fragment,")
            && !t.starts_with("React.createElement(React.Fragment)")
        {
            occ.child_arg.push(k);
            occ.child_pos
                .push(occ.arg_pos[k] + occ.args[k].len() - t.len());
        }
    }
    Ok(occ)
}

fn convert_file(
    _root: &Path,
    name: &str,
    src: &str,
    js: &str,
    reg: &mut CvReg,
) -> Result<CvFile, String> {
    let mut f = CvFile {
        name: name.to_string(),
        cva: CvTables::new(),
        meta_module_of: HashMap::new(),
        meta_tag_vars: HashMap::new(),
        meta_import: HashMap::new(),
        ..Default::default()
    };
    let imports = topscan::imports_of(src);
    f.imports = imports.imports;
    f.icons = imports.icons;
    f.meta_import = imports.import_map;
    f.meta_module_of = imports.module_of;

    let top = topscan::scan_top_js(js)?;

    // per-file cva tables
    f.cva = cva::cv_tables_of(js);

    let recs = jsx::scan_jsx_kinds(src).map_err(|e| format!("jsx scan: {}", e))?;
    let calls = cv_find_calls(js)?;
    if calls.len() != recs.len() {
        return Err(format!(
            "element scan misalignment: {} downgraded calls vs {} source elements",
            calls.len(),
            recs.len()
        ));
    }
    // pair every call with its record; a pair belongs to the (unique)
    // top-level fn whose body contains it
    let mut pairs: Vec<CvCallPair> = Vec::with_capacity(calls.len());
    let mut call_pos_to_pair: HashMap<usize, usize> = HashMap::new();
    for (k, call) in calls.into_iter().enumerate() {
        call_pos_to_pair.insert(call.pos, k);
        pairs.push(CvCallPair {
            call,
            rec: recs[k].clone(),
            fn_index: -1,
        });
    }
    // pushed fns in declaration order: function decls always; var arrows only
    // when declared WITH `export`; an anonymous default arrow as "default"
    let mut pushed: Vec<usize> = Vec::new();
    for (di, d) in top.decls.iter().enumerate() {
        if d.is_fn || (d.is_arrow && d.exported) || di as isize == top.default_arrow {
            pushed.push(di);
        }
    }
    for k in 0..pairs.len() {
        for (pi, di) in pushed.iter().enumerate() {
            let d = &top.decls[*di];
            if pairs[k].call.pos > d.body[0] && pairs[k].call.pos < d.body[1] {
                pairs[k].fn_index = pi as isize;
                break;
            }
        }
    }

    // exported names: export-spec names + declarations declared with `export`
    let mut exported: HashSet<String> = HashSet::new();
    for n in &top.exported_names {
        exported.insert(n.clone());
    }
    for d in &top.decls {
        if d.exported && (d.is_fn || d.is_arrow) {
            exported.insert(d.name.clone());
        }
    }

    // component assembly
    let mut file_ref_keys: HashSet<String> = HashSet::new();
    for (pi, di) in pushed.iter().enumerate() {
        let d = top.decls[*di].clone();
        let fn_name = if *di as isize == top.default_arrow {
            "default".to_string()
        } else {
            d.name.clone()
        };
        // merged per file (conflicts throw inside cvTagVarsOf)
        let tag_vars = cv_tag_vars_of(js, d.body)?;
        for (k, v) in tag_vars {
            f.meta_tag_vars.insert(k, v);
        }
        let mut comp = CvComponent {
            fn_name: fn_name.clone(),
            is_export: exported.contains(&fn_name),
            elements: Vec::new(),
        };
        let param_defaults = cv_param_defaults(js, &d);
        {
            let mut ctx = CvCtx {
                file: &mut f,
                reg,
                src,
                js,
                param_defaults,
                fn_name: fn_name.clone(),
                body_start: d.body[0],
                body_end: d.body[1],
                ref_keys: &mut file_ref_keys,
            };
            for k in 0..pairs.len() {
                if pairs[k].fn_index != pi as isize {
                    continue;
                }
                let (el, conds) = cv_process_element(&mut ctx, &mut pairs, k, &call_pos_to_pair)?;
                comp.elements.push(el);
                ctx.file.conditionals.extend(conds);
            }
        }
        f.components.push(comp);
    }

    f.tier = tier_of(name, &f.imports);
    cv_same_file_wrap(&mut f);
    Ok(f)
}

/// Builds one element record + its conditionals from a paired
/// (downgraded call, source kind list).
fn cv_process_element(
    ctx: &mut CvCtx<'_>,
    pairs: &mut [CvCallPair],
    k: usize,
    call_pos_to_pair: &HashMap<usize, usize>,
) -> Result<(Vec<(String, Json)>, Vec<Vec<(String, Json)>>), String> {
    let occ = pairs[k].call.clone();
    let rec = pairs[k].rec.clone();
    let mut props: Vec<CvProp> = Vec::new();
    let props_text = &occ.args[1];
    if props_text != "null" {
        let n = cv_parse_expr(props_text);
        if n.kind == "obj" {
            props = n.obj;
        }
    }
    let el = CvElCtx {
        props: props.clone(),
    };
    let mut slot: Option<String> = None;
    let mut classes: Vec<String> = Vec::new();
    let mut spread = false;
    let mut attrs = JsonObj::new();
    for p in &props {
        if p.spread {
            spread = true;
            continue;
        }
        if p.key == "data-slot" {
            let n = cv_parse_expr(&p.val);
            if n.kind == "str" {
                slot = Some(n.str);
            }
        }
        if p.key == "className" {
            class_strings(ctx, &p.val, &mut classes, Some(&el));
        }
        if IDENTITY_ATTR.is_match(&p.key) {
            let n = cv_parse_expr(&p.val);
            if n.kind == "str" {
                attrs = attrs.add(&p.key, Json::Str(n.str));
            }
        }
    }
    // component-wrap: an imported component (e.g. Button) renders with the
    // wrapped table's classes. ORDER is React's: the wrapped component
    // composes cn(itsVariants(...), className) — its own classes FIRST.
    if occ.tag_ok && CV_IDENT_RE.is_match(&occ.tag) {
        if let Some(imp) = ctx.file.meta_import.get(&occ.tag).cloned() {
            if let Some(comp) = ctx.reg.comp_cva.get(&imp) {
                let table = comp.table.clone();
                let comp_file = comp.file.clone();
                let comp_cva_name = comp.cva_name.clone();
                let mut args: HashMap<String, String> = HashMap::new();
                for p in &props {
                    if p.spread {
                        continue;
                    }
                    if table.has_axis(&p.key) {
                        args.insert(p.key.clone(), p.val.clone());
                    }
                }
                let mut wrapped: Vec<String> = Vec::new();
                let ref_name = format!("{}:{}", comp_file, comp_cva_name);
                resolve_cva_args(
                    ctx,
                    Some(&el),
                    &table,
                    None,
                    &args,
                    &mut wrapped,
                    &ref_name,
                    true,
                )?;
                let mut merged = wrapped;
                merged.extend(classes);
                classes = merged;
            }
        }
    }
    let classes_clean: Vec<String> = classes.iter().filter(|c| !c.is_empty()).cloned().collect();
    // children from the ORIGINAL side's kind list; element kinds consume the
    // direct element-call args of THIS call in order
    let elem_kind_count = rec.kinds.iter().filter(|kd| kd.elem).count();
    if elem_kind_count != occ.child_pos.len() {
        return Err(format!(
            "children misalignment at {}:{} element {}: {} source element children vs {} downgraded",
            ctx.file.name,
            ctx.fn_name,
            occ.tag,
            elem_kind_count,
            occ.child_pos.len()
        ));
    }
    let mut children: Vec<Json> = Vec::new();
    let mut ei = 0;
    for kd in &rec.kinds {
        if !kd.elem {
            children.push(Json::Str(kd.text.clone()));
            continue;
        }
        let child = &pairs[call_pos_to_pair[&occ.child_pos[ei]]];
        ei += 1;
        let mut sketch = format!("<{}", child.call.tag);
        let s = cv_child_slot(&child.call);
        if !s.is_empty() {
            sketch.push_str(&format!(" slot={}", s));
        }
        let n = cv_child_classes(&child.call).len();
        if n > 0 {
            sketch.push_str(&format!(" class=[{}]", n));
        }
        sketch.push('>');
        children.push(Json::Str(sketch));
    }
    let tag_json = if occ.tag_ok {
        Json::Str(occ.tag.clone())
    } else {
        Json::Null
    };
    let mut o = JsonObj::new()
        .add("tag", tag_json.clone())
        .add(
            "slot",
            match &slot {
                Some(s) => Json::Str(s.clone()),
                None => Json::Null,
            },
        )
        .add(
            "classes",
            Json::Arr(classes_clean.iter().map(|c| Json::Str(c.clone())).collect()),
        )
        .add("spread", Json::Bool(spread))
        .add("children", Json::Arr(children));
    if !attrs.is_empty() {
        // additive and omitted when empty, so 60 of the 61 IR files are
        // unchanged
        o = o.add("attrs", Json::from_obj(attrs));
    }
    // child-cond: an expression-container child that is a logical or
    // conditional; then class-cond inside className cn() args
    let mut conds: Vec<Vec<(String, Json)>> = Vec::new();
    for kd in &rec.kinds {
        if !kd.elem && kd.text == "OPT?" {
            conds.push(
                JsonObj::new()
                    .add("kind", Json::Str("child-cond".into()))
                    .add("fn", Json::Str(ctx.fn_name.clone()))
                    .add("parent", tag_json.clone())
                    .into_pairs(),
            );
        }
    }
    for p in &props {
        if p.spread || p.key != "className" {
            continue;
        }
        conds.extend(cv_detect_cond(ctx, &p.val, &slot));
    }
    Ok((o.into_pairs(), conds))
}

/// cvChildSlot / cvChildClasses — extractAttrs(child, null) for the sketch:
/// literal-only classes, no cva context.
fn cv_child_slot(occ: &CvCallOcc) -> String {
    if occ.args[1] == "null" {
        return String::new();
    }
    let n = cv_parse_expr(&occ.args[1]);
    if n.kind == "obj" {
        for p in &n.obj {
            if !p.spread && p.key == "data-slot" {
                let v = cv_parse_expr(&p.val);
                if v.kind == "str" {
                    return v.str;
                }
            }
        }
    }
    String::new()
}

fn cv_child_classes(occ: &CvCallOcc) -> Vec<String> {
    let mut acc: Vec<String> = Vec::new();
    if occ.args[1] == "null" {
        return acc;
    }
    let n = cv_parse_expr(&occ.args[1]);
    if n.kind == "obj" {
        for p in &n.obj {
            if !p.spread && p.key == "className" {
                cv_class_strings_nil(&p.val, &mut acc);
            }
        }
    }
    acc.into_iter().filter(|c| !c.is_empty()).collect()
}

// -------------------------------------------------- class collection + cva

static IDENTITY_ATTR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)^(role|type|aria-[0-9A-Za-z_-]+|dir|lang|scope|colspan|rowspan|target|rel|tabindex)$",
    )
    .unwrap()
});

/// lookupCva: (table, cross, ok)
#[allow(dead_code)] // Go-port surface; the rust converter resolves cva through lookup_cva_named
fn lookup_cva(c: &CvCtx<'_>, local: &str) -> Option<(CvTable, bool)> {
    if let Some(imp) = c.file.meta_import.get(local) {
        if let Some(e) = c.reg.cva_by_export.get(imp) {
            return Some((e.table.clone(), true));
        }
    }
    if let Some(t) = c.file.cva.get(local) {
        return Some((t.clone(), false));
    }
    None
}

/// resolveCvaArgs — el == None when the ctx only collects literals.
#[allow(clippy::too_many_arguments)]
fn resolve_cva_args(
    c: &mut CvCtx<'_>,
    el: Option<&CvElCtx>,
    table: &CvTable,
    local_name: Option<&str>,
    args: &HashMap<String, String>,
    acc: &mut Vec<String>,
    ref_name: &str,
    cross: bool,
) -> Result<(), String> {
    if !table.base.is_empty() {
        acc.push(table.base.clone());
    }
    let mut dyn_: Vec<Vec<(String, Json)>> = Vec::new();
    let mut dyn_axes: Vec<String> = Vec::new();
    let mut dyn_defaults = JsonObj::new();
    let own_attr = |name: &str| -> bool {
        match el {
            None => false,
            Some(el) => el.props.iter().any(|p| !p.spread && p.key == name),
        }
    };
    // resolveCvaArgs may append to the table's defaults (fromParam && !defOk
    // && !cross); the table handed in here is the caller's clone, mirrored
    // back by the caller where Go mutates through the shared pointer
    for axis in &table.axes {
        let raw = args.get(&axis.axis);
        let mut val = String::new();
        let mut v: Option<ExprNode> = None;
        if let Some(raw) = raw {
            let vv = cv_parse_expr(raw);
            match vv.kind.as_str() {
                "str" => val = vv.str.clone(),
                "logical" => {
                    let r = cv_parse_expr(&vv.right);
                    if r.kind == "str" {
                        val = r.str.clone();
                    } else if r.kind == "ident" {
                        val = c.param_defaults.get(&r.ident).cloned().unwrap_or_default();
                    }
                    let l = cv_parse_expr(&vv.left);
                    // `context.size || size` with data-size={…} on the
                    // element: the value arrives through React CONTEXT —
                    // treat it as a dynamic axis keyed on data-<axis>
                    if l.kind != "str"
                        && own_attr(&format!("data-{}", axis.axis))
                        && !val.is_empty()
                    {
                        dyn_axes.push(axis.axis.clone());
                        dyn_defaults = dyn_defaults.add(&axis.axis, Json::Str(val.clone()));
                        continue;
                    }
                }
                "ident" => {
                    val = c.param_defaults.get(&vv.ident).cloned().unwrap_or_default();
                }
                _ => {}
            }
            v = Some(vv);
        }
        let def = table.has_default(&axis.axis);
        if !val.is_empty() {
            match table.value_for(&axis.axis, &val) {
                Some(cls) => {
                    if !cls.is_empty() {
                        acc.push(cls);
                    }
                    // A literal PARAM default that feeds the axis is the
                    // axis's default in every React render; only the
                    // element's own root call binds the default
                    let from_param = match &v {
                        Some(vv) => {
                            vv.kind == "ident"
                                || (vv.kind == "logical"
                                    && cv_parse_expr(&vv.right).kind == "ident")
                        }
                        None => false,
                    };
                    if from_param && def.is_none() && !cross {
                        if let Some(local) = local_name {
                            if let Some(t) = c.file.cva.get_mut(local) {
                                t.set_default(&axis.axis, &val);
                            }
                        }
                    }
                    continue;
                }
                None => {
                    return Err(format!(
                        "cva unknown variant value: {}={} (ref {})",
                        axis.axis,
                        crate::jsonorder::json_string(&val),
                        ref_name
                    ));
                }
            }
        }
        if let Some(vv) = &v {
            if vv.kind == "cond" {
                let cons = cv_parse_expr(&vv.cons);
                let alt = cv_parse_expr(&vv.alt);
                if cons.kind == "str" && alt.kind == "str" {
                    if let Some(cls) = table.value_for(&axis.axis, &alt.str) {
                        if !cls.is_empty() {
                            acc.push(cls); // falsy state = base
                        }
                    }
                    let mut test = String::new();
                    let tv = cv_parse_expr(&vv.test);
                    if tv.kind == "ident" {
                        test = tv.ident.clone();
                    }
                    // attr scan: the element itself, then (asChild pattern)
                    // the whole fn
                    let mut attr = String::new();
                    if !test.is_empty() {
                        attr = find_data_attr(c, el, &test);
                    }
                    // attr-driven consequent: without the matching data-* attr
                    // the "then" classes would be silently lost — fail loudly
                    if attr.is_empty() {
                        let show = if test.is_empty() { "null" } else { &test };
                        return Err(format!(
                            "cva ident-ternary without data-* attr binding: {} (ref {})",
                            show, ref_name
                        ));
                    }
                    if let Some(cls) = table.value_for(&axis.axis, &cons.str) {
                        if !cls.is_empty() {
                            dyn_.push(
                                JsonObj::new()
                                    .add("attr", Json::Str(attr))
                                    .add("when", Json::Str("true".into()))
                                    .add("classes", Json::Str(cls))
                                    .into_pairs(),
                            );
                        }
                    }
                    continue;
                }
            }
        }
        // dynamic axis: merge default into base, emit all values
        if let Some(def) = &def {
            if let Some(cls) = table.value_for(&axis.axis, def) {
                if !cls.is_empty() {
                    acc.push(cls);
                }
            }
        }
        dyn_axes.push(axis.axis.clone());
    }
    if el.is_some() && (!dyn_.is_empty() || !dyn_axes.is_empty()) {
        record_cva_ref(c, el, ref_name, table, dyn_, dyn_axes, cross, dyn_defaults);
    }
    Ok(())
}

/// classStrings — collect class strings from a className value node.
fn class_strings(c: &mut CvCtx<'_>, text: &str, acc: &mut Vec<String>, el: Option<&CvElCtx>) {
    let n = cv_parse_expr(text);
    match n.kind.as_str() {
        "str" => acc.push(n.str),
        "tpl" => acc.push(n.str),
        // tplMulti: interpolated template skipped (documented limit)
        "call" => {
            if n.ident == "cn" {
                for a in &n.args {
                    class_strings(c, a, acc, el);
                }
                return;
            }
            if let Some((table, cross, local)) = lookup_cva_named(c, &n.ident) {
                let mut args: HashMap<String, String> = HashMap::new();
                if !n.args.is_empty() {
                    let a0 = cv_parse_expr(&n.args[0]);
                    if a0.kind == "obj" {
                        for p in &a0.obj {
                            // babel: p.key?.name — identifier keys only
                            if !p.quoted && !p.spread {
                                args.insert(p.key.clone(), p.val.clone());
                            }
                        }
                    }
                }
                let _ = resolve_cva_args(c, el, &table, Some(&local), &args, acc, &n.ident, cross);
            }
        }
        _ => {
            if n.kind == "cond" {
                // class-cond pattern (H1b): ternary inside cn() — both
                // branches are collected as unconditional classes
                class_strings(c, &n.cons, acc, el);
                class_strings(c, &n.alt, acc, el);
            }
            if n.kind == "logical" {
                class_strings(c, &n.right, acc, el);
            }
        }
    }
}

/// lookup_cva: (table, cross, local name) — the local name lets the !cross
/// default-mutation land on the file's own table (Go mutates the shared ptr).
fn lookup_cva_named(c: &CvCtx<'_>, local: &str) -> Option<(CvTable, bool, String)> {
    if let Some(imp) = c.file.meta_import.get(local) {
        if let Some(e) = c.reg.cva_by_export.get(imp) {
            return Some((e.table.clone(), true, local.to_string()));
        }
    }
    if c.file.cva.get(local).is_some() {
        let t = c.file.cva.get(local).cloned();
        return t.map(|t| (t, false, local.to_string()));
    }
    None
}

/// The ctx-nil path: literal strings only (used for the child sketches).
fn cv_class_strings_nil(text: &str, acc: &mut Vec<String>) {
    let n = cv_parse_expr(text);
    match n.kind.as_str() {
        "str" => acc.push(n.str),
        "tpl" => acc.push(n.str),
        "call" => {
            if n.ident == "cn" {
                for a in &n.args {
                    cv_class_strings_nil(a, acc);
                }
            }
        }
        "cond" => {
            cv_class_strings_nil(&n.cons, acc);
            cv_class_strings_nil(&n.alt, acc);
        }
        "logical" => cv_class_strings_nil(&n.right, acc),
        _ => {}
    }
}

fn find_data_attr(c: &CvCtx<'_>, el: Option<&CvElCtx>, test: &str) -> String {
    // the element's own props first, in order
    if let Some(el) = el {
        for p in &el.props {
            if p.spread || !p.key.starts_with("data-") {
                continue;
            }
            let n = cv_parse_expr(&p.val);
            if n.kind == "ident" && n.ident == test {
                return p.key.clone();
            }
        }
    }
    // then every JSX prop in the fn body, in document order
    cv_scan_fn_data_attr(c.js, c.body_start, c.body_end, test)
}

static CV_DATA_ATTR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#""(data-[0-9A-Za-z_-]+)"\s*:\s*([A-Za-z_$][A-Za-z0-9_$]*)"#).unwrap()
});

/// Finds the first `data-x: <ident>` JSX prop in the body span whose
/// identifier equals test. Only unmasked matches count.
fn cv_scan_fn_data_attr(js: &str, start: usize, end: usize, test: &str) -> String {
    let seg = &js[start..end];
    for m in CV_DATA_ATTR_RE.captures_iter(seg) {
        let whole = m.get(0).unwrap();
        if !cv_pos_unmasked(js, start, start + whole.start()) {
            continue;
        }
        if &seg[m.get(2).unwrap().start()..m.get(2).unwrap().end()] == test {
            return m[1].to_string();
        }
    }
    String::new()
}

fn cv_pos_unmasked(js: &str, base: usize, pos: usize) -> bool {
    let mut i = base;
    while i < pos {
        let e = cv_mask_end(js, i);
        if e > 0 {
            if e > pos {
                return false;
            }
            i = e;
            continue;
        }
        i += 1;
    }
    true
}

fn record_cva_ref(
    c: &mut CvCtx<'_>,
    el: Option<&CvElCtx>,
    ref_name: &str,
    table: &CvTable,
    dyn_: Vec<Vec<(String, Json)>>,
    dyn_axes: Vec<String>,
    cross: bool,
    defaults: JsonObj,
) {
    if !cross {
        return;
    }
    let el = match el {
        Some(e) => e,
        None => return,
    };
    let mut slot: Option<String> = None;
    for p in &el.props {
        if !p.spread && p.key == "data-slot" {
            let n = cv_parse_expr(&p.val);
            if n.kind == "str" {
                slot = Some(n.str);
            }
            break;
        }
    }
    let Some(slot) = slot else { return };
    let mut dkeys: Vec<String> = Vec::new();
    for d in &dyn_ {
        let a = match &d[0].1 {
            Json::Str(s) => s.clone(),
            _ => String::new(),
        };
        let w = match &d[1].1 {
            Json::Str(s) => s.clone(),
            _ => String::new(),
        };
        dkeys.push(format!("{}{}", a, w));
    }
    let key = format!(
        "{}|{}|{}|{}",
        slot,
        ref_name,
        dkeys.join(","),
        dyn_axes.join(",")
    );
    if c.ref_keys.contains(&key) {
        return;
    }
    c.ref_keys.insert(key);
    c.file.cva_refs.push(
        JsonObj::new()
            .add("slot", Json::Str(slot))
            .add("ref", Json::Str(ref_name.to_string()))
            .add("table", table.json())
            .add("dyn", Json::Arr(dyn_.into_iter().map(Json::Obj).collect()))
            .add(
                "dynAxes",
                Json::Arr(dyn_axes.into_iter().map(Json::Str).collect()),
            )
            .add("defaults", Json::from_obj(defaults))
            .into_pairs(),
    );
}

// ------------------------------------------------------------- cond detect

/// port of extractFn's class-cond detect() walk.
fn cv_detect_cond(ctx: &CvCtx<'_>, text: &str, slot: &Option<String>) -> Vec<Vec<(String, Json)>> {
    let mut out: Vec<Vec<(String, Json)>> = Vec::new();
    fn detect(ctx: &CvCtx<'_>, t: &str, slot: &Option<String>, out: &mut Vec<Vec<(String, Json)>>) {
        let n = cv_parse_expr(t);
        if n.kind == "cond" {
            // the predicate, when it is `ident === "literal"` / `!==`
            let mut test: Option<Vec<(String, Json)>> = None;
            if let Some(m) = CV_COND_TEST_RE.captures(n.test.trim()) {
                let name = m[1].to_string();
                let op = m[2].to_string();
                let mut o = JsonObj::new()
                    .add("name", Json::Str(name.clone()))
                    .add("op", Json::Str(op))
                    .add("value", Json::Str(cv_unquote(&m[3])));
                if let Some(mm) = cv_cond_default_re(&name).captures(&ctx.src) {
                    o = o.add("default", Json::Str(mm[1].to_string()));
                }
                test = Some(o.into_pairs());
            }
            let mut then_str = String::new();
            let mut else_str = String::new();
            let cn = cv_parse_expr(&n.cons);
            if cn.kind == "str" {
                then_str = cn.str;
            }
            let an = cv_parse_expr(&n.alt);
            if an.kind == "str" {
                else_str = an.str;
            }
            let mut c = JsonObj::new()
                .add("kind", Json::Str("class-cond".into()))
                .add("fn", Json::Str(ctx.fn_name.clone()))
                .add(
                    "slot",
                    match slot {
                        Some(s) => Json::Str(s.clone()),
                        None => Json::Null,
                    },
                )
                .add("then", Json::Str(then_str))
                .add("else", Json::Str(else_str));
            if let Some(test) = test {
                c = c.add("test", Json::Obj(test));
            }
            out.push(c.into_pairs());
            return;
        }
        if n.kind == "logical" {
            detect(ctx, &n.right, slot, out);
            return;
        }
        if n.kind == "call" {
            for a in &n.args {
                detect(ctx, a, slot, out);
            }
        }
    }
    detect(ctx, text, slot, &mut out);
    // the compiler-visible use of ctx keeps parity with Go's capture
    let _ = ctx.src;
    out
}

static CV_COND_TEST_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^([A-Za-z_$][A-Za-z0-9_$]*)\s*(===|!==)\s*("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*')$"#,
    )
    .unwrap()
});

fn cv_cond_default_re(name: &str) -> Regex {
    // the distinct names are few and recur across all 61 files; compiling per
    // ternary dominated the match itself
    static CACHE: OnceLock<Mutex<HashMap<String, Regex>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(re) = cache.lock().unwrap().get(name) {
        return re.clone();
    }
    let quoted = regex_quote(name);
    // \b → ASCII word boundary emulation via (?-u:\b)
    let re =
        Regex::new(&format!("(?-u:\\b){}\\s*=\\s*\"([^\"]+)\"", quoted)).expect("cond default re");
    cache.lock().unwrap().insert(name.to_string(), re.clone());
    re
}

/// Go regexp.QuoteMeta
pub(crate) fn regex_quote(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if matches!(
            c,
            '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$'
        ) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

// --------------------------------------------- tag vars / param defaults

/// fn-local tag variables: `const Comp = asChild ? Slot.Root : "div"` — the
/// native string is what the no-React emitter renders.
fn cv_tag_vars_of(js: &str, body: [usize; 2]) -> Result<HashMap<String, String>, String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"(?:^|[^\w$.])(const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=").unwrap()
    });
    let mut out: HashMap<String, String> = HashMap::new();
    let seg = &js[body[0]..body[1]];
    for m in re.captures_iter(seg) {
        let whole = m.get(0).unwrap();
        if !cv_pos_unmasked(js, body[0], body[0] + whole.start()) {
            continue;
        }
        let name = m[2].to_string();
        // init runs to the next top-level , or ; (multi-declarator support)
        let init_start = whole.end();
        let mut depth = 0i32;
        let mut init_end = seg.len();
        let sb = seg.as_bytes();
        let mut i = init_start;
        while i < sb.len() {
            let e = cv_mask_end(seg, i);
            if e > 0 {
                i = e;
                continue;
            }
            match sb[i] {
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' => depth -= 1,
                b',' | b';' => {
                    if depth == 0 {
                        init_end = i;
                        break;
                    }
                }
                _ => {}
            }
            if init_end != seg.len() {
                break;
            }
            i += 1;
        }
        let init = cv_parse_expr(&seg[init_start..init_end]);
        let t = match init.kind.as_str() {
            "str" => init.str.clone(),
            "cond" => {
                let a = cv_parse_expr(&init.alt);
                if a.kind == "str" {
                    a.str
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        };
        if t.is_empty() {
            continue;
        }
        if let Some(old) = out.get(&name) {
            if *old != t {
                return Err(format!("conflicting tag var {}: {} vs {}", name, old, t));
            }
        }
        out.insert(name, t);
    }
    Ok(out)
}

/// Literal-default params feed cva-call resolution. Defaults live inside
/// destructuring ObjectPatterns, not only as top-level AssignmentPattern
/// params; depth-≥2 nested defaults are (still) not extracted.
fn cv_param_defaults(js: &str, d: &topscan::CvDecl) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    if !d.single_param.is_empty() {
        return out;
    }
    let params_text = &js[d.params[0]..d.params[1] + 1];
    let inner = &params_text[1..params_text.len() - 1];
    for pm in cv_split_top(inner) {
        let p = pm.trim();
        if p.is_empty() {
            continue;
        }
        if p.starts_with('{') {
            if let Some(e) = cv_match_bracket(p, 0, b'{', b'}') {
                for q in cv_pattern_defaults(&p[1..e]) {
                    out.insert(q[0].clone(), q[1].clone());
                }
            }
            continue;
        }
        if p.starts_with('[') {
            if let Some(e) = cv_match_bracket(p, 0, b'[', b']') {
                for q in cv_pattern_defaults(&p[1..e]) {
                    out.insert(q[0].clone(), q[1].clone());
                }
            }
            continue;
        }
        if let Some((name, lit)) = cv_assign_pattern_default(p) {
            out.insert(name, lit);
        }
    }
    out
}

/// Top-level elements of a destructuring pattern body.
fn cv_pattern_defaults(inner: &str) -> Vec<[String; 2]> {
    let mut out: Vec<[String; 2]> = Vec::new();
    for part in cv_split_top(inner) {
        let mut p = part.trim().to_string();
        if p.is_empty() || p.starts_with("...") || p.starts_with('{') || p.starts_with('[') {
            continue;
        }
        // a `key:` renames — the BINDING is after the colon
        let colon = cv_prop_colon(&p);
        if colon >= 0 {
            p = p[colon as usize + 1..].trim().to_string();
        }
        if let Some((name, lit)) = cv_assign_pattern_default(&p) {
            out.push([name, lit]);
        }
    }
    out
}

/// `name = "lit"` → (name, lit); `name` alone → no.
fn cv_assign_pattern_default(p: &str) -> Option<(String, String)> {
    let b = p.as_bytes();
    let mut eq: isize = -1;
    let mut depth = 0i32;
    let mut i = 0;
    while i < b.len() {
        let e = cv_mask_end(p, i);
        if e > 0 {
            i = e;
            continue;
        }
        match b[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b'=' => {
                if depth == 0 {
                    if i + 1 < b.len() && (b[i + 1] == b'=' || b[i + 1] == b'>') {
                        i += 1;
                        continue;
                    }
                    eq = i as isize;
                }
            }
            _ => {}
        }
        if eq >= 0 {
            break;
        }
        i += 1;
    }
    if eq < 0 {
        return None;
    }
    let eq = eq as usize;
    let name = p[..eq].trim().to_string();
    if !CV_IDENT_RE.is_match(&name) {
        return None;
    }
    let lit = cv_parse_expr(&p[eq + 1..]);
    if lit.kind != "str" {
        return None;
    }
    Some((name, lit.str))
}

// ---------------------------------------------------------- same-file wrap

/// Same-file slotless wrap: a fn root that renders another same-file fn and
/// sets no data-slot rides the wrapped fn's root classes (cva-resolved).
fn cv_same_file_wrap(f: &mut CvFile) {
    let mut by_fn: HashMap<String, usize> = HashMap::new();
    for (i, c) in f.components.iter().enumerate() {
        by_fn.insert(c.fn_name.clone(), i);
    }
    fn json_strings(v: &Json) -> Vec<String> {
        match v {
            Json::Arr(items) => items
                .iter()
                .filter_map(|e| match e {
                    Json::Str(s) => Some(s.clone()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }
    fn elem_slot(root: &[(String, Json)]) -> Option<String> {
        match &root[1].1 {
            Json::Str(s) => Some(s.clone()),
            _ => None,
        }
    }
    fn eff_root(
        components: &[CvComponent],
        by_fn: &HashMap<String, usize>,
        fn_name: &str,
        stack: &mut HashSet<String>,
    ) -> Vec<String> {
        let Some(ci) = by_fn.get(fn_name) else {
            return Vec::new();
        };
        let c = &components[*ci];
        if c.elements.is_empty() || stack.contains(fn_name) {
            return Vec::new();
        }
        let root = &c.elements[0];
        if let Some(slot) = elem_slot(root) {
            if !slot.is_empty() {
                // slotted base case: classes live here
                return json_strings(&root[2].1);
            }
        }
        stack.insert(fn_name.to_string());
        let tag = match &root[0].1 {
            Json::Str(s) => s.clone(),
            _ => String::new(),
        };
        let mut wrapped = Vec::new();
        if !tag.is_empty() && by_fn.contains_key(&tag) {
            wrapped = eff_root(components, by_fn, &tag, stack);
        }
        wrapped.extend(json_strings(&root[2].1));
        wrapped
    }
    let snapshot = f.components.clone();
    for i in 0..f.components.len() {
        if f.components[i].elements.is_empty() {
            continue;
        }
        let (slot, tag) = {
            let root = &f.components[i].elements[0];
            let slot = match &root[1].1 {
                Json::Str(s) => s.clone(),
                _ => String::new(),
            };
            let tag = match &root[0].1 {
                Json::Str(s) => s.clone(),
                _ => String::new(),
            };
            (slot, tag)
        };
        if slot.is_empty()
            && !tag.is_empty()
            && by_fn.contains_key(&tag)
            && tag != f.components[i].fn_name
        {
            // [...new Set(effRoot(c.fn))].filter(Boolean) — insertion-ordered
            let mut seen: HashSet<String> = HashSet::new();
            let mut out: Vec<Json> = Vec::new();
            for s in eff_root(
                &snapshot,
                &by_fn,
                &f.components[i].fn_name,
                &mut HashSet::new(),
            ) {
                if s.is_empty() || seen.contains(&s) {
                    continue;
                }
                seen.insert(s.clone());
                out.push(Json::Str(s));
            }
            f.components[i].elements[0][2].1 = Json::Arr(out);
        }
    }
}

// ------------------------------------------------------------------ tagHints

fn is_icon_name(t: &str, icons: &[String]) -> bool {
    if icons.iter().any(|i| i == t) {
        return true;
    }
    if cv_known_icons_list().iter().any(|i| *i == t) {
        return true;
    }
    t.ends_with("Icon")
}

/// Cross-file tag resolution (needs every IR); returns an error on
/// unresolved → loud fail.
fn build_tag_hints(irs: &mut [CvFile]) -> Result<(), String> {
    let mut by_name: HashMap<String, usize> = HashMap::new();
    for (i, ir) in irs.iter().enumerate() {
        by_name.insert(ir.name.clone(), i);
    }
    let mut fn_root: HashMap<String, String> = HashMap::new();
    for ir in irs.iter() {
        for c in &ir.components {
            if !c.elements.is_empty() {
                if let Json::Str(t) = &c.elements[0][0].1 {
                    fn_root.insert(format!("{}:{}", ir.name, c.fn_name), t.clone());
                }
            }
        }
    }

    fn resolve_root(
        irs: &[CvFile],
        by_name: &HashMap<String, usize>,
        fn_root: &HashMap<String, String>,
        file: &str,
        fn_name: &str,
        seen: &mut HashSet<String>,
    ) -> String {
        let key = format!("{}:{}", file, fn_name);
        if seen.contains(&key) {
            return String::new(); // cycle guard
        }
        let Some(raw) = fn_root.get(&key).cloned() else {
            return String::new();
        };
        if native_tags().contains(raw.as_str()) {
            return raw;
        }
        if let Some(m) = TERNARY_RE.captures(&raw) {
            if native_tags().contains(&m[2]) {
                return m[2].to_string();
            }
            return String::new();
        }
        // root references another component → same-file fn, else imported
        let Some(ti) = by_name.get(file).copied() else {
            return String::new();
        };
        let mut next = seen.clone();
        next.insert(key);
        for c in &irs[ti].components {
            if c.fn_name == raw {
                return resolve_root(irs, by_name, fn_root, file, &raw, &mut next);
            }
        }
        hint_for(irs, by_name, fn_root, &irs[ti], &raw, &mut next)
    }

    fn hint_for(
        irs: &[CvFile],
        by_name: &HashMap<String, usize>,
        fn_root: &HashMap<String, String>,
        ir: &CvFile,
        tag: &str,
        seen: &mut HashSet<String>,
    ) -> String {
        if native_tags().contains(tag) {
            return tag.to_string();
        }
        if let Some(m) = TERNARY_RE.captures(tag) {
            if native_tags().contains(&m[2]) {
                return m[2].to_string();
            }
            return String::new();
        }
        if is_icon_name(tag, &ir.icons) {
            return "svg".to_string();
        }
        // app-side icon helper (@/app/... import, renders an svg placeholder)
        if let Some(mod_) = ir.meta_module_of.get(tag) {
            if mod_.starts_with("@/app/") {
                return "svg".to_string();
            }
        }
        if let Some(tv) = ir.meta_tag_vars.get(tag) {
            return tv.clone();
        }
        for c in &ir.components {
            if c.fn_name == tag {
                return resolve_root(irs, by_name, fn_root, &ir.name, tag, seen);
            }
        }
        let mod_opt = ir.meta_module_of.get(tag).cloned();
        if let Some(mod_) = mod_opt {
            if !mod_.is_empty() {
                static EXT_RE: OnceLock<Regex> = OnceLock::new();
                let ext = EXT_RE.get_or_init(|| Regex::new(r"\.[tj]sx?$").unwrap());
                let stem = ext
                    .replace_all(&mod_[mod_.rfind('/').map(|i| i + 1).unwrap_or(0)..], "")
                    .into_owned();
                if let Some(di) = by_name.get(stem.as_str()).copied() {
                    // registry files export the component under the imported name
                    let mut imported_name = tag.to_string();
                    if let Some(im) = ir.meta_import.get(tag) {
                        imported_name = im.clone();
                    }
                    let found = irs[di]
                        .components
                        .iter()
                        .any(|c| c.fn_name == imported_name);
                    if !found {
                        return String::new();
                    }
                    return resolve_root(irs, by_name, fn_root, &stem, &imported_name, seen);
                }
                if tag.contains('.') {
                    return external_member_tag(tag);
                }
                return String::new();
            }
        }
        if tag.contains('.') {
            return external_member_tag(tag);
        }
        String::new()
    }

    let mut unresolved: Vec<String> = Vec::new();
    for ir_idx in 0..irs.len() {
        let ir = &irs[ir_idx];
        let mut hints = JsonObj::new();
        for c in &ir.components {
            for el in &c.elements {
                let raw_tag = match &el[0].1 {
                    Json::Str(s) => s.clone(),
                    _ => continue,
                };
                if native_tags().contains(raw_tag.as_str()) || raw_tag.starts_with("<ternary:") {
                    continue;
                }
                if hints.index_of(&raw_tag) < 0 {
                    let h = hint_for(irs, &by_name, &fn_root, ir, &raw_tag, &mut HashSet::new());
                    if h.is_empty() {
                        // external-tier files are tombstones (react-day-picker
                        // etc.) — their framework tags are unresolvable by
                        // design; only flag tiers the emitter actually renders
                        if ir.tier != "external" {
                            unresolved.push(format!("{}:{}: {}", ir.name, c.fn_name, raw_tag));
                        }
                    } else {
                        hints = hints.add(&raw_tag, Json::Str(h));
                    }
                }
            }
        }
        irs[ir_idx].tag_hints = hints.into_pairs();
    }
    if !unresolved.is_empty() {
        return Err(format!(
            "tagHints unresolved ({}):\n  {}",
            unresolved.len(),
            unresolved.join("\n  ")
        ));
    }
    Ok(())
}

// -------------------------------------------------------------- json assembly

impl CvFile {
    pub fn json(&self) -> Vec<(String, Json)> {
        let mut comps: Vec<Json> = Vec::new();
        for c in &self.components {
            let els: Vec<Json> = c.elements.iter().map(|e| Json::Obj(e.clone())).collect();
            comps.push(Json::Obj(
                JsonObj::new()
                    .add("fn", Json::Str(c.fn_name.clone()))
                    .add("export", Json::Bool(c.is_export))
                    .add("elements", Json::Arr(els))
                    .into_pairs(),
            ));
        }
        let mut out = JsonObj::new();
        out = out.add("schema", Json::Int(2));
        out = out.add(
            "source",
            Json::Obj(
                JsonObj::new()
                    .add(
                        "commit",
                        match &self.pin_commit {
                            Some(c) => Json::Str(c.clone()),
                            None => Json::Null,
                        },
                    )
                    .into_pairs(),
            ),
        );
        out = out.add("name", Json::Str(self.name.clone()));
        out = out.add("tier", Json::Str(self.tier.clone()));
        out = out.add(
            "imports",
            Json::Arr(self.imports.iter().map(|s| Json::Str(s.clone())).collect()),
        );
        out = out.add(
            "icons",
            Json::Arr(self.icons.iter().map(|s| Json::Str(s.clone())).collect()),
        );
        out = out.add("cva", self.cva.json());
        out = out.add("components", Json::Arr(comps));
        out = out.add(
            "conditionals",
            Json::Arr(
                self.conditionals
                    .iter()
                    .map(|c| Json::Obj(c.clone()))
                    .collect(),
            ),
        );
        out = out.add(
            "cvaRefs",
            Json::Arr(self.cva_refs.iter().map(|c| Json::Obj(c.clone())).collect()),
        );
        out = out.add(
            "tagHints",
            Json::Obj(
                self.tag_hints
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            ),
        );
        out.into_pairs()
    }
}

// --------------------------------------------------------- the convert verb

/// pin.json → the pinned shadcn-ui commit (nil when absent).
fn pin_commit_of(pin_raw: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#""shadcn_ui"[\t\n\f\r ]*:[\t\n\f\r ]*\{[^{}]*"commit"[\t\n\f\r ]*:[\t\n\f\r ]*"([^"]+)"#).unwrap()
    });
    re.captures(pin_raw).map(|m| m[1].to_string())
}

/// tiers.json keys in file order + name→tier. serde_json's preserve_order
/// feature makes Map insertion-ordered, matching the Go ordered decoder.
fn cv_parse_tiers(raw: &str) -> Result<(Vec<String>, HashMap<String, String>), String> {
    let top: serde_json::Map<String, Value> =
        serde_json::from_str(raw).map_err(|e| e.to_string())?;
    let mut order = Vec::new();
    let mut out = HashMap::new();
    for (k, v) in &top {
        order.push(k.clone());
        if let Some(inner) = v.as_object() {
            if let Some(Value::String(tier)) = inner.get("tier") {
                out.insert(k.clone(), tier.clone());
            }
        }
    }
    Ok((order, out))
}

#[allow(dead_code)] // summary fields only some of which the rust path reads
struct CvSummary {
    name: String,
    tier: String,
    classes: usize,
    cond: usize,
}

fn cv_bump_pairs(pairs: &mut Vec<(String, Json)>, tier: &str, count: &HashMap<String, usize>) {
    if count[tier] == 1 {
        pairs.push((tier.to_string(), Json::Int(1)));
        return;
    }
    for p in pairs.iter_mut() {
        if p.0 == tier {
            p.1 = Json::Int(count[tier] as i64);
        }
    }
}

fn cv_compact_obj(pairs: &[(String, Json)]) -> String {
    let parts: Vec<String> = pairs
        .iter()
        .map(|(k, v)| {
            let n = match v {
                Json::Int(i) => i.to_string(),
                _ => String::new(),
            };
            format!("{}:{}", crate::jsonorder::json_string(k), n)
        })
        .collect();
    format!("{{{}}}", parts.join(","))
}

/// Compares the two dists the way JS did:
/// JSON.stringify(Object.entries(a).sort()) === …same for b.
fn cv_sorted_equal(a: &[(String, Json)], b: &[(String, Json)]) -> bool {
    let key = |pairs: &[(String, Json)]| -> Vec<String> {
        let mut keys: Vec<String> = pairs
            .iter()
            .map(|(k, v)| {
                let n = match v {
                    Json::Int(i) => i.to_string(),
                    _ => String::new(),
                };
                format!("{},{}", crate::jsonorder::json_string(k), n)
            })
            .collect();
        keys.sort();
        keys
    };
    key(a) == key(b)
}

pub fn run_convert() -> Result<(), String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let pin_raw = std::fs::read_to_string(root.join(CV_PIN_FILE)).map_err(|e| e.to_string())?;
    let pin_commit = pin_commit_of(&pin_raw);
    let tiers_raw = std::fs::read_to_string(root.join(CV_TIERS)).map_err(|e| e.to_string())?;
    // tiers.json key order is load-bearing for the wantDist insertion order
    let (tiers_order, tier_by_name) = cv_parse_tiers(&tiers_raw)?;
    std::fs::create_dir_all(root.join(CV_OUT)).map_err(|e| e.to_string())?;

    let ui_dir = root.join(CV_UI);
    let mut files: Vec<String> = Vec::new();
    for e in std::fs::read_dir(&ui_dir).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        let name = e.file_name().to_string_lossy().into_owned();
        if name.ends_with(".tsx") {
            files.push(name.trim_end_matches(".tsx").to_string());
        }
    }
    files.sort();
    if files.len() != 61 {
        eprintln!("FAIL expected 61 files, found {}", files.len());
        return Err(format!("expected 61 files, found {}", files.len()));
    }

    // global cva variant keys + cross-file cva registry
    let mut src_of: HashMap<String, String> = HashMap::new();
    let mut js_of: HashMap<String, String> = HashMap::new();
    for name in &files {
        let b = std::fs::read_to_string(ui_dir.join(format!("{}.tsx", name)))
            .map_err(|e| e.to_string())?;
        let js = esbuild_tsx(&root, &b)?;
        src_of.insert(name.clone(), b);
        js_of.insert(name.clone(), js);
    }
    let (mut reg, global_keys) = cva::cv_build_reg(&files, &js_of);
    reg.pin_commit = pin_commit.clone();
    let all_src_joined = files
        .iter()
        .map(|n| src_of[n].as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    static BLOCK_RE: OnceLock<Regex> = OnceLock::new();
    static LINE_RE: OnceLock<Regex> = OnceLock::new();
    static RAWQ_RE: OnceLock<Regex> = OnceLock::new();
    static STR_RE: OnceLock<Regex> = OnceLock::new();
    static CMPLIT_RE: OnceLock<Regex> = OnceLock::new();
    static CN_RE: OnceLock<Regex> = OnceLock::new();

    let mut dist_pairs: Vec<(String, Json)> = Vec::new();
    let mut dist_count: HashMap<String, usize> = HashMap::new();
    let mut summary: Vec<CvSummary> = Vec::new();
    let mut fail = false;
    let mut irs: Vec<CvFile> = Vec::new();
    for name in &files {
        let src = &src_of[name];
        let js = &js_of[name];

        // gate 1: babel string count == raw-text double-quote regex count
        let mut babel_count = 0usize;
        for sp in tsx::string_literals(src) {
            if !sp.template {
                babel_count += 1;
            }
        }
        babel_count += tsx::directive_spans(src).len();
        let block_re = BLOCK_RE.get_or_init(|| Regex::new(r"(?s)/\*[\s\S]*?\*/").unwrap());
        let mut stripped = block_re.replace_all(src, "").into_owned();
        let line_re = LINE_RE.get_or_init(|| Regex::new(r"(^|[^:])//[^\n]*").unwrap());
        stripped = line_re.replace_all(&stripped, "$1").into_owned();
        let rawq_re = RAWQ_RE.get_or_init(|| Regex::new(r#""(?:[^"\\]|\\.)*""#).unwrap());
        let raw_count = rawq_re.find_iter(&stripped).count();
        if babel_count != raw_count {
            eprintln!(
                "FAIL drift[{}]: babel strings {} != raw grep {}",
                name, babel_count, raw_count
            );
            fail = true;
        }

        let ir = match convert_file(&root, name, src, js, &mut reg) {
            Ok(ir) => ir,
            Err(e) => {
                eprintln!("FAIL convert[{}]: {}", name, e);
                fail = true;
                continue;
            }
        };
        let mut ir = ir;
        ir.pin_commit = pin_commit.clone();
        irs.push(ir);

        let ir = irs.last().expect("just pushed");
        // gate 2: every IR class string appears verbatim in source (with a
        // token-level fallback for arrays joined across source lines)
        let verbatim = |s: &str| src.contains(s) || all_src_joined.contains(s);
        let mut all_classes: Vec<String> = Vec::new();
        for c in &ir.components {
            for el in &c.elements {
                all_classes.extend(json_strings(&el[2].1));
            }
        }
        for tn in &ir.cva.names {
            let t = ir.cva.get(tn).expect("names come from tables");
            if !t.base.is_empty() {
                all_classes.push(t.base.clone());
            }
            for ax in &t.axes {
                for kv in &ax.values {
                    if let Json::Str(s) = &kv.v {
                        if !s.is_empty() {
                            all_classes.push(s.clone());
                        }
                    }
                }
            }
        }
        for c in &all_classes {
            let mut ok = verbatim(c);
            if !ok {
                ok = !c.is_empty() && c.split_whitespace().all(|tok| verbatim(tok));
            }
            if !ok {
                eprintln!(
                    "FAIL drift[{}]: class string not in source: {}",
                    name,
                    crate::jsonorder::json_string(c)
                );
                fail = true;
            }
        }

        // gate 2b (completeness, independent scanner): every quoted string
        // inside a cn(...) call in the raw source must be recorded in IR
        // classes (external-tier files are tombstones)
        if ir.tier != "external" {
            let mut ir_strings: HashSet<String> = HashSet::new();
            for c in &all_classes {
                ir_strings.insert(c.clone());
            }
            let cmplit_re =
                CMPLIT_RE.get_or_init(|| Regex::new(r#"[=!]==?\s*"((?:[^"\\]|\\.)*)""#).unwrap());
            let mut cmp_lits: HashSet<String> = HashSet::new();
            for m in cmplit_re.captures_iter(&stripped) {
                cmp_lits.insert(m[1].to_string());
            }
            for tn in &ir.cva.names {
                let t = ir.cva.get(tn).expect("names come from tables");
                for ax in &t.axes {
                    for kv in &ax.values {
                        cmp_lits.insert(kv.k.clone());
                    }
                }
                for dv in &t.defaults {
                    if let Json::Str(s) = &dv.v {
                        cmp_lits.insert(s.clone());
                    }
                }
            }
            for k in &global_keys {
                cmp_lits.insert(k.clone());
            }
            let cn_re = CN_RE.get_or_init(|| Regex::new(r"\bcn\s*\(").unwrap());
            let str_re = STR_RE.get_or_init(|| Regex::new(r#""((?:[^"\\]|\\.)*)""#).unwrap());
            for loc in cn_re.find_iter(&stripped) {
                let mut i = loc.end();
                let sb = stripped.as_bytes();
                let mut depth = 1;
                while i < sb.len() && depth > 0 {
                    if sb[i] == b'(' {
                        depth += 1;
                    } else if sb[i] == b')' {
                        depth -= 1;
                    }
                    i += 1;
                }
                let inner = &stripped[loc.end()..i - 1];
                for m in str_re.captures_iter(inner) {
                    let val = &m[1];
                    if !val.is_empty() && !cmp_lits.contains(val) && !ir_strings.contains(val) {
                        eprintln!(
                            "FAIL drift[{}]: cn string not in IR: {}",
                            name,
                            crate::jsonorder::json_string(val)
                        );
                        fail = true;
                    }
                }
            }
        } // tier !== external

        // gate 3: tier matches tiers.json
        match tier_by_name.get(name) {
            Some(want) if *want == ir.tier => {}
            want => {
                let w = want.cloned().unwrap_or_else(|| "undefined".to_string());
                eprintln!("FAIL tier[{}]: ir={} want={}", name, ir.tier, w);
                fail = true;
            }
        }
        *dist_count.entry(ir.tier.clone()).or_default() += 1;
        cv_bump_pairs(&mut dist_pairs, &ir.tier, &dist_count);
        summary.push(CvSummary {
            name: name.clone(),
            tier: ir.tier.clone(),
            classes: all_classes.len(),
            cond: ir.conditionals.len(),
        });
    }

    // cross-file tag hints (needs all IRs; throws on unresolved)
    if let Err(e) = build_tag_hints(&mut irs) {
        eprintln!("FAIL tagHints: {}", e);
        fail = true;
    }

    if !fail {
        for ir in &irs {
            let path = root.join(CV_OUT).join(format!("{}.json", ir.name));
            let rendered = marshal_js_step(&Json::Obj(ir.json()), "", " ");
            std::fs::write(path, rendered).map_err(|e| e.to_string())?;
        }
    }
    let mut want_count: HashMap<String, usize> = HashMap::new();
    let mut want_pairs: Vec<(String, Json)> = Vec::new();
    for n in &tiers_order {
        let tier = &tier_by_name[n];
        *want_count.entry(tier.clone()).or_default() += 1;
        cv_bump_pairs(&mut want_pairs, tier, &want_count);
    }
    if !cv_sorted_equal(&dist_pairs, &want_pairs) {
        eprintln!(
            "FAIL tier distribution: {} != {}",
            cv_compact_obj(&dist_pairs),
            cv_compact_obj(&want_pairs)
        );
        fail = true;
    }
    let cond_total: usize = summary.iter().map(|s| s.cond).sum();
    println!("convert: {} IR files -> {}", files.len(), CV_OUT);
    println!("tier dist: {}", cv_compact_obj(&dist_pairs));
    println!("conditionals total: {}", cond_total);
    if fail {
        println!("FAIL  convert drift gates");
        Err("convert drift gates failed".to_string())
    } else {
        println!("PASS  convert (0 drift, tiers match, tagHints resolved)");
        Ok(())
    }
}

fn json_strings(v: &Json) -> Vec<String> {
    match v {
        Json::Arr(items) => items
            .iter()
            .filter_map(|e| match e {
                Json::Str(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}
