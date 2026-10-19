//! The attribute vocabulary: `dist/css/vocabulary.json`, generated from the
//! shipped IRs by the `demo` node (the node that writes dist/css).
//!
//! The docs code tabs are slot-only markup — `data-slot` + `data-*` + only
//! the classes the stylesheet cannot reproduce (markers) — so the contract a
//! consumer must keep is the ATTRIBUTE vocabulary: which slot names exist,
//! which `data-<axis>` values each slot accepts, which markers every render
//! of a slot carries. This module derives exactly that from generated/ir
//! (cva tables + cvaRefs, `cva_slot`'s table→slot convention, `split_markers`
//! on the element classes, narrowed to group/peer markers and skin classes
//! with a skin body — see `SlotVocabulary::markers`) — the IR is the only list;
//! nothing here is hand-maintained.
//!
//! Scope: the components the `demo` node ships (static/kernel/trivial-js or
//! tiers.json `emit:true` — the same set that gets dist/components pages).
//! Measured note: that is the docs projection's `dist/css/<name>.css exists`
//! filter plus exactly aspect-ratio, collapsible and direction — components
//! that ship pages and (collapsible) runtime behavior but carry no upstream
//! classes, so no stylesheet. Their slots are shipped vocabulary (the runtime
//! targets `data-slot="collapsible-trigger"`); leaving them out would make
//! the checker warn on the product's own markup.
//!
//! Axes: cva tables only. A `data-<axis>` that exists only as an in-@apply
//! hook (card `data-[size=sm]`, select-trigger `data-[size=default|sm]`,
//! field-legend `data-[variant=…]`, dropdown-menu-item
//! `data-[variant=destructive]`, …) is deliberately NOT an axis: React sets
//! such values with no table naming the full set, so checking them would
//! produce false errors. The checker documents this ("checks the cva axes
//! listed in vocabulary.json").
//!
//! Output is byte-stable: keys inserted sorted, arrays sorted, two-space
//! indent, trailing newline.

use super::css::{CssIrComponent, MARKER_RE, cva_slot, split_markers};
use std::collections::BTreeMap;

/// One slot's entry. `component` is a single IR name: a slot name carried by
/// two shipped IRs would make every per-slot fact (axes, markers) ambiguous,
/// so the builder refuses (none exists today — measured; if one ever appears
/// this fails loudly instead of silently picking one).
#[derive(Debug)]
pub struct SlotVocabulary {
    pub component: String,
    /// axis -> (sorted values, default). Empty when the slot has no cva table.
    pub axes: BTreeMap<String, (Vec<String>, Option<String>)>,
    /// The marker tokens EVERY IR element of the slot carries — the
    /// intersection, so a requirement is only emitted when no render path of
    /// the slot can lack it — keeping the group/peer markers (MARKER_RE,
    /// whether or not a shipped rule names that group today) and the
    /// skin-allowlisted tokens that have a skin body (the same predicate
    /// css.rs applies when it emits the unlayered `.cn-*` rule). split_markers
    /// also folds in dead utilities and body-less skin hooks; those key no
    /// rule anywhere (measured: origin-top-center, cn-menu-target,
    /// cn-rtl-flip, cn-font-heading — the last's only rule lives in the docs
    /// site's globals.css, which does not ship), so demanding them would
    /// make --strict fail on rendering-identical markup. Empty when the slot
    /// has none.
    pub markers: Vec<String>,
}

pub struct Vocabulary {
    pub slots: BTreeMap<String, SlotVocabulary>,
    /// Slot names the shipped markup/CSS references via `data-[slot=X]`
    /// tokens without owning an element (measured: checkbox-group — React's
    /// FieldGroup renders its group slot as `checkbox-group` for checkbox
    /// fields, and field's classes carry the `data-[slot=checkbox-group]`
    /// hooks; combobox-content — referenced from input-group). Known to the
    /// checker's unknown-slot rule; no axes/markers are claimed for them.
    pub slot_refs: Vec<String>,
    /// Every axis name that occurs in any shipped table (sorted).
    pub axis_attributes: Vec<String>,
}

