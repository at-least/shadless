
// FT8: propagate theme to all preview iframes (same-origin, so we can reach
// into iframe.contentDocument). Called on initial load AND after every
// iframe load event (the iframe's contentDocument is null until load).
// Avoids editing 300+ demo files individually.
function applyThemeToIframe(iframe, dark) {
  try {
    const doc = iframe.contentDocument
    if (doc && doc.documentElement) doc.documentElement.classList.toggle('dark', dark)
  } catch {}
}
function applyThemeToIframes(dark) {
  for (const iframe of document.querySelectorAll('iframe[data-preview-frame]')) {
    applyThemeToIframe(iframe, dark)
  }
}
const root = document.documentElement
function currentDark() { return root.classList.contains('dark') }
applyThemeToIframes(currentDark())

// FT8: auto-size preview iframes to their content. Same-origin so we read
// scrollHeight from contentDocument directly (no postMessage). Floor is
// 288px = upstream's h-72 preview height: small demos center in the same
// frame shadcn.com shows, and portal dialogs (alert-dialog/dialog/sheet)
// get room to open without the iframe clipping them. Caps at 900px so
// huge demos don't dominate the page. Runs on iframe load, theme change,
// viewport resize, and any DOM mutation (catches lazy iframes).
function resizeIframe(iframe) {
  try {
    const doc = iframe.contentDocument
    if (!doc) return
    const body = doc.body
    if (!body) return
    const h = Math.min(Math.max(body.scrollHeight, 288), 900)
    iframe.style.height = h + 'px'
  } catch {}
}
function resizeAllIframes() {
  for (const iframe of document.querySelectorAll('iframe[data-preview-frame]')) {
    resizeIframe(iframe)
  }
}
// Wire BOTH resize + theme to every iframe's load event (and to new
// iframes as they appear via MutationObserver). Without the load handler,
// contentDocument is null and both operations are no-ops.
function wireIframe(iframe) {
  iframe.addEventListener('load', () => {
    applyThemeToIframe(iframe, currentDark())
    setTimeout(resizeIframe.bind(null, iframe), 50)
  })
}
for (const iframe of document.querySelectorAll('iframe[data-preview-frame]')) {
  wireIframe(iframe)
}
new MutationObserver((muts) => {
  for (const m of muts) {
    for (const node of m.addedNodes) {
      if (node.tagName === 'IFRAME' && node.dataset?.previewFrame !== undefined) {
        wireIframe(node)
      }
      // also check subtree (lazy iframes added to existing wrappers)
      if (node.querySelectorAll) {
        for (const f of node.querySelectorAll('iframe[data-preview-frame]')) wireIframe(f)
      }
    }
  }
}).observe(document.body, { childList: true, subtree: true })
window.addEventListener('resize', resizeAllIframes)

