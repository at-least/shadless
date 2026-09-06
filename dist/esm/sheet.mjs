import "./shadless.mjs"
;
// shadless sheet behavior (wireDialog) — registers with
// shadless.h.wireDialogFamily, the shared portal-glue also used by
// dialog.js/alert-dialog.js (core.js has the full story). sheet IS radix
// Dialog with sheet slot names; the content's direct-child X button
// dismisses it.
(function () {
  shadless.register("sheet", { init: shadless.h.wireDialogFamily("sheet", {
    dismissSelector: "[data-slot=sheet-content] > button",
  }) })
})()
