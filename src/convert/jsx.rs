//! Port of the original-source JSX children classifier (convert.go). One
//! record per JSX element, in document (pre-)order — matching both babel's
//! walk and the downgraded createElement call order. Fragment elements
//! (<>/React.Fragment) produce no record.

use super::scan::{cv_first_top, cv_mask_end, cv_ops_qor_logical, cv_ident_re};
use regex::Regex;
use std::sync::OnceLock;

#[derive(Clone, Debug, Default)]
pub struct JsxKind {
    pub elem: bool,   // a nested element child
    pub text: String, // "text" | "expr" | "{name}" | "OPT?"
}

#[derive(Clone, Debug, Default)]
pub struct CvJsxRec {
    pub kinds: Vec<JsxKind>,
}

pub fn cv_jsx_name_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^[A-Za-z_$][A-Za-z0-9_$-]*(?:\.[A-Za-z0-9_$-]+)*").unwrap())
}

fn cv_jsx_keywords(w: &str) -> bool {
    matches!(
        w,
        "return" | "typeof" | "case" | "do" | "else" | "in" | "of" | "new" | "delete"
            | "void" | "yield" | "await" | "instanceof" | "throw"
    )
}

struct JsxScanner<'a> {
    s: &'a str,
    recs: Vec<CvJsxRec>,
}

pub fn scan_jsx_kinds(src: &str) -> Result<Vec<CvJsxRec>, String> {
    let mut st = JsxScanner {
        s: src,
        recs: Vec::new(),
    };
    let len = src.len();
    let mut i = 0;
    while i < len {
        let e = cv_mask_end(src, i);
        if e > 0 {
            i = e;
            continue;
        }
        if src.as_bytes()[i] == b'<' && st.open_at(i) {
            i = st.parse_element(i)?;
            continue;
        }
        i += 1;
    }
    Ok(st.recs)
}

impl<'a> JsxScanner<'a> {
    /// Does the '<' at i open a JSX element? A `<` preceded by an identifier
    /// is a comparison or generic (Array<string>); preceded by a keyword
    /// (return <div>) it is JSX.
    fn open_at(&self, i: usize) -> bool {
        let b = self.s.as_bytes();
        if i + 1 >= b.len() {
            return false;
        }
        let c = b[i + 1];
        if !(c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == b'_' || c == b'$' || c == b'>') {
            return false;
        }
        let mut j = i as isize - 1;
        while j >= 0
            && (b[j as usize] == b' '
                || b[j as usize] == b'\t'
                || b[j as usize] == b'\n'
                || b[j as usize] == b'\r')
        {
            j -= 1;
        }
        if j < 0 {
            return true;
        }
        let p = b[j as usize];
        if p.is_ascii_lowercase() || p.is_ascii_uppercase() || p.is_ascii_digit() || p == b'_' || p == b'$' {
            let mut k = j;
            while k >= 0
                && (b[k as usize].is_ascii_lowercase()
                    || b[k as usize].is_ascii_uppercase()
                    || b[k as usize].is_ascii_digit()
                    || b[k as usize] == b'_'
                    || b[k as usize] == b'$')
            {
                k -= 1;
            }
            return cv_jsx_keywords(&self.s[k as usize + 1..j as usize + 1]);
        }
        p != b')' && p != b']'
    }

