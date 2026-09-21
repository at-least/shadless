//! cva tables of the downgraded output (top-level `const x = cva(…)`
//! declarators) and the cross-file cva registry.

use super::scan::*;
use crate::jsonorder::{Json, JsonObj};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default)]
pub struct CvTable {
    pub base: String,
    pub axes: Vec<CvaAxis>,
    pub defaults: Vec<CvaKv>, // ordered; value: string | bool | Raw (undefined omitted)
}

#[derive(Clone, Debug, Default)]
pub struct CvaAxis {
    pub axis: String,
    pub values: Vec<CvaKv>, // ordered; value: string ("" = legitimate no-classes)
}

#[derive(Clone, Debug)]
pub struct CvaKv {
    pub k: String,
    pub v: Json,
}

impl CvTable {
    pub fn json(&self) -> Json {
        let mut variants = JsonObj::new();
        for a in &self.axes {
            let mut vo = JsonObj::new();
            for kv in &a.values {
                vo = vo.add(&kv.k, kv.v.clone());
            }
            variants = variants.add(&a.axis, Json::from_obj(vo));
        }
        let mut defaults = JsonObj::new();
        for kv in &self.defaults {
            defaults = defaults.add(&kv.k, kv.v.clone());
        }
        let mut o = JsonObj::new();
        o = o
            .add("base", Json::Str(self.base.clone()))
            .add("variants", Json::from_obj(variants))
            .add("defaults", Json::from_obj(defaults));
        Json::from_obj(o)
    }

    /// `(table.defaults ??= {})[axis] = val` — append-only, because the
    /// mutation runs only when the axis is absent.
    pub fn set_default(&mut self, axis: &str, val: &str) {
        self.defaults.push(CvaKv {
            k: axis.to_string(),
            v: Json::Str(val.to_string()),
        });
    }

    pub fn has_default(&self, axis: &str) -> Option<String> {
        for kv in &self.defaults {
            if kv.k == axis {
                return match &kv.v {
                    Json::Str(s) => Some(s.clone()),
                    _ => Some(String::new()),
                };
            }
        }
        None
    }

    pub fn value_for(&self, axis: &str, val: &str) -> Option<String> {
        for a in &self.axes {
            if a.axis != axis {
                continue;
            }
            for kv in &a.values {
                if kv.k == val {
                    return match &kv.v {
                        Json::Str(s) => Some(s.clone()),
                        _ => Some(String::new()),
                    };
                }
            }
        }
        None
    }

    pub fn has_axis(&self, axis: &str) -> bool {
        self.axes.iter().any(|a| a.axis == axis)
    }
}

#[derive(Clone)]
pub struct CvTables {
    pub names: Vec<String>,
    by_name: HashMap<String, CvTable>,
}

impl Default for CvTables {
    fn default() -> Self {
        Self::new()
    }
}

