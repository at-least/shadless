//! Port of pipeline/overlay.go — every manual intervention on top of the
//! mechanical conversion, audited against the upstream it was written for.
//!
//! Three kinds of unit:
//!
//!   rule      a table entry in the pipeline (DEFAULT_CONTENT,
//!             TEXT_ADJUSTMENTS, DEAD_UTILITIES, SKIN_ALLOWLIST, KNOWN_ICONS,
//!             tier sets, the Persian dictionary, contract ignoreAttrs).
//!             Precondition is a structural predicate on the IR / upstream
//!             tree.
//!   authored  a whole hand-written file (kernel behavior files, the runtime,
//!             hand-authored demos). Anchored to the sha256 of the upstream
//!             inputs it was written against, recorded in overlays/manifest
//!             .json. Input changed ⇒ stale.
//!   source    a git patch on the upstream tree itself, under
//!             overlays/upstream/*.patch. Conflict ⇒ conflict bucket.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

use super::docs_transforms::{fence_shadow, text_adjustments};
use crate::gates::ledger::js_set_literal;

const OV_UP: &str = ".upstream/shadcn-ui";
const OV_REG: &str = "apps/v4/registry/bases/radix/ui";
const OV_EXAMPLES: &str = "apps/v4/examples/radix";
const OV_EXAMPLES_AR: &str = "apps/v4/examples/aria";
const OV_MDX: &str = "apps/v4/content/docs/components/radix";
const OV_MANIFEST: &str = "overlays/manifest.json";
const OV_PATCHES: &str = "overlays/upstream";
const OV_TASKS_DIR: &str = "build/gates/tasks";

#[derive(Clone)]
struct OvUnit {
    id: String,
    kind: String,
    home: String,
    file: String,
    inputs: Vec<String>,
    extra: Vec<String>,
    requires: Option<std::sync::Arc<dyn Fn(&Path) -> String + Send + Sync>>,
    dissolved: Option<std::sync::Arc<dyn Fn(&Path) -> String + Send + Sync>>,
    reason: String,
    hash: String,
    recorded: Option<OvUnitRec>,
    bucket: String,
}

#[derive(Deserialize, Default)]
struct OvPinFile {
    #[serde(rename = "shadcn_ui", default)]
    shadcn_ui: OvPinShadcnUi,
    #[serde(default)]
    kernel: OvPinKernel,
}

#[derive(Deserialize, Default)]
struct OvPinShadcnUi {
    #[serde(default)]
    tag: String,
    #[serde(default)]
    commit: String,
}

#[derive(Deserialize, Default)]
struct OvPinKernel {
    #[serde(default)]
    sha256: String,
}

#[derive(Deserialize, Default)]
struct OvIr {
    #[serde(default)]
    components: Vec<OvIrComponent>,
}

#[derive(Deserialize, Default)]
struct OvIrComponent {
    #[serde(default)]
    export: bool,
    #[serde(rename = "fn", default)]
    fn_: String,
    #[serde(default)]
    elements: Vec<OvIrElement>,
}

#[derive(Deserialize, Default)]
struct OvIrElement {
    // Go json.Unmarshal tolerates `"slot": null` (field keeps its default);
    // serde does not — Option<String> with None read as "".
    #[serde(default)]
    slot: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
struct OvUnitRec {
    #[serde(default)]
    file: String,
    #[serde(default)]
    inputs: Vec<String>,
    #[serde(default)]
    extra: Vec<String>,
    #[serde(default)]
    hash: String,
    #[serde(default)]
    pin: String,
}

#[derive(Deserialize, Default)]
struct OvManifestFile {
    #[serde(default)]
    pin: String,
    #[serde(default)]
    commit: String,
    #[serde(default)]
    units: HashMap<String, OvUnitRec>,
}

fn ov_up(rel: &str) -> String {
    format!("{}/{}", OV_UP, rel)
}

fn ov_up_exists(root: &Path, rel: &str) -> bool {
    root.join(ov_up(rel)).exists()
}

fn ov_up_read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(ov_up(rel))).unwrap_or_default()
}

fn ov_ir(root: &Path, name: &str) -> Option<OvIr> {
    let b = std::fs::read_to_string(root.join("generated/ir").join(format!("{}.json", name)))
        .ok()?;
    serde_json::from_str(&b).ok()
}

fn ov_registry_names(root: &Path) -> Vec<String> {
    let Ok(ents) = std::fs::read_dir(root.join(ov_up(OV_REG))) else {
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for e in ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if let Some(stem) = n.strip_suffix(".tsx") {
            out.push(stem.to_string());
        }
    }
    out.sort();
    out
}

/// ovTierSets — the converter's tables (the tier lists keep their literal
/// ORDER: runtime:core's hash depends on it).
fn ov_load_tier_sets() -> (Vec<(String, Vec<String>)>, Vec<String>) {
    let mut sets: Vec<(String, Vec<String>)> = Vec::new();
    for (tier, names) in crate::convert::cv_tier_sets() {
        // the audit's unit ids say "trivial" where tierOf returns "trivial-js"
        let tier = if *tier == "trivial-js" { "trivial" } else { *tier };
        sets.push((tier.to_string(), names.iter().map(|s| s.to_string()).collect()));
    }
    let icons: Vec<String> = crate::convert::cv_known_icons()
        .iter()
        .map(|s| s.to_string())
        .collect();
    (sets, icons)
}

