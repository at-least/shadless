// shadless toggle-group behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("toggle-group", { slots: {
    "toggle-group-item": {
      onClick: function (item) {
        var group = item.closest("[data-slot=toggle-group]")
        if (!group) return
        var wasOn = item.getAttribute("data-state") === "on"
        var items = group.querySelectorAll("[data-slot=toggle-group-item]")
        // mode from the ITEM's own role (root is role=group in both modes
        // under radix current semantics): single items are role=radio with
        // aria-checked; multiple items are plain buttons with aria-pressed
        if (item.getAttribute("role") === "radio") {
          items.forEach(function (it) { h.setGroupItem(it, it === item && !wasOn) })
        } else {
          item.setAttribute("aria-pressed", String(!wasOn))
          item.setAttribute("data-state", !wasOn ? "on" : "off")
          items.forEach(function (it) { if (it !== item) it.setAttribute("tabindex", "-1") })
        }
        item.setAttribute("tabindex", "0")
        item.focus()
        // single: the on item's value or null; multiple: every on item's value
        var on = Array.prototype.filter.call(items, function (it) { return it.getAttribute("data-state") === "on" }).map(h.itemValue)
        h.emit(group, "change", "toggle-group", { value: item.getAttribute("role") === "radio" ? (on[0] || null) : on, item: item })
      },
    },
    // arrows/home/end move focus only (radix roving focus); selection happens
    // via Space/Enter (= click). From non-item focus, arrows land on the LAST
    // item (measured radix quirk, probes/t7/probe-keys.mjs), Home on the first.,
    "toggle-group": {
      onKeydown: function (group, ctx, ev) {
        if (h.NAV_KEYS.indexOf(ev.key) < 0) return
        ev.preventDefault()
        var items = [...group.querySelectorAll("[data-slot=toggle-group-item]")]
        if (!items.length) return
        var next = items[h.nextIndex(ev, items)]
        if (!next) return
        items.forEach(function (it) { it.setAttribute("tabindex", it === next ? "0" : "-1") })
        next.focus()
      },
    },
  } })
})()
