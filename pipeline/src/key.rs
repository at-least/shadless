//! Port of the keys half of shadless/pipeline/key.go.
//!
//! The whole rule: key(n) = H( n.id, n.run, contents of every declared input,
//! key(d) for every d in n.needs ). Hash-field bytes ("node\x00..\n" etc.)
//! are reproduced exactly, so keys were byte-identical with the Go binary's
//! (the port's bar — the Go engine is history, go-parity-final tag), which is
//! what makes the `status` golden-diff a test of this file.

use crate::glob::files;
use crate::graph::Graph;
use crate::nodes::Node;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn hash_file(path: &Path) -> std::io::Result<String> {
    let data = fs::read(path)?;
    let mut h = Sha256::new();
    h.update(&data);
    Ok(hex::encode(h.finalize()))
}

pub struct Keyer<'g> {
    root: PathBuf,
    graph: &'g Graph,
    memo: HashMap<String, String>, // id -> key; "" = never fresh
}

impl<'g> Keyer<'g> {
    pub fn new(root: &Path, graph: &'g Graph) -> Self {
        Self {
            root: root.to_path_buf(),
            graph,
            memo: HashMap::new(),
        }
    }

    /// Ok(None) = the node (or something upstream) declares no input set and
    /// can never be skipped; Ok(Some(key)) = the content key.
    pub fn key(&mut self, id: &str) -> Result<Option<String>, String> {
        if let Some(v) = self.memo.get(id) {
            return Ok(if v.is_empty() { None } else { Some(v.clone()) });
        }
        let n = self
            .graph
            .node(id)
            .ok_or_else(|| format!("unknown node id: {}", id))?;
        if n.never_fresh() {
            self.memo.insert(id.to_string(), String::new());
            return Ok(None);
        }
        let mut h = Sha256::new();
        h.update(format!("node\x00{}\n", n.id));
        for cmd in &n.run {
            h.update(format!("run\x00{}\n", cmd.join("\x00")));
        }

        let mut deps = n.needs.clone();
        deps.sort();
        for d in &deps {
            match self.key(d)? {
                Some(dk) => h.update(format!("dep\x00{}\x00{}\n", d, dk)),
                None => {
                    // an unskippable dependency makes this node unskippable
                    self.memo.insert(id.to_string(), String::new());
                    return Ok(None);
                }
            }
        }

        let inputs: &[String] = n.inputs.as_deref().unwrap_or(&[]);
        let list = files(&self.root, inputs)?;
        for f in list {
            let fh = hash_file(&self.root.join(&f)).map_err(|e| format!("{}: {}", f, e))?;
            h.update(format!("in\x00{}\x00{}\n", f, fh));
        }

        let key = hex::encode(h.finalize());
        self.memo.insert(id.to_string(), key.clone());
        Ok(Some(key))
    }
}

/// Hash of the contents of everything the node declares as output. Errors
/// collapse to "" — an unreadable output set is never equal to a recorded
/// digest, so the node rebuilds, which is the safe direction.
pub fn outputs_digest(root: &Path, n: &Node) -> String {
    let produces: &[String] = n.produces.as_deref().unwrap_or(&[]);
    let list = match files(root, produces) {
        Ok(f) => f,
        Err(_) => return String::new(),
    };
    let mut h = Sha256::new();
    for f in list {
        let fh = match hash_file(&root.join(&f)) {
            Ok(v) => v,
            Err(_) => return String::new(),
        };
        h.update(format!("out\x00{}\x00{}\n", f, fh));
    }
    hex::encode(h.finalize())
}

/// What a stamp file holds: the input key and the output digest, together.
pub fn stamp_value(root: &Path, n: &Node, key: &str) -> String {
    format!("{}:{}", key, outputs_digest(root, n))
}

