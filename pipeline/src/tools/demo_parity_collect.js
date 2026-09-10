(props) => {
  const out = {}
  for (const [theme, dir] of [["light", "ltr"], ["dark", "ltr"], ["light", "rtl"], ["dark", "rtl"]]) {
    document.documentElement.classList.toggle("dark", theme === "dark"); document.documentElement.setAttribute("dir", dir)
    const seen = {}
    for (const el of document.querySelectorAll("body [data-slot]")) {
      const slot = el.getAttribute("data-slot"); const key = `${slot}#${seen[slot] = (seen[slot] ?? 0) + 1}`
      const cs = getComputedStyle(el); const style = {}
      for (const p of props) style[p] = cs.getPropertyValue(p)
      out[`${key}@${theme}@${dir}`] = style
    }
  }
  return out
}
