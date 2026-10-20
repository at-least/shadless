//! The per-family fixture build (portal/menu/select/nav/inline/none) —
//! the rest of example_fixture.go's switch. Self-tests are stored as data
//! (SelfTestAction) rather than closures: every Go closure captured only
//! (page, comp, prefix/ids, open-gesture) — all value-shaped.

use super::browser_shell::BPage;
use super::example_fixture::{EF_HARVEST_LAYER, EF_MENU_IDS, EF_NAV_IDS, EF_TABS_DRIVER};
use super::example_fixture::{
    EF_RE_HARVEST_MARK, EF_RE_ORIG_AND_ID, EF_RE_ORIG_ATTR, EF_RETARGET_JS, EfDef, EfSlotStable,
    ef_ensure_content_id, ef_learn, ef_remap, ef_strip_radix_ids,
};
use super::families::FamilyEnt;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

pub enum SelfTestAction {
    Dialog {
        comp: String,
    },
    Portal {
        prefix: String,
        comp: String,
        hover: bool,
    },
    MenuOrSelect {
        trigger: String,
        comp: String,
        contextmenu: bool,
    },
    Nav {
        trigger: String,
        content: String,
    },
    Inline,
    None {
        comp: String,
    },
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
        SelfTestAction::Portal {
            prefix,
            comp,
            hover,
        } => {
            let sel = format!("#{}-trigger", prefix);
            if *hover {
                let Some(b) = page.loc_box("", &sel, 0).map_err(|e| e.to_string())? else {
                    return Err("no trigger box".to_string());
                };
                page.mouse_move(b.x + b.width / 2.0, b.y + b.height + 60.0, 1)
                    .map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(300);
                page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 6)
                    .map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(1100);
            } else {
                page.loc_click("", &sel, 0, "left")
                    .map_err(|e| e.to_string())?;
                let _ = page.wait_for_timeout(1500);
            }
            wait_true(
                page,
                &format!(
                    "!!document.querySelector(\"[data-slot='{}-content']\")",
                    comp
                ),
            )?;
            page.key_press("Escape").map_err(|e| e.to_string())?;
            page.mouse_move(0.0, 0.0, 1).map_err(|e| e.to_string())?;
            let _ = page.wait_for_timeout(700);
            wait_true(
                page,
                &format!(
                    "!document.querySelector(\"[data-slot='{}-content']\")",
                    comp
                ),
            )?;
            Ok(())
        }
        SelfTestAction::MenuOrSelect {
            trigger,
            comp,
            contextmenu,
        } => {
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
                page.loc_click("", &sel, 0, "left")
                    .map_err(|e| e.to_string())?;
            }
            let _ = page.wait_for_timeout(500);
            let content_sel = format!("[data-slot=\"{}-content\"]", comp);
            wait_true(
                page,
                &format!(
                    "!!document.querySelector('{}[data-state=open]')",
                    content_sel
                ),
            )?;
            if comp == "select" {
                select_check_follows_pick(page, &sel, &content_sel)?;
            } else {
                menu_check_follows_pick(page, &sel, *contextmenu, &content_sel)?;
            }
            let n = page.loc_count("", &format!("[data-slot=\"{}-sub-trigger\"]", comp))?;
            if n > 0 {
                if let Some(b) = page
                    .loc_box("", &format!("[data-slot=\"{}-sub-trigger\"]", comp), 0)
                    .map_err(|e| e.to_string())?
                {
                    page.mouse_move(b.x + 4.0, b.y + b.height / 2.0, 3)
                        .map_err(|e| e.to_string())?;
                    page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 6)
                        .map_err(|e| e.to_string())?;
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
                &format!(
                    "!document.querySelector('{}[data-state=open]')",
                    content_sel
                ),
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
                    return Err(
                        "carousel did not scroll on next (previous button state unchanged)"
                            .to_string(),
                    );
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
    root: &'a Path,
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
// one argument per Ctx field: the call site has them all as separate values
#[allow(clippy::too_many_arguments)]
pub fn dispatch_family(
    page: &BPage<'_>,
    root: &Path,
    comp: &str,
    fam: &FamilyEnt,
    contracts: bool,
    def: Option<&EfDef>,
    id_map: &mut HashMap<String, String>,
    out: &mut FamilyOut,
) -> Result<(), String> {
    let mut ctx = Ctx {
        page,
        root,
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
            return Err(format!(
                "instance {}: nothing mounted after {}",
                i, ctx.fam.open
            ));
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
        parts.push(
            EF_RE_HARVEST_MARK
                .replace_all(&portal_html, "")
                .into_owned(),
        );
        ctx.page.key_press("Escape").map_err(|e| e.to_string())?;
        ctx.page
            .mouse_move(0.0, 0.0, 1)
            .map_err(|e| e.to_string())?;
        let _ = ctx.page.wait_for_timeout(700);
    }
    let ids: Vec<String> = (0..count)
        .map(|i| format!("{}-trigger", prefix_of(i)))
        .collect();
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
        let html = m
            .get("html")
            .and_then(|h| h.as_str())
            .unwrap_or("")
            .to_string();
        let sub_count = m.get("subCount").and_then(|c| c.as_i64()).unwrap_or(0);
        ef_learn(
            &html,
            &[
                EfSlotStable {
                    slot: format!("{}-content", comp),
                    stable: layer_id.to_string(),
                },
                EfSlotStable {
                    slot: format!("{}-sub-content", comp),
                    stable: layer_id.to_string(),
                },
            ],
            id_map,
        );
        templates_.push(Layer {
            layer_id: layer_id.to_string(),
            html: EF_RE_HARVEST_MARK.replace_all(&html, "").into_owned(),
        });
        for j in 0..sub_count {
            let st = format!("#{}s{}-trigger", layer_id, j);
            let Some(b) = page.loc_box("", &st, 0).map_err(|e| e.to_string())? else {
                return Err(format!("layer {}: no sub-trigger box", layer_id));
            };
            // radix opens a sub menu on pointer movement over its trigger
            page.mouse_move(b.x + 4.0, b.y + b.height / 2.0, 3)
                .map_err(|e| e.to_string())?;
            page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 6)
                .map_err(|e| e.to_string())?;
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
            page.mouse_move(b.x + b.width / 2.0, b.y - 40.0, 4)
                .map_err(|e| e.to_string())?;
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
        for m in EF_RE_ORIG_AND_ID.captures_iter(&t.html) {
            ctx.id_map.insert(m[1].to_string(), m[2].to_string());
        }
    }
    let mut tpls: Vec<String> = Vec::new();
    for t in &templates_ {
        tpls.push(format!(
            "<template id=\"{}-tpl\">\n{}\n</template>",
            t.layer_id,
            EF_RE_ORIG_ATTR
                .replace_all(&ef_strip_radix_ids(&ef_remap(&t.html, ctx.id_map)), "")
                .into_owned()
        ));
    }
    let first_enabled = enabled.iter().position(|e| *e);
    // radix renders a select's check (ItemIndicator) only inside the selected
    // item, so a page with nothing selected harvests none and could never
    // show one after a pick: pick an item in the oracle (the body is already
    // captured) and harvest the check it renders
    if is_select
        && !templates_
            .iter()
            .any(|t| t.html.contains("data-state=\"checked\""))
        && let Some(i) = first_enabled
    {
        tpls.push(format!(
            "<template data-for=\"select-item-indicator\">\n{}\n</template>",
            ef_strip_radix_ids(&harvest_select_indicator(
                ctx.page,
                &trigger_sel,
                i,
                &content_sel
            )?)
        ));
    }
    // the same for a menu whose checkbox / radio items all start unchecked
    // (context-menu-checkboxes): no item carries a check to copy
    let checkable =
        |h: &str| h.contains("role=\"menuitemcheckbox\"") || h.contains("role=\"menuitemradio\"");
    if !is_select
        && templates_.iter().any(|t| checkable(&t.html))
        && !templates_
            .iter()
            .any(|t| t.html.contains("aria-checked=\"true\""))
    {
        if ctx.contracts {
            return Err(format!(
                "{}: the contract usage must hold a checked item — it is where the family's check is taken from",
                comp
            ));
        }
        tpls.push(format!(
            "<template data-for=\"menu-item-indicator\">\n{}\n</template>",
            family_menu_indicator(ctx.root, &comp)?
        ));
    }
    out.templates = tpls.join("\n");
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

/// On the open select: pick an enabled item that is not selected, reopen,
/// and require the check in that item's wrapper and in no other — the page
/// must show the check where React would, whether its fixture started with
/// a selection (the check moves) or without (it mounts from the template).
fn select_check_follows_pick(page: &BPage, trigger: &str, content_sel: &str) -> Result<(), String> {
    let pick = format!(
        "{} [role=option]:not([data-disabled]):not([data-state=checked])",
        content_sel
    );
    if page.loc_count("", &pick)? == 0 {
        return Ok(());
    }
    page.loc_click("", &pick, 0, "left")
        .map_err(|e| e.to_string())?;
    let _ = page.wait_for_timeout(500);
    page.loc_click("", trigger, 0, "left")
        .map_err(|e| e.to_string())?;
    let _ = page.wait_for_timeout(500);
    let v = page
        .evaluate_fn_arg(
            r##"(sel) => [...document.querySelectorAll(sel + " [role=option]")].map((o) => o.getAttribute("data-state") + ":" + (o.firstElementChild?.childElementCount ?? 0)).join(",")"##,
            json!(content_sel),
        )
        .map_err(|e| e.to_string())?;
    let got = v.as_str().unwrap_or("").to_string();
    let items: Vec<&str> = got.split(',').collect();
    let ok = items
        .iter()
        .all(|o| *o == "checked:1" || *o == "unchecked:0");
    if !ok || !items.contains(&"checked:1") {
        return Err(format!(
            "select: after a pick the check is not in the picked item alone (data-state:checks per item = {})",
            got
        ));
    }
    Ok(())
}

/// Open a menu: right-click a context menu's trigger, click any other.
fn open_menu(page: &BPage, trigger: &str, index: usize, contextmenu: bool) -> Result<(), String> {
    if contextmenu {
        let Some(b) = page
            .loc_box("", trigger, index as i64)
            .map_err(|e| e.to_string())?
        else {
            return Err(format!("no trigger box for {}", trigger));
        };
        page.mouse_click_button(b.x + b.width / 2.0, b.y + b.height / 2.0, "right")
            .map_err(|e| e.to_string())?;
    } else {
        page.loc_click("", trigger, index as i64, "left")
            .map_err(|e| e.to_string())?;
    }
    let _ = page.wait_for_timeout(500);
    Ok(())
}

/// On the open menu: pick an enabled checkbox / radio item that is not
/// checked, reopen, and require it checked — radix flips the item on select
/// and renders the check (ItemIndicator) in checked items only, so every
/// item's check must match its state.
fn menu_check_follows_pick(
    page: &BPage,
    trigger: &str,
    contextmenu: bool,
    content_sel: &str,
) -> Result<(), String> {
    let items = format!(
        "{c} [role=menuitemcheckbox], {c} [role=menuitemradio]",
        c = content_sel
    );
    let pick = format!(
        "{c} [role=menuitemcheckbox]:not([data-disabled]):not([data-state=checked]), {c} [role=menuitemradio]:not([data-disabled]):not([data-state=checked])",
        c = content_sel
    );
    if page.loc_count("", &pick)? == 0 {
        return Ok(());
    }
    let at = page
        .evaluate_fn_arg(
            r##"(a) => [...document.querySelectorAll(a.items)].indexOf(document.querySelector(a.pick))"##,
            json!({ "items": items, "pick": pick }),
        )
        .map_err(|e| e.to_string())?
        .as_u64()
        .ok_or("menu: the picked item is not among the menu's items")? as usize;
    page.loc_click("", &pick, 0, "left")
        .map_err(|e| e.to_string())?;
    let _ = page.wait_for_timeout(500);
    open_menu(page, trigger, 0, contextmenu)?;
    let v = page
        .evaluate_fn_arg(
            r##"(sel) => [...document.querySelectorAll(sel)].map((o) => o.getAttribute("data-state") + ":" + (o.firstElementChild?.childElementCount ?? 0))"##,
            json!(items),
        )
        .map_err(|e| e.to_string())?;
    let got: Vec<&str> = v
        .as_array()
        .map(|a| a.iter().filter_map(|x| x.as_str()).collect())
        .unwrap_or_default();
    let ok = got.iter().all(|o| *o == "checked:1" || *o == "unchecked:0");
    if !ok || got.get(at) != Some(&"checked:1") {
        return Err(format!(
            "menu: after picking item #{} the checks do not follow the state (data-state:checks per item = {})",
            at,
            got.join(",")
        ));
    }
    Ok(())
}

/// The check radix mounts in a checked item of this menu family, from the
/// family's contract fixture — src/kernel/<comp>.html is the contract
/// usage's own React render, and that usage holds a checked item. A page
/// whose items all start unchecked has no check to copy, and its React
/// example may never show one: context-menu-checkboxes passes
/// `defaultChecked`, which radix's CheckboxItem does not take.
fn family_menu_indicator(root: &Path, comp: &str) -> Result<String, String> {
    let path = format!("src/kernel/{}.html", comp);
    let html = std::fs::read_to_string(root.join(&path)).map_err(|e| format!("{}: {}", path, e))?;
    let missing = || format!("{}: no checked menu item to take the check from", path);
    let rest = &html[html.find("aria-checked=\"true\"").ok_or_else(missing)?..];
    let start = rest
        .find("<span data-state=\"checked\">")
        .ok_or_else(missing)?;
    let end = "</svg></span>";
    let len = rest[start..].find(end).ok_or_else(missing)? + end.len();
    Ok(rest[start..start + len].to_string())
}

/// Select the first enabled item of the `index`th select in the oracle and
/// return the indicator radix then mounts in it: the selected item's
/// wrapper's child, as the fixtures carry it.
fn harvest_select_indicator(
    page: &BPage,
    trigger_sel: &str,
    index: usize,
    content_sel: &str,
) -> Result<String, String> {
    let trigger = format!("#root {}", trigger_sel);
    let live = format!("{}:not([data-ef-harvested])", content_sel);
    page.loc_click("", &trigger, index as i64, "left")
        .map_err(|e| e.to_string())?;
    let _ = page.wait_for_timeout(500);
    page.loc_click(
        "",
        &format!("{} [role=option]:not([data-disabled])", live),
        0,
        "left",
    )
    .map_err(|e| e.to_string())?;
    let _ = page.wait_for_timeout(500);
    page.loc_click("", &trigger, index as i64, "left")
        .map_err(|e| e.to_string())?;
    let _ = page.wait_for_timeout(500);
    let v = page
        .evaluate_fn_arg(
            r##"(sel) => document.querySelector(sel + " [role=option][data-state=checked]")?.firstElementChild?.firstElementChild?.outerHTML ?? null"##,
            json!(live),
        )
        .map_err(|e| e.to_string())?;
    page.key_press("Escape").map_err(|e| e.to_string())?;
    let _ = page.wait_for_timeout(400);
    v.as_str()
        .map(str::to_string)
        .ok_or_else(|| "select: the picked item mounted no indicator in the oracle".to_string())
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
            ctx.id_map
                .insert(nav_orig[i].clone(), format!("{}-trigger", nav_id_of(i)));
        }
        parts.push(EF_RE_HARVEST_MARK.replace_all(&html, "").into_owned());
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
    let v = ctx
        .page
        .evaluate_fn(EF_TABS_DRIVER)
        .map_err(|e| e.to_string())?;
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
