//! Port of pipeline/contract.go — tools/contracts/run.mjs: record facts from
//! the shadcn oracle (React, real browser) and from the shadless page, diff
//! after normalizing recorded differences.
//!
//! oracleOpen/shadlessOpen in result.json are the RAW recorder facts; the
//! open-state diff compares the NORMALIZED (cBuildFact) projection of the
//! same two objects. Two independent decodes of the same wire bytes, on
//! purpose — cRawToJsonable for the byte-identical persisted JSON,
//! cBuildFact for the structured comparison.

use super::browser_shell::{BPage, BrowserShell};
use super::example_fixture::{build_contract_oracle, EfDef};
use crate::jsonorder::{json_string, Json, JsonObj};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::OnceLock;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Cdef {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub usage: String,
    #[serde(default)]
    pub imports: String,
    #[serde(default)]
    pub slots: Vec<String>,
    #[serde(default)]
    pub open: String,
    #[serde(rename = "openShadless", default)]
    pub open_shadless: String,
    #[serde(rename = "mountedClasses", default)]
    pub mounted_classes: Option<bool>,
    #[serde(rename = "mountedCheck", default)]
    pub mounted_check: Option<bool>,
    #[serde(rename = "shadlessPage", default)]
    pub shadless_page: String,
    #[serde(default)]
    pub scenarios: Vec<String>,
    #[serde(rename = "triggerSlot", default)]
    pub trigger_slot: String,
    #[serde(rename = "stateProbe", default)]
    pub state_probe: String,
    #[serde(rename = "oracleCss", default)]
    pub oracle_css: String,
    #[serde(rename = "ignoreAttrs", default)]
    pub ignore_attrs: HashMap<String, Vec<String>>,
    #[serde(rename = "closeSelector", default)]
    pub close_selector: String,
    #[serde(rename = "overlaySlot", default)]
    pub overlay_slot: String,
    #[serde(rename = "contentSlot", default)]
    pub content_slot: String,
}

/// Go tolerates `null` for any field (json.Unmarshal leaves the default);
/// serde does not — same drop_nulls normalization the emit chain uses.
fn drop_nulls(v: Value) -> Value {
    match v {
        Value::Object(m) => Value::Object(
            m.into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, drop_nulls(v)))
                .collect(),
        ),
        Value::Array(a) => Value::Array(a.into_iter().map(drop_nulls).collect()),
        other => other,
    }
}

pub fn c_load_def(shell: &BrowserShell, root: &Path, name: &str) -> Result<Cdef, String> {
    let file = abs_or_die(root, &format!("tools/contracts/components/{}.mjs", name));
    let res = shell.call(&serde_json::json!({
        "op": "loadContractDef",
        "file": format!("file://{}", file.to_string_lossy()),
    }))?;
    let def_v = res.get("def").cloned().unwrap_or(Value::Null);
    serde_json::from_value(drop_nulls(def_v)).map_err(|e| e.to_string())
}

/// Go absOrDie (style_parity.go:411): filepath.Abs against the process cwd —
/// the pipeline always runs from the repo root.
pub fn abs_or_die(root: &Path, p: &str) -> std::path::PathBuf {
    let q = Path::new(p);
    if q.is_absolute() {
        q.to_path_buf()
    } else {
        root.join(q)
    }
}

// ---------- recorder (injected into both pages) -----------------------------

pub fn c_recorder_src(slots: &[String]) -> String {
    let parts: Vec<String> = slots.iter().map(|s| json_string(s)).collect();
    let slots_json = format!("[{}]", parts.join(","));
    format!(
        r#"
window.__facts = function (tag) {{
  var doc = document;
  function attrs(el) {{
    if (!el) return null;
    var o = {{ tag: el.tagName.toLowerCase() }};
    for (var a of el.getAttributeNames()) if (a !== "class") o[a] = el.getAttribute(a);
    o.text = (el.textContent || "").trim().replace(/\s+/g, "").slice(0, 24);
    return o;
  }}
  var f = {{ step: tag }};
  {slots}.forEach(function (s) {{
    var el = s.charAt(0) === "&"
      ? doc.querySelector(s.slice(1)) // "&<raw-css>" — full selector
      : doc.querySelector("[data-slot=" + s + "]");
    f[s] = attrs(el);
  }});
  f.activeElement = doc.activeElement
    ? (doc.activeElement.getAttribute("data-slot") ||
       doc.activeElement.tagName.toLowerCase()) : null;
  f.scrollLock = {{
    attr: doc.body.getAttribute("data-scroll-locked"),
    pointerEvents: doc.body.style.pointerEvents,
  }};
  return f;
}};
"#,
        slots = slots_json
    )
}

