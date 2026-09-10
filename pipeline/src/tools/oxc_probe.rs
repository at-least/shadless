//! Oxc A/B probe — the M0 experiment in reverse (PLAN.md「Oxc 替換」).
//!
//! M0 proved `node_modules/.bin/esbuild` == Go `api.Transform` byte-for-byte;
//! this probe measures how far pure-Rust oxc (parse → transform → codegen) is
//! from that same esbuild output on the SAME input set M0 used
//! (`build/resolved-ui/ui/*.tsx`). Byte-identity is NOT expected (PLAN.md:
//! an independent printer cannot match esbuild on all inputs) — the probe
//! quantifies the divergence: per-file verdict, first-divergence offset with
//! context, and a summary. Findings feed the decide-then-investigate loop in
//! PLAN.md; nothing here feeds the graph.
//!
//! Hidden verb `__oxc-probe`, compiled only under `--features oxc`:
//!     pipeline __oxc-probe [--minify] [--limit N]

use std::io::Write as _;
use std::path::Path;

/// Transform one TSX source through oxc: parse → semantic → transform →
/// codegen. Flags mirror `convert::esbuild_tsx` one-to-one:
/// loader=tsx (SourceType), jsx=transform + factory/fragment (classic
/// runtime), format=esm (no-op for a single file), charset=utf8 (output is
/// a Rust String; non-ASCII is printed raw).
fn oxc_tsx(path: &Path, src: &str) -> Result<String, String> {
    use oxc_allocator::Allocator;
    use oxc_codegen::{Codegen, CodegenOptions, IndentChar};
    use oxc_parser::Parser;
    use oxc_span::SourceType;
    use oxc_transformer::{JsxOptions, JsxRuntime, TransformOptions, Transformer};

    let source_type = SourceType::from_path(path)
        .map_err(|e| format!("source type: {e}"))?;
    let allocator = Allocator::default();

    let parser_ret = Parser::new(&allocator, src, source_type).parse();
    if let Some(err) = parser_ret.diagnostics.into_iter().next() {
        return Err(format!("oxc parse: {err}"));
    }
    let mut program = parser_ret.program;

    let jsx = JsxOptions {
        jsx_plugin: true,
        runtime: JsxRuntime::Classic,
        pragma: Some("React.createElement".to_string()),
        pragma_frag: Some("React.Fragment".to_string()),
        ..JsxOptions::default()
    };
    let options = TransformOptions {
        jsx,
        ..TransformOptions::default()
    };

    let scoping = oxc_semantic::SemanticBuilder::new()
        .build(&program)
        .semantic
        .into_scoping();

    let transformer = Transformer::new(&allocator, path, &options);
    let transform_ret = transformer.build_with_scoping(scoping, &mut program);
    if let Some(err) = transform_ret.diagnostics.into_iter().next() {
        return Err(format!("oxc transform: {err}"));
    }

    // esbuild prints 2-space indentation; oxc defaults to 1 tab.
    // esbuild's transform drops normal/jsdoc comments (keeps /*! legal and
    // pure annotations); oxc's CodegenOptions defaults print them.
    let codegen_ret = Codegen::new()
        .with_options(CodegenOptions {
            indent_char: IndentChar::Space,
            indent_width: 2,
            comments: oxc_codegen::CommentOptions {
                normal: false,
                jsdoc: false,
                ..oxc_codegen::CommentOptions::default()
            },
            ..CodegenOptions::default()
        })
        .build(&program);
    Ok(codegen_ret.code)
}

