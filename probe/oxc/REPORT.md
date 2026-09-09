# Oxc probe — oxc in-process vs pinned esbuild CLI(2026-09-10)

對應 PLAN.md「為什麼不是 Rolldown/Oxc」掛帳的 feature-flag 實驗第一輪:
M0 的問題反過來問一次——**純 Rust oxc(0.149.0)離 esbuild Transform 的
輸出有多遠?** 輸入與 M0 相同:`build/resolved-ui/ui/*.tsx`(61 檔,
遞迴列舉排序;上游 pin 見 `src/registry/pin.json`);esbuild 側為
`node_modules` pinned **0.28.2**(node_modules 內容未納入指紋基線,
重現時以 `node_modules/esbuild/package.json` 為準)。

## 執行

    cargo build --release --features oxc
    cd ../shadless && ../shadless-rs/target/release/pipeline __oxc-probe [--normalize]

A 側 = 現行 `convert::esbuild_tsx`(六 flags,probe/m0 已證 == Go
api.Transform);B 側 = oxc parser → semantic → transformer(JSX classic
runtime、`React.createElement`/`React.Fragment`)→ codegen(2 空格縮排、
丟 normal/jsdoc 註解)。`--normalize` 把兩側所有空白剝掉再比,
分離「印表機策略」與「transform 語意」兩類分歧。

## 結果(第一輪)

    == oxc vs esbuild: 0/61 byte-identical, 29 whitespace-only, 32 diverged, 0 oxc errors

- **0/61 位元組一致**——PLAN 的預測直接命中:獨立 printer 不可能在
  esbuild 的斷行/填行策略上同位元組。oxc 用自己的換行策略(不合併
  esbuild 會摺的物件/參數列),`indent_char/indent_width` 對齊縮排後
  仍差在斷行。
- **29/61「剝除所有空白後一致」**——注意用語:normalize 連字串/JSX
  文字內部的空白也剝(兩側同樣剝,所以是必要條件非充分條件),此類
  僅保證「非空白位元組序列一致」,不等於 JSX 語意完全等價(如 JSX
  children 內空白摺疊差異會被歸入此類)。JSX transform、TS strip、
  `/* @__PURE__ */` 標記的形狀在語意上對齊。
- **32/61 有真實(非空白)分歧**,三類,全部是印表機/序層:
  1. **export specifier 順序**:esbuild 依原始碼序;oxc 不同序
     (分歧上下文統計 23/32 落在 export 語句,為上下文窗口估計值;
     例如 `export{Accordion,AccordionContent,AccordionItem,…}` vs
     `export{Accordion,AccordionItem,AccordionTrigger,…}`)。
  2. **`undefined` → `void 0`**:esbuild 印 `void 0`(ternary 位置等);
     oxc 保留 `undefined`。
  3. **遮蔽綁定改名**:esbuild 對遮蔽外層的內層綁定改名
     (`(api)=>`→`(api2)=>`、`(open)=>`→`(open2)=>`、`className`→`className2`);
     oxc 保留原名。
- 修掉的第四類:oxc 預設保留 normal/jsdoc 註解,esbuild transform 會丟
  (`comments.normal=false, jsdoc=false` 對齊;legal/pure 保留)。

## 判讀(對照 PLAN 的兩條路)

- **路線 (a) transform 替換(convert.go 的 esbuildTsx)——不可行**,
  與 PLAN 預測一致:位元組契約(掃描器 regex 逐字消費輸出,連遮蔽
  改名 `api2` 這種形狀都餵進 regex)在 0/61 的現實下沒有旗標可救。
  export 順序與改名都不是 CodegenOptions 可配置項。
- **路線 (c) oracle api.Bundle 點——已實作並通過驗收(見下輪)**。
- minify(`--minify --target=es2017`)尚未探:優先級低於以上判讀,
  同樣受位元組契約約束(dist/shadless.min.js 是 committed 產品)。

## 第二輪:oracle bundle 點的 rolldown 替換(2026-09-10,通過)

`src/oracle/oxc_bundle.rs`(cargo feature `oxc`)+ `SHADLESS_ORACLE_BUNDLER=oxc`
運行時閘(閘在 oracle 群組內——Cargo feature 進 hull,切換即全圖 stale,
違反隔離目的)。rolldown 1.2.8:`BundlerBuilder` → `BundlerOptions`
(input/cwd/format=Iife/resolve.alias)→ `generate()` 取單一 JS chunk。

與 esbuild 對齊的三件事(缺一即失敗,均由 227 頁驗收逼出):

1. **alias 最長匹配優先**:alias 表含 `@` catch-all(指向 upstream app
   目錄);oxc_resolver 走 webpack 語意「第一個符合者勝」,按鍵長度降序
   餵入才等價於 esbuild 的最長匹配。
2. **關閉 tsconfig 自動探測**(`TsConfig::Auto(false)`):rolldown 會沿
   importer 樹自動發現 tsconfig.json 並套用其 paths——upstream app 的
   `@/*` 映射壓過我們的 alias 表,把無法解析的 app 依賴(streamdown)
   拖進 bundle → 頁面 ReferenceError。esbuild 不讀 tsconfig,故無此題。
   (診斷線索:esbuild bundle 裡 `Markdown` 是 stub,rolldown bundle 卻
   含 upstream markdown.tsx。)
3. **NODE_ENV**:rolldown 對 Browser 平台自動定義 `process.env.NODE_ENV`
   (非 minify → development),規則與 esbuild 相同,react 走同一分支,
   無需 define。

快取語意:oracle 快取為 Go/RS 兩引擎共用、key 必須與 Go 位元組同值,
故 oxc 路徑使用**獨立**的 outfile(`oxc-bundle-<name>.js`)與 key 檔
(`.oxc-key-<name>`;前綴避開 `bundle-*.js` glob),不污染共用快取。
gate 在快取查找之前檢查:feature-less binary + 環境變數一律報錯,
暖快取不會被靜默沿用。

驗收:`SHADLESS_ORACLE_BUNDLER=oxc pipeline example-oracle --check`
**PASS 227 pages == oracle render**;預設 esbuild 路徑同跑 PASS(無回歸)。
bundle 大小對照(bubble-variants):esbuild 1,399,996 B vs rolldown
約 1.14 MB(渲染結果等價,位元組本就不要求一致)。

## 基線(指紋隔離的前置,顧問要求)

- `baseline-fps.txt`:實驗開工前的全套群組指紋(hull/convert/emit/…/global)。
- `baseline-status.txt`:68 fresh + 1 NEVER-FRESH。
- 新增 oxc 依賴改 Cargo.lock = hull 編輯 → 一次性全圖收斂後,迭代只應
  移動 tools 群組(oxc_probe.rs 所在);若 status 顯示其他群組節點
  無端 stale,隔離假設即被證偽。
