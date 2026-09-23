//! Port of pipeline/internal/twmerge — tailwind-merge v3.6.0's default-config
//! merge. Line-faithful to the bundle: mergeClassList walks right-to-left,
//! parseClassName splits on ':' outside brackets, groups are looked up in a
//! trie built from the embedded config.json (dumped from node
//! getDefaultConfig() with functions replaced by $fn/$theme tokens), and
//! conformance to twMerge's own output is pinned by snapshot.json (555 real
//! class strings, dumped from the JS implementation).
//!
//! Regex translation notes (Go RE2 → Rust regex, both must agree with the JS):
//! Go's `\w`, `\d`, `\s`, `\b` are ASCII; Rust's are Unicode — each is
//! rewritten to its explicit ASCII form. Capture-group numbers are preserved.

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

const CONFIG_JSON: &str = include_str!("config.json");
#[cfg(test)]
const SNAPSHOT_JSON: &str = include_str!("snapshot.json");

// ------------------------------------------------------------------ regexes

struct Regexes {
    arbitrary_value: Regex,
    arbitrary_var: Regex,
    fraction: Regex,
    tshirt: Regex,
    length_unit: Regex,
    color_func: Regex,
    shadow: Regex,
    image: Regex,
    number_like: Regex,
    whitespace: Regex,
}

fn regexes() -> &'static Regexes {
    static R: OnceLock<Regexes> = OnceLock::new();
    R.get_or_init(|| Regexes {
        arbitrary_value: Regex::new(r#"^\[(?:([0-9A-Za-z_][0-9A-Za-z_-]*):)?(.+)\]$"#).unwrap(),
        arbitrary_var: Regex::new(r"^\((?:([0-9A-Za-z_][0-9A-Za-z_-]*):)?(.+)\)$").unwrap(),
        fraction: Regex::new(r"^[0-9]+(?:\.[0-9]+)?/[0-9]+(?:\.[0-9]+)?$").unwrap(),
        tshirt: Regex::new(r"^([0-9]+(\.[0-9]+)?)?(xs|sm|md|lg|xl)$").unwrap(),
        // The JS regex is UNANCHORED (a substring like "8rem" of "0.8rem"
        // satisfies it) except for its ^0$ branch. Copied verbatim so the
        // validator's true/false boundary matches exactly.
        length_unit: Regex::new(
            r"[0-9]+(%|px|r?em|[sdl]?v([hwib]|min|max)|pt|pc|in|cm|mm|cap|ch|ex|r?lh|cq(w|h|i|b|min|max))|(?-u:\b)(calc|min|max|clamp)\(.+\)|^0$",
        )
        .unwrap(),
        color_func: Regex::new(r"^(rgba?|hsla?|hwb|(ok)?(lab|lch)|color-mix)\(.+\)$").unwrap(),
        shadow: Regex::new(
            r"^(inset_)?-?(([0-9]+)?\.?([0-9]+)[a-z]+|0)_-?(([0-9]+)?\.?([0-9]+)[a-z]+|0)",
        )
        .unwrap(),
        image: Regex::new(
            r"^(url|image|image-set|cross-fade|element|(repeating-)?(linear|radial|conic)-gradient)\(.+\)$",
        )
        .unwrap(),
        number_like: Regex::new(r"^[+-]?([0-9]+\.?[0-9]*|\.[0-9]+)([eE][+-]?[0-9]+)?$").unwrap(),
        whitespace: Regex::new(r"[\t\n\f\r ]+").unwrap(),
    })
}

// --------------------------------------------------------------- validators

fn is_fraction(v: &str) -> bool {
    regexes().fraction.is_match(v)
}

/// Mirrors !!value && !Number.isNaN(Number(value)) — Number() also accepts
/// "Infinity"/hex, but class bodies never carry those; the decimal form is
/// the faithful subset and the snapshot pins the difference.
fn is_number(v: &str) -> bool {
    !v.is_empty() && regexes().number_like.is_match(v)
}

fn is_integer(v: &str) -> bool {
    if v.is_empty() {
        return false;
    }
    // JS: Number.isInteger — integer means no fraction/exponent after parse
    if !regexes().number_like.is_match(v) || v.contains(['.', 'e', 'E']) {
        return false;
    }
    v.parse::<i64>().is_ok() // Go strconv.Atoi: overflow fails the same way
}

