// Per-component contract: slider (Wave B, kernel wireSlider)
// stateProbe = aria-valuenow of the thumb → scenarios compare VALUE, not open/close
export default {

  usage: `
React.createElement(Slider, { defaultValue: [50], id: "s1" })`,

  imports: `
import { Slider } from "@/registry/bases/radix/ui/slider";
`,

  slots: ["slider", "slider-track", "slider-range", "slider-thumb"],

  // "open" = focus the thumb (keyboard scenarios start from it)
  open: `await page.focus("[role=slider]")`,
  openShadless: `await page.focus("[role=slider]")`,
  triggerSlot: "slider",
  contentSlot: "slider",

  stateProbe: `document.querySelector('[data-slot=slider-thumb]').getAttribute('aria-valuenow')`,

  shadlessPage: "src/kernel/slider.html",

  // oracle pages carry no stylesheet — slider geometry needs real layout for
  // pointer scenarios, so inject the minimal sizing the classes would provide
  oracleCss: `
    [data-slot=slider] { position: relative; display: flex; align-items: center; touch-action: none; user-select: none; width: 300px; }
    [data-slot=slider-track] { position: relative; height: 6px; width: 100%; border-radius: 9999px; overflow: hidden; }
    [data-slot=slider-range] { position: absolute; height: 100%; }
  `,

  scenarios: ["key:ArrowRight", "key:Home", "key:PageUp", "key:End+key:ArrowLeft",
    "clickAt:[data-slot=slider-track]@75,50"],
}
