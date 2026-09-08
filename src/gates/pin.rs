//! Port of pipeline/gate_pin.go — record and verify the upstream pins (the
//! shadcn-ui clone, the vendored kernel IIFE).
//!
//! The gate is `run_pin(root, true, false)` — verify only, record nothing.

use regex::Regex;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub const SHADCN_DIR: &str = ".upstream/shadcn-ui";
pub const SHADCN_REPO: &str = "https://github.com/shadcn-ui/ui";
pub const KERNEL_IIFE: &str = "vendor/radix-kernel.iife.js";
pub const PIN_FILE_PATH: &str = "src/registry/pin.json";

fn release_tag_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^shadcn@\d").unwrap())
}

fn registry_base_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"registry/bases/([^/]+)/").unwrap())
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct PinShadcnUi {
    #[serde(default)]
    pub repo: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub commit: String,
    #[serde(default)]
    pub registry: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct PinKernel {
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub sha256: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct PinFile {
    #[serde(rename = "shadcn_ui", default)]
    pub shadcn_ui: PinShadcnUi,
    #[serde(default)]
    pub kernel: PinKernel,
    #[serde(default)]
    pub recorded: String,
}

pub fn pinned_base(p: &PinFile) -> Result<String, String> {
    match registry_base_re().captures(&p.shadcn_ui.registry) {
        Some(m) => Ok(m[1].to_string()),
        None => Err(format!(
            "pin.json `shadcn_ui.registry` is {:?}, not a path of the form apps/v4/registry/bases/<base>/ui",
            p.shadcn_ui.registry
        )),
    }
}

/// Verifies that the base pin.json names exists upstream and is the one the
/// graph actually converts from.
fn check_pinned_base(root: &Path, p: &PinFile) -> Result<(), String> {
    let base = pinned_base(p)?;
    if !root.join(SHADCN_DIR).join("apps/v4/registry/bases").join(&base).exists() {
        return Err(format!(
            "pin.json targets base {:?}, which the pinned checkout does not have under apps/v4/registry/bases",
            base
        ));
    }
    let g = crate::graph::load_graph_at(root)?;
    let n = g
        .node("convert")
        .ok_or_else(|| "no convert node to check the pinned base against".to_string())?;
    let want = format!("registry/bases/{}/", base);
    for in_ in n.inputs.iter().flatten() {
        if in_.contains(&want) {
            return Ok(());
        }
    }
    Err(format!(
        "pin.json targets base {:?} but the convert node declares no input under {} — the pin and the graph name different registries",
        base, want
    ))
}

pub fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn sha256_file(path: &Path) -> Result<String, String> {
    crate::key::hash_file(path).map_err(|e| e.to_string())
}

pub fn read_pin(root: &Path) -> Result<PinFile, String> {
    let b = std::fs::read_to_string(root.join(PIN_FILE_PATH))
        .map_err(|e| format!("{}: {}", PIN_FILE_PATH, e))?;
    serde_json::from_str(&b).map_err(|e| format!("{}: {}", PIN_FILE_PATH, e))
}

/// Bootstraps a missing .upstream/shadcn-ui checkout by cloning at the tag
/// src/registry/pin.json already records — a fresh checkout always has a
/// committed pin to clone to, so this never has to guess a version.
fn clone_upstream(root: &Path) -> Result<(), String> {
    let p = read_pin(root)
        .map_err(|e| format!("cannot read {} to learn which tag to clone: {}", PIN_FILE_PATH, e))?;
    if p.shadcn_ui.tag.is_empty() {
        return Err(format!("{} has no shadcn_ui.tag recorded", PIN_FILE_PATH));
    }
    eprintln!(
        "PIN: {} not found — cloning shadcn-ui at {}",
        SHADCN_DIR, p.shadcn_ui.tag
    );
    git(
        root,
        &[
            "clone",
            "--quiet",
            "--branch",
            &p.shadcn_ui.tag,
            SHADCN_REPO,
            SHADCN_DIR,
        ],
    )
    .map_err(|e| format!("git clone --branch {}: {}", p.shadcn_ui.tag, e))?;
    Ok(())
}

/// mutations.go truncate: plain byte cut, no suffix.
pub fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        return s.to_string();
    }
    let mut end = n;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

/// pipeline pin — record, or verify an existing pin.json (--check-only);
/// --force re-records at the checkout's HEAD despite drift (the re-pin drill).
pub fn run_pin(root: &Path, check_only: bool, force: bool) -> i32 {
    if !root.join(SHADCN_DIR).exists() {
        if let Err(e) = clone_upstream(root) {
            eprintln!("PIN FAIL: {} not found and auto-clone failed: {}", SHADCN_DIR, e);
            return 1;
        }
    }
    let head = match git(root, &["-C", SHADCN_DIR, "rev-parse", "HEAD"]) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("PIN FAIL: cannot read upstream HEAD: {}", e);
            return 1;
        }
    };
    let kernel_sha = match sha256_file(&root.join(KERNEL_IIFE)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("PIN FAIL: cannot hash {}: {}", KERNEL_IIFE, e);
            return 1;
        }
    };

    if check_only {
        let recorded = match read_pin(root) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("PIN FAIL: src/registry/pin.json missing — run npm run pin");
                return 1;
            }
        };
        let mut fail = false;
        if let Err(e) = check_pinned_base(root, &recorded) {
            eprintln!("PIN FAIL: {}", e);
            fail = true;
        }
        if recorded.shadcn_ui.commit != head {
            eprintln!(
                "PIN FAIL: pin.json commit {} != upstream HEAD {}",
                truncate(&recorded.shadcn_ui.commit, 10),
                truncate(&head, 10)
            );
            fail = true;
        }
        if !release_tag_re().is_match(&recorded.shadcn_ui.tag) {
            eprintln!(
                "PIN FAIL: pin.json tag {:?} is not a shadcn@* release tag",
                recorded.shadcn_ui.tag
            );
            fail = true;
        }
        if recorded.kernel.sha256 != kernel_sha {
            eprintln!(
                "PIN FAIL: kernel sha256 drift (pin.json {}… != vendor {}…)",
                truncate(&recorded.kernel.sha256, 12),
                truncate(&kernel_sha, 12)
            );
            fail = true;
        }
        if fail {
            return 1;
        }
        println!(
            "pin OK (check-only): shadcn={} ({}) kernel={}…",
            recorded.shadcn_ui.tag,
            truncate(&head, 10),
            truncate(&kernel_sha, 12)
        );
        return 0;
    }

    // tags whose target commit == HEAD (peels annotated tags to their commit)
    let raw = git(
        root,
        &[
            "-C",
            SHADCN_DIR,
            "for-each-ref",
            "--format=%(refname:short)",
            "--points-at",
            "HEAD",
            "refs/tags",
        ],
    )
    .unwrap_or_default();
    let tags_at_head: Vec<String> = raw
        .split('\n')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();
    let mut release_tag = String::new();
    for t in &tags_at_head {
        if release_tag_re().is_match(t) {
            release_tag = t.clone();
            break;
        }
    }
    if release_tag.is_empty() {
        let at = if tags_at_head.is_empty() {
            "(none)".to_string()
        } else {
            tags_at_head.join(", ")
        };
        eprintln!(
            "PIN FAIL: upstream HEAD {} is not a shadcn@* release tag",
            truncate(&head, 10)
        );
        eprintln!("  tags at HEAD: {}", at);
        eprintln!(
            "  checkout a shadcn@* tag before pinning (e.g. git -C {} checkout shadcn@4.19.0)",
            SHADCN_DIR
        );
        return 1;
    }

    let next = PinFile {
        shadcn_ui: PinShadcnUi {
            repo: SHADCN_REPO.to_string(),
            tag: release_tag.clone(),
            commit: head.clone(),
            registry: "apps/v4/registry/bases/radix/ui".to_string(),
        },
        kernel: PinKernel {
            file: KERNEL_IIFE.to_string(),
            sha256: kernel_sha.clone(),
        },
        recorded: today(),
    };

    let write = |next: &PinFile| -> Result<(), String> {
        // Go json.MarshalIndent(next, "", "  ") — struct field order
        let b = format!(
            "{{\n  \"shadcn_ui\": {{\n    \"repo\": {},\n    \"tag\": {},\n    \"commit\": {},\n    \"registry\": {}\n  }},\n  \"kernel\": {{\n    \"file\": {},\n    \"sha256\": {}\n  }},\n  \"recorded\": {}\n}}\n",
            crate::jsonorder::json_string(&next.shadcn_ui.repo),
            crate::jsonorder::json_string(&next.shadcn_ui.tag),
            crate::jsonorder::json_string(&next.shadcn_ui.commit),
            crate::jsonorder::json_string(&next.shadcn_ui.registry),
            crate::jsonorder::json_string(&next.kernel.file),
            crate::jsonorder::json_string(&next.kernel.sha256),
            crate::jsonorder::json_string(&next.recorded),
        );
        std::fs::create_dir_all(root.join("src/registry")).map_err(|e| e.to_string())?;
        std::fs::write(root.join(PIN_FILE_PATH), b).map_err(|e| e.to_string())
    };

    match read_pin(root) {
        Err(_) => {
            if let Err(e) = write(&next) {
                eprintln!("PIN FAIL: {}", e);
                return 1;
            }
            println!(
                "pin recorded: shadcn={} ({}) kernel={}…",
                release_tag,
                truncate(&head, 10),
                truncate(&kernel_sha, 12)
            );
            0
        }
        Ok(old) => {
            let drift =
                old.shadcn_ui.commit != next.shadcn_ui.commit || old.kernel.sha256 != next.kernel.sha256;
            if drift && force {
                if let Err(e) = write(&next) {
                    eprintln!("PIN FAIL: {}", e);
                    return 1;
                }
                println!(
                    "pin re-recorded: shadcn={} -> {} ({}) kernel={}…",
                    old.shadcn_ui.tag,
                    release_tag,
                    truncate(&head, 10),
                    truncate(&kernel_sha, 12)
                );
                return 0;
            }
            if drift {
                eprintln!("PIN DRIFT detected:");
                if old.shadcn_ui.commit != next.shadcn_ui.commit {
                    eprintln!(
                        "  shadcn: {} -> {}",
                        old.shadcn_ui.commit, next.shadcn_ui.commit
                    );
                }
                if old.kernel.sha256 != next.kernel.sha256 {
                    eprintln!(
                        "  kernel sha256: {} -> {}",
                        old.kernel.sha256, next.kernel.sha256
                    );
                }
                return 1;
            }
            println!(
                "pin OK: shadcn={} ({}) kernel={}…",
                release_tag,
                truncate(&head, 10),
                truncate(&kernel_sha, 12)
            );
            0
        }
    }
}

