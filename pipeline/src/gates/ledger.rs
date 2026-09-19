//! Port of pipeline/ledger.go + jssource.go — the recorded-difference ledger.
//!
//! EXEMPTIONS.md is GENERATED from gates/ledger.json (`ledger --render`), so
//! the human-readable file cannot drift from the checked one. Every entry
//! declares how it ENDS (permanent | auto-dissolve | debt); budgets are the
//! ratchet done right (fails both on growth AND on unrecorded shrinkage).
//!
//! Key order is load-bearing: gates/ledger.json and exemptions.json are
//! committed, and a second ad-hoc JSON round trip through unordered maps
//! would reshuffle them on every write. serde_json's preserve_order feature
//! replaces Go's jsonKeyOrder helper directly.

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::OnceLock;

pub const RENDERED_PATH: &str = "EXEMPTIONS.md";
pub const GOLDEN_EX_PATH: &str = "src/registry/upstream-snapshot/exemptions.json";
pub const CONTRACTS_DIR: &str = "tools/contracts/components";
pub const EMITTER_CSS: &str = "src/emitter/css.mjs";
pub const EMITTER_SKIN: &str = "src/emitter/skin.mjs";
const TODO_REASON_PFX: &str = "TODO";

fn ledger_classes() -> &'static [&'static str] {
    &["permanent", "auto-dissolve", "debt"]
}

// ---------------------------------------------------------------- jssource

fn re_js_line_comment() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)//[^\n]*$").unwrap())
}
fn re_js_string() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#""((?:[^"\\]|\\.)*)""#).unwrap())
}
fn re_js_attr_list() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#""((?:[^"\\]|\\.)*)"[\t\n\f\r ]*:[\t\n\f\r ]*\[([^\]]*)\]"#).unwrap()
    })
}

/// Every double-quoted string in src, in order, with `//` comments removed
/// first so a commented-out entry is not counted.
pub fn js_strings_in(src: &str) -> Vec<String> {
    let src = re_js_line_comment().replace_all(src, "");
    re_js_string()
        .captures_iter(&src)
        .map(|m| js_unescape(&m[1]))
        .collect()
}

/// Resolves the escapes a JS string literal can carry in these sources. \u is
/// handled because upstream reasons contain em dashes.
pub fn js_unescape(s: &str) -> String {
    if !s.contains('\\') {
        return s.to_string();
    }
    // Byte-faithful like Go jssource.go: non-escape bytes are copied raw
    // (multibyte UTF-8 stays intact), escapes resolve to ASCII or a single
    // \uXXXX code point. A surrogate or bad code point renders as U+FFFD,
    // matching Go's WriteRune(RuneError).
    let b = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'\\' || i + 1 >= b.len() {
            out.push(b[i]);
            i += 1;
            continue;
        }
        i += 1;
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
            b'u' => {
                let hex: Vec<u8> = b[i + 1..(i + 5).min(b.len())].to_vec();
                if hex.len() == 4 && hex.iter().all(|c| c.is_ascii_hexdigit()) {
                    let r = u32::from_str_radix(std::str::from_utf8(&hex).unwrap(), 16).unwrap();
                    let c = char::from_u32(r).unwrap_or('\u{FFFD}');
                    let mut buf = [0u8; 4];
                    out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                    i += 5;
                } else {
                    out.push(b'u');
                    i += 1;
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8(out)
        .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned())
}

/// jsBalanced returns the text between the opening delimiter at src[open] and
/// its match, exclusive. Delegates to convert.go's cvMatchBracket, so it
/// skips strings, template-literal interpolation, and comments.
fn js_balanced(src: &str, open: usize, oc: u8, cc: u8) -> Result<&str, String> {
    let end = crate::convert::scan::cv_match_bracket(src, open, oc, cc)
        .ok_or_else(|| format!("unbalanced {:?} ... {:?}", oc as char, cc as char))?;
    Ok(&src[open + 1..end])
}

/// Extracts `export const NAME = new Set([ "a", "b" ])` — the shape every
/// allowlist in this repo uses. The name is part of the anchor, so renaming
/// the export in JS fails here instead of silently emptying the set.
pub fn js_set_literal(src: &str, name: &str) -> Result<Vec<String>, String> {
    let re = Regex::new(&format!(
        r"(?:export[\t\n\f\r ]+)?const[\t\n\f\r ]+{}[\t\n\f\r ]*=[\t\n\f\r ]*new[\t\n\f\r ]+Set\([\t\n\f\r ]*\[",
        regex::escape(name)
    ))
    .unwrap();
    let m = re
        .find(src)
        .ok_or_else(|| format!("{}: `new Set([...])` declaration not found", name))?;
    let body = js_balanced(src, m.end() - 1, b'[', b']')
        .map_err(|e| format!("{}: {}", name, e))?;
    Ok(js_strings_in(body))
}

/// Returns the text of `name: { ... }` within src, or None when the field is
/// absent. Absence is legitimate for optional fields; a field that IS present
/// and malformed is an error.
pub fn js_object_field<'a>(src: &'a str, name: &str) -> Result<Option<&'a str>, String> {
    let re = Regex::new(&format!(
        r"(?m)^[\t\n\f\r ]*{}[\t\n\f\r ]*:[\t\n\f\r ]*\{{",
        regex::escape(name)
    ))
    .unwrap();
    let m = match re.find(src) {
        Some(m) => m,
        None => return Ok(None),
    };
    let body =
        js_balanced(src, m.end() - 1, b'{', b'}').map_err(|e| format!("{}: {}", name, e))?;
    Ok(Some(body))
}

/// Reports whether `name: false` appears at field position. Anchored to the
/// line start so a mention inside a comment or a longer identifier does not
/// count.
pub fn js_field_is_false(src: &str, name: &str) -> bool {
    let re = Regex::new(&format!(
        r"(?m)^[\t\n\f\r ]*{}[\t\n\f\r ]*:[\t\n\f\r ]*false[\t\n\f\r ]*,?[\t\n\f\r ]*$",
        regex::escape(name)
    ))
    .unwrap();
    re.is_match(src)
}

