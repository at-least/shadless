//! Port of pipeline/upstream_snapshot.go — see the Go source for the contract.
//!
//! Golden-master snapshot of ui.shadcn.com example DOM (hop 1 of the 1:1
//! gate). Crawls /docs/components/<base>/<page>, slices each preview's demo
//! DOM out of the SSR payload, normalizes radix auto ids, stores
//! src/registry/upstream-snapshot/ as a COMMITTED artifact. Network tool.

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;

fn re_radix_csr1() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"radix-:r[a-z0-9]*:?").unwrap())
}
fn re_radix_csr2() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"radix-_r_[a-z0-9-]*").unwrap())
}
fn re_radix_ssr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?i)radix-_R_[a-z0-9-]*").unwrap())
}
fn re_preview_wr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r#"<div data-slot="preview"[^>]*><div data-align="[^"]*" data-chromeless="false" class="preview[^"]*">"#)
            .unwrap()
    })
}
fn re_comp_prev() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"<ComponentPreview\b([^>]*)>").unwrap())
}
fn re_name_attr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#"name="([^"]*)""#).unwrap())
}
fn re_fence_open() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("^`{3,}").unwrap())
}
fn re_leading_backticks() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("^`+").unwrap())
}

pub fn norm_snapshot(html: &str) -> String {
    let out = re_radix_csr1().replace_all(html, "radix-<auto>");
    let out = re_radix_csr2().replace_all(&out, "radix-<auto>");
    // SSR react-useId ids share the CSR bucket — runtime-generated, not
    // part of the contract
    re_radix_ssr().replace_all(&out, "radix-<auto>").into_owned()
}

/// fenceShadow blanks fenced regions (newlines and offsets preserved) so span
/// searches see only prose/markup. Line-based: 3+ backtick fences, info
/// strings, unclosed fences blank to EOF (pipeline/docs_transforms.go:26-50).
pub fn fence_shadow(text: &str) -> String {
    let mut lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
    let mut open: isize = -1;
    for line in lines.iter_mut() {
        if open < 0 {
            if re_fence_open().is_match(line) {
                open = re_leading_backticks().find(line).map(|m| m.end()).unwrap_or(0) as isize;
                *line = blank_line(line);
            }
        } else {
            if let Some(m) = re_leading_backticks().find(line) {
                if m.end() as isize >= open {
                    open = -1;
                }
            }
            *line = blank_line(line);
        }
    }
    lines.join("\n")
}

fn blank_line(s: &str) -> String {
    " ".repeat(s.chars().count())
}

/// snapshotPreviewNames: preview names in mdx document order (fences shadowed
/// so fenced ComponentPreview text cannot reorder the mapping).
pub fn snapshot_preview_names(mdx: &str) -> Vec<String> {
    let mut out = Vec::new();
    for m in re_comp_prev().captures_iter(&fence_shadow(mdx)) {
        if let Some(nm) = re_name_attr().captures(&m[1]) {
            out.push(nm[1].to_string());
        }
    }
    out
}

/// snapshotSlicePreviews slices each preview's demo container content out of
/// SSR HTML by stack-balancing <div>/</div>.
pub fn snapshot_slice_previews(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    for loc in re_preview_wr().find_iter(html) {
        let start = loc.end();
        let mut depth: i64 = 1;
        let mut i = start;
        while depth > 0 && i < html.len() {
            let next_open = html[i..].find("<div");
            let next_close = html[i..].find("</div>");
            if next_close.is_none() {
                break;
            }
            let next_close = next_close.unwrap();
            if let Some(next_open) = next_open {
                if next_open < next_close {
                    depth += 1;
                    i += next_open + 4;
                } else {
                    depth -= 1;
                    i += next_close + 6;
                }
            } else {
                depth -= 1;
                i += next_close + 6;
            }
        }
        let end = html[..i.saturating_sub(6)].rfind("</div>");
        if let Some(end) = end {
            if end >= start {
                out.push(html[start..end].to_string());
            }
        }
    }
    out
}

