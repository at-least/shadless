//! Port of pipeline/emitter_css.go — the Go port of src/emitter/css.mjs.
//! Faithful per branch; rule ORDER is observable in dist output.

use super::tags::{kebab, normalize_tag};
use crate::twmerge;
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;


/// Go's json.Unmarshal treats JSON null as "leave the field at its default";
/// serde rejects null for String/Vec/Map. Normalize by dropping null-valued
/// object keys and null array items before deserializing.
pub fn drop_nulls(v: &mut serde_json::Value) {
    match v {
        serde_json::Value::Object(m) => {
            let nulls: Vec<String> = m
                .iter()
                .filter(|(_, val)| val.is_null())
                .map(|(k, _)| k.clone())
                .collect();
            for k in nulls {
                m.remove(&k);
            }
            for (_, val) in m.iter_mut() {
                drop_nulls(val);
            }
        }
        serde_json::Value::Array(a) => {
            a.retain(|val| !val.is_null());
            for val in a.iter_mut() {
                drop_nulls(val);
            }
        }
        _ => {}
    }
}

// IR shape as css.mjs (serde preserves the JSON's
// insertion order for the ordered cva tables via preserve_order).
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CssIrComponent {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub imports: Vec<String>,
    #[serde(default)]
    pub cva: OrderedCva,
    #[serde(default)]
    pub components: Vec<IrFn>,
    #[serde(default)]
    pub conditionals: Vec<IrCond>,
    #[serde(default)]
    pub cva_refs: Vec<IrCvaRef>,
    #[serde(default)]
    pub tag_hints: HashMap<String, String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct IrFn {
    #[serde(rename = "fn", default)]
    pub fn_: String,
    #[serde(rename = "export", default)]
    pub export: bool,
    #[serde(default)]
    pub elements: Vec<IrEl>,
}

#[derive(Deserialize, Clone, Default)]
pub struct IrEl {
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub slot: String,
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub spread: bool,
    #[serde(default)]
    pub children: Vec<String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct IrCond {
    #[serde(default)]
    pub kind: String,
    #[serde(rename = "fn", default)]
    pub fn_: String,
    #[serde(default)]
    pub slot: Option<String>,
    #[serde(default)]
    pub then: String,
    #[serde(rename = "else", default)]
    pub else_: String,
    #[serde(default)]
    pub test: Option<IrCondTest>,
}

#[derive(Deserialize, Clone, Default)]
pub struct IrCondTest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub op: String,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

#[derive(Deserialize, Clone, Default)]
pub struct IrCvaRef {
    #[serde(default)]
    pub slot: String,
    #[serde(rename = "ref", default)]
    pub r#ref: String,
    #[serde(default)]
    pub table: CvaTable,
    #[serde(rename = "dyn", default)]
    pub dyn_: Vec<IrCvaDyn>,
    #[serde(rename = "dynAxes", default)]
    pub dyn_axes: Vec<String>,
    #[serde(default)]
    pub defaults: HashMap<String, String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct IrCvaDyn {
    #[serde(default)]
    pub attr: String,
    #[serde(default)]
    pub when: String,
    #[serde(default)]
    pub classes: String,
}

/// cvaTable keeps the JSON's insertion order for axes AND values: JS's
/// Object.entries iterates in source order, and the emitted rule ORDER is
/// observable in dist output. (serde_json preserve_order backs the IndexMap.)
#[derive(Deserialize, Clone, Default)]
pub struct CvaTable {
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub variants: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub defaults: HashMap<String, String>,
}

impl CvaTable {
    pub fn axis_order(&self) -> Vec<String> {
        self.variants.keys().cloned().collect()
    }
    pub fn values(&self, axis: &str) -> &serde_json::Map<String, serde_json::Value> {
        static E: OnceLock<serde_json::Map<String, serde_json::Value>> = OnceLock::new();
        self.variants
            .get(axis)
            .and_then(|v| v.as_object())
            .unwrap_or_else(|| E.get_or_init(serde_json::Map::new))
    }
    pub fn class_of(&self, axis: &str, val: &str) -> Option<String> {
        self.values(axis)
            .get(val)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }
}

/// orderedCva: ir.cva with table order preserved (Object.entries order).
#[derive(Deserialize, Clone, Default)]
pub struct OrderedCva {
    #[serde(flatten)]
    map: serde_json::Map<String, serde_json::Value>,
}

impl OrderedCva {
    pub fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }
    pub fn table(&self, k: &str) -> Option<CvaTable> {
        self.map.get(k).and_then(|v| {
            let mut t: CvaTable = serde_json::from_value(v.clone()).ok()?;
            t.defaults = HashMap::new();
            if let Some(obj) = v.as_object() {
                if let Some(d) = obj.get("defaults").and_then(|d| d.as_object()) {
                    for (k, val) in d {
                        if let Some(s) = val.as_str() {
                            t.defaults.insert(k.clone(), s.to_string());
                        }
                    }
                }
            }
            Some(t)
        })
    }
}

// ---- from css.mjs, verbatim in intent --------------------------------------

pub fn css_escape(t: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"[^A-Za-z0-9_-]").unwrap());
    re.replace_all(t, |m: &regex::Captures| format!("\\{}", &m[0]))
        .into_owned()
}

