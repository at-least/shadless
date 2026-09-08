//! Port of pipeline/gate_css_direction.go's runCSSDirectionUpdate — the
//! `pipeline css-direction --update` subcommand: print a fresh inventory of
//! the physical direction utilities in dist/shadless.css for pasting into
//! the baseline after review.

use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

fn leading_word() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^([a-z]+)(.*)$").unwrap())
}

pub fn run_css_direction_update(root: &Path) -> i32 {
    let css = match std::fs::read_to_string(root.join("dist/shadless.css")) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("css-direction: {}", e);
            return 1;
        }
    };
    let entries = crate::gates::scan_directions(&css);
    println!(
        "// fresh inventory ({} entries) — paste into directionBaseline in pipeline/gate_css_direction.go after review:",
        entries.len()
    );
    for e in entries {
        let m = leading_word().captures(&e.token).unwrap();
        println!("\t{:?} + {:?}: {},", &m[1], &m[2], e.n);
    }
    0
}
