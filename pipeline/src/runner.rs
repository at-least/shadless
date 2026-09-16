//! Port of pipeline/run.go — parallel execution.
//!
//! Independent nodes run concurrently; a node is dispatched the moment every
//! node it `needs` has finished. Its key is computed at dispatch time, not up
//! front, because the key folds in its dependencies' keys and those are only
//! final once they have run.
//!
//! Shape in Rust: the dispatch loop stays on the calling thread (as in Go —
//! one place makes the fresh/stop decisions); each running node gets a worker
//! thread, bounded by two counting semaphores. The browser token is acquired
//! BEFORE the job slot, so a browser node waiting for its turn never occupies
//! a worker slot a fast node could use.
//!
//! The undeclared-write check compares the input universe before and after a
//! node's commands, which only means anything when nothing else is writing at
//! the same time. It is therefore enforced at -j1 and skipped above it, with a
//! note, rather than silently reporting another node's writes.

use crate::graph::Graph;
use crate::key::{outputs_present, stamp_value, Keyer};
use crate::nodes::Node;
use crate::stamps::{remove_stamp, stamp_file, write_stamp};
use crate::verify::{
    input_universe, opens_from_logs, report_undeclared_reads, report_violations,
    undeclared_reads, undeclared_writes, Violation,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::io;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Instant;

/// The one input every browser node already has to declare: a node that drives
/// tools/browser-shell.mjs without listing it would go falsely fresh on a
/// shell change. The declaration doubles as the identity — there is no second
/// list of browser nodes to forget to update.
pub const SHELL_INPUT: &str = "tools/browser-shell.mjs";

/// The JS half of the read evidence. NODE_OPTIONS is inherited by child
/// processes, so one variable covers a tool and everything it spawns —
/// including the tailwind CLI, which is itself a node script.
pub const FS_RECORDER: &str = "tools/fs-record.mjs";

/// Counting semaphore with Go `chan struct{}` semantics.
pub struct Sem {
    remain: Mutex<usize>,
    cv: Condvar,
}

impl Sem {
    pub fn new(n: usize) -> Arc<Sem> {
        Arc::new(Sem {
            remain: Mutex::new(n.max(1)),
            cv: Condvar::new(),
        })
    }
    pub fn acquire(&self) {
        let mut g = self.remain.lock().unwrap();
        while *g == 0 {
            g = self.cv.wait(g).unwrap();
        }
        *g -= 1;
    }
    pub fn release(&self) {
        *self.remain.lock().unwrap() += 1;
        self.cv.notify_one();
    }
}

pub fn is_go_test(argv: &[String]) -> bool {
    argv.len() >= 2 && argv[0] == "go" && argv[1] == "test"
}

pub fn is_browser_node(n: &Node) -> bool {
    n.inputs
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .any(|p| p == SHELL_INPUT)
}

/// tailOf: the last 25 non-empty lines — enough for the re-pin drill to
/// attribute a failure to a component, short enough to paste into a report.
pub fn tail_of(out: &[u8]) -> String {
    let text = String::from_utf8_lossy(out);
    let lines: Vec<&str> = text
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .collect();
    let start = lines.len().saturating_sub(25);
    lines[start..].join("\n")
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FailedNode {
    pub cmd: String,
    pub tail: String,
}

/// Mirrors the shape gates/run.mjs wrote, because gates/upstream.mjs reads it
/// as data. BTreeMap keeps keys sorted, as Go's json.Marshal does for maps.
#[derive(Serialize, Deserialize, Default)]
pub struct RunReport {
    pub failed: BTreeMap<String, FailedNode>,
    pub blocked: Vec<String>,
    pub passed: Vec<String>,
}

struct Result_ {
    node: Node,
    err: Option<String>,
    output: Vec<u8>,
    violations: Vec<Violation>,
    reads: Vec<String>,
    elapsed: f64,
}

pub struct Counts {
    pub ran: usize,
    pub skipped: usize,
    pub failed: usize,
    pub violations: usize,
    pub bad_reads: usize,
}

pub struct Runner {
    pub root: PathBuf,
    pub graph: Arc<Graph>,
    pub jobs: usize,
    pub browser_jobs: usize, // cap on concurrent Chromium-launching nodes, ≤ jobs
    pub force: bool,
    pub continue_on_fail: bool,
    stamps: Mutex<HashMap<String, String>>,
    report: Mutex<RunReport>,
}

struct RunState<'a> {
    by_id: HashMap<String, &'a Node>,
    pending: HashMap<String, usize>,
    dependents: HashMap<String, Vec<String>>,
    done: HashSet<String>,
    skipped: usize,
    ready: VecDeque<String>,
    inflight: usize,
    stop: bool,
    /// dispatch-time key computations that failed. They are failures —
    /// the run verdict must not go green over a plan that did nothing —
    /// but unlike command failures they have no output tail to show.
    key_errors: usize,
    tx: std::sync::mpsc::Sender<Result_>,
    sem: Arc<Sem>,
    browser_sem: Arc<Sem>,
}

impl Runner {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        root: PathBuf,
        graph: Arc<Graph>,
        jobs: usize,
        browser_jobs: usize,
        force: bool,
        continue_on_fail: bool,
        stamps: HashMap<String, String>,
    ) -> Self {
        Runner {
            root,
            graph,
            jobs,
            browser_jobs,
            force,
            continue_on_fail,
            stamps: Mutex::new(stamps),
            report: Mutex::new(RunReport::default()),
        }
    }

    fn record(&self, id: &str, key: String) {
        self.stamps.lock().unwrap().insert(id.to_string(), key.clone());
        let _ = write_stamp(&self.root, id, &key); // one file per node: parallel nodes do not contend
    }

    fn forget(&self, id: &str) {
        self.stamps.lock().unwrap().remove(id);
        remove_stamp(&self.root, id);
    }

    fn recorded(&self, id: &str) -> String {
        self.stamps
            .lock()
            .unwrap()
            .get(id)
            .cloned()
            .unwrap_or_default()
    }

    /// Runs one node's commands, capturing output so parallel logs stay
    /// readable: a node's output is printed as one block when it finishes.
    ///
    /// Every `go test` command additionally gets -test.testlogfile (absolute —
    /// these commands carry `-C pipeline`, and a relative one would land inside
    /// the package directory); every command runs with the NODE_OPTIONS JS
    /// recorder injected. Cosmetics note: Go interleaves both streams into one
    /// buffer; Rust appends stdout-then-stderr per command.
    fn exec(&self, n: &Node) -> (Vec<u8>, Vec<PathBuf>, Option<String>) {
        let mut buf: Vec<u8> = Vec::new();
        let mut logs: Vec<PathBuf> = Vec::new();
        // only hold the scratch dir while there is evidence to collect
        let dir = tempfile::Builder::new()
            .prefix("pipeline-access-")
            .tempdir()
            .ok();
        let js_log = dir.as_ref().map(|d| d.path().join("js.log"));
        for (i, argv) in n.run.iter().enumerate() {
            let mut cmd = argv.clone();
            if let Some(d) = &dir {
                if is_go_test(argv) {
                    let log = d.path().join(format!("{}-{}.log", stamp_file(&n.id), i));
                    cmd.push(format!("-test.testlogfile={}", log.display()));
                    logs.push(log);
                }
            }
            let exe = match crate::engine::resolve_argv0(&cmd[0]) {
                Ok(p) => p,
                Err(e) => {
                    let msg = format!("fork/exec {}: {}", cmd[0], e);
                    return (buf, logs, Some(msg));
                }
            };
            let mut c = Command::new(exe);
            c.args(&cmd[1..]).current_dir(&self.root);
            if let Some(jl) = &js_log {
                let node_opts = format!(
                    "{} --import {}",
                    std::env::var("NODE_OPTIONS").unwrap_or_default().trim(),
                    self.root.join(FS_RECORDER).display()
                );
                c.env("SHADLESS_FSLOG", jl);
                c.env("NODE_OPTIONS", node_opts);
            }
            match c.output() {
                Ok(o) => {
                    buf.extend_from_slice(&o.stdout);
                    buf.extend_from_slice(&o.stderr);
                    if !o.status.success() {
                        // Go's exec.ExitError prints as "exit status N"
                        let e = match o.status.code() {
                            Some(code) => format!("exit status {}", code),
                            None => "signal: killed".to_string(),
                        };
                        return (buf, logs, Some(e));
                    }
                }
                Err(e) => {
                    let msg = if e.kind() == io::ErrorKind::NotFound {
                        format!("fork/exec {}: no such file or directory", cmd[0])
                    } else {
                        e.to_string()
                    };
                    return (buf, logs, Some(msg));
                }
            }
        }
        (buf, logs, None)
    }

    /// Collects the undeclared reads across a node's testlogs, then removes the
    /// temp directory holding them (dropped TempDir does that).
    fn reads_from(&self, n: &Node, logs: &[PathBuf]) -> Vec<String> {
        if logs.is_empty() {
            return Vec::new();
        }
        let all = opens_from_logs(&self.root, logs);
        undeclared_reads(&self.root, &self.graph, n, &all).unwrap_or_default()
    }

    fn run_one(&self, n: &Node, key: &Option<String>, jobs1: bool) -> Result_ {
        let start = Instant::now();
        let before = if jobs1 {
            input_universe(&self.root, &self.graph).ok()
        } else {
            None
        };
        let (out, logs, err) = self.exec(n);
        // computed even for a failed node: a gate that went red still read what
        // it read, and the declaration is wrong either way
        let reads = self.reads_from(n, &logs);
        if let Some(e) = err {
            self.forget(&n.id); // a failed node stays stale; it claims nothing
            return Result_ {
                node: n.clone(),
                err: Some(e),
                output: out,
                violations: Vec::new(),
                reads,
                elapsed: start.elapsed().as_secs_f64(),
            };
        }
        let mut violations = Vec::new();
        if jobs1 {
            if let (Some(before), Ok(after)) =
                (before, input_universe(&self.root, &self.graph))
            {
                violations = undeclared_writes(&self.root, &self.graph, n, &before, &after)
                    .unwrap_or_default();
            }
        }
        if let Some(key) = key {
            // recompute after the run: a node whose own output feeds its key
            // would otherwise be stamped with a key it no longer has
            match Keyer::new(&self.root, &self.graph).key(&n.id) {
                Ok(Some(after)) => self.record(&n.id, stamp_value(&self.root, n, &after)),
                _ => self.record(&n.id, stamp_value(&self.root, n, key)),
            }
        }
        Result_ {
            node: n.clone(),
            err: None,
            output: out,
            violations,
            reads,
            elapsed: start.elapsed().as_secs_f64(),
        }
    }

    fn dispatch(
        self: &Arc<Self>,
        st: &mut RunState<'_>,
        id: &str,
    ) {
        let n = st.by_id[id].clone();
        // the key must be computed here: every dependency has finished, so
        // their stamps are final
        let key = match Keyer::new(&self.root, &self.graph).key(id) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("pipeline: {}", e);
                // a node that cannot be keyed is a failed node: it goes in
                // the report (so the blocked list does not absorb it) and
                // the run verdict counts it, instead of stopping with every
                // count at zero and exit 0
                self.report.lock().unwrap().failed.insert(
                    id.to_string(),
                    FailedNode {
                        cmd: "(key)".to_string(),
                        tail: e,
                    },
                );
                st.key_errors += 1;
                st.stop = true;
                return;
            }
        };
        // the recorded stamp carries the output digest too: a node whose own
        // output was edited under it is stale, however the edit got there
        let mut fresh = match &key {
            Some(k) => self.recorded(id) == stamp_value(&self.root, &n, k) && !self.force,
            None => false,
        };
        if fresh {
            if let Err(missing) = outputs_present(&self.root, &n) {
                println!("  {}: key matches but {} is missing — rebuilding", id, missing);
                fresh = false;
            }
        }
        if fresh {
            st.skipped += 1;
            st.done.insert(id.to_string());
            if let Some(ds) = st.dependents.get(id) {
                for d in ds.clone() {
                    if let Some(e) = st.pending.get_mut(&d) {
                        *e -= 1;
                        if *e == 0 {
                            st.ready.push_back(d);
                        }
                    }
                }
            }
            return;
        }
        st.inflight += 1;
        let runner = Arc::clone(self);
        let tx = st.tx.clone();
        let sem = Arc::clone(&st.sem);
        let browser_sem = Arc::clone(&st.browser_sem);
        let jobs1 = self.jobs == 1;
        std::thread::spawn(move || {
            if is_browser_node(&n) {
                // the browser token comes BEFORE the main slot: a browser node
                // waiting for its turn must not occupy a worker slot that a
                // fast node could be running in
                browser_sem.acquire();
            }
            sem.acquire();
            let res = runner.run_one(&n, &key, jobs1);
            let _ = tx.send(res);
            // Go's defers run LIFO: the job slot is handed back first, then the
            // browser token (which only a browser node ever took)
            sem.release();
            if is_browser_node(&n) {
                browser_sem.release();
            }
        });
    }