fn ov_skin_allowlist(root: &Path) -> Result<Vec<String>, String> {
    let src = std::fs::read_to_string(root.join("src/emitter/skin.mjs"))
        .map_err(|e| e.to_string())?;
    js_set_literal(&src, "SKIN_ALLOWLIST")
}

// ---- rule units ----

fn ov_rule_units(
    root: &Path,
    _pin: &OvPinFile,
    tier_sets: &[(String, Vec<String>)],
    icons: &[String],
) -> Result<Vec<OvUnit>, String> {
    let mut units: Vec<OvUnit> = Vec::new();

    // DEFAULT_CONTENT — (component, fn) keyed example content for static
    // pages; the live table is the Go one (the emit node runs Go)
    let mut comps: Vec<&str> = crate::emit::default_content::default_content()
        .keys()
        .copied()
        .collect();
    comps.sort();
    for comp in comps {
        let mut fns: Vec<&str> = crate::emit::default_content::default_content()[comp]
            .keys()
            .copied()
            .collect();
        fns.sort();
        for fn_ in fns {
            let comp = comp.to_string();
            let fn_ = fn_.to_string();
            units.push(OvUnit {
                id: format!("default-content:{}.{}", comp, fn_),
                kind: "rule".to_string(),
                home: "pipeline/default_content.go DEFAULT_CONTENT".to_string(),
                file: String::new(),
                inputs: Vec::new(),
                extra: Vec::new(),
                requires: Some(std::sync::Arc::new(move |root: &Path| -> String {
                    let Some(i) = ov_ir(root, &comp) else {
                        return format!("component {} has no IR (gone upstream?)", comp);
                    };
                    for c in &i.components {
                        if c.export && c.fn_ == fn_ {
                            return String::new();
                        }
                    }
                    format!("{} no longer exports {}", comp, fn_)
                })),
                dissolved: Some(std::sync::Arc::new(|_| String::new())),
                reason: String::new(),
                hash: String::new(),
                recorded: None,
                bucket: String::new(),
            });
        }
    }

    // TEXT_ADJUSTMENTS — prose rewrites anchored to upstream mdx sentences
    for adj in text_adjustments() {
        for (i, op) in adj.ops.iter().enumerate() {
            let files: Vec<String> = adj.files.iter().map(|s| s.to_string()).collect();
            let find = op.find.to_string();
            units.push(OvUnit {
                id: format!("text-adjustment:{}#{}", adj.id, i),
                kind: "rule".to_string(),
                home: "pipeline/docs_transforms.go TEXT_ADJUSTMENTS".to_string(),
                file: String::new(),
                inputs: Vec::new(),
                extra: Vec::new(),
                requires: Some(std::sync::Arc::new(move |root: &Path| -> String {
                    for f in &files {
                        if !ov_up_exists(root, &format!("{}/{}", OV_MDX, f)) {
                            return format!("{} missing upstream", f);
                        }
                        if !fence_shadow(&ov_up_read(root, &format!("{}/{}", OV_MDX, f)))
                            .contains(&find)
                        {
                            let mut find = find.clone();
                            if find.len() > 50 {
                                find.truncate(50);
                            }
                            return format!("find string no longer in {}: {:?}", f, format!("{}…", find));
                        }
                    }
                    String::new()
                })),
                dissolved: Some(std::sync::Arc::new(|_| String::new())),
                reason: String::new(),
                hash: String::new(),
                recorded: None,
                bucket: String::new(),
            });
        }
    }

    // DEAD_UTILITIES — classes the registry uses but no stylesheet defines
    fn registry_text(root: &Path) -> String {
        let mut parts: Vec<String> = Vec::new();
        for n in ov_registry_names(root) {
            parts.push(ov_up_read(root, &format!("{}/{}.tsx", OV_REG, n)));
        }
        parts.join("\n")
    }
    // the converter also runs over the examples (oracle bundles)
    fn source_text(root: &Path) -> String {
        let mut parts: Vec<String> = vec![registry_text(root)];
        for d in [OV_EXAMPLES, OV_EXAMPLES_AR] {
            let Ok(ents) = std::fs::read_dir(root.join(ov_up(d))) else {
                continue;
            };
            for e in ents.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if n.ends_with(".tsx") {
                    parts.push(ov_up_read(root, &format!("{}/{}", d, n)));
                }
            }
        }
        parts.join("\n")
    }
    // the ONE skin this build ingests — other skins defining a class is
    // irrelevant
    fn upstream_styles(root: &Path) -> String {
        ov_up_read(root, "apps/v4/registry/styles/style-nova.css")
    }

    let mut dead_keys: Vec<String> = vec!["origin-top-center".to_string()];
    dead_keys.sort();
    for tok in dead_keys {
        let tok = tok.clone();
        let tok_req = tok.clone();
        let tok_dis = tok.clone();
        units.push(OvUnit {
            id: format!("dead-utility:{}", tok),
            kind: "rule".to_string(),
            home: "pipeline/emitter_css.go DEAD_UTILITIES".to_string(),
            file: String::new(),
            inputs: Vec::new(),
            extra: Vec::new(),
            requires: Some(std::sync::Arc::new(move |root: &Path| -> String {
                if registry_text(root).contains(&tok_req) {
                    return String::new();
                }
                format!("{} no longer referenced by the registry", tok_req)
            })),
            dissolved: Some(std::sync::Arc::new(move |root: &Path| -> String {
                let re = Regex::new(&format!(
                    r"@utility[ \t]+{}\b|\.{}[ \t]*[{{,]",
                    regex::escape(&tok_dis),
                    regex::escape(&tok_dis)
                ))
                .unwrap();
                if re.is_match(&upstream_styles(root)) {
                    return format!("upstream now defines {} — it is not dead any more", tok_dis);
                }
                String::new()
            })),
            reason: String::new(),
            hash: String::new(),
            recorded: None,
            bucket: String::new(),
        });
    }

    let allow = ov_skin_allowlist(root)?;
    for tok in allow {
        let tok = tok.clone();
        let tok_req = tok.clone();
        let tok_dis = tok.clone();
        units.push(OvUnit {
            id: format!("skin-allowlist:{}", tok),
            kind: "rule".to_string(),
            home: "src/emitter/skin.mjs SKIN_ALLOWLIST".to_string(),
            file: String::new(),
            inputs: Vec::new(),
            extra: Vec::new(),
            requires: Some(std::sync::Arc::new(move |root: &Path| -> String {
                if registry_text(root).contains(&tok_req) || upstream_styles(root).contains(&tok_req) {
                    return String::new();
                }
                format!("{} appears nowhere upstream", tok_req)
            })),
            dissolved: Some(std::sync::Arc::new(move |root: &Path| -> String {
                let re = Regex::new(&format!(
                    r"@utility[ \t]+{}\b|\.{}[ \t]*\{{",
                    regex::escape(&tok_dis),
                    regex::escape(&tok_dis)
                ))
                .unwrap();
                if !re.is_match(&upstream_styles(root)) {
                    return String::new();
                }
                crate::emit::load_skin();
                if crate::emit::skin_data().map.contains_key(&tok_dis) {
                    return String::new();
                }
                format!(
                    "upstream now defines {} in a form the SKIN_MAP parser can't capture — emitter_css.go's auto-emit won't cover it",
                    tok_dis
                )
            })),
            reason: String::new(),
            hash: String::new(),
            recorded: None,
            bucket: String::new(),
        });
    }

    // KNOWN_ICONS — icon component names the converter treats as <svg>
    for icon in icons {
        let icon = icon.clone();
        units.push(OvUnit {
            id: format!("known-icon:{}", icon),
            kind: "rule".to_string(),
            home: "pipeline/convert.go KNOWN_ICONS".to_string(),
            file: String::new(),
            inputs: Vec::new(),
            extra: Vec::new(),
            requires: Some(std::sync::Arc::new(|_| String::new())),
            dissolved: Some(std::sync::Arc::new(move |root: &Path| -> String {
                let re = Regex::new(&format!(r"\b{}\b", regex::escape(&icon))).unwrap();
                if re.is_match(&source_text(root)) {
                    return String::new();
                }
                format!("{} is used by no registry or example file", icon)
            })),
            reason: String::new(),
            hash: String::new(),
            recorded: None,
            bucket: String::new(),
        });
    }

    // tier classification — every registry component must be classified,
    // and every classified name must exist.
    let mut classified: HashMap<String, bool> = HashMap::new();
    for (tier, names) in tier_sets {
        for name in names {
            classified.insert(name.clone(), true);
            let name = name.clone();
            let tier = tier.clone();
            units.push(OvUnit {
                id: format!("tier:{}:{}", tier, name),
                kind: "rule".to_string(),
                home: "pipeline/convert.go tier sets".to_string(),
                file: String::new(),
                inputs: Vec::new(),
                extra: Vec::new(),
                requires: Some(std::sync::Arc::new(move |root: &Path| -> String {
                    if ov_up_exists(root, &format!("{}/{}.tsx", OV_REG, name)) {
                        return String::new();
                    }
                    format!("{}.tsx is gone from the registry", name)
                })),
                dissolved: Some(std::sync::Arc::new(|_| String::new())),
                reason: String::new(),
                hash: String::new(),
                recorded: None,
                bucket: String::new(),
            });
        }
    }
    units.push(OvUnit {
        id: "tier:coverage".to_string(),
        kind: "rule".to_string(),
        home: "pipeline/convert.go tier sets".to_string(),
        file: String::new(),
        inputs: Vec::new(),
        extra: Vec::new(),
        requires: Some(std::sync::Arc::new(move |root: &Path| -> String {
            // static tier is the default (no set); the converter decides
            // static vs needs-runtime from the source. What must not
            // happen: a component that imports a radix primitive and sits
            // in no set.
            let inert: [&str; 4] = ["Slot", "Direction", "VisuallyHidden", "Primitive"];
            let re = Regex::new(
                r#"import\s*\{([^}]*)\}\s*from\s*"(?:radix-ui|@radix-ui/[0-9A-Za-z_-]+)""#,
            )
            .unwrap();
            let name_re = Regex::new(r"[ \t]+as[ \t]+").unwrap();
            let behavior_import = |src: &str| -> bool {
                for m in re.captures_iter(src) {
                    for x in m[1].split(',') {
                        let name = name_re.split(x.trim()).next().unwrap_or("").trim();
                        if name.is_empty() || inert.contains(&name) {
                            continue;
                        }
                        return true;
                    }
                }
                false
            };
            let mut unclassified: Vec<String> = Vec::new();
            for n in ov_registry_names(root) {
                if !classified.contains_key(&n)
                    && behavior_import(&ov_up_read(root, &format!("{}/{}.tsx", OV_REG, n)))
                {
                    unclassified.push(n);
                }
            }
            if !unclassified.is_empty() {
                return format!(
                    "radix-backed components with no tier: {}",
                    unclassified.join(", ")
                );
            }
            String::new()
        })),
        dissolved: Some(std::sync::Arc::new(|_| String::new())),
        reason: String::new(),
        hash: String::new(),
        recorded: None,
        bucket: String::new(),
    });

    // Persian dictionary — keys must exist in upstream's Arabic dictionary
    {
        let keys: Vec<String> = crate::emit::build_rtl::persian().into_keys().collect();
        units.push(OvUnit {
            id: "rtl:persian-dictionary".to_string(),
            kind: "rule".to_string(),
            home: "engine persian dictionary".to_string(),
            file: String::new(),
            inputs: Vec::new(),
            extra: Vec::new(),
            requires: Some(std::sync::Arc::new(move |root: &Path| -> String {
                // the vendored dictionary, not examples/aria
                #[derive(Deserialize)]
                struct Dict {
                    #[serde(default)]
                    ar: DictLang,
                    #[serde(default)]
                    fa: Option<DictLang>,
                }
                #[derive(Deserialize, Default)]
                struct DictLang {
                    #[serde(default)]
                    values: HashMap<String, String>,
                }
                let Ok(b) = std::fs::read_to_string(root.join("src/registry/rtl-translations.json")) else {
                    return "alert-rtl gone from src/registry/rtl-translations.json".to_string();
                };
                let Ok(dict) = serde_json::from_str::<HashMap<String, Dict>>(&b) else {
                    return "alert-rtl gone from src/registry/rtl-translations.json".to_string();
                };
                let Some(alert) = dict.get("alert-rtl") else {
                    return "alert-rtl has no Arabic dictionary".to_string();
                };
                if alert.ar.values.is_empty() {
                    return "alert-rtl has no Arabic dictionary".to_string();
                }
                let mut missing: Vec<String> = Vec::new();
                for k in &keys {
                    if !alert.ar.values.contains_key(k) {
                        missing.push(k.clone());
                    }
                }
                if !missing.is_empty() {
                    return format!(
                        "Persian keys with no Arabic counterpart upstream: {}",
                        missing.join(", ")
                    );
                }
                String::new()
            })),
            dissolved: Some(std::sync::Arc::new(move |root: &Path| -> String {
                #[derive(Deserialize)]
                struct Dict {
                    #[serde(default)]
                    fa: Option<serde_json::Value>,
                }
                let Ok(b) = std::fs::read_to_string(root.join("src/registry/rtl-translations.json")) else {
                    return String::new();
                };
                let Ok(dict) = serde_json::from_str::<HashMap<String, Dict>>(&b) else {
                    return String::new();
                };
                if let Some(alert) = dict.get("alert-rtl") {
                    if alert.fa.is_some() {
                        return "upstream now ships a Persian dictionary — use it instead".to_string();
                    }
                }
                String::new()
            })),
            reason: String::new(),
            hash: String::new(),
            recorded: None,
            bucket: String::new(),
        });
    }
    Ok(units)
}

