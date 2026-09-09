# shadless-rs 移植進度

對照 PLAN.md 的里程碑。驗收原則:每個里程碑都要能對 Go binary 做出可判別的 diff。

## 已完成

- **M0 esbuild probe**(probe/m0/):pinned CLI(0.28.2)對 Go `api.Transform`/minify
  全部 61 檔 + minify 位元組一致;`dist/shadless.min.js` 封環一致;錯誤路徑契約確認。
- **M1 graph 層**:`nodes.rs`(41 nodes 逐欄位)、`fanout.rs`(contracts 29 shards)、
  `produces.rs`(tiers.json 推導)、`glob.rs`(Go regexp 語意的 glob)、`key.rs`
  (sha256 摺疊,欄位位元組佈局逐字)、`stamps.rs`、`graph.rs`(宣告序 DFS 後序)、
  `main.rs`(plan/list/status/inputs + 錯誤路徑)。
  驗證:tests/gen_golden.sh 四層測試台 layers 1–3 全綠;key 69 節點逐位元組一致
  (oracle = probe/keys-go 的 goprobe,用真 Go keyer 原始碼編譯)。
- **M2 runner**:`runner.rs`(dispatch-on-ready 主執行緒排程、worker threads、
  jobs/browser_jobs 兩個計數信號量、stamp-on-success/forget-on-failure、
  --keep-going 的 run-report.json)、`verify.rs`(未宣告讀寫稽查、testlog 與
  fs-record 解析)、main 的 `run`/`adopt`。
  修過的 bug:`sem.release()` 一度寫成 `drop(Arc)`,導致單槽信號量死鎖——
  用 sem 內部插樁定位(證據:release trace 從未出現)。
  發現並記錄:Go 原版在 -j1 的執行順序本身就是不確定的(dispatch 不擋,
  goroutine 競爭 sem),因此層 4 比較採「正規化計時 + 行集合排序後 diff」。
  驗證:harness 四層 **338/338 全綠**,含 fixture 上由 GO key 寫 stamp 的可判別
  fresh-skip 情境、--force 確定性失敗、失敗節點 stamp 移除、blocked 會計、
  run-report.json 與 Go 位元組一致(632 bytes)。
- **cargo test**:27 單元測試 + tests/golden.rs 重播 327 案例全綠。

## 測試台(tests/gen_golden.sh)四層

1. CLI 矩陣(真樹):plan/list/status/inputs × 全目標/旗標/錯誤路徑,
   stdout+stderr+exit code 三比;goldens 存 tests/golden/ 供重播。
2. key 對照:goprobe vs `__keys`,真樹 69 節點。
3. 可判別 fixture:GO key 寫 stamp → 只有序件 key 逐位元組正確才會同報 fresh。
4. runner 語意:fresh-skip / --force / stamp-on-failure / blocked / run-report。

## 里程碑狀態

