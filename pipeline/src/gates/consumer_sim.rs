//! Port of pipeline/gate_consumer_sim.go — machine-proof of the PRIMARY
//! distribution path (tools/consumer-sim.mjs).
//!
//! A consumer with only tailwindcss installed @imports shadless tokens + the
//! per-component css files they use, pastes markup carrying inline utilities,
//! and their own build emits exactly that component's styles. The scratch dir
//! is an OS temp dir — a consumer's project, not a corner of this repo — and
//! must NOT sit under a gitignored path: tailwind's automatic source
//! detection stops honouring ignore rules when the cwd itself is ignored.

use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

fn re_core_import() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"(?m)^@import[\t\n\f\r ]+("[^"]+"|url\([^)]*\));?$"#).unwrap())
}
fn re_font_medium() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\.font-medium\b").unwrap())
}

const SIM_IMPORTED: [&str; 2] = ["button", "alert"];
const SIM_NOT_IMPORTED: [&str; 5] = ["dialog", "accordion", "select", "tooltip", "carousel"];

const CONSUMER_PAGE: &str = r#"<!doctype html>
<html><head><meta charset="utf-8"><title>consumer</title></head><body>
<button data-slot="button" data-variant="outline" class="font-medium">Continue</button>
<div data-slot="alert" data-variant="default">Account created — you are signed in.</div>
</body></html>
"#;

