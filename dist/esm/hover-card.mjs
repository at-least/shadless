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
      var open = false, current = null;
      var wired = RadixKernel.wireHoverCard({
        trigger: trigger,
        openDelay: 700, closeDelay: 300,  // radix HoverCard.Root defaults
        popperOptions: { sideOffset: 4 }, // shadcn HoverCardContent default
        buildContent: function () {
          var frag = tpl.content.cloneNode(true);
          var host = document.createElement("div");
          host.appendChild(frag);
          var content = host.querySelector("[data-slot=hover-card-content]");
          content.setAttribute("data-state", "open");
          current = content;
          return content;
        },
        onOpen: function () { open = true; shadless.h.emit(trigger, "open", "hover-card"); },
        onClosed: function () { open = false; current = null; shadless.h.emit(trigger, "close", "hover-card"); },
      });
      for (var ev in wired.handlers)
        trigger.addEventListener(ev.slice(2), wired.handlers[ev], { signal: w.signal });
      shadless.instances.set(trigger, { component: "hover-card",
        open: function () { if (!open) trigger.dispatchEvent(new PointerEvent("pointerenter", { pointerType: "mouse" })) },
        close: function () { if (open) wired.dismiss() },
        toggle: function () { open ? wired.dismiss() : this.open() },
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
