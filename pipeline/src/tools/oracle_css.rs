//! Port of pipeline/oracle_css.go; the contract was proven byte-identical
//! during the port (go-parity-final tag).
//!
//! A stylesheet for the React oracle that owes nothing to src/emitter:
//! build/gates/oracle.css is built from upstream's own inputs only, compiled
//! with the same tailwindcss the product uses. Nothing under src/ is read —
//! that is the whole point.

use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

const UPSTREAM_DIR: &str = ".upstream/shadcn-ui";
const ORACLE_OUT_DIR: &str = "build/gates";

static SKIN_RULE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\.style-nova\s+\.cn-[0-9A-Za-z_-]+").unwrap());

/// buildOracleEntryCSS applies the app's own globals.css line-by-line: the
/// shadcn/tailwind.css import is inlined verbatim, the legacy-themes.css
/// import is spliced back in (as an @import of legacyImportPath) only when
/// hasLegacy says the file exists, every @source line the app declared is
/// dropped (the oracle scans its own resolved tree instead, appended below),
/// and every other line passes through unchanged. sourceDirs become the
/// oracle's own @source list, and skinCSS (the pinned skin, verbatim) is
/// appended last.
pub fn build_oracle_entry_css(
    app_css: &str,
    shadcn_tailwind_css: &str,
    skin_css: &str,
    legacy_import_path: &str,
    has_legacy: bool,
    source_dirs: &[String],
) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in app_css.split('\n') {
        if line.contains("\"shadcn/tailwind.css\"") {
            lines.push("/* shadcn/tailwind.css (inlined from packages/shadcn/src) */".to_string());
            lines.push(shadcn_tailwind_css.to_string());
        } else if line.contains("\"./legacy-themes.css\"") {
            if has_legacy {
                lines.push(format!("@import {:?};", legacy_import_path));
            }
        } else if line.starts_with("@source ") {
            // the app's own style dirs; replaced below
        } else {
            lines.push(line.to_string());
        }
    }
    for d in source_dirs {
        lines.push(format!("@source {:?};", d));
    }
    lines.push("/* === style-nova.css (the pinned skin, verbatim) === */".to_string());
    lines.push(skin_css.to_string());
    lines.join("\n")
}

