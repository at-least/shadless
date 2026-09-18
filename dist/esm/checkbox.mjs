import "./shadless.mjs"
;
// shadless checkbox behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  function set(root, checked, ctx) {
    h.setChecked(root, checked)
    var ind = root.querySelector("[data-slot=checkbox-indicator]")
    if (checked && !ind) {
      var node = h.cloneTemplate(h.findTemplate(ctx, "checkbox-indicator"))
      if (node) root.appendChild(node)
    } else if (!checked && ind) ind.remove()
    h.syncForm(root)
  }
  shadless.register("checkbox", { slots: {
    checkbox: {
      init: function (root) {
        h.formMirror(root, {
          read: function () { return root.getAttribute("aria-checked") === "true" },
          write: function (v) { set(root, v, null) },
        })
      },
      onKeydown: function (root, ctx, ev) {
        // radix Checkbox ignores Enter: its click handler preventDefaults the
        // native button's Enter-synthesized click (Space stays native) —
        // measured 2026-08-22, pinned by the contract's key:Enter scenario,
        // which this page failed until the quirk was ported
        if (ev.key === "Enter") ev.preventDefault()
      },
      onClick: function (root, ctx) {
        var checked = root.getAttribute("aria-checked") !== "true"
        set(root, checked, ctx)
        h.emit(root, "change", "checkbox", { checked: checked })
      },
    },
  } })
})()
