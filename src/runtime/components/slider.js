// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