pub fn run_oracle_css() -> i32 {
    let wd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("oracle-css: {}", e);
            return 1;
        }
    };
    let root = match crate::emit::tw::find_repo_root(&wd) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("oracle-css: {}", e);
            return 1;
        }
    };
    let abs = |p: &str| -> PathBuf { root.join(p) };
    let read = |p: &str| -> Result<String, String> {
        let path = abs(p);
        fs::read(&path)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| crate::fsutil::go_path_err("open", &path, &e))
    };

    let app = match read(&format!("{}/apps/v4/app/globals.css", UPSTREAM_DIR)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("oracle-css: {}", e);
            return 1;
        }
    };
    let shadcn_tw = match read(&format!(
        "{}/packages/shadcn/src/tailwind.css",
        UPSTREAM_DIR
    )) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("oracle-css: {}", e);
            return 1;
        }
    };
    let skin = match read(&format!(
        "{}/apps/v4/registry/styles/style-nova.css",
        UPSTREAM_DIR
    )) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("oracle-css: {}", e);
            return 1;
        }
    };
    let legacy = abs(&format!("{}/apps/v4/app/legacy-themes.css", UPSTREAM_DIR));
    let legacy_err = fs::metadata(&legacy).err();

    let entry_css = build_oracle_entry_css(
        &app,
        &shadcn_tw,
        &skin,
        legacy.to_str().unwrap_or(""),
        legacy_err.is_none(),
        &[
            abs("build/resolved-ui").to_string_lossy().into_owned(),
            // usage trees carry example classes
            abs("tools/contracts/components")
                .to_string_lossy()
                .into_owned(),
            // the examples' own utilities (max-w-lg on an accordion demo, …): the
            // demo pages carry them inline, so the oracle stylesheet must define them
            abs(&format!("{}/apps/v4/examples", UPSTREAM_DIR))
                .to_string_lossy()
                .into_owned(),
        ],
    );

    if let Err(e) = fs::create_dir_all(abs(ORACLE_OUT_DIR)) {
        eprintln!(
            "oracle-css: {}",
            crate::fsutil::go_path_err("mkdir", &abs(ORACLE_OUT_DIR), &e)
        );
        return 1;
    }
    let entry = format!("{}/oracle.entry.css", ORACLE_OUT_DIR);
    if let Err(e) = fs::write(abs(&entry), entry_css.as_bytes()) {
        eprintln!(
            "oracle-css: {}",
            crate::fsutil::go_path_err("open", &abs(&entry), &e)
        );
        return 1;
    }

    let out = format!("{}/oracle.css", ORACLE_OUT_DIR);
    // Through the same wrapper the product uses, and with NO compile cwd: an
    // empty scratch dir, so the only content scanned is the @source list above
    // — all of it absolute, all of it upstream. (See the Go source for why
    // passing oracleOutDir here was a bug.)
    if let Err(e) = crate::emit::tw::tw_compile(&root, &entry, &out, "", false, false) {
        eprintln!("oracle-css: {}", e);
        return 1;
    }
    let css = match read(&out) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("oracle-css: {}", e);
            return 1;
        }
    };
    println!(
        "oracle-css: {} ({:.0}KB, {} skin rules, zero bytes from src/)",
        out,
        css.len() as f64 / 1024.0,
        SKIN_RULE.find_iter(&css).count()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mirrors pipeline/oracle_css_test.go TestUnitBuildOracleEntryCSS.
    #[test]
    fn unit_build_oracle_entry_css() {
        let app = [
            r#"@import "tailwindcss";"#,
            r#"@import "shadcn/tailwind.css";"#,
            r#"@import "./legacy-themes.css";"#,
            r#"@source "./app/own-dir";"#,
            ".custom { color: red; }",
        ]
        .join("\n");
        let shadcn_tw = ".tw-base { all: unset; }";
        let skin = ".style-nova .cn-button { color: blue; }";
        let source_dirs: Vec<String> = vec![
            "/abs/resolved-ui".to_string(),
            "/abs/tools/contracts/components".to_string(),
        ];

        // legacy present
        let got = build_oracle_entry_css(
            &app,
            shadcn_tw,
            skin,
            "/abs/legacy-themes.css",
            true,
            &source_dirs,
        );
        let want = [
            r#"@import "tailwindcss";"#,
            "/* shadcn/tailwind.css (inlined from packages/shadcn/src) */",
            shadcn_tw,
            r#"@import "/abs/legacy-themes.css";"#,
            ".custom { color: red; }",
            r#"@source "/abs/resolved-ui";"#,
            r#"@source "/abs/tools/contracts/components";"#,
            "/* === style-nova.css (the pinned skin, verbatim) === */",
            skin,
        ]
        .join("\n");
        assert_eq!(got, want, "legacy present");

        // legacy absent
        let got = build_oracle_entry_css(
            &app,
            shadcn_tw,
            skin,
            "/abs/legacy-themes.css",
            false,
            &source_dirs,
        );
        assert!(
            !got.contains("legacy-themes"),
            "legacy import present despite has_legacy=false: {}",
            got
        );
        let want = [
            r#"@import "tailwindcss";"#,
            "/* shadcn/tailwind.css (inlined from packages/shadcn/src) */",
            shadcn_tw,
            ".custom { color: red; }",
            r#"@source "/abs/resolved-ui";"#,
            r#"@source "/abs/tools/contracts/components";"#,
            "/* === style-nova.css (the pinned skin, verbatim) === */",
            skin,
        ]
        .join("\n");
        assert_eq!(got, want, "legacy absent");

        // app @source lines dropped
        let got = build_oracle_entry_css(
            &app,
            shadcn_tw,
            skin,
            "/abs/legacy-themes.css",
            true,
            &source_dirs,
        );
        assert!(
            !got.contains("own-dir"),
            "app's own @source line survived: {}",
            got
        );

        // passthrough line kept verbatim
        let got = build_oracle_entry_css(
            &app,
            shadcn_tw,
            skin,
            "/abs/legacy-themes.css",
            true,
            &source_dirs,
        );
        assert!(
            got.contains(".custom { color: red; }"),
            "passthrough line dropped: {}",
            got
        );
    }
}
