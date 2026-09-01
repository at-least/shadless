({ sel, sub, layerId }) => {
  const content = document.querySelector(sel + ":not([data-ef-harvested])")
  const subs = [...content.querySelectorAll(sub)]
  subs.forEach((t, j) => { if (t.id && t.id.startsWith("radix-")) t.setAttribute("data-ef-orig", t.id); t.setAttribute("data-radixuigo-menu-subtrigger", `${layerId}s${j}`); t.id = `${layerId}s${j}-trigger` })
  const html = content.outerHTML
  content.setAttribute("data-ef-harvested", "")
  return { html, subCount: subs.length }
}
