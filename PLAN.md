# shadless-rs — Go pipeline 的 Rust 移植實驗計畫

移植對象:`../shadless/pipeline/`(module `shadless/pipeline`,~29k 行非測試碼 + 6.3k 行測試,
41 nodes / 24 gates,唯一外部依賴 `github.com/evanw/esbuild v0.28.2`)。

## 鐵律(沿用 repo 自己的 PORT.md)

> **A tool moves to Rust when Rust can produce the same bytes.**

驗收標準是**位元組級一致**:對真實樹跑 Go pipeline,再跑 Rust pipeline,`generated/ir/*.json`、
`dist/**`、CLI stdout/stderr/exit code 全部 byte-diff。判定工具用 repo 既有機制:
`reproducible` gate、`ir-diff`、以及 Go binary 的 golden 輸出。
做不到位元組一致的部分(見「不移植」)維持 node subprocess —— 這正是 Go 版今天的形狀。

## 範圍

**移植**(pipeline/ module 全部):
- graph/runner:`nodes.go`(41 nodes)、`run.go`、`key.go`(sha256 key 折疊)、
  `fanout.go`(contracts 29 路展開)、`verify.go`(未宣告讀寫稽查)、`main.go` 的 CLI
- 純邏輯:`internal/twmerge`(tailwind-merge 的 Go 移植,492 行)、`internal/tsx` 掃描器、
  `jsonorder.go`(自製 JSON.stringify 語意序列化器)
- 大頭:`convert.go`(3.4k 行,TSX→IR)、`emit.go` + `emitter_css/html.go`、
  `ledger.go`、`overlay.go`、`rtl_dict.go`、`audit_boundary.go`、`mutations.go`(31 mutations)
- gates:`gate_*.go` → `#[test]`(24 個)

**不移植,維持 spawn node**(Go 版也只是包 subprocess):
- playwright 瀏覽器 gates(`demo-parity`、`style-parity`、`contracts:*` 等)
  —— 引 `playwright` Rust crate 會造出第二個 chromium 版本,PORT.md 已實測否決過同類方案
- tailwind CLI(`tw` step)、`docs-build`(mdx/shiki)、JS emitter、React oracle

**不碰**:`.dagger/`(實測不在上游關鍵路徑上,與本移植無關)—— 見「Dagger:實驗不需要它」。

## 技術選型

| 面向 | 選擇 | 理由 |
|---|---|---|
| 工具鏈 | stable rustc 1.98, edition 2024, 單一 crate(lib+bin) | 檔案→模組 1:1 對映(convert.go→convert.rs),機械式可比對 |
| CLI | clap v4 (derive) | 子命令名稱、flag、stdout 格式逐字對齊 Go 版, golden-diff 的前提 |
| esbuild | **spawn node_modules 裡 pinned 的 esbuild binary** | 與 Go in-process api.Transform 同一引擎同一版本 → 同位元組;PORT.md 接受 Go dep 用的就是這個論證。SWC/oxc 產生不同中間文字,會迫使重調 3.4k 行掃描器的 regex,是最高風險路徑 —— 留作第二個實驗 |

### 為什麼不是 Rolldown/Oxc(2026-09 查證)