fn re_residue_text() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^text-(xs|sm|base|lg|xl|[0-9]xl)$").unwrap())
}
fn re_residue_size() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^size-").unwrap())
}
fn re_residue_text_leading() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(leading-|text-(xs|sm|base|lg|xl|[0-9]xl)$)").unwrap())
}
fn re_residue_size_w() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(w-|size-)").unwrap())
}
fn re_residue_size_h() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(h-|size-)").unwrap())
}

/// When twMerge dropped a base token whose extra properties (line-height on
/// text-sm, height on size-4) are not restated by the value, the value rule
/// gets an explicit reset.
pub fn residue_resets(base: &str, value: &str) -> Vec<String> {
    let merged: HashSet<String> = twmerge::merge(&format!("{} {}", base, value))
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let mut out: Vec<String> = Vec::new();
    let value_toks: Vec<&str> = value.split_whitespace().collect();
    for tok in base.split_whitespace() {
        if merged.contains(tok) {
            continue;
        }
        if re_residue_text().is_match(tok) {
            if !value_toks.iter().any(|t| re_residue_text_leading().is_match(t)) {
                out.push("leading-[inherit]".to_string());
            }
        } else if re_residue_size().is_match(tok) {
            if !value_toks.iter().any(|t| re_residue_size_w().is_match(t)) {
                out.push("w-auto".to_string());
            }
            if !value_toks.iter().any(|t| re_residue_size_h().is_match(t)) {
                out.push("h-auto".to_string());
            }
        }
    }
    dedup(&out)
}

/// The (optional) residue-reset rule and the main
/// `[data-slot=slot]:where(...)` rule for one cva axis value.
#[allow(clippy::type_complexity)]
pub fn residue_rule_parts(
    slot: &str,
    axis: &str,
    val: &str,
    def: Option<&str>,
    base_apply: &str,
    apply: &str,
) -> (String, bool, String) {
    let mut full = apply.to_string();
    let resets = residue_resets(base_apply, apply);
    let mut reset_rule = String::new();
    let mut has_reset = false;
    if !resets.is_empty() {
        full = format!("{} {}", apply, resets.join(" "));
        let causes: Vec<String> = apply
            .split_whitespace()
            .filter(|vt| !residue_resets(base_apply, vt).is_empty())
            .map(|s| s.to_string())
            .collect();
        if !causes.is_empty() {
            let mut sel = format!(".{}", css_escape(&causes[0]));
            for c in &causes[1..] {
                sel.push_str(&format!(", .{}", css_escape(c)));
            }
            reset_rule = format!(
                "  [data-slot=\"{}\"]:where({}) {{ @apply {}; }}",
                slot,
                sel,
                resets.join(" ")
            );
            has_reset = true;
        }
    }
    let mut main_rule = format!(
        "  [data-slot=\"{}\"]:where([data-{}=\"{}\"]) {{ @apply {}; }}",
        slot, axis, val, full
    );
    if def == Some(val) {
        main_rule = format!(
            "  [data-slot=\"{}\"]:where(:not([data-{}]), [data-{}=\"{}\"], [data-{}=\"\"]) {{ @apply {}; }}",
            slot, axis, axis, val, axis, full
        );
    }
    (reset_rule, has_reset, main_rule)
}

