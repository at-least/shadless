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
      // source. Nothing is hidden: the source sits under the preview inside
      // the same border, scrolling if it is long.
      md.use(container, 'demo', {
        render(tokens: any[], idx: number) {
          if (tokens[idx].nesting !== 1) return '</div>\n'
          const name = tokens[idx].info.trim().slice('demo'.length).trim()
          return `<div class="demo-card" data-demo="${name}">\n`
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
