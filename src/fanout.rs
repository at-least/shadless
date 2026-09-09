//! Port of pipeline/fanout.go: where the runner's view of the graph is FINER
//! than the registry's. `contracts` becomes one node per contract def, each
//! with its own key and its own worker slot.

use crate::nodes::Node;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn fan_contracts(root: &Path, n: &Node) -> std::io::Result<Vec<Node>> {
    let entries = fs::read_dir(root.join("tools/contracts/components"))?;
    let mut names: Vec<String> = entries
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|f| f.ends_with(".mjs"))
        .map(|f| f.trim_end_matches(".mjs").to_string())
        .collect();
    names.sort();

    // the two broad patterns cover every def, which is what we are narrowing;
    // the harness files they also matched are named explicitly
    let shared: Vec<String> = n
        .inputs
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .filter(|p| *p != "tools/contracts/**/*.mjs" && *p != "tools/contracts/components/**")
        .cloned()
        .collect();

    let mut out = Vec::with_capacity(names.len());
    for name in &names {
        let mut c = n.clone();
        c.id = format!("contracts:{}", name);
        c.run = vec![vec![
            crate::nodes::engine_argv0(),
            "contract".to_string(),
            name.clone(),
        ]];
        let mut inputs = vec![format!("tools/contracts/components/{}.mjs", name)];
        inputs.extend(shared.clone());
        c.inputs = Some(inputs);
        c.produces = Some(vec![format!("tools/contracts/out/{}", name)]);
        out.push(c);
    }
    Ok(out)
}

/// expandFanout rewrites the node list, rewiring every `needs` that pointed at
/// a split node to point at all of its parts.
pub fn expand_fanout(root: &Path, nodes: Vec<Node>) -> Result<Vec<Node>, String> {
    let mut replaced: HashMap<String, Vec<String>> = HashMap::new();
    let mut out: Vec<Node> = Vec::new();
    for n in nodes {
        if n.id == "contracts" {
            let parts =
                fan_contracts(root, &n).map_err(|e| format!("tools/contracts/components: {}", e))?;
            replaced.insert(
                n.id.clone(),
                parts.iter().map(|p| p.id.clone()).collect(),
            );
            out.extend(parts);
            continue;
        }
        out.push(n);
    }
    if replaced.is_empty() {
        return Ok(out);
    }
    for n in out.iter_mut() {
        let mut needs = Vec::with_capacity(n.needs.len());
        for d in &n.needs {
            match replaced.get(d) {
                Some(parts) => needs.extend(parts.iter().cloned()),
                None => needs.push(d.clone()),
            }
        }
        n.needs = needs;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contracts_fans_out_and_needs_rewire() {
        let tmp = std::env::temp_dir().join(format!("shadless-rs-fanout-{}", std::process::id()));
        let comp = tmp.join("tools/contracts/components");
        fs::create_dir_all(&comp).unwrap();
        for name in ["dialog.mjs", "button.mjs", "readme.txt"] {
            fs::write(comp.join(name), "").unwrap();
        }
        let list = expand_fanout(&tmp, crate::nodes::all()).unwrap();
        // 40 authored nodes + 2 shards replacing contracts
        assert_eq!(list.len(), 42);
        let shards: Vec<&Node> = list.iter().filter(|n| n.id.starts_with("contracts:")).collect();
        assert_eq!(
            shards.iter().map(|n| n.id.as_str()).collect::<Vec<_>>(),
            vec!["contracts:button", "contracts:dialog"]
        );
        // shard run/inputs/produces — argv0 must be whatever the presented
        // graph runs (Go path under SHADLESS_GRAPH=go-mirror, else __self__)
        assert_eq!(
            shards[0].run,
            vec![vec![
                crate::nodes::engine_argv0(),
                "contract".to_string(),
                "button".to_string()
            ]]
        );
        assert_eq!(
            shards[0].produces.as_deref(),
            Some(&["tools/contracts/out/button".to_string()][..])
        );
        assert_eq!(
            shards[0].inputs.as_deref().unwrap().first(),
            Some(&"tools/contracts/components/button.mjs".to_string())
        );
        // the broad def-glob is narrowed out of the shared inputs
        assert!(!shards[0]
            .inputs
            .as_deref()
            .unwrap()
            .contains(&"tools/contracts/components/**".to_string()));
        // style-parity's needs rewired to every shard, in sorted order
        let sp = list.iter().find(|n| n.id == "style-parity").unwrap();
        assert_eq!(
            sp.needs,
            vec![
                "contracts:button".to_string(),
                "contracts:dialog".to_string(),
                "oracle-css".to_string(),
                "build-js".to_string()
            ]
        );
        let _ = fs::remove_dir_all(&tmp);
    }
}
