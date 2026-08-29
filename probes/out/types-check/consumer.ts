
import shadless, { get, init, h, theme, type Handle, type SelectHandle } from "shadless/runtime"
import "shadless/js/dialog"

const handle: Handle | null = get("#d1-trigger")
if (handle) {
  switch (handle.component) {
    case "select": {
      const v: string | null = handle.value()
      const l: string | null = handle.label()
      handle.select("#opt")
      void v; void l
      break
    }
    case "tabs": handle.activate(1, true); const i: number = handle.active(); void i; break
    case "slider": handle.setValue(3, 0, { commit: true }); const vs: number[] = handle.values(); void vs; break
    case "carousel": handle.scrollNext(); break
    default: handle.open(); handle.close(false); const o: boolean = handle.isOpen(); void o
  }
}
const sel = shadless.get("#s1-trigger") as SelectHandle | null
if (sel && sel.component === "select") sel.selected()

document.addEventListener("shadless:change", (e) => {
  const c: string = e.detail.component
  const values: number[] | undefined = e.detail.values
  const label: string | null | undefined = e.detail.label
  void c; void values; void label
})
document.addEventListener("shadless:commit", (e) => { const v: number[] = e.detail.values; void v })
document.addEventListener("shadless:themechange", (e) => { const m: "light" | "dark" = e.detail.mode; void m })

init(document.body, { force: true })
const idx: number = h.nextIndex(new KeyboardEvent("keydown"), [])
const rec = h.wire(document.body)
if (rec) document.body.addEventListener("click", () => {}, { signal: rec.signal })
h.formMirror(document.body, { read: () => true, write: (v) => { void v } })
theme.set("dark")
window.shadlessNoAutoInit = true
void idx
