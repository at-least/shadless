// shadless accordion behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("accordion", { slots: {
    "accordion-trigger": {
      init: function (trigger) {
        var item = trigger.closest("[data-slot=accordion-item]")
        h.linkControls(trigger, item && item.querySelector("[data-slot=accordion-content]"))
      },
      onClick: function (trigger) {
        var item = trigger.closest("[data-slot=accordion-item]")
        var accordion = trigger.closest("[data-slot=accordion]")
        if (!item || !accordion) return
        var wasOpen = trigger.getAttribute("data-state") === "open"
        var multiple = accordion.getAttribute("data-type") === "multiple"
        if (!multiple) {
          var triggers = accordion.querySelectorAll("[data-slot=accordion-trigger]")
          triggers.forEach(function (t) {
            var it = t.closest("[data-slot=accordion-item]")
            if (!it) return
            var c = it.querySelector("[data-slot=accordion-content]")
            var wasOpenSibling = t !== trigger && t.getAttribute("data-state") === "open"
            h.setDisclosed(t, c, false)
            it.setAttribute("data-state", "closed")
            if (wasOpenSibling) h.emit(t, "close", "accordion")
          })
        }
        var open = !wasOpen
        var content = item.querySelector("[data-slot=accordion-content]")
        h.setDisclosed(trigger, content, open)
        item.setAttribute("data-state", open ? "open" : "closed")
        h.emit(trigger, open ? "open" : "close", "accordion")
      },
    },
    // accordion arrows/home/end: move focus between triggers only,
    "accordion": {
      onKeydown: function (accordion, ctx, ev) {
        if (h.NAV_KEYS.indexOf(ev.key) < 0) return
        ev.preventDefault()
        var items = [...accordion.querySelectorAll("[data-slot=accordion-trigger]")]
        if (!items.length) return
        var next = items[h.nextIndex(ev, items)]
        if (next) next.focus()
      },
    },
    // toggle-group: single (items role=radio) selects exclusively with click-
    // again deselecting; multiple (items as plain buttons) toggles items independently.
    // Root is role=group in BOTH modes (radix current semantics — radiogroup
    // was the old single-mode root role).
    // Roving tabindex tracks the active item in both modes.,
  } })
})()
