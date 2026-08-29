import "./shadless.mjs"
;
// shadless radio-group behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  shadless.register("radio-group", { slots: {
    "radio-group-item": {
      onClick: function (item, ctx) {
        var group = item.closest("[data-slot=radio-group]")
        if (!group) return
        var items = group.querySelectorAll("[data-slot=radio-group-item]")
        var was = item.getAttribute("aria-checked") === "true"
        items.forEach(function (it) { h.setRadioItem(it, it === item, ctx) })
        item.setAttribute("tabindex", "0")
        h.syncForm(group)
        if (!was) h.emit(group, "change", "radio-group", { value: h.itemValue(item), item: item })
      },
    },
    // radio-group arrows: move focus; with NO item checked they also check the
    // target (measured radix quirk, probes/t7/probe-keys3.mjs),
    "radio-group": {
      init: function (group) {
        var checked = function () { return group.querySelector("[data-slot=radio-group-item][aria-checked=true]") }
        h.formMirror(group, {
          read: function () { var it = checked(); return it ? h.itemValue(it) : null },
          write: function (v) {
            group.querySelectorAll("[data-slot=radio-group-item]").forEach(function (it) {
              h.setRadioItem(it, v != null && h.itemValue(it) === v, null)
            })
          },
        })
      },
      onKeydown: function (group, ctx, ev) {
        if (h.NAV_KEYS.indexOf(ev.key) < 0) return
        ev.preventDefault()
        var items = [...group.querySelectorAll("[data-slot=radio-group-item]")]
        if (!items.length) return
        var next = items[h.nextIndex(ev, items)]
        if (!next) return
        var none = !items.some(function (it) { return it.getAttribute("aria-checked") === "true" })
        if (none) {
          items.forEach(function (it) { h.setRadioItem(it, it === next, ctx) })
          h.syncForm(group)
          h.emit(group, "change", "radio-group", { value: h.itemValue(next), item: next })
        }
        items.forEach(function (it) { it.setAttribute("tabindex", it === next ? "0" : "-1") })
        next.focus()
      },
    },
    // avatar: settle image vs fallback from load state (radix Presence),
  } })
})()