/// Everything the node claims to produce is on disk? Checked against the
/// literal prefix of each pattern: "did this get built", not "is every file
/// byte-for-byte what it was". Err(prefix) names the missing output.
pub fn outputs_present(root: &Path, n: &Node) -> Result<(), String> {
    let produces: &[String] = n.produces.as_deref().unwrap_or(&[]);
    for pat in produces {
        if pat.starts_with('!') {
            continue;
        }
        let p = crate::glob::literal_prefix(pat);
        if !p.is_empty() && fs::metadata(root.join(&p)).is_err() {
            return Err(p);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::Node as N;

    fn tmp(tag: &str) -> PathBuf {
        let t = std::env::temp_dir().join(format!(
            "shadless-rs-key-{}-{:?}-{}",
            std::process::id(),
            std::thread::current().id(),
            tag
        ));
        fs::create_dir_all(&t).unwrap();
        t
    }

    fn mnode(
        id: &str,
        needs: &[&str],
        inputs: Option<&[&str]>,
        produces: Option<&[&str]>,
    ) -> N {
        N {
            id: id.to_string(),
            kind: "build".to_string(),
            tier: "fast".to_string(),
            needs: needs.iter().map(|s| s.to_string()).collect(),
            run: vec![vec!["echo".to_string(), id.to_string()]],
            inputs: inputs.map(|v| v.iter().map(|s| s.to_string()).collect()),
            produces: produces.map(|v| v.iter().map(|s| s.to_string()).collect()),
            why: String::new(),
            mutations: vec![],
        }
    }

    #[test]
    fn hash_file_matches_known_sha256() {
        let t = tmp("hash");
        fs::write(t.join("empty"), "").unwrap();
        fs::write(t.join("abc"), "abc").unwrap();
        assert_eq!(
            hash_file(&t.join("empty")).unwrap(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hash_file(&t.join("abc")).unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn key_is_stable_and_tracks_inputs_and_run() {
        let t = tmp("stable");
        fs::write(t.join("in.txt"), "v1").unwrap();
        let g = Graph::new(vec![mnode("a", &[], Some(&["in.txt"]), None)]).unwrap();
        let k1 = Keyer::new(&t, &g).key("a").unwrap().unwrap();
        let k2 = Keyer::new(&t, &g).key("a").unwrap().unwrap();
        assert_eq!(k1, k2);

        // content change invalidates
        fs::write(t.join("in.txt"), "v2").unwrap();
        let k3 = Keyer::new(&t, &g).key("a").unwrap().unwrap();
        assert_ne!(k1, k3);

        // an unrelated file does not
        fs::write(t.join("other.txt"), "x").unwrap();
        let k4 = Keyer::new(&t, &g).key("a").unwrap().unwrap();
        assert_eq!(k3, k4);

        // the command is part of the key
        let mut n = mnode("a", &[], Some(&["in.txt"]), None);
        n.run = vec![vec!["echo".to_string(), "changed".to_string()]];
        let g2 = Graph::new(vec![n]).unwrap();
        let k5 = Keyer::new(&t, &g2).key("a").unwrap().unwrap();
        assert_ne!(k3, k5);
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn dep_keys_fold_and_needs_order_is_irrelevant() {
        let t = tmp("fold");
        fs::write(t.join("c.txt"), "child").unwrap();
        let child = mnode("child", &[], Some(&["c.txt"]), None);
        let mut parent = mnode("parent", &["child"], Some(&["c.txt"]), None);
        parent.needs = vec!["child".to_string()];
        let g1 = Graph::new(vec![parent.clone(), child.clone()]).unwrap();
        let k1 = Keyer::new(&t, &g1).key("parent").unwrap().unwrap();

        // a change in the CHILD's inputs propagates into the parent's key
        fs::write(t.join("c.txt"), "child2").unwrap();
        let g2 = Graph::new(vec![parent.clone(), child.clone()]).unwrap();
        let k2 = Keyer::new(&t, &g2).key("parent").unwrap().unwrap();
        assert_ne!(k1, k2);

        // needs declared in a different order hash the same (deps are sorted)
        parent.needs = vec!["child".to_string(), "aaa".to_string(), "zzz".to_string()];
        let mut aaa = mnode("aaa", &[], None, None);
        aaa.inputs = Some(vec![]);
        let mut zzz = aaa.clone();
        zzz.id = "zzz".to_string();
        let g3 = Graph::new(vec![parent.clone(), child.clone(), aaa.clone(), zzz.clone()]).unwrap();
        let k3 = Keyer::new(&t, &g3).key("parent").unwrap().unwrap();
        parent.needs = vec!["zzz".to_string(), "child".to_string(), "aaa".to_string()];
        let g4 = Graph::new(vec![parent, child, aaa, zzz]).unwrap();
        let k4 = Keyer::new(&t, &g4).key("parent").unwrap().unwrap();
        assert_eq!(k3, k4);
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn never_fresh_poisons_downstream() {
        let t = tmp("never");
        let judge = mnode("judge", &[], None, None); // inputs: None
        let downstream = mnode("down", &["judge"], Some(&["in.txt"]), None);
        let g = Graph::new(vec![judge, downstream]).unwrap();
        let mut k = Keyer::new(&t, &g);
        assert!(k.key("judge").unwrap().is_none());
        assert!(k.key("down").unwrap().is_none());
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn outputs_digest_present_and_stamp_value() {
        let t = tmp("outputs");
        fs::create_dir_all(t.join("out")).unwrap();
        fs::write(t.join("out/x.txt"), "one").unwrap();
        let bare = mnode("a", &[], Some(&["in.txt"]), None);
        // no declared outputs: digest of nothing
        assert_eq!(
            outputs_digest(&t, &bare),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        let mut producing = mnode("a", &[], Some(&["in.txt"]), Some(&["out"]));
        let d1 = outputs_digest(&t, &producing);
        assert_ne!(d1, outputs_digest(&t, &bare));
        fs::write(t.join("out/x.txt"), "two").unwrap();
        assert_ne!(d1, outputs_digest(&t, &producing));

        // present: dir prefix exists; the ! exclusion is skipped
        assert_eq!(outputs_present(&t, &producing), Ok(()));
        producing.produces = Some(vec![
            "out".to_string(),
            "!missing/**".to_string(),
            "out/x.txt".to_string(),
        ]);
        assert_eq!(outputs_present(&t, &producing), Ok(()));
        producing.produces = Some(vec!["no/such/dir/file".to_string()]);
        // a fully literal pattern's prefix is the whole path (Go semantics):
        // the FILE itself is what's missing, not its directory
        assert_eq!(
            outputs_present(&t, &producing),
            Err("no/such/dir/file".to_string())
        );

        let key = "deadbeef";
        assert!(stamp_value(&t, &bare, key).starts_with(&format!("{}:", key)));
        let _ = fs::remove_dir_all(&t);
    }
}