fn strip_ws(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn first_divergence(a: &str, b: &str) -> (usize, String) {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let mut i = 0;
    while i < ab.len() && i < bb.len() && ab[i] == bb[i] {
        i += 1;
    }
    let ctx = |s: &str| -> String {
        let mut from = i.saturating_sub(60);
        while from > 0 && !s.is_char_boundary(from) {
            from -= 1;
        }
        let mut to = (i + 90).min(s.len());
        while !s.is_char_boundary(to) {
            to += 1;
        }
        s[from..to].replace('\n', "\\n")
    };
    (
        i,
        format!(
            "first divergence at byte {i} (a.len={}, b.len={})\n  a: …{}\n  b: …{}",
            ab.len(),
            bb.len(),
            ctx(a),
            ctx(b)
        ),
    )
}

/// Run the A/B probe. `root` is the upstream tree; `rest` the CLI args after
/// the verb.
pub fn run_oxc_probe(root: &Path, rest: &[String]) -> Result<i32, String> {
    let mut limit: usize = usize::MAX;
    let mut normalize = false;
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--limit" => {
                i += 1;
                limit = rest
                    .get(i)
                    .and_then(|v| v.parse().ok())
                    .ok_or_else(|| "--limit needs a number".to_string())?;
            }
            // strip ALL whitespace on both sides before comparing: splits
            // "printer policy" divergence (line breaks/indent) from real
            // transform divergence. Crude by design (string contents lose
            // internal whitespace too, identically on both sides).
            "--normalize" => normalize = true,
            other => {
                return Err(format!(
                    "unknown flag {other} (supported: --limit N, --normalize)"
                ))
            }
        }
        i += 1;
    }

    let ui = root.join("build/resolved-ui/ui");
    let mut inputs: Vec<std::path::PathBuf> = walkdir::WalkDir::new(&ui)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().map_or(false, |x| x == "tsx")
        })
        .map(|e| e.path().to_path_buf())
        .collect();
    inputs.sort();
    if inputs.len() > limit {
        inputs.truncate(limit);
    }
    if inputs.is_empty() {
        return Err(format!(
            "no .tsx under {} — build the resolved-ui tree first",
            ui.display()
        ));
    }

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut identical = 0usize;
    let mut whitespace_only = 0usize;
    let mut failures = 0usize;
    let mut diverged = 0usize;
    let mut offsets: Vec<usize> = Vec::new();
    let mut size_delta: Vec<i64> = Vec::new();

    for path in &inputs {
        let name = path.strip_prefix(root).unwrap_or(path).display().to_string();
        let src = std::fs::read_to_string(path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        let a = crate::convert::esbuild_tsx(root, &src)?;
        let b = oxc_tsx(path, &src);
        match b {
            Err(e) => {
                failures += 1;
                writeln!(out, "✘ {name}: oxc error: {e}").ok();
            }
            Ok(b) if b == a => {
                identical += 1;
                writeln!(out, "✔ {name} ({} bytes, identical)", a.len()).ok();
            }
            Ok(b) => {
                size_delta.push(a.len() as i64 - b.len() as i64);
                let a_cmp = if normalize { strip_ws(&a) } else { a.clone() };
                let b_cmp = if normalize { strip_ws(&b) } else { b.clone() };
                if normalize && a_cmp == b_cmp {
                    whitespace_only += 1;
                    writeln!(
                        out,
                        "≈ {name}: whitespace-only divergence (esbuild {} bytes, oxc {} bytes)",
                        a.len(),
                        b.len()
                    )
                    .ok();
                } else {
                    diverged += 1;
                    let (off, detail) = first_divergence(&a_cmp, &b_cmp);
                    offsets.push(off);
                    writeln!(out, "≠ {name}: {detail}").ok();
                }
            }
        }
    }

    writeln!(
        out,
        "== oxc vs esbuild: {identical}/{} byte-identical, {whitespace_only} whitespace-only, \
         {diverged} diverged, {failures} oxc errors",
        inputs.len()
    )
    .ok();
    if !offsets.is_empty() {
        offsets.sort_unstable();
        let median = offsets[offsets.len() / 2];
        writeln!(
            out,
            "== first-divergence offset: min={} median={} max={} ({} diverged files)",
            offsets[0],
            median,
            offsets[offsets.len() - 1],
            offsets.len()
        )
        .ok();
    }
    if !size_delta.is_empty() {
        size_delta.sort();
        writeln!(
            out,
            "== esbuild−oxc size delta: min={} median={} max={} bytes",
            size_delta[0],
            size_delta[size_delta.len() / 2],
            size_delta[size_delta.len() - 1]
        )
        .ok();
    }
    out.flush().ok();
    Ok(if identical == inputs.len() && failures == 0 { 0 } else { 1 })
}
