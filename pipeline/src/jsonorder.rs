//! Port of pipeline/jsonorder.go — a JSON writer with JS semantics for the
//! generated files this pipeline commits.
//!
//! encoding/json cannot produce them: Go maps have no key order, and
//! JSON.stringify output — which is what is in git — is ordered by insertion.
//! Two more silent differences: Go escapes < > & by default (JS does not) and
//! JS DROPS a key whose value is `undefined` but KEEPS one whose value is
//! `null` — absence is modelled by never adding the pair, explicit null by
//! [`Json::Null`]. Numbers that were JS literals travel as [`Json::Raw`] so
//! 1.5 and 2e3 serialize exactly as the JS wrote them — no float formatting
//! ever happens.

/// An ordered object; a key that should be absent is one you never add.
#[derive(Default)]
pub struct JsonObj {
    pairs: Vec<(String, Json)>,
}

impl JsonObj {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(mut self, k: &str, v: Json) -> Self {
        self.pairs.push((k.to_string(), v));
        self
    }
    pub fn into_pairs(self) -> Vec<(String, Json)> {
        self.pairs
    }
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    pub fn get(&self, i: usize) -> &(String, Json) {
        &self.pairs[i]
    }
    pub fn index_of(&self, k: &str) -> isize {
        for (i, p) in self.pairs.iter().enumerate() {
            if p.0 == k {
                return i as isize;
            }
        }
        -1
    }
    #[allow(dead_code)] // Go-port accessor; kept for parity
    fn pairs(&self) -> &[(String, Json)] {
        &self.pairs
    }
}

/// Builder shorthand mirroring jsonObj{}.add(...).add(...)
pub fn obj(pairs: Vec<(&str, Json)>) -> Json {
    Json::Obj(
        pairs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<Vec<_>>(),
    )
}

#[derive(Clone, Debug)]
pub enum Json {
    Obj(Vec<(String, Json)>),
    Arr(Vec<Json>),
    Str(String),
    /// jsonRaw: pre-rendered JSON text (a number literal lifted verbatim)
    Raw(String),
    Int(i64),
    Bool(bool),
    /// jsonNull: a key that exists with a null value, as distinct from a key
    /// that is not there at all
    Null,
}

impl Json {
    pub fn from_obj(o: JsonObj) -> Json {
        Json::Obj(o.into_pairs())
    }
}

pub fn arr(items: Vec<Json>) -> Json {
    Json::Arr(items)
}

/// Escapes exactly as JSON.stringify does: quote, backslash and the C0
/// controls, with \b \f \n \r \t spelled out and everything else — including
/// <, > and &, U+2028/U+2029 and DEL — passed through as UTF-8. (Go's rune
/// iteration also maps invalid UTF-8 bytes to a literal U+FFFD; Rust strings
/// cannot carry invalid UTF-8, and the pipeline's inputs are valid.)
pub fn json_string(s: &str) -> String {
    let mut b = String::with_capacity(s.len() + 2);
    b.push('"');
    for r in s.chars() {
        match r {
            '"' => b.push_str("\\\""),
            '\\' => b.push_str("\\\\"),
            '\u{8}' => b.push_str("\\b"),
            '\u{c}' => b.push_str("\\f"),
            '\n' => b.push_str("\\n"),
            '\r' => b.push_str("\\r"),
            '\t' => b.push_str("\\t"),
            _ => {
                if (r as u32) < 0x20 {
                    b.push_str(&format!("\\u{:04x}", r as u32));
                } else {
                    b.push(r);
                }
            }
        }
    }
    b.push('"');
    b
}

/// Renders v the way JSON.stringify(v, null, 2) would.
pub fn marshal_js(v: &Json) -> String {
    marshal_js_step(v, "", "  ")
}

