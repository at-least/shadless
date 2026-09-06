// Per-component contract: hover-card (Wave B, kernel wireHoverCard, hover-open)
export default {

  usage: `
React.createElement(HoverCard, null,
  React.createElement(HoverCardTrigger, { id: "d1-trigger" }, "Hover me"),
  React.createElement(HoverCardContent, null, "The React Framework for web and native user interfaces."),
)`,

  imports: `
import { HoverCard, HoverCardTrigger, HoverCardContent } from "@/registry/bases/radix/ui/hover-card";
`,

  slots: ["hover-card-trigger", "hover-card-content"],

  // radix HoverCard defaults: openDelay 700ms / closeDelay 300ms on BOTH sides
  open: `await page.hover("#d1-trigger"); await page.waitForTimeout(1000)`,
  openShadless: `await page.hover("#d1-trigger"); await page.waitForTimeout(1000)`,
  triggerSlot: "hover-card-trigger",
  contentSlot: "hover-card-content",

  shadlessPage: "src/kernel/hover-card.html",

  scenarios: ["escape", "outside-click", "pointer-away"],
}
