// frontmatter + stripImports (src/docs/frontmatter.mjs) — migrated from
// tools/unit-check.mjs 2026-08-26, assertions unchanged.
import { parseFrontmatter, stripImports } from "../../src/docs/frontmatter.mjs"

export function run(t) {
  t.eq("frontmatter: basic", parseFrontmatter("---\ntitle: Hi\ndesc: x\n---\nbody"), { title: "Hi", desc: "x" })
  t.eq("frontmatter: none", parseFrontmatter("# no frontmatter"), {})
  t.eq("frontmatter: CRLF tolerated", parseFrontmatter("---\r\ntitle: Hi\r\n---\r\nbody"), { title: "Hi" })
  t.eq("frontmatter: matched-quote pair stripped", parseFrontmatter('---\ntitle: "Hi"\n---'), { title: "Hi" })
  t.eq("frontmatter: trailing apostrophe kept", parseFrontmatter("---\ntitle: years'\n---"), { title: "years'" })
  t.eq("frontmatter: nested one level", parseFrontmatter("---\ntop:\n  sub: 1\n---"), { top: { sub: 1 } })
  t.eq("frontmatter: booleans/numbers coerced", parseFrontmatter("---\na: true\nb: false\nc: 42\n---"), { a: true, b: false, c: 42 })

  t.eq("strip: bare import does not swallow prose", stripImports('import "a";\n\n# Heading\n\nPara.\n\nimport { x } from "b";\n'),
    "\n\n# Heading\n\nPara.\n\n\n")
  t.eq("strip: multiline named import", stripImports("import {\n  A,\n  B,\n} from \"pkg\";\nrest"), "\nrest")
  t.eq("strip: default + namespace", stripImports('import x from "p";\nimport * as ns from "q";\ntext'),
    "\n\ntext")
  // NOTE: stripImports is fence-AGNOSTIC by design — fence-aware stripping
  // lives in docs-build's stripImportsOutsideFences. Do not assert fence
  // behavior here.
}
