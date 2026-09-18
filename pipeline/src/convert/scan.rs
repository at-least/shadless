//! Text-scanning utilities over the downgraded JS text and the masked code
//! regions of the original — the port of convert.go's scanning half.
//!
//! Everything is byte-indexed over ASCII delimiters, exactly like the Go
//! original; all Go regexes here are RE2-safe and were translated preserving
//! ASCII semantics and capture-group numbers.

use regex::Regex;
use std::sync::OnceLock;

pub fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("convert regex compiles")
}

// ------------------------------------------------------------- masking

/// If a masked token (string, template, comment, regex) starts at s[i],
/// return the index just past it; else 0.
pub fn cv_mask_end(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    match b[i] {
        b'"' | b'\'' => cv_skip_string(s, i),
        b'`' => cv_skip_template(s, i),
        b'/' => {
            if i + 1 < b.len() {
                match b[i + 1] {
                    b'/' => return cv_skip_line_comment(s, i),
                    b'*' => return cv_skip_block_comment(s, i),
                    _ => {}
                }
                if cv_regex_allowed(s, i) {
                    return cv_skip_regex(s, i);
                }
            }
            0
        }
        _ => 0,
    }
}

pub fn cv_skip_string(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    let q = b[i];
    let mut i = i + 1;
    while i < b.len() {
        if b[i] == b'\\' {
            i += 2;
            continue;
        }
        if b[i] == q || b[i] == b'\n' {
            return i + 1;
        }
        i += 1;
    }
    i
}

fn cv_skip_template(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    let mut i = i + 1;
    while i < b.len() {
        let c = b[i];
        if c == b'\\' {
            i += 2;
            continue;
        }
        if c == b'`' {
            return i + 1;
        }
        if c == b'$' && i + 1 < b.len() && b[i + 1] == b'{' {
            i = cv_skip_braces(s, i + 2, 1);
            continue;
        }
        i += 1;
    }
    i
}

/// Consumes until `depth` open braces are closed, mask-aware.
pub fn cv_skip_braces(s: &str, mut i: usize, mut depth: i32) -> usize {
    let b = s.as_bytes();
    while i < b.len() && depth > 0 {
        let e = cv_mask_end(s, i);
        if e > 0 {
            i = e;
            continue;
        }
        match b[i] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    i
}

fn cv_skip_line_comment(s: &str, mut i: usize) -> usize {
    let b = s.as_bytes();
    while i < b.len() && b[i] != b'\n' {
        i += 1;
    }
    i
}

fn cv_skip_block_comment(s: &str, mut i: usize) -> usize {
    let b = s.as_bytes();
    i += 2;
    while i + 1 < b.len() && !(b[i] == b'*' && b[i + 1] == b'/') {
        i += 1;
    }
    i + 2
}

pub fn cv_regex_allowed(s: &str, i: usize) -> bool {
    let b = s.as_bytes();
    for j in (0..i).rev() {
        let c = b[j];
        if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
            continue;
        }
        return matches!(
            c,
            b',' | b';'
                | b':'
                | b'('
                | b'['
                | b'{'
                | b'='
                | b'<'
                | b'>'
                | b'&'
                | b'|'
                | b'^'
                | b'!'
                | b'~'
                | b'?'
                | b'-'
        );
    }
    true
}

fn cv_skip_regex(s: &str, mut i: usize) -> usize {
    let b = s.as_bytes();
    i += 1;
    let mut in_class = false;
    while i < b.len() {
        let c = b[i];
        if c == b'\\' {
            i += 2;
            continue;
        }
        if c == b'[' {
            in_class = true;
        } else if c == b']' {
            in_class = false;
        } else if c == b'/' && !in_class {
            i += 1;
            while i < b.len() && (b[i].is_ascii_lowercase() || b[i].is_ascii_uppercase()) {
                i += 1;
            }
            return i;
        } else if c == b'\n' {
            return i;
        }
        i += 1;
    }
    i
}