fn is_percent(v: &str) -> bool {
    v.ends_with('%') && is_number(&v[..v.len() - 1])
}

fn is_tshirt_size(v: &str) -> bool {
    regexes().tshirt.is_match(v)
}
fn is_any(_: &str) -> bool {
    true
}
fn is_never(_: &str) -> bool {
    false
}

fn is_length_only(v: &str) -> bool {
    regexes().length_unit.is_match(v) && !regexes().color_func.is_match(v)
}
fn is_shadow(v: &str) -> bool {
    regexes().shadow.is_match(v)
}
fn is_image(v: &str) -> bool {
    regexes().image.is_match(v)
}

fn is_any_non_arbitrary(v: &str) -> bool {
    !is_arbitrary_value(v) && !is_arbitrary_variable(v)
}
fn is_arbitrary_value(v: &str) -> bool {
    regexes().arbitrary_value.is_match(v)
}
fn is_arbitrary_variable(v: &str) -> bool {
    regexes().arbitrary_var.is_match(v)
}

fn is_arbitrary_length(v: &str) -> bool {
    get_is_arbitrary_value(v, is_label_length, is_length_only)
}
fn is_arbitrary_number(v: &str) -> bool {
    get_is_arbitrary_value(v, is_label_number, is_number)
}
fn is_arbitrary_weight(v: &str) -> bool {
    get_is_arbitrary_value(v, is_label_weight, is_any)
}
fn is_arbitrary_size(v: &str) -> bool {
    get_is_arbitrary_value(v, is_label_size, is_never)
}
fn is_arbitrary_family_name(v: &str) -> bool {
    get_is_arbitrary_value(v, is_label_family_name, is_never)
}
fn is_arbitrary_position(v: &str) -> bool {
    get_is_arbitrary_value(v, is_label_position, is_never)
}
fn is_arbitrary_image(v: &str) -> bool {
    get_is_arbitrary_value(v, is_label_image, is_image)
}
fn is_arbitrary_shadow(v: &str) -> bool {
    get_is_arbitrary_value(v, is_label_shadow, is_shadow)
}

fn is_arbitrary_variable_length(v: &str) -> bool {
    get_is_arbitrary_variable(v, is_label_length, false)
}
fn is_arbitrary_variable_family_name(v: &str) -> bool {
    get_is_arbitrary_variable(v, is_label_family_name, false)
}
fn is_arbitrary_variable_position(v: &str) -> bool {
    get_is_arbitrary_variable(v, is_label_position, false)
}
fn is_arbitrary_variable_size(v: &str) -> bool {
    get_is_arbitrary_variable(v, is_label_size, false)
}
fn is_arbitrary_variable_image(v: &str) -> bool {
    get_is_arbitrary_variable(v, is_label_image, false)
}
fn is_arbitrary_variable_shadow(v: &str) -> bool {
    get_is_arbitrary_variable(v, is_label_shadow, true)
}
fn is_arbitrary_variable_weight(v: &str) -> bool {
    get_is_arbitrary_variable(v, is_label_weight, true)
}

fn get_is_arbitrary_value(
    value: &str,
    test_label: fn(&str) -> bool,
    test_value: fn(&str) -> bool,
) -> bool {
    let Some(m) = regexes().arbitrary_value.captures(value) else {
        return false;
    };
    let label = m.get(1).map(|m| m.as_str()).unwrap_or("");
    if !label.is_empty() {
        return test_label(label);
    }
    test_value(m.get(2).map(|m| m.as_str()).unwrap_or(""))
}

/// The third parameter is shouldMatchNoLabel: a label-less (…) variable counts
/// ONLY for shadow and weight (the bundle passes true there; everything else
/// passes undefined → false).
fn get_is_arbitrary_variable(
    value: &str,
    test_label: fn(&str) -> bool,
    should_match_no_label: bool,
) -> bool {
    let Some(m) = regexes().arbitrary_var.captures(value) else {
        return false;
    };
    let label = m.get(1).map(|m| m.as_str()).unwrap_or("");
    if !label.is_empty() {
        return test_label(label);
    }
    should_match_no_label
}

