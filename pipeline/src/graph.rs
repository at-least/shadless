//! Port of pipeline/graph.go — the pipeline graph and the one rule:
//!
//! ```text
//! a node needs recomputing only if its own inputs changed, or something
//! upstream of it did.
//! ```
//!
//! `plan` is a declaration-order DFS post-order (targets in argv order, each
//! node's `needs` in declared order) — the traversal order IS the output
//! interface and is golden-diffed (tests/gen_golden.sh records this engine's
//! own bytes).

use crate::nodes::{self, Node};
use std::collections::{HashMap, HashSet};
use std::path::Path;

const TIERS: [&str; 2] = ["fast", "full"];

fn tier_rank(t: &str) -> usize {
    for (i, x) in TIERS.iter().enumerate() {
        if *x == t {
            return i;
        }
    }
    TIERS.len() // unknown tier sorts last rather than silently passing as fast
}

pub struct Graph {
    nodes: HashMap<String, Node>,
    order: Vec<String>, // declaration order, for stable output
}

impl Graph {
    pub(crate) fn new(list: Vec<Node>) -> Result<Graph, String> {
        let mut g = Graph {
            nodes: HashMap::with_capacity(list.len()),
            order: Vec::with_capacity(list.len()),
        };
        for n in list {
            if g.nodes.contains_key(&n.id) {
                return Err(format!("duplicate node id: {}", n.id));
            }
            g.order.push(n.id.clone());
            g.nodes.insert(n.id.clone(), n);
        }
        for n in g.nodes.values() {
            for d in &n.needs {
                if !g.nodes.contains_key(d) {
                    return Err(format!("node {} needs unknown node {}", n.id, d));
                }
            }
        }
        Ok(g)
    }

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn ids(&self) -> &[String] {
        &self.order
    }

    /// The transitive closure of targets, in declaration-order DFS post-order.
    /// Cycles are an authoring error, reported with the path that closed them.
    pub fn plan(&self, targets: &[String]) -> Result<Vec<Node>, String> {
        let mut seen = HashSet::new();
        let mut visiting = HashSet::new();
        let mut out = Vec::new();
        for t in targets {
            visit(self, t, &[], &mut seen, &mut visiting, &mut out)?;
        }
        Ok(out)
    }

    /// The most expensive tier in a node's dependency closure, not the tier it
    /// declares: a "fast" gate that can only run after a browser build is not fast.
    pub fn effective_tier(&self, id: &str) -> String {
        fn walk(g: &Graph, id: &str, memo: &mut HashMap<String, usize>) -> usize {
            if let Some(r) = memo.get(id) {
                return *r;
            }
            let n = g.node(id).expect("graph is closed over needs");
            let mut worst = tier_rank(&n.tier);
            memo.insert(id.to_string(), worst); // cycle guard; Plan reports real cycles
            for d in &n.needs {
                let r = walk(g, d, memo);
                if r > worst {
                    worst = r;
                }
            }
            memo.insert(id.to_string(), worst);
            worst
        }
        let mut memo = HashMap::new();
        TIERS[walk(self, id, &mut memo)].to_string()
    }

    /// Every gate whose ENTIRE closure fits within tier, plus the builds those
    /// gates need.
    pub fn plan_tier(&self, tier: &str) -> Result<Vec<Node>, String> {
        let max = tier_rank(tier);
        let targets: Vec<String> = self
            .order
            .iter()
            .filter(|id| {
                let n = self.nodes.get(*id).expect("order holds only graph nodes");
                n.kind == "gate" && tier_rank(&self.effective_tier(id)) <= max
            })
            .cloned()
            .collect();
        self.plan(&targets)
    }

    /// Every build node and what it needs — every artifact, no gates.
    pub fn plan_builds(&self) -> Result<Vec<Node>, String> {
        let targets: Vec<String> = self
            .order
            .iter()
            .filter(|id| self.nodes.get(*id).expect("order nodes").kind == "build")
            .cloned()
            .collect();
        self.plan(&targets)
    }
}

fn visit(
    g: &Graph,
    id: &str,
    path: &[String],
    seen: &mut HashSet<String>,
    visiting: &mut HashSet<String>,
    out: &mut Vec<Node>,
) -> Result<(), String> {
    if seen.contains(id) {
        return Ok(());
    }
    if visiting.contains(id) {
        let mut p = path.to_vec();
        p.push(id.to_string());
        return Err(format!("cycle in graph: [{}]", p.join(" ")));
    }
    let n = g.node(id).ok_or_else(|| format!("unknown node id: {}", id))?;
    visiting.insert(id.to_string());
    let mut child_path = path.to_vec();
    child_path.push(id.to_string());
    for d in &n.needs {
        visit(g, d, &child_path, seen, visiting, out)?;
    }
    visiting.remove(id);
    seen.insert(id.to_string());
    out.push(n.clone());
    Ok(())
}