/// Renders an io::Error the way Go's *PathError does: `open <path>: <strerror>`
/// (Go's own lowercase strerror table, not libc's).
fn go_err(op: &str, path: &Path, e: &std::io::Error) -> String {
    let msg = match e.raw_os_error() {
        Some(1) => "operation not permitted",
        Some(2) => "no such file or directory",
        Some(13) => "permission denied",
        Some(17) => "file exists",
        Some(20) => "not a directory",
        Some(21) => "is a directory",
        Some(39) => "directory not empty",
        _ => return e.to_string(),
    };
    format!("{} {}: {}", op, path.display(), msg)
}

/// Renders a ureq transport error the way Go's net/http does:
/// `Get "URL": dial tcp HOST:PORT: connect: <strerror>` for connect failures,
/// `Get "URL": dial tcp: lookup HOST: no such host` for DNS, and the
/// Client.Timeout message for timeouts. Anything else falls back to ureq's
/// Display. (Go's strerror table again — lowercase, not libc's.)
fn go_http_err(url: &str, e: ureq::Error) -> String {
    let disp = e.to_string();
    let t = match e.into_transport() {
        Some(t) => t,
        None => return disp,
    };
    let host = t
        .url()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
        .unwrap_or_default();
    let host_port = match t.url() {
        Some(u) => format!(
            "{}:{}",
            u.host_str().unwrap_or(""),
            u.port_or_known_default().unwrap_or(80)
        ),
        None => String::new(),
    };
    match t.kind() {
        ureq::ErrorKind::Dns => {
            format!("Get \"{}\": dial tcp: lookup {}: no such host", url, host)
        }
        ureq::ErrorKind::ConnectionFailed => {
            if let Some(src) = std::error::Error::source(&t) {
                let s = src.to_string();
                if s.contains("timed out") {
                    return format!(
                        "Get \"{}\": context deadline exceeded (Client.Timeout exceeded while awaiting headers)",
                        url
                    );
                }
                if let Some(ioe) = src.downcast_ref::<std::io::Error>() {
                    if let Some(code) = ioe.raw_os_error() {
                        let msg = match code {
                            1 => "operation not permitted",
                            2 => "no such file or directory",
                            13 => "permission denied",
                            17 => "file exists",
                            20 => "not a directory",
                            21 => "is a directory",
                            39 => "directory not empty",
                            110 => "connection timed out",
                            111 => "connection refused",
                            _ => return disp,
                        };
                        return format!(
                            "Get \"{}\": dial tcp {}: connect: {}",
                            url, host_port, msg
                        );
                    }
                }
            }
            disp
        }
        _ => disp,
    }
}

