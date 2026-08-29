
import React from "react";
import { createRoot } from "react-dom/client";
import * as D from "@radix-ui/react-dialog";

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

const tree = (open) => React.createElement(D.Root, {
  open,
  onOpenChange: (o) => { window.__open = o; root.render(tree(o)) },
},
  React.createElement(D.Trigger, { "data-slot": "dialog-trigger", id: "rt-trigger" }, "Open"),
  React.createElement(D.Portal, { "data-slot": "dialog-portal" },
    React.createElement(D.Overlay, { "data-slot": "dialog-overlay", className: "ov" }),
    React.createElement(D.Content, { "data-slot": "dialog-content", className: "ct" },
      React.createElement(D.Title, { "data-slot": "dialog-title" }, "Sure?"),
      React.createElement(D.Description, { "data-slot": "dialog-description" }, "Undone."),
      React.createElement(D.Close, { "data-slot": "dialog-close" }, "x"),
    ),
  ),
);
const root = createRoot(document.getElementById("root"));
root.render(tree(true));
window.__scenario = async function (kind) {
  var doc = document;
  if (kind === "overlay-pointerdown")
    doc.querySelector("[data-slot=dialog-overlay]").dispatchEvent(
      new PointerEvent("pointerdown", { bubbles: true, cancelable: true, pointerId: 1 }));
  else if (kind === "overlay-click")
    doc.querySelector("[data-slot=dialog-overlay]").dispatchEvent(
      new MouseEvent("click", { bubbles: true, cancelable: true }));
  else if (kind === "escape")
    doc.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true }));
  else if (kind === "close-button")
    doc.querySelector("[data-slot=dialog-close]").click();
  else if (kind === "reopen") root.render(tree(true));
  await new Promise(function (r) { setTimeout(r, 250) });
  var gone = !doc.querySelector("[data-slot=dialog-content], [role=dialog]");
  window.__facts("after-" + kind);
  return gone ? "closes" : "open";
};
setTimeout(function () { window.__facts("primitives-open") }, 150);
