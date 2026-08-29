import "./shadless.mjs"
;
// shadless switch behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  function set(root, checked) {
    h.setChecked(root, checked)
    var thumb = root.querySelector("[data-slot=switch-thumb]")
    if (thumb) thumb.setAttribute("data-state", checked ? "checked" : "unchecked")
    h.syncForm(root)
  }
  shadless.register("switch", { slots: {
    switch: {
      init: function (root) {
        h.formMirror(root, {
          read: function () { return root.getAttribute("aria-checked") === "true" },
          write: function (v) { set(root, v) },
        })
      },
      onClick: function (root) {
        var checked = root.getAttribute("aria-checked") !== "true"
        set(root, checked)
        h.emit(root, "change", "switch", { checked: checked })
      },
    },
    // toggle: aria-pressed + data-state on/off (radix Toggle).,
  } })
})()