fn is_label_position(label: &str) -> bool {
    label == "position" || label == "percentage"
}
fn is_label_image(label: &str) -> bool {
    label == "image" || label == "url"
}
fn is_label_size(label: &str) -> bool {
    label == "length" || label == "size" || label == "bg-size"
}
fn is_label_length(label: &str) -> bool {
    label == "length"
}
fn is_label_number(label: &str) -> bool {
    label == "number"
}
fn is_label_family_name(label: &str) -> bool {
    label == "family-name"
}
fn is_label_weight(label: &str) -> bool {
    label == "number" || label == "weight"
}
fn is_label_shadow(label: &str) -> bool {
    label == "shadow"
}

/// Mirrors the bundle's offset checks verbatim. '@container' (bare) does NOT
/// match (offset 10 must carry '/' or a unit prefix), consistent with the JS
/// `value[10] === '/'` short-circuit. Byte indices are safe: the first ten
/// bytes are the ASCII literal "@container".
fn is_named_container_query(v: &str) -> bool {
    let b = v.as_bytes();
    if !v.starts_with("@container") {
        return false;
    }
    if b.len() > 11 && b[10] == b'/' && b[11] != 0 {
        return true;
    }
    if b.len() > 16 && b[11] == b's' && b[16] != 0 && b[10..].starts_with(b"-size/") {
        return true;
    }
    if b.len() > 18 && b[11] == b'n' && b[18] != 0 && b[10..].starts_with(b"-normal/") {
        return true;
    }
    false
}

/// validatorsByName is keyed by the bundle's identifiers; config.json carries
/// those names in its $fn slots.
fn validator_by_name(name: &str) -> fn(&str) -> bool {
    match name {
        "isAny" => is_any,
        "isAnyNonArbitrary" => is_any_non_arbitrary,
        "isArbitraryFamilyName" => is_arbitrary_family_name,
        "isArbitraryImage" => is_arbitrary_image,
        "isArbitraryLength" => is_arbitrary_length,
        "isArbitraryNumber" => is_arbitrary_number,
        "isArbitraryPosition" => is_arbitrary_position,
        "isArbitraryShadow" => is_arbitrary_shadow,
        "isArbitrarySize" => is_arbitrary_size,
        "isArbitraryValue" => is_arbitrary_value,
        "isArbitraryVariable" => is_arbitrary_variable,
        "isArbitraryVariableFamilyName" => is_arbitrary_variable_family_name,
        "isArbitraryVariableImage" => is_arbitrary_variable_image,
        "isArbitraryVariableLength" => is_arbitrary_variable_length,
        "isArbitraryVariablePosition" => is_arbitrary_variable_position,
        "isArbitraryVariableShadow" => is_arbitrary_variable_shadow,
        "isArbitraryVariableSize" => is_arbitrary_variable_size,
        "isArbitraryVariableWeight" => is_arbitrary_variable_weight,
        "isArbitraryWeight" => is_arbitrary_weight,
        "isFraction" => is_fraction,
        "isInteger" => is_integer,
        "isNumber" => is_number,
        "isPercent" => is_percent,
        "isTshirtSize" => is_tshirt_size,
        "isNamedContainerQuery" => is_named_container_query,
        "isNever" => is_never,
        "isLengthOnly" => is_length_only,
        "isShadow" => is_shadow,
        "isImage" => is_image,
        _ => panic!("twmerge: unknown validator {}", name),
    }
}

// --------------------------------------------------- class map (the trie)

type Validator = fn(&str) -> bool;

struct ClassPart {
    next: HashMap<String, ClassPart>,
    group_id: String,
    validators: Vec<(String, Validator)>,
}

impl ClassPart {
    fn new() -> Self {
        ClassPart {
            next: HashMap::new(),
            group_id: String::new(),
            validators: Vec::new(),
        }
    }