// ---------- shadless page (relative-path rewrite until emitter lands) -------

fn re_contract_attr_path() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"(src|href)="([^"]+)""#).unwrap())
}

pub fn c_rewrite_relative_paths(html: &str, dir: &str) -> String {
    re_contract_attr_path()
        .replace_all(html, |m: &regex::Captures| {
            let (k, v) = (&m[1], &m[2]);
            if v.starts_with("http:")
                || v.starts_with("https:")
                || v.starts_with("file:")
                || v.starts_with("data:")
                || v.starts_with("//")
            {
                return m[0].to_string();
            }
            format!("{}=\"{}/{}\"", k, dir, v)
        })
        .into_owned()
}

// ---------- normalization of recorded differences ---------------------------

fn re_contract_auto() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    // Go \w \d are ASCII; Rust regex classes are Unicode — spell the ASCII
    // sets out.
    R.get_or_init(|| Regex::new(r"^(radix-[0-9A-Za-z_:-]*|[a-z]+[0-9][0-9A-Za-z_-]*)$").unwrap())
}
fn re_contract_outline_none() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^outline:[\t\n\f\r ]*none$").unwrap())
}
fn re_contract_pointer_auto() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^pointer-events:[\t\n\f\r ]*auto$").unwrap())
}
fn re_contract_ws() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"[\t\n\f\r ]+").unwrap())
}

/// normVal, ported from tools/contracts/run.mjs.
pub fn c_norm_val(v: &str, key: &str) -> String {
    if key == "style" {
        let mut kept: Vec<String> = Vec::new();
        for s in v.split(';') {
            let s = s.trim();
            if s.is_empty()
                || s.starts_with("--radix-")
                || re_contract_outline_none().is_match(s)
                || re_contract_pointer_auto().is_match(s)
            {
                continue;
            }
            kept.push(s.to_string());
        }
        return kept.join("; ");
    }
    if re_contract_auto().is_match(v) {
        return "<auto-id>".to_string();
    }
    let parts: Vec<&str> = re_contract_ws().split(v).collect();
    let mut all_auto = !v.is_empty();
    for p in &parts {
        if p.is_empty() {
            continue;
        }
        if !re_contract_auto().is_match(p) {
            all_auto = false;
            break;
        }
    }
    if all_auto {
        return parts
            .iter()
            .map(|_| "<auto-id>".to_string())
            .collect::<Vec<_>>()
            .join(" ");
    }
    v.to_string()
}

// ---------- normalized fact structures --------------------------------------

/// cAttrObj: one slot's NORMALIZED element facts — normFact's rebuilt `el`.
#[derive(Default)]
pub struct CAttrObj {
    keys: Vec<String>,
    val: HashMap<String, String>,
}

/// attrs()+normFact's per-slot rebuild, together: decode the wire object and
/// apply normVal in the same pass. (serde_json with preserve_order keeps the
/// wire key order where Go used decodeOrderedObject.)
fn c_build_attr_obj(raw: &Value) -> Result<Option<CAttrObj>, String> {
    if raw.is_null() {
        return Ok(None);
    }
    let obj = raw
        .as_object()
        .ok_or_else(|| "attr slot is not an object".to_string())?;
    let mut out = CAttrObj::default();
    for (k, v) in obj {
        if k == "data-radix-collection-item" {
            continue;
        }
        let s = v.as_str().ok_or("attr value not a string")?;
        if k == "tag" {
            out.keys.push(k.clone());
            out.val.insert(k.clone(), s.to_string());
            continue;
        }
        let nv = c_norm_val(s, k);
        if k == "style" && nv.is_empty() {
            continue;
        }
        out.keys.push(k.clone());
        out.val.insert(k.clone(), nv);
    }
    Ok(Some(out))
}

/// cFact: the NORMALIZED projection of one side's recorder facts, keyed for
/// the open-state diff.
#[derive(Default)]
pub struct CFact {
    slots: HashMap<String, Option<CAttrObj>>,
    active_element: Option<String>,
    scroll_lock_attr: Option<String>,
    scroll_lock_pe: String,
}