/// Dispatches the plan, honouring `needs`, and returns how many nodes ran,
/// were skipped, and failed, plus the two declaration violations seen:
/// writes outside `produces` and reads outside `inputs`.
    pub fn run(self: &Arc<Self>, plan: &[Node]) -> Counts {
        let in_plan: HashSet<&str> = plan.iter().map(|n| n.id.as_str()).collect();
        let mut pending: HashMap<String, usize> = HashMap::new();
        let mut dependents: HashMap<String, Vec<String>> = HashMap::new();
        for n in plan {
            for d in &n.needs {
                if in_plan.contains(d.as_str()) {
                    *pending.entry(n.id.clone()).or_default() += 1;
                    dependents
                        .entry(d.clone())
                        .or_default()
                        .push(n.id.clone());
                }
            }
        }
        let by_id: HashMap<String, &Node> =
            plan.iter().map(|n| (n.id.clone(), n)).collect();
        let ready: VecDeque<String> = plan
            .iter()
            .filter(|n| pending.get(&n.id).copied().unwrap_or(0) == 0)
            .map(|n| n.id.clone())
            .collect();
        let (tx, rx) = std::sync::mpsc::channel::<Result_>();
        let mut st = RunState {
            by_id,
            pending,
            dependents,
            done: HashSet::new(),
            skipped: 0,
            ready,
            inflight: 0,
            stop: false,
            key_errors: 0,
            tx,
            sem: Sem::new(self.jobs),
            browser_sem: Sem::new(self.browser_jobs.max(1)),
        };

        let mut ran = 0;
        let mut failed = 0;
        let mut violations = 0;
        let mut bad_reads = 0;
        loop {
            while !st.stop {
                let Some(id) = st.ready.pop_front() else {
                    break;
                };
                self.dispatch(&mut st, &id);
            }
            if st.inflight == 0 {
                break;
            }
            let Ok(res) = rx.recv() else {
                break;
            };
            st.inflight -= 1;
            let id = res.node.id.clone();
            let status = if res.err.is_some() { "✗" } else { "✔" };
            println!("{} {} ({:.1}s)", status, id, res.elapsed);
            if !res.output.is_empty() && (res.err.is_some() || std::env::var("PIPELINE_VERBOSE").map(|v| !v.is_empty()).unwrap_or(false)) {
                let mut so = std::io::stdout().lock();
                let _ = so.write_all(&res.output);
                let _ = so.flush();
            }
            report_violations(&id, &res.violations);
            violations += res.violations.len();
            report_undeclared_reads(&id, &res.reads);
            bad_reads += res.reads.len();
            if let Some(e) = &res.err {
                failed += 1;
                let cmd = res
                    .node
                    .run
                    .last()
                    .map(|c| c.join(" "))
                    .unwrap_or_default();
                self.report.lock().unwrap().failed.insert(
                    id.clone(),
                    FailedNode {
                        cmd,
                        tail: tail_of(&res.output),
                    },
                );
                // The `why` is the point of the report: a red gate is only
                // actionable if you know what it was protecting.
                eprintln!("\nFAIL  {}: {}", id, e);
                if !res.node.why.is_empty() {
                    eprintln!("\n  why this node exists:\n    {}", res.node.why);
                }
                eprintln!(
                    "\n  reproduce just this node (rebuilds only what it needs):\n    pipeline run {}",
                    id
                );
                for argv in &res.node.run {
                    eprintln!("  run the command alone:\n    {}", argv.join(" "));
                }
                if !self.continue_on_fail {
                    st.stop = true;
                }
                continue; // dependents never become ready: their pending count never reaches 0
            }
            ran += 1;
            st.done.insert(id.clone());
            self.report
                .lock()
                .unwrap()
                .passed
                .push(id.clone());
            if let Some(ds) = st.dependents.get(&id).cloned() {
                for d in ds {
                    if let Some(e) = st.pending.get_mut(&d) {
                        *e -= 1;
                        if *e == 0 && !st.stop {
                            st.ready.push_back(d);
                        }
                    }
                }
            }
        }
        // whatever never ran and never skipped was blocked by a failed dependency
        let mut report = self.report.lock().unwrap();
        for n in plan {
            if !st.done.contains(&n.id) && !report.failed.contains_key(&n.id) {
                report.blocked.push(n.id.clone());
            }
        }
        let blocked = report.blocked.len();
        drop(report);
        if blocked > 0 {
            eprintln!("{} node(s) not reached (a dependency failed)", blocked);
        }
        Counts {
            ran,
            skipped: st.skipped,
            failed: failed + st.key_errors,
            violations,
            bad_reads,
        }
    }

    /// Persists the run report. Only --keep-going asks for it: a run that stops
    /// at the first red has nothing to classify.
    ///
    /// Byte-parity note: Go's json.MarshalIndent HTML-escapes < > & inside
    /// strings; serde_json does not, so the same escaping is applied to the
    /// serialized text (those characters cannot occur in JSON syntax, only in
    /// string content, so the substitution is safe).
    pub fn write_report(&self) -> Result<(), String> {
        let mut report = self.report.lock().unwrap();
        if report.blocked.is_empty() {
            report.blocked = Vec::new(); // serialize as [] not null
        }
        if report.passed.is_empty() {
            report.passed = Vec::new();
        }
        let dir = self.root.join("build").join("gates");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let mut b = serde_json::to_string_pretty(&*report).map_err(|e| e.to_string())?;
        b = b.replace('<', "\\u003c").replace('>', "\\u003e").replace('&', "\\u0026");
        b.push('\n');
        std::fs::write(dir.join("run-report.json"), b).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::Node as N;
    use std::fs;
    use std::path::Path;

    /// A browser node whose command takes the exclusive lock for 150ms: with a
    /// working browser cap the two serialize and both succeed; an inflated cap
    /// (the inverse of the drop-instead-of-release bug) lets them overlap and
    /// the losing command fails its lock test.
    fn browser_node(id: &str, lock: &Path) -> N {
        let script = format!(
            "test ! -e {} && touch {} && sleep 0.15 && rm -f {}",
            lock.display(),
            lock.display(),
            lock.display()
        );
        N {
            id: id.to_string(),
            kind: "build".to_string(),
            tier: "fast".to_string(),
            needs: vec![],
            run: vec![vec![
                "sh".to_string(),
                "-c".to_string(),
                script,
            ]],
            // the shell-input declaration is what makes it a browser node
            inputs: Some(vec![SHELL_INPUT.to_string()]),
            produces: None,
            why: String::new(),
            mutations: vec![],
        }
    }

    #[test]
    fn browser_cap_serializes_browser_nodes() {
        // no parens in the path: the lock path is embedded in a `sh -c` script,
        // where parentheses are syntax characters
        let t = std::env::temp_dir().join(format!(
            "shadless-rs-browser-{}-ba-bb",
            std::process::id()
        ));
        fs::create_dir_all(&t).unwrap();
        let lock = t.join("browser.lock");
        let a = browser_node("ba", &lock);
        let mut b = browser_node("bb", &lock);
        b.needs = vec![]; // independent: both ready at once
        let g = Arc::new(Graph::new(vec![a.clone(), b.clone()]).unwrap());
        assert!(is_browser_node(&a) && is_browser_node(&b));
        let runner = Arc::new(Runner::new(
            t.clone(),
            Arc::clone(&g),
            2, // two job slots: only the browser cap (1) can stop the overlap
            1,
            false,
            false,
            HashMap::new(),
        ));
        let counts = runner.run(&[a, b]);
        assert_eq!(counts.ran, 2, "both browser nodes must succeed");
        assert_eq!(counts.failed, 0);
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn key_computation_failure_is_a_reported_failure_not_a_silent_stop() {
        // A key error (here: a non-ASCII byte in a glob pattern, which
        // quote_meta_byte rejects) used to stop the run with every count at
        // zero — "ran 0, skipped 0" and exit 0, a green verdict over a run
        // that did nothing. It must count as a failure and land in the
        // run report.
        let t = std::env::temp_dir().join(format!("shadless-rs-keyerr-{}", std::process::id()));
        fs::create_dir_all(&t).unwrap();
        let a = N {
            id: "a".to_string(),
            kind: "build".to_string(),
            tier: "fast".to_string(),
            needs: vec![],
            run: vec![vec!["true".to_string()]],
            inputs: Some(vec!["\u{e9}*".to_string()]), // é*: non-ASCII pattern byte
            produces: None,
            why: String::new(),
            mutations: vec![],
        };
        let g = Arc::new(Graph::new(vec![a.clone()]).unwrap());
        let runner = Arc::new(Runner::new(
            t.clone(),
            Arc::clone(&g),
            1,
            1,
            false,
            false,
            HashMap::new(),
        ));
        let counts = runner.run(&[a]);
        assert_eq!(counts.failed, 1, "a key computation failure is a failure");
        let report = runner.report.lock().unwrap();
        let f = report
            .failed
            .get("a")
            .expect("key failure reported as a failed node");
        assert_eq!(f.cmd, "(key)");
        assert!(!f.tail.is_empty());
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn sem_release_is_explicit_not_drop() {
        // regression guard for the drop(Arc)-instead-of-release() deadlock:
        // one slot, two waiters, both must get through
        let sem = Sem::new(1);
        let entered = Arc::new(Mutex::new(0usize));
        let handles: Vec<_> = (0..2)
            .map(|_| {
                let sem = Arc::clone(&sem);
                let entered = Arc::clone(&entered);
                std::thread::spawn(move || {
                    sem.acquire();
                    *entered.lock().unwrap() += 1;
                    sem.release();
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(*entered.lock().unwrap(), 2);
        // the semaphore itself must be back at full capacity
        sem.acquire();
        assert_eq!(*sem.remain.lock().unwrap(), 0);
    }
}
