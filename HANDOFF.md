# HANDOFF — shadless, 2026-09-02

給下一個 context 的交接。讀完這份 + `git log --oneline -15` 就能接著做。
repo: https://github.com/at-least/shadless

**先看這個**：`origin/main` 還停在 `6c5c630`，本地 `main` 領先 **91 個 commit，全部沒 push**。
working tree 乾淨。**`make` 全綠**。

---

## 1. 現在怎麼跑

CI 不存在（`.github/` 整個沒有）。所有東西都在本機或 Dagger 裡跑。

```sh
make                      # 全跑：build 步驟 + 每個 gate
make fast                 # 不開瀏覽器的 gate（<5s，pre-commit 用）
make only ID=<node>       # 一個節點 + 它需要的 closure
make meta                 # mutation test（ONLY=<id> 跑單一個、TIER=fast）
make serve                # 文件站（PORT ?= 8765）
npm run docs              # 重建文件站
```

圖在 `pipeline/nodes.go`，stamps 在 `pipeline/stamps/`（不追蹤 → 新 clone 冷跑）。

---

## 2. 今天（2026-09-01）做的：mjs → Go 移植，Wave 0–2 完成

目標：把所有 .mjs 建置工具盡可能轉成 Go。策略（使用者拍板）：
**chromium 閘門最後做（Go 主控、node 只做瀏覽器薄殼）、converter/babel 最後做、清理先行**。

### 已完成（19 commits，`e7921e2`…`abaab47`）

| 工具 | 去向 | 驗證 |
|---|---|---|
| `src/docs/{assets,components,jsx,highlight,highlight-client,icons}.mjs` + `docs-page-lib` + `build-demo` | **刪**（死碼，HANDOFF 舊 §4.2） | unit-check 279 斷言 |
| tailwind-merge v3.6.0 | `pipeline/internal/twmerge` | 555/555 快照一致（`tools/twmerge-dump.mjs` 重建） |
| @babel/parser 的 StringLiteral offsets | `pipeline/internal/tsx`（手寫掃描器） | 562 檔 10,722 spans 對齊 babel（1 個已記載容忍差異） |
| `tools/resolve-skins.mjs` | `pipeline resolve-skins`（convert 節點直接跑） | `TestUnitResolveSkinsParity` byte-identical |
| `tools/rtl-dict.mjs` | `pipeline rtl-dict` | `TestUnitRtlDictParity` byte-identical |
| `tools/rtl-lib` 的 substituteAndPatch + `tools/build-rtl.mjs` | `pipeline build-rtl` | `TestUnitBuildRtlParity` |
| `src/tags.mjs` | `pipeline/tags.go`（JS 檔留給殘餘 JS 消費者） | tags 斷言隨 unit/emitter |
| theme-prepaint 常數 | `pipeline/prepaint.go`（單一 Go 事實來源） | — |
| `src/emitter/css.mjs` 的 componentCss/wrapComponentCss | `pipeline/emitter_css.go` | **61 個 IR golden 全等**（`TestUnitEmitterCssGolden` 對 JS 直跑比對） |
| DEFAULT_CONTENT | `pipeline/default_content.go`（`tools/default-content-dump.mjs` 生成） | key 驗證 + overlay mutation 打 Go 檔 |
| `src/emitter/index.mjs`（emit 節點本體） | `pipeline emit` | `TestUnitEmitParity` byte-identical；jsdom slot-tree gate → `x/net/html` |
| `tools/demo.mjs` + `demo-lib.mjs` | `pipeline demo` | `TestUnitDemoParity` byte-identical |
| `tools/docs-consistency.mjs` | `pipeline docs-consistency` | 與 JS 版同輸出 |
| **docs 群（db74f9c）**：`docs-build.mjs`(657行)、`docs-guides.mjs`、`docs-fidelity{,-lib}.mjs`、`src/docs/{transforms,frontmatter,demo-scripts}.mjs` | `pipeline docs-build / docs-guides / docs-fidelity` + `docs_{transforms,frontmatter,scripts,guides,build,families,fidelity*}.go` | `TestUnitDocsBuildParity` byte-identical（components/guides/index/content-map/sidebar 全部）；docs-fidelity Go/JS 同輸出後刪 JS；prettier 走 `tools/prettier-batch.mjs` 子進程（stdin JSON in、JSON out） |
| `tools/upstream-snapshot.mjs`（abaab47） | `pipeline upstream-snapshot`（net/http） | byte-stable JSON；`.dagger/` 在 golang 容器內建 binary（goImage） |
| **Wave 3（775f10d…24fa3d2）**：demo-smoke、docs-smoke、interactivity-sweep、example-golden、example-oracle、style-parity、demo-parity、path-parity | 全部 `pipeline <verb>`，走 `tools/browser-shell.mjs` 薄殼（launch/newPage+事件擷取/goto/evaluateFn+arg/locator/mouse/keyboard/addStyleTag/setContent/driver/loadContractDef/routeAbortExternal） | 每個的 PASS 行與 JS 版 byte-for-byte 相同後才刪 JS；parity-baseline 共享邏輯在 `pipeline/parity_baseline.go`；oracle bundle 走 esbuild Go API（`oracle_lib.go`），canonOf 留 JS 本質、embed 成 `oracle_canon.js` |
| `tools/example-fixture.mjs`（42dfadc，--contracts 同支） | `pipeline example-fixture [--contracts]` | 105+14 頁 byte-identical（跑完 git 乾淨）、PASS 行同 JS；id 映射/收割/tabs 重組/API 走查是 embed 的 page-JS（ef_*.js）；家族表加 js 欄；families golden 改讀 testdata 快照 |