- [x] M0 esbuild probe
- [x] M1 scaffold + graph/key/CLI
- [x] M2 runner + verify + run/adopt
- [x] M3 internal/twmerge + internal/tsx(twmerge 555 snapshot 案例一致;tsx 562 檔 babel conformance 一致;regex \w\d\s\b 已 ASCII 化)
- [x] M3.5 api.Build probe gate(2026-09-08 以替代路徑結案:probe gate 未單獨建,oracle 鏈直接以端到端驗收——example-oracle --check 227 頁 PASS、example-golden 296==snapshot、contract 29 契約雙邊一致;見下方 Oracle 鏈條目)
- [x] M4 convert.go → IR(**61/61 byte-identical**,2026-09-07;全部 4 個 drift gates PASS、tier dist/conditionals 與 Go 一致)
- [x] M5(完成)jsbuild + emit 鏈(2026-09-07);demo/rtl/product_css/oracle/docs 鏈 2026-09-08 全數移植並雙邊驗證(見下方「剩餘 18 指令全部移植」等條目)
- [x] M6 gates 24 個(2026-09-08 完成:11 個真樹 gates 為 #[test]+CLI、全部 gate 函式移植並雙邊 verdict 驗證;2026-09-09 gate_parity 24 gates 雙引擎 verdict 對照)
- [x] M7 自我接管(2026-09-09:預設圖零 spawn Go;`SHADLESS_GRAPH=go-mirror` 保留 Go 對照模式;gate_parity 抓出並修復 4 缺陷;詳見下方「自我接管」與 PLAN.md 同名節)

## M3 筆記(2026-09-07)

- src/twmerge/mod.rs:regex 已逐條 ASCII 化(`\w`→[0-9A-Za-z_]、`\b`→(?-u:\b)、`\s`→[\t\n\f\r ]);config.json/snapshot.json 內嵌自 internal/twmerge。
- src/tsx/mod.rs:spans-snapshot 562 檔全對齊 babel;唯一容忍分歧 aria/message-scroller-streaming.tsx(+1 prose backtick)與 Go 測試同款鏡射。
- 手測案例教訓:`data-x={\`code\`}` 的 template 是 code-context literal(2 spans 不是 1);temp 目錄名不可含括號(sh -c 會 exit 2)。

## M4 前置條件(advisor checkpoint 2026-09-07,M3 已接受)

1. **Oracle 決定性先驗證**:動 Go convert 兩次、byte-compare generated/ir——若 Go 本身
   有 map 迭代不確定性漏進輸出,60+ 檔的 gate 會先天不穩。
2. **先移植 jsonorder.go 並獨立單元測試**(不要直接啃 convert.go):測試 fixture 要含
   `<` `>` `&`(Go 預設 JSON escape 成 \u003c 等;Tailwind arbitrary variants 如
   [&>div]:p-4 會踩到)、U+2028/U+2029、浮點格式(Go strconv vs ryu 的指數門檻)、
   key 順序(serde 需 preserve_order/IndexMap 語意,jsonorder 移植本身就是手寫序列化器)。
3. **M4 分段移植 + 先建 per-file diff harness**(first-byte-divergence + 上下文,
   同 gen_golden.sh 的模式),失敗檔案要能秒級自定位。
4. Go map 餵 sort 的地方:mirror 精確 sort key 與 tie-breaker,不是只「有排」。

驗收遞進:每段 probe gate 綠 → 61/61 IR byte-identity → 全 harness 重跑確認無 M0–M3 退化。

## M4 完成筆記(2026-09-07)

- src/convert/{mod,scan,jsx,topscan,cva}.rs,約 2800 行;esbuild 走 probe/m0 驗證過的
  CLI spawn(六個 flags);IR 寫出用 jsonorder(marshal_js_step step=" ")。
- 已修的移植 bug:cv_unquote 多位元組(以 Vec<u8> 逐位元組重寫)、
  cv_assign_pattern_default 遺漏 eq<0 早退、cross-file table 變異走 local name。
- 驗收:tests/ir_diff.sh —— 61/61 byte-identical;harness 469/469 無退化。
- ir_diff.sh 的設計:跑前快照+git 乾淨檢查、per-file 首位元組分歧診斷、跑後還原。

## M5 筆記(2026-09-07)

- src/jsbuild.rs：iifeBase/esmBase/esmComponent 字串組裝逐字；minify spawn
  `esbuild --minify --target=es2017`（M0 已證與 api.Transform 同位元組）。
- src/emit/{tags,prepaint,htmlutil,default_content,css}.rs + mod.rs（buildTree/
  renderTree/renderFn/resolveDefault/mergeRootAttrs/validate/runEmit 全 gates）。
  HTML gate 用 html5ever+markup5ever_rcdom（x/net/html 的 html5 對應物）。
- serde 教訓：Go json.Unmarshal 對 null 容忍（欄位保持零值）→ emit 端先 drop_nulls
  再 deserialize；欄位 rename 全表：fn/tagHints/cvaRefs/ref/dyn/dynAxes/else。
- default_content.rs 是**生成檔**（生成器在本 session 對話中，必要時重寫；
  96 entries/23 components，attrs/children/quote 邊界都已對齊）。
- emit 已知錯誤路徑差異：skin 檔缺失時 Rust panic（Go 印 stderr + exit 1）——僅影響壞樹。

## M5 延伸完成筆記(2026-09-07)

- src/emit/demo.rs:rewrite_paths 五 regex、ensureLink、fileOrder(css 迭代用 ReadDir
  檔名序,與排序後的 names 不同!)、orphan dist/css 清理、globals.css 組裝、
  per-tier fixture 分派、demo index。**dist_diff demo:268/268 byte-identical**。
- src/emit/build_rtl.rs:substitute_and_patch(最長值優先、lang/dir patch)、
  manifest 用 BTreeMap(Go map marshal 排序)。**87 rtl 變體 + manifest 全一致**。
- src/emit/product_css.rs:extractTokens keep-list、tw-animate 內聯、verifyProduct、
  gateProductVerify(--verify 邏輯已備 M6 接線)。**dist 乾淨 = 與 committed 一致**。
- src/emit/tw.rs:find_repo_root(SHADLESS_ROOT + nodes.go walk)、scratch-dir 語意
  (空 cwd = 零內容掃描)。**A/B:out.css 564KB / full.css 345,892B / full.min.css
  290,235B 全部 byte-identical**。
- 已知錯誤路徑差異:skin 檔缺失時 load_skin panic(101)vs Go exit 1——僅壞樹。
- 暫態陷阱:dist 被工具直跑弄髒時 golden 重播會誤報——先 git checkout dist/ 再跑。

## Oracle 鏈進度(2026-09-07,本輪)

- src/oracle/browser_shell.rs:JSON-line 協定驅動全移植(launch/newPage/evaluate/
  loc* / driver / keyPress 全 ops)。**關鍵修復:close() 必須 take() stdin 讓 node
  看到 EOF(Go: stdin.Close()),否則 wait() 永久掛起**——症狀是整跑「hang」但其實
  工作早已完成、卡在行程收尾。
- src/oracle/oracle_lib.rs:invariant 快取鍵(逐位元組同 Go:pin commit+lockfile+
  resolve_skins.go+skin.mjs+oracle_lib.go+stubs 排序)+ per-example tsx;alias 表;
  esbuild Build 經 CLI(--bundle --format=iife --loader:.tsx=tsx --jsx=automatic +
  重複 --alias:k=v);oracleNorm(radix auto-id 穩定化);canon/oracle_root_html。
  rel 路徑需自寫 rel_path(Go filepath.Rel;用絕對路徑先正規化)。
- src/oracle/example_oracle.rs:**--check gate PASS(227 pages == oracle render,
  ~31s warm)**;manifest 寫出格式逐字(\n {\n "name":...);protocol patch 單錨點。
- src/oracle/example_golden.rs:**gate PASS(296 == upstream snapshot,147 exempt)**;
  classify/diff 模式皆移植;serde preserve_order 取代 orderedJSONKeys。
- Rust match arm 語法:let x = match {...} 要分號;GO 不用。

## 剩餘(誠實)

- oracle 鏈:example_fixture.go(1238 行,最大檔)、contract.go(985 行)、oracle
  的 --check 以外 ops 尚未逐一 golden。
- M6:24 gates → #[test](product-verify 的 gate_product_verify 已備妥待接線;
  pin/pack/dist-complete/coverage/reproducible/css-direction/script-refs/
  consumer-sim/ledger/audit/mutations 等)。

## example-fixture 現況(2026-09-07,未完成)

已移植:src/oracle/example_fixture.rs + fixture_families.rs + families.rs + 5 個內嵌 JS。
生成層已驗證:tooltip-disabled 頁與 Go 版**位元組一致**;alert-dialog-basic 頁 body
與 Go 版一致到 template 結尾。

**未解**:26 頁的 self-test 執行失敗(portal 開啟、menu 子選單、select click timeout)。
證據鏈:點擊後 500ms 檢查「未開」,但探針(數百 ms 後)顯示 content 已掛載 + radix
focus-guard 已出現(= 對話框確實開了)——掛載晚於 500ms 檢查窗;3 秒輪詢仍逾時,
顯示掛載遠晚於輪詢窗口,疑與 self-test 當下頁面狀態有關(同一頁面手動導航測試正常)。
Go 同時序(400/500ms)在同機器一致通過。

**下一步建議**:以 python shell 驅動複製「runner 完整前序(goto oracle → click →
harvest → goto scratch)」並在 self-test 點擊前後取樣 DOM;或比對 efRemap 後
模板內 radix id 與 trigger id 的對應是否與 Go 版一致(手動測試顯示 glue 有回應,
故頁面結構無誤,疑點在 id 對應或事件時序)。

## M6 進度(2026-09-07)

已移植為 #[test] 的 gates(5/24):dist-complete、reproducible、css-direction(含 baseline)、
product-verify(用 emit::product_css::verify_product)、script-refs(Makefile +
package.json + nodes.go -run 模式全檢)。全部在真樹上通過(src/gates/mod.rs)。

example-fixture 除錯結論(2026-09-07):**生成層完全正確**(失敗頁與 committed
位元組一致,tooltip-disabled/popover-basic/dropdown-menu-submenu 等全驗證)。
self-test 執行層尚有 26 頁失敗:portal hover/click 開啟、menu 子選單 hover、
select disabled click——點擊後 500ms 檢查「未開」但探針顯示隨後已掛載(慢掛載),
且 menu 子選單 hover 在 runner 內不開、standalone 正常。疑點:runner 的 playwright
page 前序狀態(oracle 頁交互後)影響後續頁面的 actionability。下一步:對照 Go/RS
runner 在同一頁的 events() 前後差異,或加大等待窗驗證語意。

## example-fixture self-test 除錯結論(2026-09-08)

**已修(失敗 26 → 12)**:
1. Portal self-test 的**重複 loc_click**(hover 開啟後又點擊 = 關閉)——移除後
   popover/tooltip/hover-card 的 click-portal 頁全數通過。
2. EF_NAV_IDS 的 arg:Go 傳裸字串 selector,誤包 json!({"sel": ...}) → 導致
   navigation-menu-demo 的 trigger id 未被改名、click timeout。
3. MenuOrSelect 空 trigger(全 disabled)需 return Ok(Go firstEnabled<0 → nil)。

**剩餘 12 頁失敗(生成已證實位元組一致)**:
- menu 子選單 10 頁(dropdown/context/menubar + button-group-demo 等):self-test
  中 hover 子觸發器後「sub menu did not open」;生成時同樣的 hover 有開。
- portal 2 頁(hover-card-rtl/sides)、navigation-menu-demo、select-disabled:
  locator.click timeout。
- 共同點:全部是 runner 長流程(browser 累積狀態)中的後段頁面;同頁面 standalone
  導航測試正常(tooltip-disabled 已證)。疑點指向 runner page 的殘留狀態
  (mouse 位置/route/events capture)或 Go/RS 對 playwright 動作的細微差異。
- 下一步:EF_KEEP=1 全跑後對 navigation-menu-demo 的 scratch 頁做 DevTools 級
  檢查(getComputedStyle、elementFromPoint 於點擊座標),確認是否有覆蓋元素。

## submenu 除錯深入(2026-09-08)

EF_DEBUG 探針(已加入 fixture_families.rs 的 menu_select submenu hover 後)顯示:
**`subMounted: true`、`onTop: DIV[X-sub-trigger]`** —— hover 落點正確、子選單已掛載,
但緊接的 `sub_v` 檢查(同一 selector)仍回 false → 「sub menu did not open」。
悖論:同一 selector、連續兩次查詢,第一次 true 第二次 false。

疑點:probe 與 check 之間有 `evaluate_fn_arg` 的 JSON round-trip;或 self-test 與
harvest 的差異在於 self-test 對**已開啟的 menu 再點擊主觸發器**(runner 流程)造成
radix 關閉/重開的競態。Go runner 同序通過,故差異在 RS 端 evaluate_fn_arg 的
JSON 序列化或時序。下一步:將 sub_v 檢查改為 wait_true 輪詢(已證實對 dialog
開啟檢查有效),逐家修改;或以 DevTools 斷點追 evaluate_fn_arg 的實際回傳值。

## submenu 除錯第二輪(2026-09-08)

- EF_NAV_IDS arg 修正(裸字串)後,「sub menu did not open」的 polling 檢查全數通過;
  失敗頁 12 → 導航選單 1 頁(navigation-menu-demo click timeout)。
- python 複製 runner 精確序列(oracle 頁 → click 觸發 → Escape×4+mouse(0,0) →
  goto scratch → click #n0-trigger)→ **全部通過**(shadless 載入、content 掛載)。
- 隔離測試(30 次迴圈導航後點擊)也通過。**結論:失敗只發生在 runner 長流程中,
  為累積的瀏覽器/page 狀態**(約第 40+ 次導航後),單頁隔離無法重現。
- 下一步:在 runner 中每 N 頁重啟 browser shell,或以 elementFromPoint/DevTools
  於失敗當下取樣;或比對 Go runner 同流程的失敗集(若 Go 也失敗則為環境)。

## submenu 輪詢結論(2026-09-08 第二輪)

子選單開啟檢查已改 wait_true 3 秒輪詢——**仍失敗**:runner 流程中 hover 子觸發器
600ms + 3 秒輪詢後子選單仍未掛載;但同一頁面 standalone 手動 hover(python 驅動)
600-800ms 即開啟。排除時序;指向 runner 長流程的 page 狀態(前序頁面的 radix
選單以真實 locClick 開啟,滑鼠位於主觸發器上;goto 後 playwright 滑鼠殘留位置、
或 route/events 攔截層與 hover 的互動)。

下一步(未完成):1) 在 runner 的 menu self-test 中以 evaluate 的
elementFromPoint + getBoundingClientRect 取樣 hover 座標(已加 EF_DEBUG 探針
於 fixture_families.rs submenu 檢查前);2) 對照 Go runner 同頁同座標的行為;
3) 若 Go 亦失敗則為環境,若 Go 通過則比對 playwright 滑鼠事件序列
(真實 locClick 開啟 vs 手動 synthetic 開啟)。

