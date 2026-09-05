({ sel, isSelect, attr, ids }) => {
  // references in the root that named the author's trigger id (a <label
  // for>, aria-labelledby, …) follow the rename; see efRetargetJs
  const retarget = (o, t) => { if (o && o !== t.id) { const A = ["for", "aria-labelledby", "aria-describedby", "aria-controls", "aria-owns"]; document.querySelectorAll("#root " + A.map((a) => "[" + a + "]").join(",")).forEach((e) => { for (const a of A) { const v = e.getAttribute(a); if (v == null) continue; const ts = v.split(/\s+/); if (ts.includes(o)) e.setAttribute(a, ts.map((x) => (x === o ? t.id : x)).join(" ")) } }) } }
  document.querySelectorAll("#root " + sel).forEach((t, i) => {
    const id = ids[i], o = t.id
    t.id = `${id}-trigger`; if (!isSelect) t.setAttribute(attr, id)
    retarget(o, t)
  })
}
