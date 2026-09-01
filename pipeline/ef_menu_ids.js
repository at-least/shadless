({ sel, isSelect, attr, ids }) => document.querySelectorAll("#root " + sel).forEach((t, i) => {
  const id = ids[i]
  t.id = `${id}-trigger`; if (!isSelect) t.setAttribute(attr, id)
})