Rolldown 已穩定(crates.io `rolldown` 1.2.7,底層是 Oxc 家族:`oxc_transformer`/`oxc_minifier`
皆發佈於 crates.io,0.137.x;官網自述 "esbuild feature parity"、"the unified bundler
powering Vite 8+")。注意官網宣稱的是**功能對等(feature parity),不是輸出對等**——
全站無任何與 esbuild 位元組一致的宣稱;transforms/minify 是 bundler 內建功能,
沒有 esbuild Transform API 那種獨立進入點。純 Rust 路線存在,但「直接換」卡在三個事實:

1. **角色不對**:Go 版四個 esbuild 使用點——`convert.go`(Transform,輸出文字被 regex
   掃描器逐字消費)、`jsbuild.go`(字串串接 + minify,**不是 bundling**)、`rtl_dict.go`
   (Transform)——三個都不需要 bundler;只有 `example_fixture.go`/`oracle_lib.go` 的
   `api.Build`(React oracle 測試載體,輸出是 temp、無位元組要求)用得到 Rolldown 的本體能力。
   純 Rust 替換的對應積木是 Oxc 的 transformer/minifier,不是 Rolldown。
2. **位元組一致**:獨立實作的 printer/minifier(引號正規化、空白、識別字改名、死碼消除)
   不可能在全部輸入上同位元組。`dist/shadless.min.js` 是 committed 產品,`reproducible`
   逐位元組比較;jsbuild.go 原話:"a minifier that merely produced *valid* output would
   fail that gate forever"。換 = 重錄全部出貨檔 = PORT.md 點名的失敗模式(silently
   changing the product),且 byte-diff 對該產物的驗證力歸零。
3. **IR 這關躲不掉**:即使 re-baseline dist,IR 仍須與 Go 版逐字一致(`generated/ir` 是
   commit 的,下游與 overlay hash 全依賴)。以 Oxc 印表機重調掃描器換取 IR-identity 是
   合法的第二實驗,但要在有已驗證 baseline 可 A/B 之後做(見 M4 後)。

混合路線(採納):第一輪 spawn esbuild 把移植機器驗證完;「Oxc 替換 transform/minify」
掛 feature flag 作為 M4 之後的獨立實驗;oracle bundle(`api.Build` 處)若要移植,
Rolldown/Oxc 在該點無位元組負擔,可先行。

### DAG runner:為什麼自寫(已定案:本地 runner 為主要交付)

使用者定案:本地 `pipeline run` 取代 Dagger 成為日常路徑;`.dagger` module 留給 CI 不動。
「Rust 有沒有現成 DAG 工具」的盤點結論——**沒有合身的**,因為這個 graph 的模型是
「內容雜湊的行為記憶化」,不是通用工具的任何一種:

| 候選 | 不合身的原因 |
|---|---|
| just / cargo-make / go-task | 有依賴、無 staleness(更無內容雜湊) |
| ninja | mtime/restat 語意、規則須產出檔案;24/41 nodes 是「宣稱而不產出」的 gates,`status` 判定無法重現 |
| turborepo / moon | 模型最接近(hash 行為、Rust 核心),但 workspace/package 導向,gate-as-test 塞不進其輸出快取模型,且把 JS 生態工具放在 Rust 實驗正中央 |
| petgraph | 圖演算法本身用不上:Go 的 `Plan`(graph.go)是「target 宣告序、`Needs` 宣告序」的 DFS 後序,**遍歷順序是輸出介面的一部分**(golden-diff 標的),本來就必須逐行手寫那個 DFS(~30 行,含 cycle path 回報格式);petgraph 的 toposort 給不出這個序。註:advisor 建議可引入 petgraph 當演算法引擎,此處依實測的 Go 遍歷語意不取——差一個 Cargo.toml 行,日後要 DOT 匯出再加 |

自寫範圍因此很小:scheduler ~150 行(std::thread + mpsc,indegree 計數的 ready-queue,
browser 用量為普通計數器上限)、key 摺疊 + stamps 移植 ~300 行。**沒有工具能代勞的部分
——key 雜湊摺疊與 stamp 語意——才是風險所在**,也是 golden-diff 的標的。

決定性規則(移植時逐條對 Go 驗證):
- `plan` 輸出 = 宣告序 DFS 後序,逐字重現;`list`/`status` 同序迭代。
- runner 的 ready-queue 平手取字典序最小(Go 執行序本就依時序,只求 log 可讀,不求逐位元組)。
- M2 驗收清單:全 41-node graph 的 `plan`/`list`/`status` 對 Go binary golden-diff;
  人為 cycle 案例的錯誤訊息;fresh vs `--force`;`inputs: null` 的 NEVER-FRESH;
  乾淨 clone 的 OutputsPresent;`-j1` 未宣告讀寫稽查判定一致。

