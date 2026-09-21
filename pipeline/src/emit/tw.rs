//! Port of pipeline/tw.go — hermetic @tailwindcss/cli wrapper.
//!
//! The compile cwd is the whole point of the wrapper: paths are resolved
//! against the REPO ROOT and never against that cwd — including the CLI
//! binary, which must come from this repo's node_modules. An empty compile
//! cwd means "fresh empty scratch dir", i.e. zero content scanning (the
//! product build must emit ONLY @apply-driven rules).

use std::path::{Path, PathBuf};

/// SHADLESS_ROOT wins when set; else walk up to the product tree: the dir
/// holding package.json with a pipeline/ directory (the engine lives at
/// pipeline/ in the same repo).
pub fn find_repo_root(dir: &Path) -> Result<PathBuf, String> {
    if let Ok(r) = std::env::var("SHADLESS_ROOT") {
        return Ok(PathBuf::from(r));
    }
    let mut dir = dir.to_path_buf();
    loop {
        if is_repo_root(&dir) {
            return Ok(dir);
        }
        let parent = match dir.parent() {
            Some(p) => p.to_path_buf(),
            None => {
                return Err("repo root (the tree holding package.json + pipeline/) not found above the working directory; set SHADLESS_ROOT to say where it is".to_string())
            }
        };
        if parent == dir {
            return Err("repo root (the tree holding package.json + pipeline/) not found above the working directory; set SHADLESS_ROOT to say where it is".to_string());
        }
        dir = parent;
    }
}

fn is_repo_root(dir: &Path) -> bool {
    dir.join("package.json").exists() && dir.join("pipeline").is_dir()
}

/// `in` and `out` are resolved against the repo root, never against the
/// compile cwd, and so is the CLI binary.
pub fn tw_compile(
    root: &Path,
    input: &str,
    out: &str,
    compile_cwd: &str,
    minify: bool,
    quiet: bool,
) -> Result<(), String> {
    let abs = |p: &str| -> PathBuf {
        let p = Path::new(p);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            root.join(p)
        }
    };
    let mut argv = vec![
        "-i".to_string(),
        abs(input).to_string_lossy().into_owned(),
        "-o".to_string(),
        abs(out).to_string_lossy().into_owned(),
    ];
    if minify {
        argv.push("--minify".to_string());
    }

    // an empty compile cwd means a scratch dir: zero content scanning. Hold
    // the TempDir to the end of the function so the compile runs inside it
    // and the drop deletes it — scratch.keep() here consumed the guard
    // without ever deleting, leaking one empty /tmp dir per call.
    let scratch = if compile_cwd.is_empty() {
        Some(
            tempfile::Builder::new()
                .prefix("shadless-tw-")
                .tempdir()
                .map_err(|e| e.to_string())?,
        )
    } else {
        None
    };
    let dir: PathBuf = match &scratch {
        Some(s) => s.path().to_path_buf(),
        None => abs(compile_cwd),
    };

    let mut cmd = std::process::Command::new(root.join("node_modules/.bin/tailwindcss"));
    cmd.args(&argv).current_dir(&dir);
    if !quiet {
        cmd.stdout(std::process::Stdio::inherit());
        cmd.stderr(std::process::Stdio::inherit());
    } else {
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());
    }
    let status = cmd.status().map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("exit status {}", status.code().unwrap_or(-1)));
    }
    Ok(())
}

pub fn run_tw(args: &[String]) -> i32 {
    let mut minify = false;
    let mut compile_cwd = String::new();
    let mut positional: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--minify" => minify = true,
            "--cwd" => {
                if i + 1 >= args.len() {
                    eprintln!("tw: --cwd needs a directory");
                    return 1;
                }
                compile_cwd = args[i + 1].clone();
                i += 1;
            }
            other => positional.push(other.to_string()),
        }
        i += 1;
    }
    if positional.len() != 2 {
        eprintln!("usage: pipeline tw <in> <out> [--minify] [--cwd DIR]");
        return 1;
    }
    let wd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("tw: {}", e);
            return 1;
        }
    };
    let root = match find_repo_root(&wd) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("tw: {}", e);
            return 1;
        }
    };
    match tw_compile(
        &root,
        &positional[0],
        &positional[1],
        &compile_cwd,
        minify,
        false,
    ) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("tw: {}", e);
            1
        }
    }
}