/// LoadGraphAt: build the graph from nodes::all(), expanding any node the
/// fan-out table splits into independent per-item nodes and resolving the
/// `produces` entries that are derived from the tree rather than written out.
pub fn load_graph_at(root: &Path) -> Result<Graph, String> {
    let list = crate::fanout::expand_fanout(root, nodes::all())?;
    let list = crate::produces::apply_derived_produces(root, list)?;
    Graph::new(list)
}

/// AuthoredGraph (Go meta.go): the authored table with no fanout expansion —
/// the view the mutation harness reasons about (`contracts` as one gate, not
/// 29 shards).
pub fn authored() -> Result<Graph, String> {
    Graph::new(nodes::all())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn authored() -> Graph {
        Graph::new(nodes::all()).expect("authored graph is consistent")
    }

    #[test]
    fn authored_graph_shape() {
        let g = authored();
        assert_eq!(g.ids().len(), 41);
        assert!(g.node("pin").is_some());
        assert!(g.node("contracts").is_some());
        assert!(g.node("nope").is_none());
    }

    #[test]
    fn plan_is_declaration_order_dfs_post_order() {
        let g = authored();
        // "pin" is declared first and needs nothing: it is the whole plan.
        let plan = g.plan(&["pin".to_string()]).unwrap();
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].id, "pin");
        // example-oracle needs emit then build-js (declared order): build-js's
        // own needs (none) come first, then emit's closure (convert's needs
        // pin then build-js), so pin precedes build-js even though targets
        // start at example-oracle.
        let plan = g.plan(&["example-oracle".to_string()]).unwrap();
        let ids: Vec<&str> = plan.iter().map(|n| n.id.as_str()).collect();
        let pos = |s: &str| ids.iter().position(|x| *x == s).unwrap();
        assert!(pos("pin") < pos("build-js"));
        assert!(pos("build-js") < pos("convert"));
        assert!(pos("convert") < pos("emit"));
        assert!(pos("emit") < pos("example-oracle"));
        assert_eq!(*ids.last().unwrap(), "example-oracle");
    }

    #[test]
    fn cycle_reports_the_closing_path() {
        let mut n_a = nodes::all().into_iter().find(|n| n.id == "pin").unwrap();
        n_a.needs = vec!["unit".to_string()];
        let mut list = nodes::all();
        list.retain(|n| n.id != "pin");
        list.push(n_a);
        // unit needs build-js; rewire pin->unit and unit->pin to close a cycle
        let unit = list.iter_mut().find(|n| n.id == "unit").unwrap();
        unit.needs.push("pin".to_string());
        let g = Graph::new(list).unwrap();
        let err = g.plan(&["pin".to_string()]).unwrap_err();
        assert_eq!(err, "cycle in graph: [pin unit pin]");
    }

    #[test]
    fn duplicate_and_unknown_deps_are_authoring_errors() {
        let mut list = nodes::all();
        let pin = list.first().unwrap().clone();
        list.push(pin);
        let err = Graph::new(list.clone()).err().unwrap();
        assert_eq!(err, "duplicate node id: pin");
        let mut list = nodes::all();
        list.first_mut().unwrap().needs.push("ghost".to_string());
        let err = Graph::new(list).err().unwrap();
        assert_eq!(err, "node pin needs unknown node ghost");
    }

    #[test]
    fn effective_tier_is_the_closure_maximum() {
        let g = authored();
        assert_eq!(g.effective_tier("pin"), "fast");
        // css-direction declares fast but sits downstream of demo (full).
        assert_eq!(g.effective_tier("css-direction"), "full");
        assert_eq!(g.effective_tier("typecheck"), "fast");
    }

    #[test]
    fn plan_tier_fast_is_every_fast_gate_plus_needed_builds() {
        let g = authored();
        let plan = g.plan_tier("fast").unwrap();
        let gates: Vec<&str> = plan
            .iter()
            .filter(|n| n.kind == "gate")
            .map(|n| n.id.as_str())
            .collect();
        // gates whose ENTIRE closure fits in fast: unit only needs build-js
        // (a fast build) so it stays; dist-complete/pack/coverage/overlay/
        // css-direction and both docs gates all reach a full-tier build.
        // docs-upstream-mirror is tier fast but kind build — not a gate.
        assert_eq!(
            gates,
            vec!["pin", "unit", "typecheck", "ledger", "script-refs"]
        );
        assert!(plan.iter().all(|n| n.tier == "fast" || n.kind == "build"));
    }

    #[test]
    fn plan_builds_has_no_gates() {
        let g = authored();
        let plan = g.plan_builds().unwrap();
        // the closure is "every build node and what it needs": pin (a gate)
        // legitimately appears because convert/rtl-dict/docs-upstream-mirror
        // need it; every other entry is a build.
        assert!(plan.iter().all(|n| n.kind == "build" || n.id == "pin"));
        assert!(plan.iter().any(|n| n.id == "emit"));
        assert!(plan.iter().any(|n| n.id == "pin"));
        assert!(!plan.iter().any(|n| n.id == "coverage"));
    }
}
