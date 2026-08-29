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
      onClick: function (root, ctx) {
        var checked = root.getAttribute("aria-checked") !== "true"
        set(root, checked, ctx)
        h.emit(root, "change", "checkbox", { checked: checked })
      },
    },
    // switch: thumb is always in DOM; root + thumb data-state stay in sync.,
  } })
})()
