//! Port of pipeline/gate_consumer_sim.go — machine-proof of the PRIMARY
//! distribution path (tools/consumer-sim.mjs).
//!
//! A consumer with only tailwindcss installed @imports shadless tokens + the
//! per-component css files they use, pastes markup carrying inline utilities,
//! and their own build emits exactly that component's styles. The scratch dir
//! is an OS temp dir — a consumer's project, not a corner of this repo — and
//! must NOT sit under a gitignored path: tailwind's automatic source
//! detection stops honouring ignore rules when the cwd itself is ignored.
//!
//! Every component stylesheet is then compiled individually a SECOND time on
//! the DECLARED FLOOR: the bottom of package.json's
//! `peerDependencies.tailwindcss` range, installed as the
//! `@tailwindcss/cli-floor` alias. That range used to be a promise nothing
//! checked — it said `^4` while dist/css used 4.1/4.3-era built-ins
//! (scrollbar-none, scrollbar-thin, wrap-break-word), so a consumer on
//! tailwindcss 4.0–4.2 got failing builds the repo never saw. The gate now
//! pins range and toolchain together and compiles on both.

use regex::Regex;
use std::path::Path;
use std::sync::{Arc, LazyLock, Mutex};

static RE_CORE_IMPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^@import[\t\n\f\r ]+("[^"]+"|url\([^)]*\));?$"#).unwrap());
static RE_FONT_MEDIUM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\.font-medium\b").unwrap());

const SIM_IMPORTED: [&str; 2] = ["button", "alert"];
const SIM_NOT_IMPORTED: [&str; 5] = ["dialog", "accordion", "select", "tooltip", "carousel"];

const CONSUMER_PAGE: &str = r#"<!doctype html>
<html><head><meta charset="utf-8"><title>consumer</title></head><body>
<button data-slot="button" data-variant="outline" class="font-medium">Continue</button>
<div data-slot="alert" data-variant="default">Account created — you are signed in.</div>
</body></html>
"#;

/// The floor CLI's tailwindcss tree, as installed: the alias package pins its
/// own tailwindcss exactly, so npm must nest the matching one inside it —
/// that nested tree is the engine the floor CLI loads, and the floor sim's
/// `node_modules/tailwindcss` symlink aims here so the core's `@import
/// "tailwindcss"` loads the same version's css (see install_consumer). The
/// version evidence is the CLI's pinned engine — read and asserted in
/// lockstep, and stamped into every output header the gate checks.
const FLOOR_CLI: &str = "node_modules/@tailwindcss/cli-floor";
const FLOOR_TW: &str = "node_modules/@tailwindcss/cli-floor/node_modules/tailwindcss";

/// X.Y.Z, and nothing looser. Prerelease and partial versions fail: the floor
/// is a version the gate compiles with, and a shape it cannot parse cannot be
/// one.
fn parse_version(v: &str, what: &str) -> Result<(u64, u64, u64), String> {
    let bad = || format!("{what}: expected X.Y.Z, got {v:?}");
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 3 {
        return Err(bad());
    }
    let mut n = [0u64; 3];
    for (i, p) in parts.iter().enumerate() {
        n[i] = p.parse().map_err(|_| bad())?;
    }
    Ok((n[0], n[1], n[2]))
}

/// package.json's peerDependencies.tailwindcss, in the one shape the floor
/// contract accepts: `^X.Y.Z`. Any other shape — `^4`, `*`, `>=4.3.0` — fails
/// the gate instead of silently meaning "no floor": the range's bottom IS the
/// version the second toolchain compiles with, and a looser shape cannot be
/// compared against it. This is the pin between the promise and the proof.
fn peer_floor(root: &Path) -> Result<(u64, u64, u64), String> {
    let text = std::fs::read_to_string(root.join("package.json"))
        .map_err(|e| format!("FAIL  consumer-sim: package.json: {e}"))?;
    let pkg: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("FAIL  consumer-sim: package.json: {e}"))?;
    let range = pkg["peerDependencies"]["tailwindcss"]
        .as_str()
        .ok_or_else(|| {
            "FAIL  consumer-sim: package.json has no peerDependencies.tailwindcss string"
                .to_string()
        })?;
    let rest = range.strip_prefix('^').ok_or_else(|| {
        format!("FAIL  consumer-sim: peerDependencies.tailwindcss must be the floor itself, \"^X.Y.Z\" — got {range:?}")
    })?;
    parse_version(rest, "peerDependencies.tailwindcss")
        .map_err(|e| format!("FAIL  consumer-sim: {e}"))
}

/// The version field of an installed package.json, read loudly.
fn installed_version(path: &Path, what: &str) -> Result<String, String> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        format!("FAIL  consumer-sim: {what} is not installed: {e} — npm install first")
    })?;
    let pkg: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("FAIL  consumer-sim: {what}: {e}"))?;
    pkg["version"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("FAIL  consumer-sim: {what}: no version field"))
}

