//! Port of pipeline/ir_diff.go — slot-level semantic diff between two IR sets.
//! See the Go source for the contract; byte-identity with the Go binary is the
//! acceptance bar.

use crate::emit::tw::find_repo_root;
use crate::jsonorder::{marshal_js, Json, JsonObj};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Deserialize)]
struct IrComponent {
    #[serde(default)]
    tier: String,
    #[serde(default)]
    components: Vec<IrComponentEntry>,
    #[serde(default)]
    cva: HashMap<String, IrCvaTable>,
}

#[derive(Deserialize)]
struct IrComponentEntry {
    #[serde(default)]
    elements: Vec<IrElement>,
}

#[derive(Deserialize)]
struct IrElement {
    #[serde(default)]
    slot: Option<String>,
    #[serde(default)]
    classes: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct IrCvaTable {
    #[serde(default)]
    base: Value,
    #[serde(default)]
    variants: HashMap<String, HashMap<String, Value>>,
    #[serde(default)]
    defaults: HashMap<String, Value>,
}

type IrSet = HashMap<String, IrComponent>;

fn load_ir_from_dir(dir: &Path) -> IrSet {
    let mut out = IrSet::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if !name.ends_with(".json") {
            continue;
        }
        let Ok(b) = std::fs::read(dir.join(&name)) else {
            continue;
        };
        if let Ok(c) = serde_json::from_slice::<IrComponent>(&b) {
            out.insert(name.trim_end_matches(".json").to_string(), c);
        }
    }
    out
}

fn load_ir_from_git(root: &Path, ref_: &str) -> IrSet {
    let mut out = IrSet::new();
    let Ok(ls) = Command::new("git")
        .args(["ls-tree", "--name-only", ref_, "generated/ir/"])
        .current_dir(root)
        .output()
    else {
        return out;
    };
    let listing = String::from_utf8_lossy(&ls.stdout).into_owned();
    for f in listing.split('\n') {
        if !f.ends_with(".json") {
            continue;
        }
        let Ok(show) = Command::new("git")
            .args(["show", &format!("{}:{}", ref_, f)])
            .current_dir(root)
            .output()
        else {
            continue;
        };
        if let Ok(c) = serde_json::from_slice::<IrComponent>(&show.stdout) {
            let name = f.rsplit('/').next().unwrap_or(f);
            out.insert(name.trim_end_matches(".json").to_string(), c);
        }
    }
    out
}

/// orderedSet keeps insertion order, which is what the JS Set iteration gave
/// and therefore what the added/removed lists print.
#[derive(Default)]
struct OrderedSet {
    order: Vec<String>,
    seen: HashSet<String>,
}

impl OrderedSet {
    fn new() -> Self {
        Self::default()
    }
    fn add(&mut self, x: String) {
        if self.seen.insert(x.clone()) {
            self.order.push(x);
        }
    }
}

fn set_diff(a: &OrderedSet, b: &OrderedSet) -> (Vec<String>, Vec<String>) {
    let mut added = Vec::new();
    for x in &b.order {
        if !a.seen.contains(x) {
            added.push(x.clone());
        }
    }
    let mut removed = Vec::new();
    for x in &a.order {
        if !b.seen.contains(x) {
            removed.push(x.clone());
        }
    }
    (added, removed)
}

/// slotsOf maps slot -> the union of class tokens declared on it.
fn slots_of(ir: &IrComponent) -> (HashMap<String, OrderedSet>, Vec<String>) {
    let mut m: HashMap<String, OrderedSet> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for c in &ir.components {
        for e in &c.elements {
            let Some(slot) = &e.slot else { continue };
            if slot.is_empty() {
                continue;
            }
            if !m.contains_key(slot) {
                m.insert(slot.clone(), OrderedSet::new());
                order.push(slot.clone());
            }
            let joined = e.classes.as_deref().unwrap_or(&[]).join(" ");
            for tok in joined.split_whitespace() {
                m.get_mut(slot).expect("inserted above").add(tok.to_string());
            }
        }
    }
    (m, order)
}

/// jsString mirrors JS String(): the comparison the predecessor made on cva
/// values, which may legitimately be absent.
fn js_string(v: &Value) -> String {
    match v {
        Value::Null => "undefined".to_string(),
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.to_string()
            } else {
                format!("{}", n.as_f64().unwrap_or(0.0))
            }
        }
        other => format!("{}", other),
    }
}

#[derive(Default)]
struct IrChange {
    what: String,
    slot: String,
    table: String,
    axis: String,
    value: String,
    from: String,
    to: String,
    added: Option<Vec<String>>,
    removed: Option<Vec<String>>,
}

