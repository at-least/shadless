//! Port of pipeline/produces.go: derived `produces`, where a node's real
//! output set is DATA (src/registry/tiers.json), not a glob. `emit` writes the
//! static-tier pages and `demo` writes the shipped non-static ones; both
//! declared `dist/components/*.html`, which over-declared in the unsafe
//! direction (the access checks treat a dependency-closure `produces` as
//! covered).

use crate::nodes::Node;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const COMPONENT_PAGES_GLOB: &str = "dist/components/*.html";
const COMPONENT_PAGES_NO_RT: &str = "!dist/components/*-rtl-*.html";
const TIERS_PATH: &str = "src/registry/tiers.json";

#[derive(Deserialize)]
struct ComponentTier {
    #[serde(default)]
    tier: String,
    #[serde(default)]
    emit: bool,
}

fn component_tiers(root: &Path) -> Result<HashMap<String, ComponentTier>, String> {
    let b = fs::read_to_string(root.join(TIERS_PATH)).map_err(|e| e.to_string())?;
    let m: HashMap<String, ComponentTier> =
        serde_json::from_str(&b).map_err(|e| format!("{}: {}", TIERS_PATH, e))?;
    if m.is_empty() {
        return Err(format!("{}: no components", TIERS_PATH));
    }
    Ok(m)
}

/// Whether a component gets a page in dist/components at all: three tiers ship
/// wholesale; stragglers carry `emit: true`.
fn shipped(t: &ComponentTier) -> bool {
    matches!(t.tier.as_str(), "static" | "kernel" | "trivial-js") || t.emit
}

fn pages(
    root: &Path,
    want: impl Fn(&ComponentTier) -> bool,
) -> Result<Vec<String>, String> {
    let m = component_tiers(root)?;
    let mut out: Vec<String> = m
        .iter()
        .filter(|(_, t)| want(t))
        .map(|(name, _)| format!("dist/components/{}.html", name))
        .collect();
    out.sort();
    if out.is_empty() {
        return Err(format!("{}: no pages matched", TIERS_PATH));
    }
    Ok(out)
}

/// Substitute the explicit page list for the directory glob, in place, leaving
/// every other `produces` entry as declared. A derivation that cannot read its
/// data must not quietly fall back to the glob it is replacing.
pub fn apply_derived_produces(root: &Path, nodes: Vec<Node>) -> Result<Vec<Node>, String> {
    let mut out = nodes;
    for n in out.iter_mut() {
        let static_pages = n.id == "emit";
        let non_static_pages = n.id == "demo";
        if !static_pages && !non_static_pages {
            continue;
        }
        let list = if static_pages {
            pages(root, |t| t.tier == "static")
        } else {
            pages(root, |t| shipped(t) && t.tier != "static")
        }
        .map_err(|e| format!("produces for {}: {}", n.id, e))?;
        let mut kept: Vec<String> = Vec::new();
        let mut substituted = false;
        for p in n.produces.as_deref().unwrap_or(&[]) {
            if p == COMPONENT_PAGES_GLOB || p == COMPONENT_PAGES_NO_RT {
                substituted = true;
                continue;
            }
            kept.push(p.clone());
        }
        if !substituted {
            return Err(format!(
                "produces for {}: no {:?} to substitute",
                n.id, COMPONENT_PAGES_GLOB
            ));
        }
        let mut produces = list;
        produces.extend(kept);
        n.produces = Some(produces);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_tiers(body: &str) -> std::path::PathBuf {
        let tmp = std::env::temp_dir().join(format!(
            "shadless-rs-produces-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        fs::create_dir_all(tmp.join("src/registry")).unwrap();
        fs::write(tmp.join(TIERS_PATH), body).unwrap();
        tmp
    }

    fn emit_node() -> Node {
        crate::nodes::all()
            .into_iter()
            .find(|n| n.id == "emit")
            .unwrap()
    }

    #[test]
    fn substitution_replaces_both_component_globs() {
        let tmp = tmp_tiers(
            r#"{"a":{"tier":"static"},"b":{"tier":"static"},"c":{"tier":"kernel"},"d":{"tier":"trivial-js"},"e":{"tier":"interactive","emit":true},"f":{"tier":"interactive"}}"#,
        );
        let list = apply_derived_produces(&tmp, vec![emit_node()]).unwrap();
        let p = list[0].produces.as_deref().unwrap();
        // emit: static tier only; the !-rtl exclusion glob is substituted away
        assert_eq!(
            p,
            vec![
                "dist/components/a.html".to_string(),
                "dist/components/b.html".to_string(),
                "dist/shadless.css".to_string(),
                "build/emit".to_string(),
            ]
        );
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn non_static_pages_ship_everything_but_static() {
        let tmp = tmp_tiers(
            r#"{"a":{"tier":"static"},"c":{"tier":"kernel"},"e":{"tier":"interactive","emit":true},"f":{"tier":"interactive"}}"#,
        );
        let mut demo = crate::nodes::all()
            .into_iter()
            .find(|n| n.id == "demo")
            .unwrap();
        let list = apply_derived_produces(&tmp, vec![demo.clone()]).unwrap();
        demo = list.into_iter().next().unwrap();
        let p = demo.produces.as_deref().unwrap();
        assert!(p.contains(&"dist/components/c.html".to_string()));
        assert!(p.contains(&"dist/components/e.html".to_string()));
        assert!(!p.contains(&"dist/components/a.html".to_string()));
        assert!(!p.contains(&"dist/components/f.html".to_string()));
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn unreadable_data_fails_loudly() {
        let tmp = std::env::temp_dir().join(format!("shadless-rs-produces-miss-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let err = apply_derived_produces(&tmp, vec![emit_node()]).unwrap_err();
        assert!(err.starts_with("produces for emit:"), "{}", err);
    }
}
