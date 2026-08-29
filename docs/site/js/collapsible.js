// shadless collapsible behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("collapsible", { slots: {
    "collapsible-trigger": {
      init: function (trigger) {
        var root = trigger.closest("[data-slot=collapsible]")
        h.linkControls(trigger, root && root.querySelector("[data-slot=collapsible-content]"))
      },
      onClick: function (trigger) {
        var root = trigger.closest("[data-slot=collapsible]")
        var content = root && root.querySelector("[data-slot=collapsible-content]")
        if (!root) return
        var open = trigger.getAttribute("data-state") !== "open"
        h.setDisclosed(trigger, content, open)
        root.setAttribute("data-state", open ? "open" : "closed")
        h.emit(trigger, open ? "open" : "close", "collapsible")
      },
    },
    // accordion trigger: type=single (default) closes siblings; data-type=
    // multiple toggles items independently. Siblings without an item/content
    // ancestor are skipped, not crashed on.,
  } })
})()