#[derive(Debug)]
pub struct JsAttrEntry {
    pub key: String,
    pub values: Vec<String>,
}

/// Parses an `ignoreAttrs`-shaped object body into ordered (key, values)
/// pairs. Order is preserved because the ledger ids derived from it are
/// written to a file whose diff should stay readable.
pub fn js_attr_map(body: &str) -> Vec<JsAttrEntry> {
    let body = re_js_line_comment().replace_all(body, "");
    re_js_attr_list()
        .captures_iter(&body)
        .map(|m| JsAttrEntry {
            key: js_unescape(&m[1]),
            values: js_strings_in(&m[2]),
        })
        .collect()
}

// ---------------------------------------------------------------- the file

#[derive(Deserialize, Clone, Default)]
pub struct LedgerEntry {
    #[serde(default)]
    pub class: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub source: String,
    #[serde(default, rename = "recorded_at_pin")]
    pub recorded: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct LedgerBudget {
    #[serde(default)]
    pub max: i64,
    #[serde(default)]
    pub target: i64,
    #[serde(default)]
    pub class: String,
    #[serde(default)]
    pub reason: String,
}

/// Keeps key order alongside the maps: gates/ledger.json is committed, and a
/// map-only round trip would reshuffle 66 entries on every write, turning a
/// one-line change into a whole-file diff.
#[derive(Deserialize, Default)]
pub struct LedgerFile {
    #[serde(default)]
    pub pin: String,
    #[serde(default)]
    pub entries: HashMap<String, LedgerEntry>,
    #[serde(default)]
    pub budgets: HashMap<String, LedgerBudget>,
    #[serde(default)]
    pub notes: Vec<String>,
    // filled from the raw JSON's preserve_order maps after deserialize:
    #[serde(skip)]
    pub entry_order: Vec<String>,
    #[serde(skip)]
    pub budget_order: Vec<String>,
}

pub fn read_ledger(root: &Path) -> Result<LedgerFile, String> {
    let b = std::fs::read_to_string(root.join(super::LEDGER_PATH))
        .map_err(|e| format!("{}: {}", super::LEDGER_PATH, e))?;
    let mut l: LedgerFile = serde_json::from_str(&b)
        .map_err(|e| format!("{}: {}", super::LEDGER_PATH, e))?;
    // key order from the same bytes, via preserve_order maps
    let top: Value = serde_json::from_str(&b).map_err(|e| format!("{}: {}", super::LEDGER_PATH, e))?;
    l.entry_order = key_order(&top, "entries")?;
    l.budget_order = key_order(&top, "budgets")?;
    Ok(l)
}

/// The keys of a top-level object field in file order (serde_json
/// preserve_order makes this a plain iteration). A present-but-non-object
/// field is an error, as in Go — an empty list would silently lose every
/// entry on the next write.
fn key_order(top: &Value, field: &str) -> Result<Vec<String>, String> {
    match top.get(field) {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Object(o)) => Ok(o.keys().cloned().collect()),
        Some(_) => Err(format!("{} is not an object", field)),
    }
}

/// Emits JSON.stringify(l, null, 2) + "\n" — the bytes the JS wrote, so the
/// port does not reformat a committed file on its first run.
impl LedgerFile {
    pub fn write(&self, root: &Path) -> Result<(), String> {
        use crate::jsonorder::{marshal_js, Json, JsonObj};
        let mut entries = JsonObj::new();
        for id in &self.entry_order {
            let e = &self.entries[id];
            entries = entries.add(
                id,
                Json::Obj(
                    JsonObj::new()
                        .add("class", Json::Str(e.class.clone()))
                        .add("reason", Json::Str(e.reason.clone()))
                        .add("source", Json::Str(e.source.clone()))
                        .add("recorded_at_pin", Json::Str(e.recorded.clone()))
                        .into_pairs(),
                ),
            );
        }
        let mut budgets = JsonObj::new();
        for n in &self.budget_order {
            let b = &self.budgets[n];
            budgets = budgets.add(
                n,
                Json::Obj(
                    JsonObj::new()
                        .add("max", Json::Int(b.max))
                        .add("target", Json::Int(b.target))
                        .add("class", Json::Str(b.class.clone()))
                        .add("reason", Json::Str(b.reason.clone()))
                        .into_pairs(),
                ),
            );
        }
        let notes: Vec<Json> = self.notes.iter().map(|n| Json::Str(n.clone())).collect();
        let out = JsonObj::new()
            .add("pin", Json::Str(self.pin.clone()))
            .add("entries", Json::from_obj(entries))
            .add("budgets", Json::from_obj(budgets))
            .add("notes", Json::Arr(notes));
        std::fs::write(
            root.join(super::LEDGER_PATH),
            format!("{}\n", marshal_js(&Json::from_obj(out))),
        )
        .map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------- sources

/// One exemption the codebase currently claims, and where from.
pub struct SourceId {
    pub id: String,
    pub source: String,
}

/// Derives every id the sources currently claim. The ids ARE the contract
/// with the sources, so this must stay the same derivation the JS used — an
/// id that changes shape reads as "one exemption vanished, an undocumented
/// one appeared" and fails the gate in both directions at once.
pub fn collect_source_ids(root: &Path) -> Result<Vec<SourceId>, String> {
    let mut out: Vec<SourceId> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let add = |id: String, src: &str, out: &mut Vec<SourceId>, seen: &mut HashSet<String>| {
        if seen.insert(id.clone()) {
            out.push(SourceId {
                id,
                source: src.to_string(),
            });
        }
    };

    let ents = std::fs::read_dir(root.join(CONTRACTS_DIR))
        .map_err(|e| format!("{}: {}", CONTRACTS_DIR, e))?;
    let mut names: Vec<String> = ents
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".mjs"))
        .collect();
    names.sort();
    for f in &names {
        let name = f.strip_suffix(".mjs").unwrap_or(f);
        let src = std::fs::read_to_string(root.join(CONTRACTS_DIR).join(f))
            .map_err(|e| format!("{}/{}: {}", CONTRACTS_DIR, f, e))?;
        if js_field_is_false(&src, "mountedCheck") {
            add(format!("mounted-check:{}", name), "contracts", &mut out, &mut seen);
        }
        if js_field_is_false(&src, "mountedClasses") {
            add(format!("mounted-classes:{}", name), "contracts", &mut out, &mut seen);
        }
        let attrs = js_object_field(&src, "ignoreAttrs")
            .map_err(|e| format!("{}/{}: {}", CONTRACTS_DIR, f, e))?;
        if let Some(attrs) = attrs {
            for e in js_attr_map(attrs) {
                for a in &e.values {
                    add(
                        format!("ignore-attrs:{}:{}:{}", name, e.key, a),
                        "contracts",
                        &mut out,
                        &mut seen,
                    );
                }
            }
        }
    }