/// Splits s on top-level commas (mask-aware; () [] {} all nest).
pub fn cv_split_top(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let (mut depth, mut start) = (0i32, 0usize);
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let e = cv_mask_end(s, i);
        if e > 0 {
            i = e;
            continue;
        }
        match b[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b',' => {
                if depth == 0 {
                    out.push(s[start..i].to_string());
                    start = i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out.push(s[start..].to_string());
    out
}

/// At a code position, return the candidate token's length for a hit, else 0.
pub type CvTopOps = fn(&str, usize) -> usize;

pub fn cv_ops_q(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    if b[i] != b'?' || i + 1 >= b.len() {
        return 0;
    }
    match b[i + 1] {
        b'.' | b'?' => 0,
        _ => 1,
    }
}

pub fn cv_ops_qor_logical(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    if b[i] == b'?' {
        if i + 1 >= b.len() {
            return 1;
        }
        if b[i + 1] == b'.' {
            return 0;
        }
        if b[i + 1] == b'?' && i + 2 < b.len() && b[i + 2] == b'=' {
            return 0;
        }
        return 1;
    }
    if i + 1 < b.len() && (&s[i..i + 2] == "&&" || &s[i..i + 2] == "||") {
        if i + 2 < b.len() && b[i + 2] == b'=' {
            return 0;
        }
        return 2;
    }
    0
}

pub fn cv_ops_or(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    if i + 1 < b.len() && (&s[i..i + 2] == "||" || &s[i..i + 2] == "??") {
        if i + 2 < b.len() && b[i + 2] == b'=' {
            return 0;
        }
        return 2;
    }
    0
}

pub fn cv_ops_and(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    if i + 1 < b.len() && &s[i..i + 2] == "&&" {
        if i + 2 < b.len() && b[i + 2] == b'=' {
            return 0;
        }
        return 2;
    }
    0
}

/// Finds the first top-level position matching ops. Masking as everywhere;
/// {} [] nest; parens nest ONLY when they are call invocations (identifier or
/// ]/) before '(') — grouping parens stay transparent, mirroring babel's AST
/// where parens are not nodes. An => arrow with an expression body masks its
/// body: it ends at a ',' or a closer dropping below the arrow's depth.
pub fn cv_first_top(s: &str, ops: CvTopOps) -> isize {
    let b = s.as_bytes();
    let (mut depth, mut arrow_depth) = (0i32, -1i32);
    let mut paren_stack: Vec<bool> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let e = cv_mask_end(s, i);
        if e > 0 {
            i = e;
            continue;
        }
        let c = b[i];
        if c == b'(' {
            let mut call = false;
            let mut j = i as isize - 1;
            while j >= 0 {
                let ch = b[j as usize];
                if ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r' {
                    j -= 1;
                    continue;
                }
                call = ch.is_ascii_lowercase()
                    || ch.is_ascii_uppercase()
                    || ch.is_ascii_digit()
                    || ch == b'_'
                    || ch == b'$'
                    || ch == b')'
                    || ch == b']';
                break;
            }
            paren_stack.push(call);
            if call {
                depth += 1;
            }
        } else if c == b')' {
            if let Some(top) = paren_stack.pop() {
                if top {
                    depth -= 1;
                }
            }
            if arrow_depth >= 0 && depth < arrow_depth {
                arrow_depth = -1;
            }
        } else if c == b'[' || c == b'{' {
            depth += 1;
        } else if c == b']' || c == b'}' {
            depth -= 1;
            if arrow_depth >= 0 && depth < arrow_depth {
                arrow_depth = -1;
            }
        } else if c == b',' && arrow_depth >= 0 && depth == arrow_depth {
            arrow_depth = -1;
        } else if c == b'=' && i + 1 < b.len() && b[i + 1] == b'>' {
            let mut k = i + 2;
            while k < b.len()
                && (b[k] == b' ' || b[k] == b'\t' || b[k] == b'\n' || b[k] == b'\r')
            {
                k += 1;
            }
            if k < b.len() && b[k] != b'{' {
                arrow_depth = depth;
            }
            i += 2;
            continue;
        }
        if depth == 0 && arrow_depth < 0 && ops(s, i) > 0 {
            return i as isize;
        }
        i += 1;
    }
    -1
}

/// The ':' matching a '?' at q (nested ternaries and masked tokens respected).
pub fn cv_find_ternary_colon(s: &str, q: usize) -> isize {
    let b = s.as_bytes();
    let (mut nest, mut depth) = (0i32, 0i32);
    let mut i = q + 1;
    while i < b.len() {
        let e = cv_mask_end(s, i);
        if e > 0 {
            i = e;
            continue;
        }
        match b[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b'?' => {
                // optional chaining (`user?.slug`) is not a ternary `?`:
                // counting it nested made the colon search skip the REAL
                // colon and the whole expression degraded to "other"
                if b.get(i + 1) == Some(&b'.') {
                    i += 1;
                } else {
                    nest += 1;
                }
            }
            b':' => {
                if depth == 0 && nest == 0 {
                    return i as isize;
                }
                nest -= 1;
            }
            _ => {}
        }
        i += 1;
    }
    -1
}

// --------------------------------------------------------------- expr model

#[derive(Clone, Debug, Default)]
pub struct ExprNode {
    pub kind: String, // str tpl tplMulti ident bool nul num obj call cond logical other
    pub str: String,  // str: value; tpl: cooked; num: raw literal text
    pub ident: String,
    pub args: Vec<String>,
    pub obj: Vec<CvProp>,
    pub test: String,
    pub cons: String,
    pub alt: String,
    pub op: String,
    pub left: String,
    pub right: String,
}

#[derive(Clone, Debug, Default)]
pub struct CvProp {
    pub key: String,
    pub val: String,
    pub quoted: bool, // "key": — babel prop.key.name is undefined for these
    pub shorthand: bool,
    pub spread: bool,
}

pub fn cv_ident_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"^[A-Za-z_$][A-Za-z0-9_$]*$"))
}
pub fn cv_member_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"^[A-Za-z_$][A-Za-z0-9_$-]*(?:\.[A-Za-z0-9_$-]+)+$"))
}
fn cv_num_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"^(?:0[xXbBoO][0-9a-fA-F_]+|[0-9][0-9_]*(?:\.[0-9_]+)?(?:[eE][+-]?[0-9]+)?|\.[0-9_]+)"))
}
fn cv_callee_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"^[A-Za-z_$][A-Za-z0-9_$]*"))
}

