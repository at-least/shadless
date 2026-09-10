//! Port of pipeline/jsbuild.go — the JS product surface. One base plus one
//! file per component, mirroring the CSS surface.
//!
//!   dist/shadless.js       vendored radix kernel + src/runtime/core.js
//!   dist/shadless.min.js   the same, minified
//!   dist/js/<name>.js      src/runtime/components/<name>.js
//!   dist/esm/shadless.mjs  the ONLY ES-module base
//!   dist/esm/<name>.mjs    `import "./shadless.mjs"` + the component file
//!
//! Minification spawns the pinned node_modules esbuild binary — the Go
//! in-process api.Transform with these exact options was proven byte-identical
//! to this CLI in probe/m0 (75,503 bytes, equal to the committed artifact).

use std::io::Write as _;
use std::path::Path;

/// The members of window.shadless re-exported by name; TestUnitRuntimeNamedExports
/// in Go asserts the set against the source.
const NAMED_EXPORTS: &[&str] = &[
    "init", "initAll", "destroy", "refresh", "start", "stop", "register", "get", "instances", "h",
    "theme",
];

/// The `;` is load-bearing: `})(window)` followed by `(function () {` would
/// otherwise parse as a CALL.
fn iife_base(kernel: &str, core: &str) -> String {
    format!("{}\n;\n{}", kernel, core)
}

fn esm_base(kernel: &str, core: &str) -> String {
    iife_base(kernel, core)
        + &format!(
            "\n;\nconst shadless = globalThis.shadless\nexport default shadless\nexport const {{ {} }} = shadless\n",
            NAMED_EXPORTS.join(", ")
        )
}

fn esm_component(src: &str) -> String {
    format!("import \"./shadless.mjs\"\n;\n{}", src)
}

/// Minify via the pinned esbuild CLI — the same options the Go/JS passed
/// (MinifyWhitespace+Identifiers+Syntax, target es2017, no format flag).
fn minify(root: &Path, src: &str) -> Result<String, String> {
    let out = std::process::Command::new(root.join("node_modules/.bin/esbuild"))
        .args(["--minify", "--target=es2017"])
        .current_dir(root)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child
                .stdin
                .as_mut()
                .expect("piped")
                .write_all(src.as_bytes())?;
            child.wait_with_output()
        })
        .map_err(|e| format!("esbuild: {}", e))?;
    if !out.status.success() {
        return Err(format!(
            "esbuild: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// buildJs writes the whole JS surface under dist and returns the component
/// names it emitted.
pub fn build_js(root: &Path) -> Result<Vec<String>, String> {
    let read = |rel: &str| -> Result<String, String> {
        std::fs::read_to_string(root.join(rel)).map_err(|e| format!("{}: {}", rel, e))
    };
    let write = |rel: &str, content: &str| -> Result<(), String> {
        let p = root.join("dist").join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(p, content).map_err(|e| e.to_string())
    };

    let kernel = read("vendor/radix-kernel.iife.js")?;
    let core = read("src/runtime/core.js")?;
    let base = iife_base(&kernel, &core);
    write("shadless.js", &base)?;
    let min_base = minify(root, &base)?;
    write("shadless.min.js", &min_base)?;

    // dist/js and dist/esm are rebuilt from scratch: a component deleted
    // upstream must not leave its file behind, where the pack gate would keep
    // shipping it.
    for d in ["js", "esm"] {
        let p = root.join("dist").join(d);
        if p.exists() {
            std::fs::remove_dir_all(&p).map_err(|e| e.to_string())?;
        }
        std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
    }

    let esm = esm_base(&kernel, &core);
    write("esm/shadless.mjs", &esm)?;
    // No minified ESM base is emitted: a second base module cannot be shared
    // with dist/esm/<name>.mjs, which hardcodes `import "./shadless.mjs"` —
    // importing one alongside any component yields two instances.
    let dts = read("src/runtime/shadless.d.ts")?;
    write("esm/shadless.d.ts", &dts)?;

    let comps = root.join("src/runtime/components");
    let mut files: Vec<String> = Vec::new();
    for e in std::fs::read_dir(&comps).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        let name = e.file_name().to_string_lossy().into_owned();
        if name.ends_with(".js") {
            files.push(name);
        }
    }
    files.sort();

    let mut names: Vec<String> = Vec::new();
    for f in &files {
        let name = f.trim_end_matches(".js").to_string();
        let mut src = read(&format!("src/runtime/components/{}", f))?;
        if name == "carousel" {
            let embla = read("vendor/embla-carousel.iife.js")?;
            src = format!("{}\n;\n{}", embla, src);
        }
        write(&format!("js/{}", f), &src)?;
        write(&format!("esm/{}.mjs", name), &esm_component(&src))?;
        write(
            &format!("esm/{}.d.ts", name),
            &format!(
                "// registers the {} behavior with the base (side-effect module)\nimport \"./shadless.mjs\"\nexport {{}}\n",
                name
            ),
        )?;
        names.push(name);
    }
    Ok(names)
}

pub fn run_build_js() -> i32 {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    match build_js(&root) {
        Ok(names) => {
            println!(
                "build-js: dist/shadless.js (base) + {} component files in dist/js/ (+ dist/esm/ mirrors)",
                names.len()
            );
            0
        }
        Err(e) => {
            eprintln!("build-js: {}", e);
            1
        }
    }
}
