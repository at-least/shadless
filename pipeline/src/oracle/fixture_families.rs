//! The per-family fixture build (portal/menu/select/nav/inline/none) —
//! the rest of example_fixture.go's switch. Self-tests are stored as data
//! (SelfTestAction) rather than closures: every Go closure captured only
//! (page, comp, prefix/ids, open-gesture) — all value-shaped.

use super::browser_shell::BPage;
use super::example_fixture::{
    ef_ensure_content_id, ef_learn, ef_re_harvest_mark, ef_re_orig_and_id, ef_re_orig_attr,
    ef_remap, ef_strip_radix_ids, EfDef, EfSlotStable, EF_RETARGET_JS,
};
use super::families::FamilyEnt;
use super::example_fixture::{EF_HARVEST_LAYER, EF_MENU_IDS, EF_NAV_IDS, EF_TABS_DRIVER};
use serde_json::json;
use std::collections::HashMap;

pub enum SelfTestAction {
    Dialog { comp: String },
    Portal { prefix: String, comp: String, hover: bool },
    MenuOrSelect { trigger: String, comp: String, contextmenu: bool },
    Nav { trigger: String, content: String },
    Inline,
    None { comp: String },
}

pub fn run_self_test(action: &SelfTestAction, page: &BPage<'_>) -> Result<(), String> {
    // poll until `expr` is true, up to 3s (the Go fixed 500ms assumes an idle
    // machine; this build runs under the same load pattern as the oracle
    // chain and needs a little more)
    fn wait_true(page: &BPage<'_>, expr: &str) -> Result<(), String> {
        for _ in 0..30 {
            let v = page
                .evaluate_fn_arg(r#"(e) => !!(eval(e))"#, json!(expr))
                .unwrap_or(serde_json::Value::Null);
            if v == serde_json::Value::Bool(true) {
                return Ok(());
            }
            page.wait_for_timeout(100).map_err(|e| e.to_string())?;
        }
        Err(format!("timeout waiting for: {}", expr))
    }

    match action {
        SelfTestAction::Dialog { comp } => {
            page.loc_click("", "#d1-trigger", 0, "left")?;
            let _ = page.wait_for_timeout(1500);
            let mine = format!("[data-slot=\"{}-content\"]", comp);
            wait_true(page, &format!("!!document.querySelector('{}')", mine))?;
            let _ = page.evaluate_fn(
                r#"() => {
              const btn = document.querySelector('[data-slot$="-content"] > button')
                ?? document.querySelector('[data-slot$="-action"], [data-slot$="-cancel"]')
              if (btn) btn.click()
              else document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }))
            }"#,
            );
            let _ = page.wait_for_timeout(500);
            let closed_v = page
                .evaluate_fn_arg(r#"(sel) => !document.querySelector(sel)"#, json!(mine))
                .unwrap_or(serde_json::Value::Null);
            if closed_v != serde_json::Value::Bool(true) {
                return Err("did not close".to_string());
            }
            Ok(())
        }
        SelfTestAction::Portal { prefix, comp, hover } => {
            let sel = format!("#{}-trigger", prefix);
            if *hover {
                let Some(b) = page.loc_box("", &sel, 0).map_err(|e| e.to_string())? else {
                    return Err("no trigger box".to_string());
                };
                page.mouse_move(b.x + b.width / 2.0, b.y + b.height + 60.0, 1).map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(300);
                page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 6).map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(1100);
            } else {
                page.loc_click("", &sel, 0, "left").map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(1500);
            }
            wait_true(
                page,
                &format!("!!document.querySelector(\"[data-slot='{}-content']\")", comp),
            )?;
            page.key_press("Escape").map_err(|e| e.to_string())?;
            page.mouse_move(0.0, 0.0, 1).map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(700);
            wait_true(
                page,
                &format!("!document.querySelector(\"[data-slot='{}-content']\")", comp),
            )?;
            Ok(())
        }
        SelfTestAction::MenuOrSelect { trigger, comp, contextmenu } => {
            if trigger.is_empty() {
                // nothing openable by design (all instances disabled)
                return Ok(());
            }
            let sel = format!("#{}-trigger", trigger);
            if *contextmenu {
                let Some(b) = page.loc_box("", &sel, 0).map_err(|e| e.to_string())? else {
                    return Err("no trigger box".to_string());
                };
                page.mouse_click_button(b.x + b.width / 2.0, b.y + b.height / 2.0, "right")
                    .map_err(|e| e.to_string())?;
            } else {
                page.loc_click("", &sel, 0, "left").map_err(|e| e.to_string())?;
            }
            let _ = page.wait_for_timeout(500);
            let content_sel = format!("[data-slot=\"{}-content\"]", comp);
            wait_true(
                page,
                &format!("!!document.querySelector('{}[data-state=open]')", content_sel),
            )?;
            let n = page.loc_count("", &format!("[data-slot=\"{}-sub-trigger\"]", comp))?;
            if n > 0 {
                if let Some(b) = page.loc_box("", &format!("[data-slot=\"{}-sub-trigger\"]", comp), 0)
                    .map_err(|e| e.to_string())?
                {
                    page.mouse_move(b.x + 4.0, b.y + b.height / 2.0, 3).map_err(|e| e.to_string())?;
                    page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 6).map_err(|e| e.to_string())?;
                    let _ = page.wait_for_timeout(600);
                    if std::env::var("EF_DEBUG").is_ok() {
                        let v = page.evaluate(&format!(
                            "(() => {{ const el = document.querySelector('[data-slot=\"{}-sub-content\"]'); const fp = document.elementFromPoint({}, {}); return JSON.stringify({{ subMounted: !!el, onTop: fp ? fp.tagName + '[' + (fp.getAttribute('data-slot') || fp.id || '') + ']' : 'none' }}) }})()",
                            comp, b.x + b.width / 2.0, b.y + b.height / 2.0
                        ))
                        .unwrap_or(serde_json::Value::Null);
                        eprintln!("[dbg] submenu probe: {}", v);
                    }
                    wait_true(
                        page,
                        &format!(
                            "!!document.querySelector(\"[data-slot='{}-sub-content']\")",
                            comp
                        ),
                    )
                    .map_err(|_| "sub menu did not open".to_string())?;
                    page.key_press("Escape").map_err(|e| e.to_string())?;
                    let _ = page.wait_for_timeout(300);
                }
            }
            page.key_press("Escape").map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(500);
            wait_true(
                page,
                &format!("!document.querySelector('{}[data-state=open]')", content_sel),
            )?;
            Ok(())
        }
        SelfTestAction::Nav { trigger, content } => {
            // the nav trigger activates on pointer events; a synthetic .click()
            // dispatches no pointerdown, so use REAL mouse input at the
            // trigger's center. Coordinates come from page.evaluate — the
            // playwright locator path wedges on this page in long sessions
            // (locBox 30s timeouts on an element evaluate finds).
            let rect = page
                .evaluate(&format!(
                    "(() => {{ const e = document.getElementById('{}-trigger'); const r = e.getBoundingClientRect(); return [r.x + r.width/2, r.y + r.height/2] }})()",
                    trigger
                ))
                .map_err(|e| e.to_string())?;
            let coords = rect
                .as_array()
                .map(|a| (a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0)))
                .expect("rect is [x, y]");
            if std::env::var("EF_DEBUG").is_ok() {
                let v = page
                    .evaluate(&format!(
                        "(() => {{ const t = document.elementFromPoint({}, {}); return t ? t.tagName + '|' + (t.id || '') : 'none' }})()",
                        coords.0, coords.1
                    ))
                    .unwrap_or(serde_json::Value::Null);
                eprintln!("[dbg] nav click target: {}", v);
            }
            page.mouse_click(coords.0, coords.1)
                .map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(500);
            wait_true(page, &format!("!!document.querySelector('{}')", content))?;
            page.key_press("Escape").map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(500);
            wait_true(
                page,
                &format!("!document.querySelector('{}[data-state=open]')", content),
            )?;
            Ok(())
        }
        SelfTestAction::Inline => {
            let idx_v = page
                .evaluate_fn(
                    r#"() => [...document.querySelectorAll("[data-slot=tabs-trigger]")]
              .findIndex((t) => !t.disabled && t.getAttribute("data-state") !== "active")"#,
                )
                .map_err(|e| e.to_string())?;
            let idx = idx_v.as_i64().unwrap_or(-1);
            if idx < 0 {
                return Ok(());
            }
            page.loc_click("", "[data-slot=tabs-trigger]", idx, "left")
                .map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(300);
            let ok_v = page
                .evaluate_fn_arg(
                    r#"(i) => {
              const t = document.querySelectorAll("[data-slot=tabs-trigger]")[i]
              const p = document.getElementById(t.getAttribute("aria-controls"))
              return t.getAttribute("data-state") === "active" && (!p || !p.hasAttribute("hidden"))
            }"#,
                    json!(idx),
                )
                .unwrap_or(serde_json::Value::Null);
            if ok_v != serde_json::Value::Bool(true) {
                return Err(format!("tab {} did not activate", idx));
            }
            Ok(())
        }
        SelfTestAction::None { comp } => match comp.as_str() {
            "slider" => {
                let n = page.loc_count("", "[data-slot=slider-thumb]")?;
                if n == 0 {
                    return Ok(());
                }
                let (before, _) = page
                    .loc_attr("", "[data-slot=slider-thumb]", "aria-valuenow")
                    .unwrap_or((String::new(), false));
                let _ = page
                    .evaluate_fn_arg(
                        r#"(sel) => document.querySelector(sel).focus()"#,
                        json!("[data-slot=slider-thumb]"),
                    )
                    .unwrap_or(serde_json::Value::Null);
                page.key_press("ArrowRight").map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(200);
                let (after, _) = page
                    .loc_attr("", "[data-slot=slider-thumb]", "aria-valuenow")
                    .unwrap_or((String::new(), false));
                if before == after {
                    let dis_v = page
                        .evaluate_fn_arg(
                            r#"(sel) => { const t = document.querySelector(sel); return t.getAttribute("aria-disabled") === "true" || !!t.closest("[data-disabled]") }"#,
                            json!("[data-slot=slider-thumb]"),
                        )
                        .unwrap_or(serde_json::Value::Null);
                    if dis_v != serde_json::Value::Bool(true) {
                        return Err("slider thumb did not move on ArrowRight".to_string());
                    }
                }
                Ok(())
            }
            "carousel" => {
                let n = page.loc_count("", "[data-slot=carousel-next]:not([disabled])")?;
                if n == 0 {
                    return Ok(());
                }
                let before_v = page
                    .evaluate_fn(
                        r#"() => [...document.querySelectorAll("[data-slot=carousel-previous]")].map((b) => b.disabled).join()"#,
                    )
                    .unwrap_or(serde_json::Value::Null);
                page.loc_click("", "[data-slot=carousel-next]:not([disabled])", 0, "left")
                    .map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(500);
                let after_v = page
                    .evaluate_fn(
                        r#"() => [...document.querySelectorAll("[data-slot=carousel-previous]")].map((b) => b.disabled).join()"#,
                    )
                    .unwrap_or(serde_json::Value::Null);
                if before_v == after_v {
                    return Err("carousel did not scroll on next (previous button state unchanged)".to_string());
                }
                Ok(())
            }
            "scroll-area" => {
                let wired_v = page
                    .evaluate_fn(
                        r#"() => [...document.querySelectorAll("[data-slot=scroll-area-scrollbar]")].every((b) => b.hasAttribute("data-state") || b.style.length > 0 || b.querySelector("[data-slot=scroll-area-thumb]")?.style.length > 0)"#,
                    )
                    .unwrap_or(serde_json::Value::Null);
                if wired_v != serde_json::Value::Bool(true) {
                    return Err("scroll-area scrollbars not wired".to_string());
                }
                Ok(())
            }
            _ => Ok(()),
        },
    }
}

