# HANDOFF — shadless pipeline work, 2026-08-30

給下一個 context 的交接。讀完這份 + `git log --oneline -8` 就能接著做。
repo: https://github.com/at-least/shadless（`origin/main` = `6c5c630`，working tree 乾淨）。§5 那批已做完，下一批看 §6。

## 1. 今天做了什麼（8 個 commit，皆已 push）

| commit | 內容 |
|---|---|
| `f94f61f` | Initial commit（歷史已重寫：原始 commit 含 533MB esbuild bundle，已從所有歷史移除並 force-push） |
| `28386e6` | docs：語法高亮改成瀏覽器端（`src/docs/highlight-client.mjs` → esbuild 成 `docs/site/highlight.js`，834KB/130KB gz，離線、同兩套 shiki 主題）；build 只輸出純文字 `<span data-line>` + `data-lang`。修 `src/docs/demo-scripts.mjs:17` 的 `glue/`→`js/`（JS 分頁原本內嵌 163KB `shadless.js` ×121 = docs/site 的 90%，現在顯示元件自己的 `js/<name>.js`）。docs/site 137MB → 12MB |
| `34090ad` | 停止追蹤 esbuild bundle |
| `4a7c5a1` | oracle bundle cache 搬到 `node_modules/.cache/shadless/oracle`（`CACHE_DIR` in `tools/oracle-lib.mjs`，`SHADLESS_CACHE` 可覆蓋），四個 consumer 共用一份；harness html 用相對路徑且不再追蹤 |
| `08595b0` | 所有跨節點中間產物集中到 gitignored 的 `build/`；`probes/out/tiers.json` → `src/registry/tiers.json`；`probes/out/upstream-payload` → `src/registry/upstream-snapshot`；`gates/out` → `build/gates`；`rtl-langs.json` → `build/`。**注意：這個 commit 曾把 partial out.css 和 116 個頁面的刪除掃進去，已在 `5a09ee1` 修回** |
| `5a09ee1` | **Wireit**：registry 每個節點加 `inputs`；`gates/wireit.mjs --write` 從 registry 生成 `package.json` 的 `wireit` 區塊（`w:<id>` + `w:fast/medium/full/builds`）；gate `wireit-sync`（mutation `wireit-drift`）確保同步；Makefile / CI 改走 wireit，`WIREIT_PARALLEL=4` |
| `2b1955a` | `emit` 不再 wipe `dist/components` 也不再覆寫 `globals.css/out.css/demo-index.html`（scratch 到 `build/emit/`）→ medium tier 不再破壞 committed tree。`pin` 宣告真實 inputs（`inputs: null` 會讓所有 dependent 永遠 not-fresh）。`consumer-sim` scratch 改用 OS tmpdir |
| `c89b8dc` | `dist/out.css` 改成**明確** `@source` 清單（`tools/demo.mjs` `SOURCES`），不再用 tailwind 自動偵測（不確定性 + 1,200 行垃圾 rule）。`product-css.mjs` 抽 theme 時把 `source(none)` 還原。`pack`/`dist-complete` 加上對 dist producer 的 `needs`（平行時會 race） |
| `bcb16fa` | `overlay-stale-authored` mutation 改指 `behavior:sheet`（CI meta 之前是紅的）；`overlay.mjs` 刪掉死的 `glue:/kernel-fixture:/contract:` 前綴 |

數字：tracked 670MB → 35MB、pack 22MB → 5.9MB；full graph 冷跑 994s → 427s、暖跑 1s（40/41 skip）；medium 暖跑 0.2s。

## 2. 現在怎麼跑

```sh
make fast / make medium / make build     # = npm run w:fast / w:medium / w:full（wireit，增量+平行）
npm run w:<node-id>                       # 單一節點 + 它需要的 closure（node id 見 make list）
make all                                  # 舊的 gates/run.mjs 全跑（不 skip）
node gates/meta.mjs --tier=medium         # mutation test（fast+medium，約 2 分鐘）；--only=<id>
node gates/wireit.mjs --write             # 改了 gates/registry.mjs 之後必跑，否則 wireit-sync 會紅
WIREIT_FAILURES=continue npm run w:full   # 看全部失敗而非停在第一個
```