pub fn run_upstream_snapshot(args: &[String]) -> i32 {
    let mut only = "";
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--page" && i + 1 < args.len() {
            only = &args[i + 1];
        }
        i += 1;
    }

    let pin_b = match fs::read("src/registry/pin.json") {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "upstream-snapshot: {}",
                go_err("open", Path::new("src/registry/pin.json"), &e)
            );
            return 1;
        }
    };
    #[derive(serde::Deserialize)]
    struct Pin {
        #[serde(rename = "shadcn_ui")]
        shadcn_ui: PinShadcnUi,
    }
    #[derive(serde::Deserialize)]
    struct PinShadcnUi {
        registry: String,
    }
    let pin: Pin = match serde_json::from_slice(&pin_b) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("upstream-snapshot: {}", e);
            return 1;
        }
    };
    let m = match Regex::new(r"registry/bases/([^/]+)/")
        .unwrap()
        .captures(&pin.shadcn_ui.registry)
    {
        Some(m) => m,
        None => {
            eprintln!("FAIL  upstream-snapshot: src/registry/pin.json has no `shadcn_ui.registry` of the form apps/v4/registry/bases/<base>/ui — cannot tell which base to crawl");
            return 1;
        }
    };
    let base = &m[1];
    let docs_dir = format!(".upstream/shadcn-ui/apps/v4/content/docs/components/{}", base);
    let out_dir = "src/registry/upstream-snapshot";
    let origin = std::env::var("SHADLESS_SNAPSHOT_ORIGIN").unwrap_or_default();
    let origin = if origin.is_empty() {
        "https://ui.shadcn.com"
    } else {
        &origin
    };
    let crawl_base = format!("{}/docs/components/{}", origin, base);

    // pages with no component-preview on upstream (verified: sidebar and
    // typography render through different chrome — and both sit in our grey
    // list anyway)
    let mut skip: HashMap<&str, bool> = HashMap::new();
    skip.insert("sidebar", true);
    skip.insert("typography", true);

    let ents = match fs::read_dir(&docs_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!(
                "upstream-snapshot: {}",
                go_err("open", Path::new(&docs_dir), &e)
            );
            return 1;
        }
    };
    let mut pages: Vec<String> = Vec::new();
    for e in ents {
        let e = match e {
            Ok(e) => e,
            Err(e) => {
                eprintln!(
                    "upstream-snapshot: {}",
                    go_err("open", Path::new(&docs_dir), &e)
                );
                return 1;
            }
        };
        let name = e.file_name().to_string_lossy().into_owned();
        if name.ends_with(".mdx") {
            let p = name.trim_end_matches(".mdx").to_string();
            if !skip.contains_key(p.as_str()) && (only.is_empty() || p == only) {
                pages.push(p);
            }
        }
    }
    pages.sort();

    if let Err(e) = fs::create_dir_all(out_dir) {
        eprintln!(
            "upstream-snapshot: {}",
            go_err("mkdir", Path::new(out_dir), &e)
        );
        return 1;
    }
    let mut total = 0;
    let mut failed = 0;
    for page in &pages {
        let mdx_b = fs::read(format!("{}/{}.mdx", docs_dir, page)).unwrap_or_default();
        let names = snapshot_preview_names(&String::from_utf8_lossy(&mdx_b));
        let res = match ureq::get(&format!("{}/{}", crawl_base, page))
            .timeout(Duration::from_secs(30))
            .call()
        {
            Ok(r) => r,
            // ureq returns non-2xx as Err(Status); Go's client returns the
            // response and checks StatusCode — same message either way.
            Err(ureq::Error::Status(code, _)) => {
                eprintln!("FAIL {}: HTTP {}", page, code);
                failed += 1;
                continue;
            }
            Err(e) => {
                eprintln!(
                    "FAIL {}: {}",
                    page,
                    go_http_err(&format!("{}/{}", crawl_base, page), e)
                );
                failed += 1;
                continue;
            }
        };
        if res.status() != 200 {
            eprintln!("FAIL {}: HTTP {}", page, res.status());
            failed += 1;
            continue;
        }
        let mut html_b: Vec<u8> = Vec::new();
        // Go ignores the io.ReadAll error here; so do we.
        let _ = res.into_reader().read_to_end(&mut html_b);
        let slices = snapshot_slice_previews(&String::from_utf8_lossy(&html_b));
        if slices.len() != names.len() {
            eprintln!(
                "FAIL {}: {} mdx previews != {} SSR slices",
                page,
                names.len(),
                slices.len()
            );
            failed += 1;
            continue;
        }
        // byte-stable JSON: JSON.stringify(doc, null, 1) + "\n", key order =
        // page, previews, preview names in mdx order
        let mut b = String::new();
        b.push_str("{\n \"page\": ");
        b.push_str(&crate::jsonorder::json_string(page));
        b.push_str(",\n \"previews\": {");
        for (i, n) in names.iter().enumerate() {
            if i > 0 {
                b.push(',');
            }
            b.push_str("\n  ");
            b.push_str(&crate::jsonorder::json_string(n));
            b.push_str(": ");
            b.push_str(&crate::jsonorder::json_string(
                &norm_snapshot(slices[i].trim()),
            ));
        }
        b.push_str("\n }\n}\n");
        if let Err(e) = fs::write(format!("{}/{}.json", out_dir, page), b.as_bytes()) {
            eprintln!(
                "upstream-snapshot: {}",
                go_err("open", Path::new(&format!("{}/{}.json", out_dir, page)), &e)
            );
            return 1;
        }
        total += names.len();
        println!("{}: {} previews", page, names.len());
    }
    if failed > 0 {
        eprintln!("FAIL  upstream-snapshot ({} pages)", failed);
        return 1;
    }
    println!(
        "upstream-snapshot: {} previews across {} pages -> {}",
        total,
        pages.len(),
        out_dir
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mirrors pipeline/upstream_snapshot_test.go TestUnitNormSnapshot.
    #[test]
    fn unit_norm_snapshot() {
        let cases = [
            ("radix-:r1:", "radix-<auto>"),   // CSR useId, colon-delimited
            ("radix-_r_ab", "radix-<auto>"),  // CSR useId, underscore-delimited
            ("radix-_R_AB", "radix-<auto>"),  // SSR useId shares the CSR bucket
        ];
        for (input, want) in cases {
            assert_eq!(norm_snapshot(input), want, "norm_snapshot({:?})", input);
        }
        // all three shapes normalize inside a real attribute soup, not just bare
        let mixed = r#"<div id="radix-:r1:" aria-labelledby="radix-_r_ab-trigger">radix-_R_AB</div>"#;
        let want = r#"<div id="radix-<auto>" aria-labelledby="radix-<auto>">radix-<auto></div>"#;
        assert_eq!(norm_snapshot(mixed), want, "norm_snapshot(mixed)");
    }

    /// Mirrors pipeline/upstream_snapshot_test.go TestUnitSnapshotPreviewNames.
    #[test]
    fn unit_snapshot_preview_names() {
        // A decoy <ComponentPreview> sits inside a fenced code block: fenceShadow
        // must blank it so it cannot reorder or duplicate the real mapping.
        let mdx = "text\n\n\
<ComponentPreview name=\"accordion-demo\" />\n\n\
```tsx\n\
<ComponentPreview name=\"decoy-demo\" />\n\
```\n\n\
<ComponentPreview name=\"accordion-demo-2\" />\n";
        let got = snapshot_preview_names(mdx);
        let want = vec!["accordion-demo".to_string(), "accordion-demo-2".to_string()];
        assert_eq!(got, want);
    }

    /// Mirrors pipeline/upstream_snapshot_test.go TestUnitSnapshotSlicePreviews.
    #[test]
    fn unit_snapshot_slice_previews() {
        // One level of nesting inside the preview's demo container: the
        // depth-balancing loop must walk past the demo's own <div>...</div>
        // before it finds the wrapper's closing tags.
        let simple = r#"<div data-slot="preview" foo="bar"><div data-align="center" data-chromeless="false" class="preview foo"><div class="c">x</div></div></div>"#;
        let got = snapshot_slice_previews(simple);
        let want = r#"<div class="c">x"#;
        assert_eq!(got.len(), 1, "simple: got {:?}", got);
        assert_eq!(got[0], want, "simple: got {:?}", got);

        // Two levels of nesting: the loop must keep balancing through BOTH the
        // inner and outer content divs before landing on the wrapper's own close.
        let nested = r#"<div data-slot="preview"><div data-align="center" data-chromeless="false" class="preview"><div class="outer"><div class="inner">y</div></div></div></div>"#;
        let got2 = snapshot_slice_previews(nested);
        let want2 = r#"<div class="outer"><div class="inner">y</div>"#;
        assert_eq!(got2.len(), 1, "nested: got {:?}", got2);
        assert_eq!(got2[0], want2, "nested: got {:?}", got2);

        // Two previews back to back must each get their own slice.
        let two = format!("{}{}", simple, nested);
        let got3 = snapshot_slice_previews(&two);
        assert_eq!(got3.len(), 2, "two previews: got {:?}", got3);
        assert_eq!(got3[0], want, "two previews: got {:?}", got3);
        assert_eq!(got3[1], want2, "two previews: got {:?}", got3);

        // No preview wrapper at all: no slices, no panic.
        let got4 = snapshot_slice_previews("<div>nothing here</div>");
        assert_eq!(got4.len(), 0, "no wrapper: got {:?}, want none", got4);
    }
}
