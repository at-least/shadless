// Per-component contract definition. The usage tree mirrors how a consumer
// uses the shadcn component (oracle side renders it with React).
export default {
  name: "dialog",

  // React usage tree for the oracle entry (runs in browser after bundle).
  usage: `
React.createElement(Dialog, {
  open: window.__open, onOpenChange: (o) => window.__setOpen(o),
},
  React.createElement(DialogTrigger, { id: "d1-trigger" }, "Open dialog"),
  React.createElement(DialogContent, null,
    React.createElement(DialogHeader, null,
      React.createElement(DialogTitle, null, "Are you absolutely sure?"),
      React.createElement(DialogDescription, null, "This action cannot be undone."),
    ),
  ),
)`,

  imports: `
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/registry/bases/radix/ui/dialog";
`,

  // slots recorded as facts (data-slot selectors)
  slots: [
    "dialog-trigger", "dialog-overlay", "dialog-content",
    "dialog-title", "dialog-description", "dialog-close",
  ],

  // open the shadless-side page (oracle renders already-open)
  openShadless: `await page.click("#d1-trigger")`,

  // shadless page (until T5/T6 emit real output): h4 spike demo
  // fixture is slot-keyed: mounted content has no inline classes by
  // design (styles ride out.css [data-slot] rules) — class compare off
  mountedClasses: false,
  // controlled-open oracle (window.__open=true): content mounts at INITIAL render, not via the open step — the before/after bag diff is inapplicable
  mountedCheck: false,
  shadlessPage: "src/kernel/dialog.html",

  // real-mouse / keyboard scenarios; result = "closes" | "open"
  scenarios: ["overlay-mouse-click", "escape", "close-button"],
  triggerSlot: "dialog-trigger",
}