// ovIgnoreAttrUnits — contract ignoreAttrs: the slot they exempt must
// still exist. The defs load through the browser shell (node only, no
// chromium — loadContractDef never launches).
fn ov_ignore_attr_units(
    root: &Path,
    shell: &crate::oracle::browser_shell::BrowserShell,
) -> Result<Vec<OvUnit>, String> {
    let ents = std::fs::read_dir(root.join("tools/contracts/components"))
        .map_err(|e| e.to_string())?;
    let mut files: Vec<String> = Vec::new();
    for e in ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if n.ends_with(".mjs") {
            files.push(n);
        }
    }
    files.sort();
    let mut units: Vec<OvUnit> = Vec::new();
    for f in files {
        let name = f.trim_end_matches(".mjs").to_string();
        let base = name.trim_end_matches("-multiple").to_string();
        let res = shell.call(&serde_json::json!({
            "op": "loadContractDef",
            "file": format!(
                "file://{}",
                root.join("tools/contracts/components").join(&f).to_string_lossy()
            ),
        }))?;
        #[derive(Deserialize)]
        struct Def {
            #[serde(rename = "ignoreAttrs", default)]
            ignore_attrs: HashMap<String, serde_json::Value>,
        }
        let Ok(def) = serde_json::from_value::<Def>(
            res.get("def").cloned().unwrap_or(serde_json::Value::Null),
        ) else {
            continue;
        };
        let mut slots: Vec<String> = def.ignore_attrs.keys().cloned().collect();
        slots.sort();
        for slot in slots {
            let slot = slot.clone();
            let base = base.clone();
            units.push(OvUnit {
                id: format!("ignore-attrs:{}:{}", name, slot),
                kind: "rule".to_string(),
                home: format!("tools/contracts/components/{}", f),
                file: String::new(),
                inputs: Vec::new(),
                extra: Vec::new(),
                requires: Some(std::sync::Arc::new(move |root: &Path| -> String {
                    let Some(i) = ov_ir(root, &base) else {
                        return format!("{} has no IR", base);
                    };
                    let mut slots_set: HashMap<String, bool> = HashMap::new();
                    for c in &i.components {
                        for e in &c.elements {
                            if let Some(slot) = &e.slot {
                                if !slot.is_empty() {
                                    slots_set.insert(slot.clone(), true);
                                }
                            }
                        }
                    }
                    if slots_set.contains_key(&slot) {
                        return String::new();
                    }
                    format!("slot {} no longer emitted by {}", slot, base)
                })),
                dissolved: Some(std::sync::Arc::new(|_| String::new())),
                reason: String::new(),
                hash: String::new(),
                recorded: None,
                bucket: String::new(),
            });
        }
    }
    Ok(units)
}