pub fn c_build_fact(raw: &Value, slots: &[String]) -> Result<CFact, String> {
    let mut f = CFact::default();
    for s in slots {
        if let Some(r) = raw.get(s) {
            let el = c_build_attr_obj(r)?;
            f.slots.insert(s.clone(), el);
        }
    }
    if let Some(r) = raw.get("activeElement") {
        if !r.is_null() {
            let s = r.as_str().ok_or("activeElement not a string")?;
            f.active_element = Some(s.to_string());
        }
    }
    if let Some(sl) = raw.get("scrollLock") {
        if let Some(attr) = sl.get("attr") {
            if !attr.is_null() {
                let s = attr.as_str().ok_or("scrollLock.attr not a string")?;
                f.scroll_lock_attr = Some(s.to_string());
            }
        }
        // Go ignores the unmarshal error here (json.Unmarshal(pr, &pe))
        if let Some(pe) = sl.get("pointerEvents").and_then(|v| v.as_str()) {
            f.scroll_lock_pe = pe.to_string();
        }
    }
    Ok(f)
}

/// rawToJsonable decodes wire values into the jsonorder value tree, keeping
/// key order — the UNNORMALIZED facts persisted to result.json. Facts only
/// ever contain objects, strings and nulls; anything else is an error (as in
/// Go).
pub fn c_raw_to_jsonable(raw: &Value) -> Result<Json, String> {
    match raw {
        Value::Null => Ok(Json::Null),
        Value::String(s) => Ok(Json::Str(s.clone())),
        Value::Object(obj) => {
            let mut out = JsonObj::new();
            for (k, v) in obj {
                let jv = c_raw_to_jsonable(v)?;
                out = out.add(k, jv);
            }
            Ok(Json::from_obj(out))
        }
        other => Err(format!("cRawToJsonable: unexpected value {}", other)),
    }
}

fn c_union_keys_ordered(a: Option<&CAttrObj>, b: Option<&CAttrObj>) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for el in [a, b].into_iter().flatten() {
        for k in &el.keys {
            if seen.insert(k.clone()) {
                out.push(k.clone());
            }
        }
    }
    out
}

fn c_lookup_attr(o: Option<&CAttrObj>, k: &str) -> Option<String> {
    o.and_then(|o| o.val.get(k).cloned())
}

fn c_show_maybe(present: bool, v: &str) -> String {
    if !present {
        return "undefined".to_string();
    }
    json_string(v)
}

// ---------- scenarios --------------------------------------------------------

fn c_overlay_point(
    page: &BPage,
    overlay_slot: &str,
    content_slot: &str,
) -> Result<(f64, f64), String> {
    let ov = page.loc_box("", &format!("[data-slot={}]", overlay_slot), 0)?;
    let ct = page.loc_box("", &format!("[data-slot={}]", content_slot), 0)?;
    let (ov_some, ct_some) = (ov.is_some(), ct.is_some());
    let (Some(ov), Some(ct)) = (ov, ct) else {
        return Err(format!(
            "overlay-point: element not visible (overlay={} content={})",
            ov_some, ct_some
        ));
    };
    let (mut x, mut y) = (ov.x + 15.0, ov.y + 15.0);
    let in_ct = |px: f64, py: f64| {
        px >= ct.x && px <= ct.x + ct.width && py >= ct.y && py <= ct.y + ct.height
    };
    if in_ct(x, y) {
        x = ov.x + ov.width - 15.0;
        y = ov.y + 15.0;
    }
    if in_ct(x, y) {
        return Err("no overlay-only point".to_string());
    }
    Ok((x, y))
}

pub fn or_default(s: &str, def: &str) -> String {
    if !s.is_empty() {
        s.to_string()
    } else {
        def.to_string()
    }
}

/// cClosedStart: a scenario with this prefix runs from the CLOSED state — the
/// harness otherwise opens the component before every step, which cannot
/// express "does the keyboard open it at all".
const C_CLOSED_PREFIX: &str = "closed:";

fn c_closed_start(step: &str) -> bool {
    step.starts_with(C_CLOSED_PREFIX)
}

/// Go's strconv.ParseFloat with the error discarded (`fx, _ :=`): a bad
/// fraction parses as 0.
fn parse_f64_go(s: &str) -> f64 {
    s.parse().unwrap_or(0.0)
}

