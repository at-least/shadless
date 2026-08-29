// shadless dialog behavior (wireDialog) — registers with the base; multi-instance: every
// [data-slot=dialog-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("dialog", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=dialog-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, portal = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        var o = portal && portal.querySelector("[data-slot=dialog-overlay]");
        var c = portal && portal.querySelector("[data-slot=dialog-content]");
        if (o) o.setAttribute("data-state", s);
        if (c) c.setAttribute("data-state", s);
      }

      function mount() {
        portal = document.createElement("div");
        portal.setAttribute("data-slot", "dialog-portal");
        portal.appendChild(tpl.content.cloneNode(true));
        document.body.appendChild(portal);
        var overlay = portal.querySelector("[data-slot=dialog-overlay]");
        var content = portal.querySelector("[data-slot=dialog-content]");
        // pointer-events restored on the portal chain, overlay aria-hidden,
        // trigger<->content wiring (the h3b contract)
        if (overlay) {
          overlay.style.pointerEvents = "auto";
          overlay.setAttribute("aria-hidden", "true");
          overlay.setAttribute("data-aria-hidden", "true");
        }
        content.style.pointerEvents = "auto";
        trigger.setAttribute("aria-controls", content.id);

        setState("open");
        handles = RadixKernel.wireDialog({
          content: content,
          portal: portal,
          trigger: trigger,
          onClosed: function () {
            setState("closed");
            portal.remove();
            portal = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "dialog");
          },
        });
        portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
          b.addEventListener("click", function () { handles && handles.close(true); });
        });
        open = true;
        shadless.h.emit(trigger, "open", "dialog");
      }

      function unmount(restoreFocus) {
        if (handles) handles.close(restoreFocus !== false);
      }

      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "dialog",
        open: function () { if (!open) mount() },
        close: function (restoreFocus) { if (open) unmount(restoreFocus) },
        toggle: function () { open ? unmount() : mount() },
        isOpen: function () { return open },
      })
    });
  } })
})()
