//! Port of pipeline/rtl_dict.go — lift the RTL translation dictionaries out of
//! upstream's aria examples into src/registry/rtl-translations.json. See the Go
//! source for the contract; byte-identity with the Go binary is the acceptance
//! bar.

use crate::convert::esbuild_tsx;
use crate::jsonorder::json_string;
use crate::tsx::{self, Span};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

const RTL_DICT_EXAMPLES: &str = ".upstream/shadcn-ui/apps/v4/examples/aria";
const RTL_DICT_OUT: &str = "src/registry/rtl-translations.json";
const RTL_TIERS_PATH: &str = "src/registry/tiers.json";

/// rtlLangs is one dictionary: insertion-ordered languages, each with its
/// dir and insertion-ordered values. The committed JSON records source
/// order (JS Map insertion order through JSON.stringify), so Go maps are
/// not enough.
struct RtlLangs {
    names: Vec<String>,
    dir: HashMap<String, String>,
    values: HashMap<String, RtlValues>,
}

struct RtlValues {
    keys: Vec<String>,
    vals: HashMap<String, String>,
}

#[derive(Deserialize)]
struct TierEntry {
    #[serde(default)]
    tier: String,
    #[serde(default)]
    emit: bool,
}

pub fn run_rtl_dict() -> i32 {
    let root = std::env::current_dir().unwrap_or_default();
    run_rtl_dict_at(&root)
}

fn run_rtl_dict_at(root: &Path) -> i32 {
    let examples_dir = root.join(RTL_DICT_EXAMPLES);
    let entries = match std::fs::read_dir(&examples_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("rtl-dict: {}", go_fs_err(RTL_DICT_EXAMPLES, &e));
            return 1;
        }
    };
    let mut files: Vec<String> = Vec::new();
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if name.ends_with("-rtl.tsx") {
            files.push(name);
        }
    }
    files.sort();
    if files.is_empty() {
        eprintln!(
            "FAIL  rtl-dict: no *-rtl.tsx under {} — the pinned upstream moved or the checkout is incomplete",
            RTL_DICT_EXAMPLES
        );
        return 1;
    }

    let tiers_b = match std::fs::read(root.join(RTL_TIERS_PATH)) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("rtl-dict: {}", go_fs_err(RTL_TIERS_PATH, &e));
            return 1;
        }
    };
    let tiers: HashMap<String, TierEntry> = match serde_json::from_slice(&tiers_b) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("rtl-dict: tiers: {}", e);
            return 1;
        }
    };
    let shipped = |name: &str| -> bool {
        let Some(t) = tiers.get(name.trim_end_matches("-rtl")) else {
            return false;
        };
        t.emit || t.tier == "static" || t.tier == "kernel" || t.tier == "trivial-js"
    };

    let mut dict_order: Vec<String> = Vec::new();
    let mut dicts: HashMap<String, RtlLangs> = HashMap::new();
    let mut failures: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    for file in &files {
        let name = file.trim_end_matches(".tsx").to_string();
        let b = match std::fs::read_to_string(examples_dir.join(file)) {
            Ok(b) => b,
            Err(e) => {
                // Go: name+": "+err.Error() — the RELATIVE joined path (the
                // constants are relative) plus errno text
                let joined = format!("{}/{}", RTL_DICT_EXAMPLES, file);
                failures.push(format!("{}: {}", name, go_fs_err(&joined, &e)));
                continue;
            }
        };
        let langs: Option<RtlLangs> = match extract_translations(root, &b) {
            Ok(l) => Some(l),
            Err(e) => {
                if !e.contains("no translations") {
                    failures.push(format!("{}: {}", name, e.split('\n').next().unwrap_or("")));
                    continue;
                }
                None
            }
        };
        let bad = if langs.is_none() || name == "drawer-rtl" {
            // drawer's upstream translations carry `locale:` hints the
            // shadless dictionary has no slot for — upstream treats it as
            // non-translation-bearing for our purposes, same as the
            // tombstoned calendar/shimmer
            Some("no `translations` object literal")
        } else if langs
            .as_ref()
            .expect("some")
            .dir
            .get("ar")
            .map_or("", |s| s.as_str())
            .is_empty()
        {
            Some("no Arabic dictionary")
        } else {
            None
        };
        if let Some(bad) = bad {
            if shipped(&name) {
                failures.push(format!("{}: {} in {}", name, bad, file));
            } else {
                skipped.push(name.clone());
            }
            continue;
        }
        dict_order.push(name.clone());
        dicts.insert(name, langs.expect("checked above"));
    }

    if !failures.is_empty() {
        for f in &failures {
            let i = f.find(": ").expect("failures are name: message");
            eprintln!("FAIL [{}]: {}", &f[..i], &f[i + 2..]);
        }
        eprintln!(
            "FAIL  rtl-dict ({}/{} dictionaries unreadable) — nothing written; {} keeps its previous contents",
            failures.len(),
            files.len(),
            RTL_DICT_OUT
        );
        return 1;
    }

    let mut out = String::new();
    out.push('{');
    for (i, name) in dict_order.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let nb = json_string(name);
        out.push_str("\n ");
        out.push_str(&nb);
        out.push_str(": ");
        write_langs_json(&mut out, &dicts[name], 1);
    }
    out.push_str("\n}\n");
    if let Err(e) = std::fs::write(root.join(RTL_DICT_OUT), out.as_bytes()) {
        eprintln!("rtl-dict: {}", go_fs_err(RTL_DICT_OUT, &e));
        return 1;
    }
    print!(
        "rtl-dict: {} dictionaries lifted from {} to {}",
        dict_order.len(),
        RTL_DICT_EXAMPLES,
        RTL_DICT_OUT
    );
    if !skipped.is_empty() {
        print!(
            " ({} upstream -rtl examples carry no dictionary and are for components we do not ship: {})",
            skipped.len(),
            skipped.join(", ")
        );
    }
    println!();
    0
}