/// stepIt: ops chain ("focus:#t1+key:ArrowRight"); legacy single-op names
/// still work.
fn c_step_it(page: &BPage, def: &Cdef, step: &str) -> Result<String, String> {
    let step = step.strip_prefix(C_CLOSED_PREFIX).unwrap_or(step);
    for op in step.split('+') {
        let op = op.to_string();
        if op == "overlay-mouse-click" {
            let (x, y) = c_overlay_point(
                page,
                &or_default(&def.overlay_slot, "dialog-overlay"),
                &or_default(&def.content_slot, "dialog-content"),
            )?;
            page.mouse_click(x, y)?;
        } else if op == "escape" {
            page.key_press("Escape")?;
        } else if op == "close-button" {
            page.loc_click_timeout(
                "",
                &or_default(&def.close_selector, "[data-slot=dialog-close]"),
                0,
                "left",
                30000,
            )?;
        } else if op == "outside-click" {
            page.mouse_click(5.0, 5.0)?;
        } else if op == "pointer-away" {
            // real pointers move continuously; a single-jump move races
            // radix's async grace-tracker attach (stays open forever) —
            // artifact, not semantics
            page.mouse_move(5.0, 5.0, 10)?;
            page.wait_for_timeout(400)?;
        } else if op == "trigger-toggle" {
            page.loc_click_timeout(
                "",
                &format!("[data-slot={}]", def.trigger_slot),
                0,
                "left",
                30000,
            )?;
        } else if let Some(sel) = op.strip_prefix("js-click:") {
            // radix modal dropdown: body pointer-events:none while open — a
            // real playwright click on the trigger never lands; dispatch a
            // DOM click
            page.loc_eval("", sel, "(el) => el.click()", 0)?;
        } else if let Some(css) = op.strip_prefix("mouse-click:") {
            // mouse click at element center — bypasses playwright
            // actionability
            let Some(b) = page.loc_box("", css, 0)? else {
                return Err(format!("mouse-click: {} not visible", css));
            };
            page.mouse_click(b.x + b.width / 2.0, b.y + b.height / 2.0)?;
        } else if let Some(rest) = op.strip_prefix("clickAt:") {
            // click at x%,y% inside element box (e.g. track clicks on a
            // slider)
            let (sel, xy) = rest.split_once('@').ok_or("clickAt: missing @")?;
            let (fx, fy) = xy.split_once(',').unwrap_or((xy, ""));
            let (fx, fy) = (parse_f64_go(fx), parse_f64_go(fy));
            let Some(b) = page.loc_box("", sel, 0)? else {
                return Err(format!("clickAt: {} not visible", sel));
            };
            page.mouse_click(b.x + b.width * fx / 100.0, b.y + b.height * fy / 100.0)?;
        } else if let Some(css) = op.strip_prefix("move:") {
            // move pointer to element center (hover state)
            let Some(b) = page.loc_box("", css, 0)? else {
                return Err(format!("move: {} not visible", css));
            };
            page.mouse_move(b.x + b.width / 2.0, b.y + b.height / 2.0, 5)?;
        } else if let Some(xy) = op.strip_prefix("wheel:") {
            let (dx, dy) = xy.split_once(',').unwrap_or((xy, ""));
            page.wheel(parse_f64_go(dx), parse_f64_go(dy))?;
        } else if let Some(sel) = op.strip_prefix("click:") {
            // generic steps: click:<css> / focus:<css> / key:<Key>
            page.loc_click_timeout("", sel, 0, "left", 30000)?;
        } else if let Some(sel) = op.strip_prefix("focus:") {
            page.focus(sel, 30000)?;
            // radix roving-focus moves via rAF — settle
            page.wait_for_timeout(120)?;
        } else if let Some(key) = op.strip_prefix("key:") {
            page.key_press(key)?;
            page.wait_for_timeout(120)?;
        }
    }
    page.wait_for_timeout(350)?;
    if !def.state_probe.is_empty() {
        let v = page.evaluate(&def.state_probe)?;
        // Go: s, _ := v.(string) — a non-string decodes as ""
        return Ok(v.as_str().unwrap_or("").to_string());
    }
    // presence probe: both sides remove the content (incl. portal wrapper)
    // on close — verified per-kernel
    let expr = format!(
        r#"!document.querySelector("[data-slot={}]") ? "closes" : "open""#,
        or_default(&def.content_slot, "dialog-content")
    );
    let v = page.evaluate(&expr)?;
    Ok(v.as_str().unwrap_or("").to_string())
}

// ---------- run both sides ---------------------------------------------------