pub fn dedup(ss: &[String]) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for s in ss {
        if seen.insert(s.clone()) {
            out.push(s.clone());
        }
    }
    out
}

pub fn marker_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(group|peer)(/[0-9A-Za-z_-]+)?$").unwrap())
}

pub(crate) fn dead_utilities(t: &str) -> bool {
    t == "origin-top-center"
}

pub struct SplitMarkersOut {
    pub apply: String,
    pub markers: Vec<String>,
}

pub fn split_markers(s: &str) -> SplitMarkersOut {
    let mut apply_toks: Vec<String> = Vec::new();
    let mut markers: Vec<String> = Vec::new();
    for t in s.split_whitespace() {
        if marker_re().is_match(t) || super::skin_data().allowlist.contains(t) || dead_utilities(t)
        {
            markers.push(t.to_string());
        } else {
            apply_toks.push(t.to_string());
        }
    }
    SplitMarkersOut {
        apply: apply_toks.join(" "),
        markers,
    }
}

/// cvaSlot: naming convention (buttonVariants → Button) else a single fn whose
/// name starts with the stem. No slot → no rule.
pub fn cva_slot(ir: &CssIrComponent) -> HashMap<String, (CvaTable, String)> {
    let mut out: HashMap<String, (CvTable_, String)> = HashMap::new();
    for var_name in ir.cva.keys() {
        let table = ir.cva.table(&var_name).expect("keys come from map");
        let stem = var_name.strip_suffix("Variants").unwrap_or(&var_name);
        let mut found: Option<usize> = None;
        for (i, c) in ir.components.iter().enumerate() {
            if c.fn_.eq_ignore_ascii_case(stem) {
                found = Some(i);
                break;
            }
        }
        if found.is_none() {
            for (i, c) in ir.components.iter().enumerate() {
                if c.fn_.to_lowercase().starts_with(&stem.to_lowercase()) {
                    found = Some(i);
                    break;
                }
            }
        }
        let mut slot = String::new();
        if let Some(i) = found {
            for e in &ir.components[i].elements {
                if !e.slot.is_empty() {
                    slot = e.slot.clone();
                    break;
                }
            }
        }
        out.insert(var_name, (table, slot));
    }
    out
}
type CvTable_ = CvaTable;

pub fn re_clean_tag_prefix() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^<ternary:[^/]+/").unwrap())
}

pub fn clean_tag(t: &str) -> String {
    let t2 = re_clean_tag_prefix().replace_all(t, "");
    kebab(t2.trim_end_matches('>'))
}

// ---- componentCss -----------------------------------------------------------

pub struct ComponentCssOut {
    pub rules: Vec<String>,
    pub markers: HashMap<String, Vec<String>>,
    pub anchors: HashMap<String, String>,
    pub anchor_markers: HashMap<String, Vec<String>>,
    pub unlayered: Vec<String>,
}

