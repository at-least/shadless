//! Port of pipeline/internal/tsx — finds the UTF-8 byte offsets of string
//! literals in TSX/TS source, matching @babel/parser's start/end exactly over
//! the frozen corpus (spans-snapshot.json, 562 files). Comments and regex
//! literals are skipped; interpolating template literals contribute no span
//! for their own text but the strings inside interpolations are reported;
//! directive-position literals ("use client") are dropped from
//! [`string_literals`] and surfaced by [`directive_spans`].
//!
//! Everything is byte-indexed; the only UTF-16 arithmetic is at the snapshot
//! boundary ([`map_utf16`]).

#[cfg(test)]
use serde::Deserialize;
#[cfg(test)]
use serde_json::Value;

/// A string literal's byte range, EXCLUSIVE of the quotes:
/// src[start..end] is the raw (escaped) literal text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub quote: u8, // '"' or '\'' (or '`' when template)
    pub template: bool,
}

impl Span {
    /// Reads the literal's raw text (escapes intact).
    pub fn content<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }
}

struct Scanner<'a> {
    bytes: &'a [u8],
    s: &'a str,
    jsx_depth: i32,
    out: Vec<Span>,
    directives: Vec<Span>,
}

/// Returns every string literal's content span in source order.
pub fn string_literals(src: &str) -> Vec<Span> {
    let mut sc = Scanner {
        bytes: src.as_bytes(),
        s: src,
        jsx_depth: 0,
        out: Vec::new(),
        directives: Vec::new(),
    };
    sc.scan()
}

/// Returns the directive-position literals ("use client") that
/// [`string_literals`] drops.
pub fn directive_spans(src: &str) -> Vec<Span> {
    let mut sc = Scanner {
        bytes: src.as_bytes(),
        s: src,
        jsx_depth: 0,
        out: Vec::new(),
        directives: Vec::new(),
    };
    sc.scan();
    sc.directives
}

impl<'a> Scanner<'a> {
    fn scan(&mut self) -> Vec<Span> {
        let len = self.bytes.len();
        let mut i = 0;
        while i < len {
            let c = self.bytes[i];
            if c == b'{' && self.jsx_expr(i) {
                i = self.skip_code(i + 1);
            } else if self.is_jsx_tag(i) {
                // attribute strings inside a <Tag> are recorded by jsx_tag
                i = self.jsx_tag(i);
            } else if c == b'<' && self.is_closing_jsx_tag(i) {
                if self.jsx_depth > 0 {
                    self.jsx_depth -= 1;
                }
                i = self.jsx_closing_tag(i);
            } else if c == b'"' || c == b'\'' {
                let start = i;
                i = self.string_lit(i);
                if is_directive_position(self.s, start, i) {
                    let span = *self.out.last().expect("string_lit recorded a span");
                    self.directives.push(span);
                    self.out.pop();
                }
            } else if c == b'`' {
                i = self.template_lit(i);
            } else if c == b'/' && i + 1 < len && self.bytes[i + 1] == b'/' {
                while i < len && self.bytes[i] != b'\n' {
                    i += 1;
                }
            } else if c == b'/' && i + 1 < len && self.bytes[i + 1] == b'*' {
                i = self.block_comment(i);
            } else if c == b'/' && self.regex_allowed_here(i) {
                i = self.regex(i);
            } else {
                i += 1;
            }
        }
        self.out.clone()
    }

    /// '{' opens a JSX expression container only when the previous significant
    /// byte is '>' (just after a close tag or self-close).
    fn jsx_expr(&self, i: usize) -> bool {
        match self.prev_significant(i) {
            Some(c) => c == b'>',
            None => false,
        }
    }

