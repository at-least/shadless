//! Port of pipeline/parity_baseline.go — see the Go source for the contract.
//!
//! The recorded-difference baseline shared by the three parity gates
//! (style-parity, demo-parity, path-parity). Values are pinned too: a
//! recorded cell that MOVES fails and has to be re-recorded — the same
//! ratchet the ids already had.

use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct parity_cell {
    pub id: String,
    pub oracle: String,
    pub shadless: String,
}

fn re_parity_calc() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"calc\([^)]*\)").unwrap())
}
fn re_parity_num() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"-?\d*\.?\d+(?:e[-+]?\d+)?").unwrap())
}
fn re_parity_oklab() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"oklab\((-?[\d.]+) 0 0\)").unwrap())
}

/// parseFloat2dp mirrors pipeline/demo_parity.go:189-206: JS
/// Math.round(x*100)/100 || 0. The int64 truncation is Go's float64→int64
/// conversion; on amd64 that compiles to CVTTSD2SI, which yields the
/// "integer indefinite" value i64::MIN for out-of-range and NaN inputs —
/// Rust's `as i64` saturates instead, so the Go behavior is spelled out.
fn parse_float_2dp(s: &str) -> f64 {
    let f: f64 = s.parse().unwrap();
    let r = go_f64_to_i64(f * 100.0 + 0.5 * sign(f)) as f64 / 100.0;
    if r == 0.0 {
        return 0.0;
    }
    r
}

/// Go amd64's float64→int64 conversion (CVTTSD2SI): values outside
/// [-2^63, 2^63) and NaN become i64::MIN.
fn go_f64_to_i64(x: f64) -> i64 {
    if x.is_nan() || x >= 9.223372036854775808e18 || x < -9.223372036854775808e18 {
        i64::MIN
    } else {
        x as i64
    }
}

fn sign(f: f64) -> f64 {
    if f < 0.0 {
        -1.0
    } else {
        1.0
    }
}

/// jsNumberString mirrors JS String(number): decimal notation with the
/// shortest round-trip, exponent only at >=1e21 (never for our values —
/// %g would print 3.35544e+07 where the baseline records 33554400).
///
/// Go's strconv.FormatFloat(f, 'f', -1, 64) prints the shortest round-trip
/// digits (Ryū) in FIXED notation — never exponent, no trailing ".0".
/// Rust's `{}` Display uses Grisu3, which picks different digits for some
/// large values (e.g. -709637921969813.25 prints as -709637921969813.3),
/// so the digits come from serde_json's float serializer (the same Ryū
/// algorithm Go's strconv uses) and are re-rendered in Go's fixed notation.
fn js_number_string(f: f64) -> String {
    let s = serde_json::to_string(&f).unwrap(); // e.g. "-709637921969813.2", "1e+16", "2.0"
    let (neg, body) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.as_str()),
    };
    // serde_json switches to scientific at exp >= 16 or exp <= -5; Go's 'f'
    // verb never does, so expand the exponent back out.
    let (mant, exp) = match body.find('e') {
        Some(i) => (&body[..i], body[i + 1..].parse::<i32>().unwrap()),
        None => (body, 0),
    };
    let (int_part, frac_part) = match mant.find('.') {
        Some(i) => (&mant[..i], &mant[i + 1..]),
        None => (mant, ""),
    };
    // serde_json appends ".0" to integral values; Go drops it.
    let frac_part = if frac_part == "0" { "" } else { frac_part };
    let digits = format!("{}{}", int_part, frac_part);
    let decimal_pos = int_part.len() as i32 + exp;
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    if decimal_pos <= 0 {
        out.push_str("0.");
        for _ in 0..(-decimal_pos) {
            out.push('0');
        }
        out.push_str(&digits);
    } else if (decimal_pos as usize) >= digits.len() {
        out.push_str(&digits);
        for _ in 0..(decimal_pos as usize - digits.len()) {
            out.push('0');
        }
    } else {
        let dp = decimal_pos as usize;
        out.push_str(&digits[..dp]);
        out.push('.');
        out.push_str(&digits[dp..]);
    }
    out
}