pub fn component_css(ir: &CssIrComponent) -> Result<ComponentCssOut, String> {
    super::load_skin();
    let mut rules: Vec<String> = Vec::new();
    let cva_map = cva_slot(ir);
    let mut cva_slots: HashSet<String> = HashSet::new();
    for (_, (_, slot)) in cva_map.iter() {
        if !slot.is_empty() {
            cva_slots.insert(slot.clone());
        }
    }
    let mut markers: HashMap<String, Vec<String>> = HashMap::new();
    let mut anchors: HashMap<String, String> = HashMap::new();
    let mut anchors_order: Vec<String> = Vec::new();
    let mut anchor_markers: HashMap<String, Vec<String>> = HashMap::new();
    let mut late_anchor_rules: Vec<String> = Vec::new();
    let mut used_tokens: HashSet<String> = HashSet::new();
    let token = |t: &str, used: &mut HashSet<String>| -> String {
        let mut out = t.to_string();
        let mut n = 1;
        while used.contains(&out) {
            n += 1;
            out = format!("{}-{}", t, n);
        }
        used.insert(out.clone());
        out
    };
    let mut token = |t: &str| -> String { token(t, &mut used_tokens) };

    // pass 1: group elements — slotted (by slot) / slotless (anchors)
    let mut by_slot: HashMap<String, Vec<(IrEl, String)>> = HashMap::new();
    let mut by_slot_order: Vec<String> = Vec::new();
    for c in &ir.components {
        for (idx, el) in c.elements.iter().enumerate() {
            if el.classes.is_empty() {
                continue;
            }
            let key = format!("{}#{}", c.fn_, idx);
            if !el.slot.is_empty() && !cva_slots.contains(&el.slot) {
                if !by_slot.contains_key(&el.slot) {
                    by_slot_order.push(el.slot.clone());
                }
                by_slot
                    .entry(el.slot.clone())
                    .or_default()
                    .push((el.clone(), key));
            } else if el.slot.is_empty() {
                let base = if !c.elements.is_empty()
                    && c.elements[0].tag == el.tag
                    && idx == 0
                {
                    kebab(&c.fn_)
                } else {
                    format!("{}-{}", kebab(&c.fn_), clean_tag(&el.tag))
                };
                anchors.insert(key.clone(), token(&base));
                anchors_order.push(key);
            }
        }
    }

    // conditional class branches (class-cond with a readable test)
    struct CondBranch {
        then: String,
        r#else: String,
        test: IrCondTest,
    }
    let mut cond_branches: HashMap<String, Vec<CondBranch>> = HashMap::new();
    let mut root_slot = String::new();
    for c in &ir.components {
        for e in &c.elements {
            if !e.slot.is_empty() {
                root_slot = e.slot.clone();
                break;
            }
        }
        if !root_slot.is_empty() {
            break;
        }
    }
    for cond in &ir.conditionals {
        if cond.kind != "class-cond" || cond.test.is_none() {
            continue;
        }
        let Some(c) = find_fn(ir, &cond.fn_) else {
            continue;
        };
        for (idx, el) in c.elements.iter().enumerate() {
            if contains_tok(&el.classes, &cond.then) && contains_tok(&el.classes, &cond.else_) {
                let key = format!("{}#{}", cond.fn_, idx);
                cond_branches.entry(key).or_default().push(CondBranch {
                    then: cond.then.clone(),
                    r#else: cond.else_.clone(),
                    test: cond.test.clone().expect("checked above"),
                });
            }
        }
    }

    let strip_branches = |el: &IrEl, key: &str| -> String {
        let empty: Vec<CondBranch> = Vec::new();
        let branches = cond_branches.get(key).unwrap_or(&empty);
        let keep: Vec<String> = el
            .classes
            .iter()
            .filter(|c| !branches.iter().any(|cd| c.as_str() == cd.then || c.as_str() == cd.r#else))
            .cloned()
            .collect();
        keep.join(" ")
    };

    // branchRules emits the twin :is()/absent-default rule pair per
    // conditional on this element.
    let branch_rules = |selector: &str, key: &str| -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let empty: Vec<CondBranch> = Vec::new();
        for cond in cond_branches.get(key).unwrap_or(&empty) {
            let t = &cond.test;
            let attr = format!("data-{}", kebab(&t.name));
            let v = &t.value;
            let (explicit_true, explicit_false) = if t.op == "===" {
                (cond.then.clone(), cond.r#else.clone())
            } else {
                (cond.r#else.clone(), cond.then.clone())
            };
            let absent_is_true = match &t.default {
                None => true,
                Some(d) if d.is_string() => {
                    (d.as_str() == Some(v.as_str())) == (t.op == "===")
                }
                Some(_) => true,
            };
            let (ctx, ctx_other) = if !root_slot.is_empty() {
                (
                    format!(", [data-slot=\"{}\"][{}=\"{}\"] *", root_slot, attr, v),
                    format!(
                        ", [data-slot=\"{}\"][{}]:not([{}=\"{}\"]) *",
                        root_slot, attr, attr, v
                    ),
                )
            } else {
                (String::new(), String::new())
            };
            let is_v = format!(":is([{}=\"{}\"]{})", attr, v, ctx);
            let is_other = format!(":is([{}]:not([{}=\"{}\"]){})", attr, attr, v, ctx_other);
            let tm = split_markers(&explicit_true);
            let fm = split_markers(&explicit_false);
            let not_inline = branch_not_inline(&tm.apply, &fm.apply);
            let mut true_sel = format!("{}:where(", selector);
            if absent_is_true {
                true_sel.push_str(&format!(":not({})", &is_other[4..is_other.len() - 1]));
            } else {
                true_sel.push_str(&is_v);
            }
            true_sel.push_str(&not_inline);
            true_sel.push(')');
            let mut false_sel = format!("{}:where(", selector);
            if absent_is_true {
                false_sel.push_str(&is_other);
            } else {
                false_sel.push_str(&format!(":not({})", &is_v[4..is_v.len() - 1]));
            }
            false_sel.push_str(&not_inline);
            false_sel.push(')');
            if !tm.apply.is_empty() {
                out.push(format!("  {} {{ @apply {}; }}", true_sel, tm.apply));
            }
            if !fm.apply.is_empty() {
                out.push(format!("  {} {{ @apply {}; }}", false_sel, fm.apply));
            }
        }
        out
    };

    // anchor rules — in INSERTION order (JS Map)
    for key in &anchors_order {
        let t = anchors[key].clone();
        let (fn_name, idx) = split_anchor_key(key);
        let Some(c) = find_fn(ir, &fn_name) else {
            continue;
        };
        if idx >= c.elements.len() {
            continue;
        }
        let el = &c.elements[idx];
        let s = split_markers(&strip_branches(el, key));
        if !s.markers.is_empty() {
            anchor_markers.insert(key.clone(), s.markers.clone());
        }
        if !s.apply.is_empty() {
            rules.push(format!("  .{} {{ @apply {}; }}", t, s.apply));
        }
        rules.extend(branch_rules(&format!(".{}", t), key));
    }

    // plain-class slot rules — INSERTION order (JS Map ordering)
    for slot in &by_slot_order {
        let items = &by_slot[slot];
        struct SigInfo {
            tags: HashSet<String>,
            // the JS twin emits the FIRST tag the signature saw; a HashSet
            // pick would be per-process random here (RandomState), and the
            // multi-tag-signature shape would flip bytes run to run
            first_tag: String,
            markers: Vec<String>,
        }
        let mut sigs: HashMap<String, SigInfo> = HashMap::new();
        let mut sig_order: Vec<String> = Vec::new();
        for item in items {
            let s = split_markers(&strip_branches(&item.0, &item.1));
            if !sigs.contains_key(&s.apply) {
                sigs.insert(s.apply.clone(), SigInfo {
                    tags: HashSet::new(),
                    first_tag: String::new(),
                    markers: Vec::new(),
                });
                sig_order.push(s.apply.clone());
            }
            let info = sigs.get_mut(&s.apply).unwrap();
            let tag = normalize_tag(&item.0.tag, &ir.tag_hints).unwrap_or_else(|| "?".to_string());
            if info.first_tag.is_empty() {
                info.first_tag = tag.clone();
            }
            info.tags.insert(tag);
            info.markers.extend(s.markers.iter().cloned());
        }
        let mut all_markers: Vec<String> = Vec::new();
        for it in items {
            let s = split_markers(&it.0.classes.join(" "));
            all_markers.extend(s.markers);
        }
        if !all_markers.is_empty() {
            markers.insert(slot.clone(), dedup(&all_markers));
        }
        if sig_order.len() == 1 {
            let apply = &sig_order[0];
            if !apply.is_empty() {
                rules.push(format!("  [data-slot=\"{}\"] {{ @apply {}; }}", slot, apply));
            }
            for it in items {
                rules.extend(branch_rules(
                    &format!("[data-slot=\"{}\"]", slot),
                    &it.1,
                ));
            }
            continue;
        }
        // conflicting class sets on one slot
        let mut tag_overlap = false;
        for info in sigs.values() {
            for tag in &info.tags {
                if tag == "?" {
                    tag_overlap = true;
                    break;
                }
                for other in sigs.values() {
                    if !std::ptr::eq(other, info) && other.tags.contains(tag) {
                        tag_overlap = true;
                        break;
                    }
                }
            }
        }
        if !tag_overlap {
            for apply in &sig_order {
                let info = &sigs[apply];
                let tag = info.first_tag.clone();
                if !apply.is_empty() {
                    rules.push(format!(
                        "  [data-slot=\"{}\"]:is({}) {{ @apply {}; }}",
                        slot, tag, apply
                    ));
                }
            }
            continue;
        }
        let mut token_lists: Option<Vec<Vec<String>>> = Some(Vec::new());
        for apply in &sig_order {
            let toks: Vec<String> = apply.split_whitespace().map(|s| s.to_string()).collect();
            if toks.is_empty() {
                token_lists = None;
                break;
            }
            token_lists.as_mut().expect("some").push(toks);
        }
        let Some(token_lists) = token_lists else {
            continue;
        };
        let common = intersect_strings(&token_lists);
        if !common.is_empty() {
            rules.push(format!(
                "  [data-slot=\"{}\"] {{ @apply {}; }}",
                slot,
                common.join(" ")
            ));
        }
        for it in items {
            let own: Vec<String> = split_markers(&it.0.classes.join(" "))
                .apply
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let rest: Vec<String> = own
                .iter()
                .filter(|t| !contains_tok(&common, t))
                .cloned()
                .collect();
            if rest.is_empty() {
                continue;
            }
            let tk = token(slot);
            anchors.insert(it.1.clone(), tk.clone());
            anchors_order.push(it.1.clone());
            late_anchor_rules.push(format!("  .{} {{ @apply {}; }}", tk, rest.join(" ")));
        }
    }

    // cva rules — variants and axes iterate in JSON insertion order
    rules.append(&mut late_anchor_rules);
    let cva_names = ir.cva.keys();
    for var_name in &cva_names {
        let (cv_table, cv_slot) = &cva_map[var_name];
        if cv_slot.is_empty() {
            continue;
        }
        let s = split_markers(&cv_table.base);
        if !s.markers.is_empty() {
            markers.insert(cv_slot.clone(), s.markers.clone());
        }
        if !s.apply.is_empty() {
            rules.push(format!(
                "  [data-slot=\"{}\"] {{ @apply {}; }}",
                cv_slot, s.apply
            ));
        }
        let base_apply = s.apply;
        for axis in cv_table.axis_order() {
            let def = cv_table.defaults.get(&axis).cloned();
            for val in cv_table.values(&axis).keys().cloned().collect::<Vec<_>>() {
                let Some(cls) = cv_table.class_of(&axis, &val) else {
                    continue;
                };
                if cls.is_empty() {
                    continue;
                }
                let t = split_markers(&cls);
                if !t.markers.is_empty() {
                    markers.entry(cv_slot.clone()).or_default().extend(t.markers.iter().cloned());
                }
                if t.apply.is_empty() {
                    continue;
                }
                let (reset_rule, has_reset, main_rule) = residue_rule_parts(
                    cv_slot,
                    &axis,
                    &val,
                    def.as_deref(),
                    &base_apply,
                    &t.apply,
                );
                if has_reset {
                    rules.push(reset_rule);
                }
                rules.push(main_rule);
            }
        }
    }

    // cross-file cva refs
    for r in &ir.cva_refs {
        for d in &r.dyn_ {
            rules.push(format!(
                "  [data-slot=\"{}\"][{}=\"{}\"] {{ @apply {}; }}",
                r.slot, d.attr, d.when, d.classes
            ));
        }
        for axis in &r.dyn_axes {
            let vals = r.table.values(axis);
            let def = r.defaults.get(axis).cloned();
            let base_apply = split_markers(&r.table.base).apply;
            for val in vals.keys().cloned().collect::<Vec<_>>() {
                let Some(cls) = r.table.class_of(axis, &val) else {
                    continue;
                };
                if cls.is_empty() {
                    continue;
                }
                let t = split_markers(&cls);
                if t.apply.is_empty() {
                    continue;
                }
                if def.is_none() {
                    rules.push(format!(
                        "  [data-slot=\"{}\"][data-{}=\"{}\"] {{ @apply {}; }}",
                        r.slot, axis, val, t.apply
                    ));
                    continue;
                }
                let (reset_rule, has_reset, main_rule) = residue_rule_parts(
                    &r.slot,
                    axis,
                    &val,
                    def.as_deref(),
                    &base_apply,
                    &t.apply,
                );
                rules.push(main_rule);
                if has_reset {
                    rules.push(reset_rule);
                }
            }
        }
    }

    // allowlisted skin markers that DO have a skin body must emit their own
    // class-anchored rule
    let skin = super::skin_data();
    let mut used_allowlist: Vec<String> = Vec::new();
    for ms in markers.values() {
        for t in ms {
            if skin.allowlist.contains(t) && skin.map.contains_key(t) {
                used_allowlist.push(t.clone());
            }
        }
    }
    for ms in anchor_markers.values() {
        for t in ms {
            if skin.allowlist.contains(t) && skin.map.contains_key(t) {
                used_allowlist.push(t.clone());
            }
        }
    }
    used_allowlist.sort();
    used_allowlist.dedup();
    let unlayered: Vec<String> = used_allowlist
        .iter()
        .map(|t| format!(".{} {{ @apply {}; }}", t, skin.map[t]))
        .collect();

    Ok(ComponentCssOut {
        rules: re_apply_merge(&rules),
        markers,
        anchors,
        anchor_markers,
        unlayered: re_apply_merge(&unlayered),
    })
}

/// wrapComponentCss mirrors css.mjs's wrapComponentCss — one layer block plus
/// an optional unlayered skin-marker block.
pub fn wrap_component_css(name: &str, css: &ComponentCssOut) -> String {
    let mut parts = vec![format!(
        "/* {} */\n@layer components {{\n{}\n}}",
        name,
        css.rules.join("\n")
    )];
    if !css.unlayered.is_empty() {
        parts.push(format!(
            "/* {}: skin markers (unlayered, as upstream ships them) */\n{}",
            name,
            css.unlayered.join("\n")
        ));
    }
    parts.join("\n")
}

pub fn re_apply_merge(rules: &[String]) -> Vec<String> {
    static APPLY_MERGE: OnceLock<Regex> = OnceLock::new();
    let re = APPLY_MERGE.get_or_init(|| Regex::new(r"@apply ([^;]+);").unwrap());
    rules
        .iter()
        .map(|r| {
            re.replace_all(r, |m: &regex::Captures| {
                format!("@apply {};", twmerge::merge(&m[1]))
            })
            .into_owned()
        })
        .collect()
}

// ---- helpers ----------------------------------------------------------------

pub fn find_fn<'a>(ir: &'a CssIrComponent, name: &str) -> Option<&'a IrFn> {
    ir.components.iter().find(|c| c.fn_ == name)
}

pub fn split_anchor_key(key: &str) -> (String, usize) {
    let i = key.rfind('#').expect("anchor key carries #");
    let n: usize = key[i + 1..].parse().unwrap_or(0);
    (key[..i].to_string(), n)
}

pub fn contains_tok(ss: &[String], s: &str) -> bool {
    ss.iter().any(|x| x == s)
}

pub fn intersect_strings(lists: &[Vec<String>]) -> Vec<String> {
    if lists.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<String> = Vec::new();
    for t in &lists[0] {
        let mut everywhere = true;
        for l in &lists[1..] {
            if !contains_tok(l, t) {
                everywhere = false;
                break;
            }
        }
        if everywhere {
            out.push(t.clone());
        }
    }
    out
}

/// The ":not(.token)…" chain that suppresses a branch rule when the element
/// already carries either branch inline (or its logical twin).
pub fn branch_not_inline(t_apply: &str, f_apply: &str) -> String {
    let mut toks: Vec<String> = Vec::new();
    let mut all: Vec<String> = t_apply
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    all.extend(f_apply.split_whitespace().map(|s| s.to_string()));
    for t in dedup(&all) {
        toks.extend(twins_of(&t));
    }
    let mut parts: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for t in toks {
        if t.is_empty() {
            continue;
        }
        for sh in shadows_of(&t) {
            if seen.insert(sh.clone()) {
                parts.push(sh);
            }
        }
    }
    parts.join("")
}

/// Physical/logical spacing twins (pl-4 ↔ ps-4 in an RTL box).
fn twins_of(tok: &str) -> Vec<String> {
    static RE_TWIN: OnceLock<Regex> = OnceLock::new();
    let re = RE_TWIN.get_or_init(|| Regex::new(r"^(-?)(p|m|inset|scroll-p|scroll-m)(l|r|s|e)-(.+)$").unwrap());
    let Some(m) = re.captures(tok) else {
        return vec![tok.to_string()];
    };
    let alt = match &m[3] {
        "l" => "s",
        "r" => "e",
        "s" => "l",
        "e" => "r",
        _ => unreachable!(),
    };
    vec![
        tok.to_string(),
        format!("{}{}{}-{}", &m[1], &m[2], alt, &m[4]),
    ]
}

/// A branch utility is suppressed when a utility from the same spacing/inset
/// group is already inline, so the :not() uses group-prefix shape.
fn shadows_of(tok: &str) -> Vec<String> {
    static RE_SHADOW: OnceLock<Regex> = OnceLock::new();
    let re = RE_SHADOW.get_or_init(|| Regex::new(r"^(-?)(p|m|inset|top|right|bottom|left|start|end)([tblrsexy])?-").unwrap());
    let Some(m) = re.captures(tok) else {
        return vec![format!(":not(.{})", css_escape(tok))];
    };
    let prefix = m.get(0).unwrap().as_str();
    vec![
        format!(":not([class^=\"{}\"])", prefix),
        format!(":not([class*=\" {}\"])", prefix),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One apply-signature spanning two tags beside a disjoint one: the
    /// `:is(tag)` selector used to come from a HashSet pick — per-process
    /// random. The same IR must emit one stable selector, the way the JS
    /// twin (first tag seen) does.
    #[test]
    fn unit_cva_sig_tag_pick_is_deterministic() {
        let ir = CssIrComponent {
            name: "t".into(),
            tier: "static".into(),
            components: vec![IrFn {
                fn_: "T".into(),
                export: true,
                elements: vec![
                    IrEl { tag: "button".into(), slot: "t".into(), classes: vec!["px-2".into()], ..Default::default() },
                    IrEl { tag: "a".into(), slot: "t".into(), classes: vec!["px-2".into()], ..Default::default() },
                    IrEl { tag: "span".into(), slot: "t".into(), classes: vec!["px-3".into()], ..Default::default() },
                ],
            }],
            ..Default::default()
        };
        let mut seen = std::collections::HashSet::new();
        for _ in 0..20 {
            let out = component_css(&ir).unwrap();
            let sel: Vec<&String> = out.rules.iter().filter(|r| r.contains("px-2")).collect();
            assert_eq!(sel.len(), 1, "exactly one px-2 rule");
            seen.insert(sel[0].clone());
        }
        assert_eq!(seen.len(), 1, "one stable selector, got {seen:?}");
    }
}
