# HANDOFF — shadless, 2026-09-02

給下一個 context 的交接。讀完這份 + `git log --oneline -15` 就能接著做。
repo: https://github.com/at-least/shadless

**先看這個**：先前那 92+ commits 已經 push 過了——這份 HANDOFF 寫完後才推的，交接時 `origin/main` 已經追上。
本輪（contracts port）又疊了 **3 個新 commit 沒 push**（`20f1fb4`…`b4a79ba`）。
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
| `gates/overlay.mjs`（fc579c1） | `pipeline overlay --audit|--record|--tasks|--report` | audit 輸出 byte-identical（199 applied + 1 dissolved）；--record 排序正規化（hash 不變）；規則表讀 Go var 本體 + jsSetLiteral 抽 JS 表；defs 走 shell 不啟動 chromium |
| **`src/converter/index.mjs`（796 行，最後的 babel 依賴）** | `pipeline convert`（`pipeline/convert.go`，~1900 行） | 61 檔 IR **byte-identical**（`TestUnitConverterIrParity`：跑 convert + `git status src/registry/ir` 必須乾淨）；drift gates PASS 行逐字同 JS；`tools/unit/converter.mjs` 斷言搬進 `convert_test.go`（TestUnitConverter*） |
| **`tools/contracts/run.mjs`（341 行，最後的 node 命令）+ `tools/contracts/oracle-build.mjs`** | `pipeline contract <name>` / `pipeline contracts`（`pipeline/contract.go`） | 29 個 contract defs 全部 **byte-identical**（stdout、`result.json`、`shadless.html`）；`browser-shell.mjs` 新增 `addScriptTag`/`focus`/`wheel`/單元素 `locEval` op，`loadContractDef` 補 `closeSelector`/`overlaySlot`/`contentSlot` 投影；`contracts-strip-glue` mutation 仍紅、`make only ID=contracts:*` 全綠；`tools/oracle-lib.mjs` 因此無人 import，一併刪除 |

`tools/fixture-families.mjs` **還活著**（example-fixture 讀 FAMILY 表）— `TestUnitFixtureFamiliesGolden` 釘住 Go 表與 JS 表不漂移。`src/docs/transforms.mjs` **還活著**（overlay 讀 TEXT_ADJUSTMENTS）。

### 移植時學到的（下一波會再踩）

- **順序就是介面**：JS 的 Map/Object 迭代序（cva 表→軸→值、bySlot、anchors、DEFAULT_CONTENT attrs）都會進到產出 bytes。Go 端用 `orderedCva`/`decodeOrderedObject`/`bySlotOrder`/`anchorsOrder`/`[]attrPair` 保持。**任何新 port 都要先問「JS 這邊是什麼順序」**。
- `encoding/json` 的 HTML escaping（`&`→`\u0026`）和 `JSON.stringify` 不同——用 `jsonString`（jsonorder.go）。
- 檔名序 vs 元件名序不同：`alert-dialog.json` < `alert.json`（`-` < `.`）。cssParts 用檔名序、頁面用元件名序——JS 兩種都用，Go 必須跟。
- **`git add -A` 之前先 `git status -- dist docs`**（這次真的吞了 92 個 fixture 頁，靠 example-fixture 重生成 + 與 `f0df8e6~1` 比對救回）。
- `fs-record` 的 wrap 必須帶原函式的屬性（`.native`）——vite 讀 `fs.realpathSync.native`。
- **converter port（2026-09-02）**：路線是 esbuild Transform 降級 TSX → 掃 `React.createElement(` 輸出。關鍵坑：(1) children 的 "text"/"expr"/"OPT?" 要靠**掃原始碼的 JSX children 分類器**按元素出現序與降級側配對（`{" "}` 和 JSXText 降級後都是字串參數、`{/* comment */}` 容器整個消失）；(2) fragment `<>` 兩側都不算元素；(3) `cvFirstTop` 的 hit 一定要卡 `depth==0`（否則 props 物件裡的 `?` 被當根運算子，整個 props 解析成 cond）；(4) 關鍵字後讀下一個詞要跳空白（`cvNextWord`）；(5) upstream 原始碼**沒有分號**——cvSkipStmt 要在 depth-0 換行停，否則一條 import 吞整檔；(6) gate 1 的字串數要排除 Template span（`String.raw` 標籤模板 babel 不算）；(7) `export default function D` 被 esbuild 改寫成 `export { D as default }`，as-default 註冊的是宣告名。
- **contracts port（2026-09-02）**：(1) `toggle-group.mjs` 的 scenarios 陣列裡有一個裸逗號（真正的陣列 hole），JS 的 spread/`for-of` 會把它讀成字面 `undefined`（falsy，run loop 跟 null-step 走同一條路，never 寫進 `oracleS`/`shadlessS`），但這個 hole 過 browser-shell 的 JSON 線只會變成 `null`→Go 的 `""`，跟一個「真的空字串 scenario」完全無法區分——得另外用一個 `scenarioRan` 存在性表，才能重現輸出裡那行 `undefined: oracle=undefined shadless=undefined` 和 `result.json` 裡缺的那個 key。(2) scenario 診斷行的樣板 `` `oracle=${o} shadless=${s} ${same?"":"DIFF"}` `` 在「相同」的情況下**還是有一個字面空白**在行尾（`${same?"":"DIFF"}` 前面那個空格是固定的，不是條件式的一部分）——移植成 `Printf` 時很容易把這個 trailing space 弄丟，A/B diff 才抓到。

