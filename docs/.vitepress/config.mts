import { defineConfig } from 'vitepress'
import container from 'markdown-it-container'
import sidebar from './sidebar.json'

// The site shell. Everything here is VitePress's job — routing, the sidebar,
// dead-link checking, markdown, shiki. What this repo owns is the CONTENT
// transform (tools/docs-build.mjs: upstream React mdx → markdown for a product
// that ships static HTML) and the demos the pages iframe from /demos/, which
// are the pipeline's own output copied into public/ at build time.
export default defineConfig({
  title: 'shadless',
  description: 'shadcn/ui as static HTML and a vanilla runtime — no React.',
  srcDir: '.',
  outDir: '.vitepress/dist',
  cleanUrls: true,
  // A link into a page this mirror does not carry is a build failure, not a
  // 404 for a reader. This replaces tools/docs-links.mjs.
  ignoreDeadLinks: false,
  // docs/ also holds the pipeline's own trees; only the generated markdown is
  // site content. content/ is the hand-authored mdx SOURCE for the guides —
  // docs-build compiles it into guides/.
  srcExclude: ['content/**', 'demos/**', 'site/**', 'fonts/**'],
  markdown: {
    config: (md) => {
      // `::::demo <name>` — one card holding the preview iframe and the demo's
      // source, the shape upstream's ComponentPreview uses: the code sits
      // below the preview, capped behind a gradient, revealed by a button.
      //
      // The toggle is a checkbox and a label, not script: the card works
      // before (and without) hydration, and there is no per-page wiring to
      // re-run on client-side navigation.
      md.use(container, 'demo', {
        render(tokens: any[], idx: number) {
          if (tokens[idx].nesting !== 1) {
            return '</div>\n<label class="demo-view-code" for="' +
              tokens[idx].__demoId + '"></label>\n</div>\n'
          }
          const name = tokens[idx].info.trim().slice('demo'.length).trim()
          const id = `vc-${name}`
          // the closing token needs the same id; markdown-it hands them to us
          // separately, so stash it on the matching close token
          for (let i = idx + 1; i < tokens.length; i++) {
            if (tokens[i].type === 'container_demo_close') { tokens[i].__demoId = id; break }
          }
          return `<div class="demo-card" data-demo="${name}">\n` +
            `<input class="demo-toggle" type="checkbox" id="${id}" tabindex="-1" aria-hidden="true">\n` +
            '<div class="demo-body">\n'
        },
      })
    },
  },
  themeConfig: {
    nav: [
      { text: 'Components', link: '/' },
      { text: 'Introduction', link: '/guides/introduction' },
      { text: 'Installation', link: '/guides/installation' },
    ],
    sidebar,
    outline: [2, 3],
    socialLinks: [{ icon: 'github', link: 'https://github.com/at-least/shadless' }],
    search: { provider: 'local' },
  },
})