/// Table axes → sorted (values, default), the cvaRef `defaults` overriding
/// the table's own when present (css.rs uses the same precedence).
fn table_axes(
    table: &super::css::CvaTable,
    ref_defaults: Option<&std::collections::HashMap<String, String>>,
    out: &mut BTreeMap<String, (Vec<String>, Option<String>)>,
) {
    for axis in table.axis_order() {
        let mut values: Vec<String> = table.values(&axis).keys().cloned().collect();
        values.sort();
        let def = ref_defaults
            .and_then(|d| d.get(&axis))
            .cloned()
            .or_else(|| table.defaults.get(&axis).cloned());
        match out.get_mut(&axis) {
            Some((seen, def_slot)) => {
                for v in values {
                    if !seen.contains(&v) {
                        seen.push(v);
                    }
                }
                seen.sort();
                if def_slot.is_none() {
                    *def_slot = def;
                }
            }
            None => {
                out.insert(axis, (values, def));
            }
        }
    }
}

/// Builds the vocabulary from the SHIPPED IRs, in the given (sorted) order.
/// A slot name in two IRs, or two tables disagreeing on an axis default for
/// one slot, is a hard error: the artifact must not encode an ambiguity.
pub fn build_vocabulary(irs: &[(&str, &CssIrComponent)]) -> Result<Vocabulary, String> {
    static HOOK_SLOT_RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let hook_slot_re =
        HOOK_SLOT_RE.get_or_init(|| regex::Regex::new(r"data-\[slot=([0-9A-Za-z_-]+)\]").unwrap());
    let mut slots: BTreeMap<String, SlotVocabulary> = BTreeMap::new();
    let mut slot_refs: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut axis_attributes: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (name, ir) in irs {
        // element slots + markers
        let mut elements_of: BTreeMap<&str, Vec<&super::css::IrEl>> = BTreeMap::new();
        for c in &ir.components {
            for e in &c.elements {
                if e.slot.is_empty() {
                    continue;
                }
                elements_of.entry(e.slot.as_str()).or_default().push(e);
                for cl in &e.classes {
                    for m in hook_slot_re.captures_iter(cl) {
                        slot_refs.insert(m[1].to_string());
                    }
                }
            }
        }
        for (slot, els) in &elements_of {
            let entry = slots
                .entry(slot.to_string())
                .or_insert_with(|| SlotVocabulary {
                    component: name.to_string(),
                    axes: BTreeMap::new(),
                    markers: Vec::new(),
                });
            if entry.component != *name {
                return Err(format!(
                    "vocabulary: slot {:?} is carried by shipped IRs {:?} and {:?} — per-slot axes/markers would be ambiguous; name one slot differently",
                    slot, entry.component, name
                ));
            }
            // markers = intersection of split_markers over EVERY element,
            // narrowed to group/peer markers and skin tokens with a skin
            // body (see the SlotVocabulary::markers doc)
            let skin = super::skin_data();
            let mut marker_sets: Vec<std::collections::BTreeSet<String>> = Vec::new();
            for e in els {
                let mk: std::collections::BTreeSet<String> = split_markers(&e.classes.join(" "))
                    .markers
                    .into_iter()
                    .filter(|t| {
                        MARKER_RE.is_match(t)
                            || (skin.allowlist.contains(t) && skin.map.contains_key(t))
                    })
                    .collect();
                marker_sets.push(mk);
            }
            let mut common: Vec<String> = match marker_sets.pop() {
                Some(first) => first.into_iter().collect(),
                None => Vec::new(),
            };
            for set in &marker_sets {
                common.retain(|m| set.contains(m));
            }
            common.sort();
            if entry.markers.is_empty() {
                entry.markers = common;
            } else if entry.markers != common {
                return Err(format!(
                    "vocabulary: slot {:?} has different marker intersections within {:?}: {:?} vs {:?}",
                    slot, name, entry.markers, common
                ));
            }
        }
        // own cva tables (css.rs cva_slot: table name -> slot)
        let cva = cva_slot(ir);
        let mut cva_by_slot: BTreeMap<&str, Vec<&super::css::CvaTable>> = BTreeMap::new();
        for (table, slot) in cva.values() {
            if !slot.is_empty() {
                cva_by_slot.entry(slot.as_str()).or_default().push(table);
            }
        }
        for (slot, tables) in &cva_by_slot {
            let entry = slots.get_mut(*slot).ok_or_else(|| {
                format!(
                    "vocabulary: cva table of {:?} maps to slot {:?} which no element carries",
                    name, slot
                )
            })?;
            for t in tables {
                table_axes(t, None, &mut entry.axes);
            }
        }
        // cross-file cva refs (table copy rides the ref; ref defaults win)
        for r in &ir.cva_refs {
            if r.slot.is_empty() {
                continue;
            }
            let entry = slots
                .entry(r.slot.clone())
                .or_insert_with(|| SlotVocabulary {
                    component: name.to_string(),
                    axes: BTreeMap::new(),
                    markers: Vec::new(),
                });
            if entry.component != *name {
                return Err(format!(
                    "vocabulary: slot {:?} is carried by shipped IRs {:?} and {:?} — per-slot axes/markers would be ambiguous; name one slot differently",
                    r.slot, entry.component, name
                ));
            }
            table_axes(&r.table, Some(&r.defaults), &mut entry.axes);
        }
        for t in cva_by_slot.values().flatten() {
            for axis in t.axis_order() {
                axis_attributes.insert(axis);
            }
        }
        for r in &ir.cva_refs {
            for axis in r.table.axis_order() {
                axis_attributes.insert(axis);
            }
        }
    }
    slot_refs.retain(|s| !slots.contains_key(s));
    Ok(Vocabulary {
        slots,
        slot_refs: slot_refs.into_iter().collect(),
        axis_attributes: axis_attributes.into_iter().collect(),
    })
}

