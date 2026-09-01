# HANDOFF — shadless, 2026-09-02

給下一個 context 的交接。讀完這份 + `git log --oneline -15` 就能接著做。
repo: https://github.com/at-least/shadless

**先看這個**：`origin/main` 還停在 `6c5c630`，本地 `main` 領先 **92 個 commit，全部沒 push**。
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

`tools/fixture-families.mjs` **還活著**（example-fixture 讀 FAMILY 表）— `TestUnitFixtureFamiliesGolden` 釘住 Go 表與 JS 表不漂移。`src/docs/transforms.mjs` **還活著**（overlay 讀 TEXT_ADJUSTMENTS）。

### 移植時學到的（下一波會再踩）

- **順序就是介面**：JS 的 Map/Object 迭代序（cva 表→軸→值、bySlot、anchors、DEFAULT_CONTENT attrs）都會進到產出 bytes。Go 端用 `orderedCva`/`decodeOrderedObject`/`bySlotOrder`/`anchorsOrder`/`[]attrPair` 保持。**任何新 port 都要先問「JS 這邊是什麼順序」**。
- `encoding/json` 的 HTML escaping（`&`→`\u0026`）和 `JSON.stringify` 不同——用 `jsonString`（jsonorder.go）。
- 檔名序 vs 元件名序不同：`alert-dialog.json` < `alert.json`（`-` < `.`）。cssParts 用檔名序、頁面用元件名序——JS 兩種都用，Go 必須跟。
- **`git add -A` 之前先 `git status -- dist docs`**（這次真的吞了 92 個 fixture 頁，靠 example-fixture 重生成 + 與 `f0df8e6~1` 比對救回）。
- `fs-record` 的 wrap 必須帶原函式的屬性（`.native`）——vite 讀 `fs.realpathSync.native`。

### 剩餘的 `node` 命令（2 支）

- **`src/converter/index.mjs`（796 行，最後的 babel 依賴）**：方向已拍板、調查已完成——**見 §4.5，可直接開工**。port 完即可刪 `src/emitter/{index,css,skin}.mjs`、`src/tags.mjs`、`src/docs/transforms.mjs`、`tools/rtl-lib.mjs`
- `tools/unit-check.mjs`（殘餘 suites css/prepaint/converter/emitter/runtime/types——對應 JS 工具刪除時跟著搬進 Go 測試）
- **`tools/contracts/run.mjs`（`npm run contracts`）**：import `contracts/oracle-build.mjs` + `oracle-lib.mjs`，這兩支因此還活著；run.mjs port 完即可刪（Go 側 `buildContractOracleGo` 已在 `example_fixture.go`）

`src/tags.mjs`、`src/emitter/{index,css,skin}.mjs`、`src/docs/transforms.mjs`、`tools/rtl-lib.mjs`（converter port 完才刪；overlay 現在只從 converter/index.mjs + skin.mjs 用 jsSetLiteral 抽表）、`tools/oracle-lib.mjs` + `tools/contracts/oracle-build.mjs`（contracts runner 讀）**還活著**——別刪。

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
92+ commits 沒推。`make` 已全綠，直接推。

### 4.1 oracle 字型問題（原樣保留，擋 Dagger 全綠）
`tools/oracle-lib.mjs` harness 無 CSS → chromium 量測值是預設字型的函數；Docker 下 635 檔有 27 個不同（tooltip/popover/hover-card 的 popper transform-origin 從渲染文字寬度算出）。兩條路：(a) oracle 不烤量測值進頁面 (b) 宣告字型集 + 重錄 22 頁 + baseline。不要用 .dagger/ apt 裝字型蓋過去。

### 4.2 Wave 3 chromium 群 ✅（2026-09-02 完成，見 §2 表）
共享薄殼 `tools/browser-shell.mjs` + Go 主控的模式已驗證；八個閘門 + example-fixture + overlay 全部 port 完。

