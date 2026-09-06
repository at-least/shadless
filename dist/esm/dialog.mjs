import "./shadless.mjs"
;
// shadless dialog behavior (wireDialog) — registers with shadless.h.wireDialogFamily,
// the shared portal-glue also used by alert-dialog.js/sheet.js (core.js has
// the full story). Every [data-slot=dialog-trigger] "<k>-trigger" wires to a
// <template id="<k>-portal">; the content's own [data-slot=dialog-close]
// buttons dismiss it.
(function () {
  shadless.register("dialog", { init: shadless.h.wireDialogFamily("dialog", {
    dismissSelector: "[data-slot=dialog-close]",
  }) })
})()
