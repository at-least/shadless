// shadless popover behavior (wirePopover) — registers with the base; multi-instance: every
// [data-slot=popover-trigger] "<k>-trigger" ↔ <template id="<k>-portal">.
(function () {
  shadless.register("popover", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=popover-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-portal"));
      if (!tpl) return;
      var open = false, handles = null, content = null;

      function setState(s) {
        trigger.setAttribute("data-state", s);
        trigger.setAttribute("aria-expanded", s === "open" ? "true" : "false");
        if (content) content.setAttribute("data-state", s);
      }
      function mount() {
        var frag = tpl.content.cloneNode(true);
        var host = document.createElement("div");
        host.appendChild(frag);
        content = host.querySelector("[data-slot=popover-content]");
        document.body.appendChild(content);
        setState("open");
        handles = RadixKernel.wirePopover({
          content: content,
          trigger: trigger,
          contentStateFlip: true,
          onClosed: function () {
            setState("closed");
            if (content) content.remove();
            content = null; handles = null; open = false;
            shadless.h.emit(trigger, "close", "popover");
          },
        });
        open = true;
        shadless.h.emit(trigger, "open", "popover");
      }
      function unmount(restoreFocus) { if (handles) handles.close(restoreFocus !== false); }
      trigger.addEventListener("click", function () { open ? unmount() : mount(); }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "popover",
        open: function () { if (!open) mount() },
        close: function (restoreFocus) { if (open) unmount(restoreFocus) },
        toggle: function () { open ? unmount() : mount() },
        isOpen: function () { return open },
      })
    });
  } })
})()
