// Browser entry for docs code highlighting — bundled by tools/docs-build.mjs
// (esbuild, IIFE) into docs/site/highlight.js. Self-contained: the two
// upstream themes + the seven fence languages + shiki's JavaScript regex
// engine, no wasm, no network — the docs gates run offline.
//
// Why client-side: highlighting is presentation only. Pre-rendering it at
// build time made every token a <span style="--shiki-light:…;--shiki-dark:…">
// and turned docs/site into 137 MB of markup (a 5.9× blow-up over the code
// text, ×70 pages). The build now ships plain text inside the same
// <span data-line> line structure (gutter, counters, textContent and every
// grep-based gate unchanged); this file colours the tokens after load,
// emitting the exact inline vars the build used to, so site.css and
// tools/docs-upstream.mjs (palette ⊆ theme) still hold.
import { createHighlighterCoreSync } from 'shiki/core'
import { createJavaScriptRegexEngine } from 'shiki/engine/javascript'
import tsx from '@shikijs/langs/tsx'
import ts from '@shikijs/langs/typescript'
import css from '@shikijs/langs/css'
import diff from '@shikijs/langs/diff'
import json from '@shikijs/langs/json'
import html from '@shikijs/langs/html'
import js from '@shikijs/langs/javascript'
import light from '@shikijs/themes/github-light-default'
import dark from '@shikijs/themes/vesper'

const LIGHT_THEME = 'github-light-default'
const DARK_THEME = 'vesper'
const norm = (c) => (c ? c.toLowerCase() : c)

let highlighter = null
function getHighlighter() {
  if (!highlighter) {
    highlighter = createHighlighterCoreSync({
      themes: [light, dark],
      langs: [tsx, ts, css, diff, json, html, js],
      engine: createJavaScriptRegexEngine(),
    })
  }
  return highlighter
}

// Colour one <code data-lang> block in place. Lines are the build's
// <span data-line> children; each is rewritten with token spans whose
// textContent (incl. the trailing '\n') is exactly what it held before.
function highlightBlock(code) {
  const lang = code.getAttribute('data-lang')
  if (!lang || code.dataset.highlighted) return
  const lineEls = [...code.querySelectorAll(':scope > span[data-line]')]
  if (!lineEls.length) return
  const h = getHighlighter()
  const lightFg = h.getTheme(LIGHT_THEME).fg
  const darkFg = h.getTheme(DARK_THEME).fg
  // strip the per-line trailing '\n' the build adds; shiki splits on '\n'
  const text = lineEls.map((l) => l.textContent.replace(/\n$/, '')).join('\n')
  let tokens
  try {
    tokens = h.codeToTokens(text, {
      lang, themes: { light: LIGHT_THEME, dark: DARK_THEME }, defaultColor: false,
    }).tokens
  } catch {
    return // unknown grammar or engine limit: leave the plain text
  }
  if (tokens.length !== lineEls.length) return
  lineEls.forEach((lineEl, i) => {
    const frag = document.createDocumentFragment()
    for (const t of tokens[i]) {
      const s = t.htmlStyle ?? {}
      const lt = s['--shiki-light']
      const dk = s['--shiki-dark']
      const fs = `${s['--shiki-light-font-style'] ?? ''}${s['--shiki-dark-font-style'] ?? ''}`
      const italic = /italic/.test(fs)
      const bold = /bold/.test(fs)
      if (norm(lt) === norm(lightFg) && norm(dk) === norm(darkFg) && !italic && !bold) {
        frag.appendChild(document.createTextNode(t.content))
        continue
      }
      const span = document.createElement('span')
      if (lt) span.style.setProperty('--shiki-light', lt)
      if (dk) span.style.setProperty('--shiki-dark', dk)
      if (italic) span.style.fontStyle = 'italic'
      if (bold) span.style.fontWeight = 'bold'
      span.textContent = t.content
      frag.appendChild(span)
    }
    frag.appendChild(document.createTextNode('\n'))
    lineEl.replaceChildren(frag)
  })
  code.dataset.highlighted = 'true'
}

function highlightAll(root = document) {
  for (const code of root.querySelectorAll('pre code[data-lang]')) highlightBlock(code)
  document.documentElement.dataset.highlighted = 'true'
}

if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', () => highlightAll())
else highlightAll()