/// parityNormValue is the shared getComputedStyle() normalizer behind all
/// three parity gates (style-parity, demo-parity, path-parity): round every
/// embedded number to 2dp, using reParityNum/reParityOklab declared in
/// demo_parity.go, and canonicalise axis-only oklab() to oklch() (Chrome
/// serialises the same colour both ways). style-parity additionally
/// canonicalizes calc(...) via canonicalizeCalc; demoParityNorm and ppNorm
/// call this with it off, which demo-parity and path-parity do not do
/// themselves.
pub fn parity_norm_value(v: &str, canonicalize_calc: bool) -> String {
    if v.is_empty() {
        return v.to_string();
    }
    let v = re_parity_num().replace_all(v, |caps: &regex::Captures| {
        let r = parse_float_2dp(&caps[0]);
        if r == 0.0 {
            return "0".to_string(); // Object.is(-0) guard
        }
        js_number_string(r)
    });
    let v = re_parity_oklab().replace_all(&v, "oklch($1 0 0)");
    let mut v = v.into_owned();
    if canonicalize_calc {
        v = re_parity_calc().replace_all(&v, "calc(…)").into_owned();
    }
    v
}

/// cellMap: duplicate ids would silently drop a difference — rejected.
pub fn cell_map(cells: &[parity_cell]) -> (HashMap<String, parity_cell>, Vec<String>) {
    let mut m: HashMap<String, parity_cell> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for c in cells {
        if m.contains_key(&c.id) {
            panic!(
                "duplicate parity cell id: {} — the id is not unique enough to ratchet on",
                c.id
            );
        }
        m.insert(c.id.clone(), c.clone());
        order.push(c.id.clone());
    }
    (m, order)
}

/// The raw baseline body: pin, note, flaky. Cells are not needed in raw form
/// — load_parity_baseline returns them as the id→cell map.
#[derive(Clone, Debug, Default)]
pub struct RawBaseline {
    pub pin: String,
    pub note: String,
    pub flaky: Vec<String>,
}

/// loadParityBaseline errors on the pre-value format (bare cell ids).
/// Absent file → Ok(None), the caller records.
pub fn load_parity_baseline(
    root: &Path,
    path: &str,
) -> Result<Option<(RawBaseline, HashMap<String, parity_cell>)>, String> {
    let b = match std::fs::read(root.join(path)) {
        Ok(b) => b,
        Err(_) => return Ok(None), // absent → None, caller records
    };
    #[derive(serde::Deserialize)]
    struct Raw {
        #[serde(default)]
        pin: String,
        #[serde(default)]
        note: String,
        #[serde(default)]
        flaky: Vec<String>,
        #[serde(default)]
        cells: Vec<RawCell>,
    }
    #[derive(serde::Deserialize)]
    struct RawCell {
        id: String,
        #[serde(default)]
        oracle: String,
        #[serde(default)]
        shadless: String,
    }
    let raw: Raw = serde_json::from_slice(&b).map_err(|e| e.to_string())?;
    let mut cells: HashMap<String, parity_cell> = HashMap::new();
    for c in &raw.cells {
        cells.insert(
            c.id.clone(),
            parity_cell {
                id: c.id.clone(),
                oracle: c.oracle.clone(),
                shadless: c.shadless.clone(),
            },
        );
    }
    if let Err(e) = detect_prevalue(&b) {
        return Err(e);
    }
    Ok(Some((
        RawBaseline {
            pin: raw.pin,
            note: raw.note,
            flaky: raw.flaky,
        },
        cells,
    )))
}

fn detect_prevalue(b: &[u8]) -> Result<(), String> {
    #[derive(serde::Deserialize)]
    struct Probe {
        #[serde(default)]
        cells: Vec<serde_json::Value>,
    }
    let probe: Probe = match serde_json::from_slice(b) {
        Ok(p) => p,
        Err(_) => return Ok(()),
    };
    for c in &probe.cells {
        if c.is_string() {
            return Err("pre-value format (bare cell ids). Re-record".to_string());
        }
    }
    Ok(())
}

