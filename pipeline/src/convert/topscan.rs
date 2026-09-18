//! Top-level declaration scan + imports of the downgraded JS / original
//! source (convert.go scanTopJs / importsOf / cvParseImport).

use super::scan::*;
use regex::Regex;

#[derive(Clone, Debug, Default)]
pub struct CvDecl {
    pub name: String,
    pub is_fn: bool,    // function declaration (always a component)
    pub is_arrow: bool, // const/let/var with arrow/function init
    pub exported: bool, // declared with `export`
    pub params: [usize; 2],
    pub body: [usize; 2],
    pub single_param: String, // arrow with one bare ident param (`x => …`)
}

pub struct CvTopScan<'a> {
    js: &'a str,
    pub decls: Vec<CvDecl>,
    pub exported_names: std::collections::HashSet<String>,
    pub decl_index: std::collections::HashMap<String, usize>, // name → decls index
    pub default_arrow: isize, // decls index pushed as the "default" component, -1
}

pub fn scan_top_js(js: &str) -> Result<CvTopScan<'_>, String> {
    let mut t = CvTopScan {
        js,
        decls: Vec::new(),
        exported_names: std::collections::HashSet::new(),
        decl_index: std::collections::HashMap::new(),
        default_arrow: -1,
    };
    let b = js.as_bytes();
    let (mut depth, mut i) = (0i32, 0usize);
    while i < b.len() {
        let e = cv_mask_end(js, i);
        if e > 0 {
            i = e;
            continue;
        }
        let c = b[i];
        if c == b'(' || c == b'[' || c == b'{' {
            depth += 1;
            i += 1;
            continue;
        }
        if c == b')' || c == b']' || c == b'}' {
            depth -= 1;
            i += 1;
            continue;
        }
        if depth > 0 {
            i += 1;
            continue;
        }
        let (w, wlen) = cv_word_at(js, i);
        match w.as_str() {
            // imports come from the ORIGINAL source (type-only imports are
            // dropped by the downgrade but babel counted them); skip here
            "import" => i = cv_skip_stmt(js, i),
            "export" => {
                // the token after `export` may be `{`, which is no word
                let mut j = i + wlen;
                while j < b.len() {
                    let e = cv_mask_end(js, j);
                    if e > 0 {
                        j = e;
                        continue;
                    }
                    if b[j] == b' ' || b[j] == b'\t' || b[j] == b'\n' || b[j] == b'\r' {
                        j += 1;
                        continue;
                    }
                    break;
                }
                let (w2, w2len) = cv_word_at(js, j);
                let n = if j < b.len() && b[j] == b'{' {
                    t.parse_export_specs(j)?
                } else if w2 == "function" {
                    t.parse_fn_decl(j, true)?
                } else if w2 == "const" || w2 == "let" || w2 == "var" {
                    t.parse_var_decl(j, w2len, true)?
                } else {
                    cv_skip_stmt(js, i)
                };
                i = n;
            }
            "function" => {
                let n = t.parse_fn_decl(i, false)?;
                i = n;
            }
            "const" | "let" | "var" => {
                let n = t.parse_var_decl(i, wlen, false)?;
                i = n;
            }
            "async" => i += wlen,
            _ => i += 1,
        }
    }
    Ok(t)
}

impl<'a> CvTopScan<'a> {
    fn parse_fn_decl(&mut self, i: usize, exported: bool) -> Result<usize, String> {
        let js = self.js;
        let (_, _, kw_end) = cv_next_word(js, i); // past `function`
        let (name, _, name_end) = cv_next_word(js, kw_end);
        if name.is_empty() {
            return Ok(cv_skip_stmt(js, i));
        }
        let (po, pe, ok) = cv_paren_span(js, name_end);
        if !ok {
            return Ok(cv_skip_stmt(js, i));
        }
        let (bo, be, ok) = cv_brace_span(js, pe + 1);
        if !ok {
            return Ok(cv_skip_stmt(js, i));
        }
        self.decls.push(CvDecl {
            name: name.clone(),
            is_fn: true,
            exported,
            params: [po, pe],
            body: [bo, be],
            ..Default::default()
        });
        self.decl_index.insert(name, self.decls.len() - 1);
        Ok(be)
    }