/// writeLangsJSON emits {"en":{"dir":…,"values":{…}},…} preserving source
/// order at indent level `depth` — the layout the JS version wrote.
fn write_langs_json(b: &mut String, l: &RtlLangs, depth: usize) {
    let ind = " ".repeat(depth);
    b.push('{');
    for (i, name) in l.names.iter().enumerate() {
        if i > 0 {
            b.push(',');
        }
        b.push('\n');
        b.push_str(&ind);
        b.push(' ');
        let kb = json_string(name);
        let db = json_string(&l.dir[name]);
        b.push_str(&kb);
        b.push_str(": {\n");
        b.push_str(&ind);
        b.push_str("  \"dir\": ");
        b.push_str(&db);
        b.push_str(",\n");
        b.push_str(&ind);
        b.push_str("  \"values\": {");
        let v = &l.values[name];
        if v.keys.is_empty() {
            b.push('}'); // empty object stays one line, as JSON.stringify emits it
        } else {
            for (j, k) in v.keys.iter().enumerate() {
                if j > 0 {
                    b.push(',');
                }
                b.push('\n');
                b.push_str(&ind);
                b.push_str("   ");
                let kb2 = json_string(k);
                let vb = json_string(&v.vals[k]);
                b.push_str(&kb2);
                b.push_str(": ");
                b.push_str(&vb);
            }
            b.push('\n');
            b.push_str(&ind);
            b.push_str("  }");
        }
        b.push('\n');
        b.push_str(&ind);
        b.push_str(" }"); // one space — matches JSON.stringify's pretty layout between values
    }
    b.push('\n');
    b.push_str(&ind);
    b.push('}');
}

// ---------------------------------------------------------------------------
// extractTranslations: esbuild to strip TS syntax, then a scanner walk of
// the `translations` identifier (exact name — \b, or a rename like
// `translationsRenamed` would still match). Values may be StringLiteral or a
// single-quasi TemplateLiteral; both surface as strings.

static TRANSLATIONS_ANCHOR: LazyLock<Regex> = LazyLock::new(|| {
    // Go \b is an ASCII word boundary; (?-u:\b) is the Rust equivalent.
    Regex::new(r"const translations(?-u:\b)").expect("static regex")
});