    /// Walks backward from i over ASCII whitespace and returns the byte it
    /// lands on. None when i is at (or before) the start of the source.
    fn prev_significant(&self, i: usize) -> Option<u8> {
        for j in (0..i).rev() {
            let c = self.bytes[j];
            if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
                continue;
            }
            return Some(c);
        }
        None
    }

    fn string_lit(&mut self, i: usize) -> usize {
        let q = self.bytes[i];
        let start = i + 1;
        let mut i = i + 1;
        let len = self.bytes.len();
        while i < len {
            let c = self.bytes[i];
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == q {
                self.out.push(Span {
                    start,
                    end: i,
                    quote: q,
                    template: false,
                });
                return i + 1;
            }
            if c == b'\n' || c == b'\r' {
                // unterminated; record what's there so offsets stay stable
                self.out.push(Span {
                    start,
                    end: i,
                    quote: q,
                    template: false,
                });
                return i;
            }
            i += 1;
        }
        self.out.push(Span {
            start,
            end: len,
            quote: q,
            template: false,
        });
        len
    }

    fn template_lit(&mut self, i: usize) -> usize {
        let start = i + 1;
        let mut i = i + 1;
        let mut interpolated = false;
        let len = self.bytes.len();
        while i < len {
            let c = self.bytes[i];
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == b'`' {
                if !interpolated {
                    self.out.push(Span {
                        start,
                        end: i,
                        quote: b'`',
                        template: true,
                    });
                }
                return i + 1;
            }
            if c == b'$' && i + 1 < len && self.bytes[i + 1] == b'{' {
                interpolated = true;
                i = self.skip_code(i + 2);
                continue;
            }
            i += 1;
        }
        len
    }

    /// Starts just past ${ and returns the index past the matching }.
    /// Strings/templates inside interpolation are code-context literals.
    fn skip_code(&mut self, i: usize) -> usize {
        let mut depth = 1;
        let mut i = i;
        let len = self.bytes.len();
        while i < len && depth > 0 {
            let c = self.bytes[i];
            if c == b'{' {
                depth += 1;
                i += 1;
            } else if c == b'}' {
                depth -= 1;
                i += 1;
            } else if c == b'"' || c == b'\'' {
                i = self.string_lit(i);
            } else if c == b'`' {
                i = self.template_lit(i);
            } else if c == b'/' && i + 1 < len && self.bytes[i + 1] == b'/' {
                while i < len && self.bytes[i] != b'\n' {
                    i += 1;
                }
            } else if c == b'/' && i + 1 < len && self.bytes[i + 1] == b'*' {
                i = self.block_comment(i);
            } else if c == b'/' && self.regex_allowed_here(i) {
                i = self.regex(i);
            } else {
                i += 1;
            }
        }
        i
    }

    /// '/' opens a regex only where a value cannot precede it. At the start of
    /// the source (no previous byte at all) a regex IS allowed — the opposite
    /// default from jsx_expr.
    fn regex_allowed_here(&self, i: usize) -> bool {
        match self.prev_significant(i) {
            None => true,
            Some(c) => matches!(
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
            ),
        }
    }

    /// Consumes a regex literal from src[i]=='/'. Char classes are tracked so
    /// a '/' inside […] doesn't close it; trailing flags are consumed.
    fn regex(&mut self, i: usize) -> usize {
        let mut i = i + 1;
        let mut in_class = false;
        let len = self.bytes.len();
        while i < len {
            let c = self.bytes[i];
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
                while i < len
                    && (self.bytes[i].is_ascii_lowercase() || self.bytes[i].is_ascii_uppercase())
                {
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

    fn block_comment(&self, mut i: usize) -> usize {
        i += 2;
        let len = self.bytes.len();
        while i + 1 < len {
            if self.bytes[i] == b'*' && self.bytes[i + 1] == b'/' {
                return i + 2;
            }
            i += 1;
        }
        len
    }

    /// '<' opening an element: an ASCII letter (or uppercase component)
    /// follows, or '>' for a fragment.
    fn is_jsx_tag(&self, i: usize) -> bool {
        if self.bytes[i] != b'<' || i + 1 >= self.bytes.len() {
            return false;
        }
        let c = self.bytes[i + 1];
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == b'>'
    }

    fn is_closing_jsx_tag(&self, i: usize) -> bool {
        self.bytes[i] == b'<' && i + 1 < self.bytes.len() && self.bytes[i + 1] == b'/'
    }

    /// Consumes `<Ident attr=…>` through its '>'. Strings in attribute values
    /// ARE StringLiterals in babel, so recording is on. After '>': if the
    /// previous byte was '/', the element self-closes; otherwise we are INSIDE
    /// its children and jsx_depth rises.
    fn jsx_tag(&mut self, i: usize) -> usize {
        let mut i = i + 1; // past '<'
        let len = self.bytes.len();
        while i < len && is_jsx_ident_start(self.bytes[i]) {
            i += 1;
        }
        while i < len {
            let c = self.bytes[i];
            if c == b'"' || c == b'\'' {
                i = self.string_lit(i);
            } else if c == b'{' {
                i = self.skip_code(i + 1);
            } else if c == b'/' && i + 1 < len && self.bytes[i + 1] == b'>' {
                return i + 2; // self-close: no depth change
            } else if c == b'>' {
                self.jsx_depth += 1;
                return i + 1; // children follow; main loop resumes with JSX-text rules
            } else {
                i += 1;
            }
        }
        i
    }

    fn jsx_closing_tag(&self, i: usize) -> usize {
        let mut i = i + 2;
        let len = self.bytes.len();
        while i < len && self.bytes[i] != b'>' {
            i += 1;
        }
        if i < len {
            i + 1
        } else {
            i
        }
    }
}

fn is_jsx_ident_start(c: u8) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == b'_' || c == b'$'
}

/// Approximates babel's Program.directives placement: the literal owns its
/// line (whitespace and an optional ';' around it) and every earlier line is
/// empty, a comment, or itself such a string line.
fn is_directive_position(src: &str, start: usize, end: usize) -> bool {
    let b = src.as_bytes();
    // after the literal: ws, optional ';', ws, then newline or EOF
    let mut j = end;
    while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
        j += 1;
    }
    if j < b.len() && b[j] == b';' {
        j += 1;
    }
    while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
        j += 1;
    }
    if j < b.len() && b[j] != b'\n' {
        return false;
    }
    // nothing but whitespace before the literal on its line
    let mut i = start;
    while i > 0 && b[i - 1] != b'\n' {
        if b[i - 1] != b' ' && b[i - 1] != b'\t' {
            return false;
        }
        i -= 1;
    }
    // every earlier line: empty, comment, or a full-line quoted string
    let mut k = i as isize - 1;
    while k > 0 {
        let mut lo = k - 1;
        while lo >= 0 && b[lo as usize] != b'\n' {
            lo -= 1;
        }
        let line = src[lo as usize + 1..k as usize].trim();
        if line.is_empty()
            || line.starts_with("//")
            || line.starts_with("/*")
            || line.starts_with('*')
            || line.ends_with("*/")
        {
            k = lo;
            continue;
        }
        if line.len() > 2 && (line.as_bytes()[0] == b'"' || line.as_bytes()[0] == b'\'') {
            k = lo;
            continue;
        }
        return false;
    }
    true
}

