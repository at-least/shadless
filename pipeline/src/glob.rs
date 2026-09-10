//! Port of the glob half of shadless/pipeline/key.go.
//!
//! Patterns are the registry's: `*` within a segment, `**` across segments,
//! `?` for one non-slash char, a leading `!` excluding (applied last, in
//! [`files`]). Like the Go original, patterns compile to anchored regexps and
//! the literal prefix decides where the walk starts. The translation is done
//! character-for-character so accept/reject parity is structural, and the
//! golden-diff harness over `pipeline inputs <node>` checks it against the
//! real tree.

use regex::Regex;
use std::fs;
use std::path::Path;

/// Go's regexp.QuoteMeta set (escapes are byte-exact for ASCII patterns).
const META: &[u8] = b"\\.+*?()|[]{}^$";

fn quote_meta_byte(byte: u8) -> Result<String, String> {
    if byte < 0x80 {
        let c = byte as char;
        if META.contains(&byte) {
            Ok(format!("\\{}", c))
        } else {
            Ok(c.to_string())
        }
    } else {
        // Go compiles the pattern as UTF-8 and fails on a lone high byte; the
        // repo's patterns are all ASCII, so mirror the failure.
        Err(format!("invalid glob pattern: non-ASCII byte 0x{:02x}", byte))
    }
}

/// Port of globToRegexp: `**/` -> `(?:[^/]+/)*`, `**` -> `.*`, `*` -> `[^/]*`,
/// `?` -> `[^/]`, everything else quoted; anchored at both ends.
pub fn glob_to_regexp(pat: &str) -> Result<Regex, String> {
    let mut b = String::from("^");
    let bytes = pat.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &pat[i..];
        if rest.starts_with("**/") {
            b.push_str("(?:[^/]+/)*");
            i += 3;
        } else if rest.starts_with("**") {
            b.push_str(".*");
            i += 2;
        } else if bytes[i] == b'*' {
            b.push_str("[^/]*");
            i += 1;
        } else if bytes[i] == b'?' {
            b.push_str("[^/]");
            i += 1;
        } else {
            b.push_str(&quote_meta_byte(bytes[i])?);
            i += 1;
        }
    }
    b.push('$');
    Regex::new(&b).map_err(|e| e.to_string())
}