fn extract_translations(root: &Path, src: &str) -> Result<RtlLangs, String> {
    let js = match esbuild_tsx(root, src) {
        Ok(js) => js,
        Err(e) => {
            // esbuild_tsx wraps the CLI stderr as "esbuild: <stderr>"; Go's
            // in-process api.Transform reports Errors[0].Text — the bare
            // message of the first error line, without the CLI's decoration.
            let stderr = e.strip_prefix("esbuild: ").unwrap_or(&e);
            return Err(format!("esbuild: {}", esbuild_error_text(stderr)));
        }
    };
    let m = TRANSLATIONS_ANCHOR.find(&js);
    let Some(m) = m else {
        return Err("no translations object".to_string());
    };
    // find the '=' after `const translations` (TS type annotation is already
    // stripped by esbuild)
    let rest = &js[m.end()..];
    let Some(j) = rest.find('=') else {
        return Err("no translations assignment".to_string());
    };
    let mut j = m.end() + j + 1;
    let bytes = js.as_bytes();
    while j < bytes.len() && (bytes[j] == b' ' || bytes[j] == b'\n' || bytes[j] == b'\t') {
        j += 1;
    }
    if j >= bytes.len() || bytes[j] != b'{' {
        let hi = (j + 10).min(bytes.len());
        return Err(format!(
            "translations is not an object literal (found {})",
            go_quote_bytes(&bytes[j..hi])
        ));
    }
    let (langs, _, err) = parse_lang_object(&tsx::string_literals(&js), &js, j);
    match err {
        Some(e) => Err(e),
        None => Ok(langs.expect("no error means langs")),
    }
}

/// parseLangObject parses `{ lang: { dir: "…", values: {…} }, … }` —
/// open points at the '{'. The lang NAME is the object's key; its dir and
/// insertion-ordered values come from the entry.
fn parse_lang_object(
    spans: &[Span],
    src: &str,
    open: usize,
) -> (Option<RtlLangs>, usize, Option<String>) {
    let mut out = RtlLangs {
        names: Vec::new(),
        dir: HashMap::new(),
        values: HashMap::new(),
    };
    let mut i = open + 1;
    loop {
        i = skip_ws(src, i);
        if i >= src.len() {
            return (
                None,
                0,
                Some("unterminated translations object".to_string()),
            );
        }
        if src.as_bytes()[i] == b'}' {
            return (Some(out), i + 1, None);
        }
        let k0 = i;
        while i < src.len() && is_ident(src.as_bytes()[i]) {
            i += 1;
        }
        if i == k0 {
            let hi = (i + 20).min(src.len());
            return (
                None,
                0,
                Some(format!(
                    "expected identifier key at {}: {}",
                    i,
                    go_quote_bytes(&src.as_bytes()[i..hi])
                )),
            );
        }
        let name = src[k0..i].to_string();
        i = skip_ws(src, i);
        if i >= src.len() || src.as_bytes()[i] != b':' {
            return (
                None,
                0,
                Some(format!("expected ':' after {}", go_quote(&name))),
            );
        }
        i = skip_ws(src, i + 1);
        if i >= src.len() || src.as_bytes()[i] != b'{' {
            return (
                None,
                0,
                Some(format!("expected {{…}} as value of {}", go_quote(&name))),
            );
        }
        let (dir, vals, next, err) = parse_lang_entry(spans, src, i);
        if let Some(e) = err {
            return (None, 0, Some(e));
        }
        out.names.push(name.clone());
        out.dir.insert(name.clone(), dir);
        out.values.insert(name, vals);
        i = skip_ws(src, next);
        if i < src.len() && src.as_bytes()[i] == b',' {
            i += 1;
        }
    }
}

/// parseLangEntry returns the dir and the ordered values. Extra keys (locale,
/// …) are read and discarded — an unhandled value KIND is still an error.
fn parse_lang_entry(
    spans: &[Span],
    src: &str,
    open: usize,
) -> (String, RtlValues, usize, Option<String>) {
    let mut dir = "ltr".to_string();
    let mut vals = RtlValues {
        keys: Vec::new(),
        vals: HashMap::new(),
    };
    let mut i = open + 1;
    loop {
        i = skip_ws(src, i);
        if i >= src.len() {
            return (
                String::new(),
                vals,
                0,
                Some("unterminated lang entry".to_string()),
            );
        }
        if src.as_bytes()[i] == b'}' {
            return (dir, vals, i + 1, None);
        }
        let k0 = i;
        while i < src.len() && is_ident(src.as_bytes()[i]) {
            i += 1;
        }
        let key = src[k0..i].to_string();
        i = skip_ws(src, i);
        // NOTE: unguarded read, exactly as in Go — an actually-unterminated
        // `{ dir` (no trailing char at all) panics here in both ports.
        if src.as_bytes()[i] != b':' {
            return (
                String::new(),
                vals,
                0,
                Some(format!(
                    "expected ':' after {} in lang entry",
                    go_quote(&key)
                )),
            );
        }
        i = skip_ws(src, i + 1);
        if key == "dir" {
            let (lit, next, err) = read_string_at(spans, src, i);
            if let Some(e) = err {
                return (String::new(), vals, 0, Some(format!("dir: {}", e)));
            }
            dir = lit;
            i = next;
        } else if key == "values" {
            if src.as_bytes()[i] != b'{' {
                return (
                    String::new(),
                    vals,
                    0,
                    Some("values not an object".to_string()),
                );
            }
            let (v, next, err) = parse_values(spans, src, i);
            if let Some(e) = err {
                return (String::new(), vals, 0, Some(e));
            }
            vals = v;
            i = next;
        } else if src.as_bytes()[i] == b'"' || src.as_bytes()[i] == b'\'' {
            let (_, next, err) = read_string_at(spans, src, i);
            if let Some(e) = err {
                return (
                    String::new(),
                    vals,
                    0,
                    Some(format!("extra key {}: {}", go_quote(&key), e)),
                );
            }
            i = next;
        } else {
            return (
                String::new(),
                vals,
                0,
                Some(format!(
                    "extra key {} has an unhandled value kind",
                    go_quote(&key)
                )),
            );
        }
        i = skip_ws(src, i);
        if i < src.len() && src.as_bytes()[i] == b',' {
            i += 1;
        }
    }
}