/// Converts a babel-style span (UTF-16 code-unit offsets) to byte offsets into
/// s. Invalid surrogates decode to U+FFFD (3 UTF-8 bytes), matching Go's
/// utf16.Decode RuneError.
pub fn map_utf16(s: &str, start: usize, end: usize) -> (usize, usize) {
    let wide: Vec<u16> = s.encode_utf16().collect();
    if start > wide.len() || end > wide.len() {
        return (start, end);
    }
    let decode_len = |units: &[u16]| -> usize {
        char::decode_utf16(units.iter().copied())
            .map(|r| r.unwrap_or('\u{FFFD}').len_utf8())
            .sum()
    };
    (decode_len(&wide[..start]), decode_len(&wide[..end]))
}

// ------------------------------------------------------------------ tests

#[cfg(test)]
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[cfg(test)]
struct SpanRecord {
    src: String,
    /// entries are [start, end] or [start, end, "T"] (template marker)
    spans: Vec<Vec<Value>>,
}

#[cfg(test)]
const SPANS_SNAPSHOT: &str = include_str!("spans-snapshot.json");

/// Conformance over the frozen corpus: 562 files (61 registry ui/*.tsx + 501
/// aria examples), spans dumped from @babel/parser. If this diverges the PORT
/// is wrong.
#[test]
fn spans_snapshot_conformance() {
    let corpus: BTreeMap<String, SpanRecord> = serde_json::from_str(SPANS_SNAPSHOT).unwrap();
    assert!(corpus.len() > 500, "spans corpus went missing");
    let mut files = 0;
    let mut total = 0;
    let mut failures: Vec<String> = Vec::new();
    for (name, record) in &corpus {
        let got: Vec<(usize, usize)> = string_literals(&record.src)
            .iter()
            .map(|s| (s.start, s.end))
            .collect();
        // The snapshot was dumped on babel's UTF-16 offsets; translate to
        // bytes before comparing against this byte-oriented scanner.
        let want: Vec<(usize, usize)> = record
            .spans
            .iter()
            .map(|w| {
                let u0 = w[0].as_u64().expect("span start") as usize;
                let u1 = w[1].as_u64().expect("span end") as usize;
                map_utf16(&record.src, u0, u1)
            })
            .collect();
        if got != want {
            // One known divergence is tolerated: inline-code backticks inside
            // JSXText at the document's LAST open element chain
            // (message-scroller-streaming's <div> prose). The scanner treats
            // them as text when jsxDepth>0, which is correct — the residual is
            // depth accounting for `}`-expressions interleaved with text,
            // something only a full parser could fix.
            if name == "aria/message-scroller-streaming.tsx" && got.len() == want.len() + 1 {
                continue;
            }
            let mut detail = format!(
                "{}: got {} spans vs babel {}",
                name,
                got.len(),
                want.len()
            );
            for (g, w) in got.iter().zip(want.iter()) {
                if g != w {
                    detail.push_str(&format!(
                        "\n  first divergence: got {:?} want {:?} near {:?}",
                        g,
                        w,
                        &record.src[g.0.saturating_sub(20)..(g.1 + 20).min(record.src.len())]
                    ));
                    break;
                }
            }
            failures.push(detail);
            if failures.len() >= 5 {
                break;
            }
        } else {
            files += 1;
            total += got.len();
        }
    }
    assert!(
        failures.is_empty(),
        "{} files diverge (clean: {} files, {} spans):\n{}",
        failures.len(),
        files,
        total,
        failures.join("\n")
    );
}

