// Per-component contract: sheet (Wave B, kernel wireDialog — sheet IS radix Dialog)
export default {
  name: "sheet",

  usage: `
React.createElement(Sheet, {
  open: window.__open, onOpenChange: (o) => window.__setOpen(o),
},
  React.createElement(SheetTrigger, { id: "d1-trigger" }, "Open sheet"),
  React.createElement(SheetContent, null,
    React.createElement(SheetHeader, null,
      React.createElement(SheetTitle, null, "Are you absolutely sure?"),
      React.createElement(SheetDescription, null, "This action cannot be undone."),
    ),
  ),
)`,

  imports: `
import { Sheet, SheetTrigger, SheetContent, SheetHeader, SheetTitle, SheetDescription } from "@/registry/bases/radix/ui/sheet";
`,

  // upstream sheet close button has NO data-slot -> not recorded as a fact slot
  slots: ["sheet-trigger", "sheet-overlay", "sheet-content", "sheet-title", "sheet-description"],

  openShadless: `await page.click("#d1-trigger")`,

  // fixture is slot-keyed: mounted content has no inline classes by
  // design (styles ride out.css [data-slot] rules) — class compare off
  mountedClasses: false,
  // controlled-open oracle: content mounts at INITIAL render — mounted-diff inapplicable
  mountedCheck: false,
  shadlessPage: "src/kernel/sheet.html",

  scenarios: ["overlay-mouse-click", "escape", "close-button"],
  triggerSlot: "sheet-trigger",
  contentSlot: "sheet-content",
  overlaySlot: "sheet-overlay",
  closeSelector: "[data-slot=sheet-content]>button",
}