fn parse_values(spans: &[Span], src: &str, open: usize) -> (RtlValues, usize, Option<String>) {
    let mut vals = RtlValues {
        keys: Vec::new(),
        vals: HashMap::new(),
    };
    let mut i = open + 1;
    loop {
        i = skip_ws(src, i);
        if i >= src.len() {
            return (vals, 0, Some("unterminated values object".to_string()));
        }
        if src.as_bytes()[i] == b'}' {
            return (vals, i + 1, None);
        }
        let k0 = i;
        // key may be identifier or quoted string
        let key: String;
        if src.as_bytes()[i] == b'"' || src.as_bytes()[i] == b'\'' {
            let (lit, next, err) = read_string_at(spans, src, i);
            if let Some(e) = err {
                return (vals, 0, Some(e));
            }
            key = lit;
            i = next;
        } else {
            while i < src.len() && is_ident(src.as_bytes()[i]) {
                i += 1;
            }
            key = src[k0..i].to_string();
        }
        i = skip_ws(src, i);
        if i >= src.len() || src.as_bytes()[i] != b':' {
            return (
                vals,
                0,
                Some(format!(
                    "expected ':' after value key {} at {}",
                    go_quote(&key),
                    k0
                )),
            );
        }
        i = skip_ws(src, i + 1);
        let (lit, next, err) = read_string_at(spans, src, i);
        if let Some(e) = err {
            return (vals, 0, Some(format!("value of {}: {}", go_quote(&key), e)));
        }
        if !vals.vals.contains_key(&key) {
            vals.keys.push(key.clone());
        }
        vals.vals.insert(key, lit);
        i = skip_ws(src, next);
        if i < src.len() && src.as_bytes()[i] == b',' {
            i += 1;
        }
    }
}

/// readStringAt decodes the string literal whose span STARTS at i (i.e. the
/// opening quote). The span list is position-keyed; we index it lazily.
fn read_string_at(spans: &[Span], src: &str, i: usize) -> (String, usize, Option<String>) {
    for sp in spans {
        if sp.start == i + 1 {
            // decode escapes the way JS would
            match decode_js_string(sp.content(src)) {
                Ok(dec) => return (dec, sp.end + 1, None),
                Err(e) => return (String::new(), 0, Some(e)),
            }
        }
    }
    let hi = (i + 24).min(src.len());
    (
        String::new(),
        0,
        Some(format!(
            "no string literal at {}: {}",
            i,
            go_quote_bytes(&src.as_bytes()[i..hi])
        )),
    )
}

fn skip_ws(s: &str, mut i: usize) -> usize {
    let b = s.as_bytes();
    while i < b.len() && (b[i] == b' ' || b[i] == b'\n' || b[i] == b'\t' || b[i] == b'\r') {
        i += 1;
    }
    i
}

fn is_ident(c: u8) -> bool {
    (b'a'..=b'z').contains(&c)
        || (b'A'..=b'Z').contains(&c)
        || c == b'_'
        || c == b'$'
        || (b'0'..=b'9').contains(&c)
}