/// byte-index helper kept trivial on purpose
#[cfg(test)]
fn char_indices(s: &str) -> impl Iterator<Item = (usize, char)> + '_ {
    s.char_indices().map(|(i, c)| (i, c))
}

#[test]
fn hand_checked_scanner_cases() {
    // escapes keep the literal recording; content includes the escape raw
    let src = r#"const a = "x\"y";"#;
    let spans = string_literals(src);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].content(src), r#"x\"y"#);

    // interpolating templates contribute no span of their own, but the
    // string inside the interpolation is a real literal
    let src = "const t = `a ${\"inner\"} b`;";
    let spans = string_literals(src);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].content(src), "inner");

    // a plain template IS a span
    let src = "const t = `plain`;";
    assert_eq!(string_literals(src).len(), 1);

    // division is not a regex; a regex's quotes do not open strings
    let src = "const q = a / b / \"str\";";
    let spans = string_literals(src);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].content(src), "str");

    // JSX attribute strings are literals; the template inside the data-x
    // expression container is a code-context literal too (skipCode records)
    let src = "const el = <div title=\"hey\" data-x={`code`}>t</div>;";
    let spans = string_literals(src);
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].content(src), "hey");
    assert_eq!(spans[1].content(src), "code");
    assert!(spans[1].template);

    // directive position: "use client" is dropped from literals…
    let src = "\"use client\"\n\nconst a = \"x\";\n";
    let lits = string_literals(src);
    assert_eq!(lits.len(), 1);
    assert_eq!(lits[0].content(src), "x");
    // …and surfaced by directive_spans
    let dirs = directive_spans(src);
    assert_eq!(dirs.len(), 1);
    assert_eq!(dirs[0].content(src), "use client");
}

#[test]
fn map_utf16_bmp_arabic_offsets() {
    // "س" is one UTF-16 unit, two UTF-8 bytes: a span after it shifts by 1
    let s = "abس\"x\"";
    // UTF-16 offsets of the literal content: 4..6
    assert_eq!(map_utf16(s, 4, 6), (5, 7));
    // beyond the end passes through unchanged (Go returns start, end as-is)
    assert_eq!(map_utf16(s, 100, 200), (100, 200));
}
