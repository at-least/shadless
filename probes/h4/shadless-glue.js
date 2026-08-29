// generated: shadless dialog glue (h4 spike)
(function () {
  var trigger = document.getElementById("d1-trigger");
  var tpl = document.getElementById("d1-portal");
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

    // h3b contract: pointer-events restored on portal chain (scroll-lock sets
    // body pointer-events:none), overlay aria-hidden, trigger↔content wiring
    overlay.style.pointerEvents = "auto";
    overlay.setAttribute("aria-hidden", "true");
    overlay.setAttribute("data-aria-hidden", "true");
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
      },
    });
    // h3b contract: Close buttons inside content dismiss
    portal.querySelectorAll("[data-slot=dialog-close]").forEach(function (b) {
      b.addEventListener("click", function () { handles && handles.close(true); });
    });
  }

  function unmount(restoreFocus) {
    if (handles) handles.close(restoreFocus !== false);
  }

  trigger.addEventListener("click", function () { open ? unmount() : mount(); });
})();