/// Same with an explicit per-level step, for files JSON.stringify wrote with a
/// different one — src/registry/upstream-snapshot/exemptions.json uses 1.
/// `indent` is the CURRENT prefix (callers start with ""), not the step.
pub fn marshal_js_step(v: &Json, indent: &str, step: &str) -> String {
    let inner = format!("{}{}", indent, step);
    match v {
        Json::Obj(pairs) => {
            if pairs.is_empty() {
                return "{}".to_string();
            }
            let parts: Vec<String> = pairs
                .iter()
                .map(|(k, val)| {
                    format!(
                        "{}{}: {}",
                        inner,
                        json_string(k),
                        marshal_js_step(val, &inner, step)
                    )
                })
                .collect();
            format!("{{\n{}\n{}}}", parts.join(",\n"), indent)
        }
        Json::Arr(items) => {
            if items.is_empty() {
                return "[]".to_string();
            }
            let parts: Vec<String> = items
                .iter()
                .map(|e| format!("{}{}", inner, marshal_js_step(e, &inner, step)))
                .collect();
            format!("[\n{}\n{}]", parts.join(",\n"), indent)
        }
        Json::Str(s) => json_string(s),
        Json::Raw(r) => r.clone(),
        Json::Int(i) => i.to_string(),
        Json::Bool(b) => b.to_string(),
        Json::Null => "null".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Byte-equality against the Go jsonorder.go implementation's own output
    /// (probe/json-go: the REAL jsonorder.go compiled with a fixture main).
    /// Regenerate with probe/json-go/setup.sh.
    #[test]
    fn golden_bytes_match_go() {
        let golden = include_str!("../probe/json-go/golden.txt");

        let weird = "a\"b\\c\u{8}d\u{c}e\nf\rg\th".to_string()
            + "\u{0}\u{1}\u{1f}"
            + "<b>&</b>"
            + "\u{2028}\u{2029}"
            + "é中\u{1F600}"
            + "\u{7f}"
            + "\u{a0}";
        let rtl = "زر عربي עברית".to_string();

        let fixtures: Vec<(&str, Json)> = vec![
            ("plain-string", Json::Raw(json_string("hello world"))),
            ("escapes", Json::Raw(json_string(&weird))),
            ("rtl", Json::Raw(json_string(&rtl))),
            (
                "raw-numbers",
                obj(vec![
                    ("a", Json::Raw("1.5".into())),
                    ("b", Json::Raw("2e3".into())),
                    ("c", Json::Raw("-0".into())),
                    ("d", Json::Raw("0.30000000000000004".into())),
                    ("e", Json::Raw("1e-5".into())),
                ]),
            ),
            (
                "ints",
                obj(vec![
                    ("zero", Json::Int(0)),
                    ("neg", Json::Int(-7)),
                    ("big", Json::Int(i64::MAX)),
                ]),
            ),
            (
                "bools-null",
                obj(vec![
                    ("t", Json::Bool(true)),
                    ("f", Json::Bool(false)),
                    ("null", Json::Null),
                ]),
            ),
            ("empty", Json::Raw(format!("{}\n{}", marshal_js(&Json::Obj(vec![])), marshal_js(&Json::Arr(vec![]))))),
            (
                "nested",
                obj(vec![
                    (
                        "obj",
                        obj(vec![
                            ("list", arr(vec![Json::Str("a".into()), Json::Int(1), Json::Bool(true), Json::Null])),
                            ("e", Json::Obj(vec![])),
                        ]),
                    ),
                    ("arr", arr(vec![obj(vec![("k", Json::Str("v".into()))])])),
                ]),
            ),
            (
                "arrays-of-strings",
                obj(vec![("ss", arr(vec![Json::Str("x".into()), Json::Str("y\"z".into())]))]),
            ),
            (
                "step-indent-1",
                Json::Raw(marshal_js_step(
                    &obj(vec![("k", obj(vec![("in", Json::Str("v".into()))]))]),
                    "",
                    " ",
                )),
            ),
            ("top-indent", Json::Raw(marshal_js_step(&obj(vec![("k", Json::Str("v".into()))]), "  ", "  "))),
            (
                "colon-slash",
                Json::Raw(json_string("hover:before:bg-[url('a:b')][/&]")),
            ),
        ];

        let mut out = String::new();
        for (name, v) in fixtures {
            out.push_str(&format!("==== {} ====\n", name));
            // every fixture value is itself already-rendered JSON text
            let rendered = match v {
                Json::Raw(r) => r,
                other => marshal_js(&other),
            };
            out.push_str(&rendered);
            out.push('\n');
        }

        if out != golden {
            // find the first diverging line for a fast diagnosis
            for (i, (a, b)) in out.lines().zip(golden.lines()).enumerate() {
                if a != b {
                    panic!(
                        "golden diverges at line {}\n  rust: {:?}\n  go:   {:?}",
                        i + 1,
                        a,
                        b
                    );
                }
            }
            panic!(
                "golden diverges in length: rust {} lines, go {} lines",
                out.lines().count(),
                golden.lines().count()
            );
        }
    }

    #[test]
    fn escaping_rules() {
        assert_eq!(json_string("a<b>&c"), "\"a<b>&c\"");
        assert_eq!(json_string("\u{7f}\u{a0}"), "\"\u{7f}\u{a0}\"");
        assert_eq!(json_string("\u{2028}"), "\"\u{2028}\"");
        assert_eq!(json_string("\u{1}\u{1f}"), "\"\\u0001\\u001f\"");
        assert_eq!(json_string("\n\r\t"), "\"\\n\\r\\t\"");
    }
}