## example-fixture 完成(2026-09-08):105/105 頁 self-test 全數通過

**根因**:Nav action 儲存的 trigger 已含 "-trigger" 後綴,self-test 又拼一次
→ `#n0-trigger-trigger` 不存在 → locBox/click 30 秒逾時。修復:儲存裸 nav_ids[0]。
(同輪修正:portal self-test 的重複 loc_click、EF_NAV_IDS/EF_NAV_IDS 類 arg 包裝、
MenuOrSelect 空 trigger 跳過、nav click 改真實滑鼠事件。)
驗收:rs example-fixture **PASS 105 頁**,docs/ 乾淨(生成與 committed 一致)。

## 給下一個 session 的筆記

- 重跑全部驗證:`./tests/gen_golden.sh && cargo test`(需要 ../shadless 樹與其 module cache)。
- goprobe 重建:probe/keys-go/setup.sh(複製 pipeline 原始碼、拔 main、接 keys 印表機)。
- Rust 端隱藏子命令 `__keys` 是測試台專用,不對外文件化。
- serde_json 對生成檔(JSON IR)不可用——M4 必須移植 jsonorder.go 的手寫序列化器
  (jsonRaw 原樣數字、absent-vs-null、無 HTML escape)。
- 浮點/時間格式只在 runner 摘要行出現,測試台已正規化。
- Go regexp 的 `\w`/`\b` 是 ASCII,Rust regex 是 Unicode——M4 掃描器移植時逐條 audit。
- walkdir 列舉後必須顯式排序(Go filepath.Walk 是字典序)。
- harness 的 layer-4 比較是「排序後集合 diff」——Go 原版執行順序在本來就不確定。