// ------------------------------------------------------------------ families

pub struct FamilyOut {
    pub body_html: String,
    pub templates: String,
    pub self_test: Option<SelfTestAction>,
    pub js_files: Vec<String>,
}

struct Ctx<'a> {
    page: &'a BPage<'a>,
    comp: String,
    fam: &'a FamilyEnt,
    id_map: &'a mut HashMap<String, String>,
    contracts: bool,
    def: Option<&'a EfDef>,
}

impl<'a> Ctx<'a> {
    fn root_html(&self) -> Result<String, String> {
        let v = self
            .page
            .evaluate_fn(r##"() => document.querySelector("#root").innerHTML"##)
            .map_err(|e| e.to_string())?;
        Ok(v.as_str().unwrap_or("").to_string())
    }
}

/// Dispatches the non-dialog families.
pub fn dispatch_family(
    page: &BPage<'_>,
    comp: &str,
    fam: &FamilyEnt,
    contracts: bool,
    def: Option<&EfDef>,
    id_map: &mut HashMap<String, String>,
    out: &mut FamilyOut,
) -> Result<(), String> {
    let mut ctx = Ctx {
        page,
        comp: comp.to_string(),
        fam,
        id_map,
        contracts,
        def,
    };
    // every family contributes its glue script (Go: family[f].js)
    if !fam.js.is_empty() {
        out.js_files.push(fam.js.to_string());
    }
    match fam.kind {
        "portal" => portal(&mut ctx, out),
        "menu" | "select" => menu_select(&mut ctx, out),
        "nav" => nav(&mut ctx, out),
        "inline" => inline(&mut ctx, out),
        _ => none(&mut ctx, out),
    }
}

fn portal(ctx: &mut Ctx<'_>, out: &mut FamilyOut) -> Result<(), String> {
    let comp = ctx.comp.clone();
    let trigger_sel = format!("[data-slot=\"{}-trigger\"]", comp);
    let content_sel = format!("[data-slot=\"{}-content\"]", comp);
    let count_v = ctx
        .page
        .evaluate_fn_arg(
            r##"(sel) => document.querySelectorAll("#root " + sel).length"##,
            json!(trigger_sel),
        )
        .map_err(|e| e.to_string())?;
    let count = count_v.as_i64().unwrap_or(0) as usize;
    if count == 0 {
        return Err("no trigger in the oracle render".to_string());
    }
    let mut parts: Vec<String> = Vec::new();
    let mut refs: Vec<bool> = vec![false; count];
    let mounted = |page: &BPage, sel: &str| -> Result<String, String> {
        let v = page
            .evaluate_fn_arg(
                r##"(sel) => {
              const c = document.querySelector(sel + ":not([data-ef-harvested])")
              if (!c) return null
              const top = c.closest("body > *") || c
              const html = top.outerHTML
              c.setAttribute("data-ef-harvested", "")
              return html
            }"##,
                json!(sel),
            )
            .map_err(|e| e.to_string())?;
        Ok(v.as_str().unwrap_or("").to_string())
    };
    let orig_v = ctx
        .page
        .evaluate_fn_arg(
            r##"(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id)"##,
            json!(trigger_sel),
        )
        .map_err(|e| e.to_string())?;
    let mut orig_ids: Vec<String> = Vec::new();
    if let Some(arr) = orig_v.as_array() {
        for e in arr {
            orig_ids.push(e.as_str().unwrap_or("").to_string());
        }
    }
    let prefix_of = |i: usize| -> String {
        if let Some(m) = super::example_fixture::word_trigger_prefix(&orig_ids[i]) {
            return m;
        }
        format!("k{}", i)
    };
    for i in 0..count {
        let act = |page: &BPage| -> Result<(), String> {
            if ctx.fam.open == "hover" {
                page.mouse_move(2.0, 2.0, 1).map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(400);
                let Some(b) = page
                    .loc_box("", &format!("#root {}", trigger_sel), i as i64)
                    .map_err(|e| e.to_string())?
                else {
                    return Err(format!("instance {}: no trigger box", i));
                };
                page.mouse_move(b.x + b.width / 2.0, b.y + b.height + 60.0, 1)
                    .map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(300);
                page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 6)
                    .map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(1100); // radix open delays
                return Ok(());
            }
            page.loc_click("", &format!("#root {}", trigger_sel), i as i64, "left")
                .map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(500);
            Ok(())
        };
        act(ctx.page)?;
        let mut portal_html = mounted(ctx.page, &content_sel)?;
        if portal_html.is_empty() {
            act(ctx.page)?;
            portal_html = mounted(ctx.page, &content_sel)?;
        }
        if portal_html.is_empty() {
            return Err(format!("instance {}: nothing mounted after {}", i, ctx.fam.open));
        }
        ef_learn(
            &portal_html,
            &[EfSlotStable {
                slot: format!("{}-content", comp),
                stable: prefix_of(i),
            }],
            ctx.id_map,
        );
        if orig_ids[i].starts_with("radix-") {
            ctx.id_map
                .insert(orig_ids[i].clone(), format!("{}-trigger", prefix_of(i)));
        }
        // does the open trigger point at its content? read WHILE OPEN
        let ref_v = ctx
            .page
            .evaluate_fn_arg(
                r##"(a) => { const t = document.querySelectorAll("#root " + a.sel)[a.i]; return t.hasAttribute("aria-describedby") || t.hasAttribute("aria-controls") }"##,
                json!({ "sel": trigger_sel, "i": i }),
            )
            .map_err(|e| e.to_string())?;
        refs[i] = ref_v == serde_json::Value::Bool(true);
        parts.push(ef_re_harvest_mark().replace_all(&portal_html, "").into_owned());
        ctx.page.key_press("Escape").map_err(|e| e.to_string())?;
        ctx.page.mouse_move(0.0, 0.0, 1).map_err(|e| e.to_string())?;
        let _ = ctx.page.wait_for_timeout(700);
    }
    let ids: Vec<String> = (0..count).map(|i| format!("{}-trigger", prefix_of(i))).collect();
    let prefixes: Vec<String> = (0..count).map(prefix_of).collect();
    ctx.page
        .evaluate_fn_arg(
            &format!(
                r##"({{ sel, ids }}) => document.querySelectorAll("#root " + sel).forEach((t, i) => {{ const o = t.id; t.id = ids[i]; {} }})"##,
                EF_RETARGET_JS
            ),
            json!({ "sel": trigger_sel, "ids": ids }),
        )
        .map_err(|e| e.to_string())?;
    let rh = ctx.root_html()?;
    out.body_html = ef_strip_radix_ids(&ef_remap(&rh, ctx.id_map));
    let mut tpls: Vec<String> = Vec::new();
    for (k, p) in parts.iter().enumerate() {
        let mut content = ef_strip_radix_ids(&ef_remap(p, ctx.id_map));
        if refs[k] {
            content = ef_ensure_content_id(&content, &ctx.comp, prefixes[k].as_str());
        }
        tpls.push(format!(
            "<template id=\"{}-portal\">\n{}\n</template>",
            prefixes[k], content
        ));
    }
    out.templates = tpls.join("\n");
    out.self_test = Some(SelfTestAction::Portal {
        prefix: prefixes[0].clone(),
        comp: comp.to_string(),
        hover: ctx.fam.open == "hover",
    });
    Ok(())
}