fn c_mounted_bag_src(with_classes: bool) -> String {
    let wc = if with_classes { "true" } else { "false" };
    format!(
        r#"(() => {{
  const bag = []
  for (const el of document.body.querySelectorAll("*")) {{
    if (el.tagName === "SCRIPT" || el.tagName === "TEMPLATE") continue
    const cls = {wc} ? (el.getAttribute("class") || "").split(/\s+/).filter(Boolean).sort().join(" ") : ""
    bag.push(el.tagName + "|" + (el.getAttribute("data-slot") || "") + "|" + cls)
  }}
  bag.sort()
  return bag
}})()"#
    )
}

fn c_to_string_slice(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|e| e.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

/// cBagDiffBoth compares two multisets and returns the items only in each:
/// only_before is what's left of before once every match in after is removed,
/// only_after is what in after never matched anything in before.
fn c_bag_diff_both(before: &[String], after: &[String]) -> (Vec<String>, Vec<String>) {
    let mut remaining: Vec<String> = before.to_vec();
    let mut only_after: Vec<String> = Vec::new();
    for item in after {
        match remaining.iter().position(|r| r == item) {
            Some(idx) => {
                remaining.remove(idx);
            }
            None => only_after.push(item.clone()),
        }
    }
    (remaining, only_after)
}

fn c_bag_diff(before: &[String], after: &[String]) -> Vec<String> {
    c_bag_diff_both(before, after).1
}

fn c_mounted_diff(
    page: &BPage,
    with_classes: bool,
    driver_code: &str,
) -> Result<Vec<String>, String> {
    let before_v = page.evaluate(&c_mounted_bag_src(with_classes))?;
    let before = c_to_string_slice(&before_v);
    page.driver(driver_code)?;
    page.wait_for_timeout(400)?;
    let after_v = page.evaluate(&c_mounted_bag_src(with_classes))?;
    let after = c_to_string_slice(&after_v);
    Ok(c_bag_diff(&before, &after))
}

struct CRunResult {
    result: String,
    #[allow(dead_code)] // mirrored from the oracle-side record; the gate reads `result`
    has_result: bool,
    fact_raw: Value,
    mounted: Vec<String>,
    mounted_ok: bool,
}

fn bool_or_true(p: Option<bool>) -> bool {
    p.unwrap_or(true)
}

fn c_oracle_run(
    shell: &BrowserShell,
    def: &Cdef,
    out: &Path,
    step: &str,
) -> Result<CRunResult, String> {
    let page = shell.new_page(false)?;
    let r = c_oracle_run_page(&page, def, out, step);
    page.close();
    r
}

fn c_oracle_run_page(
    page: &BPage,
    def: &Cdef,
    out: &Path,
    step: &str,
) -> Result<CRunResult, String> {
    let url = format!("file://{}", abs_or_die(out, "oracle.html").to_string_lossy());
    page.goto_url(&url)?;
    page.wait_for_timeout(500)?;
    let mut mounted: Vec<String> = Vec::new();
    let mut mounted_ok = false;
    if !def.open.is_empty() && !c_closed_start(step) {
        if step.is_empty() {
            let m = c_mounted_diff(page, bool_or_true(def.mounted_classes), &def.open)?;
            mounted = m;
            mounted_ok = true;
        } else {
            page.driver(&def.open)?;
            page.wait_for_timeout(400)?;
        }
    }
    let mut result = String::new();
    let has_result = !step.is_empty();
    if has_result {
        result = c_step_it(page, def, step)?;
    }
    let fact_raw = page.evaluate_ordered(r#"window.__facts("oracle")"#)?;
    Ok(CRunResult {
        result,
        has_result,
        fact_raw,
        mounted,
        mounted_ok,
    })
}

fn c_shadless_run(
    shell: &BrowserShell,
    def: &Cdef,
    out: &Path,
    step: &str,
    recorder: &str,
) -> Result<(CRunResult, Vec<String>), String> {
    let page = shell.new_page_errors_only()?;
    let r = (|| -> Result<(CRunResult, Vec<String>), String> {
        let url = format!(
            "file://{}",
            abs_or_die(out, "shadless.html").to_string_lossy()
        );
        page.goto_url(&url)?;
        page.add_script_tag(recorder)?;
        page.wait_for_timeout(400)?;
        let mut mounted: Vec<String> = Vec::new();
        let mut mounted_ok = false;
        if !def.open_shadless.is_empty() && !c_closed_start(step) {
            if step.is_empty() {
                let m = c_mounted_diff(
                    &page,
                    bool_or_true(def.mounted_classes),
                    &def.open_shadless,
                )?;
                mounted = m;
                mounted_ok = true;
            } else {
                page.driver(&def.open_shadless)?; // def may use await
            }
        }
        page.wait_for_timeout(300)?;
        let mut result = String::new();
        let has_result = !step.is_empty();
        if has_result {
            result = c_step_it(&page, def, step)?;
        }
        let fact_raw = page.evaluate_ordered(r#"window.__facts("shadless")"#)?;
        let errs = page.events().unwrap_or_default();
        Ok((
            CRunResult {
                result,
                has_result,
                fact_raw,
                mounted,
                mounted_ok,
            },
            errs,
        ))
    })();
    page.close();
    r
}

// ---------- entry point ------------------------------------------------------

/// mutations.go truncate: plain byte cut, no suffix.
fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        return s.to_string();
    }
    let mut end = n;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

pub fn run_contract(name: &str) -> i32 {
    match run_contract_inner(name) {
        Ok(()) => 0,
        Err(ContractError::Fail) => 1, // FAIL banner already printed
        Err(ContractError::Msg(m)) => {
            eprintln!("contracts: {}", m);
            1
        }
    }
}

enum ContractError {
    Fail,
    Msg(String),
}

impl From<String> for ContractError {
    fn from(m: String) -> Self {
        ContractError::Msg(m)
    }
}

fn run_contract_inner(name: &str) -> Result<(), ContractError> {
    let root = std::env::current_dir().map_err(|e| ContractError::Msg(e.to_string()))?;
    let out = root.join(format!("tools/contracts/out/{}", name));

    let shell =
        BrowserShell::start().map_err(|e| ContractError::Msg(format!("{}", e)))?;

    let def = c_load_def(&shell, &root, name)?;
    let recorder = c_recorder_src(&def.slots);

    build_contract_oracle(
        &root,
        &EfDef {
            imports: def.imports.clone(),
            usage: def.usage.clone(),
            oracle_css: def.oracle_css.clone(),
            ..EfDef::default()
        },
        &out,
        &recorder,
    )
    .map_err(ContractError::Msg)?;

    let shadless_html = std::fs::read_to_string(&def.shadless_page).map_err(|e| {
        ContractError::Msg(format!("{}: {}", def.shadless_page, e))
    })?;
    let dir = abs_or_die(
        &root,
        &match Path::new(&def.shadless_page).parent() {
            // Go filepath.Dir("page.html") is "."; Path::parent gives ""
            Some(p) if !p.as_os_str().is_empty() => p.to_string_lossy().into_owned(),
            _ => ".".to_string(),
        },
    );
    std::fs::write(
        out.join("shadless.html"),
        c_rewrite_relative_paths(&shadless_html, &dir.to_string_lossy()),
    )
    .map_err(|e| ContractError::Msg(e.to_string()))?;

    shell.launch().map_err(ContractError::Msg)?;

    let mut oracle_open_raw = Value::Null;
    let mut shadless_open_raw = Value::Null;
    let mut oracle_mounted: Vec<String> = Vec::new();
    let mut shadless_mounted: Vec<String> = Vec::new();
    let mut oracle_mounted_ok = false;
    let mut shadless_mounted_ok = false;
    let mut oracle_s: HashMap<String, String> = HashMap::new();
    let mut shadless_s: HashMap<String, String> = HashMap::new();
    let mut scenario_ran: HashSet<String> = HashSet::new();
    let mut flaky: Vec<String> = Vec::new();

    let mut steps: Vec<String> = vec![String::new()];
    steps.extend(def.scenarios.iter().cloned());
    for step in &steps {
        let mut o = c_oracle_run(&shell, &def, &out, step).map_err(ContractError::Msg)?;
        let (mut c, errs) =
            c_shadless_run(&shell, &def, &out, step, &recorder).map_err(ContractError::Msg)?;
        // example-fixture pins itself to uncaught page errors; the contract
        // gate only printed them — and the runner hides a passing node's
        // output, so the one diagnostic proving the shipped page is broken
        // was invisible exactly when it mattered. A pageerror is a failure,
        // same contract as example-fixture.
        if !errs.is_empty() {
            let mut msg = format!(
                "shadless page threw {} uncaught error(s)",
                errs.len()
            );
            if let Some(first) = errs.first() {
                let line = first.lines().next().unwrap_or("");
                msg.push_str(": ");
                msg.push_str(line);
            }
            return Err(ContractError::Msg(msg));
        }
        if step.is_empty() {
            oracle_open_raw = o.fact_raw.clone();
            shadless_open_raw = c.fact_raw.clone();
            oracle_mounted = o.mounted.clone();
            oracle_mounted_ok = o.mounted_ok;
            shadless_mounted = c.mounted.clone();
            shadless_mounted_ok = c.mounted_ok;
            continue;
        }
        if o.result != c.result {
            // A real behavioral difference reproduces; a timing race in
            // either browser page does not. Re-run BOTH sides once from a
            // fresh page. If they now agree, record the agreed value and
            // say so.
            let mut o2 =
                c_oracle_run(&shell, &def, &out, step).map_err(ContractError::Msg)?;
            let (mut c2, errs2) = c_shadless_run(&shell, &def, &out, step, &recorder)
                .map_err(ContractError::Msg)?;
            for e in &errs2 {
                println!("  [shadless pageerror] {}", e);
            }
            if o2.result == c2.result {
                flaky.push(format!(
                    "{}: first run oracle={} shadless={}",
                    step, o.result, c.result
                ));
            }
            std::mem::swap(&mut o, &mut o2);
            std::mem::swap(&mut c, &mut c2);
        }
        oracle_s.insert(step.clone(), o.result.clone());
        shadless_s.insert(step.clone(), c.result.clone());
        scenario_ran.insert(step.clone());
    }
    if !flaky.is_empty() {
        println!(
            "contracts[{}]: {} scenario(s) agreed only on re-run (timing flake, not a diff)\n    {}",
            name,
            flaky.len(),
            flaky.join("\n    ")
        );
    }

    let oracle_open = c_build_fact(&oracle_open_raw, &def.slots).map_err(ContractError::Msg)?;
    let shadless_open =
        c_build_fact(&shadless_open_raw, &def.slots).map_err(ContractError::Msg)?;

    // ---------- diff ----------------------------------------------------------
    let mut pass = true;
    println!("contracts[{}]: open-state facts", name);
    for k in &def.slots {
        let a_el = oracle_open.slots.get(k).and_then(|o| o.as_ref());
        let b_el = shadless_open.slots.get(k).and_then(|o| o.as_ref());
        let keys = c_union_keys_ordered(a_el, b_el);
        let mut ign: HashSet<&str> = HashSet::new();
        ign.insert("tag");
        if let Some(xs) = def.ignore_attrs.get(k) {
            for x in xs {
                ign.insert(x.as_str());
            }
        }
        let mut diffs: Vec<String> = Vec::new();
        for kk in &keys {
            if kk.starts_with("data-radixuigo-") || kk.starts_with("data-radix-popper-") {
                continue; // kernel glue protocol internals (positioning feedback on the anchor)
            }
            if kk == "data-radix-menu-content" {
                continue; // radix internals marker
            }
            if ign.contains(kk.as_str()) {
                continue;
            }
            let av = c_lookup_attr(a_el, kk);
            let bv = c_lookup_attr(b_el, kk);
            let a_cmp = av.clone().unwrap_or_else(|| "<absent>".to_string());
            let b_cmp = bv.clone().unwrap_or_else(|| "<absent>".to_string());
            if a_cmp != b_cmp {
                diffs.push(format!(
                    "    {}: oracle={} shadless={}",
                    kk,
                    c_show_maybe(av.is_some(), av.as_deref().unwrap_or("")),
                    c_show_maybe(bv.is_some(), bv.as_deref().unwrap_or("")),
                ));
            }
        }
        let mut status = "match";
        if !diffs.is_empty() {
            status = "DIFF";
            pass = false;
        }
        println!("  {}: {}", k, status);
        for d in &diffs {
            println!("{}", d);
        }
    }
    let ae_show = |f: &CFact| -> String {
        match &f.active_element {
            None => "null".to_string(),
            Some(s) => json_string(s),
        }
    };
    let sl_show = |f: &CFact| -> String {
        let attr = match &f.scroll_lock_attr {
            None => "null".to_string(),
            Some(s) => json_string(s),
        };
        format!(
            "{{\"attr\":{},\"pointerEvents\":{}}}",
            attr,
            json_string(&f.scroll_lock_pe)
        )
    };
    for k in ["activeElement", "scrollLock"] {
        let (a_str, b_str) = if k == "activeElement" {
            (ae_show(&oracle_open), ae_show(&shadless_open))
        } else {
            (sl_show(&oracle_open), sl_show(&shadless_open))
        };
        if a_str == b_str {
            println!("  {}: match", k);
        } else {
            println!("  {}: DIFF oracle={} shadless={}", k, a_str, b_str);
            pass = false;
        }
    }

    println!("contracts[{}]: scenarios", name);
    for s in &def.scenarios {
        let (label, o_r, s_r): (String, String, String) = if !scenario_ran.contains(s) {
            // A genuine array hole (an accidental extra comma in a def's
            // scenarios list, e.g. toggle-group.mjs): JS's spread/for-of
            // visits it as literal `undefined` — falsy, so the label and
            // both values print JS's stringification of undefined rather
            // than being omitted.
            (
                "undefined".to_string(),
                "undefined".to_string(),
                "undefined".to_string(),
            )
        } else {
            (s.clone(), oracle_s[s].clone(), shadless_s[s].clone())
        };
        let same = o_r == s_r;
        let mut stat = "";
        if !same {
            stat = "DIFF";
            pass = false;
        }
        println!("  {}: oracle={} shadless={} {}", label, o_r, s_r, stat);
    }

    // mounted-diff structural check: the JS-created DOM must match too —
    // class drift inside portaled/mounted content had NO guard before
    if (oracle_mounted_ok || shadless_mounted_ok) && bool_or_true(def.mounted_check) {
        println!("contracts[{}]: mounted DOM", name);
        let (om, only_shadless) = c_bag_diff_both(&oracle_mounted, &shadless_mounted);
        if om.is_empty() && only_shadless.is_empty() {
            println!("  {} mounted elements match", oracle_mounted.len());
        } else {
            pass = false;
            for (i, x) in om.iter().enumerate() {
                if i >= 4 {
                    break;
                }
                println!("  only-oracle:   {}", truncate(x, 160));
            }
            for (i, x) in only_shadless.iter().enumerate() {
                if i >= 4 {
                    break;
                }
                println!("  only-shadless: {}", truncate(x, 160));
            }
        }
    }

    let oracle_open_json =
        c_raw_to_jsonable(&oracle_open_raw).map_err(ContractError::Msg)?;
    let shadless_open_json =
        c_raw_to_jsonable(&shadless_open_raw).map_err(ContractError::Msg)?;
    let mut oracle_obj = JsonObj::new();
    let mut shadless_obj = JsonObj::new();
    for s in &def.scenarios {
        if !scenario_ran.contains(s) {
            // array hole — JS's oracleS/shadlessS never gained this key
            continue;
        }
        oracle_obj = oracle_obj.add(s, Json::Str(oracle_s[s].clone()));
        shadless_obj = shadless_obj.add(s, Json::Str(shadless_s[s].clone()));
    }
    let result = JsonObj::new()
        .add("oracleOpen", oracle_open_json)
        .add("shadlessOpen", shadless_open_json)
        .add("oracle", Json::from_obj(oracle_obj))
        .add("shadless", Json::from_obj(shadless_obj))
        .add("pass", Json::Bool(pass));
    std::fs::write(
        out.join("result.json"),
        crate::jsonorder::marshal_js(&Json::from_obj(result)),
    )
    .map_err(|e| ContractError::Msg(e.to_string()))?;

    shell.close();
    if pass {
        println!("\nPASS  contracts {}", name);
        return Ok(());
    }
    println!("\nFAIL  contracts {}", name);
    Err(ContractError::Fail)
}

/// runContractsAll: no arg — run every component def in a child process each;
/// exit 1 if any FAILs.
pub fn run_contracts_all() -> i32 {
    let ents = match std::fs::read_dir("tools/contracts/components") {
        Ok(e) => e,
        Err(e) => {
            eprintln!("contracts: {}", e);
            return 1;
        }
    };
    let mut names: Vec<String> = Vec::new();
    for e in ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if let Some(base) = n.strip_suffix(".mjs") {
            names.push(base.to_string());
        }
    }
    names.sort();

    let self_exe = std::env::current_exe().unwrap_or_else(|_| "pipeline".into());
    let mut failed = 0;
    for c in &names {
        println!("\n=== {} ===", c);
        let status = std::process::Command::new(&self_exe)
            .arg("contract")
            .arg(c)
            .status();
        match status {
            Ok(st) if st.success() => {}
            _ => failed += 1,
        }
    }
    if failed > 0 {
        println!(
            "\nFAIL  contracts full-run ({}/{} failed)",
            failed,
            names.len()
        );
        return 1;
    }
    println!("\nPASS  contracts full-run ({} components)", names.len());
    0
}
