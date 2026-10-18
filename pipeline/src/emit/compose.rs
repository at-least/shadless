//! The class composer shared by path-parity and the docs code-tab
//! projection: for one IR element, the token string React's cn() renders
//! (cva base + per-axis selected-or-default value classes + context-cva
//! defaults + conditionals resolved by their default branch, through
//! twMerge). Hoisted verbatim out of tools/path_parity.rs (which built
//! synthetic per-slot pages with them); the projection feeds the same
//! functions the real demo DOM's slot + data-* attributes instead.

use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::emit::css::{CssIrComponent, CvaTable, IrEl, IrFn};

// upstream @custom-variant bodies
const SHORTHAND: [(&str, &str, &str); 8] = [
    ("data-open", "data-state", "open"),
    ("data-closed", "data-state", "closed"),
    ("data-checked", "data-state", "checked"),
    ("data-unchecked", "data-state", "unchecked"),
    ("data-active", "data-state", "active"),
    ("data-selected", "data-selected", "true"),
    ("data-disabled", "data-disabled", "true"),
    ("data-horizontal", "data-orientation", "horizontal"),
    // note: data-vertical is in the Go map too — see below
];

static RE_CHILD_SLOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"data-\[slot=([0-9A-Za-z_-]+)\]").unwrap());
static RE_COMBINATOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(group|peer|has|in)-").unwrap());
static RE_ATTR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(data|aria)-\[([0-9A-Za-z_-]+)(?:=([0-9A-Za-z_-]+))?\]$").unwrap()
});
static RE_ARIA: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^aria-(expanded|invalid|checked|disabled|pressed|selected|current)$").unwrap()
});
static RE_DATA_BARE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^data-(inset|highlighted|empty|pressed|autoscrolling|popup-open)$").unwrap()
});
static RE_NOT_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^not-.*$").unwrap());

pub fn child_slots(cls: &str) -> Vec<String> {
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for m in RE_CHILD_SLOT.captures_iter(cls) {
        let s = m[1].to_string();
        if seen.insert(s.clone()) {
            out.push(s);
        }
    }
    out
}

/// inlineClasses: React evaluates a conditional; keep the branch the
/// default selects, then twMerge the surviving list.
pub fn inline_classes(ir: &CssIrComponent, fn_: &str, el: &IrEl) -> String {
    let mut drop: std::collections::HashSet<String> = std::collections::HashSet::new();
    for c in &ir.conditionals {
        if c.kind != "class-cond" || c.fn_ != fn_ || c.test.is_none() {
            continue;
        }
        if !crate::emit::css::contains_tok(&el.classes, &c.then)
            || !crate::emit::css::contains_tok(&el.classes, &c.else_)
        {
            continue;
        }
        let test = c.test.as_ref().unwrap();
        let truthy = match &test.default {
            None => true,
            Some(serde_json::Value::String(d)) => (d == &test.value) == (test.op == "==="),
            Some(_) => true,
        };
        if truthy {
            drop.insert(c.else_.clone());
        } else {
            drop.insert(c.then.clone());
        }
    }
    let keep: Vec<String> = el
        .classes
        .iter()
        .filter(|c| !drop.contains(*c))
        .cloned()
        .collect();
    crate::twmerge::merge(&keep.join(" "))
}

