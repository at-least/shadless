// Per-component contract: scroll-area (Wave B, kernel wireScrollArea)
// stateProbe = scrollTop | scrollbar data-state | thumb data-state
export default {

  usage: `
React.createElement(ScrollArea, { style: { height: "200px", width: "200px" } },
  ${Array.from({ length: 20 }, (_, i) =>
    `React.createElement("div", { style: { height: "20px", lineHeight: "19px" } }, "${i + 1}")`).join(",\n  ")}
)`,

  imports: `
import { ScrollArea } from "@/registry/bases/radix/ui/scroll-area";
`,

  slots: ["scroll-area", "scroll-area-viewport", "scroll-area-scrollbar", "scroll-area-thumb"],

  // "open" = hover the root (type=hover scrollbar becomes visible)
  open: `await page.locator("[data-slot=scroll-area]").hover()`,
  openShadless: `await page.locator("[data-slot=scroll-area]").hover()`,
  triggerSlot: "scroll-area",
  contentSlot: "scroll-area",

  stateProbe: `(function () {
    var v = document.querySelector("[data-slot=scroll-area-viewport]");
    var sb = document.querySelector("[data-slot=scroll-area-scrollbar]");
    var th = document.querySelector("[data-slot=scroll-area-thumb]");
    return Math.round(v.scrollTop) + "|" + (sb && sb.getAttribute("data-state")) + "|" + (th && th.getAttribute("data-state"));
  })()`,

  // fixture is slot-keyed: mounted content has no inline classes by
  // design (styles ride out.css [data-slot] rules) — class compare off
  mountedClasses: false,
  // runtime mounts only measurement styles (Wave B recorded inline-style difference) — no structural additions to compare
  mountedCheck: false,
  shadlessPage: "src/kernel/scroll-area.html",

  // oracle pages carry no stylesheet — viewport sizing + scroll layout needed
  oracleCss: `
    [data-slot=scroll-area] { position: relative; }
    [data-slot=scroll-area-viewport] { height: 100%; width: 100%; }
  `,

  // runtime-only inline styles (radix/kernel positioning + thumb metrics vars)
  ignoreAttrs: {
    "scroll-area": ["style", "text"], // text: radix injects internal collection text
    "scroll-area-viewport": ["style", "data-radix-scroll-area-viewport"],
    "scroll-area-scrollbar": ["style"],
    "scroll-area-thumb": ["style"],
  },

  scenarios: ["move:[data-slot=scroll-area-viewport]+wheel:0,150",
    "wheel:0,300", "wheel:0,-500"],
}