    fn part(&mut self, path: &str) -> &mut ClassPart {
        let mut cur = self;
        for p in path.split('-') {
            cur = cur.next.entry(p.to_string()).or_insert_with(ClassPart::new);
        }
        cur
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawConfig {
    #[serde(default)]
    theme: HashMap<String, Vec<Value>>,
    #[serde(default)]
    class_groups: HashMap<String, Vec<Value>>,
    /// Preserves JS Object.keys order: groups that SHARE a trie prefix
    /// (text-*, border-*) add validators to the same node, and the first
    /// match wins. Without this list a catch-all validator (color's isAny)
    /// can be probed before the group-specific one and the wrong group
    /// claims the class.
    class_group_order: Vec<String>,
    #[serde(default)]
    conflicting_class_groups: HashMap<String, Vec<String>>,
    #[serde(default)]
    conflicting_class_group_modifiers: HashMap<String, Vec<String>>,
    #[serde(default)]
    postfix_lookup_class_groups: HashSet<String>,
    #[serde(default)]
    order_sensitive_modifiers: Vec<String>,
}

struct Built {
    class_map: ClassPart,
    modifier_weight: HashMap<String, usize>,
    conflicting_groups: HashMap<String, Vec<String>>,
    conflicting_group_modifiers: HashMap<String, Vec<String>>,
    postfix_lookup_groups: HashSet<String>,
}

/// processDefinition dispatches the JSON-encoded definition onto the trie:
/// "literal" → endpoint; {"$fn": name} → validator; {"$theme": key} → theme
/// list expanded in place; {"prefix": [defs]} → nest.
fn process_definition(
    def: &Value,
    parent: &mut ClassPart,
    group_id: &str,
    theme: &HashMap<String, Vec<Value>>,
) {
    match def {
        Value::String(d) => {
            let target = if d.is_empty() {
                parent
            } else {
                parent.part(d)
            };
            target.group_id = group_id.to_string();
        }
        Value::Object(map) => {
            if let Some(Value::String(name)) = map.get("$fn") {
                let f = validator_by_name(name);
                parent.validators.push((group_id.to_string(), f));
                return;
            }
            if let Some(Value::String(key)) = map.get("$theme") {
                if let Some(subs) = theme.get(key) {
                    for sub in subs {
                        process_definition(sub, parent, group_id, theme);
                    }
                }
                return;
            }
            for (prefix, sub) in map {
                if let Value::Array(list) = sub {
                    for s in list {
                        process_definition(s, parent.part(prefix), group_id, theme);
                    }
                }
            }
        }
        _ => panic!("twmerge: unhandled definition shape in config.json"),
    }
}

fn build() -> Built {
    let cfg: RawConfig = serde_json::from_str(CONFIG_JSON).expect("twmerge: config.json");
    let mut root = ClassPart::new();
    let mut modifier_weight = HashMap::new();
    for (i, m) in cfg.order_sensitive_modifiers.iter().enumerate() {
        modifier_weight.insert(m.clone(), 1_000_000 + i);
    }
    for id in &cfg.class_group_order {
        if let Some(defs) = cfg.class_groups.get(id) {
            for def in defs {
                process_definition(def, &mut root, id, &cfg.theme);
            }
        }
    }
    Built {
        class_map: root,
        modifier_weight,
        conflicting_groups: cfg.conflicting_class_groups,
        conflicting_group_modifiers: cfg.conflicting_class_group_modifiers,
        postfix_lookup_groups: cfg.postfix_lookup_class_groups,
    }
}

fn built() -> &'static Built {
    static B: OnceLock<Built> = OnceLock::new();
    B.get_or_init(build)
}

// ------------------------------------------------- group lookup

fn get_group_recursive(parts: &[&str], start: usize, node: &ClassPart) -> String {
    if parts.len() - start == 0 {
        return node.group_id.clone();
    }
    if let Some(next) = node.next.get(parts[start]) {
        let r = get_group_recursive(parts, start + 1, next);
        if !r.is_empty() {
            return r;
        }
    }
    let rest = parts[start..].join("-");
    for (gid, validator) in &node.validators {
        if validator(&rest) {
            return gid.clone();
        }
    }
    String::new()
}

/// An arbitrary-property class ([margin:0]) gets a synthetic "$prop" group;
/// real classes split on '-' and a leading empty part (negative values) is
/// skipped.
fn get_class_group_id(class_name: &str) -> String {
    let b = built();
    if class_name.starts_with('[') && class_name.ends_with(']') {
        let content = &class_name[1..class_name.len() - 1];
        return match content.find(':') {
            None => String::new(),
            Some(i) => {
                let prop = &content[..i];
                if !prop.is_empty() {
                    format!("${}", prop)
                } else {
                    String::new()
                }
            }
        };
    }
    let parts: Vec<&str> = class_name.split('-').collect();
    let start = if parts[0].is_empty() && parts.len() > 1 {
        1
    } else {
        0
    };
    get_group_recursive(&parts, start, &b.class_map)
}

// ------------------------------------------------- modifier handling

/// Predefined modifiers sorted alphabetically; order-sensitive and arbitrary
/// ones keep their relative position (segments are flushed in place).
fn sort_modifiers(modifiers: &[String]) -> Vec<String> {
    if modifiers.len() < 2 {
        return modifiers.to_vec();
    }
    let b = built();
    let mut out: Vec<String> = Vec::new();
    let mut segment: Vec<String> = Vec::new();
    for m in modifiers {
        let is_arb = m.starts_with('[');
        let sensitive = b.modifier_weight.contains_key(m);
        if is_arb || sensitive {
            segment.sort();
            out.append(&mut segment);
            out.push(m.clone());
        } else {
            segment.push(m.clone());
        }
    }
    segment.sort();
    out.append(&mut segment);
    out
}

struct ParsedClass {
    modifiers: Vec<String>,
    important: bool,
    base: String,
    postfix_position: isize, // -1 when none
}

/// Mirrors bundle-cjs.js:253-291: split at ':' when both bracketDepth and
/// parenDepth are zero; track the last top-level '/'. All slice boundaries
/// land on ASCII delimiters, so byte offsets are UTF-8-safe.
fn parse_class_name(name: &str) -> ParsedClass {
    let mut modifiers: Vec<String> = Vec::new();
    let (mut bracket_depth, mut paren_depth) = (0i32, 0i32);
    let mut modifier_start = 0usize;
    let mut postfix: isize = -1;
    let bytes = name.as_bytes();
    for i in 0..bytes.len() {
        let c = bytes[i];
        if bracket_depth == 0 && paren_depth == 0 {
            match c {
                b':' => {
                    modifiers.push(name[modifier_start..i].to_string());
                    modifier_start = i + 1;
                }
                b'/' => postfix = i as isize,
                _ => {}
            }
        }
        match c {
            b'[' => bracket_depth += 1,
            b']' => bracket_depth -= 1,
            b'(' => paren_depth += 1,
            b')' => paren_depth -= 1,
            _ => {}
        }
    }
    let mut base = name[modifier_start..].to_string();
    let mut important = false;
    if base.ends_with('!') {
        base.pop();
        important = true;
    } else if base.starts_with('!') {
        base = base[1..].to_string();
        important = true;
    }
    let pp = if postfix != -1 && postfix as usize > modifier_start {
        postfix - modifier_start as isize
    } else {
        -1
    };
    ParsedClass {
        modifiers,
        important,
        base,
        postfix_position: pp,
    }
}

/// Merge implements twMerge("…") over the embedded default config.
pub fn merge(class_list: &str) -> String {
    let b = built();
    let trimmed = class_list.trim();
    let classes: Vec<&str> = regexes().whitespace.split(trimmed).collect();
    let mut conflict_ids: Vec<String> = Vec::new();
    let mut result: Vec<String> = Vec::new();

    for original in classes.iter().rev() {
        let original = *original;
        let p = parse_class_name(original);
        let mut has_postfix = p.postfix_position != -1;
        let mut group_id;
        if has_postfix {
            group_id = get_class_group_id(&p.base[..p.postfix_position as usize]);
            if !group_id.is_empty() && b.postfix_lookup_groups.contains(&group_id) {
                let with_postfix = get_class_group_id(&p.base);
                if !with_postfix.is_empty() && with_postfix != group_id {
                    group_id = with_postfix;
                    has_postfix = false;
                }
            }
        } else {
            group_id = get_class_group_id(&p.base);
        }
        if group_id.is_empty() {
            if has_postfix {
                group_id = get_class_group_id(&p.base);
                if !group_id.is_empty() {
                    has_postfix = false;
                }
            }
            if group_id.is_empty() {
                result.push(original.to_string());
                continue;
            }
        }
        let mut modifier = String::new();
        if p.modifiers.len() == 1 {
            modifier = p.modifiers[0].clone();
        } else if p.modifiers.len() > 1 {
            modifier = sort_modifiers(&p.modifiers).join(":");
        }
        if p.important {
            modifier.push('!');
        }
        let class_id = format!("{}{}", modifier, group_id);
        if conflict_ids.contains(&class_id) {
            continue;
        }
        conflict_ids.push(class_id);
        let mut conflicting = b
            .conflicting_groups
            .get(&group_id)
            .cloned()
            .unwrap_or_default();
        if has_postfix {
            if let Some(mods) = b.conflicting_group_modifiers.get(&group_id) {
                if !mods.is_empty() {
                    conflicting.extend(mods.iter().cloned());
                }
            }
        }
        for g in &conflicting {
            conflict_ids.push(format!("{}{}", modifier, g));
        }
        result.push(original.to_string());
    }
    result.reverse();
    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hand-checked against `twMerge` (tailwind-merge v3.6.0) outputs — the
    /// same cases pipeline/internal/twmerge/twmerge_test.go pins.
    #[test]
    fn hand_checked_basics() {
        let cases = [
            ("absolute inset-0 z-10 outline-none absolute inset-0 z-10 outline-none", "absolute inset-0 z-10 outline-none"),
            ("text-sm text-lg", "text-lg"),
            ("w-full w-1/2", "w-1/2"),
            ("max-h-(--available-height) max-h-72", "max-h-72"),
            ("p-4 px-2", "p-4 px-2"),
            ("flex flex-col", "flex flex-col"),
            ("ml-auto ms-auto", "ml-auto ms-auto"),
            ("bg-red-500 hover:bg-red-600", "bg-red-500 hover:bg-red-600"),
            ("data-horizontal:w-full data-vertical:h-full data-horizontal:w-full", "data-vertical:h-full data-horizontal:w-full"),
            ("text-lg leading-snug", "text-lg leading-snug"),
            ("text-sm/7 text-lg", "text-lg"),
            ("p-1 text-sm leading-none text-md/5", "p-1 text-md/5"),
            ("foo text-lg baz text-sm", "foo baz text-sm"),
            ("h-[100px] h-20", "h-20"),
            ("inset-0 inset-2", "inset-2"),
            ("[margin:0] [margin:4px]", "[margin:4px]"),
            ("[mask:luminance] [mask:alpha]", "[mask:alpha]"),
            ("!p-4 p-2 !p-1", "p-2 !p-1"),
            // order-sensitive modifier weights: "before" keeps its relative
            // position instead of sorting alphabetically
            ("hover:before:p-2 before:hover:p-4", "hover:before:p-2 before:hover:p-4"),
        ];
        for (input, want) in cases {
            assert_eq!(merge(input), want, "Merge({:?})", input);
        }
    }

    /// Survivors keep INPUT order: px-4 dies to p-6's conflicting px, flex
    /// dies to grid (display group).
    #[test]
    fn merge_order_preserved() {
        assert_eq!(
            merge("flex a-token p-2 px-4 grid p-6 b-token"),
            "a-token grid p-6 b-token"
        );
    }

    #[test]
    fn parse_and_sort() {
        let p = parse_class_name("focus:hover:bg-red-500/50");
        assert_eq!(p.base, "bg-red-500/50");
        assert_eq!(p.modifiers.len(), 2);
        assert_ne!(p.postfix_position, -1);
        assert_eq!(
            merge("data-[active]:focus:p-2 focus:p-1 data-[active]:focus:p-4"),
            "focus:p-1 data-[active]:focus:p-4"
        );
    }

    /// Conformance: twMerge's actual output on the repo's real class strings,
    /// snapshotted by the JS implementation (snapshot.json). If this diverges
    /// the PORT is wrong, not the snapshot.
    #[test]
    fn snapshot_conformance() {
        let cases: Vec<(String, String)> = serde_json::from_str(SNAPSHOT_JSON).unwrap();
        assert!(cases.len() > 500, "snapshot corpus went missing");
        for (input, want) in &cases {
            let got = merge(input);
            assert_eq!(got, *want, "Merge({:?}) disagreeing case", input);
        }
    }
}
