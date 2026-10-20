// Per-component contract: dropdown-menu (Wave B, kernel wireMenu)
export default {

  usage: `React.createElement(Demo)`,

  // radix checkbox / radio items hold no state of their own — a menu that
  // toggles is one whose app holds it (the dropdown-menu-checkboxes /
  // -radio-group examples), so the fixture declares that app
  imports: `
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuCheckboxItem, DropdownMenuRadioGroup, DropdownMenuRadioItem } from "@/registry/bases/radix/ui/dropdown-menu";
function Demo() {
  const [bar, setBar] = React.useState(true);
  const [urls, setUrls] = React.useState(false);
  const [pos, setPos] = React.useState("top");
  return React.createElement(DropdownMenu, null,
    React.createElement(DropdownMenuTrigger, { id: "d1-trigger" }, "Open menu"),
    React.createElement(DropdownMenuContent, null,
      React.createElement(DropdownMenuItem, { id: "d1-item-1" }, "Item 1"),
      React.createElement(DropdownMenuItem, null, "Item 2"),
      React.createElement(DropdownMenuCheckboxItem, { checked: bar, onCheckedChange: setBar }, "Status Bar"),
      React.createElement(DropdownMenuCheckboxItem, { checked: urls, onCheckedChange: setUrls }, "Full URLs"),
      React.createElement(DropdownMenuRadioGroup, { value: pos, onValueChange: setPos },
        React.createElement(DropdownMenuRadioItem, { value: "top" }, "Top"),
        React.createElement(DropdownMenuRadioItem, { value: "bottom" }, "Bottom"),
      ),
    ),
  );
}
`,

  slots: ["dropdown-menu-trigger", "dropdown-menu-content", "&[role=menuitem]"],

  open: `await page.click("#d1-trigger")`,
  openShadless: `await page.click("#d1-trigger")`,
  triggerSlot: "dropdown-menu-trigger",
  contentSlot: "dropdown-menu-content",

  shadlessPage: "src/kernel/dropdown-menu.html",

  ignoreAttrs: {
    "dropdown-menu-trigger": ["data-radix-popper-align"],
    "dropdown-menu-content": ["data-align"],
  },

  // open/closed, the highlighted item, and each checkbox / radio item's
  // state and whether it shows the check (radix mounts ItemIndicator in
  // checked items only)
  stateProbe: `(function () {
    var c = document.querySelector("[data-slot=dropdown-menu-content]");
    var h = document.querySelector("[data-highlighted]");
    var items = c ? Array.prototype.map.call(c.querySelectorAll("[role=menuitemcheckbox], [role=menuitemradio]"), function (i) {
      return i.textContent.trim() + ":" + i.getAttribute("data-state") + ":" + (i.firstElementChild.querySelector("svg") ? "check" : "-");
    }).join(",") : "";
    return (c ? "open" : "closed") + "|" + (h ? h.textContent.trim() : "-") + "|" + items;
  })()`,

  scenarios: ["escape", "outside-click", "mouse-click:[data-slot=dropdown-menu-trigger]", "click:[role=menuitem]",
    "key:ArrowDown", "key:ArrowDown+key:ArrowDown", "key:ArrowDown+key:Enter", "key:ArrowDown+key:ArrowUp",
    // pick a checkbox / radio item and reopen: the item flipped, its check
    // with it. The reopen waits out the close: the kernel ignores the
    // trigger while the closing layer is still exiting (~116ms), where radix
    // reopens at once — a separate kernel difference, not this behavior.
    "click:[role=menuitemcheckbox][data-state=unchecked]+wait:200+mouse-click:[data-slot=dropdown-menu-trigger]",
    "click:[role=menuitemcheckbox][data-state=checked]+wait:200+mouse-click:[data-slot=dropdown-menu-trigger]",
    "click:[role=menuitemradio][data-state=unchecked]+wait:200+mouse-click:[data-slot=dropdown-menu-trigger]"],
}
