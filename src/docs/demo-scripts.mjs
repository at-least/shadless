// Shared demo-page script extraction (Wave H D3).
// Two consumers (components.mjs demoSources → docs "JS" tab, docs-build
// manualTabMdx → installation copy list) previously ran their own regexes;
// both accidentally captured the FT9 theme pre-paint boilerplate as "demo
// JS", polluting every JS tab and making `inlineInit` always-true.
//
// extractDemoScripts(html):
//   srcScripts    — local script srcs the demo loads (glue/*.js, shadless.js;
//                   vendor IIFEs/out.css are NOT local init and stay out)
//   inlineScripts — inline <script> bodies EXCLUDING the theme pre-paint
//                   boilerplate (identified by its storage-key signature)

const PREPAINT_SIG = 'shadless-docs-theme'

export function extractDemoScripts(html) {
  const srcScripts = []
  for (const m of html.matchAll(/<script src="\.\.\/(glue\/[\w.-]+\.js|shadless\.js)"><\/script>/g))
    srcScripts.push(m[1])
  const inlineScripts = []
  for (const m of html.matchAll(/<script>([\s\S]*?)<\/script>/g)) {
    const body = m[1].trim()
    if (!body) continue
    if (body.includes(PREPAINT_SIG)) continue // theme persistence, not demo JS
    inlineScripts.push(body)
  }
  return { srcScripts, inlineScripts }
}