    for r in golden_reasons(root)? {
        add(format!("golden:{}", r), "golden", &mut out, &mut seen);
    }

    let css_src = std::fs::read_to_string(root.join(EMITTER_CSS))
        .map_err(|e| format!("{}: {}", EMITTER_CSS, e))?;
    let dead = js_set_literal(&css_src, "DEAD_UTILITIES")
        .map_err(|e| format!("{}: {}", EMITTER_CSS, e))?;
    for t in dead {
        add(format!("dead-utility:{}", t), "emitter", &mut out, &mut seen);
    }

    let skin_src = std::fs::read_to_string(root.join(EMITTER_SKIN))
        .map_err(|e| format!("{}: {}", EMITTER_SKIN, e))?;
    let skin = js_set_literal(&skin_src, "SKIN_ALLOWLIST")
        .map_err(|e| format!("{}: {}", EMITTER_SKIN, e))?;
    for t in skin {
        add(format!("skin-allowlist:{}", t), "emitter", &mut out, &mut seen);
    }
    Ok(out)
}

/// src/registry/upstream-snapshot/exemptions.json, whose key order is
/// load-bearing (it is committed) and whose values carry the reason each demo
/// is exempt from the golden dual hop.
pub struct GoldenExemptions {
    pub order: Vec<String>,
    pub reasons: HashMap<String, String>,
}

pub fn read_golden_exemptions(root: &Path) -> Result<GoldenExemptions, String> {
    let b = std::fs::read_to_string(root.join(GOLDEN_EX_PATH))
        .map_err(|e| format!("{}: {}", GOLDEN_EX_PATH, e))?;
    #[derive(Deserialize)]
    struct Ex {
        #[serde(default)]
        reason: String,
    }
    #[derive(Deserialize)]
    struct Raw {
        #[serde(default)]
        examples: HashMap<String, Ex>,
    }
    let raw: Raw =
        serde_json::from_str(&b).map_err(|e| format!("{}: {}", GOLDEN_EX_PATH, e))?;
    let top: Value =
        serde_json::from_str(&b).map_err(|e| format!("{}: {}", GOLDEN_EX_PATH, e))?;
    let order = key_order(&top, "examples")?;
    let mut reasons = HashMap::new();
    for (k, v) in raw.examples {
        reasons.insert(k, v.reason);
    }
    Ok(GoldenExemptions { order, reasons })
}

impl GoldenExemptions {
    /// Emits JSON.stringify(golden, null, 1) + "\n" — indent 1, as the JS
    /// wrote it, so dissolving does not reformat the whole file.
    pub fn write(&self, root: &Path) -> Result<(), String> {
        use crate::jsonorder::{marshal_js_step, Json, JsonObj};
        let mut examples = JsonObj::new();
        for demo in &self.order {
            examples = examples.add(
                demo,
                Json::Obj(
                    JsonObj::new()
                        .add("reason", Json::Str(self.reasons[demo].clone()))
                        .into_pairs(),
                ),
            );
        }
        let out = JsonObj::new().add("examples", Json::from_obj(examples));
        std::fs::write(
            root.join(GOLDEN_EX_PATH),
            format!("{}\n", marshal_js_step(&Json::from_obj(out), "", " ")),
        )
        .map_err(|e| e.to_string())
    }
}

/// The distinct reasons, in file order.
pub fn golden_reasons(root: &Path) -> Result<Vec<String>, String> {
    let g = read_golden_exemptions(root)?;
    let mut out: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for demo in &g.order {
        let r = &g.reasons[demo];
        if seen.insert(r.clone()) {
            out.push(r.clone());
        }
    }
    Ok(out)
}

/// Reads the live value of every budgeted number from the same place the gate
/// that reports it reads. This is the ONLY definition of each metric; a second
/// one is how a ratchet silently stops ratcheting.
///
/// A value of -1 means "could not read", which fails. Absence from the map
/// means "not checked here" — the coverage.* budgets need the IR, so the
/// coverage gate owns them.
pub fn collect_budget_values(root: &Path) -> Result<HashMap<String, i64>, String> {
    let mut v: HashMap<String, i64> = HashMap::new();

    let g = read_golden_exemptions(root)?;
    v.insert("golden.exempt-demos".to_string(), g.order.len() as i64);

    // The sweep's own dead-family list is the single definition (the Go
    // source it used to be counted from is gone with the Go engine).
    v.insert(
        "interactivity.dead-families".to_string(),
        crate::tools::interactivity_sweep::SWEEP_KNOWN_DEAD.len() as i64,
    );

    for (name, path) in [
        ("demo-parity.dirty-cells", "gates/demo-parity-baseline.json"),
        ("path-parity.dirty-cells", "gates/path-parity-baseline.json"),
        ("style-parity.dirty-cells", "gates/style-parity-baseline.json"),
    ] {
        v.insert(name.to_string(), baseline_cell_count(root, path));
    }
    Ok(v)
}

fn baseline_cell_count(root: &Path, path: &str) -> i64 {
    let Ok(b) = std::fs::read_to_string(root.join(path)) else {
        return -1;
    };
    #[derive(Deserialize)]
    struct Doc {
        #[serde(default)]
        cells: Vec<Value>,
    }
    match serde_json::from_str::<Doc>(&b) {
        Ok(d) => d.cells.len() as i64,
        Err(_) => -1,
    }
}