// ---- authored units ----

fn ov_authored_units(root: &Path, pin: &OvPinFile, trivial: &[String]) -> Vec<OvUnit> {
    let mut units: Vec<OvUnit> = Vec::new();
    let reg_file = |name: &str| -> String {
        format!("{}/{}.tsx", OV_REG, name.trim_end_matches("-multiple"))
    };

    // src/kernel/*.html are GENERATED by pipeline example-fixture
    // --contracts from the contract defs' React usage trees (the defs are
    // the authored units). Per-component behavior files: written against
    // the component's tsx (and the vendored kernel, whose sha is part of
    // the anchor).
    let mut behavior_files: Vec<String> = Vec::new();
    if let Ok(ents) = std::fs::read_dir(root.join("src/runtime/components")) {
        for e in ents.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.ends_with(".js") {
                behavior_files.push(n);
            }
        }
    }
    behavior_files.sort();
    for f in behavior_files {
        let name = f.trim_end_matches(".js").to_string();
        units.push(OvUnit {
            id: format!("behavior:{}", name),
            kind: "authored".to_string(),
            home: String::new(),
            file: format!("src/runtime/components/{}", f),
            inputs: vec![reg_file(&name)],
            extra: vec![format!("kernel:{}", pin.kernel.sha256)],
            requires: None,
            dissolved: None,
            reason: String::new(),
            hash: String::new(),
            recorded: None,
            bucket: String::new(),
        });
    }
    // the trivial-tier runtime is written against every trivial component
    let mut core_inputs: Vec<String> = Vec::new();
    for t in trivial {
        core_inputs.push(reg_file(t));
    }
    units.push(OvUnit {
        id: "runtime:core".to_string(),
        kind: "authored".to_string(),
        home: String::new(),
        file: "src/runtime/core.js".to_string(),
        inputs: core_inputs,
        extra: Vec::new(),
        requires: None,
        dissolved: None,
        reason: String::new(),
        hash: String::new(),
        recorded: None,
        bucket: String::new(),
    });
    // hand-authored demos: docs/demos pages the oracle does not own
    {
        let mut owned: HashMap<String, bool> = HashMap::new();
        let mut fixture_owned: HashMap<String, bool> = HashMap::new();
        if let Ok(b) = std::fs::read_to_string(root.join("docs/example-oracle.json")) {
            #[derive(Deserialize)]
            struct T {
                #[serde(default)]
                name: String,
            }
            if let Ok(arr) = serde_json::from_str::<Vec<T>>(&b) {
                for t in arr {
                    owned.insert(t.name, true);
                }
            }
        }
        if let Ok(b) = std::fs::read_to_string(root.join("docs/example-fixture-targets.json")) {
            #[derive(Deserialize)]
            struct T {
                #[serde(default)]
                name: String,
            }
            if let Ok(arr) = serde_json::from_str::<Vec<T>>(&b) {
                for t in arr {
                    fixture_owned.insert(t.name, true);
                }
            }
        }
        let re_rtl = Regex::new(r"-rtl-(he|en|fa)\.html$").unwrap();
        let mut demos: Vec<String> = Vec::new();
        if let Ok(dents) = std::fs::read_dir(root.join("docs/demos")) {
            for e in dents.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if n.ends_with(".html") && !re_rtl.is_match(&n) {
                    demos.push(n);
                }
            }
        }
        demos.sort();
        for f in demos {
            let name = f.trim_end_matches(".html").to_string();
            if owned.contains_key(&name) || fixture_owned.contains_key(&name) {
                continue; // the oracle / example-fixture owns these
            }
            let ex = format!("{}/{}.tsx", OV_EXAMPLES, name);
            let mut u = OvUnit {
                id: format!("demo:{}", name),
                kind: "authored".to_string(),
                home: String::new(),
                file: format!("docs/demos/{}", f),
                inputs: Vec::new(),
                extra: Vec::new(),
                requires: None,
                dissolved: None,
                reason: String::new(),
                hash: String::new(),
                recorded: None,
                bucket: String::new(),
            };
            if ov_up_exists(root, &ex) {
                u.inputs = vec![ex];
            } else {
                u.extra = vec!["no-upstream-input".to_string()];
            }
            units.push(u);
        }
    }
    units
}