struct IrEntry {
    kind: String, // added | removed | changed
    tier: String,
    changes: Vec<IrChange>,
}

fn sorted_keys<V>(m: &HashMap<String, V>) -> Vec<String> {
    let mut out: Vec<String> = m.keys().cloned().collect();
    out.sort();
    out
}

fn sorted_set(m: &HashSet<String>) -> Vec<String> {
    let mut out: Vec<String> = m.iter().cloned().collect();
    out.sort();
    out
}

/// diffIr returns only the components that changed.
fn diff_ir(before: &IrSet, after: &IrSet) -> (Vec<String>, HashMap<String, IrEntry>) {
    let mut names: HashSet<String> = before.keys().cloned().collect();
    names.extend(after.keys().cloned());
    let mut order: Vec<String> = Vec::new();
    let mut components: HashMap<String, IrEntry> = HashMap::new();
    for name in sorted_set(&names) {
        let a = before.get(&name);
        let b = after.get(&name);
        if a.is_none() {
            order.push(name.clone());
            components.insert(
                name,
                IrEntry {
                    kind: "added".to_string(),
                    tier: b.expect("in after").tier.clone(),
                    changes: Vec::new(),
                },
            );
            continue;
        }
        if b.is_none() {
            order.push(name.clone());
            components.insert(
                name,
                IrEntry {
                    kind: "removed".to_string(),
                    tier: a.expect("in before").tier.clone(),
                    changes: Vec::new(),
                },
            );
            continue;
        }
        let a = a.expect("in before");
        let b = b.expect("in after");
        let mut d = IrEntry {
            kind: "changed".to_string(),
            tier: String::new(),
            changes: Vec::new(),
        };
        if a.tier != b.tier {
            d.changes.push(IrChange {
                what: "tier".to_string(),
                from: a.tier.clone(),
                to: b.tier.clone(),
                ..Default::default()
            });
        }
        let (sa, order_a) = slots_of(a);
        let (sb, order_b) = slots_of(b);
        for s in &order_b {
            if !sa.contains_key(s) {
                d.changes.push(IrChange {
                    what: "slot-added".to_string(),
                    slot: s.clone(),
                    ..Default::default()
                });
            }
        }
        for s in &order_a {
            if !sb.contains_key(s) {
                d.changes.push(IrChange {
                    what: "slot-removed".to_string(),
                    slot: s.clone(),
                    ..Default::default()
                });
            }
        }
        for s in &order_a {
            if !sb.contains_key(s) {
                continue;
            }
            let (added, removed) = set_diff(&sa[s], &sb[s]);
            if !added.is_empty() || !removed.is_empty() {
                d.changes.push(IrChange {
                    what: "classes".to_string(),
                    slot: s.clone(),
                    added: Some(added),
                    removed: Some(removed),
                    ..Default::default()
                });
            }
        }
        for tbl in sorted_keys(&b.cva) {
            if !a.cva.contains_key(&tbl) {
                d.changes.push(IrChange {
                    what: "cva-added".to_string(),
                    table: tbl.clone(),
                    ..Default::default()
                });
            }
        }
        for tbl in sorted_keys(&a.cva) {
            if !b.cva.contains_key(&tbl) {
                d.changes.push(IrChange {
                    what: "cva-removed".to_string(),
                    table: tbl.clone(),
                    ..Default::default()
                });
            }
        }
        for tbl in sorted_keys(&a.cva) {
            let Some(cb) = b.cva.get(&tbl) else { continue };
            let ca = &a.cva[&tbl];
            for ax in sorted_keys(&cb.variants) {
                if !ca.variants.contains_key(&ax) {
                    d.changes.push(IrChange {
                        what: "cva-axis-added".to_string(),
                        table: tbl.clone(),
                        axis: ax.clone(),
                        added: Some(sorted_keys(&cb.variants[&ax])),
                        ..Default::default()
                    });
                }
            }
            for ax in sorted_keys(&ca.variants) {
                if !cb.variants.contains_key(&ax) {
                    d.changes.push(IrChange {
                        what: "cva-axis-removed".to_string(),
                        table: tbl.clone(),
                        axis: ax.clone(),
                        ..Default::default()
                    });
                }
            }
            for ax in sorted_keys(&ca.variants) {
                let Some(vb) = cb.variants.get(&ax) else { continue };
                let va = &ca.variants[&ax];
                let mut s_a = OrderedSet::new();
                let mut s_b = OrderedSet::new();
                for k in sorted_keys(va) {
                    s_a.add(k.clone());
                }
                for k in sorted_keys(vb) {
                    s_b.add(k.clone());
                }
                let (added, removed) = set_diff(&s_a, &s_b);
                if !added.is_empty() || !removed.is_empty() {
                    d.changes.push(IrChange {
                        what: "cva-values".to_string(),
                        table: tbl.clone(),
                        axis: ax.clone(),
                        added: Some(added),
                        removed: Some(removed),
                        ..Default::default()
                    });
                }
                for v in sorted_keys(va) {
                    if let Some(bv) = vb.get(&v) {
                        if js_string(&va[&v]) != js_string(bv) {
                            d.changes.push(IrChange {
                                what: "cva-value-classes".to_string(),
                                table: tbl.clone(),
                                axis: ax.clone(),
                                value: v.clone(),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            let mut axes: HashSet<String> = ca.defaults.keys().cloned().collect();
            axes.extend(cb.defaults.keys().cloned());
            for ax in sorted_set(&axes) {
                let da = ca.defaults.get(&ax).unwrap_or(&Value::Null);
                let db = cb.defaults.get(&ax).unwrap_or(&Value::Null);
                if js_string(da) != js_string(db) {
                    d.changes.push(IrChange {
                        what: "cva-default".to_string(),
                        table: tbl.clone(),
                        axis: ax.clone(),
                        from: js_string(da),
                        to: js_string(db),
                        ..Default::default()
                    });
                }
            }
            if js_string(&ca.base) != js_string(&cb.base) {
                d.changes.push(IrChange {
                    what: "cva-base".to_string(),
                    table: tbl.clone(),
                    ..Default::default()
                });
            }
        }
        if !d.changes.is_empty() {
            order.push(name.clone());
            components.insert(name, d);
        }
    }
    (order, components)
}

fn signed(added: &[String], removed: &[String], limit: usize) -> String {
    let mut parts: Vec<String> = Vec::new();
    for x in added {
        parts.push(format!("+{}", x));
    }
    for x in removed {
        parts.push(format!("-{}", x));
    }
    if limit > 0 && parts.len() > limit {
        parts.truncate(limit);
    }
    parts.join(" ")
}

fn render_ir_diff(order: &[String], components: &HashMap<String, IrEntry>) -> String {
    if order.is_empty() {
        return "no semantic change in the IR".to_string();
    }
    let mut lines: Vec<String> = Vec::new();
    for name in order {
        let d = &components[name];
        if d.kind != "changed" {
            lines.push(format!(
                "{:<20} {} (tier {})",
                name,
                d.kind.to_uppercase(),
                d.tier
            ));
            continue;
        }
        lines.push(name.clone());
        for c in &d.changes {
            let s = match c.what.as_str() {
                "classes" => format!(
                    "classes[{}] +{} -{}: {}",
                    c.slot,
                    c.added.as_ref().map_or(0, |v| v.len()),
                    c.removed.as_ref().map_or(0, |v| v.len()),
                    signed(
                        c.added.as_deref().unwrap_or(&[]),
                        c.removed.as_deref().unwrap_or(&[]),
                        8
                    )
                ),
                "cva-values" => format!(
                    "cva {}.{}: {}",
                    c.table,
                    c.axis,
                    signed(
                        c.added.as_deref().unwrap_or(&[]),
                        c.removed.as_deref().unwrap_or(&[]),
                        0
                    )
                ),
                "cva-default" => format!(
                    "cva {}.{} default {} -> {}",
                    c.table, c.axis, c.from, c.to
                ),
                "cva-value-classes" => {
                    format!("cva {}.{}={} classes changed", c.table, c.axis, c.value)
                }
                "tier" => format!("tier {} -> {}", c.from, c.to),
                _ => {
                    let mut tail = if !c.slot.is_empty() {
                        c.slot.clone()
                    } else {
                        c.table.clone()
                    };
                    if !c.axis.is_empty() {
                        tail = format!("{}.{}", tail, c.axis);
                    }
                    format!("{} {}", c.what, tail).trim().to_string()
                }
            };
            lines.push(format!("  {}", s));
        }
    }
    lines.join("\n")
}

pub fn run_ir_diff(args: &[String]) -> i32 {
    let as_json = args.iter().any(|a| a == "--json");
    let pos: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with("--"))
        .cloned()
        .collect();
    if pos.is_empty() {
        eprintln!("usage: pipeline ir-diff <git-ref> | <dirA> <dirB> [--json]");
        return 2;
    }
    let wd = std::env::current_dir().unwrap_or_default();
    let root = match find_repo_root(&wd) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ir-diff: {}", e);
            return 1;
        }
    };
    let resolve = |p: &str| -> PathBuf {
        let p = Path::new(p);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            root.join(p)
        }
    };
    let before = if pos.len() > 1 {
        load_ir_from_dir(&resolve(&pos[0]))
    } else {
        load_ir_from_git(&root, &pos[0])
    };
    let mut after = load_ir_from_dir(&resolve("generated/ir"));
    if pos.len() > 1 {
        after = load_ir_from_dir(&resolve(&pos[1]));
    }
    let (order, components) = diff_ir(&before, &after);

    if !as_json {
        println!("{}", render_ir_diff(&order, &components));
        return 0;
    }
    let mut obj = JsonObj::new();
    for name in &order {
        let d = &components[name];
        let mut e = JsonObj::new().add("kind", Json::Str(d.kind.clone()));
        if d.kind != "changed" {
            e = e.add("tier", Json::Str(d.tier.clone()));
        } else {
            let mut changes: Vec<Json> = Vec::with_capacity(d.changes.len());
            for c in &d.changes {
                let mut co = JsonObj::new().add("what", Json::Str(c.what.clone()));
                for (k, v) in [
                    ("slot", &c.slot),
                    ("table", &c.table),
                    ("axis", &c.axis),
                    ("value", &c.value),
                    ("from", &c.from),
                    ("to", &c.to),
                ] {
                    if !v.is_empty() {
                        co = co.add(k, Json::Str(v.clone()));
                    }
                }
                if let Some(a) = &c.added {
                    co = co.add(
                        "added",
                        Json::Arr(a.iter().map(|s| Json::Str(s.clone())).collect()),
                    );
                }
                if let Some(r) = &c.removed {
                    co = co.add(
                        "removed",
                        Json::Arr(r.iter().map(|s| Json::Str(s.clone())).collect()),
                    );
                }
                changes.push(Json::from_obj(co));
            }
            e = e.add("changes", Json::Arr(changes));
        }
        obj = obj.add(name, Json::from_obj(e));
    }
    println!(
        "{}",
        marshal_js(&Json::from_obj(
            JsonObj::new().add("components", Json::from_obj(obj))
        ))
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn must_ir_component(src: &str) -> IrComponent {
        serde_json::from_str(src).expect("bad fixture json")
    }

    /// wantChange is the subset of IrChange a case cares about; unset string
    /// fields compare against "" and unset slices against nil-or-empty.
    struct WantChange {
        what: &'static str,
        slot: &'static str,
        table: &'static str,
        axis: &'static str,
        value: &'static str,
        from: &'static str,
        to: &'static str,
        added: Option<Vec<&'static str>>,
        removed: Option<Vec<&'static str>>,
    }

    impl WantChange {
        fn matches(&self, c: &IrChange) -> bool {
            let same = |a: &Option<Vec<String>>, b: &Option<Vec<&'static str>>| match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => a.iter().map(String::as_str).eq(b.iter().copied()),
                // Go's sameStrSlice treats nil and empty as equal
                (Some(a), None) => a.is_empty(),
                (None, Some(b)) => b.is_empty(),
            };
            self.what == c.what
                && self.slot == c.slot
                && self.table == c.table
                && self.axis == c.axis
                && self.value == c.value
                && self.from == c.from
                && self.to == c.to
                && same(&c.added, &self.added)
                && same(&c.removed, &self.removed)
        }
    }

    /// diffIr end to end: a component added, a component removed, and a
    /// component that changed in every way the algorithm tracks.
    #[test]
    fn diff_ir_end_to_end() {
        let mut before = IrSet::new();
        before.insert(
            "widget".to_string(),
            must_ir_component(
                r#"{
                "tier": "core",
                "components": [{"elements": [
                    {"slot": "root", "classes": ["flex", "gap-2"]},
                    {"slot": "icon", "classes": ["size-4"]}
                ]}],
                "cva": {
                    "variant": {
                        "base": "inline-flex",
                        "variants": {
                            "size": {"sm": "h-8", "lg": "h-10"},
                            "tone": {"solid": "bg-black"}
                        },
                        "defaults": {"size": "sm"}
                    },
                    "color": {"base": "", "variants": {"c": {"red": "text-red"}}, "defaults": {}}
                }
            }"#,
            ),
        );
        before.insert(
            "gone".to_string(),
            must_ir_component(r#"{"tier": "core", "components": [], "cva": {}}"#),
        );
        let mut after = IrSet::new();
        after.insert(
            "widget".to_string(),
            must_ir_component(
                r#"{
                "tier": "extended",
                "components": [{"elements": [
                    {"slot": "root", "classes": ["flex", "px-2"]},
                    {"slot": "label", "classes": ["text-sm"]}
                ]}],
                "cva": {
                    "variant": {
                        "base": "inline-flex",
                        "variants": {
                            "size": {"sm": "h-9", "lg": "h-10", "xl": "h-12"},
                            "state": {"active": "opacity-100"}
                        },
                        "defaults": {"size": "lg"}
                    },
                    "shape": {"base": "", "variants": {"s": {"round": "rounded"}}, "defaults": {}}
                }
            }"#,
            ),
        );
        after.insert(
            "fresh".to_string(),
            must_ir_component(r#"{"tier": "core", "components": [], "cva": {}}"#),
        );

        let (order, components) = diff_ir(&before, &after);

        assert_eq!(order, vec!["fresh", "gone", "widget"], "sorted order");
        let fresh = &components["fresh"];
        assert_eq!(fresh.kind, "added");
        assert_eq!(fresh.tier, "core");
        let gone = &components["gone"];
        assert_eq!(gone.kind, "removed");
        assert_eq!(gone.tier, "core");

        let w = &components["widget"];
        assert_eq!(w.kind, "changed");
        let want: Vec<WantChange> = vec![
            WantChange { what: "tier", slot: "", table: "", axis: "", value: "", from: "core", to: "extended", added: None, removed: None },
            WantChange { what: "slot-added", slot: "label", table: "", axis: "", value: "", from: "", to: "", added: None, removed: None },
            WantChange { what: "slot-removed", slot: "icon", table: "", axis: "", value: "", from: "", to: "", added: None, removed: None },
            WantChange { what: "classes", slot: "root", table: "", axis: "", value: "", from: "", to: "", added: Some(vec!["px-2"]), removed: Some(vec!["gap-2"]) },
            WantChange { what: "cva-added", slot: "", table: "shape", axis: "", value: "", from: "", to: "", added: None, removed: None },
            WantChange { what: "cva-removed", slot: "", table: "color", axis: "", value: "", from: "", to: "", added: None, removed: None },
            WantChange { what: "cva-axis-added", slot: "", table: "variant", axis: "state", value: "", from: "", to: "", added: Some(vec!["active"]), removed: None },
            WantChange { what: "cva-axis-removed", slot: "", table: "variant", axis: "tone", value: "", from: "", to: "", added: None, removed: None },
            WantChange { what: "cva-values", slot: "", table: "variant", axis: "size", value: "", from: "", to: "", added: Some(vec!["xl"]), removed: None },
            WantChange { what: "cva-value-classes", slot: "", table: "variant", axis: "size", value: "sm", from: "", to: "", added: None, removed: None },
            WantChange { what: "cva-default", slot: "", table: "variant", axis: "size", value: "", from: "sm", to: "lg", added: None, removed: None },
        ];
        assert_eq!(
            w.changes.len(),
            want.len(),
            "widget.Changes has {} entries, want {}: {:?}",
            w.changes.len(),
            want.len(),
            w.changes
                .iter()
                .map(|c| c.what.clone())
                .collect::<Vec<_>>()
        );
        for (i, c) in w.changes.iter().enumerate() {
            assert!(
                want[i].matches(c),
                "change[{}] = {:?}, want {:?}",
                i,
                (c.what.clone(), c.slot.clone(), c.table.clone(), c.axis.clone()),
                want[i].what
            );
        }
    }

    /// setDiff must follow INSERTION order, not sorted order.
    #[test]
    fn set_diff_follows_insertion_order() {
        let mut a = OrderedSet::new();
        for x in ["shared", "z", "q"] {
            a.add(x.to_string());
        }
        let mut b = OrderedSet::new();
        for x in ["b", "shared", "a"] {
            b.add(x.to_string());
        }
        let (added, removed) = set_diff(&a, &b);
        assert_eq!(added, vec!["b", "a"], "insertion order (sorted would be [a b])");
        assert_eq!(removed, vec!["z", "q"], "insertion order (sorted would be [q z])");
    }

    /// jsString mirrors JS String(): undefined for a missing (null) cva value,
    /// the bare integer for a whole-number float, and the decimal form for a
    /// fractional one.
    #[test]
    fn js_string_cases() {
        let cases: Vec<(Value, &str)> = vec![
            (Value::Null, "undefined"),
            (Value::String("solid".to_string()), "solid"),
            (serde_json::json!(4), "4"),
            (serde_json::json!(3.5), "3.5"),
        ];
        for (v, want) in cases {
            assert_eq!(js_string(&v), want, "js_string({:?})", v);
        }
    }
}