    /// Walks the declarators of a const/let/var statement; a declarator whose
    /// init is an arrow/function records its absolute params/body spans.
    fn parse_var_decl(&mut self, i: usize, kwlen: usize, exported: bool) -> Result<usize, String> {
        let js = self.js;
        let end = cv_skip_stmt(js, i);
        let mut seg_end = end;
        if seg_end > i && js.as_bytes()[seg_end - 1] == b';' {
            seg_end -= 1;
        }
        for dcl in cv_split_top_indexed(&js[i + kwlen..seg_end]) {
            let text = dcl.text.trim().to_string();
            if text.is_empty() {
                continue;
            }
            let (name, nlen) = cv_word_at(&text, 0);
            if name.is_empty() {
                continue;
            }
            let tb = text.as_bytes();
            let mut eq: isize = -1;
            {
                let mut depth = 0i32;
                let mut k = nlen;
                while k < tb.len() {
                    let e = cv_mask_end(&text, k);
                    if e > 0 {
                        k = e;
                        continue;
                    }
                    match tb[k] {
                        b'(' | b'[' | b'{' => depth += 1,
                        b')' | b']' | b'}' => depth -= 1,
                        b'=' => {
                            if depth == 0
                                && !(k + 1 < tb.len() && (tb[k + 1] == b'=' || tb[k + 1] == b'>'))
                            {
                                eq = k as isize;
                            }
                        }
                        _ => {}
                    }
                    if eq >= 0 {
                        break;
                    }
                    k += 1;
                }
            }
            if eq < 0 {
                continue;
            }
            let init_text = &text[eq as usize + 1..];
            // eq indexes the TRIMMED text and dcl.off is relative to the
            // SLICE (js[i+kwlen..] is what cv_split_top_indexed scanned):
            // both offsets must be rebased to js before combining, or every
            // arrow span comes out shifted (and destructured param defaults
            // were silently unextractable). Rebase on trim_START only — a
            // trailing whitespace tail (ASI newline, `… , B = 2;`) would
            // otherwise shift the spans right.
            let trim_left = dcl.text.len() - dcl.text.trim_start().len();
            let init_base = i + kwlen + dcl.off + trim_left + eq as usize + 1; // absolute position of initText[0]
            let decl_idx = self.decls.len();
            self.decls.push(CvDecl {
                name: name.clone(),
                exported,
                ..Default::default()
            });
            if let Some(ar) = cv_arrow_span(init_text, init_base) {
                let d = &mut self.decls[decl_idx];
                d.is_arrow = true;
                d.params = ar.params;
                d.body = ar.body;
                d.single_param = ar.single;
                self.decl_index.insert(name, decl_idx);
            }
        }
        Ok(end)
    }

    /// `export { A, B as C }` — babel collects the EXPORTED name; esbuild
    /// rewrites `export default function D(){}` to `function D` + `D as
    /// default`, and an anonymous default to a synthesized `<x>_default` var.
    fn parse_export_specs(&mut self, i: usize) -> Result<usize, String> {
        let js = self.js;
        let e = match cv_match_bracket(js, i, b'{', b'}') {
            Some(e) => e,
            None => return Ok(cv_skip_stmt(js, i)),
        };
        for sp in cv_split_top(&js[i + 1..e]) {
            let fields: Vec<&str> = sp.split_whitespace().collect();
            let local;
            let exported;
            if fields.len() >= 3 && fields[1] == "as" {
                local = fields[0].to_string();
                exported = fields[2].to_string();
            } else if !fields.is_empty() {
                local = fields[0].to_string();
                exported = fields[0].to_string();
            } else {
                continue;
            }
            if exported == "default" {
                let di = self.decl_index.get(&local).copied();
                let is_fn_like = match di {
                    Some(d) => self.decls[d].is_fn || self.decls[d].is_arrow,
                    None => false,
                };
                if is_fn_like && !local.ends_with("_default") {
                    self.exported_names.insert(local);
                } else {
                    self.exported_names.insert("default".to_string());
                    if let Some(d) = di {
                        if self.decls[d].is_arrow {
                            self.default_arrow = d as isize;
                        }
                    }
                }
                continue;
            }
            self.exported_names.insert(exported);
        }
        Ok(e + 1)
    }
}

pub struct CvSplitPart {
    pub text: String,
    pub off: usize, // offset of text's start within the scanned string
}

pub fn cv_split_top_indexed(s: &str) -> Vec<CvSplitPart> {
    let mut out = Vec::new();
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
                    out.push(CvSplitPart {
                        text: s[start..i].to_string(),
                        off: start,
                    });
                    start = i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out.push(CvSplitPart {
        text: s[start..].to_string(),
        off: start,
    });
    out
}

