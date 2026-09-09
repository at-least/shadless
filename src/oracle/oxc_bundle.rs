//! rolldown-backed oracle bundle — the no-byte-contract `api.Build` point.
//!
//! PLAN.md「Oxc 替換」的開放支線:oracle 的 bundle 是瀏覽器執行的暫存產物,
//! 驗收標準是渲染出來的 DOM(`example-oracle --check`),不是位元組。本檔
//! 用 rolldown(oxc 家族的 bundler)取代該點的 esbuild spawn,由
//! `SHADLESS_ORACLE_BUNDLER=oxc` 在 oracle 群組內閘——預設路徑(與 Go
//! 位元組契約綁定的 esbuild)一個字都不動。
//!
//! 與 esbuild 行為對齊的點:alias 語意(精確匹配或 `key/` 前綴)、
//! platform=Browser 時自動定義 `process.env.NODE_ENV=development`
//! (rolldown 與 esbuild 同規則,react 走 development 分支)、jsx 自動
//! runtime(rolldown 對 Tsx 的預設)。

use std::collections::HashMap;
use std::path::Path;

pub fn bundle_oracle_oxc(
    root: &Path,
    entry: &Path,
    aliases: &HashMap<String, String>,
    outfile: &Path,
) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| format!("rolldown: tokio runtime: {e}"))?;
    rt.block_on(bundle_async(root, entry, aliases, outfile))
}

async fn bundle_async(
    root: &Path,
    entry: &Path,
    aliases: &HashMap<String, String>,
    outfile: &Path,
) -> Result<(), String> {
    use rolldown::{BundlerBuilder, BundlerOptions, InputItem, OutputFormat, ResolveOptions, TsConfig};

    // esbuild --alias:k=v: exact match or `k/…` prefix, and the LONGEST
    // matching key wins. oxc_resolver (webpack semantics) takes the FIRST
    // matching key in list order, so feed the keys most-specific-first —
    // otherwise the bare `@` alias (present in the real table) swallows
    // every `@/…` import.
    let mut alias_keys: Vec<&String> = aliases.keys().collect();
    alias_keys.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
    let alias: Vec<(String, Vec<Option<String>>)> = alias_keys
        .into_iter()
        .map(|k| (k.clone(), vec![Some(aliases[k].clone())]))
        .collect();

    // esbuild ignores tsconfig.json unless pointed at one; rolldown's
    // TsConfig::Auto(true) default auto-discovers it up the importer's tree —
    // the upstream app's tsconfig maps `@/*` into the app dir and beats our
    // alias table, dragging unresolvable app deps (streamdown) into the
    // bundle. Disable discovery: the alias table is the ONLY path mapping.
    let options = BundlerOptions {
        input: Some(vec![InputItem {
            name: Some("entry".to_string()),
            import: entry.to_string_lossy().into_owned(),
        }]),
        cwd: Some(root.to_path_buf()),
        format: Some(OutputFormat::Iife),
        tsconfig: Some(TsConfig::Auto(false)),
        resolve: Some(ResolveOptions {
            alias: Some(alias),
            ..ResolveOptions::default()
        }),
        ..BundlerOptions::default()
    };

    let mut bundler = BundlerBuilder::default()
        .with_options(options)
        .build()
        .map_err(|e| format!("rolldown: build: {e}"))?;
    let output = bundler
        .generate()
        .await
        .map_err(|e| format!("rolldown: generate: {e}"))?;

    for w in &output.warnings {
        eprintln!("rolldown warning: {w}");
    }
    // single JS chunk expected (Output is not re-exported by the rolldown
    // crate; its filename()/content_as_bytes() methods carry everything needed)
    let mut js_chunks: Vec<&str> = Vec::new();
    let mut total = 0usize;
    for o in &output.assets {
        total += 1;
        if o.filename().ends_with(".js") {
            js_chunks.push(o.filename());
        }
    }
    let bytes = match js_chunks.len() {
        1 => {
            let o = output
                .assets
                .iter()
                .find(|o| o.filename().ends_with(".js"))
                .expect("checked above");
            o.content_as_bytes()
        }
        0 => return Err("rolldown: produced no JS chunk".to_string()),
        n => {
            return Err(format!(
                "rolldown: produced {n} JS chunks of {total} outputs, expected 1 \
                 (code splitting is not wanted here)"
            ))
        }
    };
    std::fs::write(outfile, bytes).map_err(|e| format!("{}: {e}", outfile.display()))?;
    Ok(())
}

