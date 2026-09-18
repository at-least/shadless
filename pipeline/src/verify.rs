//! Port of pipeline/verify.go — declared vs actual writes and reads.
//!
//! The write check compares the input universe before and after a node's run
//! (only meaningful at -j1, where nothing else is writing). The read check
//! parses the evidence the commands themselves produced: `go test
//! -test.testlogfile` records every file the test binary opened, and
//! tools/fs-record.mjs (injected via NODE_OPTIONS) records node-side reads.

use crate::glob::{files, glob_to_regexp, literal_prefix};
use crate::graph::Graph;
use crate::key::hash_file;
use crate::nodes::Node;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

/// Every file the graph declares as an input, anywhere.
pub fn input_universe(root: &Path, g: &Graph) -> Result<BTreeMap<String, String>, String> {
    let mut snap = BTreeMap::new();
    for id in g.ids() {
        let n = g.node(id).expect("ids are graph nodes");
        if n.never_fresh() {
            continue;
        }
        let list = files(root, n.inputs.as_deref().unwrap_or(&[]))?;
        for f in list {
            if snap.contains_key(&f) {
                continue;
            }
            // fail closed: an input that exists but cannot be read would
            // otherwise vanish from the universe and weaken the check
            let h = hash_file(&root.join(&f)).map_err(|e| format!("{}: {}", f, e))?;
            snap.insert(f, h);
        }
    }
    Ok(snap)
}

pub struct Violation {
    pub path: String,
    pub readers: Vec<String>, // nodes declaring path as an input
}