## contract.go + 剩餘 M6 gates(2026-09-08,本輪)

### contract.go → src/oracle/contract.rs(985 行)
- cdef(serde rename)/cLoadDef(shell op loadContractDef,drop_nulls 容 Go null)/
  cRecorderSrc/cRewriteRelativePaths/cNormVal(Go ASCII \w\d 明示)/
  cBuildAttrObj+cBuildFact(serde preserve_order 取代 decodeOrderedObject)/
  cRawToJsonable→jsonorder 樹/cStepIt op 鏈(ParseFloat 失敗→0,同 Go 捨棄錯誤)/
  cOracleRun/cShadlessRun/runContract(flaky 雙側重跑、scenario hole 印 undefined、
  mounted-DOM 前 4 條截斷、result.json 經 marshal_js)/runContractsAll(子行程逐契約)。
- build_contract_oracle 補上 recorder 參數實際注入 entry(原寫死 "")並改用
  oracle_cache_dir(SHADLESS_CACHE 生效)。
- **驗收:`contracts` 全量 29 契約 Go vs RS stdout 位元組一致、result.json 全同、
  退出碼全同;突變週期(contracts-strip-glue)雙方同輸出同 FAIL(exit 1)。**

### M6 gates → src/gates/{pin,pack,coverage,ledger,consumer_sim,audit_boundary,mutations}.rs
- pin(run_pin --check-only/--force)、pack(npm pack --dry-run --json)、
  coverage+coverageBudget(900 格矩陣;coverage.json 經 marshal_js_step step=" "
  對 Go MarshalIndent(...," ") 位元組一致 169,702B)、
  consumer_sim(tempdir+雙 symlink+逐組件 tailwind 編譯)、
  ledger+jssource(read/write 位元組形、source-id 推導、ratchet、record/dissolve/
  render)、audit_boundary(patterns OnceLock+re_cache、manifest ownership、
  drift/discover/--strict)、mutations.go+meta.go(30 mutations、snapshot、
  meta_wiring/select_mutations/run_gate、AuthoredGraph 而非 fanout 展開圖)。
- CLI 接線:pin/coverage/ledger/audit-boundary(coverage 無 --record 時 exit 2,
  訊息改指 cargo test — 已知刻意偏差)。
- **雙側驗證(皆與 Go 二進位逐位元組比較)**:coverage.json、audit-boundary --strict
  JSON、pin --check-only、ledger --record/coverage --record/ledger --dissolve 寫出檔、
  ledger --render(19,066B)、pin drift FAIL、contract FAIL;cargo test 85+1 綠
  (真樹 gates:pin/pack/coverage/consumer-sim/ledger)。
- 全程回歸:ir_diff 61/61、dist_diff 268/268 ×3、gen_golden 469/469。

### 已知刻意偏差(記錄)
1. Rust String 無法表達 Go truncate 的 mid-UTF-8 位元組截斷→退到字元邊界
   (mounted-diff 傾印行,僅多位元組 class 字串第 160 位元組落在字元中間才不同)。
2. Go 對畸形 clickAt:/wheel: step 會 index panic;RS 回幾淨錯誤或 0 預設。
3. coverage/ledger CLI 的「gate 是測試」訊息指向 cargo test 而非 go test。
4. in_manifest 找 repo root:cwd 向上找尋→SHADLESS_ROOT→(僅測試)crate 鄰位。
5. ledger --record 會把 committed ledger.json 重排為寫入器形狀 — Go 同款行為
   (雙側寫出位元組一致已證);本次實驗中誤跑一次已 git checkout 還原。

### 環境註記
- shadless Go repo 的 pipeline/pipeline 二進位已用其 committed source 重建
  (原二進位過舊、無 contract 指令)→ 該 repo `git status` 顯示此檔 M。