pub struct CvArrowInfo {
    pub params: [usize; 2],
    pub body: [usize; 2],
    pub single: String,
}

/// Is init (starting at absolute offset base) a function value? If so, return
/// absolute params/body spans.
pub fn cv_arrow_span(init: &str, base: usize) -> Option<CvArrowInfo> {
    let mut t = init.trim_start_matches([' ', '\t', '\n', '\r']);
    let mut off = base + (init.len() - t.len());
    let (w, l) = cv_word_at(t, 0);
    if w == "async" {
        t = t[l..].trim_start_matches([' ', '\t', '\n', '\r']);
        off += l;
    }
    let (w, l) = cv_word_at(t, 0);
    if w == "function" {
        let (_, _, nend) = cv_next_word(t, l); // optional fn name
        let (po, pe, ok) = cv_paren_span(t, nend);
        if !ok {
            return None;
        }
        let (bo, be, ok) = cv_brace_span(t, pe + 1);
        if !ok {
            return None;
        }
        return Some(CvArrowInfo {
            params: [off + po, off + pe],
            body: [off + bo, off + be],
            single: String::new(),
        });
    }
    if t.starts_with('(') {
        let (po, pe, ok) = cv_paren_span(t, 0);
        // cv_is_arrow_at starts from the char AFTER the span end — pe itself
        // is the ')' and is not whitespace, so the check below never fired
        // and a destructured arrow was never recognized at all
        if !ok || !cv_is_arrow_at(t, pe + 1) {
            return None;
        }
        return Some(CvArrowInfo {
            params: [off + po, off + pe],
            body: [off + pe + 2, off + t.len()],
            single: String::new(),
        });
    }
    // single bare ident param: `x => …`
    let (name, nlen) = cv_word_at(t, 0);
    if nlen > 0 && cv_is_arrow_at(t, nlen) {
        return Some(CvArrowInfo {
            params: [0, 0],
            body: [off + nlen + 2, off + t.len()],
            single: name,
        });
    }
    None
}

// ------------------------------------------------------------------ imports

pub struct Imports {
    pub imports: Vec<String>,
    pub icons: Vec<String>,
    pub import_map: std::collections::HashMap<String, String>,
    pub module_of: std::collections::HashMap<String, String>,
}

struct CvImpSpec {
    local: String,
    imported: String,
    ns: bool,
}

/// Imports come from the ORIGINAL source. babel counted every
/// ImportDeclaration's source — INCLUDING type-only imports, which the
/// downgrade drops.
pub fn imports_of(src: &str) -> Imports {
    let mut out = Imports {
        imports: Vec::new(),
        icons: Vec::new(),
        import_map: std::collections::HashMap::new(),
        module_of: std::collections::HashMap::new(),
    };
    let b = src.as_bytes();
    let (mut depth, mut i) = (0i32, 0usize);
    while i < b.len() {
        let e = cv_mask_end(src, i);
        if e > 0 {
            i = e;
            continue;
        }
        let c = b[i];
        if c == b'(' || c == b'[' || c == b'{' {
            depth += 1;
            i += 1;
            continue;
        }
        if c == b')' || c == b']' || c == b'}' {
            depth -= 1;
            i += 1;
            continue;
        }
        if depth > 0 {
            i += 1;
            continue;
        }
        let (w, _) = cv_word_at(src, i);
        if w == "import" {
            let stmt_end = cv_skip_stmt(src, i);
            let (from, list, default_local) = cv_parse_import(&src[i..stmt_end]);
            if !from.is_empty() {
                out.imports.push(from.clone());
                for sp in &list {
                    if sp.ns {
                        // babel: s.imported is undefined → local name
                        if from == "lucide-react" {
                            out.icons.push(sp.local.clone());
                        }
                        continue;
                    }
                    out.import_map.insert(sp.local.clone(), sp.imported.clone());
                    out.module_of.insert(sp.local.clone(), from.clone());
                    if from == "lucide-react" {
                        out.icons.push(sp.imported.clone());
                    }
                }
                if !default_local.is_empty() && from == "lucide-react" {
                    out.icons.push(default_local.clone());
                }
            }
            i = stmt_end;
            continue;
        }
        i += 1;
    }
    out
}

