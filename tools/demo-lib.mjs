// Pure path-rewrite helpers extracted from tools/demo.mjs (Wave H) so the
// 2026-08-25 incident class (a rewrite regex deleted as "dead code" — the
// bare `out.css` form is NOT covered by the `[^"]*-out\.css` pattern) has a
// seconds-level regression net.

// path rewrites: probe-relative assets → dist-relative
export function rewritePaths(html) {
  return html
    // per-component compiled css (t6: <link href="tooltip-out.css">) → unified
    // out.css; the bare form is a separate replace (the `[^"]*-` prefix is
    // mandatory in the first pattern — it must NOT match plain "out.css")
    .replace(/(<link[^>]*href=")[^"]*-out\.css(")/g, `$1../out.css$2`)
    .replace(/(<link[^>]*href=")out\.css(")/g, `$1../out.css$2`)
    // generated contract fixtures (tools/example-fixture.mjs --contracts) link dist directly
    .replace(/(<link[^>]*href=")\.\.\/\.\.\/dist\/out\.css(")/g, `$1../out.css$2`)
    // the JS surface: fixtures link dist directly → dist-relative
    .replace(/(<script[^>]*src=")\.\.\/\.\.\/dist\/shadless\.js(")/g, `$1../shadless.js$2`)
    .replace(/(<script[^>]*src=")\.\.\/\.\.\/dist\/js\/([\w-]+\.js)(")/g, `$1../js/$2$3`)
}

// ensure a stylesheet link exists (t7 fixtures ship without one)
export function ensureLink(html) {
  if (/<link[^>]*out\.css/.test(html)) return html
  return html.replace(/(<head>)/, `$1\n<link rel="stylesheet" href="../out.css">`)
}
