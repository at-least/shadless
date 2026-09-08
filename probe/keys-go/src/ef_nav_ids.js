(sel) => {
  const retarget = (o, t) => { if (o && o !== t.id) { const A = ["for", "aria-labelledby", "aria-describedby", "aria-controls", "aria-owns"]; document.querySelectorAll("#root " + A.map((a) => "[" + a + "]").join(",")).forEach((e) => { for (const a of A) { const v = e.getAttribute(a); if (v == null) continue; const ts = v.split(/\s+/); if (ts.includes(o)) e.setAttribute(a, ts.map((x) => (x === o ? t.id : x)).join(" ")) } }) } }
  document.querySelectorAll("#root " + sel).forEach((t, i) => { const id = /^(\w+)-trigger$/.exec(t.id)?.[1] ?? `n${i}`; const o = t.id; t.id = `${id}-trigger`; t.setAttribute("data-radixuigo-nav-trigger", id); retarget(o, t) })
  // the viewport React left behind (if any) is the glue's to create
  document.querySelectorAll('#root [data-slot="navigation-menu-viewport"]').forEach((v) => v.remove())
}
