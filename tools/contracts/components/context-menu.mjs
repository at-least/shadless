// Per-component contract: context-menu (Wave B, kernel wireMenu, point anchor)
export default {

  usage: `React.createElement(Demo)`,

  // radix checkbox / radio items hold no state of their own — a menu that
  // toggles is one whose app holds it (the context-menu-radio example), so
  // the fixture declares that app
  imports: `
import { ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem, ContextMenuCheckboxItem, ContextMenuRadioGroup, ContextMenuRadioItem } from "@/registry/bases/radix/ui/context-menu";
function Demo() {
  const [bar, setBar] = React.useState(true);
  const [urls, setUrls] = React.useState(false);
  const [person, setPerson] = React.useState("pedro");
  return React.createElement(ContextMenu, null,
    React.createElement(ContextMenuTrigger, { id: "d1-trigger" }, "Right click here"),
    React.createElement(ContextMenuContent, null,
      React.createElement(ContextMenuItem, { id: "d1-item-1" }, "Item 1"),
      React.createElement(ContextMenuItem, null, "Item 2"),
      React.createElement(ContextMenuCheckboxItem, { checked: bar, onCheckedChange: setBar }, "Bookmarks Bar"),
      React.createElement(ContextMenuCheckboxItem, { checked: urls, onCheckedChange: setUrls }, "Full URLs"),
      React.createElement(ContextMenuRadioGroup, { value: person, onValueChange: setPerson },
        React.createElement(ContextMenuRadioItem, { value: "pedro" }, "Pedro"),
        React.createElement(ContextMenuRadioItem, { value: "colm" }, "Colm"),
      ),
    ),
  );
}
`,

  slots: ["context-menu-trigger", "context-menu-content", "&[role=menuitem]"],

  open: `await page.click("#d1-trigger", { button: "right" })`,
  openShadless: `await page.click("#d1-trigger", { button: "right" })`,
  triggerSlot: "context-menu-trigger",
  contentSlot: "context-menu-content",

  shadlessPage: "src/kernel/context-menu.html",

  // open/closed, the highlighted item, and each checkbox / radio item's
  // state and whether it shows the check (radix mounts ItemIndicator in
  // checked items only)
  stateProbe: `(function () {
    var c = document.querySelector("[data-slot=context-menu-content]");
    var h = document.querySelector("[data-highlighted]");
    var items = c ? Array.prototype.map.call(c.querySelectorAll("[role=menuitemcheckbox], [role=menuitemradio]"), function (i) {
      return i.textContent.trim() + ":" + i.getAttribute("data-state") + ":" + (i.firstElementChild.querySelector("svg") ? "check" : "-");
    }).join(",") : "";
    return (c ? "open" : "closed") + "|" + (h ? h.textContent.trim() : "-") + "|" + items;
  })()`,

  scenarios: ["escape", "outside-click", "click:[role=menuitem]",
    "key:ArrowDown", "key:ArrowDown+key:Enter",
    // pick a checkbox / radio item and reopen: the item flipped, its check
    // with it. The reopen waits out the close: the kernel ignores the
    // trigger while the closing layer is still exiting (~116ms), where radix
    // reopens at once — a separate kernel difference, not this behavior.
    "click:[role=menuitemcheckbox][data-state=unchecked]+wait:200+right-click:[data-slot=context-menu-trigger]",
    "click:[role=menuitemcheckbox][data-state=checked]+wait:200+right-click:[data-slot=context-menu-trigger]",
    "click:[role=menuitemradio][data-state=unchecked]+wait:200+right-click:[data-slot=context-menu-trigger]"],
}