### Dagger:實驗不需要它(2026-09 實測)

實測上游 repo 的 dagger 使用面:`.dagger` **不在任何關鍵路徑上**——repo 無 `.github/`
(沒有 CI 定義)、Makefile 全部走 host runner + `go test`(零 dagger 引用)、
README/CONTRIBUTING/docs 零提及。`dagger.json`(v0.21.9, Go SDK)與 PORT.md 的
「The Dagger port」(convert/emit/contract 三切片已驗證)是一條平行、部分完成的
容器化嘗試,目前沒有任何東西呼叫它。

結論:
1. **Rust 實驗零依賴 Dagger**:所有驗收(golden-diff、byte-diff、gates)都在 host 上,
   `.dagger` 不必碰、不必改接、不必為它準備 `rust-toolchain.toml`。
2. 不刪它(那是上游的東西,不是本移植的範圍),但也不維護。上游若日後要容器化 CI,
   之前的分析仍適用:rust image 從 `rust-toolchain.toml` 解析 + `cargo build`,
   module 其餘原樣接上同 CLI 的 Rust binary。
3. 無 Dagger 的 CI 形狀(供上游日後參考,不屬於本移植):輸出已 commit,CI 可走
   `pipeline run --gates-only`(「假設產物 fresh」的語意本來就存在);或全量 cold run
   吃 29×browser contracts + oracle 的成本。stamps 已改為不追蹤,故沒有免費的暖快取,
   這是上游的產品決定。
4. **CI 不列入實驗範圍(定案)**:驗證紀律已內建在本地 gates(gates 是 tests,
   `cargo test` 全跑)。本地驗證唯一蓋不到的是「沒人跑就不會發現」的外部漂移——上游
   shadcn pin 移動(`re-pin drill` 是手動流程)、或鎖定版本之外的環境差異。port 若
   日後成為日常路徑,一個排程跑 `pipeline run full` 就是最低需求的 CI,屆時再議。
| 並行 | std::thread + mpsc(crossbeam 僅取 scoped threads 時) | scheduler 手寫 ~150 行(見「DAG runner」);rayon 是 data-parallel 形狀不對,tokio 無網路 I/O 買不到東西;**petgraph 不引入** —— Plan 的遍歷序是宣告序 DFS 後序(見下),petgraph 的 toposort 反而給不出 |
| JSON IR | **逐行移植 jsonorder.go**(~150 行),不用 serde_json | serde Formatter 表達不了 jsonRaw(原樣數字字面值)、absent-vs-null、no-HTML-escape;150 行手寫消掉一整類風險 |
| 雜湊 | sha2 + 手工對齊 Go 的 key 拼接位元組佈局 | key 裡任何 `%q`/`%+v` 格式化都要逐字重現,否則跨工具 freshness 失效 |
| regex | `regex` crate | 兩邊都是 RE2 血統(無 backref/lookahead);**但 Rust `\b`/`\w` 預設 Unicode、Go 預設 ASCII** —— 逐處 audit,`(?i)` 也要查 |
| glob | globset,或直接移植 key.go 的自製 `**` matcher | Go 的 filepath.Glob 沒有 `**`,現有實作是自製編譯;用 globset 須先對真實 graph diff 測兩者行為;**列舉結果必須顯式排序**(Go filepath.Walk 是字典序,walkdir 是 OS 序) |
| 測試 | `#[test]` + `--test-threads=1`(或 cargo-nextest) | cargo 無測試結果快取,天然等於 Go 那個 load-bearing 的 `-count=1`;**但 libtest 預設多執行緒,而 gates 共享同一棵樹**,必須釘死單執行緒;nextest 每測一 process 更乾淨 |
| dev-deps | assert_cmd + predicates、tempfile、walkdir | golden fixtures 必須含 stdout、stderr、exit code 三者 |