/// Go time.Now().Format("2006-01-02") — local timezone. std has no local
/// date; `date +%F` is the same shape the repo's scripts already rely on.
fn today() -> String {
    let out = std::process::Command::new("date").arg("+%F").output();
    if let Ok(o) = out {
        if o.status.success() {
            return String::from_utf8_lossy(&o.stdout).trim().to_string();
        }
    }
    String::new()
}

pub fn abs_pin(root: &Path, p: &str) -> PathBuf {
    let q = Path::new(p);
    if q.is_absolute() {
        q.to_path_buf()
    } else {
        root.join(q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real_root() -> std::path::PathBuf {
        match std::env::var("SHADLESS_ROOT") {
            Ok(r) => std::path::PathBuf::from(r),
            Err(_) => {
                let m = Path::new(env!("CARGO_MANIFEST_DIR")).join("../shadless");
                m.canonicalize().unwrap_or(m)
            }
        }
    }

    /// Go TestPin: the gate runs `runPin(root, true, false)` from the repo
    /// root — verify only, record nothing.
    #[test]
    fn pin_on_real_tree() {
        let root = real_root();
        if !root.join(".upstream/shadcn-ui").exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: no pinned upstream checkout)");
            }
            eprintln!("skip: no pinned upstream checkout");
            return;
        }
        if run_pin(&root, true, false) != 0 {
            panic!("FAIL  pin (see PIN FAIL above)");
        }
    }
}
