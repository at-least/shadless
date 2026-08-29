// Per-component contract: select (Wave B, kernel wireSelect)
export default {
  name: "select",

  usage: `
React.createElement(Select, { defaultValue: "a" },
  React.createElement(SelectTrigger, { id: "s1-trigger" },
    React.createElement(SelectValue, null),
  ),
  React.createElement(SelectContent, null,
    React.createElement(SelectItem, { value: "a" }, "Apple"),
    React.createElement(SelectItem, { value: "b" }, "Banana"),
  ),
)`,

  imports: `
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@/registry/bases/radix/ui/select";
`,

  slots: ["select-trigger", "select-value", "select-content", "&[role=option]"],

  open: `await page.click("#s1-trigger")`,
  openShadless: `await page.click("#s1-trigger")`,
  triggerSlot: "select-trigger",
  contentSlot: "select-content",

  // fixture is slot-keyed: mounted content has no inline classes by
  // design (styles ride out.css [data-slot] rules) — class compare off
  mountedClasses: false,
  // controlled-open oracle: content mounts at INITIAL render — mounted-diff inapplicable
  mountedCheck: false,
  shadlessPage: "src/kernel/select.html",

  // radix injects runtime layout style (box-sizing/max-height/flex) on content —
  // kernel puts positioning on the wrapper instead; recorded difference
  ignoreAttrs: { "select-content": ["style", "text"] }, // text: radix viewport internals

  scenarios: ["escape", "outside-click", "mouse-click:[data-slot=select-trigger]", "click:[role=option]", "key:ArrowDown+key:Enter"],
}