長跑放背景：`npm run w:full > build/full.log 2>&1`，看 `grep -E "Ran |^❌ \[|^FAIL" build/full.log`。

## 3. 陷阱（每一條今天都踩過）

- **`git add -A` 之前先 `git status -- dist docs`**。任何被中途殺掉的 pipeline 都可能留下半生成的 dist。（`emit` 與 `example-oracle` 現在都不會了 — 後者改成全成功才寫，見 §5.1。）
- ~~**`example-oracle` 靜默自我縮小**~~ 已修（`ddc03fb`）。
- **改 `src/registry/ir/*.json` 當測試沒有用**：`convert` 節點會從 pinned .tsx 重新生成 IR，把你的改動蓋掉。要驗 IR→輸出這條路，直接 `node tools/demo.mjs`，不要走 `npm run w:<id>`。
- **wireit 的 output glob 必須在可平行的節點間互斥**（已用 `!…-rtl-*.html` 排除），否則 manifest 會 race 出 "deleted unexpectedly"。
- **wireit：`inputs: null` 的節點讓所有 dependent 永遠 not-fresh**。只有 `reproducible` 可以是 null。
- **tailwind 自動內容偵測在 cwd 位於 gitignored 目錄時會失效**（會跟著 symlink 掃進整個 repo）——所以 `consumer-sim` 用 OS tmpdir、`out.css` 用明確 `@source`。
- `tools/demo.mjs` 的 `SOURCES` 與 `gates/registry.mjs` 裡 `demo-css` 的 `inputs` 必須同步。
- 出貨頁面裡有 `has-[&gt;svg]:gap-x-2` 這種 HTML-escaped 的 class；tailwind scanner 讀不到，是靠掃 `src/registry/ir/*.json` 才編出來的。消費者複製這段 markup 時他們自己的 scan 也讀不到——**尚未處理的消費者面 bug**。
- 記憶檔（`~/.claude/projects/-home-newlix-github-at-least-shadless/memory/`）裡的 build-sequencing 與 review 兩份也更新過，內容與本檔一致。

## 4. 完整檢討的未結項目（已親手驗證，含檔案位置）

**出貨面（確定在出錯）**
- ~~`dist/css/<name>.css` 沒有 writer~~ 已修 `f735e8a`。
- ~~`DEFAULT_CONTENT` 的 52 處 `hsl(var(--x))`~~ 已修 `3d89071`。（**仍未處理**：沒有任何 gate 在看 inline style。）
- runtime：`src/runtime/components/dropdown-menu.js` 與 `context-menu.js` 只差兩行、共用 `__menuWired["menu"]` guard → 後載入者永遠不註冊，且倖存者把對方的 trigger 標成自己的 `component`（違反 `shadless.d.ts` 的 union）；`popover.js:39` 丟掉 `restoreFocus`（`destroy()` 會搶焦點）；`carousel.js:35` 的 `window.__api` 出貨到 npm；`navigation-menu.js:24` 把 20 個 Tailwind class 寫在 JS 裡（消費者的 scan 掃不到 → viewport 無樣式）；ESM 路徑下 `runtime.min` + `js/<name>` 會把 kernel 跑兩次。
- `vendor/radix-kernel.iife.js` 是 ~2,355 行手寫 TS 的行為複刻（非 radix）+ floating-ui，原始碼不在 repo；`@floating-ui` 沒 pin；embla 的 entry `tools/.embla-entry.mjs` 不存在、無 sha pin；`SECURITY.md:25` 描述錯誤。

