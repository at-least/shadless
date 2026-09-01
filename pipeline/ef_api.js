async (fams) => {
  const sleep = (ms) => new Promise((r) => setTimeout(r, ms))
  const seen = []
  const log = (e) => seen.push(e.type.slice(9) + ":" + e.detail.component + ":" + (e.target === trig ? "trigger" : e.target.id))
  document.addEventListener("shadless:open", log)
  document.addEventListener("shadless:close", log)
  var trig = null
  for (const comp of fams) {
    trig = document.querySelector(`[data-slot="${comp}-trigger"][id$="-trigger"]:not([disabled])`)
    if (!trig) continue
    const a = shadless.get(trig)
    if (!a || typeof a.open !== "function") return `${comp}: shadless.get(trigger) has no open()`
    seen.length = 0
    a.open(); await sleep(800)
    if (!a.isOpen() || !document.querySelector(`[data-slot="${comp}-content"]`)) return `${comp}: open() did not open`
    if (seen.join(",") !== `open:${comp}:trigger`) return `${comp}: open() events = [${seen}] (want open:${comp}:trigger)`
    a.close(); await sleep(600)
    if (a.isOpen()) return `${comp}: close() did not close`
    if (seen.join(",") !== `open:${comp}:trigger,close:${comp}:trigger`) return `${comp}: close() events = [${seen}] (want open,close on the trigger)`
  }
  return null
}
