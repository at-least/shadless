({ props }) => {
  // Every element under body in document order — INCLUDING <template>
  // contents (the portal DOM; a querySelectorAll("body *") walk never
  // sees them). Template-contained elements do not render, so they are
  // covered by the structural tags/slots assert only and are flagged
  // rendered=false for the computed-style comparison.
  const els = []
  const walk = (container) => {
    for (const el of container.children) {
      els.push(el)
      walk(el)
      if (el.tagName === "TEMPLATE" && el.content) walk(el.content)
    }
  }
  walk(document.body)
  const out = {}
  const tags = []
  const slots = []
  const rendered = []
  for (const [theme, dir] of [["light", "ltr"], ["dark", "ltr"], ["light", "rtl"], ["dark", "rtl"]]) {
    document.documentElement.classList.toggle("dark", theme === "dark"); document.documentElement.setAttribute("dir", dir)
    els.forEach((el, i) => {
      if (theme === "light" && dir === "ltr") { tags.push(el.tagName); slots.push(el.getAttribute("data-slot") || ""); rendered.push(el.isConnected) }
      if (!el.isConnected) return
      const cs = getComputedStyle(el); const style = {}
      for (const p of props) style[p] = cs.getPropertyValue(p)
      out[`${i}@${theme}@${dir}`] = style
    })
  }
  return { tags, slots, rendered, out }
}
