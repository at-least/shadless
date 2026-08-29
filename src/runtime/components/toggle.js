// shadless toggle behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle", { slots: {
    toggle: {
      onClick: function (root) {
        var on = root.getAttribute("aria-pressed") !== "true"
        root.setAttribute("aria-pressed", String(on))
        root.setAttribute("data-state", on ? "on" : "off")
        h.emit(root, "change", "toggle", { pressed: on })
      },
    },
    // radio-group: click checks exclusively; checked item cannot be unchecked,
  } })
})()
