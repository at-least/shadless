import "./shadless.mjs"
;
// shadless hover-card behavior (wireHoverCard) — registers with the base; multi-instance: every
// [data-slot=hover-card-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("hover-card", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=hover-card-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      if (!tpl.content.querySelector("[data-slot=hover-card-content]")) {
        // the template is the component's contract (dialog family): without
        // a content slot there is nothing to wire — report and stay closed
        console.error("shadless: hover-card template \"" + tpl.id + "\" carries no hover-card-content slot — staying closed");
        return;
      }
      var open = false, current = null;
      var wired = RadixKernel.wireHoverCard({
        trigger: trigger,
        openDelay: 700, closeDelay: 300,  // radix HoverCard.Root defaults
        popperOptions: { sideOffset: 4 }, // shadcn HoverCardContent default
        buildContent: function () {
          var content = shadless.h.mountFromTemplate(tpl, "[data-slot=hover-card-content]");
          content.setAttribute("data-state", "open");
          current = content;
          return content;
        },
        onOpen: function () { open = true; shadless.h.emit(trigger, "open", "hover-card"); },
        onClosed: function () { open = false; current = null; shadless.h.emit(trigger, "close", "hover-card"); },
      });
      shadless.h.bindHandlers(trigger, wired.handlers, w.signal);
      var openFn = function () { if (!open) trigger.dispatchEvent(new PointerEvent("pointerenter", { pointerType: "mouse" })) }
      shadless.instances.set(trigger, { component: "hover-card",
        open: openFn,
        close: function () { if (open) wired.dismiss() },
        toggle: function () { open ? wired.dismiss() : openFn() },
        isOpen: function () { return open },
      })
      // radix HoverCardContent is a DismissableLayer (Escape + outside
      // pointerdown); kernel wireHoverCard has neither — absorb in glue.
      document.addEventListener("keydown", function (e) {
        if (open && e.key === "Escape") wired.dismiss();
      }, { signal: w.signal });
      document.addEventListener("pointerdown", function (e) {
        if (!open || !current) return;
        var t = e.target;
        if (!current.contains(t) && !trigger.contains(t)) wired.dismiss();
      }, { signal: w.signal });
    });
  } })
})()
