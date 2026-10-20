// Per-component contract: menubar (medium tier — wireMenu + menubar glue)
export default {

  usage: `React.createElement(Demo)`,

  // radix checkbox / radio items hold no state of their own — a menu that
  // toggles is one whose app holds it (the menubar-radio example), so the
  // fixture declares that app
  imports: `
import { Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem, MenubarCheckboxItem, MenubarRadioGroup, MenubarRadioItem } from "@/registry/bases/radix/ui/menubar";
function Demo() {
  const [bar, setBar] = React.useState(true);
  const [urls, setUrls] = React.useState(false);
  const [theme, setTheme] = React.useState("light");
  return React.createElement(Menubar, null,
    React.createElement(MenubarMenu, null,
      React.createElement(MenubarTrigger, { id: "d1-trigger" }, "File"),
      React.createElement(MenubarContent, null,
        React.createElement(MenubarItem, { id: "d1-item-1" }, "New Tab"),
        React.createElement(MenubarItem, null, "Print"),
        React.createElement(MenubarCheckboxItem, { checked: bar, onCheckedChange: setBar }, "Bookmarks Bar"),
        React.createElement(MenubarCheckboxItem, { checked: urls, onCheckedChange: setUrls }, "Full URLs"),
        React.createElement(MenubarRadioGroup, { value: theme, onValueChange: setTheme },
          React.createElement(MenubarRadioItem, { value: "light" }, "Light"),
          React.createElement(MenubarRadioItem, { value: "dark" }, "Dark"),
        ),
      ),
    ),
    React.createElement(MenubarMenu, null,
      React.createElement(MenubarTrigger, { id: "d2-trigger" }, "Edit"),
      React.createElement(MenubarContent, null,
        React.createElement(MenubarItem, null, "Undo"),
        React.createElement(MenubarItem, null, "Redo"),
      ),
    ),
  );
}
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
    // each checkbox / radio item's state and whether it shows the check
    // (radix mounts ItemIndicator in checked items only)
    var items = c ? Array.prototype.map.call(c.querySelectorAll("[role=menuitemcheckbox], [role=menuitemradio]"), function (i) {
      return i.textContent.trim() + ":" + i.getAttribute("data-state") + ":" + (i.firstElementChild.querySelector("svg") ? "check" : "-");
    }).join(",") : "";
    return (c ? "open" : "closed") + "|" + (h ? h.textContent.trim() : "-")
      + "|" + f(t1) + "|" + f(t2) + "|ae=" + (function (a) {
      if (a.id && (a.id === "d1-trigger" || a.id === "d2-trigger")) return a.id;
      if (a.getAttribute && a.getAttribute("data-slot")) return "content";
      return a.id || a.tagName;
    })(document.activeElement) + "|" + items;
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
    // pick a checkbox / radio item and reopen: the item flipped, its check with it
    "click:[role=menuitemcheckbox][data-state=unchecked]+click:#d1-trigger",
    "click:[role=menuitemcheckbox][data-state=checked]+click:#d1-trigger",
    "click:[role=menuitemradio][data-state=unchecked]+click:#d1-trigger",
  ],
}
