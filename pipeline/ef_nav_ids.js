(sel) => {
  document.querySelectorAll("#root " + sel).forEach((t, i) => { const id = /^(\w+)-trigger$/.exec(t.id)?.[1] ?? `n${i}`; t.id = `${id}-trigger`; t.setAttribute("data-radixuigo-nav-trigger", id) })
  // the viewport React left behind (if any) is the glue's to create
  document.querySelectorAll('#root [data-slot="navigation-menu-viewport"]').forEach((v) => v.remove())
}
