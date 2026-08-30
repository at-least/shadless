# HANDOFF — shadless, 2026-08-31

給下一個 context 的交接。讀完這份 + `git log --oneline -15` 就能接著做。
repo: https://github.com/at-least/shadless

**先看這個**：`origin/main` 還停在 `6c5c630`，本地 `main` 領先 **61 個 commit，全部沒 push**。
working tree 乾淨（`212594e`）。

---

## 1. 現在怎麼跑

CI **不存在了**（`dc6113e` 把最後一個 workflow 也刪了，`.github/` 整個沒有）。
所有東西都在本機或 Dagger 裡跑。

```sh
make                      # 全跑：build 步驟 + 每個 gate（冷跑 ~840s @ -j4）
make fast                 # 不開瀏覽器的 gate（<1s，pre-commit 用）
make only ID=<node>       # 一個節點 + 它需要的 closure（節點清單見 make list）
make all                  # 同一張圖，不做 freshness skip
make meta                 # mutation test（ONLY=<id> 跑單一個、TIER=fast 只跑不開瀏覽器的）
make serve                # 看文件站（= npx vitepress preview docs，PORT ?= 8765）
npm run docs              # 重建文件：build-rtl → docs-catalog → docs-build → vitepress build
npm run docs:dev          # 邊改邊看
```

圖在 `pipeline/nodes.go`（Go，NodeID 由編譯器檢查），68 個節點。stamps 一個節點一個檔在
`pipeline/stamps/`，**不追蹤** → 新 clone 一定是冷的。

Dagger：`.dagger/` 把 build 那半邊搬進容器（convert / emit / oracle / build-js /
example-fixture / demo-rtl / demo / contract-fixture / CSS / 29 個 contracts / 上游站台）。
`dagger call <Step>`；`.dagger/trees.go` 是兩棵沒有單一 owner 的樹（`distTree` / `demosTree`）。

---

## 2. 今天（2026-08-31）做完的三件事

**a. Dagger port**（`b75d138`…`d6db264`，約 50 個 commit）
wireit 拿掉換成 Go runner；節點宣告自己真正讀寫什麼（undeclared read/write 檢查）；
contracts 從 350.9s 攤成 55.0s；medium tier 刪掉；`emit-smoke` 刪掉；上游自己的 docs site
現在也在 Dagger 裡建（`.dagger/upstream.go`，crawl 我們 pin 的那個 commit 而不是
production，436/443 previews byte-identical）。

**b. 決定：只做 radix**（`97cdc85`）
上游三個平行 registry（radix / base / aria）不移植後兩個——`vendor/radix-kernel.iife.js`
是 143KB 手寫行為複刻且 **repo 裡沒有原始碼**，支援 base 等於從零再寫一份。
「我們 target 哪個 base」本來寫在三個地方沒人比對，現在 `pin` gate 會檢查。

**c. 文件站改成 VitePress**（`70defe9`…`212594e`）
- `docs-upstream` 刪掉（不再鏡射上游 chrome）
- `docs/site` 不再追蹤（12MB / 583 檔），`docs-consistency` §1–3 刪掉（套套邏輯）
- `tools/docs-build.mjs` 從「自己刻的 SSG」變成純文字轉換，吐 markdown；VitePress 建站
- `docs-links` 刪掉（VitePress 自己會因為 dead link 而 fail）
- 量過才動手：現有 transform 跑完後，65 頁上游 mdx 只剩 **4 種** MDX 形狀要對應
  （ComponentPreview 460 / Steps·Step 18·30 / Callout 16 / ComponentSource 3）；
  CodeTabs·Tabs* 從 61/61/122/122 掉到 **0**
- 任何沒對應到的 JSX 會 **中斷 build**（`assertNoJsx`），取代舊 builder 三個靜默跳過

現況數字：`docs/components` + `docs/guides` 共 60 個 tracked markdown；
`docs/public`（demo + 資產）與 `docs/.vitepress/{dist,cache}` gitignored；
`vitepress build` 27s / dist 42MB。

---

## 3. 陷阱（每一條都踩過）

- **`git add -A` 之前先 `git status -- dist docs`**。被中途殺掉的 pipeline 會留下半生成的樹。
- **runner 的紅色標記是 `✗` 不是 `❌`**；grep 錯的那個會把紅的一輪讀成綠的。
- **不要 `... | tail`**。管線會吃掉 exit code，`FAIL` 的行也會被截掉。今天靠這個誤判過一次
  「13 頁 render 失敗」——實際上是下一條。