pub fn current_pin(root: &Path) -> Result<String, String> {
    let p = super::pin::read_pin(root)?;
    Ok(p.shadcn_ui.tag)
}

/// Golden reasons carry their own end-condition in the text; trust it so a
/// re-pin dissolves them without anyone re-classifying 147 rows by hand.
pub fn class_of_golden_reason(r: &str) -> &'static str {
    let l = r.to_lowercase();
    for sig in ["re-check on re-pin", "deploy lag", "frame lag"] {
        if l.contains(sig) {
            return "auto-dissolve";
        }
    }
    "permanent"
}

// ---------------------------------------------------------------- verify

pub fn gate_ledger(root: &Path) -> Result<(), String> {
    let l = read_ledger(root)?;
    let sources = collect_source_ids(root)?;
    let in_sources: HashSet<&str> = sources.iter().map(|s| s.id.as_str()).collect();

    let mut problems: Vec<String> = Vec::new();
    for id in &l.entry_order {
        let e = &l.entries[id];
        if !ledger_classes().contains(&e.class.as_str()) {
            problems.push(format!("{}: unknown class {:?}", id, e.class));
        }
        if e.reason.len() < 8 {
            problems.push(format!("{}: missing or trivial reason", id));
        }
        if e.recorded.is_empty() {
            problems.push(format!("{}: no recorded_at_pin", id));
        }
    }

    let mut undocumented: Vec<String> = sources
        .iter()
        .filter(|s| !l.entries.contains_key(&s.id))
        .map(|s| s.id.clone())
        .collect();
    undocumented.sort();
    let mut stale: Vec<String> = l
        .entry_order
        .iter()
        .filter(|id| !in_sources.contains(id.as_str()))
        .cloned()
        .collect();
    stale.sort();
    if !undocumented.is_empty() {
        problems.push(format!(
            "exemptions in the sources with no ledger entry (add them, with a class):\n    {}",
            undocumented.join("\n    ")
        ));
    }
    if !stale.is_empty() {
        problems.push(format!(
            "ledger entries whose source flag is gone (delete them):\n    {}",
            stale.join("\n    ")
        ));
    }

    // The human render must match this ledger byte-for-byte — a ledger edit
    // without `make ledger-render` used to ship an EXEMPTIONS.md documenting
    // budgets and classes that no longer existed, green forever.
    let rendered_path = root.join(RENDERED_PATH);
    let committed = match std::fs::read(&rendered_path) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(format!(
                "FAIL  ledger\n  {} is missing — run `make ledger-render` and commit it",
                RENDERED_PATH
            ));
        }
        Err(e) => return Err(format!("FAIL  ledger\n  reading {}: {}", RENDERED_PATH, e)),
    };
    let rendered = render_ledger_markdown(root)?;
    if committed.trim_end() != rendered.trim_end() {
        problems.push(
            "EXEMPTIONS.md is stale against gates/ledger.json — run `make ledger-render` and commit the result"
                .to_string(),
        );
    }

    let values = collect_budget_values(root)?;
    for name in &l.budget_order {
        let b = &l.budgets[name];
        let actual = match values.get(name) {
            None => {
                if budget_checked_by_coverage(name) {
                    continue; // checked by the coverage gate's test
                }
                problems.push(format!("budget {}: no live value known", name));
                continue;
            }
            Some(v) => *v,
        };
        if actual < 0 {
            problems.push(format!(
                "budget {}: could not read the live value",
                name
            ));
            continue;
        }
        if actual > b.max {
            problems.push(format!(
                "budget {}: {} > recorded max {} — this number may only shrink",
                name, actual, b.max
            ));
        } else if actual < b.max {
            problems.push(format!(
                "budget {}: {} < recorded max {} — it improved; \
re-record so the slack cannot be silently re-spent:  pipeline ledger --record",
                name, actual, b.max
            ));
        }
    }

    if !problems.is_empty() {
        return Err(format!("FAIL  ledger\n  {}", problems.join("\n  ")));
    }

    let pin = current_pin(root)?;
    let mut stale_auto: Vec<String> = Vec::new();
    let mut by_class: HashMap<String, i64> = HashMap::new();
    for id in &l.entry_order {
        let e = &l.entries[id];
        *by_class.entry(e.class.clone()).or_insert(0) += 1;
        if e.class == "auto-dissolve" && e.recorded != pin {
            stale_auto.push(id.clone());
        }
    }
    let mut counts: Vec<String> = Vec::new();
    for c in ledger_classes() {
        let n = by_class.get(*c).copied().unwrap_or(0);
        if n > 0 {
            counts.push(format!("{} {}", n, c));
        }
    }
    let mut msg = format!(
        "PASS  ledger ({} exemptions: {}; {} budgets at their recorded max)",
        l.entry_order.len(),
        counts.join(", "),
        l.budget_order.len()
    );
    if !stale_auto.is_empty() {
        msg += &format!(
            "\n  note: {} auto-dissolve entries predate pin {} — they should have dissolved at the last re-pin",
            stale_auto.len(),
            pin
        );
    }
    println!("{}", msg);
    Ok(())
}

// ---------------------------------------------------------------- record