### 剩餘的 `node` 命令：**0 支** ✅（2026-09-02 完成，contracts port，見上表）

unit-check 的 suites 剩 css/prepaint/emitter/runtime/types（converter suite 已隨 port 刪除、斷言在 `pipeline/convert_test.go`）。

`src/tags.mjs`、`src/emitter/{index,css,skin}.mjs`、`src/docs/transforms.mjs`、`tools/rtl-lib.mjs`（**還活著**——Go golden 測試直接 import JS 版比對：`emitter_css_test.go`、`tools/unit/{css,emitter}.mjs`；skin.mjs 另有 ledger/overlay 的 jsSetLiteral 讀者）**別刪**。`tools/oracle-lib.mjs` 和 `tools/contracts/oracle-build.mjs` 隨 contracts port 一起刪了——`buildContractOracleGo`（`example_fixture.go`）已經是兩邊唯一的 oracle bundle 建造者。tier 表/KNOWN_ICONS 已是 Go var（`convert.go cvTierSets/cvKnownIcons`），overlay 的 audit 單位 id 用 "trivial"（tierOf 回傳 "trivial-js"，`ovLoadTierSets` 負責映射）。`.dagger/main.go` 的 `converted`/`Contract`/`Contracts` 都已改跑 Go binary（resolve-skins + convert；contract 讀 `goBinary()`）。

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
本輪（contracts port）3 個新 commit 沒推。`make` 已全綠，直接推。

### 4.1 oracle 字型問題（原樣保留，擋 Dagger 全綠）
`pipeline/oracle_lib.go`（原 `tools/oracle-lib.mjs`，已刪）的 harness 無 CSS → chromium 量測值是預設字型的函數；Docker 下 635 檔有 27 個不同（tooltip/popover/hover-card 的 popper transform-origin 從渲染文字寬度算出）。兩條路：(a) oracle 不烤量測值進頁面 (b) 宣告字型集 + 重錄 22 頁 + baseline。不要用 .dagger/ apt 裝字型蓋過去。

### 4.2 Wave 3 chromium 群 ✅（2026-09-02 完成，見 §2 表）
共享薄殼 `tools/browser-shell.mjs` + Go 主控的模式已驗證；八個閘門 + example-fixture + overlay 全部 port 完。

### 4.4 runtime bug 群（這些會出貨到 npm；原樣保留）
dropdown/context-menu 共用 guard、popover 搶焦點、carousel window.__api、navigation-menu JS 內 Tailwind class、ESM kernel 跑兩次。

### 4.5 Wave 4：converter port ✅（2026-09-02 完成，見 §2 表）

路線「1」（esbuild Transform 先降級 TSX → JS，再對降級輸出做有限掃描，不用新 parser 依賴）已實作在 `pipeline/convert.go`。61 檔 byte-identical、drift gates 逐字、`make` 全綠後刪了 `src/converter/index.mjs` + `tools/unit/converter.mjs`。