/// Files that changed across a node's run, are read by somebody, and are not
/// covered by that node's `produces`.
pub fn undeclared_writes(
    root: &Path,
    g: &Graph,
    n: &Node,
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
) -> Result<Vec<Violation>, String> {
    let produces: &[String] = n.produces.as_deref().unwrap_or(&[]);
    let produced = files(root, produces)?;
    let declared: HashSet<&String> = produced.iter().collect();
    // `produces` may name a directory that did not exist before the run, so
    // also treat anything under a declared literal prefix as covered.
    let covered = |p: &str| -> bool {
        if declared.contains(&p.to_string()) {
            return true;
        }
        for pat in produces {
            if let Ok(re) = glob_to_regexp(pat) {
                if re.is_match(p) {
                    return true;
                }
            }
            let lp = literal_prefix(pat);
            if lp == *pat && p.len() > lp.len() + 1 && p.starts_with(&format!("{}/", lp)) {
                return true;
            }
        }
        false
    };

    let mut out: Vec<Violation> = Vec::new();
    for (path, h) in after {
        if before.get(path).map(String::as_str) == Some(h) {
            continue;
        }
        if covered(path) {
            continue;
        }
        let mut readers: Vec<String> = Vec::new();
        for id in g.ids() {
            let m = g.node(id).expect("ids are graph nodes");
            if m.never_fresh() {
                continue;
            }
            let fs_list = match files(root, m.inputs.as_deref().unwrap_or(&[])) {
                Ok(f) => f,
                Err(_) => continue,
            };
            if fs_list.iter().any(|f| f == path) {
                readers.push(id.clone());
            }
        }
        if readers.is_empty() {
            continue; // written but read by nobody: cannot affect freshness
        }
        out.push(Violation {
            path: path.clone(),
            readers,
        });
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

/// Port of reportViolations. Go `%v` of a []NodeID prints "[a b]".
pub fn report_violations(id: &str, vs: &[Violation]) {
    if vs.is_empty() {
        return;
    }
    println!(
        "  ⚠ {} wrote {} file(s) it does not declare in `produces`:",
        id,
        vs.len()
    );
    for v in vs {
        println!(
            "      {}   (declared as an input by: [{}])",
            v.path,
            v.readers.join(" ")
        );
    }
}

/// The shared skeleton behind both file-access logs: read logPath, pull a path
/// out of each line, resolve it against root, drop anything outside the repo or
/// excluded, drop directories, dedupe, sort.
fn parse_opened_files(
    root: &Path,
    log_path: &Path,
    parse_line: impl Fn(&str) -> Option<String>,
    exclude: impl Fn(&str) -> bool,
) -> Result<Vec<String>, String> {
    let b = fs::read_to_string(log_path).map_err(|e| e.to_string())?;
    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for line in b.split('\n') {
        let Some(path) = parse_line(line) else {
            continue;
        };
        if !Path::new(&path).is_absolute() {
            continue; // relative entries are the test's own cwd noise
        }
        let Ok(rel) = Path::new(&path).strip_prefix(root) else {
            continue; // outside the repo
        };
        let rel = rel.to_string_lossy().into_owned();
        if rel.starts_with("..") || seen.contains(&rel) || exclude(&rel) {
            continue;
        }
        match fs::metadata(&path) {
            Ok(info) if !info.is_dir() => {}
            _ => continue,
        }
        seen.insert(rel.clone());
        out.push(rel);
    }
    out.sort();
    Ok(out)
}

/// The repo-relative regular files a testlog records as opened. Directories,
/// absent paths and anything outside the repo (GOROOT, the module cache) are
/// dropped.
fn testlog_opens(root: &Path, log_path: &Path) -> Result<Vec<String>, String> {
    parse_opened_files(
        root,
        log_path,
        |line| line.strip_prefix("open ").map(|p| p.to_string()),
        |_| false,
    )
}

fn excluded_from_access_check(rel: &str) -> bool {
    for prefix in ["node_modules/", "build/", ".git/"] {
        if rel.starts_with(prefix) {
            return true;
        }
    }
    false
}

/// Parses the log tools/fs-record.mjs writes: one absolute path per line,
/// already de-duplicated per process, possibly appended to by several
/// processes in a node's command list.
fn fs_record_opens(root: &Path, log_path: &Path) -> Result<Vec<String>, String> {
    parse_opened_files(
        root,
        log_path,
        |line| {
            let p = line.trim();
            if p.is_empty() {
                None
            } else {
                Some(p.to_string())
            }
        },
        excluded_from_access_check,
    )
}

/// Collects the undeclared-read evidence across a node's logs (both kinds),
/// deduped in first-seen order.
pub fn opens_from_logs(root: &Path, logs: &[std::path::PathBuf]) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut all: Vec<String> = Vec::new();
    for log in logs {
        let opens = if log.to_string_lossy().ends_with("js.log") {
            fs_record_opens(root, log)
        } else {
            testlog_opens(root, log)
        };
        let opens = match opens {
            Ok(o) => o,
            Err(e) => {
                // An absent log is "no node child ran" — no evidence, not a
                // violation (fs-record.mjs touches the log on load, so a
                // node child that ran leaves it present). One that exists
                // but cannot be read is lost evidence and must say so.
                if log.exists() {
                    eprintln!(
                        "pipeline: access log {} unreadable ({}); its read evidence is lost",
                        log.display(),
                        e
                    );
                }
                continue;
            }
        };
        for p in opens {
            if seen.insert(p.clone()) {
                all.push(p);
            }
        }
    }
    all
}

/// Files the node opened that nothing puts in its key. Covered: its own
/// `inputs`, its own `produces`, and anything any node in its dependency
/// closure declares in either direction (the merkle chain: a dependency's key
/// already hashes that dependency's inputs, so a change to either reaches this
/// node transitively).
pub fn undeclared_reads(
    root: &Path,
    g: &Graph,
    n: &Node,
    opens: &[String],
) -> Result<Vec<String>, String> {
    if n.never_fresh() {
        return Ok(Vec::new()); // it can never be skipped, so nothing can go stale-green
    }
    let mut covered: HashSet<String> = HashSet::new();
    let add =
        |patterns: &[String], covered: &mut HashSet<String>| -> Result<(), String> {
            for f in files(root, patterns)? {
                covered.insert(f);
            }
            Ok(())
        };
    add(n.inputs.as_deref().unwrap_or(&[]), &mut covered)?;

    // everything the closure declares, in either direction
    let mut produce_patterns: Vec<String> = n.produces.as_deref().unwrap_or(&[]).to_vec();
    if let Ok(closure) = g.plan(&[n.id.clone()]) {
        for d in &closure {
            if d.id == n.id {
                continue;
            }
            produce_patterns.extend(d.produces.as_deref().unwrap_or(&[]).iter().cloned());
            add(d.inputs.as_deref().unwrap_or(&[]), &mut covered)?;
        }
    }
    add(&produce_patterns, &mut covered)?;
    // a `produces` entry may name a directory that did not exist when the
    // patterns were expanded, so also treat anything under a literal prefix
    // as covered — the same allowance the write check makes
    let under_produces = |p: &str| -> bool {
        produce_patterns.iter().any(|pat| {
            let lp = literal_prefix(pat);
            lp == *pat && p.starts_with(&format!("{}/", lp))
        })
    };

    let mut out: Vec<String> = Vec::new();
    for p in opens {
        if covered.contains(p) || under_produces(p) {
            continue;
        }
        out.push(p.clone());
    }
    Ok(out)
}

/// Port of reportUndeclaredReads.
pub fn report_undeclared_reads(id: &str, paths: &[String]) {
    if paths.is_empty() {
        return;
    }
    println!(
        "  ⚠ {} opened {} file(s) it declares in neither `inputs` nor `produces`:",
        id,
        paths.len()
    );
    for p in paths {
        println!("      {}", p);
    }
    println!("      a file it READS belongs in `inputs` (it is not in the node's key, so a\n      change to it leaves the node falsely fresh); a file it WRITES belongs in `produces`");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::Node as N;

    fn mnode(id: &str, needs: &[&str], inputs: Option<&[&str]>, produces: Option<&[&str]>) -> N {
        N {
            id: id.to_string(),
            kind: "build".to_string(),
            tier: "fast".to_string(),
            needs: needs.iter().map(|s| s.to_string()).collect(),
            run: vec![],
            inputs: inputs.map(|v| v.iter().map(|s| s.to_string()).collect()),
            produces: produces.map(|v| v.iter().map(|s| s.to_string()).collect()),
            why: String::new(),
            mutations: vec![],
        }
    }

    #[test]
    fn testlog_parse_drops_relative_dirs_and_outside() {
        let t = std::env::temp_dir().join(format!("shadless-rs-verify-{}", std::process::id()));
        fs::create_dir_all(t.join("sub")).unwrap();
        fs::write(t.join("f.txt"), "x").unwrap();
        let log = t.join("t.log");
        fs::write(
            &log,
            format!(
                "open {}/f.txt\nopen {}\nopen {}/sub\nopen /etc/hostname\nopen relative.txt\nopen {}/f.txt\nopen {}/gone\n",
                t.display(),
                t.join("f.txt").display(),
                t.display(),
                t.display(),
                t.display()
            ),
        )
        .unwrap();
        let opens = testlog_opens(&t, &log).unwrap();
        // the dir, the outside-repo path, the relative line, the duplicate and
        // the absent file are dropped; the one regular file survives, sorted
        assert_eq!(opens, vec!["f.txt".to_string()]);
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn fs_record_parse_excludes_scratch_and_deps() {
        let t = std::env::temp_dir().join(format!("shadless-rs-verify2-{}", std::process::id()));
        fs::create_dir_all(t.join("node_modules/x")).unwrap();
        fs::create_dir_all(t.join("build")).unwrap();
        fs::write(t.join("a.js"), "").unwrap();
        fs::write(t.join("node_modules/x/i.js"), "").unwrap();
        fs::write(t.join("build/s.css"), "").unwrap();
        let log = t.join("js.log");
        fs::write(
            &log,
            format!(
                "{}/a.js\n  {}/node_modules/x/i.js  \n{}/build/s.css\n\n",
                t.display(),
                t.display(),
                t.display()
            ),
        )
        .unwrap();
        assert_eq!(fs_record_opens(&t, &log).unwrap(), vec!["a.js".to_string()]);
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn undeclared_reads_covers_the_dependency_closure() {
        let t = std::env::temp_dir().join(format!("shadless-rs-verify3-{}", std::process::id()));
        fs::create_dir_all(&t).unwrap();
        for f in ["mine.txt", "dep-in.txt", "dep-out.txt", "stray.txt"] {
            fs::write(t.join(f), "x").unwrap();
        }
        // dep: inputs [dep-in], produces [dep-out]; emit: inputs [mine], produces [] —
        // emit reads dep-out and dep-in through the closure; stray.txt is naked.
        let dep = mnode(
            "dep",
            &[],
            Some(&["dep-in.txt"]),
            Some(&["dep-out.txt"]),
        );
        let emit = mnode("emit", &["dep"], Some(&["mine.txt", "dep-out.txt"]), None);
        let g = Graph::new(vec![dep, emit.clone()]).unwrap();
        let reads = undeclared_reads(
            &t,
            &g,
            &emit,
            &[
                "mine.txt".to_string(),
                "dep-out.txt".to_string(),
                "dep-in.txt".to_string(),
                "stray.txt".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(reads, vec!["stray.txt".to_string()]);
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn undeclared_writes_needs_a_reader_to_count() {
        let t = std::env::temp_dir().join(format!("shadless-rs-verify4-{}", std::process::id()));
        fs::create_dir_all(&t).unwrap();
        fs::write(t.join("read.txt"), "v1").unwrap();
        fs::write(t.join("unwatched.txt"), "v1").unwrap();
        let reader = mnode("reader", &[], Some(&["read.txt"]), None);
        let writer = mnode("writer", &[], Some(&["other.txt"]), Some(&["declared.out"]));
        let g = Graph::new(vec![reader, writer]).unwrap();
        let writer = g.node("writer").unwrap();

        let before = input_universe(&t, &g).unwrap();
        // writer changes a watched input it does not produce, plus one it covers
        fs::write(t.join("read.txt"), "v2").unwrap();
        fs::create_dir_all(t.join("declared.out")).unwrap();
        let after = input_universe(&t, &g).unwrap();

        let vs = undeclared_writes(&t, &g, writer, &before, &after).unwrap();
        assert_eq!(vs.len(), 1);
        assert_eq!(vs[0].path, "read.txt");
        assert_eq!(vs[0].readers, vec!["reader".to_string()]);
        let _ = fs::remove_dir_all(&t);
    }
}
