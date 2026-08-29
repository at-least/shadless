

window.__log = [];
window.__facts = function (tag) {
  var doc = document;
  function attrs(el) {
    if (!el) return null;
    var o = { tag: el.tagName.toLowerCase() };
    for (var a of el.getAttributeNames()) if (a !== "class") o[a] = el.getAttribute(a);
    o.text = (el.textContent || "").trim().slice(0, 24);
    return o;
  }
  window.__log.push({
    step: tag,
    bodyChildren: Array.from(doc.body.children).map(function (c) {
      var s = c.getAttribute("data-slot");
      return c.tagName.toLowerCase() + (s ? "[slot=" + s + "]"
        : c.getAttribute("role") ? "[role=" + c.getAttribute("role") + "]"
        : c.tagName === "SPAN" ? "[guard?]" : "");
    }),
    trigger: attrs(doc.querySelector("[data-slot=dialog-trigger]")),
    overlay: attrs(doc.querySelector("[data-slot=dialog-overlay]")),
    content: attrs(doc.querySelector("[data-slot=dialog-content], [role=dialog]")),
    title: attrs(doc.querySelector("[data-slot=dialog-title]")),
    description: attrs(doc.querySelector("[data-slot=dialog-description]")),
    close: attrs(doc.querySelector("[data-slot=dialog-close]")),
    activeElement: doc.activeElement
      ? (doc.activeElement.getAttribute("data-slot") || doc.activeElement.id ||
         doc.activeElement.tagName.toLowerCase()) : null,
    scrollLock: {
      attr: doc.body.getAttribute("data-scroll-locked"),
      overflow: doc.body.style.overflow,
      pointerEvents: doc.body.style.pointerEvents,
      paddingRight: doc.body.style.paddingRight !== "",
    },
  });
};

window.__scenario = async function (kind) {
  var doc = document;
  if (kind === "open") doc.getElementById("d1-trigger").click();
  else if (kind === "overlay-pointerdown")
    doc.querySelector("[data-slot=dialog-overlay]").dispatchEvent(
      new PointerEvent("pointerdown", { bubbles: true, cancelable: true, pointerId: 1 }));
  else if (kind === "overlay-click")
    doc.querySelector("[data-slot=dialog-overlay]").dispatchEvent(
      new MouseEvent("click", { bubbles: true, cancelable: true }));
  else if (kind === "escape")
    doc.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true }));
  else if (kind === "close-button")
    doc.querySelector("[data-slot=dialog-close]").click();
  else if (kind === "reopen") doc.getElementById("d1-trigger").click();
  await new Promise(function (r) { setTimeout(r, 300) });
  var gone = !doc.querySelector("[data-slot=dialog-content], [role=dialog]");
  window.__facts("after-" + kind);
  return gone ? "closes" : "open";
};
