// shadless tabs behavior (wireTabs) — registers with the base; multi-instance: every [data-slot=tabs] root.
// kernel handles click/arrow activation (aria-selected, data-state, hidden,
// roving tabindex on interaction); behavior wires the static a11y surface:
// roving tabindex initial value (kernel only sets it on interactive activation)
(function () {
  shadless.register("tabs", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=tabs]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var list = root.querySelector("[data-slot=tabs-list]");
      if (!list) return;
      var triggers = Array.prototype.slice.call(list.querySelectorAll("[data-slot=tabs-trigger]"));
      var panels = triggers.map(function (t) {
        var id = t.getAttribute("aria-controls");
        return (id && document.getElementById(id)) || null;
      });
      if (!triggers.length) return;
      var initial = Math.max(0, triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active"; }));
      // panel-less tab lists (tabs-icons, tabs-line, …: triggers only): the
      // kernel's wireTabs needs panels, so activation is wired here — click
      // and arrow keys flip data-state / aria-selected / roving tabindex
      if (panels.some(function (p) { return !p; })) {
        var current = -1;
        var activate = function (i) {
          triggers.forEach(function (t, k) {
            var on = k === i;
            t.setAttribute("data-state", on ? "active" : "inactive");
            t.setAttribute("aria-selected", on ? "true" : "false");
            t.setAttribute("tabindex", on ? "0" : "-1");
          });
          var changed = current !== -1 && current !== i;
          current = i;
          if (changed) shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        };
        triggers.forEach(function (t, i) {
          t.addEventListener("click", function () { if (!t.disabled) activate(i); }, { signal: w.signal });
        });
        list.addEventListener("keydown", function (e) {
          var idx = triggers.indexOf(document.activeElement);
          if (idx === -1) return;
          if (shadless.h.NAV_KEYS.indexOf(e.key) < 0) return;
          var next = shadless.h.nextIndex(e, triggers);
          if (next === -1) return;
          e.preventDefault();
          activate(next); triggers[next].focus();
        }, { signal: w.signal });
        activate(initial);
        shadless.instances.set(root, { component: "tabs",
          activate: function (i) { activate(i) },
          active: function () { return triggers.findIndex(function (t) { return t.getAttribute("data-state") === "active" }) },
        })
        return;
      }
      w.persistent = true // kernel wireTabs holds list/trigger listeners with no unwire
      triggers.forEach(function (t) { t.setAttribute("tabindex", "-1"); }); // radix roving: all -1 until focus enters
      // kernel calls onChange on wire (its initial activate) and on every
      // activate, same index included; radix onValueChange fires only when
      // the value CHANGES — emit on real transitions after wiring
      var last = initial, ready = false;
      var wired = RadixKernel.wireTabs({ list: list, triggers: triggers, panels: panels, initial: initial,
        onChange: function (i) {
          if (!ready || i === last) { last = i; return; }
          last = i;
          shadless.h.emit(root, "change", "tabs", { index: i, trigger: triggers[i] });
        } });
      ready = true;
      shadless.instances.set(root, { component: "tabs",
        activate: function (i, focus) { wired.activate(i, !!focus) },
        active: function () { return wired.active() },
      })
      // kernel computes ArrowLeft/ArrowRight from the ACTIVE index; radix moves
      // relative to the FOCUSED trigger (focus alone doesn't activate) — absorb in
      // glue with a capture-phase handler; kernel keeps click/Home/End.
      list.addEventListener("keydown", function (e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var idx = triggers.indexOf(document.activeElement);
        if (idx === -1) return;
        e.preventDefault(); e.stopPropagation();
        var next = shadless.h.nextIndex(e, triggers);
        if (next === -1) return;
        wired.activate(next, true);
        triggers[next].focus();
      }, true);
    });
  } })
})()