impl CvTables {
    pub fn new() -> Self {
        CvTables {
            names: Vec::new(),
            by_name: HashMap::new(),
        }
    }
    pub fn add(&mut self, name: &str, tb: CvTable) {
        if !self.by_name.contains_key(name) {
            self.names.push(name.to_string());
        }
        self.by_name.insert(name.to_string(), tb);
    }
    pub fn get(&self, name: &str) -> Option<&CvTable> {
        self.by_name.get(name)
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut CvTable> {
        self.by_name.get_mut(name)
    }
    pub fn json(&self) -> Json {
        let mut o = JsonObj::new();
        for n in &self.names {
            o = o.add(n, self.by_name[n].json());
        }
        Json::from_obj(o)
    }
}

/// Collects the top-level `const NAME = cva(base, cfg)` tables of a downgraded
/// module — plain (non-exported) consts included, matching babel's take()
/// over every top-level VariableDeclaration.
pub fn cv_tables_of(js: &str) -> CvTables {
    let mut ts = CvTables::new();
    let b = js.as_bytes();
    let (mut depth, mut i) = (0i32, 0usize);
    while i < b.len() {
        let e = cv_mask_end(js, i);
        if e > 0 {
            i = e;
            continue;
        }
        let c = b[i];
        if c == b'(' || c == b'[' || c == b'{' {
            depth += 1;
            i += 1;
            continue;
        }
        if c == b')' || c == b']' || c == b'}' {
            depth -= 1;
            i += 1;
            continue;
        }
        if depth > 0 {
            i += 1;
            continue;
        }
        let (w, l) = cv_word_at(js, i);
        if w == "const" || w == "let" || w == "var" {
            let end = cv_skip_stmt(js, i);
            let mut seg_end = end;
            if seg_end > i && b[seg_end - 1] == b';' {
                seg_end -= 1;
            }
            for dcl in super::topscan::cv_split_top_indexed(&js[i + l..seg_end]) {
                let text = dcl.text.trim().to_string();
                let (name, nlen) = cv_word_at(&text, 0);
                if name.is_empty() {
                    continue;
                }
                let rest = text[nlen..].trim().to_string();
                if !rest.starts_with('=') {
                    continue;
                }
                let init = cv_parse_expr(&rest[1..]);
                if init.kind != "call" || init.ident != "cva" || init.args.is_empty() {
                    continue;
                }
                let mut tb = CvTable::default();
                let a0 = cv_parse_expr(&init.args[0]);
                if a0.kind == "str" {
                    tb.base = a0.str;
                }
                if init.args.len() > 1 {
                    let cfg = cv_parse_expr(&init.args[1]);
                    if cfg.kind == "obj" {
                        for p in &cfg.obj {
                            if p.quoted || p.spread {
                                continue;
                            }
                            let pv = cv_parse_expr(&p.val);
                            if p.key == "variants" && pv.kind == "obj" {
                                for ax in &pv.obj {
                                    let av = cv_parse_expr(&ax.val);
                                    if av.kind != "obj" {
                                        continue;
                                    }
                                    let mut entry = CvaAxis {
                                        axis: ax.key.clone(),
                                        values: Vec::new(),
                                    };
                                    for vv in &av.obj {
                                        entry.values.push(CvaKv {
                                            k: vv.key.clone(),
                                            v: Json::Str(cv_val_of_string(&vv.val)),
                                        });
                                    }
                                    tb.axes.push(entry);
                                }
                            }
                            if p.key == "defaultVariants" && pv.kind == "obj" {
                                for dv in &pv.obj {
                                    if let Some(v) = cv_default_value(&dv.val) {
                                        tb.defaults.push(CvaKv {
                                            k: dv.key.clone(),
                                            v,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                ts.add(&name, tb);
            }
            i = end;
            continue;
        }
        i += 1;
    }
    ts
}

/// A cva variant value node — "lit" | ["a","b"] (join) | ""
pub fn cv_val_of_string(text: &str) -> String {
    let n = cv_parse_expr(text);
    if n.kind == "str" {
        return n.str;
    }
    let t = text.trim();
    if t.starts_with('[') {
        if let Some(e) = cv_match_bracket(t, 0, b'[', b']') {
            let mut parts: Vec<String> = Vec::new();
            for el in cv_split_top(&t[1..e]) {
                let n2 = cv_parse_expr(&el);
                if n2.kind == "str" && !n2.str.is_empty() {
                    parts.push(n2.str);
                }
            }
            return parts.join(" ");
        }
    }
    String::new()
}

/// str(dv.value) ?? dv.value?.value — non-string literals keep their value
/// (number/bool); anything else (undefined, templates, identifiers) omits the
/// key entirely.
pub fn cv_default_value(text: &str) -> Option<Json> {
    let n = cv_parse_expr(text);
    match n.kind.as_str() {
        "str" => Some(Json::Str(n.str)),
        "num" => Some(Json::Raw(n.str)),
        "bool" => Some(Json::Bool(text.trim() == "true")),
        _ => None,
    }
}

// ------------------------------------------------------- cross-file registry

pub struct CompCvaEntry {
    pub cva_name: String,
    pub table: CvTable,
    pub file: String,
}

pub struct CvaExportEntry {
    pub file: String,
    pub table: CvTable,
}

pub struct CvReg {
    pub pin_commit: Option<String>,
    pub cva_by_export: HashMap<String, CvaExportEntry>,
    pub comp_cva: HashMap<String, CompCvaEntry>,
}

impl Default for CvReg {
    fn default() -> Self {
        Self::new()
    }
}

impl CvReg {
    pub fn new() -> Self {
        CvReg {
            pin_commit: None,
            cva_by_export: HashMap::new(),
            comp_cva: HashMap::new(),
        }
    }
}

/// cvExportedNames — collectExportedNames on the downgraded module: export
/// spec names + fn/arrow declarations carrying the export keyword.
pub fn cv_exported_names(js: &str) -> Result<HashSet<String>, String> {
    let top = super::topscan::scan_top_js(js)?;
    let mut out: HashSet<String> = top.exported_names.clone();
    for d in &top.decls {
        if d.exported && (d.is_fn || d.is_arrow) {
            out.insert(d.name.clone());
        }
    }
    Ok(out)
}

/// Every cva( call in the module at any depth (babel's walk saw them all), as
/// absolute callee positions.
pub fn cv_cva_call_positions(js: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let b = js.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let e = cv_mask_end(js, i);
        if e > 0 {
            i = e;
            continue;
        }
        if b[i] == b'c' && js[i..].starts_with("cva(") {
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
            out.push(i);
            i += 4;
            continue;
        }
        i += 1;
    }
    out
}

/// The config argument text of the cva( call at pos (its second top-level
/// argument), or "".
pub fn cv_cva_cfg_at(js: &str, pos: usize) -> String {
    let open = pos + "cva".len();
    let close = match cv_match_bracket(js, open, b'(', b')') {
        Some(c) => c,
        None => return String::new(),
    };
    let args = cv_split_top(&js[open + 1..close]);
    if args.len() < 2 {
        return String::new();
    }
    args[1].trim().to_string()
}

/// The cross-file cva registry every convertFile shares: global cva variant
/// keys (cross-file: pagination uses button's variants), exported cva tables
/// (cvaByExport) and their convention-named components (buttonVariants ↔
/// Button).
pub fn cv_build_reg(files: &[String], js_of: &HashMap<String, String>) -> (CvReg, HashSet<String>) {
    let mut global_keys: HashSet<String> = HashSet::new();
    let mut reg = CvReg::new();
    for name in files {
        let js = &js_of[name];
        for pos in cv_cva_call_positions(js) {
            let cfg = cv_cva_cfg_at(js, pos);
            if cfg.is_empty() {
                continue;
            }
            let n = cv_parse_expr(&cfg);
            if n.kind == "obj" {
                for p in &n.obj {
                    if p.quoted || p.spread || p.key != "variants" {
                        continue;
                    }
                    let pv = cv_parse_expr(&p.val);
                    if pv.kind == "obj" {
                        for ax in &pv.obj {
                            let av = cv_parse_expr(&ax.val);
                            if av.kind == "obj" {
                                for vv in &av.obj {
                                    global_keys.insert(vv.key.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        let exported = match cv_exported_names(js) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let tables = cv_tables_of(js);
        for cva_name in tables.names.clone() {
            if !exported.contains(&cva_name) {
                continue;
            }
            let table = tables
                .get(&cva_name)
                .expect("names come from tables")
                .clone();
            reg.cva_by_export.insert(
                cva_name.clone(),
                CvaExportEntry {
                    file: name.clone(),
                    table: table.clone(),
                },
            );
            let stem = cva_name.strip_suffix("Variants").unwrap_or(&cva_name);
            let mut comp_name = stem.to_string();
            if let Some(first) = comp_name.get_mut(0..1) {
                let up = first.to_uppercase();
                comp_name.replace_range(0..1, &up);
            }
            if exported.contains(&comp_name) {
                reg.comp_cva.insert(
                    comp_name,
                    CompCvaEntry {
                        cva_name: cva_name.clone(),
                        table: table.clone(),
                        file: name.clone(),
                    },
                );
            }
        }
    }
    (reg, global_keys)
}
