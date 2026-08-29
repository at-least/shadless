import "./shadless.mjs"
;
// shadless tooltip behavior (wireTooltip, buildContent pattern) — registers with the base; multi-instance.
// Every [data-slot=tooltip-trigger] with an id "<k>-trigger" is wired to the
// <template id="<k>-portal"> that carries its content (the kernel fixture
// uses t1-*, example pages k0-*, k1-*, …).
(function () {
  var ONCE = {}
  shadless.register("tooltip", { init: function (live) {
    if (!ONCE.provider) { RadixKernel.configureTooltipProvider({ delayDuration: 0 }); ONCE.provider = true }
    var triggers = live.querySelectorAll("[data-slot=tooltip-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var wasOpen = false;
      var wired = RadixKernel.wireTooltip({
        trigger: trigger,
        // machine states: closed / delayed-open / instant-open / …; the
        // consumer-facing edge is closed <-> anything else
        onStateChange: function (s) {
          var isOpen = s !== "closed";
          if (isOpen === wasOpen) return;
          wasOpen = isOpen;
          shadless.h.emit(trigger, isOpen ? "open" : "close", "tooltip");
        },
        buildContent: function (state) {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "tooltip",
        open: function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) },
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? this.open() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()
