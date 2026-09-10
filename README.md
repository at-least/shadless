# shadless-rs

Rust 移植實驗:把 [`shadless`](../shadless) 上游 repo 的 `pipeline/`(Go,~29k 行,
41 節點 / 24 gates)整顆搬到 Rust,鐵律沿用上游 PORT.md:

> **A tool moves to Rust when Rust can produce the same bytes.**

驗收是位元組級的:`generated/ir/*.json`、`dist/**`、CLI stdout/stderr/exit code
全部與 Go binary byte-diff;gates 的驗收是 verdict 同值(含突變紅側)。

## 現況(2026-09)

- **移植完成**:M0–M6 全數到位(esbuild probe → graph/runner → twmerge/tsx →
  convert → emit 鏈 → 24 gates + ledger/overlay/mutations)。
- **自我接管(M7)**:引擎預設不再 spawn 任何 Go——節點命令是
  `__self__@<per-node-fp>`(本 binary),go-test gates 走 `__gate`。
  `SHADLESS_GRAPH=go-mirror` 呈現 Go 逐字表,供驗收台續用。
- **per-node 引擎指紋**:build.rs 對 hull(src 根檔案 + Cargo 檔)與八個實作
  群組分別雜湊;`nodes.rs` 的 NODE_ENTRIES/GROUP_DEPS 摺出每節點的 fp,改一個
  群組原始碼只 stale 執行它的節點與其 needs 下游。soundness 不靠解析,靠
  raw-grep 執法測試(含「include 不得伸出 src」審計)。
- **Oxc 實驗(已結案,見 probe/oxc/REPORT.md)**:transform 替換測死
  (0/61 位元組一致,印表機層分歧不可配置);oracle bundle 點已以 rolldown 1.2.8
  證明可換(`SHADLESS_ORACLE_BUNDLER=oxc`,227 頁 oracle 驗收 PASS,預設仍
  esbuild);minify 受位元組契約封閉。

## 怎麼驗證

需要 sibling checkout:`../shadless`(上游樹 + 其 node_modules 與 module cache)。
**驗收完全自證,不需要 Go 工具鏈**(2026-09-10 起;歷史上的雙引擎位元組對照
見 git tag `go-parity-final` 與 opt-in 的 gate_parity):

```sh
cargo build --release
cd ../shadless
../shadless-rs/target/release/pipeline run all     # 全圖執行 + gates,期待 exit 0
../shadless-rs/target/release/pipeline status all  # 期待 68 fresh + 1 NEVER-FRESH
cd ../shadless-rs
./tests/gen_golden.sh    # 四層自證驗收台(錄製 + 結構斷言;PATH 上有沒有 go 都通過)
cargo test               # 124 lib 測試(含真樹 gates + golden 重播)+ 整合測試
# 可選:與 Go 的雙引擎 verdict 對照(需要 Go 工具鏈與重建的 Go binary)
SHADLESS_GATE_PARITY=1 cargo test --test gate_parity  # 貴,~80 分鐘;GATE_ONLY=id 切片
```

陷阱備忘:重連結 binary 或跑 cargo 期間不要並行 `run`(fork/exec ENOENT);
真樹跑過引擎後先重跑 `gen_golden` 再 `cargo test`(status goldens 與 stamps 耦合);
stamps 是 upstream 樹的 runtime 產物,fresh clone 上全 STALE 直到第一次收斂。

## 文件

- [PLAN.md](PLAN.md) — 移植計畫、技術選型、自我接管設計、Oxc 實驗結案。
- [PROGRESS.md](PROGRESS.md) — 逐輪進度、每個 bug 的根因與證據、已知偏差清單。
- [probe/](probe/) — 驗收用探針(m0: esbuild CLI vs Go api.Transform;
  oxc: 純 Rust toolchain 離 esbuild 多遠的 A/B 量化)。