pub fn gate_consumer_sim(root: &Path) -> Result<(), String> {
    let sim = tempfile::Builder::new()
        .prefix("shadless-consumer-sim-")
        .tempdir()
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?
        .keep(); // kept alive through the gate; removed at return
    let sim: PathBuf = sim;

    // 1. core self-containment
    let core = std::fs::read_to_string(root.join("dist/shadless-core.css"))
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
    let mut stray: Vec<String> = Vec::new();
    for m in re_core_import().captures_iter(&core) {
        if &m[1] != "\"tailwindcss\"" {
            stray.push(m[1].to_string());
        }
    }
    if !stray.is_empty() {
        return Err(format!(
            "FAIL  consumer-sim: shadless-core.css is not self-contained — @import {} would need an extra package",
            stray.join(", ")
        ));
    }

    // install the package the way a consumer would have it: node_modules/
    // shadless → this repo. The entry then imports through the REAL
    // package.json exports map.
    std::fs::create_dir_all(sim.join("node_modules"))
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root.canonicalize().unwrap_or(root.to_path_buf()), sim.join("node_modules/shadless"))
            .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
        std::os::unix::fs::symlink(
            root.join("node_modules/tailwindcss").canonicalize().unwrap_or_else(|_| root.join("node_modules/tailwindcss")),
            sim.join("node_modules/tailwindcss"),
        )
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
    }

    let entry = sim.join("entry.css");
    let out_css = sim.join("out.css");
    let write_entry = |entry: &Path, imports: &[&str]| -> Result<(), String> {
        let mut body = String::from("@import \"shadless\";\n");
        for n in imports {
            body += &format!("@import \"shadless/{}.css\";\n", n);
        }
        std::fs::write(entry, body).map_err(|e| e.to_string())
    };
    write_entry(&entry, &SIM_IMPORTED).map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
    // the consumer's page: shadless markup (data-slots + inline utilities)
    std::fs::write(sim.join("page.html"), CONSUMER_PAGE).map_err(|e| format!("FAIL  consumer-sim: {}", e))?;

    // 2. the consumer's own build
    crate::emit::tw::tw_compile(
        root,
        &entry.to_string_lossy(),
        &out_css.to_string_lossy(),
        &sim.to_string_lossy(),
        false,
        false,
    )
    .map_err(|e| format!("FAIL  consumer-sim: the consumer's build did not compile: {}", e))?;
    let out = std::fs::read_to_string(&out_css)
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;

    let mut problems: Vec<String> = Vec::new();
    // 3. imported slot rules present
    for n in SIM_IMPORTED {
        if !out.contains(&format!("[data-slot=\"{}\"]", n)) {
            problems.push(format!(
                "imported component {}: no slot rule in the consumer build",
                n
            ));
        }
    }
    // 4. nothing from non-imported components
    for n in SIM_NOT_IMPORTED {
        if out.contains(&format!("[data-slot=\"{}\"]", n)) {
            problems.push(format!(
                "tree-shaking broken: {} rules leaked into a build that never imported it",
                n
            ));
        }
    }
    // 5. inline utilities from the consumer page
    if !re_font_medium().is_match(&out) {
        problems.push(
            "inline utility from the consumer page (font-medium) not emitted".to_string(),
        );
    }
    // 6. theme variables
    if !out.contains("--background:") || !out.contains(".dark") {
        problems.push(
            "theme variables / .dark override missing from the consumer build".to_string(),
        );
    }
    // 7. size sanity — a full leak lands in the hundreds of KB
    let kb = (out.len() + 512) / 1024;
    if out.len() > 80 * 1024 {
        problems.push(format!(
            "consumer build is {}KB — the whole library leaked in (expected a couple dozen KB)",
            kb
        ));
    }

    // 8. EVERY per-component stylesheet compiles individually with the core.
    // Each name gets its own entry-<name>.css/out-<name>.css so parallel
    // writers don't race; bounded by a worker pool the size of the CPU count.
    let names = std::fs::read_dir(root.join("dist/css"))
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
    let mut to_compile: Vec<String> = Vec::new();
    for e in names.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if let Some(stem) = n.strip_suffix(".css") {
            to_compile.push(stem.to_string());
        }
    }
    to_compile.sort();

    let failures: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let ok_count = Arc::new(Mutex::new(0usize));
    let queue: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(to_compile));
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let mut handles = Vec::new();
    for _ in 0..cpus {
        let queue = Arc::clone(&queue);
        let failures = Arc::clone(&failures);
        let ok_count = Arc::clone(&ok_count);
        let sim = sim.clone();
        let root = root.to_path_buf();
        handles.push(std::thread::spawn(move || loop {
            let n = queue.lock().unwrap().pop();
            let Some(n) = n else { break };
            let entry_n = sim.join(format!("entry-{}.css", n));
            let out_n = sim.join(format!("out-{}.css", n));
            let body = format!("@import \"shadless\";\n@import \"shadless/{}.css\";\n", n);
            let r = std::fs::write(&entry_n, body).map_err(|e| e.to_string()).and_then(|_| {
                crate::emit::tw::tw_compile(
                    &root,
                    &entry_n.to_string_lossy(),
                    &out_n.to_string_lossy(),
                    &sim.to_string_lossy(),
                    false,
                    true,
                )
            });
            match r {
                Ok(()) => *ok_count.lock().unwrap() += 1,
                Err(_) => failures.lock().unwrap().push(n),
            }
        }));
    }
    for h in handles {
        h.join().map_err(|_| "worker panicked".to_string())?;
    }
    let mut individual_fail = Arc::try_unwrap(failures).unwrap().into_inner().unwrap();
    individual_fail.sort(); // deterministic report regardless of thread finish order
    let individual_ok = Arc::try_unwrap(ok_count).unwrap().into_inner().unwrap();
    if !individual_fail.is_empty() {
        problems.push(format!(
            "components that do NOT compile individually with the core: {}",
            individual_fail.join(", ")
        ));
    }

    if !problems.is_empty() {
        return Err(format!("FAIL  consumer-sim\n  {}", problems.join("\n  ")));
    }
    println!(
        "PASS  consumer-sim ({} components imported, tree-shaking intact, {}KB build, core self-contained, {}/{} components compile individually)",
        SIM_IMPORTED.len(),
        kb,
        individual_ok,
        individual_ok + individual_fail.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real_root() -> std::path::PathBuf {
        match std::env::var("SHADLESS_ROOT") {
            Ok(r) => std::path::PathBuf::from(r),
            Err(_) => {
                let m = crate::crate_adjacent_tree_root()
                    .unwrap_or(Path::new(env!("CARGO_MANIFEST_DIR")).join(".."));
                m
            }
        }
    }

    /// Go TestConsumerSim: gate(t, gateConsumerSim). Spawns one tailwindcss
    /// subprocess per dist/css/*.css — slow, but that is what the Go test
    /// does too.
    #[test]
    fn consumer_sim_on_real_tree() {
        let root = real_root();
        if !root.join("dist/shadless-core.css").exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: product css not built)");
            }
            eprintln!("skip: product css not built");
            return;
        }
        gate_consumer_sim(&root).expect("consumer-sim gate must pass");
    }
}
