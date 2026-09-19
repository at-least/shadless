# pipeline — the shadless build engine (Rust)

This crate IS the engine of the repo it lives in. It began as a port of the
Go pipeline that used to sit at this same path (the port's history is a
parent of the repo's merge commit; the last dual-engine state was tag
`go-parity-final` — that tag did not survive into this repo, the Go
module's last state is `go-engine-final`), and
the port charter survives in PORT.md:

> **A step moves into the Rust engine when it can produce the same bytes.**

驗收是位元組級的:`generated/ir/*.json`、`dist/**`、CLI stdout/stderr/exit code
的位元組同值由 `reproducible` gate 對 committed 樹逐位元組執法;gates 的
驗收是 verdict 同值(含 `__meta` 突變紅側)。

## 現況(2026-09)

- **移植完成並已取代 Go 引擎**(2026-09-11,tag `rust-engine-initial`):
  M0–M6 → 自我接管(M7)→ per-node 引擎指紋 → Oxc 實驗結案(oracle
  bundle 點採用 rolldown 為預設;transform/minify 受位元組契約封閉,
  見 probe/oxc REPORT — 該實驗目錄未隨移植併入本 repo)→ 完全取代(單一 repo,零 Go)。

## 怎麼驗證

**驗收完全自證,不需要 Go 工具鏈**(歷史上的雙引擎位元組對照記錄在
git tag `go-parity-final` — 該 tag 未隨移植併入本 repo,本 repo 僅存
`go-engine-final` 與 `rust-engine-initial`;opt-in 的 gate_parity 交叉檢查
已於引擎取代 Go 時刪除):

```sh
make pipeline           # cargo build --release → build/pipeline(或 npm run pipeline)
./build/pipeline run all     # 全圖執行 + gates,期待 exit 0
./build/pipeline status all  # 期待 68 fresh + 1 NEVER-FRESH
cd pipeline
./tests/gen_golden.sh   # 四層自證驗收台(錄製 + 結構斷言;PATH 上有沒有 go 都通過)
cargo test              # ~148 lib 測試(含真樹 gates + golden 重播)+ 整合測試
```

陷阱備忘:重連結 binary 或跑 cargo 期間不要並行 `run`(fork/exec ENOENT);
真樹跑過引擎後先重跑 `gen_golden` 再 `cargo test`(status goldens 與 stamps 耦合);
stamps 是 runtime 產物(`pipeline/stamps/`,不追蹤),fresh clone 上全 STALE
直到第一次收斂,且 `npm run pin` 要先跑(unit 的 JS 讀 `.upstream` 皮膚檔)。

## 文件

- [PORT.md](PORT.md) — 章程:什麼進引擎、什麼留在外部工具鏈、歷史 tag。
- [PLAN.md](PLAN.md) — 移植計畫、技術選型、自我接管設計、Oxc 實驗結案。
- [PROGRESS.md](PROGRESS.md) — 逐輪進度、每個 bug 的根因與證據、已知偏差清單。