### 4.4 runtime bug 群（這些會出貨到 npm；原樣保留）
dropdown/context-menu 共用 guard、popover 搶焦點、carousel window.__api、navigation-menu JS 內 Tailwind class、ESM kernel 跑兩次。

### 4.5 Wave 4：converter port —— 調查已完成、方向已拍板，可直接開工（2026-09-02）

**使用者已選定路線「1」：esbuild Transform 先降級 TSX → JS，再對降級輸出做有限掃描。不用新 parser 依賴。**

**生態調查結論（已實測 import，別重查）**：沒有可直接 import 的純 Go TSX AST——
- esbuild Go API 只有 Transform/Build；AST 在 `internal/js_parser`/`js_ast`，被 Go internal 可見性規則擋住
- `microsoft/typescript-go`（v0.0.0-20260820 實測）：105 套件 104 個 internal/，只公開 `cmd/tsgo`；`internal/ast` import 被拒
- `dop251/goja`：純 Go ES6+ parser，**不吃 TS/JSX**
- tree-sitter bindings：要 cgo + grammar 動態庫

**設計核心**：esbuild `Transform`（loader tsx、`jsx: transform`＝classic、jsxFactory `React.createElement`、format esm、charset utf8、不 minify）把 JSX 降級成 `/* @__PURE__ */ React.createElement(tag, props, children…)`——**children 是位置參數**（比 automatic runtime 的 children-key 好掃）。對降級輸出掃所有 `React.createElement(` 出現處（字串/註解/regex-aware，PURE 註解會出現在 call 前），balanced-paren 取 args：`args[0]`=tag、`args[1]`=props（可能是 `null`）、`args[2:]`=children。`React.createElement(React.Fragment…)` 不是 element（babel JSXFragment 不進 walk），但其 children args 內的巢狀 call 要照走。

**語義對齊點（babel 原始 AST vs 降級輸出）**：
- JSXText 純空白被 esbuild 丟棄 ≡ `sketchChildren` 的 `value.trim()` 空判斷；entity 解碼差異無關（sketch 只記 `"text"`）
- `{expr}` 容器透明；`<C disabled>` → `disabled: true`；shorthand prop（`<Button variant>` → `variant`）→ Identifier，正是 `resolveCvaArgs` 的 paramDefaults 路徑；spread 是 props object 頂層 `...props`
- paramDefaults：掃 fn 簽名 `ident = "lit"`、括號深度 ≤1（深度 ≥2 的巢狀解構要拒絕 ≡ babel 只認 left-Identifier）
- tagVars：`const Comp = asChild ? Slot.Root : "span"` 降級後原樣保留；衝突要 throw
- `resolveCvaArgs` 保真細節：`table.defaults ??= {}` 會**突變跨檔共享的表**（REG.cvaByExport）→ Go 用指標 + cross flag；`def === undefined` 的判斷；defaults 值 `str(dv.value) ?? dv.value?.value`——非字串字面值取原值（數字/bool），undefined → **整個 key 省略**
- gate 1（字串數對帳）用 `internal/tsx.StringLiterals(原src)`（已驗證 562 檔對齊 babel）對 raw 雙引號 regex 數；gate 2/2b/3 是對原 src 的 regex，原樣 port

**IR JSON 形狀普查（已完成，61 檔）**：
- top keys 順序固定 `schema,source,name,tier,imports,icons,cva,components,conditionals,cvaRefs,tagHints`；`__meta` 序列化前剝除
- component 固定 `{fn,export,elements}`；element `{tag,slot,classes,spread,children}` ×502 + 帶 `attrs` ×23（attrs 在 children 後）；slot null ×163、spread true ×344
- conditionals：`child-cond {kind,fn,parent}` ×25；`class-cond {kind,fn,slot,then,else[,test]}` ×8（test=`{name,op,value,default?}`，鍵序如此）
- cvaRef：全 corpus 只有 pagination 一個 `{slot,ref,table,dyn,dynAxes,defaults}`——**table 是同物件內聯重複序列化**
- 無 `<ternary:` tag（dead path，防禦性保留即可）；IR 檔**無尾換行**

