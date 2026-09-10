//! Port of pipeline/path_parity.go — every consume path (css-import /
//! full-css) computes what React computes for every slotted element, in both
//! themes and both directions. The non-browser half (cvaSlot, splitMarkers,
//! normalizeTag, twMerge, cva-table composition) is the emitter's own code —
//! already ported.

use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;

use super::parity_baseline::{
    cell_map, diff_parity_baseline, load_parity_baseline, parity_cell, parity_norm_value,
    show_cell, show_change, write_parity_baseline,
};

const PP_SIM: &str = "build/gates/path-parity";
const PP_BASELINE: &str = "gates/path-parity-baseline.json";
const PP_READ_ALL: &str = include_str!("pp_readall.js");

const PP_PROPS: [&str; 45] = [
    "color", "background-color", "border-color", "border-top-width", "border-bottom-width",
    "border-left-width", "border-right-width", "border-radius", "padding-top", "padding-right",
    "padding-bottom", "padding-left", "margin-top", "margin-left", "margin-right", "width",
    "min-width", "max-width", "height", "min-height", "row-gap", "column-gap", "font-size",
    "font-weight", "line-height", "letter-spacing", "text-align", "display", "flex-direction",
    "align-items", "justify-content", "position", "top", "left", "right", "opacity", "box-shadow",
    "outline-width", "overflow", "white-space", "text-decoration-line", "transform", "translate", "scale", "visibility",
];

const PP_VOID: [&str; 4] = ["input", "img", "br", "hr"];

// upstream @custom-variant bodies
const PP_SHORTHAND: [(&str, &str, &str); 8] = [
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

fn re_pp_child_slot() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"data-\[slot=([0-9A-Za-z_-]+)\]").unwrap())
}
fn re_pp_combinator() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(group|peer|has|in)-").unwrap())
}
fn re_pp_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(data|aria)-\[([0-9A-Za-z_-]+)(?:=([0-9A-Za-z_-]+))?\]$").unwrap())
}
fn re_pp_aria() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^aria-(expanded|invalid|checked|disabled|pressed|selected|current)$").unwrap())
}
fn re_pp_data_bare() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^data-(inset|highlighted|empty|pressed|autoscrolling|popup-open)$").unwrap())
}
fn re_not_prefix() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^not-.*$").unwrap())
}
fn re_upper() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^[A-Z]").unwrap())
}

fn pp_child_slots(cls: &str) -> Vec<String> {
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for m in re_pp_child_slot().captures_iter(cls) {
        let s = m[1].to_string();
        if seen.insert(s.clone()) {
            out.push(s);
        }
    }
    out
}