pub fn state_configs(cls: &str) -> Vec<(String, String)> {
    let mut out: std::collections::HashMap<String, (String, String)> =
        std::collections::HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for tok in cls.split_whitespace() {
        let mut segs: Vec<&str> = tok.split(':').collect();
        segs.pop();
        let mut skip = false;
        for v in &segs {
            if *v == "*" || *v == "**" || RE_COMBINATOR.is_match(v) {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }
        for v in segs {
            let bare = RE_NOT_PREFIX.replace_all(v, "");
            if bare.is_empty() || bare.starts_with("data-[slot=") {
                continue;
            }
            if let Some(sh) = SHORTHAND
                .iter()
                .chain([&("data-vertical", "data-orientation", "vertical")])
                .find(|(k, _, _)| *k == bare)
            {
                let k = format!("{}={}", sh.1, sh.2);
                if !out.contains_key(&k) {
                    out.insert(k.clone(), (sh.1.to_string(), sh.2.to_string()));
                    order.push(k);
                }
                continue;
            }
            if let Some(m) = RE_ATTR.captures(&bare) {
                let axis = m[2].to_string();
                if axis == "variant" || axis == "size" {
                    continue;
                }
                let mut val = m.get(3).map(|v| v.as_str()).unwrap_or("").to_string();
                if val.is_empty() {
                    val = "true".to_string();
                }
                let k = format!("{}-{}={}", &m[1], axis, val);
                if !out.contains_key(&k) {
                    out.insert(k.clone(), (format!("{}-{}", &m[1], axis), val));
                    order.push(k);
                }
                continue;
            }
            if let Some(m) = RE_ARIA.captures(&bare) {
                let k = format!("aria-{}=true", &m[1]);
                if !out.contains_key(&k) {
                    out.insert(k.clone(), (format!("aria-{}", &m[1]), "true".to_string()));
                    order.push(k);
                }
                continue;
            }
            if let Some(m) = RE_DATA_BARE.captures(&bare) {
                let k = format!("data-{}=", &m[1]);
                if !out.contains_key(&k) {
                    out.insert(k.clone(), (format!("data-{}", &m[1]), String::new()));
                    order.push(k);
                }
            }
        }
    }
    order.iter().map(|k| out[k].clone()).collect()
}

pub fn el_of_slot<'a>(ir: &'a CssIrComponent, slot: &str) -> Option<(&'a IrFn, &'a IrEl)> {
    for c in &ir.components {
        for el in &c.elements {
            if el.slot == slot {
                return Some((c, el));
            }
        }
    }
    None
}

pub fn tag_of(ir: &CssIrComponent, el: &IrEl) -> String {
    crate::emit::tags::normalize_tag(&el.tag, &ir.tag_hints).unwrap_or_else(|| "div".to_string())
}

pub fn classes_of(
    ir: &CssIrComponent,
    cva_slots: &HashMap<String, CvaTable>,
    c: &IrFn,
    el: &IrEl,
    sel: &HashMap<String, String>,
) -> String {
    let table = cva_slots.get(&el.slot);
    if table.is_none() {
        let mut ctx_defaults: Vec<String> = Vec::new();
        for r in &ir.cva_refs {
            if r.slot != el.slot {
                continue;
            }
            for ax in &r.dyn_axes {
                let Some(d) = r.defaults.get(ax) else {
                    continue;
                };
                let mut key = sel.get(ax).cloned().unwrap_or_default();
                if key.is_empty() {
                    key = d.clone();
                }
                if let Some(v) = r.table.class_of(ax, &key)
                    && !v.is_empty()
                {
                    ctx_defaults.push(v);
                }
            }
        }
        let base = inline_classes(ir, &c.fn_, el);
        if !ctx_defaults.is_empty() {
            let mut all = vec![base];
            all.extend(ctx_defaults);
            return crate::twmerge::merge(&all.join(" "));
        }
        return base;
    }
    let table = table.unwrap();
    let mut values: std::collections::HashSet<String> = std::collections::HashSet::new();
    for axis in table.axis_order() {
        for v in table.values(&axis).keys() {
            if let Some(cls) = table.class_of(&axis, v) {
                values.insert(cls);
            }
        }
    }
    let mut extras: Vec<String> = Vec::new();
    for x in &el.classes {
        if *x != table.base && !values.contains(x) {
            extras.push(x.clone());
        }
    }
    let mut list = vec![table.base.clone()];
    for axis in table.axis_order() {
        let mut key = sel.get(&axis).cloned().unwrap_or_default();
        if key.is_empty() {
            key = table.defaults.get(&axis).cloned().unwrap_or_default();
        }
        if let Some(v) = table.class_of(&axis, &key)
            && !v.is_empty()
        {
            list.push(v);
        }
    }
    list.extend(extras);
    crate::twmerge::merge(&list.join(" "))
}
