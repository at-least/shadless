// shadless scroll-area behavior (wireScrollArea, type=hover default) — registers with the base; multi-instance
(function () {
  shadless.register("scroll-area", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=scroll-area]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var viewport = root.querySelector("[data-slot=scroll-area-viewport]");
      if (!viewport) return;
      var scrollbars = {};
      Array.prototype.forEach.call(root.querySelectorAll("[data-slot=scroll-area-scrollbar]"), function (bar) {
        var o = bar.getAttribute("data-orientation") === "horizontal" ? "horizontal" : "vertical";
        scrollbars[o] = { scrollbar: bar, thumb: bar.querySelector("[data-slot=scroll-area-thumb]") };
      });
      var wired = RadixKernel.wireScrollArea({
        root: root,
        viewport: viewport,
        content: viewport.firstElementChild,
        scrollbars: scrollbars,
      });
      w.teardown = function () { if (wired && wired.destroy) wired.destroy() };
    });
  } })
})()
