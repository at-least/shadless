// Per-component contract: tooltip (Wave B, kernel wireTooltip, hover-open)
export default {

  usage: `
React.createElement(TooltipProvider, null,
  React.createElement(Tooltip, null,
    React.createElement(TooltipTrigger, { id: "t1-trigger" }, "Hover me"),
    React.createElement(TooltipContent, null, "Add to library"),
  ),
)`,

  imports: `
import { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider } from "@/registry/bases/radix/ui/tooltip";
`,

  slots: ["tooltip-trigger", "tooltip-content"],

  // radix (current) renders id/role on an inner sr-only DUPLICATE of the
  // content text (aria-describedby target), not on the content root —
  // textContent sees the text twice and the root carries neither attr.
  // kernel+glue keep id/role on the content root with single text:
  // equivalent semantics, recorded structural difference.
  ignoreAttrs: {
    "tooltip-content": ["text", "id", "role"],
  },

  // tooltip opens on hover (provider delayDuration=0) on BOTH sides
  open: `await page.hover("#t1-trigger"); await page.waitForTimeout(250)`,
  openShadless: `await page.hover("#t1-trigger"); await page.waitForTimeout(250)`,
  triggerSlot: "tooltip-trigger",
  contentSlot: "tooltip-content",

  shadlessPage: "src/kernel/tooltip.html",

  scenarios: ["escape", "pointer-away"],
}
