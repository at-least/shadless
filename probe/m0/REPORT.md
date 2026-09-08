# M0 probe — esbuild CLI vs Go api.Transform(2026-09-07)

結論:**route (a) 成立**。Rust spawn pinned `node_modules/.bin/esbuild`(0.28.2)可得
與 Go pipeline in-process `api.Transform` 位元組一致的輸出。M0 通過,進 M1。

## 方法

`main.go` 逐字複製兩個被測呼叫點的 options(`shadless/pipeline/convert.go` 的
`esbuildTsx`、`jsbuild.go` 的 `minify`),對相同輸入產 Go 端 golden;CLI 端以 stdin
transform 模式對同輸入重跑;`diff -r` 逐位元組比較。輸入:`build/resolved-ui/ui/*.tsx`
(61 檔)+ `vendor/radix-kernel.iife.js` 與 `src/runtime/core.js` 依 `iifeBase` 串接
(`kernel + "\n;\n" + core`)。

CLI flags(transform,與 `esbuildTsx` 一一對應):

    esbuild --loader=tsx --jsx=transform --jsx-factory=React.createElement \
            --jsx-fragment=React.Fragment --format=esm --charset=utf8 < in.tsx > out.js

CLI flags(minify,與 `minify` 對應;`FormatDefault` = 不給 `--format`):

    esbuild --minify --target=es2017 < iife-base.js > out.js

## 結果(節錄自執行輸出)

    probe: wrote 61 transform outputs to .../go-out
    probe: wrote minify output (75503 bytes) to .../go-out-min

    non-empty err files: 0
    TRANSFORM: 61/61 byte-identical
    MINIFY: byte-identical
    MINIFY cmp: identical, 75503 bytes

End-to-end 封環(committed 產物 == probe 輸出 == CLI 輸出):

    cmp dist/shadless.min.js go-out-min/shadless.min.js
    → dist/shadless.min.js == probe output (75503 bytes)

## 附註

- advisor 預告的「尾端換行差異」未出現:CLI stdout 與 `res.Code` 逐位元組相同,
  wrapper 不需補正。
- `api.Build`(oracle bundle,example_fixture/oracle_lib)未在此 probe 範圍;
  該路徑無位元組要求(輸出為 temp、行為由瀏覽器 gates 驗證),依計畫可於後續
  里程碑直接以 CLI build mode 替換。

## 錯誤路徑(後補測試,2026-09-07)

刻意破損的 .tsx 餵兩邊:

    GO-ERR: Expected ">" but found end of file   (go exit: 1)
    ✘ [ERROR] Expected ">" but found end of file … (cli exit: 1)

訊息本體一致(CLI 另有 caret 格式上下文)。並已確認上游 `runConvert`
(convert.go:3014)遇 transform 失敗只印 stderr 並 return 1,**錯誤文字不流入任何
committed 產物** —— Rust wrapper 的契約因此是「非零結束 + stderr 有診斷」,
不需逐字對齊錯誤格式。

## 後續條件(done checkpoint 2026-09-07)

1. `api.Build` probe 升格為正式 gate,排在第一個移植 Build 消費節點
   (oracle_lib/example_fixture)的里程碑**之前**,用同樣 golden-diff 法;
   Build 是多輸入多輸出,騎不了 stdin→stdout,scaffold 要為它留 temp-dir 慣例。
2. M1:位元組一致鎖定在 esbuild 0.28.2;Rust 端是 spawn,故 node key 必須折入
   `esbuild --version`(或啟動時對 lockfile pinned binary 斷言),否則日後
   node_modules 升級會產生 silent stale-cache hit 或假差異。

重現方式:`cd probe/m0 && GOPROXY=off go run .`,再執行本檔所列 CLI 迴圈與 `diff -r`;
錯誤路徑重現見 /tmp/errprobe(破損輸入 + 兩端各跑一次)。
