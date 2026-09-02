// shadless carousel behavior (vanilla embla) — registers with the base; multi-instance: every
// [data-slot=carousel] root. Orientation: data-orientation when present
// (the shadless fixture), else the content's flex-col (upstream's root
// carries no attribute — the vertical example sets flex-col on the track).
// Buttons follow embla's canScrollPrev/Next; ArrowLeft/ArrowRight scroll.
(function () {
  shadless.register("carousel", { init: function (live) {
    if (typeof EmblaCarousel !== "function") return;
    var roots = live.querySelectorAll("[data-slot=carousel]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      var viewport = root.querySelector("[data-slot=carousel-content]");
      if (!viewport || !viewport.firstElementChild) return;
      var vertical = root.getAttribute("data-orientation") === "vertical" ||
        (!root.hasAttribute("data-orientation") && viewport.firstElementChild.classList.contains("flex-col"));
      var api = EmblaCarousel(viewport, { axis: vertical ? "y" : "x", direction: (root.getAttribute("dir") || document.documentElement.getAttribute("dir")) === "rtl" ? "rtl" : "ltr" });
      var prevBtn = root.querySelector("[data-slot=carousel-previous]");
      var nextBtn = root.querySelector("[data-slot=carousel-next]");
      function update() {
        if (prevBtn) prevBtn.disabled = !api.canScrollPrev();
        if (nextBtn) nextBtn.disabled = !api.canScrollNext();
      }
      api.on("select", update); api.on("reInit", update); update();
      if (prevBtn) prevBtn.addEventListener("click", function () { api.scrollPrev(); }, { signal: w.signal });
      if (nextBtn) nextBtn.addEventListener("click", function () { api.scrollNext(); }, { signal: w.signal });
      root.addEventListener("keydown", function (e) {
        if (e.key === "ArrowLeft") { e.preventDefault(); api.scrollPrev(); }
        else if (e.key === "ArrowRight") { e.preventDefault(); api.scrollNext(); }
      }, { signal: w.signal });
      w.teardown = function () { api.destroy(); delete root.__embla; };
      root.__embla = api;
      api.component = "carousel";
      shadless.instances.set(root, api); // the embla api itself: scrollNext(), scrollTo(i), on("select", …), …
    });
  } })
})()