## example-fixture self-test 收斂(2026-09-08,本輪)

- RS `example-fixture --check`:**PASS 105/105**(「105 interactive pages ==
  committed, open/close self-verified」,exit 0)——生成層、committed 比對、
  open/close 自檢、api 句柄檢查、page-error 檢查全綠。
- Go 二進位同旗標同樹:PASS 105/105,輸出與 RS 一致(同一行 PASS 文本)。
- 先前 navigation-menu-demo 的 locator.click timeout 為**負載誘發 flake**:
  當時 RS 跑 self-test 的同時有 cargo build / go build / contracts 全量在跑
  (runner 單頁長流程 + 機器高載 → CDP 動作性等待窗被拉爆);安靜機器上連續
  綠。不加 fresh-page 偏差,維持與 Go 完全相同的單頁長流程語意。
- EF_DEBUG 探針保留在碼中但無 env 時完全惰性。

## 剩餘 18 指令全部移植(2026-09-08,本輪)

### 純檔案指令 → src/tools/
- docs-upstream-mirror(copy_tree+3 guide 檔)、oracle-css(buildOracleEntryCSS 行分類
  迴圈+tw_compile 空 cwd)、upstream-snapshot(normSnapshot/stack-balanced slice/
  ureq 30s 爬取、SHADLESS_SNAPSHOT_ORIGIN)、parity-baseline(parityNormValue 用
  ryu 對 Go FormatFloat 'f' -1 位元一致、cellMap/diff/write 位元形)、
  docs-consistency(8 檢查)、ir-diff(orderedSet/diffIr/renderIrDiff/--json)、
  resolve-skins(expandClassString/applyRtlMapping 40 案例對 upstream
  transform-rtl.test.ts、resolveSource 逆序 splice)、rtl-dict(esbuild_tsx+
  tsx::string_literals+decodeJSString 嚴格)、upstream(重 pin drill 全流程+
  classifyFailures)、docs-catalog(scanSet/dedupe/status 規則/120KB catalog)、
  overlay(rule/authored/source 三類單元、--audit/--record/--tasks)、
  docs-build(四 MDX 形狀+prettier-batch+content-map+index 頁)。
- css-direction --update 接線(src/tools/css_direction_update.rs)。

### 瀏覽器指令 → src/tools/
- demo-smoke、demo-parity、interactivity-sweep、docs-smoke(python3 http.server
  暫態埠)、style-parity、docs-fidelity、path-parity(pp 模擬樹+tw 子行程)。

### 雙側位元組驗證(全部與 Go 二進位比對)
- 讀取型:demo-smoke、demo-parity(226 頁 8096 比較)、interactivity-sweep、
  style-parity(29 組件 544 元素)、path-parity(48 組件 4104 比較)、
  docs-smoke、docs-consistency、docs-fidelity、ir-diff 4 模式、
  css-direction --update、upstream --report-only(含 upstream-report.md 2556B)。
- 寫入型(快照/還原協定):docs-catalog 120,304B、docs-build 58 頁+static+
  content-map、resolve-skins 全樹、rtl-dict 62,225B、overlay --record
  manifest 11,130B、overlay --tasks 72 封包、docs-upstream-mirror、
  oracle-css(唯一差:tailwind CLI 自身 Done in Xms 計時行,Go-vs-Go 亦漂移)。
- upstream-snapshot:live ui.shadcn.com 爬取雙側位元組一致(7 previews);
  與 committed 的 7 行漂移是 live 站自 pin 後移動,已還原。

### 移植中修的 bug(全部由雙側比對抓出)
1. serde 拒 JSON null 而 Go 容忍:IR `"slot": null`、catalog `"demoPath":
   null`、contract def null 欄位 → drop_nulls/Option 正規化(overlay、
   demo-smoke、path-parity、style-parity、docs-build component_ir、
   docs_transforms load_jsx_tag_index 共 6 處)。
2. overlay OvIrComponent 缺 serde rename "fn"。
3. overlay tasks 的 git diff 失敗訊息(Go 取 stderr 首行)。
4. overlay manifest 陣列格式(Go Encoder 單元素也斷行、元素縮排深一層)。
5. path-parity 的 tw 子行程輸出(Go CombinedOutput 吞 banner)。
6. docs-smoke settled 布林陣列格式(Go %v 空白分隔)。
7. docs-build:remove_dir_all 對不存在路徑(Go RemoveAll 容忍)、
   prettier stdin 要物件陣列、parse_attrs 對 {expr} 分支的 group 存取、
   mirror_set_cache 兩個 OnceLock 分裂(寫入/讀取不同 cell)。
8. upstream 尾行多一個換行(println! vs Go Printf)。

### 回歸
cargo test 112+1 綠(新增 upstream 8 單元測試、resolve_skins 3 測試改為
Go 原版案例)、gen_golden 469/469(重新錄製:樹新增 build/rtl-langs.json)、
ir_diff 61/61、dist_diff 268/268 ×3。

### 環境註記
- build/rtl-langs.json 是 build-rtl 產物(非 committed),本輪以 RS build-rtl
  重新生成;golden 矩陣因此重錄(go.out 與 rs.out 同步更新)。
- Go repo 的 pipeline/pipeline 二進位仍為重建版(該 repo 顯示 M)。

## upstream drill 實跑驗證(2026-09-08,advisor 要求)

- 4 份 scratch 副本(node_modules symlink、.upstream git 完整),Go 與 RS 各跑
  `upstream --to=shadcn@4.19.0`(same-tag self-test,完整 tier 含瀏覽器 gates,
  各 ~14 分鐘)。
- **成功路徑**:stdout 逐行多集比對,唯一差異是計時值(go test 自身秒數、
  runner wall-clock 836.5s vs 836.7s)與並發完成順序;pin.json、ledger.json、
  overlays/manifest.json、upstream-report.md(除計時行)全同;兩側 .upstream
  同 commit、working tree 皆乾淨。stamps 差異是 scratch 路徑嵌入
  shadless.html(環境性)。
