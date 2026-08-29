// Per-component contract: alert-dialog (Wave B, kernel wireDialog + overlay
// click swallow). Semantics: overlay-click does NOT dismiss; Escape does;
// Action/Cancel dismiss; open-autofocus lands on Cancel.
export default {
  name: "alert-dialog",

  usage: `
React.createElement(AlertDialog, {
  open: window.__open, onOpenChange: (o) => window.__setOpen(o),
},
  React.createElement(AlertDialogTrigger, { id: "d1-trigger" }, "Show Dialog"),
  React.createElement(AlertDialogContent, null,
    React.createElement(AlertDialogHeader, null,
      React.createElement(AlertDialogTitle, null, "Are you absolutely sure?"),
      React.createElement(AlertDialogDescription, null, "This action cannot be undone. This will permanently delete your account and remove your data from our servers."),
    ),
    React.createElement(AlertDialogFooter, null,
      React.createElement(AlertDialogCancel, null, "Cancel"),
      React.createElement(AlertDialogAction, null, "Continue"),
    ),
  ),
)`,

  imports: `
import { AlertDialog, AlertDialogTrigger, AlertDialogContent, AlertDialogHeader, AlertDialogFooter, AlertDialogTitle, AlertDialogDescription, AlertDialogCancel, AlertDialogAction } from "@/registry/bases/radix/ui/alert-dialog";
`,

  slots: [
    "alert-dialog-trigger", "alert-dialog-overlay", "alert-dialog-content",
    "alert-dialog-title", "alert-dialog-description",
    "alert-dialog-cancel", "alert-dialog-action",
  ],

  openShadless: `await page.click("#d1-trigger")`,

  // fixture is slot-keyed: mounted content has no inline classes by
  // design (styles ride out.css [data-slot] rules) — class compare off
  mountedClasses: false,
  // controlled-open oracle (window.__open=true): content mounts at INITIAL render, not via the open step — the before/after bag diff is inapplicable
  mountedCheck: false,
  shadlessPage: "src/kernel/alert-dialog.html",

  scenarios: [
    "overlay-mouse-click", // stays open on BOTH sides
    "escape",
    "click:[data-slot=alert-dialog-cancel]",
    "click:[data-slot=alert-dialog-action]",
  ],
  triggerSlot: "alert-dialog-trigger",
  contentSlot: "alert-dialog-content",
  overlaySlot: "alert-dialog-overlay",
}
