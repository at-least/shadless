// Resolve (with --install: build and copy) the pipeline binary,
// CARGO_TARGET_DIR-aware: `cargo build --release` writes to $CARGO_TARGET_DIR
// when set, so a copy from pipeline/target/release/pipeline silently grabs a
// stale binary. Resolve the real location from cargo's own metadata instead.
import { execSync } from "node:child_process"
import { fileURLToPath } from "node:url"
import { mkdirSync, copyFileSync } from "node:fs"

const repo = fileURLToPath(new URL("..", import.meta.url))
const crate = repo + "pipeline/"
const meta = JSON.parse(
  execSync("cargo metadata --format-version 1 --no-deps", {
    cwd: crate,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "inherit"],
  }),
)
const bin = meta.target_directory + "/release/pipeline"

if (process.argv.includes("--install")) {
  execSync("cargo build --release", { cwd: crate, stdio: "inherit" })
  mkdirSync(repo + "build", { recursive: true })
  copyFileSync(bin, repo + "build/pipeline")
} else {
  console.log(bin)
}