- **失敗路徑**(--to=shadcn@9.99.99):抓出一個真 bug——Go cmd.Output() 的
  錯誤是 exec.ExitError("exit status 1"),不是 git stderr;RS 原本印了
  pathspec 訊息。已修(up_git 回 exit status N),修後程式輸出位元組一致、
  兩樹零差異(rollback-on-error 正確)。
- oracle-css 計時行改為機械化驗證:tests/oracle_css.rs 對 Go/RS 各跑一次,
  剝除 `Done in Xms` 後比對 stdout/stderr/exit,並比對 oracle.css 與
  oracle.entry.css 位元組(跑前快照、跑後還原)。
- golden.rs 加 build/rtl-langs.json 前置檢查:未建樹時 skip 並說明,不再
  環境性紅。
- cargo clean 後全新 release build + 全測試:113+1+1 綠。

## 自我接管(2026-09-09):引擎預設不再 spawn 任何 Go

三段式:S1 圖層(5329f7e)、S2 執行層(3b201e8)、S3 驗收層+文件(本段)。
設計與偏差清單見 PLAN.md「自我接管」節。

- **S1 圖層**:`all_go()`=authored Go 逐字表(驗收 oracle);`all()`=self_host 轉換
  (`./build/pipeline`→`__self__@<fp>`、10 個 go-test gates+unit 的 engine 半→
  `__gate`);build.rs 引擎指紋(argv[0] 後綴摺進 key);`SHADLESS_GRAPH=go-mirror`
  呈現 Go 表。pipeline/ inputs 只留資料性讀取(16 條 keep 規則,證據在 nodes.rs
  KEEP_PIPELINE_INPUTS)。教訓:fanout shards 的 argv[0] 要 env-aware——第一版寫死
  `__self__`,gen_golden layer-2 key parity 抓出 29 個 shards 全數漂移。
- **S2 執行層**:engine::resolve_argv0(runner.exec、run_gate、fanout、upstream 四處
  唯一解析點);`__gate <id>` 分發器(10 gates+unit=cargo test --lib);
  usage env 分叉;upstream drill 自我驅動(mirror 才 make pipeline)。
  script-refs gate 的 -run 檢查改對 all_go()(reviewer 抓出:對自我表檢查會空轉)。
- **S3 驗收層**:`__meta [gate]`(Go TestMeta 的 binary 版,run_mutation 從 binary
  驅動——嵌套 cargo lock 的教訓來自 reviewer);tests/gate_parity.rs
  (SHADLESS_GATE_PARITY=1):24 gates 乾淨 verdict 相等+雙邊突變 verdict 相等
  (agreement 制:雙紅=樹狀態警告不計失敗;單邊不一致才失敗;pre-build 先跑
  `run all --builds-only`)。

### parity 第一輪抓到的三個真問題(全部修復,各有獨立重現)

1. **example-golden 印 FAIL 卻 exit 0**:`run_inner` 算完 `exit` 後回
   `Ok(())`,外層只看 Ok/Err——runner/meta 視為綠。這是移植形狀錯誤
   (Go 每路徑直接 `return exit`),修為 `Result<i32, String>` 逐路徑回傳。
   突變下 exit=1、乾淨 exit=0 已驗證。parity 的紅側設計(不只比乾淨綠)
   正是為了抓這種 vacuous gate。
2. **typecheck 突變 anchor 雙花括號**:`mut_replace_once` 是精確匹配無
   format 展開,RS 版把 `* @param {Record<string, string>} hints` 寫成
   `{{...}}` → anchor 永遠找不到(免疫化)。全檔 audit 僅此一處。
3. **harness 自身的 SHADLESS_ROOT 洩漏**:parity 把查找用 env 傳給 Go gate,
   `TestUnitFindRepoRootMarker` 走 env 短路而在暫存樹失敗(Go gate 假紅)。
   gate_parity 所有子行程 env_remove(SHADLESS_ROOT/SHADLESS_GRAPH/...)。

另:docs-smoke/docs-fidelity/interactivity-sweep 在未建 docs demos 的樹上
雙紅(「build first」)→ parity 加 pre-build(`run all --builds-only`)後
docs-fidelity/interactivity-sweep 轉綠。

### parity 第二輪結果(2026-09-09,完整 24 gates)

- **23/24 verdict 一致**(clean 與突變紅側皆然);唯一不一致 unit,根因是
  **`__gate unit` 語意錯誤**:跑整套 lib 會把其他 gate 的真樹測試掃進來
  (reproducible 的真樹測試),已修回 Go 的 `^TestUnit` 語意=unit_ 前綴過濾
  (57 個移植測試;Go 188 含 internal/ 子套件,移植時整併)。GATE_ONLY=unit
  重跑:clean 雙綠、meta 雙抓。
- **WARN(雙紅=樹狀態,兩引擎逐位元組同輸出,非 parity 問題)**:
  - `reproducible`:**上游 committed 的 dist/out.css 已 stale**——fresh tw
    (兩引擎)比 committed 少 `.invisible` 一個 utility(3 行)。機制:
    globals.css 是 `source(none)`+@source 白名單,白名單內已無任何檔案含
    "invisible";上游 commit 29e7368 自己就寫過「regenerate out.css — stale
    since the probes/t7/out cleanup」,同一模式重演。漣漪:跑過 Go
    `^TestUnit` 套件(部分 TestUnit* 重建產物)或 parity pre-build 後,
    dist/out.css 會變 M——樹敏感的驗收(dist_diff/gen_golden)前先
    `git checkout dist/out.css`。
  - `style-parity`:24 格 dialog/dialog-close 系 presence missing
    (oracle=present shadless=missing),兩引擎同輸出;待上游查。
  - `docs-smoke`:index.html article missing/empty,兩引擎同;待上游查。