**序列化（byte-identical 的關鍵）**：JS 是 `JSON.stringify(out, null, 1)`——indent 一格、無尾換行、不做 HTML escaping。Go 用 `json.Encoder`：`SetEscapeHTML(false)` + `SetIndent("", " ")` + 砍掉 encoder 自加的換行。**所有物件鍵序 = JS 插入序**（cva 表/variants/variant 值/defaults 按宣告序、tagHints 按元素走訪序、attrs 按 props 序）——Go map 會排序，必須自製 ordered KV 型別（`jsonorder.go` 的 `jsonString` 既有）。

**驗收序列**：
1. 先做 carousel 單檔 byte-identical（最難樣本：ternary、跨檔 Button wrap、IconPlaceholder、param defaults）
2. 全 61 檔 `git status src/registry/ir` 乾淨
3. drift gates + PASS 行逐字：`convert: 61 IR files -> src/registry/ir` / `tier dist: {...}` / `conditionals total: N` / `PASS  convert (0 drift, tiers match, tagHints resolved)`（FAIL 行也要同形）
4. `make` 全綠（NConvert 無 mutations）→ 刪 `src/converter/index.mjs` → 檢查消費者後刪 `src/emitter/{index,css,skin}.mjs`、`src/tags.mjs`、`src/docs/transforms.mjs`、`tools/rtl-lib.mjs`

**周邊改動清單**：
- `pipeline/convert.go`（+掃描器，可借 `internal/tsx` 手法但其 type 不導出，需在 pipeline 內新寫純 JS 版 scanner）；main.go 加 `convert` verb
- tier 表（KERNEL/TRIVIAL/MEDIUM/LOGIC/EXPLICIT_EXTERNAL/KNOWN_ICONS）成 **Go var**；`ovLoadTierSets` 從 jsSetLiteral(converter/index.mjs) 改讀 Go 表
- nodes.go：NConvert `Run` 改 `{"./build/pipeline", "convert"}`（resolve-skins 已是分離步驟），Inputs 改 pipeline 檔
- `tools/unit/converter.mjs`（194 行）斷言搬進 Go test（tierOf/collectExportedNames/cvaTablesOf/resolveCvaArgs/classStrings/tagVarsOf/convertFile+buildTagHints 端到端/兩個 loud failure），unit-check suite 清單跟著改
- tagHints/normalizeTag/externalMemberTag/NAT 已在 `pipeline/tags.go`，直接用

### 4.6 re-pin drill（原樣保留）
reproducible 在真 re-pin 必紅、分類器子字串比對、55 個 @radix-ui overrides 無人比對、EXEMPTIONS.md 不 --render。

### 4.7 其他（原樣保留）
converter 靜默空頁（forwardRef/cva alias/skin 巢狀）、§4.8 escaped-class 消費者 bug、VitePress 明暗 iframe、fence `text`→`html`、部署、重複實作 ×3、parity flaky[]、inline style 無 gate、radix-kernel 無原始碼、SECURITY.md 錯誤。

---

## 5. 舊交接裡已經不適用的東西

wireit、`gates/registry.mjs`、`gates/meta.mjs`、medium tier、`emit-smoke`、workflows、
`docs/site`、`docs-consistency` §1–3、`docs-links`、`docs-upstream`、**以及 2026-08-31 版的 §4.2（死 docs 模組——已刪）、§4.3（example-gate 結構性必綠——本輪全鏈跑時 gate 正常，未動）**。
`tools/build-demo.mjs`、`tools/resolve-skins.mjs`、`tools/rtl-dict.mjs`、`tools/build-rtl.mjs`、`tools/rtl-lib.mjs`（部分）、`tools/demo.mjs`、`tools/demo-lib.mjs`、`tools/docs-consistency.mjs` 都刪了——它們是 Go 了。
