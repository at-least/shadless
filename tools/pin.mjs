#!/usr/bin/env node
// pin.mjs — record + verify upstream pins (shadcn-ui clone, kernel IIFE).
// The shadcn pin must sit on a stable release tag (shadcn@*), not an arbitrary
// main commit — recording the tag name makes that explicit and lets drift gates
// catch a pin that slips onto an unreleased commit. Writes src/registry/pin.json;
// exits non-zero on drift vs existing pin, or if HEAD is not at a shadcn@* tag.
import { execSync } from "node:child_process"
import { createHash } from "node:crypto"
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs"

// --check-only: verify the EXISTING pin.json against the upstream checkout
// without recording anything (used by `npm run verify` — catches the
// tag(old)/commit(new) inconsistency upgrade tools can write directly).
const checkOnly = process.argv.includes("--check-only")
// --force: re-record the pin at the upstream checkout's HEAD even when it
// differs from the existing pin.json (the re-pin drill, gates/upstream.mjs).
// Without it, drift is an error — a stray checkout must not silently move
// the pin under a normal run.
const force = process.argv.includes("--force")

const shadcnDir = ".upstream/shadcn-ui"
const kernelIife = "vendor/radix-kernel.iife.js"

if (!existsSync(shadcnDir)) {
  console.error(`PIN FAIL: ${shadcnDir} not found — clone the upstream first:`)
  console.error(`  git clone https://github.com/shadcn-ui/ui ${shadcnDir}`)
  console.error(`  git -C ${shadcnDir} checkout shadcn@4.19.0`)
  process.exit(1)
}

if (checkOnly) {
  const pinFile0 = "src/registry/pin.json"
  if (!existsSync(pinFile0)) {
    console.error("PIN FAIL: src/registry/pin.json missing — run npm run pin")
    process.exit(1)
  }
  const recorded = JSON.parse(readFileSync(pinFile0, "utf8"))
  const head = execSync(`git -C ${shadcnDir} rev-parse HEAD`, { encoding: "utf8" }).trim()
  let fail = false
  if (recorded.shadcn_ui.commit !== head) {
    console.error(`PIN FAIL: pin.json commit ${recorded.shadcn_ui.commit.slice(0, 10)} != upstream HEAD ${head.slice(0, 10)}`)
    fail = true
  }
  if (!/^shadcn@\d/.test(recorded.shadcn_ui.tag ?? "")) {
    console.error(`PIN FAIL: pin.json tag "${recorded.shadcn_ui.tag}" is not a shadcn@* release tag`)
    fail = true
  }
  const kernelSha = createHash("sha256").update(readFileSync(kernelIife)).digest("hex")
  if (recorded.kernel?.sha256 !== kernelSha) {
    console.error(`PIN FAIL: kernel sha256 drift (pin.json ${recorded.kernel?.sha256?.slice(0, 12)}… != vendor ${kernelSha.slice(0, 12)}…)`)
    fail = true
  }
  if (fail) process.exit(1)
  console.log(`pin OK (check-only): shadcn=${recorded.shadcn_ui.tag} (${head.slice(0, 10)}) kernel=${kernelSha.slice(0, 12)}…`)
  process.exit(0)
}

const commit = execSync(`git -C ${shadcnDir} rev-parse HEAD`, { encoding: "utf8" }).trim()
// tags whose target commit == HEAD (peels annotated tags to the commit they point at)
const tagsAtHead = execSync(
  `git -C ${shadcnDir} for-each-ref --format='%(refname:short)' --points-at HEAD refs/tags`,
  { encoding: "utf8" }
).split("\n").map((s) => s.trim()).filter(Boolean)
const releaseTag = tagsAtHead.find((t) => /^shadcn@\d/.test(t))
if (!releaseTag) {
  console.error(`PIN FAIL: upstream HEAD ${commit.slice(0, 10)} is not a shadcn@* release tag`)
  console.error(`  tags at HEAD: ${tagsAtHead.length ? tagsAtHead.join(", ") : "(none)"}`)
  console.error(`  checkout a shadcn@* tag before pinning (e.g. git -C ${shadcnDir} checkout shadcn@4.19.0)`)
  process.exit(1)
}
const sha = p => createHash("sha256").update(readFileSync(p)).digest("hex")

const pin = {
  shadcn_ui: { repo: "https://github.com/shadcn-ui/ui", tag: releaseTag, commit,
    registry: "apps/v4/registry/bases/radix/ui" },
  kernel: { file: kernelIife, sha256: sha(kernelIife) },
  recorded: new Date().toISOString().slice(0, 10),
}

mkdirSync("src/registry", { recursive: true })
const pinFile = "src/registry/pin.json"
if (existsSync(pinFile)) {
  const old = JSON.parse(readFileSync(pinFile, "utf8"))
  const drift = old.shadcn_ui.commit !== pin.shadcn_ui.commit ||
    old.kernel.sha256 !== pin.kernel.sha256
  if (drift && force) {
    writeFileSync(pinFile, JSON.stringify(pin, null, 2) + "\n")
    console.log(`pin re-recorded: shadcn=${old.shadcn_ui.tag} -> ${releaseTag} (${commit.slice(0, 10)}) kernel=${pin.kernel.sha256.slice(0, 12)}…`)
    process.exit(0)
  }
  if (drift) {
    console.error("PIN DRIFT detected:")
    if (old.shadcn_ui.commit !== pin.shadcn_ui.commit)
      console.error(`  shadcn: ${old.shadcn_ui.commit} -> ${pin.shadcn_ui.commit}`)
    if (old.kernel.sha256 !== pin.kernel.sha256)
      console.error(`  kernel sha256: ${old.kernel.sha256} -> ${pin.kernel.sha256}`)
    process.exit(1)
  }
  console.log(`pin OK: shadcn=${releaseTag} (${commit.slice(0, 10)}) kernel=${pin.kernel.sha256.slice(0, 12)}…`)
} else {
  writeFileSync(pinFile, JSON.stringify(pin, null, 2) + "\n")
  console.log(`pin recorded: shadcn=${releaseTag} (${commit.slice(0, 10)}) kernel=${pin.kernel.sha256.slice(0, 12)}…`)
}