- **給下一個 session**:gate_parity 重跑很貴(完整 ~80 分鐘),用
  `GATE_ONLY=id1,id2` 切片;SHADLESS_GATE_PARITY=1 才會跑。

### S3 驗收總結(2026-09-09)

- gen_golden(go-mirror)469/469;cargo test 117+1+1;ir_diff 61/61;
  dist_diff 268/268 ×3(build-js/emit/demo)。
- gate_parity:24 gates 中 23 個 verdict 完全一致;unit 修復後
  GATE_ONLY=unit 一致;三個 WARN 皆兩引擎同輸出的上游樹狀態問題。
- `run all`(自我接管,69 節點)兩次:#1 ran 50 / skipped 16——24 gates 全走
  RS(`__gate`/`__self__`,含 29 個 contract shards、example-gate 227 頁、
  golden-gate、style-parity 64.6s ✔);red 僅 reproducible(上游 stale)與
  docs-smoke(同 WARN)。#2 ran 7 / skipped 60——失敗節點拔 stamp 重跑+
  輸入被 #1 改寫的節點重跑,其餘全 fresh,stamp 語意正確。
  **style-parity 在完整圖中轉綠**——parity 輪的雙紅是狀態相依(pre-build 後
  docs demos 未被 demo 節點重排),非產品 bug;兩引擎同步紅/同步綠,parity 無虞。
- 環境註記:parity/run 後 dist/out.css 常呈 M(上游 stale),樹敏感驗收前還原。
- **驗收契約變化**:產物鏈位元組同值不變;圖面 Go 對照移到 go-mirror 模式
  (gen_golden 469 案零重錄,只加 export);gates 驗收從 argv/keys 同值改為
  verdict 同值(含紅側)。
- 已知偏差 6 條見 PLAN.md(讀檔審計覆蓋縮減、跨引擎 stamps 全 STALE 一次、
  usage 文案分叉等)。
- 環境註記:goldens 的 status 案例與真樹 stamps 狀態耦合——在真樹上跑過引擎後
  先重跑 gen_golden 再跑 cargo test(與既有 dist 陷阱同類)。

## per-node 引擎指紋(2026-09-09,本輪)

M7 的引擎級指紋(改任何 .rs → 全圖 STALE)細化為 per-node:build.rs 對
hull(src 根檔案 + Cargo.toml/lock/build.rs/rust-toolchain.toml)與八個實作
群組(convert/emit/gates/oracle/tools/twmerge/tsx 目錄 + jsbuild.rs)分別求
sha256;nodes.rs 以 NODE_ENTRIES(節點→群組,main.rs 動詞分派的鏡射;convert
節點因執行 resolve-skins 動詞而含 tools 群組)與 GROUP_DEPS(群組依賴 DAG)
摺出 `__self__@<fp>`。unit → global(整 crate 雜湊);typecheck/docs-site/
unit cmd[0] 維持產品面命令、無指紋、不受引擎編輯影響。fanout shards 走
`contracts:<name>` 前綴取 oracle 群組。go-mirror 模式照舊回 `./build/pipeline`,
golden 台零影響。

- **advisor checkpoint(採納其修正)**:原設計把 import 閉包解析器放
  build.rs(regex 抓 crate::/super::/uniform paths)——顧問指出兩個真洞
  (`use crate::{a, b::c}` brace 形式、`as` 別名)且覆蓋測試抓不到,改採
  「群組 DAG + build.rs 零解析」:soundness 由執法測試以 raw-text grep
  (註解/字串都算引用,過近似=安全向)驗證,並直接禁掉 `crate::{` 與
  `use crate::x as y` 兩種看不見的慣用法。tools 內檔案級粒度
  (docs_transforms.rs 被 6 檔共享,需第二層依賴表)列為未來細化。
- **新執法測試 5 個**(nodes.rs):build.rs 配對(GROUPS/HULL_FILES 兩表
  逐元素相等)、hull 覆蓋(src 根檔案不在 hull 即紅——新根檔案永不會靜默
  失去 stale 能力)、NODE_ENTRIES 雙向覆蓋(self_host 後帶 __self__ 的節點
  集合 == 表列集合)、raw-grep 依賴審計(群組內任何跨群組引用必須在
  GROUP_DEPS)、粒度性質(tools 同群組同 fp、跨群組不同)。
- **重要語意發現(修正本檔早前的心智模型)**:key.rs 的 key() 摺疊
  **依賴鍵**(`dep\x00{d}\x00{dk}`,遞迴)——上游 key 變更會結構性級聯到
  needs 下游,即使上游輸出位元組不變。這是 Go 位元組同值契約的一部分,
  必須保留。因此 per-node 指紋的真實收益 = 「執行該群組的節點 + 其
  needs 下游」:編輯 gates 葉群組(ledger/script-refs 等)只 stale 自己;
  編輯 tools 會經 convert(其 entries 含 tools)級聯全圖;oracle/emit
  編輯沿 demo 鏈級聯。
- **驗證(全部可證偽預測,逐一通過)**:
  1. 收斂:方案切換後 `run all` ran 68 / skipped 1(typecheck)/ 857.1s /
     exit 0 零失敗;事後 status 68 fresh + 1 NEVER-FRESH。
  2. Spot A(append 註解到 src/tools/docs_smoke.rs → 重建):fresh 僅剩
     pin / build-js / typecheck / ledger / script-refs(舊方案會 68 全
     stale);還原重建後 68 fresh 完整恢復(stamps 有效,零重跑)。
  3. Spot B(append 註解到 src/runner.rs = hull → 重建):除 typecheck 與
     NEVER-FRESH 外全部 STALE(寧可全重跑方向無假 fresh);還原後恢復。
  4. gen_golden 469/469(go-mirror 無指紋,零重錄)+ cargo test 全套綠。
