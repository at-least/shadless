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
      if (!tpl.content || !tpl.content.querySelector("[data-slot=tooltip-content]")) {
        // the template is the component's contract (dialog family): without
        // a content slot there is nothing to wire — report and stay closed
        console.error("shadless: tooltip template \"" + tpl.id + "\" carries no tooltip-content slot — staying closed");
        return;
      }
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
          var content = shadless.h.mountFromTemplate(tpl, "[data-slot=tooltip-content]");
          content.setAttribute("data-state", state);
          var arrow = content.querySelector("svg");
          return arrow ? { content: content, arrow: arrow } : { content: content };
        },
      });
      shadless.h.bindHandlers(trigger, wired.handlers, w.signal);
      var openFn = function () { if (wired.state() === "closed") trigger.dispatchEvent(new FocusEvent("focus")) }
      shadless.instances.set(trigger, { component: "tooltip",
        open: openFn,
        close: function () { wired.close() },
        toggle: function () { wired.state() === "closed" ? openFn() : wired.close() },
        isOpen: function () { return wired.state() !== "closed" },
      })
    });
  } })
})()
