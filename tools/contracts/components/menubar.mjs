// Per-component contract: menubar (medium tier — wireMenu + menubar glue)
export default {

  usage: `
React.createElement(Menubar, null,
  React.createElement(MenubarMenu, null,
    React.createElement(MenubarTrigger, { id: "d1-trigger" }, "File"),
    React.createElement(MenubarContent, null,
      React.createElement(MenubarItem, { id: "d1-item-1" }, "New Tab"),
      React.createElement(MenubarItem, null, "Print"),
    ),
  ),
  React.createElement(MenubarMenu, null,
    React.createElement(MenubarTrigger, { id: "d2-trigger" }, "Edit"),
    React.createElement(MenubarContent, null,
      React.createElement(MenubarItem, null, "Undo"),
      React.createElement(MenubarItem, null, "Redo"),
    ),
  ),
)`,

  imports: `
import { Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem } from "@/registry/bases/radix/ui/menubar";
`,

  slots: ["menubar", "menubar-trigger", "menubar-content", "&[data-slot=menubar-item]"],

  open: `await page.click("#d1-trigger")`,
  openShadless: `await page.click("#d1-trigger")`,
  triggerSlot: "menubar-trigger",
  contentSlot: "menubar-content",

  shadlessPage: "src/kernel/menubar.html",

  stateProbe: `(function () {
    var c = document.querySelector("[data-slot=menubar-content]");
    var h = document.querySelector("[data-highlighted]");
    var t1 = document.getElementById("d1-trigger"), t2 = document.getElementById("d2-trigger");
    var f = function (t) { return t.getAttribute("data-state") + "/" + t.getAttribute("aria-expanded") + "/" + t.getAttribute("tabindex"); };
    return (c ? "open" : "closed") + "|" + (h ? h.textContent.trim() : "-")
      + "|" + f(t1) + "|" + f(t2) + "|ae=" + (function (a) {
      if (a.id && (a.id === "d1-trigger" || a.id === "d2-trigger")) return a.id;
      if (a.getAttribute && a.getAttribute("data-slot")) return "content";
      return a.id || a.tagName;
    })(document.activeElement);
  })()`,

  // NOTE: menubar triggers carry role=menuitem themselves — item selectors
  // must target [data-slot=menubar-item], never bare [role=menuitem].
  // Scenarios start from def.open (menu d1 open), so "click:#d1-trigger"
  // re-toggles CLOSE; keyboard roving is exercised after Escape lands in a
  // clean closed state.
  scenarios: [
    "key:Escape",
    "key:Escape+key:ArrowRight",
    "key:ArrowRight",
    "key:ArrowRight+key:ArrowLeft",
    "key:ArrowDown",
    "js-click:[data-slot=menubar-item]",
    "click:#d1-trigger",
  ],
}
