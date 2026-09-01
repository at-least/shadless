#!/usr/bin/env node
// prettier-batch — the docs build's ONE node dependency. The Go builder
// (pipeline docs-build) sends a JSON array of {file, body} pairs on stdin
// (body = the demo page's <body> content, scripts already stripped) and
// reads a JSON map {file: formatted} from stdout. prettier's html printer is
// whitespace-sensitivity aware; no Go formatter matches it byte-for-byte,
// and the demo markup shown on every page is its output.
import prettier from "prettier"

let input = ""
process.stdin.setEncoding("utf8")
process.stdin.on("data", (c) => (input += c))
process.stdin.on("end", async () => {
  const items = JSON.parse(input)
  const out = {}
  for (const { file, body } of items) {
    out[file] = (await prettier.format(body, { parser: "html", printWidth: 100 })).trim()
  }
  process.stdout.write(JSON.stringify(out))
})
