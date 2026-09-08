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
- [ ] M3.5 api.Build probe gate(移植 oracle_lib/example_fixture 之前)
- [x] M4 convert.go → IR(**61/61 byte-identical**,2026-09-07;全部 4 個 drift gates PASS、tier dist/conditionals 與 Go 一致)
- [x] M5(部分)jsbuild + emit 鏈完整移植並驗證：**build-js 268/268 dist byte-identical**（含 esbuild CLI minify）、**emit 268/268 dist byte-identical + build/emit globals.css 63,815B byte-identical**、emit 三 gates PASS（23 檔/96 slots/15 anchors）。demo.go/rtl/product_css/oracle/docs 鏈與 M6 gates 仍待移植
- [ ] M6 gates 24 個 → #[test]

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