    /// Consumes a full element (or fragment), appending records in pre-order.
    /// The element's OWN record slot is reserved before its children are
    /// parsed, so parent precedes children.
    fn parse_element(&mut self, i: usize) -> Result<usize, String> {
        let b = self.s.as_bytes();
        if b[i + 1] == b'>' {
            // fragment <>: no record of its own
            let (_, n) = self.parse_children(i + 2, "")?;
            return Ok(n);
        }
        let m = cv_jsx_name_re()
            .find(&self.s[i + 1..])
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let name = m;
        let mut i = i + 1 + name.len();
        // React.Fragment as an explicit tag is indistinguishable from <> in
        // the downgraded output either way; both take the fragment path.
        if name == "React.Fragment" {
            let (_, n) = self.parse_children(i, "")?;
            return Ok(n);
        }
        // reserve the record slot now — pre-order
        let slot = self.recs.len();
        self.recs.push(CvJsxRec::default());
        let mut kinds: Vec<JsxKind> = Vec::new();
        let len = self.s.len();
        loop {
            while i < len {
                let e = cv_mask_end(self.s, i);
                if e > 0 {
                    i = e;
                    continue;
                }
                let c = b[i];
                if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
                    i += 1;
                    continue;
                }
                break;
            }
            if i >= len {
                return Err(format!("unterminated element <{}>", name));
            }
            if b[i] == b'/' && i + 1 < len && b[i + 1] == b'>' {
                i += 2;
                break;
            }
            if b[i] == b'>' {
                let (k, n) = self.parse_children(i + 1, &name)?;
                kinds = k;
                i = n;
                break;
            }
            if b[i] == b'{' {
                i = self.container_end(i) + 1;
                continue;
            }
            i += 1;
        }
        self.recs[slot].kinds = kinds;
        Ok(i)
    }

    /// Consumes children until the closing tag ("" closeName = a fragment
    /// `</>`), returning the child kind list and the resume index.
    fn parse_children(
        &mut self,
        mut i: usize,
        close_name: &str,
    ) -> Result<(Vec<JsxKind>, usize), String> {
        let mut kinds: Vec<JsxKind> = Vec::new();
        let b = self.s.as_bytes();
        let len = self.s.len();
        loop {
            if i >= len {
                return Err(format!("unterminated children of <{}>", close_name));
            }
            let c = b[i];
            if c == b'<' {
                if b[i + 1] == b'/' {
                    let mut j = i + 2;
                    if close_name.is_empty() {
                        while j < len && b[j] != b'>' {
                            j += 1;
                        }
                        return Ok((kinds, j + 1));
                    }
                    let m = cv_jsx_name_re()
                        .find(&self.s[j..])
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                    if m != close_name {
                        return Err(format!("closing </{}> does not match <{}>", m, close_name));
                    }
                    j += m.len();
                    while j < len && b[j] != b'>' {
                        j += 1;
                    }
                    return Ok((kinds, j + 1));
                }
                if b[i + 1] == b'>' {
                    // nested fragment: no kind of its own
                    let (_, n) = self.parse_children(i + 2, "")?;
                    i = n;
                    continue;
                }
                let n = self.parse_element(i)?;
                kinds.push(JsxKind {
                    elem: true,
                    text: String::new(),
                });
                i = n;
                continue;
            }
            if c == b'{' {
                let end = self.container_end(i);
                kinds.push(JsxKind {
                    elem: false,
                    text: cv_classify_container(&self.s[i + 1..end]),
                });
                i = end + 1;
                continue;
            }
            // JSX text: runs until < or { — NOT JS-masked (quotes are literal)
            let mut j = i;
            while j < len && b[j] != b'<' && b[j] != b'{' {
                j += 1;
            }
            if !self.s[i..j].trim().is_empty() {
                kinds.push(JsxKind {
                    elem: false,
                    text: "text".to_string(),
                });
            }
            i = j;
        }
    }

    /// The matching '}' of s[i] == '{', recursing into nested JSX elements so
    /// their records land in pre-order.
    fn container_end(&mut self, i: usize) -> usize {
        let mut depth = 1;
        let mut i = i + 1;
        let len = self.s.len();
        while i < len && depth > 0 {
            let e = cv_mask_end(self.s, i);
            if e > 0 {
                i = e;
                continue;
            }
            let c = self.s.as_bytes()[i];
            if c == b'<' && self.open_at(i) {
                if let Ok(n) = self.parse_element(i) {
                    i = n;
                    continue;
                }
            }
            match c {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                _ => {}
            }
            i += 1;
        }
        i - 1
    }
}

/// babel's sketchChildren on the container's expression.
pub fn cv_classify_container(expr: &str) -> String {
    let t = expr.trim();
    if t.is_empty() || t.starts_with("/*") || t.starts_with("//") {
        return "expr".to_string(); // JSXEmptyExpression (comment-only container)
    }
    if cv_ident_re().is_match(t) && t != "true" && t != "false" && t != "null" {
        return format!("{{{}}}", t);
    }
    if cv_first_top(t, cv_ops_qor_logical) >= 0 {
        return "OPT?".to_string();
    }
    "expr".to_string()
}