**gate 誠實度**
- ~~parity baseline 只記 cell id 不記值~~ 已修 `c617362`。（**仍未處理**：`flaky[]` 沒有預算。）
- `example-gate`（`registry.mjs`）的 closure 含 `example-oracle`，後者正是寫出它要檢查的檔案 → 結構上必綠，48 秒白跑。刪掉；hop 2 靠 `reproducible`。
- `reproducible` 檢查 `tools/contracts/out` 但不 `needs: contracts`。（`coverage`/`overlay` 判上一輪 manifest 的問題已修 `ddc03fb`。）
- `gates/overlay.mjs:317` `dissolved` 不算失敗：`skin-allowlist:cn-menu-translucent` 的理由已被上游 `style-nova.css:1450` 推翻，overlay 自己報 `1 rules can be DELETED` 卻 PASS。
- docs transform：`## Usage`/`## Composition`/`## API Reference` 找不到就靜默跳過（`tools/docs-build.mjs:246`），`docs-fidelity-lib.mjs:75-82` 用同樣方式跳過 → 上游改標題會漏 React 內容進站且全綠。
- converter 靜默空頁：`forwardRef`/HOC 元件產出 0 element（`src/converter/index.mjs:425-446`）；`import { cva as x }` 讓 variant 全消失（`:90,:641`）；skin 巢狀規則被 `parseSkinMap:41` 靜默漏掉；`src/registry/ir/form.json` 是孤兒（上游已無 `form.tsx`，`main()` 只寫不刪）。

**re-pin drill 永遠不可能綠**（`gates/upstream.mjs` + `.github/workflows/upstream.yml`）
- `reproducible` 在任何真 re-pin 必紅，但 `upstream.mjs:159` 要求 0 failed。
- golden 快照在 drill **之後**才刷新（`upstream.yml:57-60`），drill 拿舊 live 快照比新 tag。
- 分類器（`upstream.mjs:130-133`）用 log 尾 25 行子字串比對。
- `package.json` 55 個 `@radix-ui/*` overrides 與 `vendor/radix-kernel` 在 radix 升版時沒任何東西比對。
- `EXEMPTIONS.md` 從不 `--render`。

**清理**
- `tools/build-demo.mjs` 整支是死的（`EMITTERS = {}`）但仍是 `npm run docs`/`demo` 第一步（registry 節點 `demo-build`）。
- `tools/docs-consistency.mjs` §1–3 是套套邏輯（同一個 `injectSiteSkin` 剛寫完幾秒後重讀）；§4–6 有價值。
- 重複實作：fence stripping 三份、`<ComponentPreview>` 掃描四份、guide 清單兩份且不一致（`docs-guides.mjs:29` vs `docs-catalog.mjs:102`）、grey 清單兩份（`docs-build.mjs:63` vs `docs-catalog.mjs:72`）。
- `src/emitter/css.mjs:101-391` `componentCss` 290 行七種職責；IR 的 `children` 是字串草圖再用 regex 重解析（`emitter/index.mjs:32-36`）。
- `contracts`（`tools/contracts/run.mjs:16-19`）29 個元件序列各開一個 chromium + 350–500ms 硬 sleep，佔 full tier 46%；改 worker pool 可再砍一半。
- 是否繼續追蹤 `docs/site`（12MB，純 build 產物）未決定；不追蹤就能刪 docs-consistency §1–3 並改 CI/Pages 建置。

## 5. 建議的下一批 — **已完成（2026-08-30，commits `ddc03fb`…`6c5c630`，已 push）**

1. ✅ `example-oracle` 失敗要大聲 — `ddc03fb`。改成兩階段：先把所有 target render 進記憶體，全部成功才寫頁面 + 兩份 manifest；任何失敗列出全部並 exit 1、**磁碟上一個字都不動**。`--strict` 旗標（沒人傳過）與 `failed` 誤算（把交給 example-fixture 的 kernel 頁面算成失敗）一併刪掉。`coverage`/`overlay` 加了 `needs: example-oracle`。新 mutation `example-oracle-render-failure` 注入 render-time throw → CAUGHT。
   - **副作用**：`coverage`/`overlay` 的 effective tier 變成 full，`make fast` 只剩 pin/unit/ledger/wireit-sync，`meta.mjs --tier=medium` 從 9 個 mutation 掉到 6 個。缺的三個要另外跑：`node gates/meta.mjs --only=coverage-drop-contract,overlay-stale-authored,overlay-orphaned-rule`（已驗證全 CAUGHT）。
   - **順手挖出的舊 bug**：11 個 `message-scroller-*` example 一直 render 失敗（`@ai-sdk/react` / `@shadcn/react/message-scroller` / `@shadcn/helpers/ai-sdk` 都不在 repo 裡），以前被 `KEEP` 吞掉。沒有另開清單 — 這 11 個名字連同理由早就在 `src/registry/upstream-snapshot/exemptions.json`（golden-hop exemption，`gates/ledger.json` 的 `golden.exempt-demos` budget，target 0），工具改成挑 reason 開頭是 `external dep ` 的那些才容忍失敗（`62f04bc`）。
