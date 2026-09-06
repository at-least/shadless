import "./shadless.mjs"
;
// shadless alert-dialog behavior (wireDialog) — registers with
// shadless.h.wireDialogFamily, the shared portal-glue also used by
// dialog.js/sheet.js (core.js has the full story). Every
// [data-slot=alert-dialog-trigger] "<k>-trigger" wires to a
// <template id="<k>-portal">; Action/Cancel both dismiss (radix
// Action/Cancel close the dialog), and — unlike plain dialog — clicking the
// overlay must NOT dismiss, so overlay clicks are swallowed before
// wireDialog's portal click-close can see them.
(function () {
  shadless.register("alert-dialog", { init: shadless.h.wireDialogFamily("alert-dialog", {
    dismissSelector: "[data-slot=alert-dialog-action], [data-slot=alert-dialog-cancel]",
    swallowOverlayClick: true,
  }) })
})()