- **review_change 抓出一個真洞,已修**:12 個非 .rs 檔案經
  include_str!/include! 編進 binary(oracle_canon.js、ef_*.js、twmerge 的
  config/snapshot.json、jsx_overrides.inc 等),但群組雜湊只收 .rs——改
  oracle_canon.js 不會 stale 任何節點(舊方案同款洞,但執法測試的
  soundness 宣告歸本輪修)。修法:walk 改收目錄內**全部檔案**;覆蓋測試
  延伸:src 根的非 .rs 檔必須進 HULL_FILES、src 下新目錄必須宣告為
  ENGINE_GROUP(否則未來新 tier 目錄只被 hull 引用時永遠不 stale)。
  reviewer 其餘查核全過:NODE_ENTRIES 對 main.rs 分派逐動詞核實、
  GROUP_DEPS 與實測引用一致、go-mirror/__keys/gate_parity/resolve_argv0
  無漂移、 dropping rerun-if-changed 正確(cargo 預設=任何 package 檔案
  變更即重跑 build script,是舊清單的超集)。
- **餘留(記錄,done-call 顧問要求)**:(1) 覆蓋測試只保證「src 下所有
  檔案都被雜湊」,不保證「非測試碼的 include_str!/include! 不伸出 src」
  ——未來若有人加超出 src 的 embed 會重演 reviewer 的洞;廉價跟進:
  raw-grep 審計加一條對 `include!(..`/`include_str!(..` 伸出 src 的
  失敗。(2) stamps 是 upstream 樹的 runtime 產物(未 commit)——fresh
  clone 上 status 全 STALE 直到第一次收斂,「68 fresh」不是 repo 不變量。
- **給 Oxc 實驗的前置(顧問建議)**:實驗開工前先 pin 基線(記錄
  convert/jsbuild 群組的 fp hex,事後 diff 證明只有這兩群組移動,讓
  隔離宣告可證偽);feature flag 若放 Cargo.toml 屬 hull,每切換一次
  全圖 stale——應在群組內以 env/arg 閘,或接受功能落地時的一次性
  hull stale;新增 oxc/rolldown 依賴必改 Cargo.lock(hull)→ 排一次
  ~15 分鐘全圖收斂再開始迭代。首次切換 flag 後 status 應顯示「僅
  convert/jsbuild 相依節點 STALE、瀏覽器層 fresh」,否則隔離假設有誤。
- 環境註記:上一輪的「全 stamps STALE 預期」已在本輪開頭實證收口
  (ran 52 / skipped 17 / 294.2s / exit 0,事後 68 fresh;gen_golden 469
  零漂移;goprobe 重建屬環境性 churn 已還原)。upstream 樹維持已知唯一
  `M pipeline/pipeline`(重建的 Go 二進位)。

## parity WARN 收尾(2026-09-09,第二輪)

三個 WARN 的最終處置,全部結案:

1. **docs-smoke(index.html article missing/empty)——真產品 bug,已修**。
   根因:67621cd 遷 Zola 時舊 vitezola 主題首頁渲染 `<div class="VPHome">`,
   smoke 的 render 檢查以 `.VPHome` 為首頁 fallback;ba179f1 換成復刻
   ui.shadcn.com DOM 的自製主題後,首頁改渲染 `<section class="home-hero">`,
   檢查沒跟著更新——60 個內頁走 `.vp-doc` 全過,只有 index 紅,雙引擎同輸出
   (移植忠實,上游 stale check)。修法:檢查選擇器 `.VPHome` → `.home-hero`
   (advisor 確認 check 側修、且直接刪死選擇器不留 alternation;全庫僅此一處
   引用,無測試/golden 釘住)。Go pipeline/docs_smoke.go 與 RS
   src/tools/docs_smoke.rs 同步修改,雙引擎 stdout/stderr/exit 位元組一致,
   `docs verify (61 pages, 0 console errors)` 雙綠;圖級 `run docs-smoke` 綠。
   教訓:`go build -C pipeline -o build/pipeline` 的 -o 相對 -C 目錄解析——
   少寫 `../` 時新二進位落在 pipeline/build/ 下,跑到的仍是舊 binary(本輪
   「修了還紅」的假象來源;以 pipeline/build/ 誤產物已刪)。
2. **reproducible(dist/out.css stale)——已按上游先例重新生成並提交上游**。
   機制同 parity 記錄:globals.css 是 source(none)+@source 白名單,白名單
   九個目錄逐一 grep 證實無任何 `invisible` 引用,dist/demos/docs 也無頁面
   使用該 class——committed out.css 的 `.invisible`(3 行)是 stale 殘留,
   fresh tw(雙引擎)一致地不再產出。按上游 29e7368「regenerate out.css」
   同款處置,重新生成的 dist/out.css 已提交上游(bd194f6;check 修復為
   38830ae),reproducible 恢復恆綠(此前每輪 parity/run 後都要手動
   git checkout 的循環到此為止)。
3. **style-parity(24 格 dialog/dialog-close presence)——S3 已結案**
   (狀態相依,完整圖中轉綠,見上方 S3 總結;非產品 bug)。

### out.css 重生的影響面(gate 證據)

顧問要求的 gate 證據(grep 之外):全庫僅 dist-complete gate 對樹內 out.css
有內容斷言(slot 選擇器);css_direction/coverage 的 out.css 引用是註釋、
consumer-sim 用 tempdir 自建檔、audit/mutations 是清單與突变定義,無任何
哈希釘住。提交後乾淨樹上的 cargo test 已跑全部 10 個真樹 gate 測試
(117 過 0 略),含 gate_dist_complete/css_direction_baseline/consumer_sim
——即重生的 out.css 已過所有會讀它的快 gate。demo-parity/example-fixture/
contract-fixture 雖以 out.css 為 input,但頁面無人使用 `.invisible`
(grep 證據見上),不匹配任何元素的規則不可能改變計算樣式,渲染 gate 結構上
不受影響。本輪 .rs 修改使引擎指紋變化,**全部 stamps 已 STALE——下次
`run all` 會整圖重跑,屬預期行為(已知偏差 4),非退化**。