/// Extracts the module path and specifiers from an import statement's text
/// (default/namespace specifiers reported separately).
fn cv_parse_import(stmt: &str) -> (String, Vec<CvImpSpec>, String) {
    let mut strs: Vec<String> = Vec::new();
    let b = stmt.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let e = cv_mask_end(stmt, i);
        if e > 0 {
            if b[i] == b'"' || b[i] == b'\'' {
                strs.push(cv_unquote(&stmt[i..e]));
            }
            i = e;
            continue;
        }
        i += 1;
    }
    if strs.is_empty() {
        return (String::new(), Vec::new(), String::new());
    }
    let ob = stmt.find('{');
    let eb = ob.and_then(|o| cv_match_bracket(stmt, o, b'{', b'}'));
    let from = strs[strs.len() - 1].clone();
    let mut list: Vec<CvImpSpec> = Vec::new();
    let mut head = stmt;
    if let (Some(ob), Some(eb)) = (ob, eb) {
        if eb > 0 {
            head = &stmt[..ob];
            for f in cv_split_top(&stmt[ob + 1..eb]) {
                let fields: Vec<&str> = f.split_whitespace().collect();
                let mut k = 0;
                if k < fields.len() && fields[k] == "type" {
                    k += 1;
                }
                if k >= fields.len() {
                    continue;
                }
                let imported = fields[k].to_string();
                let mut local = imported.clone();
                if k + 2 < fields.len() && fields[k + 1] == "as" {
                    local = fields[k + 2].to_string();
                }
                list.push(CvImpSpec {
                    local,
                    imported,
                    ns: false,
                });
            }
        }
    }
    static NS_RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static DEF_RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let ns_re = NS_RE.get_or_init(|| Regex::new(r"import\s*\*\s*as\s+([A-Za-z_$][A-Za-z0-9_$]*)").unwrap());
    let def_re = DEF_RE.get_or_init(|| Regex::new(r"import\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*,?\s*(?:\{|from\b)").unwrap());
    // \b is ASCII in Go: pre-anchor the match by checking the preceding byte
    let ns_m = ns_re
        .captures_iter(head)
        .find(|m| starts_at_word(head, m.get(0).unwrap().start()));
    if let Some(m) = ns_m {
        list.push(CvImpSpec {
            local: m[1].to_string(),
            imported: String::new(),
            ns: true,
        });
    } else {
        let def_m = def_re
            .captures_iter(head)
            .find(|m| starts_at_word(head, m.get(0).unwrap().start()));
        if let Some(m) = def_m {
            return (from, list, m[1].to_string());
        }
    }
    (from, list, String::new())
}

/// Go's \b is ASCII: emulate by requiring the byte before the match to be a
/// non-word byte (or start of string).
fn starts_at_word(s: &str, start: usize) -> bool {
    if start == 0 {
        return true;
    }
    let b = s.as_bytes();
    let p = b[start - 1];
    !(p.is_ascii_alphanumeric() || p == b'_' || p == b'$')
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A declarator whose raw text starts with whitespace (`var<sp>X = …`):
    /// `eq` is an index into the TRIMMED text while `dcl.off` points at the
    /// RAW segment — mixing them shifted every arrow span left by the trim
    /// amount, and destructured param defaults became silently
    /// unextractable (`( { size = "md" }` → the `{`-branch never fired).
    #[test]
    fn unit_arrow_spans_skip_declaration_leading_space() {
        let js = "var Chart = ({ size = \"md\" }) => ({ render: 1 });";
        let t = scan_top_js(js).unwrap();
        assert!(t.decls[0].is_arrow, "the arrow must be recognized");
        let d = &t.decls[0];
        let params = &js[d.params[0]..d.params[1] + 1];
        assert!(
            params.starts_with('(') && params.ends_with(')'),
            "params span must quote the real ( … ) in the source, got {params:?}"
        );
        assert!(params.contains("size = \"md\""), "params span must carry the default, got {params:?}");
        let body = &js[d.body[0]..d.body[1] + 1];
        assert!(body.contains("render"), "body span must quote the real body, got {body:?}");
    }

    /// ASI tail: `cv_skip_stmt` stops at the newline, so dcl.text keeps its
    /// trailing `\n` — the rebase must count trim_START only or the spans
    /// shift right by the tail length.
    #[test]
    fn unit_arrow_spans_survive_a_trailing_newline_tail() {
        let js = "var Chart = ({ size = \"md\" }) => ({ render: 1 })\nvar Other = 2;";
        let t = scan_top_js(js).unwrap();
        let d = t.decls.iter().find(|d| d.name == "Chart").unwrap();
        assert!(d.is_arrow);
        let params = &js[d.params[0]..d.params[1] + 1];
        assert_eq!(params, "({ size = \"md\" })", "params exact span, got {params:?}");
    }
}