fn menu_select(ctx: &mut Ctx<'_>, out: &mut FamilyOut) -> Result<(), String> {
    let comp = ctx.comp.clone();
    let is_select = ctx.fam.kind == "select";
    let trigger_sel = format!("[data-slot=\"{}-trigger\"]", comp);
    let content_sel = format!("[data-slot=\"{}-content\"]", comp);
    let sub_trigger_sel = format!("[data-slot=\"{}-sub-trigger\"]", comp);
    // disabled triggers open nothing: skip them
    let enabled_v = ctx
        .page
        .evaluate_fn_arg(
            r##"(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => !t.disabled && t.getAttribute("aria-disabled") !== "true" && !t.hasAttribute("data-disabled"))"##,
            json!(trigger_sel),
        )
        .map_err(|e| e.to_string())?;
    let mut enabled: Vec<bool> = Vec::new();
    if let Some(arr) = enabled_v.as_array() {
        for e in arr {
            enabled.push(e.as_bool() == Some(true));
        }
    }
    let count = enabled.len();
    if count == 0 {
        return Err("no trigger in the oracle render".to_string());
    }
    struct Layer {
        layer_id: String,
        html: String,
    }
    let mut templates_: Vec<Layer> = Vec::new();
    fn mounted_content(page: &BPage, sel: &str) -> bool {
        page.evaluate_fn_arg(
            r##"(sel) => !!document.querySelector(sel + ":not([data-ef-harvested])")"##,
            json!(sel),
        )
        .map(|v| v == serde_json::Value::Bool(true))
        .unwrap_or(false)
    }
    fn harvest_layer(
        page: &BPage,
        comp: &str,
        sub_trigger_sel: &str,
        id_map: &mut HashMap<String, String>,
        templates_: &mut Vec<Layer>,
        layer_id: &str,
        sel: &str,
    ) -> Result<(), String> {
        if !mounted_content(page, sel) {
            return Err(format!("layer {}: nothing mounted", layer_id));
        }
        let v = page
            .evaluate_fn_arg(
                EF_HARVEST_LAYER,
                json!({ "sel": sel, "sub": sub_trigger_sel, "layerId": layer_id }),
            )
            .map_err(|e| e.to_string())?;
        let Some(m) = v.as_object() else {
            return Err(format!("layer {}: harvest failed", layer_id));
        };
        let html = m.get("html").and_then(|h| h.as_str()).unwrap_or("").to_string();
        let sub_count = m.get("subCount").and_then(|c| c.as_i64()).unwrap_or(0);
        ef_learn(
            &html,
            &[
                EfSlotStable { slot: format!("{}-content", comp), stable: layer_id.to_string() },
                EfSlotStable { slot: format!("{}-sub-content", comp), stable: layer_id.to_string() },
            ],
            id_map,
        );
        templates_.push(Layer {
            layer_id: layer_id.to_string(),
            html: ef_re_harvest_mark().replace_all(&html, "").into_owned(),
        });
        for j in 0..sub_count {
            let st = format!("#{}s{}-trigger", layer_id, j);
            let Some(b) = page.loc_box("", &st, 0).map_err(|e| e.to_string())? else {
                return Err(format!("layer {}: no sub-trigger box", layer_id));
            };
            // radix opens a sub menu on pointer movement over its trigger
            page.mouse_move(b.x + 4.0, b.y + b.height / 2.0, 3).map_err(|e| e.to_string())?;
            page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 6).map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(600);
            let sub_sel = format!("[data-slot=\"{}-sub-content\"]", comp);
            if !mounted_content(page, &sub_sel) {
                let _ = page.evaluate_fn_arg(
                    r##"(sel) => document.querySelector(sel).focus()"##,
                    json!(st),
                );
                page.key_press("ArrowRight").map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(500);
            }
            harvest_layer(
                page,
                comp,
                sub_trigger_sel,
                id_map,
                templates_,
                &format!("{}s{}", layer_id, j),
                &sub_sel,
            )?;
            // back to the parent layer: point away
            page.mouse_move(b.x + b.width / 2.0, b.y - 40.0, 4).map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(300);
        }
        Ok(())
    }
    let existing_v = ctx
        .page
        .evaluate_fn_arg(
            r##"(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => /^(\w+)-trigger$/.exec(t.id)?.[1] ?? null)"##,
            json!(trigger_sel),
        )
        .map_err(|e| e.to_string())?;
    let menu_orig_v = ctx
        .page
        .evaluate_fn_arg(
            r##"(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id)"##,
            json!(trigger_sel),
        )
        .map_err(|e| e.to_string())?;
    let mut existing: Vec<String> = Vec::new();
    let mut menu_orig: Vec<String> = Vec::new();
    if let Some(arr) = existing_v.as_array() {
        for e in arr {
            existing.push(e.as_str().unwrap_or("").to_string());
        }
    }
    if let Some(arr) = menu_orig_v.as_array() {
        for e in arr {
            menu_orig.push(e.as_str().unwrap_or("").to_string());
        }
    }
    let id_of = |i: usize| -> String {
        if !existing[i].is_empty() {
            return existing[i].clone();
        }
        if is_select {
            format!("s{}", i)
        } else {
            format!("m{}", i)
        }
    };
    for (i, o) in menu_orig.iter().enumerate() {
        if o.starts_with("radix-") {
            ctx.id_map
                .insert(o.clone(), format!("{}-trigger", id_of(i)));
        }
    }
    for i in 0..count {
        if !enabled[i] {
            continue;
        }
        if ctx.fam.open == "contextmenu" {
            let Some(b) = ctx
                .page
                .loc_box("", &format!("#root {}", trigger_sel), i as i64)
                .map_err(|e| e.to_string())?
            else {
                return Err(format!("instance {}: no trigger box", i));
            };
            ctx.page
                .mouse_click_button(b.x + b.width / 2.0, b.y + b.height / 2.0, "right")
                .map_err(|e| e.to_string())?;
        } else {
            ctx.page
                .loc_click("", &format!("#root {}", trigger_sel), i as i64, "left")
                .map_err(|e| e.to_string())?;
        }
        let _ = ctx.page.wait_for_timeout(500);
        harvest_layer(
            ctx.page,
            &comp,
            &sub_trigger_sel,
            ctx.id_map,
            &mut templates_,
            &id_of(i),
            &content_sel,
        )?;
        // close everything (Escape per open layer)
        for _ in 0..4 {
            ctx.page.key_press("Escape").map_err(|e| e.to_string())?;
            let _ = ctx.page.wait_for_timeout(150);
        }
        let _ = ctx.page.wait_for_timeout(400);
    }
    let ids: Vec<String> = (0..count).map(id_of).collect();
    ctx.page
        .evaluate_fn_arg(
            EF_MENU_IDS,
            json!({ "sel": trigger_sel, "isSelect": is_select, "attr": ctx.fam.attr, "ids": ids }),
        )
        .map_err(|e| e.to_string())?;
    let rh = ctx.root_html()?;
    out.body_html = ef_strip_radix_ids(&ef_remap(&rh, ctx.id_map));
    // sub-trigger original ids map to their stable ids
    for t in &templates_ {
        for m in ef_re_orig_and_id().captures_iter(&t.html) {
            ctx.id_map.insert(m[1].to_string(), m[2].to_string());
        }
    }
    let mut tpls: Vec<String> = Vec::new();
    for t in &templates_ {
        tpls.push(format!(
            "<template id=\"{}-tpl\">\n{}\n</template>",
            t.layer_id,
            ef_re_orig_attr()
                .replace_all(&ef_strip_radix_ids(&ef_remap(&t.html, ctx.id_map)), "")
                .into_owned()
        ));
    }
    out.templates = tpls.join("\n");
    let first_enabled = enabled.iter().position(|e| *e);
    let comp2 = comp.clone();
    let contextmenu2 = ctx.fam.open == "contextmenu";
    let trigger_id = first_enabled.map(id_of);
    let fam_attr = ctx.fam.attr.to_string();
    out.self_test = Some(super::fixture_families::SelfTestAction::MenuOrSelect {
        trigger: trigger_id.unwrap_or_default(),
        comp: comp2,
        contextmenu: contextmenu2,
    });
    let _ = fam_attr;
    Ok(())
}