/// writeParityBaseline emits JSON.stringify(body, null, 1) + "\n".
pub fn write_parity_baseline(
    root: &Path,
    path: &str,
    note: &str,
    flaky: &[String],
    cells: &HashMap<String, parity_cell>,
) -> Result<(), String> {
    let pin_b = std::fs::read(root.join("src/registry/pin.json")).unwrap_or_default();
    #[derive(serde::Deserialize)]
    struct Pin {
        #[serde(rename = "shadcn_ui")]
        shadcn_ui: PinShadcnUi,
    }
    #[derive(serde::Deserialize)]
    struct PinShadcnUi {
        #[serde(default)]
        tag: String,
    }
    let pin: Pin = serde_json::from_slice(&pin_b).unwrap_or(Pin {
        shadcn_ui: PinShadcnUi {
            tag: String::new(),
        },
    });

    let mut ids: Vec<&String> = cells.keys().collect();
    ids.sort();

    let mut b = String::new();
    b.push_str("{\n \"pin\": ");
    b.push_str(&crate::jsonorder::json_string(&pin.shadcn_ui.tag));
    b.push_str(",\n \"note\": ");
    b.push_str(&crate::jsonorder::json_string(note));
    if !flaky.is_empty() {
        let mut sorted: Vec<&String> = flaky.iter().collect();
        sorted.sort();
        let parts: Vec<String> = sorted
            .iter()
            .map(|f| crate::jsonorder::json_string(f))
            .collect();
        b.push_str(",\n \"flaky\": [\n  ");
        b.push_str(&parts.join(",\n  "));
        b.push_str("\n ]");
    }
    if !ids.is_empty() {
        b.push_str(",\n \"cells\": [");
        for (i, id) in ids.iter().enumerate() {
            if i > 0 {
                b.push(',');
            }
            let c = &cells[*id];
            b.push_str("\n  {\n   \"id\": ");
            b.push_str(&crate::jsonorder::json_string(id));
            b.push_str(",\n   \"oracle\": ");
            b.push_str(&crate::jsonorder::json_string(&c.oracle));
            b.push_str(",\n   \"shadless\": ");
            b.push_str(&crate::jsonorder::json_string(&c.shadless));
            b.push_str("\n  }");
        }
        b.push_str("\n ]");
    } else {
        b.push_str(",\n \"cells\": []");
    }
    b.push_str("\n}\n");
    std::fs::write(root.join(path), b.as_bytes()).map_err(|e| e.to_string())
}

#[derive(Clone, Debug)]
pub struct parity_change {
    pub id: String,
    pub was: parity_cell,
    pub now: parity_cell,
}

#[derive(Default)]
pub struct parity_diff {
    pub appeared: Vec<String>,
    pub fixed: Vec<String>,
    pub changed: Vec<parity_change>,
}

/// diffParityBaseline: appeared / fixed / changed are all failures.
pub fn diff_parity_baseline(
    recorded: &HashMap<String, parity_cell>,
    actual: &HashMap<String, parity_cell>,
    actual_order: &[String],
) -> parity_diff {
    let mut d = parity_diff::default();
    for id in actual_order {
        let v = &actual[id];
        match recorded.get(id) {
            None => d.appeared.push(id.clone()),
            Some(was) => {
                if was.oracle != v.oracle || was.shadless != v.shadless {
                    d.changed.push(parity_change {
                        id: id.clone(),
                        was: was.clone(),
                        now: v.clone(),
                    });
                }
            }
        }
    }
    for id in recorded.keys() {
        if !actual.contains_key(id) {
            d.fixed.push(id.clone());
        }
    }
    d.appeared.sort();
    d.fixed.sort();
    d.changed.sort_by(|a, b| a.id.cmp(&b.id));
    d
}

fn trunc60(s: &str) -> String {
    if s.chars().count() > 60 {
        let cut: String = s.chars().take(60).collect();
        return format!("{}…", cut);
    }
    s.to_string()
}

pub fn show_cell(v: &parity_cell) -> String {
    format!("oracle={} shadless={}", trunc60(&v.oracle), trunc60(&v.shadless))
}