/// The floor toolchain: the alias CLI and the tailwindcss nested inside it.
/// The CLI pins its tailwindcss exactly, so that nested tree is the engine
/// the CLI loads and lockstep holds by construction — but the compile's
/// real engine is whatever the CLI resolves internally, and this read of the
/// nested package.json is the only window the gate has on it; a future
/// release that stops nesting (floats the dep, hoists something else) breaks
/// the read or the versions part ways. Asserted, not trusted.
fn floor_toolchain(root: &Path) -> Result<String, String> {
    let cli = installed_version(&root.join(format!("{FLOOR_CLI}/package.json")), FLOOR_CLI)?;
    let tw = installed_version(&root.join(format!("{FLOOR_TW}/package.json")), FLOOR_TW)?;
    if cli != tw {
        return Err(format!(
            "FAIL  consumer-sim: the floor CLI ({FLOOR_CLI} {cli}) and the tailwindcss it resolves ({FLOOR_TW} {tw}) are not the same version — the floor is not one toolchain"
        ));
    }
    Ok(cli)
}

/// What one per-file compile pass said.
struct PerFile {
    /// names whose compile exited 0 and whose output carries the expected
    /// toolchain header
    ok: usize,
    /// names whose compile failed (or exited 0 without a readable output)
    failed: Vec<String>,
    /// names that compiled but whose output header does not name the expected
    /// version — the header says WHICH tailwind ran, and each pass exists to
    /// prove a specific one did
    wrong_header: Vec<String>,
}

/// Compiles `@import "shadless"; @import "shadless/<n>.css";` for every name,
/// each in its own entry-<n>.css/out-<n>.css pair under `sim` so parallel
/// writers don't race, bounded by a worker pool the size of the CPU count.
/// The output header must name `expect_version`.
fn compile_each(
    root: &Path,
    sim: &Path,
    cli: crate::emit::tw::TwCli,
    names: Vec<String>,
    expect_version: &str,
) -> Result<PerFile, String> {
    let failures: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let wrong: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let ok_count = Arc::new(Mutex::new(0usize));
    let queue: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(names));
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let mut handles = Vec::new();
    for _ in 0..cpus {
        let queue = Arc::clone(&queue);
        let failures = Arc::clone(&failures);
        let wrong = Arc::clone(&wrong);
        let ok_count = Arc::clone(&ok_count);
        let sim = sim.to_path_buf();
        let root = root.to_path_buf();
        let expect_version = expect_version.to_string();
        handles.push(std::thread::spawn(move || {
            loop {
                let n = queue.lock().unwrap().pop();
                let Some(n) = n else { break };
                let entry_n = sim.join(format!("entry-{}.css", n));
                let out_n = sim.join(format!("out-{}.css", n));
                let body = format!("@import \"shadless\";\n@import \"shadless/{}.css\";\n", n);
                let r = std::fs::write(&entry_n, body)
                    .map_err(|e| e.to_string())
                    .and_then(|_| {
                        crate::emit::tw::tw_compile_cli(
                            &cli,
                            &root,
                            &entry_n.to_string_lossy(),
                            &out_n.to_string_lossy(),
                            &sim.to_string_lossy(),
                            false,
                            true,
                        )
                    });
                let want = format!("tailwindcss v{}", expect_version);
                match r {
                    // the output must be there, readable, and stamped by the
                    // toolchain this pass aimed at — an exited-0 compile whose
                    // bytes say nothing counts as failed, not passed
                    Ok(()) => match std::fs::read_to_string(&out_n) {
                        Ok(out) if out.contains(&want) => *ok_count.lock().unwrap() += 1,
                        Ok(_) => wrong.lock().unwrap().push(n),
                        Err(_) => failures.lock().unwrap().push(n),
                    },
                    Err(_) => failures.lock().unwrap().push(n),
                }
            }
        }));
    }
    for h in handles {
        h.join().map_err(|_| "worker panicked".to_string())?;
    }
    let mut failed = Arc::try_unwrap(failures).unwrap().into_inner().unwrap();
    failed.sort(); // deterministic report regardless of thread finish order
    let mut wrong_header = Arc::try_unwrap(wrong).unwrap().into_inner().unwrap();
    wrong_header.sort();
    Ok(PerFile {
        ok: Arc::try_unwrap(ok_count).unwrap().into_inner().unwrap(),
        failed,
        wrong_header,
    })
}