fn nav(ctx: &mut Ctx<'_>, out: &mut FamilyOut) -> Result<(), String> {
    let trigger_sel = "[data-slot=\"navigation-menu-trigger\"]".to_string();
    let content_sel = "[data-slot=\"navigation-menu-content\"]".to_string();
    let count_v = ctx
        .page
        .evaluate_fn_arg(
            r##"(sel) => document.querySelectorAll("#root " + sel).length"##,
            json!(trigger_sel),
        )
        .map_err(|e| e.to_string())?;
    let count = count_v.as_i64().unwrap_or(0) as usize;
    if count == 0 {
        return Err("no trigger in the oracle render".to_string());
    }
    let mut parts: Vec<String> = Vec::new();
    let nav_orig_v = ctx
        .page
        .evaluate_fn_arg(
            r##"(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id)"##,
            json!(trigger_sel),
        )
        .map_err(|e| e.to_string())?;
    let mut nav_orig: Vec<String> = Vec::new();
    if let Some(arr) = nav_orig_v.as_array() {
        for e in arr {
            nav_orig.push(e.as_str().unwrap_or("").to_string());
        }
    }
    let nav_id_of = |i: usize| -> String {
        if let Some(m) = super::example_fixture::word_trigger_prefix(&nav_orig[i]) {
            return m;
        }
        format!("n{}", i)
    };
    for i in 0..count {
        ctx.page
            .loc_click("", &format!("#root {}", trigger_sel), i as i64, "left")
            .map_err(|e| e.to_string())?;
        let _ = ctx.page.wait_for_timeout(500);
        let v = ctx
            .page
            .evaluate_fn_arg(
                r##"(sel) => {
              const c = document.querySelector(sel + ":not([data-ef-harvested])")
              if (!c) return null
              const html = c.outerHTML; c.setAttribute("data-ef-harvested", ""); return html
            }"##,
                json!(content_sel),
            )
            .map_err(|e| e.to_string())?;
        let html = v.as_str().unwrap_or("").to_string();
        if html.is_empty() {
            return Err(format!("instance {}: no content mounted after click", i));
        }
        ef_learn(
            &html,
            &[EfSlotStable {
                slot: "navigation-menu-content".to_string(),
                stable: format!("{}-content", nav_id_of(i)),
            }],
            ctx.id_map,
        );
        if nav_orig[i].starts_with("radix-") {
            ctx.id_map.insert(nav_orig[i].clone(), format!("{}-trigger", nav_id_of(i)));
        }
        parts.push(ef_re_harvest_mark().replace_all(&html, "").into_owned());
        ctx.page.key_press("Escape").map_err(|e| e.to_string())?;
        let _ = ctx.page.wait_for_timeout(400);
    }
    ctx.page
        .evaluate_fn_arg(EF_NAV_IDS, json!(trigger_sel))
        .map_err(|e| e.to_string())?;
    let rh = ctx.root_html()?;
    out.body_html = ef_strip_radix_ids(&ef_remap(&rh, ctx.id_map));
    let nav_ids_v = ctx
        .page
        .evaluate_fn_arg(
            r##"(sel) => [...document.querySelectorAll("#root " + sel)].map((t) => t.id.replace(/-trigger$/, ""))"##,
            json!(trigger_sel),
        )
        .map_err(|e| e.to_string())?;
    let mut nav_ids: Vec<String> = Vec::new();
    if let Some(arr) = nav_ids_v.as_array() {
        for e in arr {
            nav_ids.push(e.as_str().unwrap_or("").to_string());
        }
    }
    let mut tpls: Vec<String> = Vec::new();
    for (k, p) in parts.iter().enumerate() {
        tpls.push(format!(
            "<template id=\"{}-content-tpl\">\n{}\n</template>",
            nav_ids[k],
            ef_strip_radix_ids(&ef_remap(p, ctx.id_map))
        ));
    }
    out.templates = tpls.join("\n");
    out.self_test = Some(SelfTestAction::Nav {
        trigger: nav_ids[0].clone(),
        content: content_sel.clone(),
    });
    Ok(())
}

fn inline(ctx: &mut Ctx<'_>, out: &mut FamilyOut) -> Result<(), String> {
    let v = ctx.page.evaluate_fn(EF_TABS_DRIVER).map_err(|e| e.to_string())?;
    out.body_html = v.as_str().unwrap_or("").to_string();
    out.self_test = Some(SelfTestAction::Inline);
    Ok(())
}

fn none(ctx: &mut Ctx<'_>, out: &mut FamilyOut) -> Result<(), String> {
    if ctx.contracts {
        if let Some(def) = ctx.def {
            if !def.open.is_empty() {
                ctx.page.driver(&def.open).map_err(|e| e.to_string())?;
                let _ = ctx.page.wait_for_timeout(500);
            }
        }
    }
    let rh = ctx.root_html()?;
    out.body_html = ef_strip_radix_ids(&rh);
    out.self_test = Some(SelfTestAction::None {
        comp: ctx.comp.clone(),
    });
    Ok(())
}
