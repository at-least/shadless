
import React from "react";
import { createRoot } from "react-dom/client";
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/registry/new-york-v4/ui/dialog";

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
        : c.tagName === "SPAN" ? "[guard?]" : "");
    }),
    trigger: attrs(doc.querySelector("[data-slot=dialog-trigger]")),
    overlay: attrs(doc.querySelector("[data-slot=dialog-overlay]")),
    content: attrs(doc.querySelector("[data-slot=dialog-content]")),
    title: attrs(doc.querySelector("[data-slot=dialog-title]")),
    description: attrs(doc.querySelector("[data-slot=dialog-description]")),
    close: attrs(doc.querySelector("[data-slot=dialog-close]")),
    activeElement: doc.activeElement
      ? (doc.activeElement.getAttribute("data-slot") ||
         doc.activeElement.tagName.toLowerCase()) : null,
    scrollLock: {
      attr: doc.body.getAttribute("data-scroll-locked"),
      pointerEvents: doc.body.style.pointerEvents,
    },
  });
};

const tree = (open) => React.createElement(Dialog, {
  open, onOpenChange: (o) => { root.render(tree(o)) },
},
  React.createElement(DialogTrigger, { id: "d1-trigger" }, "Open dialog"),
  React.createElement(DialogContent, null,
    React.createElement(DialogHeader, null,
      React.createElement(DialogTitle, null, "Are you absolutely sure?"),
      React.createElement(DialogDescription, null, "This action cannot be undone."),
    ),
  ),
);
const root = createRoot(document.getElementById("root"));
root.render(tree(true));
setTimeout(function () { window.__facts("shadcn-open") }, 200);