pub fn cv_parse_expr(text: &str) -> ExprNode {
    let mut t = text.trim();
    let mut n = ExprNode {
        kind: "other".to_string(),
        ..Default::default()
    };
    // babel has no paren nodes — a fully-wrapping paren group is transparent
    while t.starts_with('(') {
        if let Some(e) = cv_match_bracket(t, 0, b'(', b')') {
            if e == t.len() - 1 {
                t = t[1..e].trim();
                continue;
            }
        }
        break;
    }
    if t.is_empty() {
        return n;
    }
    let b = t.as_bytes();
    if b[0] == b'"' || b[0] == b'\'' {
        n.kind = "str".to_string();
        n.str = cv_unquote(t);
        return n;
    }
    if b[0] == b'`' {
        if cv_template_has_subst(t) {
            n.kind = "tplMulti".to_string();
        } else {
            n.kind = "str".to_string();
            n.str = cv_template_cook(t);
        }
        return n;
    }
    if t == "true" || t == "false" {
        n.kind = "bool".to_string();
        return n;
    }
    if t == "null" {
        n.kind = "nul".to_string();
        return n;
    }
    if cv_ident_re().is_match(t) {
        n.kind = "ident".to_string();
        n.ident = t.to_string();
        return n;
    }
    if let Some(m) = cv_num_re().find(t) {
        if m.as_str() == t {
            n.kind = "num".to_string();
            n.str = m.as_str().to_string();
            return n;
        }
    }
    // conditional before call/logical: `f(x) ? a : b` is a cond (call parens
    // are masked, so its '?' is top-level); `f(a ? b : c)` is a call (its '?'
    // hides inside the masked invocation).
    let q = cv_first_top(t, cv_ops_q);
    if q >= 0 {
        let colon = cv_find_ternary_colon(t, q as usize);
        if colon >= 0 {
            n.kind = "cond".to_string();
            n.test = t[..q as usize].to_string();
            n.cons = t[q as usize + 1..colon as usize].to_string();
            n.alt = t[colon as usize + 1..].to_string();
            return n;
        }
    }
    // logical root: the first operator of the LOWEST precedence tier — || / ??
    // before && — so `x && y || "fb"` splits at || with right "fb".
    let pos = cv_first_top(t, cv_ops_or);
    if pos >= 0 {
        n.kind = "logical".to_string();
        n.op = t[pos as usize..pos as usize + 2].to_string();
        n.left = t[..pos as usize].to_string();
        n.right = t[pos as usize + 2..].to_string();
        return n;
    }
    let pos = cv_first_top(t, cv_ops_and);
    if pos >= 0 {
        n.kind = "logical".to_string();
        n.op = "&&".to_string();
        n.left = t[..pos as usize].to_string();
        n.right = t[pos as usize + 2..].to_string();
        return n;
    }
    let name = cv_callee_name(t);
    if !name.is_empty() {
        let nlen = name.len();
        if let Some(e) = cv_match_bracket(t, nlen, b'(', b')') {
            if t[e + 1..].trim().is_empty() {
                n.kind = "call".to_string();
                n.ident = name;
                n.args = cv_split_top(&t[nlen + 1..e]);
                return n;
            }
        }
    }
    if b[0] == b'{' {
        if let Some(e) = cv_match_bracket(t, 0, b'{', b'}') {
            if e == t.len() - 1 {
                n.kind = "obj".to_string();
                n.obj = cv_parse_obj_props(&t[1..e]);
                return n;
            }
        }
    }
    n
}

