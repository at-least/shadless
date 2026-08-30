// fs-record.mjs — records the files a JS tool READS, for the pipeline's
// undeclared-access check. Loaded via NODE_OPTIONS=--import by the runner;
// no tool imports it and no tool knows it exists.
//
// Why this exists: `inputs` in pipeline/nodes.go is hand-maintained, and a
// glob that misses a file the node actually reads is a stale GREEN — the file
// is not in the node's key, so changing it leaves the node fresh over a tree
// it would now build differently. The Go gates get this checked for free from
// `go test -test.testlogfile`; the JS nodes are the larger half of the graph
// and had nothing.
//
// How it works, and why it is not the obvious thing: patching the ESM default
// export (`import fs from "node:fs"; fs.readFileSync = ...`) does NOT affect
// `import { readFileSync } from "node:fs"`, which is how every tool here reads
// files — Node snapshots the builtin's default export separately. Patching the
// CJS exports object via createRequire DOES, because the ESM named exports are
// live bindings onto it. That difference is the whole trick.
//
// NODE_OPTIONS is inherited by child processes, so a tool that spawns another
// node tool is recorded too, with no extra wiring.
//
// Reads only. Writes are the undeclared-WRITE check's job, and keeping this
// one-sided means a path in the report always means the same thing.

import { createRequire } from "node:module"
import { resolve } from "node:path"

const require = createRequire(import.meta.url)
const fs = require("fs")
const log = process.env.SHADLESS_FSLOG

if (log) {
  // Buffered, flushed once at exit: docs-build opens thousands of files and an
  // appendFileSync per call would show up in the build time. A Set also means
  // the report is not 3000 copies of the same path.
  const seen = new Set()
  const real = { appendFileSync: fs.appendFileSync }

  const note = (p) => {
    // p may be a file descriptor (number), a Buffer, or a URL; only string
    // paths are meaningful to the check, and resolve() pins them against the
    // cwd at call time in case a tool changed directory.
    if (typeof p !== "string") return
    try { seen.add(resolve(p)) } catch {}
  }

  // The read surface the tools actually use. existsSync/statSync count: a tool
  // that branches on a file's presence is judging that file, and its
  // appearance must invalidate the node just as its content would.
  for (const name of ["readFileSync", "readdirSync", "existsSync", "statSync",
                      "lstatSync", "openSync", "realpathSync", "readlinkSync",
                      "createReadStream", "opendirSync"]) {
    const orig = fs[name]
    if (typeof orig !== "function") continue
    fs[name] = function (p, ...rest) {
      note(p)
      return orig.call(this, p, ...rest)
    }
  }

  // node:fs/promises is a separate object with its own function identities.
  try {
    const fsp = require("fs/promises")
    for (const name of ["readFile", "readdir", "stat", "lstat", "open", "access"]) {
      const orig = fsp[name]
      if (typeof orig !== "function") continue
      fsp[name] = function (p, ...rest) {
        note(p)
        return orig.call(this, p, ...rest)
      }
    }
  } catch {}

  const flush = () => {
    if (seen.size === 0) return
    try {
      real.appendFileSync.call(fs, log, [...seen].join("\n") + "\n")
    } catch {}
    seen.clear()
  }
  process.on("exit", flush)
}
