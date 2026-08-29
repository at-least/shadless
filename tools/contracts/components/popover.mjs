// Per-component contract: popover (Wave B, kernel wirePopover)
export default {
  name: "popover",

  usage: `
React.createElement(Popover, null,
  React.createElement(PopoverTrigger, { id: "p1-trigger" }, "Open popover"),
  React.createElement(PopoverContent, null,
    React.createElement(PopoverHeader, null,
      React.createElement(PopoverTitle, null, "Dimensions"),
      React.createElement(PopoverDescription, null, "Set the dimensions."),
    ),
  ),
)`,

  imports: `
import { Popover, PopoverTrigger, PopoverContent, PopoverHeader, PopoverTitle, PopoverDescription } from "@/registry/bases/radix/ui/popover";
`,

  slots: ["popover-trigger", "popover-content", "popover-title", "popover-description"],

  // open via trigger click on BOTH sides (oracle is uncontrolled)
  open: `await page.click("#p1-trigger")`,
  openShadless: `await page.click("#p1-trigger")`,
  triggerSlot: "popover-trigger",
  contentSlot: "popover-content",

  shadlessPage: "src/kernel/popover.html",

  scenarios: ["escape", "outside-click", "trigger-toggle"],
}