pub fn show_change(c: &parity_change) -> String {
    format!(
        "{}\n      recorded: {}\n      now:      {}",
        c.id,
        show_cell(&c.was),
        show_cell(&c.now)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mirrors pipeline/parity_baseline_test.go TestUnitCellMapRejectsDuplicateID.
    #[test]
    fn unit_cell_map_rejects_duplicate_id() {
        let result = std::panic::catch_unwind(|| {
            cell_map(&[
                parity_cell {
                    id: "x".to_string(),
                    oracle: "1".to_string(),
                    shadless: String::new(),
                },
                parity_cell {
                    id: "x".to_string(),
                    oracle: "2".to_string(),
                    shadless: String::new(),
                },
            ]);
        });
        assert!(result.is_err(), "a duplicate cell id did not panic");
    }

    /// Mirrors pipeline/parity_baseline_test.go TestUnitCellMapPreservesInsertionOrder.
    #[test]
    fn unit_cell_map_preserves_insertion_order() {
        let (m, order) = cell_map(&[
            parity_cell {
                id: "b/comp".to_string(),
                oracle: "1".to_string(),
                shadless: "2".to_string(),
            },
            parity_cell {
                id: "a/comp".to_string(),
                oracle: "3".to_string(),
                shadless: "4".to_string(),
            },
        ]);
        assert_eq!(m.len(), 2, "map not built correctly: {:?}", m);
        assert_eq!(m["a/comp"].shadless, "4", "map not built correctly: {:?}", m);
        assert_eq!(
            order.join(","),
            "b/comp,a/comp",
            "order = {:?}, want insertion order [b/comp a/comp]",
            order
        );
    }

    /// Mirrors pipeline/parity_baseline_test.go TestUnitDetectPrevalue.
    #[test]
    fn unit_detect_prevalue() {
        assert!(
            detect_prevalue(br#"{"cells":["bare-id"]}"#).is_err(),
            "bare-string cells (the pre-value format) should be rejected"
        );
        assert!(
            detect_prevalue(br#"{"cells":[{"id":"x","oracle":"1","shadless":"2"}]}"#).is_ok(),
            "value-shaped cells should be accepted directly"
        );
    }

    /// Mirrors pipeline/parity_baseline_test.go TestUnitLoadParityBaseline.
    #[test]
    fn unit_load_parity_baseline() {
        let dir = std::env::temp_dir().join(format!(
            "shadless-rs-parity-baseline-test-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();

        // absent file: None — the caller records
        let loaded_abs = load_parity_baseline(&dir, "missing.json").unwrap();
        assert!(
            loaded_abs.is_none(),
            "absent baseline: got {:?}, want None",
            loaded_abs
        );

        // value-shaped baseline decodes into the id->cell map
        let value_path = dir.join("value.json");
        std::fs::write(
            &value_path,
            r#"{"pin":"v1.2.3","cells":[{"id":"alert/root/color","oracle":"red","shadless":"blue"}]}"#,
        )
        .unwrap();
        let (raw, cells) = load_parity_baseline(&dir, "value.json").unwrap().unwrap();
        assert_eq!(raw.pin, "v1.2.3", "pin not decoded: {:?}", raw);
        let c = &cells["alert/root/color"];
        assert!(
            c.oracle == "red" && c.shadless == "blue",
            "cell not decoded: {:?}",
            cells
        );

        // pre-value (bare id) baseline errors instead of silently decoding empty
        let bare_val = dir.join("bare.json");
        std::fs::write(&bare_val, r#"{"cells":["alert/root/color"]}"#).unwrap();
        assert!(
            load_parity_baseline(&dir, "bare.json").is_err(),
            "pre-value baseline should error, not decode silently"
        );

        std::fs::remove_dir_all(&dir).ok();
    }

    /// Mirrors pipeline/parity_baseline_test.go TestUnitDiffParityBaseline.
    #[test]
    fn unit_diff_parity_baseline() {
        let mut recorded: HashMap<String, parity_cell> = HashMap::new();
        recorded.insert(
            "m".to_string(),
            parity_cell {
                id: "m".to_string(),
                oracle: "1".to_string(),
                shadless: "1".to_string(),
            },
        );
        recorded.insert(
            "z".to_string(),
            parity_cell {
                id: "z".to_string(),
                oracle: "9".to_string(),
                shadless: "9".to_string(),
            },
        );
        recorded.insert(
            "y".to_string(),
            parity_cell {
                id: "y".to_string(),
                oracle: "8".to_string(),
                shadless: "8".to_string(),
            },
        );
        // actualOrder is deliberately NOT alphabetical, to prove appeared/fixed
        // come back SORTED rather than dependent on iteration/insertion order.
        let mut actual: HashMap<String, parity_cell> = HashMap::new();
        actual.insert(
            "m".to_string(),
            parity_cell {
                id: "m".to_string(),
                oracle: "1".to_string(),
                shadless: "1".to_string(),
            },
        );
        actual.insert(
            "z".to_string(),
            parity_cell {
                id: "z".to_string(),
                oracle: "9".to_string(),
                shadless: "CHANGED".to_string(),
            },
        );
        actual.insert(
            "w".to_string(),
            parity_cell {
                id: "w".to_string(),
                oracle: "new".to_string(),
                shadless: "new".to_string(),
            },
        );
        actual.insert(
            "a".to_string(),
            parity_cell {
                id: "a".to_string(),
                oracle: "new2".to_string(),
                shadless: "new2".to_string(),
            },
        );
        let actual_order = vec![
            "z".to_string(),
            "w".to_string(),
            "m".to_string(),
            "a".to_string(),
        ]; // not alphabetical

        let d = diff_parity_baseline(&recorded, &actual, &actual_order);

        assert_eq!(
            d.appeared.join(","),
            "a,w",
            "appeared = {:?}, want sorted [a w]",
            d.appeared
        );
        assert_eq!(
            d.fixed.join(","),
            "y",
            "fixed = {:?}, want [y] (only the recorded id absent from actual)",
            d.fixed
        );
        assert_eq!(d.changed.len(), 1, "changed = {:?}, want one entry for z", d.changed);
        assert_eq!(d.changed[0].id, "z", "changed = {:?}, want one entry for z", d.changed);
        assert_eq!(
            d.changed[0].now.shadless, "CHANGED",
            "changed = {:?}, want one entry for z",
            d.changed
        );
    }
}
