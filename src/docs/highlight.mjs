// FT2 docs: code block STRUCTURE at build time (React-free, shiki-free).
//
// The build emits plain text: every line is a <span data-line> inside a
// code[data-line-numbers][data-line-numbers-max-digits] (gutter is a CSS
// counter ::before in site.css). Fences whose language shiki knows get a
// data-lang attribute; docs/site/highlight.js (src/docs/highlight-client.mjs)
// colours those tokens in the browser after load. Highlighting is
// presentation only, so it is no longer baked into 137 MB of markup.
//
// bash/text fences carry no data-lang and stay plain: install-command
// fences are copy-paste payloads that the acceptance gate greps verbatim
// ("npx shadcn@latest add dialog"). Line wrapping preserves textContent
// exactly, so those greps are unaffected.
export const LANG_MAP = {
  tsx: 'tsx', typescript: 'typescript', ts: 'typescript', css: 'css', diff: 'diff',
  json: 'json', html: 'html', js: 'javascript', jsx: 'tsx',
}

export function splitLines(code) {
  const lines = code.split('\n')
  // a trailing newline is the fence's own terminator, not an empty last line
  if (lines.length > 1 && lines[lines.length - 1] === '') lines.pop()
  return lines
}

// attributes for the <code> element: line-number gutter + the grammar the
// client highlighter should apply (absent for plain/bash/text fences)
export function codeAttrs(lineCount, lang) {
  return {
    'data-line-numbers': '',
    'data-line-numbers-max-digits': String(Math.max(1, lineCount)).length,
    ...(LANG_MAP[lang] ? { 'data-lang': LANG_MAP[lang] } : {}),
  }
}

// Each line span ends with '\n' so textContent preserves newlines; with
// display:block + white-space:pre the trailing newline does not create a
// blank line (same rule that keeps pretty-printed <pre> tight).
function hastLines(lines) {
  return lines.map((line) => ({
    type: 'element', tagName: 'span',
    properties: { 'data-line': '' },
    children: [{ type: 'text', value: line + '\n' }],
  }))
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
        const lines = splitLines(code)
        node.properties = { ...(node.properties ?? {}), ...codeAttrs(lines.length, lang) }
        node.children = hastLines(lines)
      }
      visit(tree)
    }
  }
}
