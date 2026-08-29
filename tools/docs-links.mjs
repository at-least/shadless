#!/usr/bin/env node
// docs-links.mjs — FT4 link check over the BUILT site (docs/site/).
// Crawls every *.html page, resolves every internal reference — <a href>
// (site-relative + /docs/… absolute), plus link/img/script/iframe src/href
// (asset regressions) and same-page #anchors — and reports the dangling
// count. External http(s)/mailto/data refs are out of scope (skipped).
// Absolute /docs/… hrefs that survive into the built HTML are dangling by
// definition: the build rewrites routable ones to relative .html and greys
// the rest (tools/docs-build.mjs + resolveDocsRoute in tools/docs-guides.mjs
// — shared table, no drift). Exit 0 iff dangling == 0.
import { readdirSync, readFileSync, existsSync, statSync } from 'node:fs'
import { join, dirname, resolve as resolvePath, basename } from 'node:path'
import { fileURLToPath } from 'node:url'
import { resolveDocsRoute } from './docs-guides.mjs'

const SITE = 'docs/site'

function walk(dir) {
  const out = []
  for (const f of readdirSync(dir).sort()) {
    const p = join(dir, f)
    if (statSync(p).isDirectory()) out.push(...walk(p))
    else if (f.endsWith('.html')) out.push(p)
  }
  return out
}

const pages = walk(SITE).filter((p) => !p.startsWith(`${SITE}/components/`))
// demo component pages (docs/site/components/*.html) are oracle-rendered
// upstream examples loaded in iframes — their hrefs are upstream demo
// payload (/login, #link, /docs/components …) in an inline-render context
// upstream; they are not shadless site navigation and are out of scope.
const rootSlugs = new Set(readdirSync(SITE).filter((f) => f.endsWith('.html')).map((f) => f.replace(/\.html$/, '')))

const dangling = []
let checked = 0

for (const page of pages) {
  const html = readFileSync(page, 'utf8')
  const ids = new Set([...html.matchAll(/\sid="([^"]+)"/g)].map((m) => m[1]))
  const refs = [...html.matchAll(/<(a|link|img|script|iframe)\b[^>]*?\b(?:href|src)="([^"]+)"/g)]
  for (const [, tag, url] of refs) {
    if (/^(https?:|mailto:|data:|\/\/|#)/i.test(url)) {
      // bare "#" is a no-op placeholder (demo convention), not an anchor ref
      if (url.startsWith('#') && url.length > 1 && !ids.has(url.slice(1))) {
        checked++
        dangling.push(`${page}: #${url.slice(1)} anchor missing`)
      }
      continue
    }
    checked++
    if (url.startsWith('/')) {
      if (url === '/') { continue } // site root = index.html by definition
      // site-root absolute (assets) or /docs/ routes
      const target = url.slice(1).split('#')[0].split('?')[0]
      if (target && existsSync(join(SITE, target))) continue
      const route = resolveDocsRoute(url.split('#')[0].split('?')[0], rootSlugs)
      if (route && route.file && existsSync(join(SITE, route.file))) continue
      dangling.push(`${page}: <${tag}> ${url} — no such page/asset`)
    } else {
      const target = url.split('#')[0].split('?')[0]
      if (target && existsSync(resolvePath(dirname(page), target))) continue
      dangling.push(`${page}: <${tag}> ${url} — no such page/asset`)
    }
  }
}

for (const d of dangling) console.error(`DANGLING ${d}`)
console.log(`docs links: ${checked} internal refs checked across ${pages.length} pages — dangling: ${dangling.length}`)
if (dangling.length > 0) {
  console.error('FAIL  docs links (dangling internal references)')
  process.exit(1)
}
console.log('PASS  docs links (0 dangling internal refs)')
