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

  // The "closed:" scenarios start from the CLOSED state — every other one is
  // run after the harness has opened the component, which cannot express
  // "does the keyboard open it at all". The fixture carries defaultValue, so
  // the kernel mounts with an item already highlighted: exactly the case where
  // opening and committing collapse into one keystroke if the opening event
  // reaches the kernel's document keydown listener.
  scenarios: ["escape", "outside-click", "mouse-click:[data-slot=select-trigger]", "click:[role=option]", "key:ArrowDown+key:Enter",
    "closed:focus:[data-slot=select-trigger]+key:Enter",
    "closed:focus:[data-slot=select-trigger]+key: ",
    "closed:focus:[data-slot=select-trigger]+key:ArrowDown"],
}