/// ppInlineClasses: React evaluates a conditional; keep the branch the
/// default selects, then twMerge the surviving list.
fn pp_inline_classes(ir: &crate::emit::css::CssIrComponent, fn_: &str, el: &crate::emit::css::IrEl) -> String {
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

fn pp_state_configs(cls: &str) -> Vec<(String, String)> {
    let mut out: std::collections::HashMap<String, (String, String)> =
        std::collections::HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for tok in cls.split_whitespace() {
        let mut segs: Vec<&str> = tok.split(':').collect();
        segs.pop();
        let mut skip = false;
        for v in &segs {
            if *v == "*" || *v == "**" || re_pp_combinator().is_match(v) {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }
        for v in segs {
            let bare = re_not_prefix().replace_all(v, "");
            if bare.is_empty() || bare.starts_with("data-[slot=") {
                continue;
            }
            if let Some(sh) = PP_SHORTHAND
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
            if let Some(m) = re_pp_attr().captures(&bare) {
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
            if let Some(m) = re_pp_aria().captures(&bare) {
                let k = format!("aria-{}=true", &m[1]);
                if !out.contains_key(&k) {
                    out.insert(k.clone(), (format!("aria-{}", &m[1]), "true".to_string()));
                    order.push(k);
                }
                continue;
            }
            if let Some(m) = re_pp_data_bare().captures(&bare) {
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

struct PpItem {
    id: usize,
    label: String,
    kids: Vec<(usize, String)>,
    slot_html: String,
    inline_html: String,
    state: bool,
    variant: bool,
}

pub fn run_path_parity(root: &Path, record: bool, details: bool) -> i32 {
    let sim = root.join(PP_SIM);
    let _ = std::fs::remove_dir_all(&sim);
    std::fs::create_dir_all(sim.join("node_modules")).ok();
    #[cfg(unix)]
    {
        let _ = std::os::unix::fs::symlink(root, sim.join("node_modules/shadless"));
    }
    let oracle_css = std::fs::read_to_string(root.join("build/gates/oracle.css"))
        .unwrap_or_default();
    let full_css = std::fs::read_to_string(root.join("dist/shadless.full.css"))
        .unwrap_or_default();

    let shell = match crate::oracle::browser_shell::BrowserShell::start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("path-parity: {}", e);
            return 1;
        }
    };
    if let Err(e) = shell.launch() {
        eprintln!("path-parity: {}", e);
        return 1;
    }

    let mut cells: Vec<parity_cell> = Vec::new();
    let (mut compared, mut components, mut state_renders, mut variant_renders) =
        (0usize, 0usize, 0usize, 0usize);
    let pp_shell = "body{margin:0;padding:0;color:var(--foreground);background:var(--background)} *{transition:none!important;animation:none!important}";
    let doc = |css: &str, body: &str, root_class: &str| -> String {
        format!(
            "<!doctype html><html class=\"{}\"><head><style>{}</style><style>{}</style></head><body>{}</body></html>",
            root_class, css, pp_shell, body
        )
    };
    let read_all = |p: &crate::oracle::browser_shell::BPage,
                    ids: &[String]|
     -> std::collections::HashMap<String, std::collections::HashMap<String, String>> {
        let Ok(v) = p.evaluate_fn_arg(
            PP_READ_ALL,
            serde_json::json!({"ids": ids, "props": PP_PROPS.to_vec()}),
        ) else {
            return Default::default();
        };
        let mut out = std::collections::HashMap::new();
        if let Some(obj) = v.as_object() {
            for (k, sv) in obj {
                if let Some(sm) = sv.as_object() {
                    let mut cellm = std::collections::HashMap::new();
                    for (prop, val) in sm {
                        if let Some(s) = val.as_str() {
                            cellm.insert(prop.clone(), s.to_string());
                        }
                    }
                    out.insert(k.clone(), cellm);
                }
            }
        }
        out
    };

    let mut files: Vec<String> = Vec::new();
    if let Ok(ents) = std::fs::read_dir(root.join("generated/ir")) {
        for e in ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.ends_with(".json") {
                files.push(n);
            }
        }
    }
    files.sort();

    for f in &files {
        let name = f.trim_end_matches(".json");
        if !root.join("dist/css").join(format!("{}.css", name)).exists() {
            continue;
        }
        let Ok(irb) = std::fs::read_to_string(root.join("generated/ir").join(f)) else {
            continue;
        };
        let mut v: serde_json::Value = match serde_json::from_str(&irb) {
            Ok(v) => v,
            Err(_) => continue,
        };
        crate::emit::css::drop_nulls(&mut v);
        let Ok(ir) = serde_json::from_value::<crate::emit::css::CssIrComponent>(v) else {
            continue;
        };
        crate::emit::load_skin();
        let cva = crate::emit::css::cva_slot(&ir);
        let mut cva_slots: std::collections::HashMap<String, crate::emit::css::CvaTable> =
            std::collections::HashMap::new();
        for (_, (table, slot)) in &cva {
            if !slot.is_empty() {
                cva_slots.insert(slot.clone(), table.clone());
            }
        }
        let mut items: Vec<PpItem> = Vec::new();
        let mut n = 0usize;
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        fn el_of_slot<'a>(
            ir: &'a crate::emit::css::CssIrComponent,
            slot: &str,
        ) -> Option<(&'a crate::emit::css::IrFn, &'a crate::emit::css::IrEl)> {
            for c in &ir.components {
                for el in &c.elements {
                    if el.slot == slot {
                        return Some((c, el));
                    }
                }
            }
            None
        }
        fn tag_of(ir: &crate::emit::css::CssIrComponent, el: &crate::emit::css::IrEl) -> String {
            crate::emit::tags::normalize_tag(&el.tag, &ir.tag_hints).unwrap_or_else(|| "div".to_string())
        }
        fn classes_of(
            ir: &crate::emit::css::CssIrComponent,
            cva_slots: &std::collections::HashMap<String, crate::emit::css::CvaTable>,
            c: &crate::emit::css::IrFn,
            el: &crate::emit::css::IrEl,
            sel: &std::collections::HashMap<String, String>,
        ) -> String {
            let table = cva_slots.get(&el.slot);
            if table.is_none() {
                let mut ctx_defaults: Vec<String> = Vec::new();
                for r in &ir.cva_refs {
                    if r.slot != el.slot {
                        continue;
                    }
                    for ax in &r.dyn_axes {
                        let Some(d) = r.defaults.get(ax) else { continue };
                        let mut key = sel.get(ax).cloned().unwrap_or_default();
                        if key.is_empty() {
                            key = d.clone();
                        }
                        if let Some(v) = r.table.class_of(ax, &key) {
                            if !v.is_empty() {
                                ctx_defaults.push(v);
                            }
                        }
                    }
                }
                let base = pp_inline_classes(&ir, &c.fn_, el);
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
                if let Some(v) = table.class_of(&axis, &key) {
                    if !v.is_empty() {
                        list.push(v);
                    }
                }
            }
            list.extend(extras);
            crate::twmerge::merge(&list.join(" "))
        }
        fn wrap_h(h: &str) -> String {
            format!(
                r#"<div style="position:relative;width:480px;height:160px;margin:8px 0">{}</div>"#,
                h
            )
        }
        fn open_tag(tag: &str, attrs: &str, inner: &str) -> String {
            if PP_VOID.contains(&tag) {
                format!("<{} {}>", tag, attrs)
            } else {
                format!("<{} {}>{}</{}>", tag, attrs, inner, tag)
            }
        }
        fn build(
            ir: &crate::emit::css::CssIrComponent,
            cva_slots: &std::collections::HashMap<String, crate::emit::css::CvaTable>,
            items: &mut Vec<PpItem>,
            n: &mut usize,
            c: &crate::emit::css::IrFn,
            el: &crate::emit::css::IrEl,
            sel: &std::collections::HashMap<String, String>,
            attrs: &str,
            label: &str,
            extra_state: bool,
            extra_variant: bool,
        ) -> String {
            let tag = tag_of(ir, el);
            let cls = classes_of(ir, cva_slots, c, el, sel);
            let markers = crate::emit::css::split_markers(&cls).markers;
            let mut kids: Vec<(usize, String)> = Vec::new();
            let (mut slot_kids, mut inline_kids) = (String::new(), String::new());
            for k in pp_child_slots(&cls) {
                let Some((kc, ke)) = el_of_slot(ir, &k) else { continue };
                let ktag = tag_of(ir, ke);
                if re_upper().is_match(&ktag) || ktag == "?" {
                    continue;
                }
                let kcls = classes_of(ir, cva_slots, kc, ke, &std::collections::HashMap::new());
                let km = crate::emit::css::split_markers(&kcls).markers.join(" ");
                let mut inert = String::new();
                for g in pp_child_slots(&kcls) {
                    inert += &format!(r#"<div data-slot="{}" style="display:none">x</div>"#, g);
                }
                let mut kinner = String::new();
                if !PP_VOID.contains(&ktag.as_str()) && ktag != "svg" {
                    kinner = format!("x{}", inert);
                }
                let kid = kids.len();
                kids.push((kid, format!("{}>{}", label, k)));
                slot_kids += &open_tag(
                    &ktag,
                    &format!(
                        r#"data-slot="{}" id="so-{}-{}" class="{}""#,
                        k, n, kid, km
                    ),
                    &kinner,
                );
                inline_kids += &open_tag(
                    &ktag,
                    &format!(
                        r#"data-slot="{}" id="in-{}-{}" class="{}""#,
                        k, n, kid, kcls
                    ),
                    &kinner,
                );
            }
            fn inner(t: &str, k: &str) -> String {
                if PP_VOID.contains(&t) || t == "svg" {
                    return String::new();
                }
                format!("x{}", k)
            }
            let id_attrs = format!(
                r#"data-slot="{}" {} id="so-{}" class="{}""#,
                el.slot,
                attrs,
                n,
                markers.join(" ")
            );
            let id_attrs_in = format!(
                r#"data-slot="{}" {} id="in-{}" class="{}""#,
                el.slot, attrs, n, cls
            );
            items.push(PpItem {
                id: *n,
                label: label.to_string(),
                kids,
                slot_html: wrap_h(&open_tag(&tag, &id_attrs, &inner(&tag, &slot_kids))),
                inline_html: wrap_h(&open_tag(&tag, &id_attrs_in, &inner(&tag, &inline_kids))),
                state: extra_state,
                variant: extra_variant,
            });
            *n += 1;
            cls
        }

        for c in &ir.components {
            for (idx, el) in c.elements.iter().enumerate() {
                if el.slot.is_empty() || seen.contains(&el.slot) {
                    continue;
                }
                let tag = tag_of(&ir, el);
                if re_upper().is_match(&tag) || tag == "?" {
                    continue;
                }
                seen.insert(el.slot.clone());
                let cc = ir
                    .components
                    .iter()
                    .find(|x| x.fn_ == c.fn_)
                    .unwrap_or(c);
                let cls = classes_of(&ir, &cva_slots, cc, el, &std::collections::HashMap::new());
                if cls.trim().is_empty() {
                    continue;
                }
                build(
                    &ir,
                    &cva_slots,
                    &mut items,
                    &mut n,
                    cc,
                    el,
                    &std::collections::HashMap::new(),
                    "",
                    &format!("{}#{}", el.slot, idx),
                    false,
                    false,
                );
                if let Some(table) = cva_slots.get(&el.slot) {
                    for axis in table.axis_order() {
                        for v in table.values(&axis).keys() {
                            let mut sel = std::collections::HashMap::new();
                            sel.insert(axis.clone(), v.clone());
                            build(
                                &ir,
                                &cva_slots,
                                &mut items,
                                &mut n,
                                cc,
                                el,
                                &sel,
                                &format!(r#"data-{}="{}""#, axis, v),
                                &format!("{}#{}[{}={}]", el.slot, idx, axis, v),
                                false,
                                true,
                            );
                        }
                    }
                }
                for r in &ir.cva_refs {
                    if r.slot != el.slot {
                        continue;
                    }
                    for axis in &r.dyn_axes {
                        if !r.defaults.contains_key(axis) {
                            continue;
                        }
                        for v in r.table.values(axis).keys() {
                            let cls_v = r.table.class_of(axis, v).unwrap_or_default();
                            if cls_v.is_empty() {
                                continue;
                            }
                            let mut sel = std::collections::HashMap::new();
                            sel.insert(axis.clone(), v.clone());
                            let inline = classes_of(&ir, &cva_slots, cc, el, &sel);
                            let markers = crate::emit::css::split_markers(&inline).markers;
                            let tag = tag_of(&ir, el);
                            items.push(PpItem {
                                id: n,
                                label: format!("{}#{}[{}={}]", el.slot, idx, axis, v),
                                variant: true,
                                slot_html: wrap_h(&open_tag(
                                    &tag,
                                    &format!(
                                        r#"data-slot="{}" data-{}="{}" id="so-{}" class="{}""#,
                                        el.slot,
                                        axis,
                                        v,
                                        n,
                                        markers.join(" ")
                                    ),
                                    "x",
                                )),
                                inline_html: wrap_h(&open_tag(
                                    &tag,
                                    &format!(
                                        r#"data-slot="{}" data-{}="{}" id="in-{}" class="{}""#,
                                        el.slot, axis, v, n, inline
                                    ),
                                    "x",
                                )),
                                kids: Vec::new(),
                                state: false,
                            });
                            n += 1;
                        }
                    }
                }
                for av in pp_state_configs(&cls) {
                    build(
                        &ir,
                        &cva_slots,
                        &mut items,
                        &mut n,
                        cc,
                        el,
                        &std::collections::HashMap::new(),
                        &format!(r#"{}="{}""#, av.0, av.1),
                        &format!("{}#{}[{}={}]", el.slot, idx, av.0, av.1),
                        true,
                        false,
                    );
                }
            }
        }
        if items.is_empty() {
            continue;
        }
        components += 1;
        for it in &items {
            if it.state {
                state_renders += 1;
            }
            if it.variant {
                variant_renders += 1;
            }
        }

        // (a) consumer build: core + this component's css
        std::fs::write(
            sim.join("entry.css"),
            format!("@import \"shadless\";\n@import \"shadless/{}.css\";\n", name),
        )
        .ok();
        // Go runs `./build/pipeline tw <in> <out> --cwd <sim>` and captures
        // CombinedOutput — the tailwind banner goes into the buffer, not the
        // terminal, and the error path prints it.
        let self_exe = std::env::current_exe().unwrap_or_else(|_| "pipeline".into());
        let tw_out = std::process::Command::new(&self_exe)
            .args([
                "tw",
                &sim.join("entry.css").to_string_lossy(),
                &sim.join("out.css").to_string_lossy(),
                "--cwd",
                &sim.to_string_lossy(),
            ])
            .current_dir(root)
            .output();
        match tw_out {
            Ok(o) if !o.status.success() => {
                eprintln!(
                    "FAIL  path-parity: shadless/{}.css does not compile alone\n{}",
                    name,
                    String::from_utf8_lossy(&o.stdout)
                );
                return 1;
            }
            Err(e) => {
                eprintln!(
                    "FAIL  path-parity: shadless/{}.css does not compile alone\n{}",
                    name, e
                );
                return 1;
            }
            _ => {}
        }
        let consumer_css = std::fs::read_to_string(sim.join("out.css")).unwrap_or_default();
        let mut slot_body: Vec<String> = Vec::new();
        let mut inline_body: Vec<String> = Vec::new();
        let mut ids: Vec<String> = Vec::new();
        let mut in_ids: Vec<String> = Vec::new();
        for it in &items {
            slot_body.push(it.slot_html.clone());
            inline_body.push(it.inline_html.clone());
            ids.push(format!("so-{}", it.id));
            for k in &it.kids {
                ids.push(format!("so-{}", k.0));
            }
            in_ids.push(format!("in-{}", it.id));
            for k in &it.kids {
                in_ids.push(format!("in-{}", k.0));
            }
        }
        if std::env::var("PP_KEEP").is_ok() {
            let _ = std::fs::write(
                sim.join(format!("{}.html", name)),
                format!(
                    "{}\n<!-- ORACLE -->\n{}",
                    doc(&consumer_css, &slot_body.join("\n"), ""),
                    doc("", &inline_body.join("\n"), "style-nova")
                ),
            );
        }
        let p_a = match shell.new_page(false) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let _ = p_a.set_content(&doc(&consumer_css, &slot_body.join("\n"), ""));
        p_a.wait_for_timeout(50);
        let p_b = match shell.new_page(false) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let _ = p_b.set_content(&doc(&full_css, &slot_body.join("\n"), ""));
        p_b.wait_for_timeout(50);
        let p_o = match shell.new_page(false) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let _ = p_o.set_content(&doc(&oracle_css, &inline_body.join("\n"), "style-nova"));
        p_o.wait_for_timeout(50);
        let a = read_all(&p_a, &ids);
        let b = read_all(&p_b, &ids);
        let o = read_all(&p_o, &in_ids);
        p_a.close();
        p_b.close();
        p_o.close();
        for it in &items {
            let mut node_ents: Vec<(String, String)> = vec![(format!("so-{}", it.id), it.label.clone())];
            for k in &it.kids {
                node_ents.push((format!("so-{}", k.0), k.1.clone()));
            }
            let themes: [(&str, &str); 4] = [
                ("light", "ltr"),
                ("dark", "ltr"),
                ("light", "rtl"),
                ("dark", "rtl"),
            ];
            for theme in themes {
                for nd in &node_ents {
                    let ref_key = nd.0.replacen("so-", "in-", 1);
                    let ref_ = o
                        .get(&format!("{}@{}@{}", ref_key, theme.0, theme.1))
                        .cloned()
                        .unwrap_or_default();
                    for path in ["css-import", "full-css"] {
                        let side = if path == "css-import" { &a } else { &b };
                        let got = side
                            .get(&format!("{}@{}@{}", nd.0, theme.0, theme.1))
                            .cloned()
                            .unwrap_or_default();
                        if ref_.is_empty() || got.is_empty() {
                            continue;
                        }
                        compared += 1;
                        for p in PP_PROPS {
                            let va = parity_norm_value(ref_.get(p).map(String::as_str).unwrap_or(""), false);
                            let vb = parity_norm_value(got.get(p).map(String::as_str).unwrap_or(""), false);
                            if va != vb {
                                cells.push(parity_cell {
                                    id: format!(
                                        "{}/{}/{}@{}@{}@{}",
                                        name, nd.1, p, path, theme.0, theme.1
                                    ),
                                    oracle: va,
                                    shadless: vb,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    if std::env::var("PP_KEEP").is_err() {
        let _ = std::fs::remove_dir_all(&sim);
    }

    let (actual, order) = cell_map(&cells);
    if details {
        for id in &order {
            if id.ends_with("@css-import@light@ltr") {
                println!("{}: {}", id, show_cell(&actual[id]));
            }
        }
    }
    if record || !root.join(PP_BASELINE).exists() {
        if let Err(e) = write_parity_baseline(
            root,
            PP_BASELINE,
            "slot-only markup via css-import / full-css vs React inline classes under upstream css; may only shrink, and a recorded cell's VALUES are pinned too",
            &[],
            &actual,
        ) {
            eprintln!("path-parity: {}", e);
            return 1;
        }
        println!(
            "path-parity: baseline recorded ({} cells over {} components, {} element×path×theme×dir comparisons incl. {} variant + {} state renders)",
            actual.len(),
            components,
            compared,
            variant_renders,
            state_renders
        );
        return 0;
    }
    let recorded = match load_parity_baseline(root, PP_BASELINE) {
        Ok(Some((_, r))) => r,
        Ok(None) => Default::default(),
        Err(e) => {
            eprintln!("path-parity: {}", e);
            return 1;
        }
    };
    let d = diff_parity_baseline(&recorded, &actual, &order);
    if !d.appeared.is_empty() {
        let n = d.appeared.len().min(40);
        let parts: Vec<String> = d.appeared[..n]
            .iter()
            .map(|id| format!("{}: {}", id, show_cell(&actual[id])))
            .collect();
        let tail = if d.appeared.len() > 40 {
            format!("\n  … +{} more", d.appeared.len() - 40)
        } else {
            String::new()
        };
        eprintln!(
            "FAIL  path-parity ({} NEW cells where a consume path ≠ React under upstream css)\n  {}{}\n",
            d.appeared.len(),
            parts.join("\n  "),
            tail
        );
        return 1;
    }
    if !d.changed.is_empty() {
        let n = d.changed.len().min(20);
        let parts: Vec<String> = d.changed[..n].iter().map(show_change).collect();
        eprintln!(
            "FAIL  path-parity ({} recorded cells still differ, but by a DIFFERENT amount — re-look, then re-record: ./build/pipeline path-parity --record)\n  {}\n",
            d.changed.len(),
            parts.join("\n  ")
        );
        return 1;
    }
    if !d.fixed.is_empty() {
        let n = d.fixed.len().min(20);
        eprintln!(
            "FAIL  path-parity ({} recorded cells no longer differ — record the win: ./build/pipeline path-parity --record && ./build/pipeline ledger --record)\n  {}\n",
            d.fixed.len(),
            d.fixed[..n].join("\n  ")
        );
        return 1;
    }
    println!(
        "PASS  path-parity ({} components, {} comparisons incl. {} state renders, {} cells at the recorded baseline incl. their values; --strict is the end state)",
        components,
        compared,
        state_renders,
        actual.len()
    );
    shell.close();
    0
}