/// decodeJSString decodes the escape sequences that appear in this corpus.
/// It is deliberately strict: anything it cannot decode is an ERROR, so a
/// new escape upstream introduces fails the build rather than passing
/// through mangled.
fn decode_js_string(raw: &str) -> Result<String, String> {
    if !raw.contains('\\') {
        return Ok(raw.to_string());
    }
    let b = raw.as_bytes();
    let mut out = String::with_capacity(raw.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'\\' {
            out.push(b[i] as char);
            i += 1;
            continue;
        }
        if i + 1 >= b.len() {
            return Err("trailing backslash".to_string());
        }
        match b[i + 1] {
            b'n' => out.push('\n'),
            b't' => out.push('\t'),
            b'r' => out.push('\r'),
            b'"' => out.push('"'),
            b'\'' => out.push('\''),
            b'\\' => out.push('\\'),
            b'`' => out.push('`'),
            b'u' => {
                // \uXXXX (4 hex) — corpus never uses \u{…}
                if i + 5 >= b.len() {
                    return Err("short \\u escape".to_string());
                }
                let hi = (i + 6).min(b.len());
                let window = &b[i + 2..hi];
                // Go's Sscanf %04x over this exact 4-byte window: skip
                // leading whitespace, optional sign, then hex digits until
                // the first non-hex byte. Zero hex digits is an error.
                let mut j = 0;
                while j < window.len()
                    && matches!(window[j], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
                {
                    j += 1;
                }
                let neg = if j < window.len() && window[j] == b'-' {
                    j += 1;
                    true
                } else {
                    if j < window.len() && window[j] == b'+' {
                        j += 1;
                    }
                    false
                };
                let k0 = j;
                while j < window.len() && window[j].is_ascii_hexdigit() {
                    j += 1;
                }
                if j == k0 {
                    return Err(format!("bad \\u escape {}", go_quote_bytes(&b[i..hi])));
                }
                let hex = std::str::from_utf8(&window[k0..j]).expect("hex digits are ASCII");
                let mut cp = u32::from_str_radix(hex, 16).expect("hex digits");
                if neg {
                    cp = cp.wrapping_neg();
                }
                // Go writes rune(cp); an invalid rune becomes U+FFFD
                out.push(char::from_u32(cp).unwrap_or('\u{FFFD}'));
                // Go bug-compat: i += 6 unconditionally — any non-hex bytes
                // inside the 4-byte window are consumed too (probed against
                // the Go binary: "a\u12zzb" decodes to "a\x12b", not
                // "a\x12zzb"). Do not "fix" this.
                i += 6;
                continue;
            }
            other => {
                return Err(format!("unsupported escape \\{}", other as char));
            }
        }
        i += 2;
    }
    Ok(out)
}

/// esbuild_error_text extracts the bare message of the first error from the
/// CLI's decorated stderr, matching Go's in-process api.Transform
/// Errors[0].Text. The CLI prints errors as `✘ [ERROR] <message>` followed by
/// a location block; warnings (▲ [WARNING]) are skipped.
fn esbuild_error_text(stderr: &str) -> String {
    for line in stderr.lines() {
        if let Some(rest) = line.strip_prefix("\u{2718} [ERROR] ") {
            return rest.to_string();
        }
    }
    stderr.lines().next().unwrap_or("").to_string()
}

/// go_fs_err renders an io::Error the way Go's os package does in the
/// rtl-dict error paths: `open <path>: <errno text>` with the path exactly as
/// the Go code passed it (relative, unjoined) and the lowercase errno string
/// (fsutil::go_strerror — the fabricated "I/O error" fallback is gone:
/// out-of-table errnos now get Go's prefixed shape like every other tool).
fn go_fs_err(path: &str, e: &std::io::Error) -> String {
    crate::fsutil::go_path_err("open", path, e)
}

/// go_quote mirrors Go's %q on a string: printable runes pass through,
/// quote/backslash and the C0 controls are escaped, other non-printables
/// become \xNN. (Only reachable through error messages; the corpus is ASCII.)
fn go_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{b}' => out.push_str("\\v"),
            c if (c as u32) < 0x20 || c == '\u{7f}' => {
                out.push_str(&format!("\\x{:02x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// go_quote over a raw byte window (Go slices strings by byte; invalid UTF-8
/// bytes print as \xNN).
fn go_quote_bytes(b: &[u8]) -> String {
    match std::str::from_utf8(b) {
        Ok(s) => go_quote(s),
        Err(_) => {
            let mut out = String::with_capacity(b.len() + 2);
            out.push('"');
            for &c in b {
                if c < 0x80 {
                    match c {
                        b'"' => out.push_str("\\\""),
                        b'\\' => out.push_str("\\\\"),
                        b'\n' => out.push_str("\\n"),
                        b'\r' => out.push_str("\\r"),
                        b'\t' => out.push_str("\\t"),
                        c if c < 0x20 || c == 0x7f => {
                            out.push_str(&format!("\\x{:02x}", c));
                        }
                        c => out.push(c as char),
                    }
                } else {
                    out.push_str(&format!("\\x{:02x}", c));
                }
            }
            out.push('"');
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed src/registry/rtl-translations.json was written by the JS
    /// rtl-dict; the port must produce byte-identical output or the whole RTL
    /// demo family shifts underneath gates that hash it. (Go's
    /// TestUnitRtlDictParity is a subprocess test; here the port runs
    /// in-process against the real tree and the written file is byte-compared
    /// to the committed one, then the original is restored.)
    #[test]
    fn rtl_dict_parity_on_real_tree() {
        let root = crate::tree_root();
        let out_path = root.join(RTL_DICT_OUT);
        if !root.join(RTL_DICT_EXAMPLES).exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (no pinned upstream checkout)");
            }
            eprintln!("skip: no pinned upstream checkout");
            return;
        }
        let committed = match std::fs::read(&out_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("skip: {}", e);
                return;
            }
        };
        // restore the committed bytes no matter how the run ends
        struct Restore {
            path: std::path::PathBuf,
            bytes: Vec<u8>,
        }
        impl Drop for Restore {
            fn drop(&mut self) {
                let _ = std::fs::write(&self.path, &self.bytes);
            }
        }
        let _guard = Restore {
            path: out_path.clone(),
            bytes: committed.clone(),
        };
        let rc = run_rtl_dict_at(&root);
        assert_eq!(rc, 0, "rtl-dict must exit 0 on the real tree");
        let got = std::fs::read(&out_path).expect("rtl-dict wrote the file");
        if got != committed {
            let first = got.iter().zip(committed.iter()).position(|(a, b)| a != b);
            panic!(
                "rtl-translations.json diverges from the committed bytes (first divergence at byte {:?}, got {:?} want {:?})",
                first,
                got.get(first.unwrap_or(0)..(first.unwrap_or(0) + 40).min(got.len()))
                    .map(String::from_utf8_lossy),
                committed
                    .get(first.unwrap_or(0)..(first.unwrap_or(0) + 40).min(committed.len()))
                    .map(String::from_utf8_lossy),
            );
        }
    }

    /// decodeJSString: the success path plus every error branch the real
    /// .upstream/*-rtl.tsx corpus never exercises (it has no unterminated
    /// escapes, no \z-style unsupported escapes, no truncated \u).
    #[test]
    fn decode_js_string_cases() {
        let cases: Vec<(&str, &str, &str, bool)> = vec![
            (
                "no backslash: fast path returns raw unchanged",
                "plain text",
                "plain text",
                false,
            ),
            ("newline", "a\\nb", "a\nb", false),
            ("tab", "a\\tb", "a\tb", false),
            ("carriage return", "a\\rb", "a\rb", false),
            ("escaped double quote", "a\\\"b", "a\"b", false),
            ("escaped single quote", "a\\'b", "a'b", false),
            ("escaped backslash", "a\\\\b", "a\\b", false),
            ("escaped backtick", "a\\`b", "a`b", false),
            ("\\u escape", "a\\u00e9b", "aéb", false),
            ("trailing backslash", "a\\", "", true),
            ("short \\u escape", "a\\u12", "", true),
            ("unsupported escape", "a\\zb", "", true),
            // Go's Sscanf %04x over the 4-byte window: leading whitespace is
            // skipped, a sign is accepted, hex stops at the first non-hex
            // byte, and zero hex digits is an error. The loop then advances
            // i += 6 unconditionally, so any non-hex bytes inside the window
            // are consumed too.
            ("\\u with leading space", "a\\u 123b", "a\u{123}b", false),
            ("\\u with sign", "a\\u-123b", "a\u{FFFD}b", false),
            ("\\u stops at non-hex", "a\\u12zzb", "a\u{12}b", false),
            ("\\u no hex digits", "a\\uzzzzb", "", true),
        ];
        for (name, input, want, want_err) in cases {
            let got = decode_js_string(input);
            if want_err {
                assert!(
                    got.is_err(),
                    "{}: decode_js_string({:?}) = {:?}, want an error",
                    name,
                    input,
                    got
                );
                continue;
            }
            assert_eq!(
                got,
                Ok(want.to_string()),
                "{}: decode_js_string({:?})",
                name,
                input
            );
        }
    }

    /// parseLangObject: happy path (two languages, one carrying a discarded
    /// extra key) plus the malformed-input branches the real corpus never
    /// reaches.
    #[test]
    fn parse_lang_object_cases() {
        let src = "{ en: { dir: \"ltr\", locale: \"en-US\", values: { title: \"Title\", greeting: \"Hi\" } }, ar: { dir: \"rtl\", values: {} } }";
        let (langs, next, err) = parse_lang_object(&tsx::string_literals(src), src, 0);
        assert!(err.is_none(), "unexpected error: {:?}", err);
        let langs = langs.expect("langs");
        assert_eq!(
            next,
            src.len(),
            "next = {}, want {} (end of object)",
            next,
            src.len()
        );
        assert_eq!(langs.names, vec!["en", "ar"]);
        assert_eq!(langs.dir["en"], "ltr");
        assert_eq!(langs.dir["ar"], "rtl");
        let en_vals = &langs.values["en"];
        assert_eq!(en_vals.keys, vec!["title", "greeting"]);
        assert_eq!(en_vals.vals["title"], "Title");
        assert_eq!(en_vals.vals["greeting"], "Hi");
        assert!(
            langs.values["ar"].keys.is_empty(),
            "ar values should be empty"
        );

        let padded = format!("{{,{}", " ".repeat(25));
        let err_cases: Vec<(&str, &str, &str)> = vec![
            (
                "unterminated object",
                "{",
                "unterminated translations object",
            ),
            // padded well past the error's own (unguarded) src[i:i+20] slice
            ("missing key", &padded, "expected identifier key"),
            (
                "missing colon after key",
                "{ en }",
                "expected ':' after \"en\"",
            ),
            (
                "value not an object literal",
                "{ en: \"x\" }",
                "as value of \"en\"",
            ),
        ];
        for (name, src, want_substr) in err_cases {
            let (_, _, err) = parse_lang_object(&tsx::string_literals(src), src, 0);
            let err = err.expect("error");
            assert!(
                err.contains(want_substr),
                "{}: parse_lang_object({:?}) error = {:?}, want substring {:?}",
                name,
                src,
                err,
                want_substr
            );
        }
    }

    /// parseValues: happy path (insertion order, quoted keys, duplicate key
    /// keeps its first position but takes the last value) plus its two error
    /// branches.
    #[test]
    fn parse_values_cases() {
        let src = "{ a: \"x\", b: \"y\", a: \"z\" }";
        let (vals, next, err) = parse_values(&tsx::string_literals(src), src, 0);
        assert!(err.is_none(), "unexpected error: {:?}", err);
        assert_eq!(next, src.len(), "next = {}, want {}", next, src.len());
        assert_eq!(
            vals.keys,
            vec!["a", "b"],
            "duplicate key must not append twice"
        );
        assert_eq!(vals.vals["a"], "z", "last write wins");
        assert_eq!(vals.vals["b"], "y");

        let quoted_src = "{ \"weird key\": \"v\" }";
        let (q_vals, _, err) = parse_values(&tsx::string_literals(quoted_src), quoted_src, 0);
        assert!(err.is_none(), "unexpected error on quoted key: {:?}", err);
        assert_eq!(q_vals.vals["weird key"], "v");

        let (_, _, err) = parse_values(&[], "{", 0);
        assert!(
            err.as_ref()
                .expect("error")
                .contains("unterminated values object"),
            "unterminated: error = {:?}",
            err
        );
        let (_, _, err) = parse_values(&[], "{ a }", 0);
        assert!(
            err.as_ref()
                .expect("error")
                .contains("expected ':' after value key \"a\""),
            "missing colon: error = {:?}",
            err
        );
        // a quoted key with no matching span (readStringAt's own error, passed
        // through): spans intentionally empty even though the source has one.
        let (_, _, err) = parse_values(&[], "{ \"a\": \"x\" }", 0);
        assert!(
            err.as_ref()
                .expect("error")
                .contains("no string literal at"),
            "quoted key, no span: error = {:?}",
            err
        );
    }
}