`tools/fixture-families.mjs` **還活著**（example-fixture 讀 FAMILY 表）— `TestUnitFixtureFamiliesGolden` 釘住 Go 表與 JS 表不漂移。`src/docs/transforms.mjs` **還活著**（overlay 讀 TEXT_ADJUSTMENTS）。

### 移植時學到的（下一波會再踩）

- **順序就是介面**：JS 的 Map/Object 迭代序（cva 表→軸→值、bySlot、anchors、DEFAULT_CONTENT attrs）都會進到產出 bytes。Go 端用 `orderedCva`/`decodeOrderedObject`/`bySlotOrder`/`anchorsOrder`/`[]attrPair` 保持。**任何新 port 都要先問「JS 這邊是什麼順序」**。
- `encoding/json` 的 HTML escaping（`&`→`\u0026`）和 `JSON.stringify` 不同——用 `jsonString`（jsonorder.go）。
- 檔名序 vs 元件名序不同：`alert-dialog.json` < `alert.json`（`-` < `.`）。cssParts 用檔名序、頁面用元件名序——JS 兩種都用，Go 必須跟。
- **`git add -A` 之前先 `git status -- dist docs`**（這次真的吞了 92 個 fixture 頁，靠 example-fixture 重生成 + 與 `f0df8e6~1` 比對救回）。
- `fs-record` 的 wrap 必須帶原函式的屬性（`.native`）——vite 讀 `fs.realpathSync.native`。

### 剩餘的 `node` 命令（3 支）

- **Wave 4（AST 群）**：`src/converter/index.mjs`（babel TSX→IR；tsx scanner 已備好大半）、`gates/overlay.mjs`（parseTs + transforms.mjs + emitter index/css/skin + tags + rtl-lib——port 完這些 JS 檔全滅；順便做舊 §4.4 dissolved 算失敗）。⚠️ overlay 讀的是 JS 版 DEFAULT_CONTENT——與 Go 版 `default_content.go` 有漂移風險，port overlay 前別改任一邊
- `tools/unit-check.mjs`（殘餘 suites css/prepaint/converter/emitter/runtime/types——對應 JS 工具 port 時跟著搬）
- **`tools/contracts/run.mjs`（`npm run contracts`）**：import `contracts/oracle-build.mjs` + `oracle-lib.mjs`，這兩支因此還活著；run.mjs port 完即可刪（Go 側 `buildContractOracleGo` 已在 `example_fixture.go`）

`src/tags.mjs`、`src/docs/transforms.mjs`（overlay 讀）、`tools/oracle-lib.mjs` + `tools/contracts/oracle-build.mjs`（contracts runner 讀）**還活著**——是對應工具 port 時的下一批，別刪。

---

## 3. 陷阱（每一條都踩過）

