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
- **路線 (c) oracle api.Bundle 點——仍開放**:該點輸出是暫存、無位元組
  要求(驗收只看 oracle render 結果),oxc/rolldown 可行性不受本輪
  結論影響。
- minify(`--minify --target=es2017`)尚未探:優先級低於以上判讀,
  同樣受位元組契約約束(dist/shadless.min.js 是 committed 產品)。

## 基線(指紋隔離的前置,顧問要求)

- `baseline-fps.txt`:實驗開工前的全套群組指紋(hull/convert/emit/…/global)。
- `baseline-status.txt`:68 fresh + 1 NEVER-FRESH。
- 新增 oxc 依賴改 Cargo.lock = hull 編輯 → 一次性全圖收斂後,迭代只應
  移動 tools 群組(oxc_probe.rs 所在);若 status 顯示其他群組節點
  無端 stale,隔離假設即被證偽。