/// Reconciles the ledger with the sources: add missing ids, drop stale ones,
/// re-record budgets. Reasons for new ids are harvested from the id itself
/// where it carries one (golden), otherwise flagged TODO so the human writes
/// the reason rather than the tool inventing it.
pub fn ledger_record(root: &Path) -> Result<(), String> {
    let mut l = read_ledger(root)?;
    let pin = current_pin(root)?;
    l.pin = pin.clone();
    let sources = collect_source_ids(root)?;
    let in_sources: HashSet<String> = sources.iter().map(|s| s.id.clone()).collect();

    let mut added = 0;
    let mut dropped = 0;
    for s in &sources {
        if l.entries.contains_key(&s.id) {
            continue;
        }
        let mut reason = "TODO: state why this difference is accepted".to_string();
        let mut class = "permanent".to_string();
        if let Some(r) = s.id.strip_prefix("golden:") {
            reason = r.to_string();
            class = class_of_golden_reason(r).to_string();
        }
        l.entries.insert(
            s.id.clone(),
            LedgerEntry {
                class,
                reason,
                source: s.source.clone(),
                recorded: pin.clone(),
            },
        );
        l.entry_order.push(s.id.clone());
        added += 1;
    }
    let mut kept: Vec<String> = Vec::new();
    for id in &l.entry_order {
        if !in_sources.contains(id) {
            l.entries.remove(id);
            dropped += 1;
            continue;
        }
        kept.push(id.clone());
    }
    l.entry_order = kept;

    let values = collect_budget_values(root)?;
    for name in &l.budget_order {
        let b = l.budgets.get_mut(name).unwrap();
        if let Some(v) = values.get(name) {
            if *v >= 0 && *v != b.max {
                println!("  budget {}: {} -> {}", name, b.max, v);
                b.max = *v;
            }
        }
    }
    l.write(root)?;
    let mut todo = 0;
    for e in l.entries.values() {
        if e.reason.starts_with(TODO_REASON_PFX) {
            todo += 1;
        }
    }
    println!(
        "ledger recorded: +{} -{}, {} entries",
        added,
        dropped,
        l.entry_order.len()
    );
    if todo > 0 {
        println!(
            "  {} entries still need a real reason (search for TODO in {})",
            todo,
            super::LEDGER_PATH
        );
    }
    Ok(())
}

// -------------------------------------------------------------- dissolve

/// Called by the upstream drill right after a re-pin: delete every
/// auto-dissolve entry recorded against a DIFFERENT pin, so the rebuild has
/// to re-earn each one. Entries recorded at the current pin stay — a
/// same-tag drill (the self-test) must be a no-op here.
///
/// The golden exemptions are ALSO a source file (one row per demo): the demos
/// whose reason dissolved are pruned there too.
pub fn ledger_dissolve(root: &Path) -> Result<(), String> {
    let mut l = read_ledger(root)?;
    let pin = current_pin(root)?;
    let mut gone: Vec<String> = Vec::new();
    for id in &l.entry_order {
        let e = &l.entries[id];
        if e.class == "auto-dissolve" && e.recorded != pin {
            gone.push(id.clone());
        }
    }
    let gone_set: HashSet<&String> = gone.iter().collect();
    for id in &gone {
        l.entries.remove(id);
    }
    l.entry_order.retain(|id| !gone_set.contains(id));
    l.pin = pin.clone();

    let mut gone_reasons: HashSet<String> = HashSet::new();
    for id in &gone {
        if let Some(r) = id.strip_prefix("golden:") {
            gone_reasons.insert(r.to_string());
        }
    }
    let mut g = read_golden_exemptions(root)?;
    let mut pruned = 0;
    let mut kept_demos: Vec<String> = Vec::new();
    for demo in &g.order {
        if gone_reasons.contains(&g.reasons[demo]) {
            g.reasons.remove(demo);
            pruned += 1;
            continue;
        }
        kept_demos.push(demo.clone());
    }
    g.order = kept_demos;
    g.write(root)?;
    // pruning is a deliberate shrink of the golden budget — record it here,
    // otherwise the drill's own verify step reports the improvement as an
    // UNEXPECTED failure
    if let Some(b) = l.budgets.get_mut("golden.exempt-demos") {
        b.max = g.order.len() as i64;
    }
    l.write(root)?;
    println!(
        "ledger dissolved: removed {} auto-dissolve entries recorded before {} ({} golden demo exemptions pruned); re-run the gates and re-record whatever legitimately survives",
        gone.len(),
        pin,
        pruned
    );
    for (i, id) in gone.iter().enumerate() {
        if i >= 10 {
            println!("  … +{} more", gone.len() - 10);
            break;
        }
        println!("  - {}", id);
    }
    Ok(())
}

// ---------------------------------------------------------------- render

/// The EXEMPTIONS.md body exactly as `ledger --render` writes it. The gate
/// renders this and byte-compares it against the committed file, so the
/// "cannot drift" promise in the module header is enforced, not assumed.
pub fn render_ledger_markdown(root: &Path) -> Result<String, String> {
    let l = read_ledger(root)?;
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for id in &l.entry_order {
        let c = l.entries[id].class.clone();
        groups.entry(c).or_default().push(id.clone());
    }
    let section = |title: &str, key: &str, blurb: &str| -> String {
        let mut rows = groups.get(key).cloned().unwrap_or_default();
        // Byte order, where the JS used localeCompare.
        rows.sort();
        let mut lines = vec![
            format!("## {} ({})", title, rows.len()),
            String::new(),
            blurb.to_string(),
            String::new(),
            "| Id | Reason | Recorded at |".to_string(),
            "|---|---|---|".to_string(),
        ];
        for id in &rows {
            let e = &l.entries[id];
            lines.push(format!(
                "| `{}` | {} | {} |",
                id,
                e.reason.replace('|', "\\|"),
                e.recorded
            ));
        }
        lines.push(String::new());
        lines.join("\n")
    };
    let mut md = vec![
        "# EXEMPTIONS — the recorded-difference ledger".to_string(),
        String::new(),
        "<!-- GENERATED from gates/ledger.json by `pipeline ledger --render`.".to_string(),
        "     Do not edit by hand: the `ledger` gate fails when this file and".to_string(),
        "     the JSON disagree, and the next render will overwrite whatever".to_string(),
        "     you wrote. -->".to_string(),
        String::new(),
        format!(
            "Pin: `{}` · {} exemptions · {} budgets",
            l.pin,
            l.entry_order.len(),
            l.budget_order.len()
        ),
        String::new(),
        r#"Every "known difference, accepted for a reason" lives here, and every entry"#.to_string(),
        "declares **how it ends**. The `ledger` gate keeps this list in lockstep".to_string(),
        "with the sources in both directions: a new exemption with no entry fails,".to_string(),
        "an entry whose source flag vanished fails.".to_string(),
        String::new(),
        section(
            "Permanent",
            "permanent",
            "Real engine or by-design differences. These do not dissolve; upstream would have to change.",
        ),
        section(
            "Auto-dissolve on re-pin",
            "auto-dissolve",
            "Deploy lag, SSR-frame lag and other pin-relative differences. `make upstream` \
DELETES every one of these after a re-pin and lets the gates re-earn them — \
nobody reviews this section by hand.",
        ),
        section(
            "Debt",
            "debt",
            "Accepted for now, tracked to zero. Governed by the budgets below.",
        ),
        "## Budgets".to_string(),
        String::new(),
        "A budget may only shrink. Growing fails the `ledger` gate; shrinking without".to_string(),
        "re-recording also fails, so slack cannot be silently re-spent.".to_string(),
        String::new(),
        "| Metric | Max | Target | Reason |".to_string(),
        "|---|---|---|---|".to_string(),
    ];
    for n in &l.budget_order {
        let b = &l.budgets[n];
        md.push(format!(
            "| `{}` | {} | {} | {} |",
            n,
            b.max,
            b.target,
            b.reason.replace('|', "\\|")
        ));
    }
    md.push(String::new());
    md.push("## Work items".to_string());
    md.push(String::new());
    md.push("Not cross-checked — these track work, not accepted differences.".to_string());
    md.push(String::new());
    for n in &l.notes {
        md.push(format!("- [ ] {}", n));
    }
    md.push(String::new());
    Ok(md.join("\n"))
}

