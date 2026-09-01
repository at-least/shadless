import { spawn } from "node:child_process"
const sh = spawn("node", ["tools/browser-shell.mjs"], { stdio: ["pipe","pipe","inherit"] })
let buf = ""
sh.stdout.on("data", d => { buf += d; const lines = buf.split("\n"); buf = lines.pop(); for (const l of lines) console.log("<<", l.slice(0,200)) })
const send = o => { console.log(">>"); sh.stdin.write(JSON.stringify(o)+"\n") }
send({op:"launch"})
setTimeout(() => send({op:"newPage"}), 300)
setTimeout(() => send({op:"goto", pageId:1, url:"http://127.0.0.1:8916/components/accordion.html"}), 700)
setTimeout(() => send({op:"evaluate", pageId:1, expr:`() => {
        const article = document.querySelector('.vp-doc')
        const text = article?.innerText?.trim() ?? ''
        return { rendered: !!article && text.length > 0, leaks: (text.match(/Component(Preview|Source)/g) ?? []) }
      }`}), 2000)
setTimeout(() => { sh.kill() }, 6000)
