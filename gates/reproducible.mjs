#!/usr/bin/env node
// gates/reproducible.mjs — committed generated trees == what the pipeline
// just produced.
//
// This is the ONLY authority on hand-edits to generated files. The old
// pre-commit hook tried to guess ("vendor outputs staged without a source
// change") with three layers of heuristics and a 6-second rebuild, and
// still had to be bypassed for legitimate commits (5e39ace). Guessing is
// gone: run the pipeline, then the tree must be clean under the generated
// paths. If it is not, either a source change legitimately moved an output
// (commit both together) or someone edited an output by hand (their edit
// was just overwritten — the diff shows exactly where).
//
// Keeping dist/ and docs/site/ in git is deliberate: a re-pin PR's most
// useful review surface is the diff of what consumers actually receive.
import { execFileSync } from "node:child_process"

export const GENERATED = [
  "dist", "docs/site", "docs/catalog.json", "docs/demos", "docs/example-oracle.json",
  "src/registry/ir", "tools/contracts/out", "src/kernel/*.html",
]

const status = execFileSync("git", ["status", "--porcelain", "--untracked-files=all", "--", ...GENERATED], { encoding: "utf8" })
  .split("\n").filter(Boolean)
if (status.length) {
  console.error(`FAIL  reproducible (${status.length} generated paths differ from the committed tree)\n  ` +
    status.slice(0, 40).join("\n  ") + (status.length > 40 ? `\n  … +${status.length - 40} more` : "") +
    `\n\n  The pipeline produced different bytes than what is committed under the generated paths.` +
    `\n  - a source change moved an output  → commit source + output together (git add -A)` +
    `\n  - a generated file was hand-edited → the edit is gone; put it in the tool or overlay that owns the file` +
    `\n  Inspect: git diff -- <path>`)
  process.exit(1)
}
console.log(`PASS  reproducible (${GENERATED.length} generated roots match the committed tree)`)