/// Lays out one scratch consumer: `node_modules/shadless` holds a COPY of
/// the package files a CSS consumer's imports reach (package.json, so the
/// entry imports go through the REAL exports map; the core; every component
/// stylesheet), and `node_modules/tailwindcss` → `tw_target`. A copy, not a
/// symlink to this repo: tailwind resolves `@import "tailwindcss"` inside
/// the core from the core's REAL path, so a symlinked package walks up into
/// the repo's own node_modules and the floor pass would compile the floor
/// engine against the dev line's theme/preflight css (measured: the floor
/// output carried 4.3.3's preflight under a v4.3.0 header). From the copy the
/// walk-up reaches this sim's `node_modules/tailwindcss`, as in a real
/// install, so both halves of tailwind — engine and css — are `tw_target`'s.
fn install_consumer(root: &Path, sim: &Path, tw_target: &Path) -> Result<(), String> {
    let pkg = sim.join("node_modules/shadless");
    std::fs::create_dir_all(pkg.join("dist/css"))
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
    let mut files = vec![
        "package.json".to_string(),
        "dist/shadless-core.css".to_string(),
    ];
    for e in std::fs::read_dir(root.join("dist/css"))
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?
    {
        let name = e
            .map_err(|e| format!("FAIL  consumer-sim: {}", e))?
            .file_name()
            .to_string_lossy()
            .into_owned();
        if name.ends_with(".css") {
            files.push(format!("dist/css/{name}"));
        }
    }
    for f in &files {
        std::fs::copy(root.join(f), pkg.join(f))
            .map_err(|e| format!("FAIL  consumer-sim: copying {f} into the consumer: {e}"))?;
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(
            tw_target
                .canonicalize()
                .unwrap_or_else(|_| tw_target.to_path_buf()),
            sim.join("node_modules/tailwindcss"),
        )
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
    }
    #[cfg(not(unix))]
    {
        let _ = (root, sim, tw_target);
        return Err("FAIL  consumer-sim: non-unix hosts have no symlink support".to_string());
    }
    Ok(())
}

/// One consumer scratch dir under the OS temp dir, as the guard whose drop
/// deletes it; the gate holds it to its end.
fn sim_scratch(prefix: &str) -> Result<tempfile::TempDir, String> {
    tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))
}