- **不要在有東西 serve 或驅動站台的時候重跑 `vitepress build`**。hashed asset 會被換掉，
  server 直接 ENOENT 死掉，正在跑的 playwright sweep 會報一堆假的 render / console 錯誤。
- **改 `src/registry/ir/*.json` 當測試沒有用**：`convert` 會從 pinned .tsx 重新生成蓋掉。
  要驗 IR→輸出直接 `node tools/demo.mjs`。
- **`PIPELINE_PARALLEL=1` 才會啟用 undeclared-WRITE 檢查**（需要安靜的樹才能歸因）。
  undeclared-READ 檢查任何 -j 都會跑。
- **`emit` 和 `demo` 的 `produces` 是從 `src/registry/tiers.json` 推導的**
  （`pipeline/produces.go`），不是 glob。不要「簡化」回 `dist/components/*.html`：
  read 檢查把依賴的 `produces` 當成已涵蓋，那個 glob 會把 emit 下游每個節點都赦免掉。
- **markdown 裡的 html block 內部不會解析 markdown**。今天中了兩次
  （`<p class="page-links">` 和 `<p class="demo-langs">`）——裡面要放連結就寫 `<a>`。
- **`rewriteLinks` 會把所有無法路由的 `/` 開頭連結變成純文字**，所以我們自己產的
  `/demos/…`、`/components/…` 必須在它跑完之後才注入。
- 出貨頁面裡有 `has-[&gt;svg]:gap-x-2` 這種 HTML-escaped class；tailwind scanner 讀不到，
  是靠掃 `src/registry/ir/*.json` 才編出來的——**見 §4.8**。

記憶檔在 `~/.claude/projects/-home-newlix-github-at-least-shadless/memory/`，內容與本檔一致。

---

## 4. 接下來要做的工作（按投報率）

### 4.0 push（先做這個）
61 個 commit 沒推。`origin/main` = `6c5c630`。推之前跑一次 `make` 確認全綠。

### 4.1 oracle 把「量測出來的 px」烤進 committed 頁面 — 擋住 Dagger 全綠
`tools/oracle-lib.mjs:141` 的 harness **完全沒有 CSS**，所以 chromium 量到、radix 寫回 DOM
的每個數值都是預設字型的函數。`docs/demos/accordion-rtl.html` 帶著
`--radix-collapsible-content-height: 19px`，在 Debian 容器裡是 36px。
量過的範圍：oracle+fixture+rtl 這片 **635 個檔裡有 27 個不同**，而且第二組是 **拉丁文**——
tooltip / popover-rtl / hover-card-rtl / kbd-tooltip 內嵌 floating-ui 從**渲染文字寬度**算出的
`--radix-popper-transform-origin` 和 arrow `left:`（`28.1485px` vs committed 的
`27.991999999999997px`，radix 用完整浮點精度序列化）。

`reproducible` 看不到它——大家都在產生那些 bytes 的同一台機器上跑。這是 Dagger 下
`example-gate` / `reproducible` 唯一還綠不了的原因。

兩條出路，都是真工作，都還沒做：
- (a) oracle 不再把量測值烤進 committed 頁面
- (b) repo 宣告自己的字型集，把 6 個受影響的 base 頁 + 16 個 RTL 衍生頁 + golden baseline 重錄

**不要**用 `.dagger/` 裡 apt 裝字型來蓋過去——那就是 module header 拒絕的「第二份沒宣告的
version-of-record」。

### 4.2 刪掉死掉的 docs 模組（~1,400 行）
`src/docs/{assets,components,jsx,highlight,highlight-client,icons}.mjs` +
`tools/docs-page-lib.mjs` 在 VitePress 之後全是死的，但還被三個地方 import：
- `tools/build-rtl.mjs` → `src/docs/components.mjs`
- `tools/unit/prepaint.mjs` → `src/docs/assets.mjs`
- `tools/unit/docs-tools.mjs` → `tools/docs-page-lib.mjs`

先解開這三條再刪。順帶：`tools/build-demo.mjs` **整支是死的**（`EMITTERS = {}`），
`demo-build` 節點早就從圖裡消失了，但 `package.json` 的 `demo` script 還在呼叫它，
`pipeline/audit_boundary.go` 還在分類它。

### 4.3 `example-gate` 結構上必綠
`pipeline/nodes.go:487`：跑 `tools/example-oracle.mjs --check`，`Needs: docs-build`，
而 docs-build 的 closure 含 `example-oracle`——也就是寫出它要檢查的那些檔的那一個。
48 秒白跑。刪掉，hop 2 交給 `reproducible`。