fn ov_hash_unit(root: &Path, u: &OvUnit) -> String {
    use sha2::{Digest, Sha256};
    let mut parts: Vec<String> = Vec::new();
    for p in &u.inputs {
        let b = std::fs::read(root.join(ov_up(p))).unwrap_or_else(|_| b"<missing>".to_vec());
        parts.push(base64_encode(&b));
    }
    for e in &u.extra {
        parts.push(base64_encode(e.as_bytes()));
    }
    let mut hasher = Sha256::new();
    hasher.update(parts.join("|").as_bytes());
    hex::encode(hasher.finalize())
}

fn base64_encode(b: &[u8]) -> String {
    const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in b.chunks(3) {
        let n = chunk.len();
        let b0 = chunk[0] as u32;
        let b1 = if n > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if n > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHA[(triple >> 18) as usize & 63] as char);
        out.push(ALPHA[(triple >> 12) as usize & 63] as char);
        out.push(if n > 1 {
            ALPHA[(triple >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if n > 2 {
            ALPHA[triple as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

// ---- source (patch) units ----

fn ov_source_units(root: &Path) -> Vec<OvUnit> {
    if !root.join(OV_PATCHES).exists() {
        return Vec::new();
    }
    let Ok(ents) = std::fs::read_dir(root.join(OV_PATCHES)) else {
        return Vec::new();
    };
    let mut files: Vec<String> = Vec::new();
    for e in ents.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if n.ends_with(".patch") {
            files.push(n);
        }
    }
    files.sort();
    files
        .into_iter()
        .map(|f| OvUnit {
            id: format!("source:{}", f),
            kind: "source".to_string(),
            home: String::new(),
            file: format!("{}/{}", OV_PATCHES, f),
            inputs: Vec::new(),
            extra: Vec::new(),
            requires: None,
            dissolved: None,
            reason: String::new(),
            hash: String::new(),
            recorded: None,
            bucket: String::new(),
        })
        .collect()
}

/// A patch series is "applied" when the upstream working tree carries
/// exactly it: every patch reverse-applies cleanly. It "conflicts" when it
/// neither applies forward (tree clean, patch broken) nor reverse-applies.
fn ov_source_state(root: &Path, u: &OvUnit) -> (&'static str, &'static str) {
    let check = |args: &[&str]| -> bool {
        std::process::Command::new("git")
            .args(["-C", OV_UP, "apply", "--check"])
            .args(args)
            .arg(root.join(&u.file))
            .current_dir(root)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    };
    if check(&["-R"]) {
        return ("applied", "");
    }
    if check(&[]) {
        return (
            "not-applied",
            "applies cleanly but is not applied — run pipeline upstream --apply-patches",
        );
    }
    (
        "conflict",
        "neither applied nor applicable to the pinned upstream — rebase the patch",
    )
}

// ---- audit / record / tasks ----

struct OvBuckets {
    applied: Vec<OvUnit>,
    dissolved: Vec<OvUnit>,
    orphaned: Vec<OvUnit>,
    stale: Vec<OvUnit>,
    conflict: Vec<OvUnit>,
    unrecorded: Vec<OvUnit>,
}

fn ov_audit(
    root: &Path,
    shell: &crate::oracle::browser_shell::BrowserShell,
    pin: &OvPinFile,
    strict: bool,
) -> OvBuckets {
    let (tier_sets, icons) = match ov_load_tier_sets() {
        (s, i) => (s, i),
    };
    let trivial: Vec<String> = tier_sets
        .iter()
        .filter(|(t, _)| t == "trivial")
        .flat_map(|(_, names)| names.clone())
        .collect();

    let mut buckets = OvBuckets {
        applied: Vec::new(),
        dissolved: Vec::new(),
        orphaned: Vec::new(),
        stale: Vec::new(),
        conflict: Vec::new(),
        unrecorded: Vec::new(),
    };
    let mut rules = match ov_rule_units(root, pin, &tier_sets, &icons) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("overlay: {}", e);
            std::process::exit(1);
        }
    };
    let ignore_units = match ov_ignore_attr_units(root, shell) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("overlay: {}", e);
            std::process::exit(1);
        }
    };
    rules.extend(ignore_units);
    rules.sort_by(|a, b| a.id.cmp(&b.id));
    for mut u in rules {
        if let Some(requires) = &u.requires {
            let r = requires(root);
            if !r.is_empty() {
                u.reason = r;
                buckets.orphaned.push(u);
                continue;
            }
        }
        if let Some(dissolved) = &u.dissolved {
            let d = dissolved(root);
            if !d.is_empty() {
                u.reason = d;
                buckets.dissolved.push(u);
                continue;
            }
        }
        buckets.applied.push(u);
    }

    let mut manifest = OvManifestFile::default();
    if let Ok(b) = std::fs::read_to_string(root.join(OV_MANIFEST)) {
        let _ = serde_json::from_str::<OvManifestFile>(&b).map(|m| manifest = m);
    }
    for mut u in ov_authored_units(root, pin, &trivial) {
        let h = ov_hash_unit(root, &u);
        u.hash = h.clone();
        match manifest.units.get(&u.id) {
            None => buckets.unrecorded.push(u),
            Some(rec) => {
                u.recorded = Some(rec.clone());
                if rec.hash != h {
                    let mut in_ = u.inputs.join(", ");
                    if in_.is_empty() {
                        in_ = u.extra.join(", ");
                    }
                    u.reason = format!("upstream inputs changed since {}: {}", rec.pin, in_);
                    buckets.stale.push(u);
                } else {
                    buckets.applied.push(u);
                }
            }
        }
    }
    for mut u in ov_source_units(root) {
        let (state, reason) = ov_source_state(root, &u);
        if state == "applied" {
            buckets.applied.push(u);
        } else {
            u.reason = reason.to_string();
            buckets.conflict.push(u);
        }
    }
    // recorded units whose file is gone
    let mut seen: HashMap<String, bool> = HashMap::new();
    for u in buckets.applied.iter().chain(buckets.stale.iter()) {
        seen.insert(u.id.clone(), true);
    }
    for id in manifest.units.keys() {
        if !seen.contains_key(id) {
            buckets.orphaned.push(OvUnit {
                id: id.clone(),
                kind: "authored".to_string(),
                home: String::new(),
                file: String::new(),
                inputs: Vec::new(),
                extra: Vec::new(),
                requires: None,
                dissolved: None,
                reason: "recorded in the manifest but the unit no longer exists — delete the entry (--record)".to_string(),
                hash: String::new(),
                recorded: None,
                bucket: String::new(),
            });
        }
    }

    let counts = format!(
        "{} applied, {} dissolved, {} orphaned, {} stale, {} conflict, {} unrecorded",
        buckets.applied.len(),
        buckets.dissolved.len(),
        buckets.orphaned.len(),
        buckets.stale.len(),
        buckets.conflict.len(),
        buckets.unrecorded.len()
    );
    // dissolved is a failure bucket, not a courtesy notice.
    let mut bad: Vec<OvUnit> = Vec::new();
    for (name, units) in [
        ("dissolved", &buckets.dissolved),
        ("orphaned", &buckets.orphaned),
        ("stale", &buckets.stale),
        ("conflict", &buckets.conflict),
        ("unrecorded", &buckets.unrecorded),
    ] {
        for u in units {
            let mut u = u.clone();
            u.bucket = name.to_string();
            bad.push(u);
        }
    }
    if !bad.is_empty() {
        let mut lines: Vec<String> = Vec::new();
        for u in &bad {
            let mut reason = u.reason.clone();
            if reason.is_empty() {
                reason = "(new authored unit — record it: ./build/pipeline overlay --record)".to_string();
            }
            let mut l = format!("{:<10} {}\n             {}", u.bucket, u.id, reason);
            if !u.file.is_empty() {
                l += &format!("\n             file: {}", u.file);
            }
            lines.push(l);
        }
        let label = if strict { "FAIL " } else { "REPORT" };
        eprintln!(
            "{} overlay ({})\n  {}",
            label,
            counts,
            lines.join("\n  ")
        );
        eprint!("\n  task packets: ./build/pipeline overlay --tasks\n");
        if strict {
            std::process::exit(1);
        }
        return buckets;
    }
    println!(
        "PASS  overlay ({}; every manual intervention still applies to {})",
        counts, pin.shadcn_ui.tag
    );
    buckets
}

fn ov_record(
    root: &Path,
    _shell: &crate::oracle::browser_shell::BrowserShell,
    pin: &OvPinFile,
) {
    let (tier_sets, _) = ov_load_tier_sets();
    let trivial: Vec<String> = tier_sets
        .iter()
        .filter(|(t, _)| t == "trivial")
        .flat_map(|(_, names)| names.clone())
        .collect();
    let mut manifest = OvManifestFile::default();
    if let Ok(b) = std::fs::read_to_string(root.join(OV_MANIFEST)) {
        let _ = serde_json::from_str::<OvManifestFile>(&b).map(|m| manifest = m);
    }
    let mut next = OvManifestFile {
        pin: pin.shadcn_ui.tag.clone(),
        commit: pin.shadcn_ui.commit.clone(),
        units: HashMap::new(),
    };
    let mut changed = 0usize;
    for u in ov_authored_units(root, pin, &trivial) {
        let h = ov_hash_unit(root, &u);
        if manifest.units.get(&u.id).map(|r| r.hash.clone()).unwrap_or_default() != h {
            changed += 1;
        }
        let extra: Vec<String> = u
            .extra
            .iter()
            .filter(|e| !e.starts_with("kernel:"))
            .cloned()
            .collect();
        let inputs = u.inputs.clone();
        next.units.insert(
            u.id,
            OvUnitRec {
                file: u.file,
                inputs,
                extra,
                hash: h,
                pin: pin.shadcn_ui.tag.clone(),
            },
        );
    }
    // Go json.NewEncoder with SetEscapeHTML(false) + SetIndent("", "  ") —
    // struct field order: pin, commit, units
    let mut units_json: Vec<String> = Vec::new();
    let mut ids: Vec<&String> = next.units.keys().collect();
    ids.sort();
    for id in ids {
        let u = &next.units[id];
        units_json.push(format!(
            "    {}: {{\n      \"file\": {},\n      \"inputs\": {},\n      \"extra\": {},\n      \"hash\": {},\n      \"pin\": {}\n    }}",
            crate::jsonorder::json_string(id),
            crate::jsonorder::json_string(&u.file),
            json_str_array(&u.inputs, "      "),
            json_str_array(&u.extra, "      "),
            crate::jsonorder::json_string(&u.hash),
            crate::jsonorder::json_string(&u.pin),
        ));
    }
    let out = format!(
        "{{\n  \"pin\": {},\n  \"commit\": {},\n  \"units\": {{\n{}\n  }}\n}}\n",
        crate::jsonorder::json_string(&next.pin),
        crate::jsonorder::json_string(&next.commit),
        units_json.join(",\n")
    );
    if let Err(e) = std::fs::write(root.join(OV_MANIFEST), out) {
        eprintln!("overlay: {}", e);
        std::process::exit(1);
    }
    println!(
        "overlay recorded: {} authored units anchored to {} ({} re-anchored)",
        next.units.len(),
        pin.shadcn_ui.tag,
        changed
    );
}

// Go json.Encoder SetIndent("", "  ") breaks every array, even a
// single-element one.
fn json_str_array(items: &[String], indent: &str) -> String {
    if items.is_empty() {
        return "[]".to_string();
    }
    // elements sit one level deeper than the array's own bracket
    let inner = format!("\n{}  ", indent);
    let parts: Vec<String> = items
        .iter()
        .map(|s| format!("{}{}", inner, crate::jsonorder::json_string(s)))
        .collect();
    format!("[{}\n{}]", parts.join(","), indent)
}

fn ov_run_tasks(
    root: &Path,
    shell: &crate::oracle::browser_shell::BrowserShell,
    pin: &OvPinFile,
) {
    let buckets = ov_audit(root, shell, pin, false);
    let _ = std::fs::remove_dir_all(root.join(OV_TASKS_DIR));
    let _ = std::fs::create_dir_all(root.join(OV_TASKS_DIR));
    let mut manifest = OvManifestFile::default();
    if let Ok(b) = std::fs::read_to_string(root.join(OV_MANIFEST)) {
        let _ = serde_json::from_str::<OvManifestFile>(&b).map(|m| manifest = m);
    }
    let gate_for = |u: &OvUnit| -> &'static str {
        if u.id.starts_with("behavior:") {
            return "contracts, style-parity, interactivity-sweep";
        }
        if u.id.starts_with("demo:") {
            return "example-gate, docs-smoke, interactivity-sweep";
        }
        if u.id.starts_with("runtime:") {
            return "contracts, demo-smoke";
        }
        "the full tier"
    };
    let mut n = 0usize;
    for bucket in ["stale", "orphaned", "conflict"] {
        let units: Vec<&OvUnit> = match bucket {
            "stale" => buckets.stale.iter().collect(),
            "orphaned" => buckets.orphaned.iter().collect(),
            _ => buckets.conflict.iter().collect(),
        };
        for u in units {
            let mut diff = String::new();
            if !u.inputs.is_empty() && !manifest.commit.is_empty() {
                for p in &u.inputs {
                    let out = std::process::Command::new("git")
                        .args([
                            "-C",
                            OV_UP,
                            "diff",
                            &format!("{}..{}", manifest.commit, pin.shadcn_ui.commit),
                            "--",
                            p,
                        ])
                        .output();
                    match out {
                        Ok(o) if o.status.success() => diff += &String::from_utf8_lossy(&o.stdout),
                        Ok(o) => {
                            // Go: msg = firstLine(stderr) of the failed git
                            let stderr = String::from_utf8_lossy(&o.stderr);
                            let msg = stderr.lines().next().unwrap_or("failed");
                            diff += &format!("(could not diff {}: {})\n", p, msg);
                        }
                        Err(e) => {
                            diff += &format!("(could not diff {}: {})\n", p, e);
                        }
                    }
                }
            }
            let what = if bucket == "stale" {
                format!(
                    "The upstream inputs this file was written against changed. Read the diff below, update `{}` so it reflects the new upstream, then run the gates listed and `./build/pipeline overlay --record`.",
                    u.file
                )
            } else if bucket == "orphaned" {
                "The thing this rule attaches to no longer exists upstream. Either delete the rule at its home, or re-anchor it. Then run the full tier.".to_string()
            } else {
                format!(
                    "Rebase the patch onto the pinned upstream (`git -C {} apply --3way`), resolve, regenerate with `git -C {} diff > {}`.",
                    OV_UP, OV_UP, u.file
                )
            };
            let mut file_line = String::new();
            let mut home_line = String::new();
            if !u.file.is_empty() {
                file_line = format!("**file**: `{}`", u.file);
            }
            if !u.home.is_empty() {
                home_line = format!("**home**: {}", u.home);
            }
            let mut diff_line = String::new();
            if !diff.is_empty() {
                let mut from = "null".to_string();
                let mut to = "null".to_string();
                if !manifest.commit.is_empty() {
                    from = manifest.commit[..manifest.commit.len().min(10)].to_string();
                }
                if pin.shadcn_ui.commit.len() >= 10 {
                    to = pin.shadcn_ui.commit[..10].to_string();
                }
                diff_line = format!("## Upstream diff ({} → {})\n\n```diff\n{}```", from, to, diff);
            }
            let mut cur_line = String::new();
            if !u.file.is_empty() {
                if let Ok(b) = std::fs::read_to_string(root.join(&u.file)) {
                    cur_line = format!("## Current file\n\n```\n{}```", b);
                }
            }
            let md = [
                format!("# {}", u.id),
                String::new(),
                format!("**bucket**: {}", bucket),
                format!("**reason**: {}", u.reason),
                file_line,
                home_line,
                String::new(),
                "## What to do".to_string(),
                String::new(),
                what,
                String::new(),
                format!(
                    "**gates to satisfy**: {}  —  `./build/pipeline run <gate>`",
                    gate_for(u)
                ),
                String::new(),
                diff_line,
                cur_line,
            ]
            .join("\n");
            let safe = Regex::new(r"[^0-9A-Za-z_.-]+")
                .unwrap()
                .replace_all(&u.id, "_")
                .into_owned();
            let _ = std::fs::write(root.join(OV_TASKS_DIR).join(format!("{}.md", safe)), md);
            n += 1;
        }
    }
    println!("overlay tasks: {} packets under {}/", n, OV_TASKS_DIR);
}

pub fn run_overlay(root: &Path, args: &[String]) -> i32 {
    let pin_b = match std::fs::read_to_string(root.join("src/registry/pin.json")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("overlay: {}", e);
            return 1;
        }
    };
    let pin: OvPinFile = match serde_json::from_str(&pin_b) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("overlay: {}", e);
            return 1;
        }
    };
    let shell = match crate::oracle::browser_shell::BrowserShell::start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("overlay: {}", e);
            return 1;
        }
    };
    let has = |f: &str| args.iter().any(|a| a == f);
    if has("--record") {
        ov_record(root, &shell, &pin);
        shell.close();
        return 0;
    }
    if has("--tasks") {
        ov_run_tasks(root, &shell, &pin);
        shell.close();
        return 0;
    }
    if has("--report") {
        ov_audit(root, &shell, &pin, false);
        shell.close();
        return 0;
    }
    ov_audit(root, &shell, &pin, true);
    shell.close();
    0
}