// CodeTabs tab switching (site-local, vanilla)
for (const root of document.querySelectorAll('[data-code-tabs]')) {
  const triggers = [...root.querySelectorAll('[data-tab-trigger]')]
  const panels = [...root.querySelectorAll('[data-tab-content]')]
  if (!triggers.length || !panels.length) continue
  const activate = (value) => {
    for (const t of triggers) t.setAttribute('aria-selected', String(t.dataset.tabTrigger === value))
    for (const p of panels) p.hidden = p.dataset.tabContent !== value
  }
  for (const t of triggers) t.addEventListener('click', () => activate(t.dataset.tabTrigger))
  activate(triggers[0].dataset.tabTrigger)
}
// mark the active sidebar entry
const here = location.pathname.split('/').pop() || 'index.html'
for (const a of document.querySelectorAll('.sidebar a')) {
  if (a.getAttribute('href') === here) a.classList.add('active')
}
// Mobile sidebar drawer (below lg): the topbar hamburger toggles the
// sidebar as an overlay (data-open on #sidebar + backdrop drives the CSS
// slide/fade). Escape, backdrop click, and any nav link close it; body
// scroll locks while open; crossing back over lg resets the state so a
// later resize down doesn't resurrect an open drawer.
;(function () {
  const btn = document.querySelector('[data-sidebar-toggle]')
  const sidebar = document.querySelector('.sidebar')
  const backdrop = document.querySelector('[data-sidebar-backdrop]')
  if (!btn || !sidebar || !backdrop) return
  const isOpen = () => sidebar.getAttribute('data-open') === 'true'
  const setOpen = (open) => {
    sidebar.setAttribute('data-open', String(open))
    backdrop.setAttribute('data-open', String(open))
    btn.setAttribute('aria-expanded', String(open))
    btn.setAttribute('aria-label', open ? 'Close navigation' : 'Open navigation')
    document.body.classList.toggle('sidebar-open', open)
    // closing while focus sits inside the drawer would strand it on a
    // visibility:hidden element — hand it back to the toggle
    if (!open && sidebar.contains(document.activeElement)) btn.focus()
  }
  btn.addEventListener('click', () => setOpen(!isOpen()))
  backdrop.addEventListener('click', () => setOpen(false))
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && isOpen()) setOpen(false)
  })
  sidebar.addEventListener('click', (e) => {
    if (e.target.closest('a')) setOpen(false)
  })
  // same 1023.98px boundary as the CSS media query — no dead zone where
  // the shell is desktop but the scroll lock survives
  const mobile = window.matchMedia('(max-width: 1023.98px)')
  const onMql = (e) => { if (!e.matches) setOpen(false) }
  if (mobile.addEventListener) mobile.addEventListener('change', onMql)
  else mobile.addListener(onMql) // Safari < 14
})()
// FT8: theme toggle button — mirror inline-ComponentPreview behavior in
// dark-mode.mdx: toggle .dark on <html>, persist to localStorage, reflect
// state on the icon.
const themeBtn = document.querySelector('[data-theme-toggle]')
if (themeBtn) {
  const reflect = () => {
    const dark = document.documentElement.classList.contains('dark')
    themeBtn.setAttribute('aria-label', dark ? 'Use light theme' : 'Use dark theme')
    themeBtn.setAttribute('data-state', dark ? 'dark' : 'light')
  }
  themeBtn.addEventListener('click', () => {
    const dark = !document.documentElement.classList.contains('dark')
    document.documentElement.classList.toggle('dark', dark)
    try { localStorage.setItem('shadless-docs-theme', dark ? 'dark' : 'light') } catch (e) {}
    reflect()
    applyThemeToIframes(dark)
    // Re-resize iframes after theme toggle (some demos may change layout height)
    setTimeout(resizeAllIframes, 100)
  })
  reflect()
}
// FT8: inline Accordion shim for prose (introduction.mdx, any guide using
// <Accordion>). Click + Enter/Space on the trigger toggles its state via
// the same data-state attributes the runtime uses. Avoid double-init when
// the same root is also wired by shadless.js.
for (const trigger of document.querySelectorAll('[data-slot="accordion-trigger"]')) {
  if (trigger.dataset.shimInit) continue
  trigger.dataset.shimInit = '1'
  const item = trigger.closest('[data-slot="accordion-item"]')
  const root = trigger.closest('[data-slot="accordion"]')
  const content = item?.querySelector('[data-slot="accordion-content"]')
  if (!item || !content) continue
  const multiple = root?.getAttribute('data-type') === 'multiple'
  trigger.addEventListener('click', () => {
    const wasOpen = trigger.getAttribute('data-state') === 'open'
    const open = !wasOpen
    if (!multiple && open && root) {
      for (const t of root.querySelectorAll('[data-slot="accordion-trigger"]')) {
        if (t === trigger) continue
        t.setAttribute('data-state', 'closed')
        t.setAttribute('aria-expanded', 'false')
        const c = t.closest('[data-slot="accordion-item"]')?.querySelector('[data-slot="accordion-content"]')
        if (c) { c.setAttribute('data-state', 'closed'); c.hidden = true }
      }
    }
    trigger.setAttribute('data-state', open ? 'open' : 'closed')
    trigger.setAttribute('aria-expanded', String(open))
    content.setAttribute('data-state', open ? 'open' : 'closed')
    content.hidden = !open
    item.setAttribute('data-state', open ? 'open' : 'closed')
  })
}

// FT13: heading permalink anchor — append a # anchor to every heading with
// an id inside .typeset. Hidden until the heading is hovered. Click on the
// anchor updates the URL hash (native <a> behavior); the surrounding click
// handler additionally copies the absolute URL to clipboard.
(function () {
  const headings = document.querySelectorAll('.typeset :is(h1, h2, h3, h4, h5, h6)[id]')
  for (const h of headings) {
    if (h.querySelector('a.anchor')) continue
    const a = document.createElement('a')
    a.className = 'anchor'
    a.href = '#' + h.id
    a.setAttribute('aria-label', 'Permalink to ' + h.textContent.trim())
    a.textContent = '#'
    h.prepend(a)
  }
  document.addEventListener('click', (e) => {
    const h = e.target?.closest?.('.typeset :is(h1, h2, h3, h4, h5, h6)[id]')
    if (!h || e.target.closest('a')) return
    // FT14: heading click — update URL hash + copy full URL. Mirrors the
    // <a class="anchor"> click behavior (which updates hash natively) so
    // both interactions produce the same effect: hash reflects heading,
    // clipboard holds the absolute link. history.pushState avoids
    // re-scrolling (the heading is already in view since it was clicked).
    // location.origin is the string "null" under file:// — derive the URL
    // from href so local previews copy something usable.
    const url = location.href.split('#')[0] + '#' + h.id
    history.pushState(null, '', '#' + h.id)
    if (navigator.clipboard?.writeText) {
      navigator.clipboard.writeText(url).catch(() => {})
    }
  })
})()