fn has_meta(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

/// Port of literalPrefix: the deepest directory of pat that contains no
/// wildcard — where the walk can start instead of scanning the repo. A fully
/// literal pattern returns itself.
pub fn literal_prefix(pat: &str) -> String {
    let segs: Vec<&str> = pat.split('/').collect();
    let mut keep: Vec<&str> = Vec::new();
    for s in &segs {
        if has_meta(s) {
            break;
        }
        keep.push(s);
    }
    if keep.len() == segs.len() {
        pat.to_string()
    } else {
        keep.join("/")
    }
}

/// Port of expand: resolve one pattern to the sorted set of files it names,
/// relative to root. A fully literal pattern naming a directory expands to
/// that whole directory; a pattern naming nothing contributes nothing.
pub fn expand(root: &Path, pat: &str) -> Result<Vec<String>, String> {
    let prefix = literal_prefix(pat);
    let fully_literal = prefix == pat;
    let start = root.join(&prefix);
    let Ok(meta) = fs::metadata(&start) else {
        return Ok(Vec::new()); // a pattern naming nothing contributes nothing
    };
    if fully_literal && !meta.is_dir() {
        return Ok(vec![prefix]);
    }
    let re = glob_to_regexp(pat)?;
    let mut out: Vec<String> = Vec::new();
    for entry in walkdir::WalkDir::new(&start) {
        // unreadable entries are not silently-passing inputs; they are absent
        let Ok(entry) = entry else { continue };
        if entry.file_type().is_dir() {
            continue;
        }
        let Ok(rel) = entry.path().strip_prefix(root) else {
            continue;
        };
        let rel = rel.to_string_lossy().into_owned();
        if fully_literal || re.is_match(&rel) {
            out.push(rel);
        }
    }
    out.sort();
    Ok(out)
}

/// Port of Files: resolve a whole pattern list, applying `!` exclusions last
/// so order in the registry does not matter. Result is sorted and deduped.
pub fn files(root: &Path, patterns: &[String]) -> Result<Vec<String>, String> {
    let mut include: Vec<String> = Vec::new();
    let mut excludes: Vec<Regex> = Vec::new();
    for p in patterns {
        if let Some(rest) = p.strip_prefix('!') {
            excludes.push(glob_to_regexp(rest)?);
            continue;
        }
        include.extend(expand(root, p)?);
    }
    include.sort();
    include.dedup();
    include.retain(|f| !excludes.iter().any(|re| re.is_match(f)));
    Ok(include)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// a.txt, b.md, sub/c.txt, sub/deep/d.txt, .hidden/h.txt
    fn tree() -> PathBuf {
        let tmp = std::env::temp_dir().join(format!(
            "shadless-rs-glob-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        for d in ["", "sub", "sub/deep", ".hidden"] {
            fs::create_dir_all(tmp.join(d)).unwrap();
        }
        for f in ["a.txt", "b.md", "sub/c.txt", "sub/deep/d.txt", ".hidden/h.txt"] {
            fs::write(tmp.join(f), f).unwrap();
        }
        tmp
    }

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn literal_file_missing_file_and_literal_dir() {
        let t = tree();
        assert_eq!(expand(&t, "a.txt").unwrap(), s(&["a.txt"]));
        assert_eq!(expand(&t, "zz.txt").unwrap(), Vec::<String>::new());
        // a fully literal pattern naming a directory expands to the whole
        // directory, recursively
        assert_eq!(
            expand(&t, "sub").unwrap(),
            s(&["sub/c.txt", "sub/deep/d.txt"])
        );
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn star_stays_in_segment_doublestar_crosses() {
        let t = tree();
        assert_eq!(expand(&t, "*.txt").unwrap(), s(&["a.txt"]));
        assert_eq!(expand(&t, "**/c.txt").unwrap(), s(&["sub/c.txt"]));
        assert_eq!(
            expand(&t, "sub/**").unwrap(),
            s(&["sub/c.txt", "sub/deep/d.txt"])
        );
        assert_eq!(expand(&t, "?.txt").unwrap(), s(&["a.txt"]));
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn files_applies_exclusions_last_and_sorts() {
        let t = tree();
        let pats = s(&[".hidden/**", "sub/**", "*.txt", "!sub/deep/**"]);
        assert_eq!(
            files(&t, &pats).unwrap(),
            s(&[".hidden/h.txt", "a.txt", "sub/c.txt"])
        );
        // order of patterns does not matter
        let pats2 = s(&["!sub/deep/**", "sub/**", ".hidden/**", "*.txt"]);
        assert_eq!(files(&t, &pats).unwrap(), files(&t, &pats2).unwrap());
        let _ = fs::remove_dir_all(&t);
    }

    #[test]
    fn literal_prefix_stops_at_first_meta_segment() {
        assert_eq!(literal_prefix("vendor/**"), "vendor");
        assert_eq!(literal_prefix("src/registry/pin.json"), "src/registry/pin.json");
        assert_eq!(literal_prefix("dist/components/*.html"), "dist/components");
        assert_eq!(literal_prefix("gates/*-baseline.json"), "gates");
        assert_eq!(literal_prefix("pipeline/*.go"), "pipeline");
    }

    #[test]
    fn glob_translation_matches_go_semantics() {
        let re = |p: &str| glob_to_regexp(p).unwrap();
        assert!(re("vendor/**").is_match("vendor/x/y.js"));
        assert!(!re("vendor/**").is_match("vendor")); // needs the slash
        assert!(re("**/c.txt").is_match("c.txt")); // (?:[^/]+/)* allows zero
        assert!(re("**/c.txt").is_match("a/b/c.txt"));
        assert!(!re("*.go").is_match("sub/a.go"));
        assert!(re("dist/components/*.html").is_match("dist/components/button.html"));
        assert!(!re("dist/components/*.html").is_match("dist/components/x/button.html"));
        assert!(re("!dist/components/*-rtl-*.html".trim_start_matches('!'))
            .is_match("dist/components/table-rtl-he.html"));
        assert!(re("^TestPin$").is_match("^TestPin$")); // quoting is literal
    }
}
