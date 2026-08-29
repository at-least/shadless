// FT2 docs: code highlighting (build-time, React-free). Shiki is initialized
// once (sync tokenization afterwards) and exposed as a rehype plugin over the
// code elements MDX produces.
//
// Upstream parity (ui.shadcn.com): DUAL themes — github-light-default for
// light mode, vesper for dark — emitted as inline --shiki-light/--shiki-dark
// vars on token spans (site.css switches on :root.dark), plus line numbers:
// every line is a <span data-line> inside a code[data-line-numbers] with
// data-line-numbers-max-digits; the gutter is a CSS counter ::before.
//
// bash/text fences are still left as plain escaped text runs (they only get
// the line-span wrapping): install-command fences are copy-paste payloads
// that the acceptance gate greps verbatim ("npx shadcn@latest add dialog").
// Line wrapping preserves textContent exactly, so those greps are unaffected.
import { createHighlighter } from 'shiki'

const LIGHT_THEME = 'github-light-default'
const DARK_THEME = 'vesper'
const LANGS = ['tsx', 'ts', 'css', 'diff', 'json', 'html', 'js']
const LANG_MAP = {
  tsx: 'tsx', typescript: 'ts', ts: 'ts', css: 'css', diff: 'diff',
  json: 'json', html: 'html', js: 'js', jsx: 'tsx',
}

// Top-level await: any importer (rehype plugin here, components.mjs source
// blocks) waits for the highlighter before tokenizing.
const highlighter = await createHighlighter({ themes: [LIGHT_THEME, DARK_THEME], langs: LANGS })
const LIGHT_FG = highlighter.getTheme(LIGHT_THEME).fg
const DARK_FG = highlighter.getTheme(DARK_THEME).fg

// Single-pass dual-theme tokenization (themes option, defaultColor:false —
// a naive zip of two single-theme runs DOES drift: segmentation differs).
// Each token's htmlStyle carries --shiki-light/--shiki-dark (+ optional
// font-style vars). Plain tokens (theme fg in BOTH themes, no font style)
// are unwrapped so fences stay grep-friendly and output stays lean.
const norm = (c) => c ? c.toLowerCase() : c
export function tokenizeLines(code, lang) {
  if (!LANG_MAP[lang]) {
    return code.split('\n').map((l) => (l ? [{ content: l }] : []))
  }
  const { tokens } = highlighter.codeToTokens(code, {
    lang: LANG_MAP[lang],
    themes: { light: LIGHT_THEME, dark: DARK_THEME },
    defaultColor: false,
  })
  return tokens.map((lineTokens) => lineTokens.map((t) => {
    const s = t.htmlStyle ?? {}
    const light = s['--shiki-light']
    const dark = s['--shiki-dark']
    const italic = /italic/.test(`${s['--shiki-light-font-style'] ?? ''}${s['--shiki-dark-font-style'] ?? ''}`)
    const bold = /bold/.test(`${s['--shiki-light-font-style'] ?? ''}${s['--shiki-dark-font-style'] ?? ''}`)
    if (norm(light) === norm(LIGHT_FG) && norm(dark) === norm(DARK_FG) && !italic && !bold) {
      return { content: t.content }
    }
    return { content: t.content, light, dark, italic, bold }
  }))
}

export function lineNumbersAttrs(lineCount) {
  return {
    'data-line-numbers': '',
    'data-line-numbers-max-digits': String(Math.max(1, lineCount)).length,
  }
}

const tokenStyle = (tok) => [
  tok.light ? `--shiki-light:${tok.light}` : '',
  tok.dark ? `--shiki-dark:${tok.dark}` : '',
  tok.italic ? 'font-style:italic' : '',
  tok.bold ? 'font-weight:bold' : '',
].filter(Boolean).join(';')

// hast span for a token (used by the rehype plugin below)
const hastToken = (tok) => {
  if (!tok.light && !tok.italic && !tok.bold) return { type: 'text', value: tok.content }
  return {
    type: 'element', tagName: 'span',
    properties: { style: tokenStyle(tok) },
    children: [{ type: 'text', value: tok.content }],
  }
}

// Each line span ends with '\n' so textContent preserves newlines; with
// display:block + white-space:pre the trailing newline does not create a
// blank line (same rule that keeps pretty-printed <pre> tight).
function hastLines(lines) {
  const out = []
  lines.forEach((tokens, i) => {
    const children = tokens.map(hastToken)
    const last = children[children.length - 1]
    if (last && last.type === 'text') last.value += '\n'
    else children.push({ type: 'text', value: '\n' })
    out.push({
      type: 'element', tagName: 'span',
      properties: { 'data-line': '' },
      children,
    })
  })
  return out
}

export function createHighlightPlugin() {
  return function rehypeHighlight() {
    return (tree) => {
      const visit = (node) => {
        if (Array.isArray(node.children)) node.children.forEach(visit)
        if (!node.tagName || node.tagName !== 'code') return
        const langClass = (node.properties?.className ?? [])
          .find((c) => typeof c === 'string' && c.startsWith('language-'))
        const lang = langClass?.slice('language-'.length)
        const code = node.children
          .filter((c) => c.type === 'text')
          .map((c) => c.value)
          .join('')
        const lines = tokenizeLines(code, lang)
        node.properties = { ...(node.properties ?? {}), ...lineNumbersAttrs(lines.length) }
        node.children = hastLines(lines)
      }
      visit(tree)
    }
  }
}