2. ✅ `dist/css/*.css` writer — `f735e8a`。寫在 `tools/demo.mjs`（它本來就在 `wrapComponentCss(ir.name, componentCss(ir))` 那一行算出同一段文字），不是 `product-css.mjs` — 後者要重寫一份 `irAll` 的 tier filter。48 個檔案全部 byte-identical 重算出來。`dist/css` 移到 `demo` 的 `produces`；`product-css` 只留它真的寫的兩個檔，並補上兩個一直沒宣告的 input（theme-prepaint 的 `SHADLESS_CSS_FIXES`、被 inline 的 tw-animate-css）。離開 emit 集合的元件會連檔案一起刪掉。
3. ✅ `hsl(var(--x))` → `var(--x)` — `3d89071` + `6c5c630`。52 處（51 純 token + 1 帶 alpha 改成 `color-mix(in oklab, …)`）。`grep -c "hsl(var(" dist/components/*.html` 歸零，12 個 dist 頁面 + 13 個 docs/site 頁面重新產生，`dist/out.css` byte-identical。
4. ✅ parity baseline 記值 — `c617362`。三份 record/compare 抽成 `gates/parity-baseline.mjs`（本來就已經 drift：只有 style-parity 有 `flaky` 和 `--strict`）。cells 變 `[{id, oracle, shadless}]`，除了 appeared/fixed 之外多一個 **`changed`**（還是不一樣，但差的量變了）。重錄 210 / 76 / 0，**每一個 id 與舊 baseline 完全相同** — 重錄只加了值，沒藏東西。新 mutation `style-parity-recorded-value-drift` 改 baseline 裡已記錄 cell 的值 → CAUGHT（這是唯一能分辨「gate 讀值」與「gate 只讀 id」的 mutation）。

驗證全跑過：`WIREIT_FAILURES=continue npm run w:full` 綠（41 節點）、`node gates/meta.mjs --tier=medium` 6/6 CAUGHT、上面那三個 tier 外的 mutation 3/3 CAUGHT、working tree 乾淨。

## 6. 下一批候選（§4 剩下的，按投報率）

1. **`example-gate` 刪掉**（closure 含 `example-oracle`，結構上必綠，48 秒白跑）；hop 2 靠 `reproducible`。順帶：`reproducible` 檢查 `tools/contracts/out` 但不 `needs: contracts`。
2. **`gates/overlay.mjs:317` `dissolved` 要算失敗** — `skin-allowlist:cn-menu-translucent` 現在就是這個狀態：overlay 自己印 `1 rules can be DELETED` 卻 PASS。
3. **runtime bug 群**（出貨到 npm）：`dropdown-menu.js`/`context-menu.js` 共用 `__menuWired["menu"]` guard → 後載入者不註冊；`popover.js:39` 丟掉 `restoreFocus`；`carousel.js:35` 的 `window.__api`；`navigation-menu.js:24` 把 20 個 class 寫在 JS 裡（消費者掃不到）。
4. **docs transform / converter 的靜默跳過**：`tools/docs-build.mjs:246` 找不到 `## Usage` 就跳過、`src/converter/index.mjs:425-446` forwardRef 產 0 element、`import { cva as x }` 讓 variant 全消失、`src/registry/ir/form.json` 是孤兒。
5. **re-pin drill 永遠不可能綠**（§4 最後一段整段）。
6. **清理**：`tools/build-demo.mjs` 全死；`docs-consistency` §1–3 套套邏輯；fence stripping ×3、`<ComponentPreview>` 掃描 ×4、guide 清單 ×2 且不一致、grey 清單 ×2。
7. **`has-[&gt;svg]:gap-x-2` 的消費者面 bug**（§3 最後一條）尚未處理。