// FT8/Step 7: SVG icon glyphs used inside the action buttons.
// Match the lucide outline style used elsewhere in shadless. Each
// icon is a minimal path; combined with the button's own border + bg
// they read as small affordance buttons, not noisy chrome.
const ICON_COPY = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>'
const ICON_CHECK = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>'

// FT8/Step 3: Copy button on every <pre> in the article area. Matches
// shadcn/ui's affordance — small button top-right of the code block,
// copies the inner code text to clipboard, briefly flips to check icon.
// Selector: article pre (covers body code blocks + ComponentPreview
// source <details>). Idempotent + MutationObserver for lazy content.
function wireCopyButton(pre) {
  if (pre.querySelector(':scope > button.code-copy')) return
  const btn = document.createElement('button')
  btn.type = 'button'
  btn.className = 'code-copy'
  btn.setAttribute('aria-label', 'Copy code to clipboard')
  btn.title = 'Copy code'
  btn.innerHTML = ICON_COPY
  const restoreCopy = () => { btn.innerHTML = ICON_COPY; btn.classList.remove('copied') }
  btn.addEventListener('click', async () => {
    const code = pre.querySelector('code')
    const text = code ? code.innerText : pre.innerText
    try {
      await navigator.clipboard.writeText(text)
      btn.innerHTML = ICON_CHECK
      btn.classList.add('copied')
      setTimeout(restoreCopy, 1200)
    } catch (e) {
      restoreCopy()
    }
  })
  pre.appendChild(btn)
}
function wireAllCopyButtons() {
  for (const pre of document.querySelectorAll('article pre')) wireCopyButton(pre)
}
wireAllCopyButtons()
new MutationObserver(wireAllCopyButtons).observe(document.body, { childList: true, subtree: true })

// FT8/Step 4 (reworked): "View Code" CTA inside each ComponentPreview's
// source veil. Clicking sets data-open="true" on the .preview-source
// wrapper — CSS uncaps .source-body and hides the veil. aria-expanded
// reflects state. Reveal-only (upstream has no collapse-back affordance).
function wireViewCodeButtons() {
  for (const btn of document.querySelectorAll('button[data-view-code]')) {
    if (btn.dataset.wired) continue
    btn.dataset.wired = '1'
    const source = btn.closest('.preview-source')
    if (!source) continue
    const body = source.querySelector('.source-body')
    if (body && !body.id) body.id = 'src-' + Math.random().toString(36).slice(2, 9)
    if (body) btn.setAttribute('aria-controls', body.id)
    btn.addEventListener('click', () => {
      source.setAttribute('data-open', 'true')
      btn.setAttribute('aria-expanded', 'true')
    })
  }
}
wireViewCodeButtons()
new MutationObserver(wireViewCodeButtons).observe(document.body, { childList: true, subtree: true })

// FT8/Step 9: RTL language selector. Each RTL preview frame has a
// preview-rtl-langs group with one button per available language
// (AR / HE / EN / FA). Clicking a button swaps the iframe src to the
// matching NAME-rtl-LANG.html file (built by tools/build-rtl.mjs).
// To keep the iframe load console-error-free (no spurious 404s for
// components that don't ship all 4 languages), the host page passes
// a list of available languages per preview via a sibling data-attr;
// buttons for missing languages are removed at build time, not
// hidden at runtime.
function wireRtlLangs() {
  for (const group of document.querySelectorAll('.preview-rtl-langs')) {
    if (group.dataset.wired) continue
    group.dataset.wired = '1'
    const frame = group.closest('.preview-frame')
    const iframe = frame?.querySelector('iframe[data-preview-frame]')
    if (!iframe) continue
    const baseSrc = iframe.getAttribute('src') // e.g. "components/alert-rtl.html"
    const dotIdx = baseSrc.lastIndexOf('.')
    const stem = dotIdx === -1 ? baseSrc : baseSrc.slice(0, dotIdx)
    const ext = dotIdx === -1 ? '' : baseSrc.slice(dotIdx)
    group.querySelectorAll('button[data-rtl-lang]').forEach((btn) => {
      const lang = btn.dataset.rtlLang
      const variantSrc = lang === 'ar' ? baseSrc : stem + "-" + lang + ext
      btn.addEventListener('click', () => {
        iframe.setAttribute('src', variantSrc)
        group.querySelectorAll('button[data-active-lang]').forEach((b) => b.removeAttribute('data-active-lang'))
        btn.setAttribute('data-active-lang', '')
        setTimeout(resizeAllIframes, 100)
      })
    })
  }
}
wireRtlLangs()
new MutationObserver(wireRtlLangs).observe(document.body, { childList: true, subtree: true })
