// T3 shoot: screenshot oracle + both models at identical viewport, per-pixel
// diff in chromium canvas. PASS if mismatched pixels < 1% (channel tol 24).
import { chromium } from "playwright"
import { readFileSync, writeFileSync } from "node:fs"

const browser = await chromium.launch()
const ctx = await browser.newContext({ viewport: { width: 900, height: 700 } })
const shot = async (file) => {
  const page = await ctx.newPage()
  await page.goto(`file://${process.cwd()}/probes/t3/${file}`)
  await page.waitForTimeout(700) // let open-animations settle
  const buf = await page.screenshot()
  await page.close()
  return buf.toString("base64")
}
const oracle = await shot("oracle.html")
const a = await shot("page-a.html")
const b = await shot("page-b.html")
await browser.close()

const diffPage = await (async () => {
  const b2 = await chromium.launch()
  const ctx2 = await b2.newContext()
  return ctx2.newPage()
})()
const results = await diffPage.evaluate(async ([o, a, b]) => {
  const load = (b64) => new Promise((res) => {
    const i = new Image()
    i.onload = () => res(i)
    i.src = "data:image/png;base64," + b64
  })
  const cmp = async (x, y) => {
    const [ix, iy] = [await load(x), await load(y)]
    const c = document.createElement("canvas")
    c.width = ix.width; c.height = ix.height
    const g = c.getContext("2d")
    g.drawImage(ix, 0, 0)
    const dx = g.getImageData(0, 0, c.width, c.height).data
    g.clearRect(0, 0, c.width, c.height); g.drawImage(iy, 0, 0)
    const dy = g.getImageData(0, 0, c.width, c.height).data
    let bad = 0
    let x0 = 1e9, y0 = 1e9, x1 = -1, y1 = -1
    for (let i = 0; i < dx.length; i += 4) {
      if (Math.abs(dx[i]-dy[i]) > 24 || Math.abs(dx[i+1]-dy[i+1]) > 24 ||
          Math.abs(dx[i+2]-dy[i+2]) > 24 || dx[i+3] !== dy[i+3]) {
        bad++
        const p = i / 4, px = p % c.width, py = (p - px) / c.width
        if (px < x0) x0 = px; if (px > x1) x1 = px
        if (py < y0) y0 = py; if (py > y1) y1 = py
      }
    }
    return { w: c.width, h: c.height, bad, pct: (100*bad)/(c.width*c.height),
             bbox: [x0, y0, x1, y1] }
  }
  return { modelA: await cmp(o, a), modelB: await cmp(o, b) }
}, [oracle, a, b])
await diffPage.close()

writeFileSync("probes/t3/oracle.png", Buffer.from(oracle, "base64"))
writeFileSync("probes/t3/page-a.png", Buffer.from(a, "base64"))
writeFileSync("probes/t3/page-b.png", Buffer.from(b, "base64"))
writeFileSync("probes/t3/diff.json", JSON.stringify(results, null, 2))

let pass = true
for (const [k, v] of Object.entries(results)) {
  console.log(`t3 ${k}: ${v.bad}/${v.w * v.h} px mismatch = ${v.pct.toFixed(4)}% -> ${v.pct < 1 ? "ok" : "OVER"}`)
  if (v.pct >= 1) pass = false
}
console.log(pass ? "PASS  t3 visual diff within tolerance" : "FAIL  t3 visual diff over tolerance")
process.exit(pass ? 0 : 1)
