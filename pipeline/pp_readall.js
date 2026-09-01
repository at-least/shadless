({ ids, props }) => {
  const out = {}
  for (const [dark, dir] of [["light", "ltr"], ["dark", "ltr"], ["light", "rtl"], ["dark", "rtl"]]) {
    document.documentElement.classList.toggle("dark", dark === "dark"); document.documentElement.setAttribute("dir", dir)
    for (const id of ids) {
      const el = document.getElementById(id); if (!el) continue
      const cs = getComputedStyle(el); const style = {}
      for (const p of props) style[p] = cs.getPropertyValue(p)
      out[`${id}@${dark}@${dir}`] = style
    }
  }
  return out
}