/// `name(` at the root of t → name, else "".
fn cv_callee_name(t: &str) -> String {
    match cv_callee_re().find(t) {
        Some(m) if !m.as_str().is_empty() => {
            let mlen = m.end();
            let b = t.as_bytes();
            if t.len() > mlen && b[mlen] == b'(' {
                m.as_str().to_string()
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

/// s[open] is the opener; return the index of its MATCHING closer
/// (mask-aware), or None.
pub fn cv_match_bracket(s: &str, open: usize, oc: u8, cc: u8) -> Option<usize> {
    let b = s.as_bytes();
    let mut depth = 0i32;
    let mut i = open;
    while i < b.len() {
        let e = cv_mask_end(s, i);
        if e > 0 {
            i = e;
            continue;
        }
        if b[i] == oc {
            depth += 1;
        } else if b[i] == cc {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

pub fn cv_parse_obj_props(inner: &str) -> Vec<CvProp> {
    let mut out: Vec<CvProp> = Vec::new();
    for part in cv_split_top(inner) {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        if let Some(rest) = p.strip_prefix("...") {
            out.push(CvProp {
                spread: true,
                key: rest.trim().to_string(),
                ..Default::default()
            });
            continue;
        }
        let pb = p.as_bytes();
        if pb[0] == b'"' || pb[0] == b'\'' {
            let end = cv_skip_string(p, 0);
            let key = cv_unquote(&p[..end]);
            let rest = p[end..].trim();
            if let Some(r) = rest.strip_prefix(':') {
                out.push(CvProp {
                    key,
                    val: r.trim().to_string(),
                    quoted: true,
                    ..Default::default()
                });
            } else {
                out.push(CvProp {
                    key,
                    quoted: true,
                    ..Default::default()
                });
            }
            continue;
        }
        let colon = cv_prop_colon(p);
        if colon < 0 {
            out.push(CvProp {
                key: p.to_string(),
                val: p.to_string(),
                shorthand: true,
                ..Default::default()
            });
            continue;
        }
        out.push(CvProp {
            key: p[..colon as usize].trim().to_string(),
            val: p[colon as usize + 1..].trim().to_string(),
            ..Default::default()
        });
    }
    out
}

pub fn cv_prop_colon(p: &str) -> isize {
    let b = p.as_bytes();
    let (mut depth, mut i) = (0i32, 0usize);
    while i < b.len() {
        let e = cv_mask_end(p, i);
        if e > 0 {
            i = e;
            continue;
        }
        match b[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b':' => {
                if depth == 0 {
                    return i as isize;
                }
            }
            _ => {}
        }
        i += 1;
    }
    -1
}

/// Parses a JS string literal's value (esbuild emits double-quoted, utf8;
/// babel's .value is the cooked text). Byte-faithful with the Go original,
/// which writes raw bytes into the builder (multibyte UTF-8 reassembles) and
/// zero-fills hex escapes it cannot parse.
pub fn cv_unquote(lit: &str) -> String {
    String::from_utf8_lossy(&cv_unquote_bytes(lit)).into_owned()
}

pub fn cv_unquote_bytes(lit: &str) -> Vec<u8> {
    let b = lit.as_bytes();
    let mut out: Vec<u8> = Vec::new();
    if b.len() < 2 {
        return out;
    }
    let q = b[0];
    let mut i = 1;
    while i < b.len() && b[i] != q {
        if b[i] != b'\\' {
            out.push(b[i]);
            i += 1;
            continue;
        }
        i += 1;
        if i >= b.len() {
            break;
        }
        match b[i] {
            b'n' => {
                out.push(b'\n');
                i += 1;
            }
            b't' => {
                out.push(b'\t');
                i += 1;
            }
            b'r' => {
                out.push(b'\r');
                i += 1;
            }
            b'b' => {
                out.push(8);
                i += 1;
            }
            b'f' => {
                out.push(12);
                i += 1;
            }
            b'v' => {
                out.push(11);
                i += 1;
            }
            b'x' => {
                if i + 2 < b.len() {
                    out.push(hex_window(b[i + 1..].to_vec(), 2));
                    i += 2;
                }
                i += 1;
            }
            b'u' => {
                if i + 1 < b.len() && b[i + 1] == b'{' {
                    if let Some(j) = lit[i + 2..].find('}') {
                        let r = hex_window_u32(b[i + 2..i + 2 + j].to_vec(), usize::MAX);
                        write_rune(&mut out, r);
                        i += 2 + j;
                    }
                    i += 1;
                } else if i + 4 < b.len() {
                    let r = hex_window_u32(b[i + 1..i + 5].to_vec(), 4);
                    write_rune(&mut out, r);
                    i += 4;
                }
                i += 1;
            }
            _ => {
                out.push(b[i]);
                i += 1;
            }
        }
    }
    out
}

fn write_rune(out: &mut Vec<u8>, r: u32) {
    match char::from_u32(r) {
        Some(c) => {
            let mut buf = [0u8; 4];
            out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
        }
        // Go's WriteRune emits RuneError for surrogates/out-of-range
        None => out.extend_from_slice("\u{FFFD}".as_bytes()),
    }
}

/// Go fmt.Sscanf "%Nx": parse up to N hex digits, stop at the first non-hex
/// byte, zero on none.
fn hex_window(bytes: Vec<u8>, n: usize) -> u8 {
    let mut v: u32 = 0;
    let mut got = 0;
    for &c in bytes.iter().take(n) {
        match (c as char).to_digit(16) {
            Some(d) => {
                v = v * 16 + d;
                got += 1;
            }
            None => break,
        }
    }
    if got == 0 {
        0
    } else {
        v as u8
    }
}

fn hex_window_u32(bytes: Vec<u8>, n: usize) -> u32 {
    let mut v: u32 = 0;
    let mut got = 0;
    for &c in bytes.iter().take(if n == usize::MAX { usize::MAX } else { n }) {
        match (c as char).to_digit(16) {
            Some(d) => {
                v = v.wrapping_mul(16).wrapping_add(d);
                got += 1;
            }
            None => break,
        }
    }
    if got == 0 {
        0
    } else {
        v
    }
}

pub fn cv_template_has_subst(t: &str) -> bool {
    let b = t.as_bytes();
    let mut i = 1;
    while i < b.len() {
        if b[i] == b'\\' {
            i += 2;
            continue;
        }
        if b[i] == b'`' {
            return false;
        }
        if b[i] == b'$' && i + 1 < b.len() && b[i + 1] == b'{' {
            return true;
        }
        i += 1;
    }
    false
}

pub fn cv_template_cook(t: &str) -> String {
    cv_unquote(&format!("`{}`", &t[1..t.len() - 1]))
}

// ---------------------------------------------------------------- word helpers

/// The next word at or after i, skipping whitespace and comments.
/// Returns (word, word_start, word_end).
pub fn cv_next_word(s: &str, i: usize) -> (String, usize, usize) {
    let b = s.as_bytes();
    let mut i = i;
    while i < b.len() {
        let e = cv_mask_end(s, i);
        if e > 0 {
            i = e;
            continue;
        }
        if b[i] == b' ' || b[i] == b'\t' || b[i] == b'\n' || b[i] == b'\r' {
            i += 1;
            continue;
        }
        break;
    }
    let (w, l) = cv_word_at(s, i);
    if l == 0 {
        return (String::new(), i, i);
    }
    (w, i, i + l)
}

pub fn cv_word_at(s: &str, i: usize) -> (String, usize) {
    let b = s.as_bytes();
    if i >= b.len() {
        return (String::new(), 0);
    }
    let c = b[i];
    if !(c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == b'_' || c == b'$') {
        return (String::new(), 0);
    }
    let mut j = i;
    while j < b.len()
        && (b[j].is_ascii_lowercase()
            || b[j].is_ascii_uppercase()
            || b[j].is_ascii_digit()
            || b[j] == b'_'
            || b[j] == b'$')
    {
        j += 1;
    }
    (s[i..j].to_string(), j - i)
}

/// Past the next ';' at depth 0 — or the next NEWLINE at depth 0, because the
/// upstream sources carry no semicolons (ASI) and an import like
/// `import { cva } from "…"` otherwise swallows the whole file.
pub fn cv_skip_stmt(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    let (mut depth, mut i) = (0i32, i);
    while i < b.len() {
        let e = cv_mask_end(s, i);
        if e > 0 {
            i = e;
            continue;
        }
        match b[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b';' | b'\n' => {
                if depth <= 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    i
}

pub fn cv_is_arrow_at(s: &str, i: usize) -> bool {
    let b = s.as_bytes();
    let mut j = i;
    while j < b.len() && (b[j] == b' ' || b[j] == b'\t' || b[j] == b'\n' || b[j] == b'\r') {
        j += 1;
    }
    j + 1 < b.len() && b[j] == b'=' && b[j + 1] == b'>'
}

/// Returns (open, close, ok) of the '(' span at i.
pub fn cv_paren_span(s: &str, i: usize) -> (usize, usize, bool) {
    let b = s.as_bytes();
    if i >= b.len() || b[i] != b'(' {
        return (0, 0, false);
    }
    match cv_match_bracket(s, i, b'(', b')') {
        Some(e) => (i, e, true),
        None => (0, 0, false),
    }
}

pub fn cv_brace_span(s: &str, i: usize) -> (usize, usize, bool) {
    let b = s.as_bytes();
    let mut i = i;
    while i < b.len() {
        let e = cv_mask_end(s, i);
        if e > 0 {
            i = e;
            continue;
        }
        if b[i] == b'{' {
            return match cv_match_bracket(s, i, b'{', b'}') {
                Some(e) => (i, e, true),
                None => (0, 0, false),
            };
        }
        if b[i] != b' ' && b[i] != b'\t' && b[i] != b'\n' && b[i] != b'\r' {
            return (0, 0, false);
        }
        i += 1;
    }
    (0, 0, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `cond ? user?.slug : "x"`: the `?` of `?.` counted as a nested
    /// ternary, so the colon search skipped the REAL `:` and the ternary
    /// degraded to kind "other" (cva axes silently empty).
    #[test]
    fn unit_ternary_colon_ignores_optional_chaining() {
        let s = "cond ? user?.slug : \"x\"";
        let q = s.find('?').unwrap();
        let colon = cv_find_ternary_colon(s, q);
        assert!(colon >= 0, "the real colon must be found");
        assert_eq!(&s[colon as usize..colon as usize + 2], ": ");
    }
}
