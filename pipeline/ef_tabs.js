async () => {
  const sleep = (ms) => new Promise((r) => setTimeout(r, ms))
  const roots = [...document.querySelectorAll("#root [data-slot=tabs]")]
  let r = 0
  for (const root of roots) {
    const triggers = [...root.querySelectorAll("[data-slot=tabs-trigger]")]
    const activeIdx = triggers.findIndex((t) => t.getAttribute("data-state") === "active")
    const panels = []
    for (let i = 0; i < triggers.length; i++) {
      triggers[i].click(); await sleep(80)
      const panel = root.querySelector("[data-slot=tabs-content]:not([hidden])") ?? root.querySelector("[data-slot=tabs-content]")
      panels.push(panel ? panel.outerHTML : null)
    }
    if (activeIdx >= 0) { triggers[activeIdx].click(); await sleep(80) }
    root.querySelectorAll("[data-slot=tabs-content]").forEach((p) => p.remove())
    // ids the usage tree already gave (t1/t2 → d1/d2 in the contract def) stay
    const tid = (t, i) => (t.id && !t.id.startsWith("radix-")) ? t.id : `tab${r}-${i}`
    const pid = (t, i) => { const c = t.getAttribute("aria-controls"); return (c && !c.startsWith("radix-")) ? c : `panel${r}-${i}` }
    const pids = triggers.map((t, i) => pid(t, i))
    triggers.forEach((t, i) => {
      // four upstream tabs demos (line, vertical, disabled, icons) render
      // triggers only — no <TabsContent> at all; naming a panel that does
      // not exist left aria-controls dangling on every one of them
      t.id = tid(t, i); if (panels[i]) t.setAttribute("aria-controls", pids[i]); else t.removeAttribute("aria-controls")
      const st = i === Math.max(activeIdx, 0) ? "active" : "inactive"
      t.setAttribute("data-state", st); t.setAttribute("aria-selected", st === "active" ? "true" : "false")
    })
    panels.forEach((html, i) => {
      if (!html) return
      const tpl = document.createElement("template"); tpl.innerHTML = html
      const p = tpl.content.firstElementChild
      p.id = pids[i]; p.setAttribute("aria-labelledby", triggers[i].id)
      // radix mounts a panel with animation-duration: 0s (Presence, no
      // enter animation on first mount); the oracle records it on the
      // active panel — every panel here starts the same way
      if (!p.getAttribute("style")) p.setAttribute("style", "animation-duration: 0s")
      const st = i === Math.max(activeIdx, 0) ? "active" : "inactive"
      p.setAttribute("data-state", st); if (st === "inactive") p.setAttribute("hidden", ""); else p.removeAttribute("hidden")
      root.appendChild(p)
    })
    r++
  }
  return document.querySelector("#root").innerHTML
}