- **runner 的紅色標記是 `✗` 不是 `❌`**；grep 錯的那個會把紅的一輪讀成綠的。
- **不要 `... | tail`**。管線會吃掉 exit code。
- **不要在有東西 serve 或驅動站台的時候重跑 `vitepress build`**（hashed asset 換掉 → ENOENT → 假 render error）。
- **改 `src/registry/ir/*.json` 當測試沒有用**：convert 會重新生成蓋掉。驗 IR→輸出用 `./build/pipeline` 子命令。
- `PIPELINE_PARALLEL=1` 才啟用 undeclared-WRITE 檢查；READ 檢查任何 -j 都跑。
- emit/demo 的 `produces` 從 `src/registry/tiers.json` 推導（`pipeline/produces.go`），不是 glob。
- markdown 的 html block 內部不解析 markdown——連結要寫 `<a>`。
- `rewriteLinks` 會把無法路由的 `/` 開頭連結變純文字——`/demos/…` 要在它之後注入。
- `has-[&gt;svg]:gap-x-2`（HTML-escaped class）tailwind scanner 讀不到，靠掃 IR——消費者面 bug（舊 §4.8）仍未處理。

記憶檔在 `~/.claude/projects/-home-newlix-github-at-least-shadless/memory/`。

---

## 4. 接下來要做的工作（按投報率）

### 4.0 push
70+ commits 沒推。`make` 已全綠，直接推。

### 4.1 oracle 字型問題（原樣保留，擋 Dagger 全綠）
`tools/oracle-lib.mjs` harness 無 CSS → chromium 量測值是預設字型的函數；Docker 下 635 檔有 27 個不同（tooltip/popover/hover-card 的 popper transform-origin 從渲染文字寬度算出）。兩條路：(a) oracle 不烤量測值進頁面 (b) 宣告字型集 + 重錄 22 頁 + baseline。不要用 .dagger/ apt 裝字型蓋過去。

### 4.2 Wave 3 chromium 群
模式：一個共享 `tools/browser-shell.mjs` 薄殼（goto/waitForFunction/evaluate/locator/keyboard，stdin/stdout JSON），Go 持有所有狀態/比對/報告。拿最小的 `demo-smoke`(99行) 試水溫，最大支 `example-fixture`(561行) 最後。path-parity port 時順便拔 css.mjs import。

### 4.4 runtime bug 群（這些會出貨到 npm；原樣保留）
dropdown/context-menu 共用 guard、popover 搶焦點、carousel window.__api、navigation-menu JS 內 Tailwind class、ESM kernel 跑兩次。

### 4.5 Wave 4 AST 群
converter（796行）用 `internal/tsx` + esbuild transpile 重寫；overlay 順便做 dissolved-算失敗（原 §4.4）。`tools/rtl-lib.mjs` 只剩 overlay 一個消費者，port 完即刪。

### 4.6 re-pin drill（原樣保留）
reproducible 在真 re-pin 必紅、分類器子字串比對、55 個 @radix-ui overrides 無人比對、EXEMPTIONS.md 不 --render。

### 4.7 其他（原樣保留）
converter 靜默空頁（forwardRef/cva alias/skin 巢狀）、§4.8 escaped-class 消費者 bug、VitePress 明暗 iframe、fence `text`→`html`、部署、重複實作 ×3、parity flaky[]、inline style 無 gate、radix-kernel 無原始碼、SECURITY.md 錯誤。

---

## 5. 舊交接裡已經不適用的東西

wireit、`gates/registry.mjs`、`gates/meta.mjs`、medium tier、`emit-smoke`、workflows、
`docs/site`、`docs-consistency` §1–3、`docs-links`、`docs-upstream`、**以及 2026-08-31 版的 §4.2（死 docs 模組——已刪）、§4.3（example-gate 結構性必綠——本輪全鏈跑時 gate 正常，未動）**。
`tools/build-demo.mjs`、`tools/resolve-skins.mjs`、`tools/rtl-dict.mjs`、`tools/build-rtl.mjs`、`tools/rtl-lib.mjs`（部分）、`tools/demo.mjs`、`tools/demo-lib.mjs`、`tools/docs-consistency.mjs` 都刪了——它們是 Go 了。
