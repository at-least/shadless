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

    // Canonicalize the entry. A relative entry (example_oracle passes root =
    // ".") reached rolldown as "./node_modules/…": react imported from that
    // relative graph resolved as a second instance next to the one the
    // absolutely-aliased .upstream imports see, and every hook-using oracle
    // render died with "Invalid hook call" → empty #root. esbuild never hits
    // this because it realpaths the entry before resolving.
    let entry = std::fs::canonicalize(entry)
        .map_err(|e| format!("{}: canonicalize: {e}", entry.display()))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// REGRESSION GUARD for the rolldown double-react failure: example-oracle
    /// calls build_oracle with root = Path::new(".") (the engine cwd); with a
    /// relative root the entry reached rolldown as "./node_modules/…", the
    /// react imported from that relative module graph resolved as a SECOND
    /// instance next to the one the absolutely-aliased .upstream imports see,
    /// and every oracle render died with "Invalid hook call" → empty #root.
    /// The bundler must canonicalize the entry so one react instance exists.
    ///
    /// The probe count: a healthy bundle registers `react.forward_ref` 3×
    /// (react + jsx-runtime + react-dom-client); a doubled react shows 5.
    #[test]
    fn build_oracle_via_relative_root_bundles_one_react() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        std::env::set_current_dir(&root).unwrap();
        let html = crate::oracle::oracle_lib::build_oracle(
            Path::new("."),
            "alert-dialog-basic",
            Path::new("build/zz-bundler-probe"),
        )
        .unwrap();
        let bundle = root
            .join("node_modules/.cache/shadless/oracle")
            .join("oxc-bundle-alert-dialog-basic.js");
        let text = String::from_utf8(std::fs::read(&bundle).unwrap()).unwrap();
        let fwd = text.matches("Symbol.for(\"react.forward_ref\")").count();
        assert_eq!(
            fwd, 3,
            "alert-dialog-basic bundle must contain exactly one react instance"
        );
        let _ = html;
        let _ = std::fs::remove_dir_all(root.join("build/zz-bundler-probe"));
    }
}