**設計核心（已落地）**：esbuild `Transform`（loader tsx、`jsx: transform`＝classic、jsxFactory `React.createElement`、format esm、charset utf8、不 minify）把 JSX 降級成 `/* @__PURE__ */ React.createElement(tag, props, children…)`。對降級輸出掃所有 call（字串/註解/regex-aware），balanced-paren 取 args；`React.Fragment` call 不算元素但其 children 照走。**children 的 "text"/"expr"/"{ident}"/"OPT?" 來自掃原始碼的 JSX children 分類器**（`scanJsxKinds`），兩側按元素 document order 配對（數量不等就 loud fail）——這是降級路線唯一拿不到的資訊（`{" "}` vs JSXText 降級後同形、`{/* comment */}` 容器整個消失）。

**語義對齊點（babel 原始 AST vs 降級輸出，皆已落地）**：
- JSXText 純空白被 esbuild 丟棄 ≡ `sketchChildren` 的 `value.trim()` 空判斷；entity 解碼差異無關（sketch 只記 `"text"`）
- `<C disabled>` → `disabled: true`，與 babel 的 null-value 走同一條 fall-through；spread 是 props object 頂層 `...props`
- paramDefaults：掃 fn 簽名 `ident = "lit"`、深度 ≤1（巢狀解構不取 ≡ babel 只認 left-Identifier）
- tagVars：`const Comp = asChild ? Slot.Root : "span"` 降級後原樣保留；衝突要 throw
- `resolveCvaArgs` 保真細節：defaults 突變用指標 + cross flag（只有 `!cross` 才寫回）；`def === undefined` 的判斷；defaults 值 `str(dv.value) ?? dv.value?.value`——非字串字面值取原值（數字/bool 以 `jsonRaw`/bool 存），undefined → **整個 key 省略**
- gate 1 = `internal/tsx.StringLiterals(原src)`（**排除 Template span**——`String.raw` 標籤模板 babel 不算）+ DirectiveSpans（「use client」41 檔；babel 的 gate 有數 DirectiveLiteral）對 raw 雙引號 regex 數；gate 2/2b/3 是對原 src 的 regex，原樣 port

**序列化**：`marshalJSStep(out, "", " ")`（= `JSON.stringify(out, null, 1)`，無尾換行）；鍵序全用插入序（jsonObj）。IR 檔案級的保證由 `TestUnitConverterIrParity` 釘住。

**周邊改動（全數完成）**：`pipeline/convert.go` + `main.go convert` verb；tier 表成 Go var（`cvTierSets`/`cvKnownIcons`）；`ovLoadTierSets` 改讀 Go 表（audit id 用 "trivial"）；nodes.go NConvert Run/Inputs 改 pipeline 檔、NOverlay inputs 移除 converter/index.mjs；unit 斷言搬 `convert_test.go`、unit-check suites 移除 converter；package.json `convert` script 改 Go；`.dagger/main.go` 的 `converted` 改 Go binary；overlay/audit_boundary 的 home/tool 標籤改 `pipeline/convert.go`。

### 4.6 re-pin drill（原樣保留）
reproducible 在真 re-pin 必紅、分類器子字串比對、55 個 @radix-ui overrides 無人比對、EXEMPTIONS.md 不 --render。

### 4.7 其他（原樣保留）
converter 靜默空頁（forwardRef/cva alias/skin 巢狀）、§4.8 escaped-class 消費者 bug、VitePress 明暗 iframe、fence `text`→`html`、部署、重複實作 ×3、parity flaky[]、inline style 無 gate、radix-kernel 無原始碼、SECURITY.md 錯誤。

---

## 5. 舊交接裡已經不適用的東西

wireit、`gates/registry.mjs`、`gates/meta.mjs`、medium tier、`emit-smoke`、workflows、
`docs/site`、`docs-consistency` §1–3、`docs-links`、`docs-upstream`、**以及 2026-08-31 版的 §4.2（死 docs 模組——已刪）、§4.3（example-gate 結構性必綠——本輪全鏈跑時 gate 正常，未動）**。
`tools/build-demo.mjs`、`tools/resolve-skins.mjs`、`tools/rtl-dict.mjs`、`tools/build-rtl.mjs`、`tools/rtl-lib.mjs`（部分）、`tools/demo.mjs`、`tools/demo-lib.mjs`、`tools/docs-consistency.mjs`、`src/converter/index.mjs`、`tools/unit/converter.mjs`、`tools/contracts/run.mjs`、`tools/contracts/oracle-build.mjs`、`tools/oracle-lib.mjs` 都刪了——它們是 Go 了。