/// Serializes byte-stable: keys inserted in sorted order (the maps are
/// BTreeMaps), arrays sorted, JSON.stringify(v, null, 2) shape via the
/// pipeline's own writer, trailing newline.
pub fn to_json(v: &Vocabulary) -> String {
    use crate::jsonorder::{Json, arr, obj};
    let slots = Json::Obj(
        v.slots
            .iter()
            .map(|(slot, sv)| {
                let axes = Json::Obj(
                    sv.axes
                        .iter()
                        .map(|(axis, (values, def))| {
                            (
                                axis.clone(),
                                obj(vec![
                                    (
                                        "default",
                                        def.as_deref()
                                            .map(|d| Json::Str(d.to_string()))
                                            .unwrap_or(Json::Null),
                                    ),
                                    (
                                        "values",
                                        arr(values.iter().map(|x| Json::Str(x.clone())).collect()),
                                    ),
                                ]),
                            )
                        })
                        .collect(),
                );
                (
                    slot.clone(),
                    obj(vec![
                        ("axes", axes),
                        ("component", Json::Str(sv.component.clone())),
                        (
                            "markers",
                            arr(sv.markers.iter().map(|m| Json::Str(m.clone())).collect()),
                        ),
                    ]),
                )
            })
            .collect(),
    );
    let doc = obj(vec![
        (
            "axisAttributes",
            arr(v
                .axis_attributes
                .iter()
                .map(|a| Json::Str(a.clone()))
                .collect()),
        ),
        ("schema", Json::Int(1)),
        (
            "slotRefs",
            arr(v.slot_refs.iter().map(|s| Json::Str(s.clone())).collect()),
        ),
        ("slots", slots),
    ]);
    format!("{}\n", crate::jsonorder::marshal_js(&doc))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ir(name: &str, json: serde_json::Value) -> (&'static str, CssIrComponent) {
        // leak the name so the (&str, _) pairs stay 'static like the real
        // caller's sorted name list
        let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
        let mut v = json;
        crate::emit::css::drop_nulls(&mut v);
        (leaked, serde_json::from_value(v).expect("test IR parses"))
    }

    #[test]
    fn unit_slot_in_two_shipped_irs_is_refused() {
        let a = ir(
            "a",
            serde_json::json!({"name": "a", "tier": "static", "components": [
                {"fn": "T", "export": true, "elements": [{"tag": "div", "slot": "shared", "classes": ["group/x"]}]}
            ]}),
        );
        let b = ir(
            "b",
            serde_json::json!({"name": "b", "tier": "static", "components": [
                {"fn": "T", "export": true, "elements": [{"tag": "div", "slot": "shared", "classes": ["group/x"]}]}
            ]}),
        );
        let err = match build_vocabulary(&[("a", &a.1), ("b", &b.1)]) {
            Err(e) => e,
            Ok(_) => panic!("a slot carried by two shipped IRs must be refused"),
        };
        assert!(err.contains("ambiguous"), "got: {err}");
    }

    #[test]
    fn unit_markers_intersect_and_cva_axes_land() {
        let button = ir(
            "button",
            serde_json::json!({
                "name": "button", "tier": "static",
                "cva": {"buttonVariants": {
                    "base": "group/button px-2",
                    "variants": {"size": {"default": "h-8", "sm": "", "lg": "h-9"}},
                    "defaults": {"size": "default"}
                }},
                "components": [
                    {"fn": "Button", "export": true, "elements": [
                        {"tag": "button", "slot": "button", "classes": ["group/button", "h-8"]}
                    ]}
                ]
            }),
        );
        // empty-apply value "sm" must still be listed: the CSS skips it, the
        // vocabulary must not (measured: attachment-media icon, marker default)
        let v = build_vocabulary(&[(button.0, &button.1)]).expect("builds");
        let b = v.slots.get("button").expect("slot present");
        assert_eq!(b.component, "button");
        let (values, def) = b.axes.get("size").expect("size axis");
        assert_eq!(
            values,
            &["default".to_string(), "lg".to_string(), "sm".to_string()]
        );
        assert_eq!(def.as_deref(), Some("default"));
        assert_eq!(b.markers, ["group/button"]);
        assert_eq!(v.axis_attributes, ["size"]);
        assert!(v.slot_refs.is_empty());
        let j = to_json(&v);
        assert!(j.contains("\"default\": \"default\""), "{j}");
        // byte-stable: same input, same bytes
        let button2 = ir(
            "button",
            serde_json::json!({
                "name": "button", "tier": "static",
                "cva": {"buttonVariants": {
                    "base": "group/button px-2",
                    "variants": {"size": {"default": "h-8", "sm": "", "lg": "h-9"}},
                    "defaults": {"size": "default"}
                }},
                "components": [
                    {"fn": "Button", "export": true, "elements": [
                        {"tag": "button", "slot": "button", "classes": ["group/button", "h-8"]}
                    ]}
                ]
            }),
        );
        assert_eq!(
            j,
            to_json(&build_vocabulary(&[(button2.0, &button2.1)]).expect("builds"))
        );
    }

    #[test]
    fn unit_markers_keep_only_rule_backed_tokens() {
        // split_markers also folds in skinAllowlist and deadUtilities tokens;
        // a marker belongs in the vocabulary only when it is a group/peer
        // marker or a skin token that has a skin body (css.rs emits the
        // unlayered .cn-* rule exactly for those, so a SHIPPED stylesheet
        // rule keys on it). cn-menu-target (body-less skin hook), origin-top-center
        // (dead utility) and cn-font-heading (its only rule is the docs
        // site's globals.css, not in the package) are inert — demanding them
        // would make --strict fail on rendering-identical markup.
        let menu = ir(
            "menu",
            serde_json::json!({
                "name": "menu", "tier": "kernel",
                "components": [
                    {"fn": "Menu", "export": true, "elements": [
                        {"tag": "div", "slot": "menu-content", "classes": [
                            "group/menu", "cn-menu-target", "cn-menu-translucent",
                            "cn-font-heading", "origin-top-center"
                        ]}
                    ]}
                ]
            }),
        );
        let v = build_vocabulary(&[(menu.0, &menu.1)]).expect("builds");
        let m = v
            .slots
            .get("menu-content")
            .expect("slot present")
            .markers
            .clone();
        assert_eq!(
            m,
            ["cn-menu-translucent", "group/menu"],
            "markers must be exactly group|peer plus skin tokens with a skin body"
        );
    }

    #[test]
    fn unit_cva_ref_defaults_win_and_data_slot_hooks_become_slot_refs() {
        let group = ir(
            "input-group",
            serde_json::json!({
                "name": "input-group", "tier": "kernel",
                "components": [
                    {"fn": "InputGroup", "export": true, "elements": [
                        {"tag": "div", "slot": "input-group", "classes": ["data-[slot=combobox-content]:flex"]}
                    ]}
                ]
            }),
        );
        let v = build_vocabulary(&[(group.0, &group.1)]).expect("builds");
        assert_eq!(v.slot_refs, ["combobox-content"]);
    }

    #[test]
    fn unit_every_shipped_ir_slot_lands_in_the_vocabulary() {
        // The scope claim, enforced: every slot of every shipped IR —
        // including the slots neither dist/css selectors nor the shipped
        // corpus ever mention (portals, providers, container/menu roots —
        // measured 22 of 262) — must reach dist/css/vocabulary.json, or the
        // checker warns "unknown-slot" on the product's own markup. The
        // expected set is collected straight off the IRs, independently of
        // build_vocabulary's element loop, so a filter dropped in that loop
        // cannot hide a slot from the artifact. Gate-covered: __gate unit
        // runs the unit_ filter (main.rs).
        let root = crate::tree_root();
        if !root.join("generated/ir").exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: no shadless tree)");
            }
            eprintln!("skip: no shadless tree");
            return;
        }
        let irs = crate::emit::demo::shipped_irs(&root)
            .expect("the shipped IR set loads")
            .0;
        let pairs: Vec<(&str, &CssIrComponent)> =
            irs.iter().map(|(n, ir)| (n.as_str(), ir)).collect();
        let v = build_vocabulary(&pairs).expect("builds");
        let mut checked = 0usize;
        for (name, ir) in &irs {
            for c in &ir.components {
                for e in &c.elements {
                    if !e.slot.is_empty() {
                        checked += 1;
                        assert!(
                            v.slots.contains_key(&e.slot),
                            "shipped IR {:?} carries slot {:?} that build_vocabulary dropped — dist/css/vocabulary.json would not know it",
                            name,
                            e.slot
                        );
                    }
                }
            }
        }
        assert!(checked > 0, "the shipped IRs carry element slots");
    }

    #[test]
    fn unit_vocabulary_json_on_disk_is_the_built_vocabulary() {
        // The demo node's fs::write of dist/css/vocabulary.json is the only
        // thing that keeps the shipped artifact fresh, and no gate compared
        // that file to what the builder produces: with the write removed (or
        // gone stale after an emitter/IR change without a demo rerun) every
        // consumer gate read the stale file and stayed green — the frozen
        // vocabulary would ship. Byte-compare the disk file against
        // to_json(build_vocabulary(shipped_irs())) in run_demo's own order,
        // so the first divergence reds `__gate unit` (the unit_ filter runs
        // this test). A missing file is a skip without the tree (fresh
        // checkout, ledger convention) and a crash once the tree exists —
        // the checker suite reds a missing file independently.
        let root = crate::tree_root();
        let disk = root.join("dist/css/vocabulary.json");
        if !root.join("generated/ir").exists() || !disk.exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree inputs missing (skip: no shadless tree)");
            }
            eprintln!("skip: no shadless tree");
            return;
        }
        let irs = crate::emit::demo::shipped_irs(&root)
            .expect("the shipped IR set loads")
            .0;
        // run_demo's exact input order: shipped names, sorted, looked up by
        // name (to_json is order-independent — BTreeMaps — but the ambiguity
        // and default precedence rules are not worth re-deriving)
        let mut names: Vec<String> = irs.iter().map(|(n, _)| n.clone()).collect();
        names.sort();
        let by_name: std::collections::HashMap<&str, &CssIrComponent> =
            irs.iter().map(|(n, ir)| (n.as_str(), ir)).collect();
        let pairs: Vec<(&str, &CssIrComponent)> = names
            .iter()
            .map(|n| (n.as_str(), by_name[n.as_str()]))
            .collect();
        let built = to_json(&build_vocabulary(&pairs).expect("builds"));
        let on_disk = std::fs::read_to_string(&disk).expect("dist/css/vocabulary.json reads");
        assert_eq!(
            built, on_disk,
            "dist/css/vocabulary.json differs from to_json(build_vocabulary(shipped_irs())) — the shipped artifact is stale or hand-edited; run ./build/pipeline demo"
        );
    }
}