其餘依賴:serde/serde_json(讀 `gates/*.json` 等輸入,不碰生成檔)、crossbeam-channel。
不用 tokio、reqwest(Go 版唯一的 dep 就是 esbuild;pin 走 shell out 到 git,Rust 照做)。

## esbuild route 的前置 probe(第一件事)

1. 對 pinned registry 全部 .tsx,收 Go `esbuildTsx()`(api.Transform)輸出存檔。
2. 用 `node_modules/.bin/esbuild --loader=tsx`(classic JSX runtime 的對應 flags)對同輸入變換,diff。
3. 已知差異點:尾端換行、錯誤文字格式;flags 必須釘到印出文字與 scanner regex 期望一致。
4. jsbuild 的 minify/bundle 同法驗證(dist/esm 產物 byte-diff)。
5. probe 不過 → 回到方案討論,不硬幹。

## 移植順序(每步有獨立驗收)

| # | 內容 | 驗收 |
|---|---|---|
| M0 | esbuild probe(上節) | 全 registry .tsx 變換輸出 byte-identical |
| M1 | scaffold + `nodes.go`/`graph.go`/`key.go`/`main.go` 的 `plan`/`list`/`status` | 對真實樹,與 Go binary 三個子命令 stdout 逐字一致(含 NEVER-FRESH/STALE 判定) |
| M2 | `run.go` runner + `verify.go` + stamps | `pipeline run` 在乾淨樹上跑出的樹與 Go 版一致;`-j1` 未宣告寫入稽查同樣運作 |
| M3 | `internal/twmerge` + `internal/tsx`(純邏輯) | 對 Go 版輸出的單元級 golden 測試 |
| M4 | `convert.go` → IR | `generated/ir/*.json` 61 檔 byte-identical;repo 的 `ir-diff` 當裁判 |
| M5 | `emit`/`tw`/`product-css`/`oracle-css`/`jsbuild` | `dist/**` byte-identical;tailwind 與 esbuild 皆 spawn |
| M6 | gates(24 個)+ ledger/overlay/audit/mutations | `cargo test --test-threads=1` 全綠且與 `go test` 判定一致;meta-gate/mutations 最後 |

## A/B 驗證方法論

乾淨 checkout → Go `pipeline run` 全綠 → 快照全部輸出 + `pipeline/stamps/` + 各 CLI 輸出
(stdout/stderr/exit code)→ Rust 同輸入執行 → 全量 byte-diff。任何差異先分類:
「Rust 錯」vs「Go 版依賴了未定義行為(如 map 迭代序)」—— 後者要在計畫裡記錄,不是顺手改 Go。

## 已知風險清單

1. **regex Unicode 預設差異**(Rust `\w`/`\b` Unicode vs Go ASCII)、`(?i)` 折疊範圍 —— M4 前逐條 audit。
2. **walkdir 目錄列舉順序** ≠ Go filepath.Walk 字典序 —— 所有影響雜湊輸入的列舉顯式排序。
3. **cargo test 多執行緒** vs Go gates 序列 —— 釘 `--test-threads=1` 或用 nextest。
4. **浮點格式化**:`%.1fs`、`%g`、run-report.json 的數字在邊界值分歧 —— golden-diff 會抓到。
5. **process spawn 細節**:env、cwd、PATH 查找、每 node 輸出緩衝成塊、JS node 的 tempdir 語意。
6. **stamps 序列化**:node id 的 `:`→`__` 跳脫、檔案尾換行,跨工具要能互讀。
7. Go map 迭代隨機 → 任何在 Go 裡靠 sort 自救的地方,Rust 端用 BTreeMap/IndexMap 保持決定性。
8. UTF-16 語意:`internal/tsx/utf16.go` 處理 JS 字串位置,移植時位置計算要逐 case 對測。
