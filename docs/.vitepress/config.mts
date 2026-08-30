import { defineConfig } from 'vitepress'
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