pub fn gate_consumer_sim(root: &Path) -> Result<(), String> {
    // 0. the floor claim, checked BEFORE any compile: package.json's peer
    // range must be `^X.Y.Z` and its bottom must BE the floor toolchain. A
    // drift here reds the gate without running a single compile — which is
    // the point of pinning the promise to the proof.
    let peer = peer_floor(root)?;
    let floor_version = floor_toolchain(root)?;
    let floor_parsed = parse_version(&floor_version, "floor CLI version")
        .map_err(|e| format!("FAIL  consumer-sim: {e}"))?;
    if peer != floor_parsed {
        return Err(format!(
            "FAIL  consumer-sim: peerDependencies.tailwindcss (^{}.{}.{}) and the floor toolchain (tailwindcss {}) have drifted — raise them together",
            peer.0, peer.1, peer.2, floor_version
        ));
    }
    let dev_version = installed_version(
        &root.join("node_modules/@tailwindcss/cli/package.json"),
        "@tailwindcss/cli",
    )?;
    parse_version(&dev_version, "dev CLI version")
        .map_err(|e| format!("FAIL  consumer-sim: {e}"))?;

    let sim_dir = sim_scratch("shadless-consumer-sim-")?;
    let sim = sim_dir.path();

    // 1. core self-containment
    let core = std::fs::read_to_string(root.join("dist/shadless-core.css"))
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;
    let mut stray: Vec<String> = Vec::new();
    for m in RE_CORE_IMPORT.captures_iter(&core) {
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

    install_consumer(root, sim, &root.join("node_modules/tailwindcss"))?;

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
    std::fs::write(sim.join("page.html"), CONSUMER_PAGE)
        .map_err(|e| format!("FAIL  consumer-sim: {}", e))?;

    // 2. the consumer's own build
    crate::emit::tw::tw_compile(
        root,
        &entry.to_string_lossy(),
        &out_css.to_string_lossy(),
        &sim.to_string_lossy(),
        false,
        false,
    )
    .map_err(|e| {
        format!(
            "FAIL  consumer-sim: the consumer's build did not compile: {}",
            e
        )
    })?;
    let out =
        std::fs::read_to_string(&out_css).map_err(|e| format!("FAIL  consumer-sim: {}", e))?;

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
    if !RE_FONT_MEDIUM.is_match(&out) {
        problems
            .push("inline utility from the consumer page (font-medium) not emitted".to_string());
    }
    // 6. theme variables
    if !out.contains("--background:") || !out.contains(".dark") {
        problems
            .push("theme variables / .dark override missing from the consumer build".to_string());
    }
    // 7. size sanity — a full leak lands in the hundreds of KB
    let kb = (out.len() + 512) / 1024;
    if out.len() > 80 * 1024 {
        problems.push(format!(
            "consumer build is {}KB — the whole library leaked in (expected a couple dozen KB)",
            kb
        ));
    }

    // 8. EVERY per-component stylesheet compiles individually with the core —
    // on the dev toolchain first, then again on the floor in its own scratch
    // consumer (same layout, tailwindcss resolves to the floor's nested tree).
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
    let total = to_compile.len();

    let dev_run = compile_each(
        root,
        sim,
        crate::emit::tw::TwCli::dev(),
        to_compile.clone(),
        &dev_version,
    )?;
    if !dev_run.failed.is_empty() {
        problems.push(format!(
            "components that do NOT compile individually with the core: {}",
            dev_run.failed.join(", ")
        ));
    }
    if !dev_run.wrong_header.is_empty() {
        problems.push(format!(
            "components that compiled without the @tailwindcss/cli {} header — a different compiler ran: {}",
            dev_version,
            dev_run.wrong_header.join(", ")
        ));
    }

    let fsim_dir = sim_scratch("shadless-consumer-floor-sim-")?;
    let fsim = fsim_dir.path();
    install_consumer(root, fsim, &root.join(FLOOR_TW))?;
    let floor_run = compile_each(
        root,
        fsim,
        crate::emit::tw::TwCli::floor(),
        to_compile,
        &floor_version,
    )?;
    if !floor_run.failed.is_empty() {
        problems.push(format!(
            "components that do NOT compile on the declared floor (tailwindcss {}): {}",
            floor_version,
            floor_run.failed.join(", ")
        ));
    }
    if !floor_run.wrong_header.is_empty() {
        problems.push(format!(
            "components that compiled without the floor tailwindcss {} header — the floor compiler did not run: {}",
            floor_version,
            floor_run.wrong_header.join(", ")
        ));
    }

    if !problems.is_empty() {
        return Err(format!("FAIL  consumer-sim\n  {}", problems.join("\n  ")));
    }
    println!(
        "PASS  consumer-sim ({} components imported, tree-shaking intact, {}KB build, core self-contained, compile individually {}/{} on dev tailwindcss {} and {}/{} on the floor tailwindcss {} — peer range pinned to the floor)",
        SIM_IMPORTED.len(),
        kb,
        dev_run.ok,
        total,
        dev_version,
        floor_run.ok,
        total,
        floor_version,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Go TestConsumerSim: gate(t, gateConsumerSim). Spawns one tailwindcss
    /// subprocess per dist/css/*.css — slow, but that is what the Go test
    /// does too.
    #[test]
    fn consumer_sim_on_real_tree() {
        let root = crate::tree_root();
        if !root.join("dist/shadless-core.css").exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: product css not built)");
            }
            eprintln!("skip: product css not built");
            return;
        }
        gate_consumer_sim(&root).expect("consumer-sim gate must pass");
    }

    /// The peer-range parser is the pin between package.json and the floor
    /// toolchain — a shape it silently mis-parses would un-pin them. Both the
    /// grammar and the `^`-only rule are pinned here, including the shapes
    /// that actually shipped or are tempting next.
    #[test]
    fn unit_peer_floor_shape() {
        assert_eq!(parse_version("4.3.0", "v").unwrap(), (4, 3, 0));
        for bad in ["4.3", "4", "", "4.3.0-beta.1", "x.y.z", "4.3.0.1"] {
            assert!(parse_version(bad, "v").is_err(), "{bad:?} must not parse");
        }
        let dir = tempfile::Builder::new()
            .prefix("shadless-peer-shape-")
            .tempdir()
            .unwrap();
        let write = |range: &str| {
            std::fs::write(
                dir.path().join("package.json"),
                format!(r#"{{"peerDependencies":{{"tailwindcss":"{range}"}}}}"#),
            )
            .unwrap();
        };
        write("^4.3.0");
        assert_eq!(peer_floor(dir.path()).unwrap(), (4, 3, 0));
        // every non-`^X.Y.Z` shape fails loudly — "^4" is the one that shipped
        for range in ["^4", "^4.3", ">=4.3.0", "*", "~4.3.0", "4.3.0", "latest"] {
            write(range);
            assert!(
                peer_floor(dir.path()).is_err(),
                "peer range {range:?} must fail the gate, not mean \"no floor\""
            );
        }
    }

    /// Every gate run made two scratch consumers with `.tempdir()?.keep()`,
    /// which disables the delete — a green run left ~3MB in the temp dir.
    #[test]
    fn unit_sim_scratch_is_deleted_when_its_guard_drops() {
        let guard = sim_scratch("shadless-sim-scratch-test-").unwrap();
        let path = guard.path().to_path_buf();
        assert!(
            path.is_dir(),
            "the scratch dir exists while the guard is held"
        );
        drop(guard);
        assert!(
            !path.exists(),
            "{} survived its guard: the scratch dir leaks",
            path.display()
        );
    }
}