pub fn ledger_render(root: &Path) -> Result<(), String> {
    let md = render_ledger_markdown(root)?;
    std::fs::write(root.join(RENDERED_PATH), md).map_err(|e| e.to_string())?;
    println!("rendered {} from {}", RENDERED_PATH, super::LEDGER_PATH);
    Ok(())
}

/// The writing half, as a subcommand: these mutate the tree and assert
/// nothing, so they are not tests. --verify is the gate (a #[test] here).
pub fn run_ledger(args: &[String]) -> i32 {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("pipeline: {}", e);
            return 1;
        }
    };
    let has = |f: &str| args.iter().any(|a| a == f);
    let r = if has("--record") {
        ledger_record(&root)
    } else if has("--render") {
        ledger_render(&root)
    } else if has("--dissolve") {
        ledger_dissolve(&root)
    } else if has("--verify") {
        eprintln!(
            "the ledger GATE is a #[test]: cargo test -p pipeline ledger\nthis subcommand only writes: pipeline ledger --record|--render|--dissolve"
        );
        return 2;
    } else {
        eprintln!("usage: pipeline ledger --record|--render|--dissolve");
        return 2;
    };
    if let Err(e) = r {
        eprintln!("{}", e);
        return 1;
    }
    0
}


/// The one budget the coverage gate checks itself (coverage.rs reads this
/// ledger and ratchets its live uncovered-cell count). Exact name on
/// purpose: any other `coverage.*` budget has no checker and must fail the
/// gate like any other unknown budget, not ride the prefix past it.
fn budget_checked_by_coverage(name: &str) -> bool {
    name == super::coverage::COVERAGE_KEY
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real-tree ledger tests tamper the one committed EXEMPTIONS.md;
    /// serialize them so they cannot see each other's tampered state.
    static RENDER_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// coverage.uncovered-cells is the only budget the coverage gate checks
    /// itself; anything else named coverage.* has no checker and must fail
    /// the ledger gate instead of riding the prefix past it.
    #[test]
    fn unit_coverage_budget_skip_is_exact_not_prefix() {
        assert!(budget_checked_by_coverage(crate::gates::coverage::COVERAGE_KEY));
        assert!(!budget_checked_by_coverage("coverage.other"));
        assert!(!budget_checked_by_coverage("demo-parity.dirty-cells"));
    }
    use std::path::PathBuf;

    fn tree(files: &[(&str, &str)]) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "shadless-ledger-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        for (p, c) in files {
            let full = root.join(p);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, c).unwrap();
        }
        root
    }

    /// Go TestUnitLedgerRoundTripsKeyOrder: write(read(ledger.json)) must be
    /// byte-stable, entries AND budgets, in a file whose key order a Go map
    /// would reshuffle. Fixture is the Go test's own.
    #[test]
    fn unit_ledger_round_trips_key_order() {
        let original = r#"{
  "pin": "shadcn@4.19.0",
  "entries": {
    "zeta": {
      "class": "permanent",
      "reason": "a reason long enough",
      "source": "contracts",
      "recorded_at_pin": "shadcn@4.19.0"
    },
    "alpha": {
      "class": "debt",
      "reason": "another reason here",
      "source": "golden",
      "recorded_at_pin": "shadcn@4.19.0"
    }
  },
  "budgets": {
    "b.two": {
      "max": 2,
      "target": 0,
      "class": "debt",
      "reason": "second"
    },
    "b.one": {
      "max": 1,
      "target": 0,
      "class": "debt",
      "reason": "first"
    }
  },
  "notes": [
    "note one"
  ]
}
"#;
        let root = tree(&[("gates/ledger.json", original)]);
        let l = read_ledger(&root).unwrap();
        // declaration order, NOT sorted — "zeta" was written first
        assert_eq!(l.entry_order, vec!["zeta", "alpha"]);
        assert_eq!(l.budget_order, vec!["b.two", "b.one"]);
        l.write(&root).unwrap();
        let got = std::fs::read_to_string(root.join("gates/ledger.json")).unwrap();
        assert_eq!(got, original, "rewriting a ledger changed its bytes");
    }

    /// Go TestUnitLedgerWritesLiteralNonASCII: reasons carry em dashes and
    /// non-ASCII must pass through literally, not \u-escaped.
    #[test]
    fn unit_ledger_writes_literal_non_ascii() {
        let root = tree(&[(
            "gates/ledger.json",
            r#"{
  "pin": "p",
  "entries": {
    "x": { "class": "permanent", "reason": "café — em dash", "source": "golden", "recorded_at_pin": "p" }
  },
  "budgets": {},
  "notes": []
}
"#,
        )]);
        let mut l = read_ledger(&root).unwrap();
        l.write(&root).unwrap();
        let got = std::fs::read_to_string(root.join("gates/ledger.json")).unwrap();
        assert!(got.contains("café — em dash"), "non-ASCII escaped: {:?}", got);
    }

    /// Go TestUnitGoldenExemptionsIndentIsOne: the exemptions file must keep
    /// its indent-1 shape or dissolving reformats the whole file.
    #[test]
    fn unit_golden_exemptions_indent_is_one() {
        let original = "{\n \"examples\": {\n  \"a\": {\n   \"reason\": \"x\"\n  }\n }\n}\n";
        let root = tree(&[(GOLDEN_EX_PATH, original)]);
        let mut g = read_golden_exemptions(&root).unwrap();
        g.order.push("b".to_string());
        g.reasons.insert("b".to_string(), "y".to_string());
        g.write(&root).unwrap();
        let got = std::fs::read_to_string(root.join(GOLDEN_EX_PATH)).unwrap();
        assert_eq!(
            got,
            "{\n \"examples\": {\n  \"a\": {\n   \"reason\": \"x\"\n  },\n  \"b\": {\n   \"reason\": \"y\"\n  }\n }\n}\n"
        );
    }

    /// Go TestUnitClassOfGoldenReason.
    #[test]
    fn unit_class_of_golden_reason() {
        assert_eq!(class_of_golden_reason("re-check on re-pin"), "auto-dissolve");
        assert_eq!(class_of_golden_reason("Deploy lag"), "auto-dissolve");
        assert_eq!(class_of_golden_reason("iframe FRAME LAG on CI"), "auto-dissolve");
        assert_eq!(class_of_golden_reason("token drift vs live"), "permanent");
    }

    /// Go TestUnitLedgerBudgetRatchetBitesBothWays.
    #[test]
    fn unit_ledger_budget_ratchet_bites_both_ways() {
        let root = tree(&[(
            "gates/ledger.json",
            r#"{
  "pin": "p",
  "entries": {},
  "budgets": {
    "x": { "max": 5, "target": 0, "class": "debt", "reason": "ratchet" }
  },
  "notes": []
}
"#,
        )]);
        // over: fail
        let problems = budget_problems_for(&root, 6);
        assert!(problems.iter().any(|p| p.contains("6 > recorded max 5")));
        // under: ALSO fail (slack must be re-recorded)
        let problems = budget_problems_for(&root, 4);
        assert!(problems.iter().any(|p| p.contains("4 < recorded max 5")));
        // at: green
        let problems = budget_problems_for(&root, 5);
        assert!(problems.is_empty());
    }

    fn budget_problems_for(root: &Path, actual: i64) -> Vec<String> {
        let l = read_ledger(root).unwrap();
        let mut problems: Vec<String> = Vec::new();
        for name in &l.budget_order {
            let b = &l.budgets[name];
            if actual > b.max {
                problems.push(format!(
                    "budget {}: {} > recorded max {} — this number may only shrink",
                    name, actual, b.max
                ));
            } else if actual < b.max {
                problems.push(format!(
                    "budget {}: {} < recorded max {} — it improved",
                    name, actual, b.max
                ));
            }
        }
        problems
    }

    /// Go TestUnitLedgerCatchesUndocumentedExemption / StaleEntry:
    /// collectSourceIds is the derivation both directions are checked
    /// against; its contract pieces are pinned here.
    #[test]
    fn unit_ledger_source_id_derivation() {
        let body = r#"export default {
  mountedClasses: false,
  mountedCheck: false,
  ignoreAttrs: {
    "dialog": ["style"],
    "dialog-content": ["text"], // trailing comment
  },
}
"#;
        let root = tree(&[
            ("tools/contracts/components/dialog.mjs", body),
            (GOLDEN_EX_PATH, "{\"examples\":{}}"),
            (EMITTER_CSS, "export const DEAD_UTILITIES = new Set([\n  \"stale-dead\",\n])\n"),
            (EMITTER_SKIN, "export const SKIN_ALLOWLIST = new Set([\n  \"keep-a\", \"keep-b\",\n])\n"),
        ]);
        let ids = collect_source_ids(&root).unwrap();
        let got: Vec<&str> = ids.iter().map(|s| s.id.as_str()).collect();
        assert!(got.contains(&"mounted-classes:dialog"), "{:?}", got);
        assert!(got.contains(&"mounted-check:dialog"), "{:?}", got);
        assert!(got.contains(&"ignore-attrs:dialog:dialog:style"), "{:?}", got);
        assert!(
            got.contains(&"ignore-attrs:dialog:dialog-content:text"),
            "{:?}",
            got
        );
        assert!(got.contains(&"dead-utility:stale-dead"), "{:?}", got);
        assert!(got.contains(&"skin-allowlist:keep-a"), "{:?}", got);
        assert!(got.contains(&"skin-allowlist:keep-b"), "{:?}", got);
    }

    /// Go TestUnitLedgerRejectsBadEntries (the validation half of
    /// gateLedger, exercised on a synthetic ledger).
    #[test]
    fn unit_ledger_rejects_bad_entries() {
        let root = tree(&[(
            "gates/ledger.json",
            r#"{
  "pin": "p",
  "entries": {
    "bad-class": { "class": "who-knows", "reason": "a real reason here", "source": "golden", "recorded_at_pin": "p" },
    "trivial": { "class": "debt", "reason": "todo", "source": "golden", "recorded_at_pin": "p" },
    "no-pin": { "class": "debt", "reason": "a real reason here", "source": "golden", "recorded_at_pin": "" }
  },
  "budgets": {},
  "notes": []
}
"#,
        )]);
        let l = read_ledger(&root).unwrap();
        let mut problems: Vec<String> = Vec::new();
        for id in &l.entry_order {
            let e = &l.entries[id];
            if !ledger_classes().contains(&e.class.as_str()) {
                problems.push(format!("{}: unknown class {:?}", id, e.class));
            }
            if e.reason.len() < 8 {
                problems.push(format!("{}: missing or trivial reason", id));
            }
            if e.recorded.is_empty() {
                problems.push(format!("{}: no recorded_at_pin", id));
            }
        }
        assert_eq!(problems.len(), 3, "{:?}", problems);
    }

    /// Go TestUnitJSSetLiteral + FailsLoudly + SkipsBracketsInStrings +
    /// TestUnitJSFieldIsFalse + TestUnitJSObjectFieldAndAttrMap +
    /// TestUnitJSUnescape.
    #[test]
    fn unit_jssource_extractors() {
        let set = "const X = new Set([\n  \"a\\\"q\", // comment \"ghost\"\n  \"b\",\n])";
        assert_eq!(js_set_literal(set, "X").unwrap(), vec!["a\"q", "b"]);
        assert!(js_set_literal("const Y = 3", "X").is_err());

        let brackets = "const S = new Set([\"a[0]\"])";
        assert_eq!(js_set_literal(brackets, "S").unwrap(), vec!["a[0]"]);

        assert!(js_field_is_false("\n  mountedCheck: false,\n", "mountedCheck"));
        assert!(!js_field_is_false("// mountedCheck: false\n", "mountedCheck"));
        assert!(!js_field_is_false("xmountedCheck: false", "mountedCheck"));
        assert!(!js_field_is_false("mountedCheck: falsey", "mountedCheck"));

        // Go TestUnitJSObjectFieldAndAttrMap's fixture: quoted keys, an
        // inline comment, and a trailing field that must not leak in
        let src = "export default {\n  ignoreAttrs: {\n    \"accordion\": [\"text\"],\n    // text: recorded structural difference\n    \"accordion-item\": [\"text\", \"data-state\"],\n  },\n  scenarios: [\"click:x\"],\n}";
        let f = js_object_field(src, "ignoreAttrs").unwrap().unwrap();
        let m = js_attr_map(f);
        assert_eq!(m.len(), 2, "{:?}", m);
        assert_eq!(m[0].key, "accordion");
        assert_eq!(m[0].values, vec!["text"]);
        assert_eq!(m[1].key, "accordion-item");
        assert_eq!(m[1].values, vec!["text", "data-state"]);
        for e in &m {
            assert_ne!(e.key, "scenarios", "the region ran past ignoreAttrs");
        }
        assert!(js_object_field(src, "noSuchField").unwrap().is_none());
        // an empty array body parses as an entry with no values
        let obj = "x: {\"a\": [\"1\"], \"b\": [] ,}";
        let f2 = js_object_field(obj, "x").unwrap().unwrap();
        let m2 = js_attr_map(f2);
        assert_eq!(m2.len(), 2);
        assert_eq!(m2[1].values, Vec::<String>::new());

        assert_eq!(js_unescape("plain"), "plain");
        assert_eq!(js_unescape("a\\nb\\tc\\rd"), "a\nb\tc\rd");
        assert_eq!(js_unescape("em — dash \\u2014 here"), "em — dash — here");
        assert_eq!(js_unescape("q\\\"x"), "q\"x");
        assert_eq!(js_unescape("back\\\\slash"), "back\\slash");
    }

    #[test]
    fn unit_collect_budget_values_reads_golden_and_sweep() {
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => PathBuf::from(r),
            Err(_) => {
                let m = crate::crate_adjacent_tree_root()
                    .unwrap_or(Path::new(env!("CARGO_MANIFEST_DIR")).join(".."));
                m
            }
        };
        if !root.join(GOLDEN_EX_PATH).exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: no shadless tree)");
            }
            eprintln!("skip: no shadless tree");
            return;
        }
        let v = collect_budget_values(&root).unwrap();
        let exempt = v.get("golden.exempt-demos").copied().unwrap();
        assert!(exempt > 0, "golden.exempt-demos must count the committed exemptions");
        assert_eq!(
            v.get("interactivity.dead-families").copied().unwrap(),
            1,
            "sweepKnownDead currently carries exactly message-scroller"
        );
    }

    /// Go TestLedger: gate(t, gateLedger) on the real tree.
    #[test]
    fn ledger_on_real_tree() {
        let _serial = RENDER_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => PathBuf::from(r),
            Err(_) => {
                let m = crate::crate_adjacent_tree_root()
                    .unwrap_or(Path::new(env!("CARGO_MANIFEST_DIR")).join(".."));
                m
            }
        };
        if !root.join(GOLDEN_EX_PATH).exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: no shadless tree)");
            }
            eprintln!("skip: no shadless tree");
            return;
        }
        gate_ledger(&root).expect("ledger gate must pass on the real tree");
    }

    /// EXEMPTIONS.md is the human render of gates/ledger.json, and its header
    /// claims the two "cannot drift" — a claim nothing enforced: a ledger
    /// edit without `make ledger-render` used to stay green forever. The
    /// gate must fail while the render is stale, and pass again once the
    /// original bytes are back.
    #[test]
    fn gate_ledger_fails_when_render_drifts() {
        // both real-tree ledger tests touch the one committed EXEMPTIONS.md
        let _serial = RENDER_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let root = match std::env::var("SHADLESS_ROOT") {
            Ok(r) => PathBuf::from(r),
            Err(_) => {
                let m = crate::crate_adjacent_tree_root()
                    .unwrap_or(Path::new(env!("CARGO_MANIFEST_DIR")).join(".."));
                m
            }
        };
        if !root.join(GOLDEN_EX_PATH).exists() {
            if std::env::var_os("CI").is_some() {
                panic!("CI: required tree input missing (skip: no shadless tree)");
            }
            eprintln!("skip: no shadless tree");
            return;
        }
        let rendered_path = root.join(RENDERED_PATH);
        let original = std::fs::read_to_string(&rendered_path).expect("EXEMPTIONS.md exists");
        std::fs::write(
            &rendered_path,
            format!("{}\n(unrelated hand edit that the ledger never rendered)\n", original),
        )
        .unwrap();
        let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            gate_ledger(&root)
        }));
        std::fs::write(&rendered_path, original).expect("restore EXEMPTIONS.md");
        let res = match out {
            Ok(r) => r,
            Err(_) => panic!("gate_ledger panicked under a drifted render"),
        };
        assert!(
            res.is_err(),
            "gate_ledger stayed green while EXEMPTIONS.md had drifted from gates/ledger.json"
        );
    }
}