### 4.4 `overlay` 的 `dissolved` 不算失敗
`gates/overlay.mjs:328-330` 只印 `N rules can be DELETED` 然後 PASS。
`skin-allowlist:cn-menu-translucent` 現在就是這個狀態（理由已被上游
`style-nova.css:1450` 推翻）。要算失敗。

### 4.5 runtime bug 群（**這些會出貨到 npm**）
- `src/runtime/components/dropdown-menu.js` 和 `context-menu.js` 只差兩行、共用
  `__menuWired["menu"]` guard → 後載入的那個永遠不註冊，而且倖存者會把對方的 trigger
  標成自己的 `component`（違反 `shadless.d.ts` 的 union）
- `popover.js:39` 丟掉 `restoreFocus` → `destroy()` 會搶焦點
- `carousel.js:35` 的 `window.__api` 出貨到 npm
- `navigation-menu.js:24` 把 20 個 Tailwind class 寫在 JS 裡（消費者的 content scan 掃不到
  → viewport 沒樣式）
- ESM 路徑下 `runtime.min` + `js/<name>` 會把 kernel 跑兩次

### 4.6 re-pin drill 永遠不可能綠
現在是 `pipeline/upstream.go`（`make upstream TO=shadcn@X.Y.Z`）。CI 那半邊的問題隨
workflow 一起消失了，但這些還在：
- `reproducible` 在任何真 re-pin 必紅，drill 卻要求 0 failed
- 分類器用 log 尾巴的子字串比對
- `package.json` 的 55 個 `@radix-ui/*` overrides 與 `vendor/radix-kernel` 在 radix 升版時
  **沒有任何東西**在比對
- `EXEMPTIONS.md` 從不 `--render`

### 4.7 converter 的靜默空頁
- `forwardRef` / HOC 元件產出 0 element（`src/converter/index.mjs:425-446`）
- `import { cva as x }` 讓 variant 全消失（`:90`, `:641`）
- skin 巢狀規則被 `parseSkinMap:41` 靜默漏掉

（docs transform 那三個靜默跳過今天修掉了；`src/registry/ir/form.json` 孤兒在 `54cf18c`
刪了。）

### 4.8 `has-[&gt;svg]:gap-x-2` 的消費者面 bug
出貨的 markup 帶 HTML-escaped class。我們自己編得出來是因為掃 IR；**消費者複製那段 markup
之後，他們自己的 tailwind scan 掃不到**，那些 class 不會被編進去。還沒處理。

### 4.9 VitePress 的後續
- **iframe 不跟站台的明暗切換走**：demo 頁讀 `shadless-docs-theme` 這個 localStorage key，
  VitePress 用自己的。要接就在 theme 裡監聽 VitePress 的 appearance 再寫那個 key。
- **markup fence 目前是 `text` 不是 `html`**（`tools/docs-build.mjs:213-214`）。量過的代價：
  `text` = 42MB dist / 27s build / 85KB per page；`html` = 58MB / 90s / 145KB。改一個字切換。
- **沒有部署**。CI 全刪了，`docs/.vitepress/dist` 只是本機產物。要上線得自己決定怎麼發。
- `docs-fidelity` 對 text transform 來說變弱了（fence 比對幾乎是恆真）——程式碼裡寫明了，
  但值不值得留著要重新判斷。

### 4.10 其他清理
- 重複實作：fence stripping ×3、guide 清單兩份且不一致（`docs-guides.mjs:29` vs
  `pipeline/docs_catalog.go`）、grey 清單兩份（`tools/docs-build.mjs:71` vs docs-catalog）
- `src/emitter/css.mjs:101-391` `componentCss` 290 行七種職責；IR 的 `children` 是字串草圖
  再用 regex 重解析（`emitter/index.mjs:32-36`）
- parity baseline 的 `flaky[]` 沒有預算
- 沒有任何 gate 在看 inline style
- `vendor/radix-kernel.iife.js` ~2,355 行手寫行為複刻，原始碼不在 repo；`@floating-ui` 沒 pin；
  embla 的 entry `tools/.embla-entry.mjs` 不存在也沒 sha pin；`SECURITY.md:25` 描述錯誤

---

## 5. 舊交接裡已經不適用的東西

wireit、`gates/registry.mjs`、`gates/meta.mjs`、`gates/wireit.mjs`、`wireit-sync` gate、
medium tier、`emit-smoke`、`.github/workflows/*`、`docs/site` 是否要追蹤的問題、
`docs-consistency` §1–3、`docs-links`、`docs-upstream`——**全部沒有了**。
`gates/` 只剩 `overlay.mjs`、三個 parity gate + 它們的 baseline、`ledger.json`、
`parity-baseline.mjs`。
